#[inline]
pub unsafe fn CanSendToFaxRecipient() -> windows_core::BOOL {
    windows_core::link!("fxsutility.dll" "system" fn CanSendToFaxRecipient() -> windows_core::BOOL);
    unsafe { CanSendToFaxRecipient() }
}
#[inline]
pub unsafe fn FaxAbort(faxhandle: super::super::Foundation::HANDLE, jobid: u32) -> windows_core::BOOL {
    windows_core::link!("winfax.dll" "system" fn FaxAbort(faxhandle : super::super::Foundation:: HANDLE, jobid : u32) -> windows_core::BOOL);
    unsafe { FaxAbort(faxhandle, jobid) }
}
#[inline]
pub unsafe fn FaxAccessCheck(faxhandle: super::super::Foundation::HANDLE, accessmask: u32) -> windows_core::BOOL {
    windows_core::link!("winfax.dll" "system" fn FaxAccessCheck(faxhandle : super::super::Foundation:: HANDLE, accessmask : u32) -> windows_core::BOOL);
    unsafe { FaxAccessCheck(faxhandle, accessmask) }
}
#[inline]
pub unsafe fn FaxClose(faxhandle: super::super::Foundation::HANDLE) -> windows_core::BOOL {
    windows_core::link!("winfax.dll" "system" fn FaxClose(faxhandle : super::super::Foundation:: HANDLE) -> windows_core::BOOL);
    unsafe { FaxClose(faxhandle) }
}
#[inline]
pub unsafe fn FaxCompleteJobParamsA(jobparams: *mut *mut FAX_JOB_PARAMA, coverpageinfo: *mut *mut FAX_COVERPAGE_INFOA) -> windows_core::BOOL {
    windows_core::link!("winfax.dll" "system" fn FaxCompleteJobParamsA(jobparams : *mut *mut FAX_JOB_PARAMA, coverpageinfo : *mut *mut FAX_COVERPAGE_INFOA) -> windows_core::BOOL);
    unsafe { FaxCompleteJobParamsA(jobparams as _, coverpageinfo as _) }
}
#[inline]
pub unsafe fn FaxCompleteJobParamsW(jobparams: *mut *mut FAX_JOB_PARAMW, coverpageinfo: *mut *mut FAX_COVERPAGE_INFOW) -> windows_core::BOOL {
    windows_core::link!("winfax.dll" "system" fn FaxCompleteJobParamsW(jobparams : *mut *mut FAX_JOB_PARAMW, coverpageinfo : *mut *mut FAX_COVERPAGE_INFOW) -> windows_core::BOOL);
    unsafe { FaxCompleteJobParamsW(jobparams as _, coverpageinfo as _) }
}
#[inline]
pub unsafe fn FaxConnectFaxServerA<P0>(machinename: P0, faxhandle: *mut super::super::Foundation::HANDLE) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("winfax.dll" "system" fn FaxConnectFaxServerA(machinename : windows_core::PCSTR, faxhandle : *mut super::super::Foundation:: HANDLE) -> windows_core::BOOL);
    unsafe { FaxConnectFaxServerA(machinename.param().abi(), faxhandle as _).ok() }
}
#[inline]
pub unsafe fn FaxConnectFaxServerW<P0>(machinename: P0, faxhandle: *mut super::super::Foundation::HANDLE) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("winfax.dll" "system" fn FaxConnectFaxServerW(machinename : windows_core::PCWSTR, faxhandle : *mut super::super::Foundation:: HANDLE) -> windows_core::BOOL);
    unsafe { FaxConnectFaxServerW(machinename.param().abi(), faxhandle as _).ok() }
}
#[inline]
pub unsafe fn FaxEnableRoutingMethodA<P1>(faxporthandle: super::super::Foundation::HANDLE, routingguid: P1, enabled: bool) -> windows_core::Result<()>
where
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("winfax.dll" "system" fn FaxEnableRoutingMethodA(faxporthandle : super::super::Foundation:: HANDLE, routingguid : windows_core::PCSTR, enabled : windows_core::BOOL) -> windows_core::BOOL);
    unsafe { FaxEnableRoutingMethodA(faxporthandle, routingguid.param().abi(), enabled.into()).ok() }
}
#[inline]
pub unsafe fn FaxEnableRoutingMethodW<P1>(faxporthandle: super::super::Foundation::HANDLE, routingguid: P1, enabled: bool) -> windows_core::Result<()>
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("winfax.dll" "system" fn FaxEnableRoutingMethodW(faxporthandle : super::super::Foundation:: HANDLE, routingguid : windows_core::PCWSTR, enabled : windows_core::BOOL) -> windows_core::BOOL);
    unsafe { FaxEnableRoutingMethodW(faxporthandle, routingguid.param().abi(), enabled.into()).ok() }
}
#[inline]
pub unsafe fn FaxEnumGlobalRoutingInfoA(faxhandle: super::super::Foundation::HANDLE, routinginfo: *mut *mut FAX_GLOBAL_ROUTING_INFOA, methodsreturned: *mut u32) -> windows_core::Result<()> {
    windows_core::link!("winfax.dll" "system" fn FaxEnumGlobalRoutingInfoA(faxhandle : super::super::Foundation:: HANDLE, routinginfo : *mut *mut FAX_GLOBAL_ROUTING_INFOA, methodsreturned : *mut u32) -> windows_core::BOOL);
    unsafe { FaxEnumGlobalRoutingInfoA(faxhandle, routinginfo as _, methodsreturned as _).ok() }
}
#[inline]
pub unsafe fn FaxEnumGlobalRoutingInfoW(faxhandle: super::super::Foundation::HANDLE, routinginfo: *mut *mut FAX_GLOBAL_ROUTING_INFOW, methodsreturned: *mut u32) -> windows_core::Result<()> {
    windows_core::link!("winfax.dll" "system" fn FaxEnumGlobalRoutingInfoW(faxhandle : super::super::Foundation:: HANDLE, routinginfo : *mut *mut FAX_GLOBAL_ROUTING_INFOW, methodsreturned : *mut u32) -> windows_core::BOOL);
    unsafe { FaxEnumGlobalRoutingInfoW(faxhandle, routinginfo as _, methodsreturned as _).ok() }
}
#[inline]
pub unsafe fn FaxEnumJobsA(faxhandle: super::super::Foundation::HANDLE, jobentry: *mut *mut FAX_JOB_ENTRYA, jobsreturned: *mut u32) -> windows_core::Result<()> {
    windows_core::link!("winfax.dll" "system" fn FaxEnumJobsA(faxhandle : super::super::Foundation:: HANDLE, jobentry : *mut *mut FAX_JOB_ENTRYA, jobsreturned : *mut u32) -> windows_core::BOOL);
    unsafe { FaxEnumJobsA(faxhandle, jobentry as _, jobsreturned as _).ok() }
}
#[inline]
pub unsafe fn FaxEnumJobsW(faxhandle: super::super::Foundation::HANDLE, jobentry: *mut *mut FAX_JOB_ENTRYW, jobsreturned: *mut u32) -> windows_core::Result<()> {
    windows_core::link!("winfax.dll" "system" fn FaxEnumJobsW(faxhandle : super::super::Foundation:: HANDLE, jobentry : *mut *mut FAX_JOB_ENTRYW, jobsreturned : *mut u32) -> windows_core::BOOL);
    unsafe { FaxEnumJobsW(faxhandle, jobentry as _, jobsreturned as _).ok() }
}
#[inline]
pub unsafe fn FaxEnumPortsA(faxhandle: super::super::Foundation::HANDLE, portinfo: *mut *mut FAX_PORT_INFOA, portsreturned: *mut u32) -> windows_core::Result<()> {
    windows_core::link!("winfax.dll" "system" fn FaxEnumPortsA(faxhandle : super::super::Foundation:: HANDLE, portinfo : *mut *mut FAX_PORT_INFOA, portsreturned : *mut u32) -> windows_core::BOOL);
    unsafe { FaxEnumPortsA(faxhandle, portinfo as _, portsreturned as _).ok() }
}
#[inline]
pub unsafe fn FaxEnumPortsW(faxhandle: super::super::Foundation::HANDLE, portinfo: *mut *mut FAX_PORT_INFOW, portsreturned: *mut u32) -> windows_core::Result<()> {
    windows_core::link!("winfax.dll" "system" fn FaxEnumPortsW(faxhandle : super::super::Foundation:: HANDLE, portinfo : *mut *mut FAX_PORT_INFOW, portsreturned : *mut u32) -> windows_core::BOOL);
    unsafe { FaxEnumPortsW(faxhandle, portinfo as _, portsreturned as _).ok() }
}
#[inline]
pub unsafe fn FaxEnumRoutingMethodsA(faxporthandle: super::super::Foundation::HANDLE, routingmethod: *mut *mut FAX_ROUTING_METHODA, methodsreturned: *mut u32) -> windows_core::Result<()> {
    windows_core::link!("winfax.dll" "system" fn FaxEnumRoutingMethodsA(faxporthandle : super::super::Foundation:: HANDLE, routingmethod : *mut *mut FAX_ROUTING_METHODA, methodsreturned : *mut u32) -> windows_core::BOOL);
    unsafe { FaxEnumRoutingMethodsA(faxporthandle, routingmethod as _, methodsreturned as _).ok() }
}
#[inline]
pub unsafe fn FaxEnumRoutingMethodsW(faxporthandle: super::super::Foundation::HANDLE, routingmethod: *mut *mut FAX_ROUTING_METHODW, methodsreturned: *mut u32) -> windows_core::Result<()> {
    windows_core::link!("winfax.dll" "system" fn FaxEnumRoutingMethodsW(faxporthandle : super::super::Foundation:: HANDLE, routingmethod : *mut *mut FAX_ROUTING_METHODW, methodsreturned : *mut u32) -> windows_core::BOOL);
    unsafe { FaxEnumRoutingMethodsW(faxporthandle, routingmethod as _, methodsreturned as _).ok() }
}
#[inline]
pub unsafe fn FaxFreeBuffer(buffer: *mut core::ffi::c_void) {
    windows_core::link!("winfax.dll" "system" fn FaxFreeBuffer(buffer : *mut core::ffi::c_void));
    unsafe { FaxFreeBuffer(buffer as _) }
}
#[inline]
pub unsafe fn FaxGetConfigurationA(faxhandle: super::super::Foundation::HANDLE, faxconfig: *mut *mut FAX_CONFIGURATIONA) -> windows_core::Result<()> {
    windows_core::link!("winfax.dll" "system" fn FaxGetConfigurationA(faxhandle : super::super::Foundation:: HANDLE, faxconfig : *mut *mut FAX_CONFIGURATIONA) -> windows_core::BOOL);
    unsafe { FaxGetConfigurationA(faxhandle, faxconfig as _).ok() }
}
#[inline]
pub unsafe fn FaxGetConfigurationW(faxhandle: super::super::Foundation::HANDLE, faxconfig: *mut *mut FAX_CONFIGURATIONW) -> windows_core::Result<()> {
    windows_core::link!("winfax.dll" "system" fn FaxGetConfigurationW(faxhandle : super::super::Foundation:: HANDLE, faxconfig : *mut *mut FAX_CONFIGURATIONW) -> windows_core::BOOL);
    unsafe { FaxGetConfigurationW(faxhandle, faxconfig as _).ok() }
}
#[inline]
pub unsafe fn FaxGetDeviceStatusA(faxporthandle: super::super::Foundation::HANDLE, devicestatus: *mut *mut FAX_DEVICE_STATUSA) -> windows_core::Result<()> {
    windows_core::link!("winfax.dll" "system" fn FaxGetDeviceStatusA(faxporthandle : super::super::Foundation:: HANDLE, devicestatus : *mut *mut FAX_DEVICE_STATUSA) -> windows_core::BOOL);
    unsafe { FaxGetDeviceStatusA(faxporthandle, devicestatus as _).ok() }
}
#[inline]
pub unsafe fn FaxGetDeviceStatusW(faxporthandle: super::super::Foundation::HANDLE, devicestatus: *mut *mut FAX_DEVICE_STATUSW) -> windows_core::Result<()> {
    windows_core::link!("winfax.dll" "system" fn FaxGetDeviceStatusW(faxporthandle : super::super::Foundation:: HANDLE, devicestatus : *mut *mut FAX_DEVICE_STATUSW) -> windows_core::BOOL);
    unsafe { FaxGetDeviceStatusW(faxporthandle, devicestatus as _).ok() }
}
#[inline]
pub unsafe fn FaxGetJobA(faxhandle: super::super::Foundation::HANDLE, jobid: u32, jobentry: *mut *mut FAX_JOB_ENTRYA) -> windows_core::Result<()> {
    windows_core::link!("winfax.dll" "system" fn FaxGetJobA(faxhandle : super::super::Foundation:: HANDLE, jobid : u32, jobentry : *mut *mut FAX_JOB_ENTRYA) -> windows_core::BOOL);
    unsafe { FaxGetJobA(faxhandle, jobid, jobentry as _).ok() }
}
#[inline]
pub unsafe fn FaxGetJobW(faxhandle: super::super::Foundation::HANDLE, jobid: u32, jobentry: *mut *mut FAX_JOB_ENTRYW) -> windows_core::Result<()> {
    windows_core::link!("winfax.dll" "system" fn FaxGetJobW(faxhandle : super::super::Foundation:: HANDLE, jobid : u32, jobentry : *mut *mut FAX_JOB_ENTRYW) -> windows_core::BOOL);
    unsafe { FaxGetJobW(faxhandle, jobid, jobentry as _).ok() }
}
#[inline]
pub unsafe fn FaxGetLoggingCategoriesA(faxhandle: super::super::Foundation::HANDLE, categories: *mut *mut FAX_LOG_CATEGORYA, numbercategories: *mut u32) -> windows_core::Result<()> {
    windows_core::link!("winfax.dll" "system" fn FaxGetLoggingCategoriesA(faxhandle : super::super::Foundation:: HANDLE, categories : *mut *mut FAX_LOG_CATEGORYA, numbercategories : *mut u32) -> windows_core::BOOL);
    unsafe { FaxGetLoggingCategoriesA(faxhandle, categories as _, numbercategories as _).ok() }
}
#[inline]
pub unsafe fn FaxGetLoggingCategoriesW(faxhandle: super::super::Foundation::HANDLE, categories: *mut *mut FAX_LOG_CATEGORYW, numbercategories: *mut u32) -> windows_core::Result<()> {
    windows_core::link!("winfax.dll" "system" fn FaxGetLoggingCategoriesW(faxhandle : super::super::Foundation:: HANDLE, categories : *mut *mut FAX_LOG_CATEGORYW, numbercategories : *mut u32) -> windows_core::BOOL);
    unsafe { FaxGetLoggingCategoriesW(faxhandle, categories as _, numbercategories as _).ok() }
}
#[inline]
pub unsafe fn FaxGetPageData(faxhandle: super::super::Foundation::HANDLE, jobid: u32, buffer: *mut *mut u8, buffersize: *mut u32, imagewidth: *mut u32, imageheight: *mut u32) -> windows_core::BOOL {
    windows_core::link!("winfax.dll" "system" fn FaxGetPageData(faxhandle : super::super::Foundation:: HANDLE, jobid : u32, buffer : *mut *mut u8, buffersize : *mut u32, imagewidth : *mut u32, imageheight : *mut u32) -> windows_core::BOOL);
    unsafe { FaxGetPageData(faxhandle, jobid, buffer as _, buffersize as _, imagewidth as _, imageheight as _) }
}
#[inline]
pub unsafe fn FaxGetPortA(faxporthandle: super::super::Foundation::HANDLE, portinfo: *mut *mut FAX_PORT_INFOA) -> windows_core::Result<()> {
    windows_core::link!("winfax.dll" "system" fn FaxGetPortA(faxporthandle : super::super::Foundation:: HANDLE, portinfo : *mut *mut FAX_PORT_INFOA) -> windows_core::BOOL);
    unsafe { FaxGetPortA(faxporthandle, portinfo as _).ok() }
}
#[inline]
pub unsafe fn FaxGetPortW(faxporthandle: super::super::Foundation::HANDLE, portinfo: *mut *mut FAX_PORT_INFOW) -> windows_core::Result<()> {
    windows_core::link!("winfax.dll" "system" fn FaxGetPortW(faxporthandle : super::super::Foundation:: HANDLE, portinfo : *mut *mut FAX_PORT_INFOW) -> windows_core::BOOL);
    unsafe { FaxGetPortW(faxporthandle, portinfo as _).ok() }
}
#[inline]
pub unsafe fn FaxGetRoutingInfoA<P1>(faxporthandle: super::super::Foundation::HANDLE, routingguid: P1, routinginfobuffer: *mut *mut u8, routinginfobuffersize: *mut u32) -> windows_core::Result<()>
where
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("winfax.dll" "system" fn FaxGetRoutingInfoA(faxporthandle : super::super::Foundation:: HANDLE, routingguid : windows_core::PCSTR, routinginfobuffer : *mut *mut u8, routinginfobuffersize : *mut u32) -> windows_core::BOOL);
    unsafe { FaxGetRoutingInfoA(faxporthandle, routingguid.param().abi(), routinginfobuffer as _, routinginfobuffersize as _).ok() }
}
#[inline]
pub unsafe fn FaxGetRoutingInfoW<P1>(faxporthandle: super::super::Foundation::HANDLE, routingguid: P1, routinginfobuffer: *mut *mut u8, routinginfobuffersize: *mut u32) -> windows_core::Result<()>
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("winfax.dll" "system" fn FaxGetRoutingInfoW(faxporthandle : super::super::Foundation:: HANDLE, routingguid : windows_core::PCWSTR, routinginfobuffer : *mut *mut u8, routinginfobuffersize : *mut u32) -> windows_core::BOOL);
    unsafe { FaxGetRoutingInfoW(faxporthandle, routingguid.param().abi(), routinginfobuffer as _, routinginfobuffersize as _).ok() }
}
#[inline]
pub unsafe fn FaxInitializeEventQueue(faxhandle: super::super::Foundation::HANDLE, completionport: super::super::Foundation::HANDLE, completionkey: usize, hwnd: super::super::Foundation::HWND, messagestart: u32) -> windows_core::BOOL {
    windows_core::link!("winfax.dll" "system" fn FaxInitializeEventQueue(faxhandle : super::super::Foundation:: HANDLE, completionport : super::super::Foundation:: HANDLE, completionkey : usize, hwnd : super::super::Foundation:: HWND, messagestart : u32) -> windows_core::BOOL);
    unsafe { FaxInitializeEventQueue(faxhandle, completionport, completionkey, hwnd, messagestart) }
}
#[inline]
pub unsafe fn FaxOpenPort(faxhandle: super::super::Foundation::HANDLE, deviceid: u32, flags: u32, faxporthandle: *mut super::super::Foundation::HANDLE) -> windows_core::BOOL {
    windows_core::link!("winfax.dll" "system" fn FaxOpenPort(faxhandle : super::super::Foundation:: HANDLE, deviceid : u32, flags : u32, faxporthandle : *mut super::super::Foundation:: HANDLE) -> windows_core::BOOL);
    unsafe { FaxOpenPort(faxhandle, deviceid, flags, faxporthandle as _) }
}
#[cfg(feature = "Win32_Graphics_Gdi")]
#[inline]
pub unsafe fn FaxPrintCoverPageA(faxcontextinfo: *const FAX_CONTEXT_INFOA, coverpageinfo: *const FAX_COVERPAGE_INFOA) -> windows_core::Result<()> {
    windows_core::link!("winfax.dll" "system" fn FaxPrintCoverPageA(faxcontextinfo : *const FAX_CONTEXT_INFOA, coverpageinfo : *const FAX_COVERPAGE_INFOA) -> windows_core::BOOL);
    unsafe { FaxPrintCoverPageA(faxcontextinfo, coverpageinfo).ok() }
}
#[cfg(feature = "Win32_Graphics_Gdi")]
#[inline]
pub unsafe fn FaxPrintCoverPageW(faxcontextinfo: *const FAX_CONTEXT_INFOW, coverpageinfo: *const FAX_COVERPAGE_INFOW) -> windows_core::Result<()> {
    windows_core::link!("winfax.dll" "system" fn FaxPrintCoverPageW(faxcontextinfo : *const FAX_CONTEXT_INFOW, coverpageinfo : *const FAX_COVERPAGE_INFOW) -> windows_core::BOOL);
    unsafe { FaxPrintCoverPageW(faxcontextinfo, coverpageinfo).ok() }
}
#[inline]
pub unsafe fn FaxRegisterRoutingExtensionW<P1, P2, P3>(faxhandle: super::super::Foundation::HANDLE, extensionname: P1, friendlyname: P2, imagename: P3, callback: PFAX_ROUTING_INSTALLATION_CALLBACKW, context: *const core::ffi::c_void) -> windows_core::Result<()>
where
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
    P3: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("winfax.dll" "system" fn FaxRegisterRoutingExtensionW(faxhandle : super::super::Foundation:: HANDLE, extensionname : windows_core::PCWSTR, friendlyname : windows_core::PCWSTR, imagename : windows_core::PCWSTR, callback : PFAX_ROUTING_INSTALLATION_CALLBACKW, context : *const core::ffi::c_void) -> windows_core::BOOL);
    unsafe { FaxRegisterRoutingExtensionW(faxhandle, extensionname.param().abi(), friendlyname.param().abi(), imagename.param().abi(), callback, context).ok() }
}
#[inline]
pub unsafe fn FaxRegisterServiceProviderW<P0, P1, P2, P3>(deviceprovider: P0, friendlyname: P1, imagename: P2, tspname: P3) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
    P3: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("winfax.dll" "system" fn FaxRegisterServiceProviderW(deviceprovider : windows_core::PCWSTR, friendlyname : windows_core::PCWSTR, imagename : windows_core::PCWSTR, tspname : windows_core::PCWSTR) -> windows_core::BOOL);
    unsafe { FaxRegisterServiceProviderW(deviceprovider.param().abi(), friendlyname.param().abi(), imagename.param().abi(), tspname.param().abi()).ok() }
}
#[inline]
pub unsafe fn FaxSendDocumentA<P1>(faxhandle: super::super::Foundation::HANDLE, filename: P1, jobparams: *const FAX_JOB_PARAMA, coverpageinfo: *const FAX_COVERPAGE_INFOA, faxjobid: *mut u32) -> windows_core::Result<()>
where
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("winfax.dll" "system" fn FaxSendDocumentA(faxhandle : super::super::Foundation:: HANDLE, filename : windows_core::PCSTR, jobparams : *const FAX_JOB_PARAMA, coverpageinfo : *const FAX_COVERPAGE_INFOA, faxjobid : *mut u32) -> windows_core::BOOL);
    unsafe { FaxSendDocumentA(faxhandle, filename.param().abi(), jobparams, coverpageinfo, faxjobid as _).ok() }
}
#[inline]
pub unsafe fn FaxSendDocumentForBroadcastA<P1>(faxhandle: super::super::Foundation::HANDLE, filename: P1, faxjobid: *mut u32, faxrecipientcallback: PFAX_RECIPIENT_CALLBACKA, context: *const core::ffi::c_void) -> windows_core::Result<()>
where
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("winfax.dll" "system" fn FaxSendDocumentForBroadcastA(faxhandle : super::super::Foundation:: HANDLE, filename : windows_core::PCSTR, faxjobid : *mut u32, faxrecipientcallback : PFAX_RECIPIENT_CALLBACKA, context : *const core::ffi::c_void) -> windows_core::BOOL);
    unsafe { FaxSendDocumentForBroadcastA(faxhandle, filename.param().abi(), faxjobid as _, faxrecipientcallback, context).ok() }
}
#[inline]
pub unsafe fn FaxSendDocumentForBroadcastW<P1>(faxhandle: super::super::Foundation::HANDLE, filename: P1, faxjobid: *mut u32, faxrecipientcallback: PFAX_RECIPIENT_CALLBACKW, context: *const core::ffi::c_void) -> windows_core::Result<()>
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("winfax.dll" "system" fn FaxSendDocumentForBroadcastW(faxhandle : super::super::Foundation:: HANDLE, filename : windows_core::PCWSTR, faxjobid : *mut u32, faxrecipientcallback : PFAX_RECIPIENT_CALLBACKW, context : *const core::ffi::c_void) -> windows_core::BOOL);
    unsafe { FaxSendDocumentForBroadcastW(faxhandle, filename.param().abi(), faxjobid as _, faxrecipientcallback, context).ok() }
}
#[inline]
pub unsafe fn FaxSendDocumentW<P1>(faxhandle: super::super::Foundation::HANDLE, filename: P1, jobparams: *const FAX_JOB_PARAMW, coverpageinfo: *const FAX_COVERPAGE_INFOW, faxjobid: *mut u32) -> windows_core::Result<()>
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("winfax.dll" "system" fn FaxSendDocumentW(faxhandle : super::super::Foundation:: HANDLE, filename : windows_core::PCWSTR, jobparams : *const FAX_JOB_PARAMW, coverpageinfo : *const FAX_COVERPAGE_INFOW, faxjobid : *mut u32) -> windows_core::BOOL);
    unsafe { FaxSendDocumentW(faxhandle, filename.param().abi(), jobparams, coverpageinfo, faxjobid as _).ok() }
}
#[inline]
pub unsafe fn FaxSetConfigurationA(faxhandle: super::super::Foundation::HANDLE, faxconfig: *const FAX_CONFIGURATIONA) -> windows_core::Result<()> {
    windows_core::link!("winfax.dll" "system" fn FaxSetConfigurationA(faxhandle : super::super::Foundation:: HANDLE, faxconfig : *const FAX_CONFIGURATIONA) -> windows_core::BOOL);
    unsafe { FaxSetConfigurationA(faxhandle, faxconfig).ok() }
}
#[inline]
pub unsafe fn FaxSetConfigurationW(faxhandle: super::super::Foundation::HANDLE, faxconfig: *const FAX_CONFIGURATIONW) -> windows_core::Result<()> {
    windows_core::link!("winfax.dll" "system" fn FaxSetConfigurationW(faxhandle : super::super::Foundation:: HANDLE, faxconfig : *const FAX_CONFIGURATIONW) -> windows_core::BOOL);
    unsafe { FaxSetConfigurationW(faxhandle, faxconfig).ok() }
}
#[inline]
pub unsafe fn FaxSetGlobalRoutingInfoA(faxhandle: super::super::Foundation::HANDLE, routinginfo: *const FAX_GLOBAL_ROUTING_INFOA) -> windows_core::Result<()> {
    windows_core::link!("winfax.dll" "system" fn FaxSetGlobalRoutingInfoA(faxhandle : super::super::Foundation:: HANDLE, routinginfo : *const FAX_GLOBAL_ROUTING_INFOA) -> windows_core::BOOL);
    unsafe { FaxSetGlobalRoutingInfoA(faxhandle, routinginfo).ok() }
}
#[inline]
pub unsafe fn FaxSetGlobalRoutingInfoW(faxhandle: super::super::Foundation::HANDLE, routinginfo: *const FAX_GLOBAL_ROUTING_INFOW) -> windows_core::Result<()> {
    windows_core::link!("winfax.dll" "system" fn FaxSetGlobalRoutingInfoW(faxhandle : super::super::Foundation:: HANDLE, routinginfo : *const FAX_GLOBAL_ROUTING_INFOW) -> windows_core::BOOL);
    unsafe { FaxSetGlobalRoutingInfoW(faxhandle, routinginfo).ok() }
}
#[inline]
pub unsafe fn FaxSetJobA(faxhandle: super::super::Foundation::HANDLE, jobid: u32, command: u32, jobentry: *const FAX_JOB_ENTRYA) -> windows_core::Result<()> {
    windows_core::link!("winfax.dll" "system" fn FaxSetJobA(faxhandle : super::super::Foundation:: HANDLE, jobid : u32, command : u32, jobentry : *const FAX_JOB_ENTRYA) -> windows_core::BOOL);
    unsafe { FaxSetJobA(faxhandle, jobid, command, jobentry).ok() }
}
#[inline]
pub unsafe fn FaxSetJobW(faxhandle: super::super::Foundation::HANDLE, jobid: u32, command: u32, jobentry: *const FAX_JOB_ENTRYW) -> windows_core::Result<()> {
    windows_core::link!("winfax.dll" "system" fn FaxSetJobW(faxhandle : super::super::Foundation:: HANDLE, jobid : u32, command : u32, jobentry : *const FAX_JOB_ENTRYW) -> windows_core::BOOL);
    unsafe { FaxSetJobW(faxhandle, jobid, command, jobentry).ok() }
}
#[inline]
pub unsafe fn FaxSetLoggingCategoriesA(faxhandle: super::super::Foundation::HANDLE, categories: *const FAX_LOG_CATEGORYA, numbercategories: u32) -> windows_core::Result<()> {
    windows_core::link!("winfax.dll" "system" fn FaxSetLoggingCategoriesA(faxhandle : super::super::Foundation:: HANDLE, categories : *const FAX_LOG_CATEGORYA, numbercategories : u32) -> windows_core::BOOL);
    unsafe { FaxSetLoggingCategoriesA(faxhandle, categories, numbercategories).ok() }
}
#[inline]
pub unsafe fn FaxSetLoggingCategoriesW(faxhandle: super::super::Foundation::HANDLE, categories: *const FAX_LOG_CATEGORYW, numbercategories: u32) -> windows_core::Result<()> {
    windows_core::link!("winfax.dll" "system" fn FaxSetLoggingCategoriesW(faxhandle : super::super::Foundation:: HANDLE, categories : *const FAX_LOG_CATEGORYW, numbercategories : u32) -> windows_core::BOOL);
    unsafe { FaxSetLoggingCategoriesW(faxhandle, categories, numbercategories).ok() }
}
#[inline]
pub unsafe fn FaxSetPortA(faxporthandle: super::super::Foundation::HANDLE, portinfo: *const FAX_PORT_INFOA) -> windows_core::Result<()> {
    windows_core::link!("winfax.dll" "system" fn FaxSetPortA(faxporthandle : super::super::Foundation:: HANDLE, portinfo : *const FAX_PORT_INFOA) -> windows_core::BOOL);
    unsafe { FaxSetPortA(faxporthandle, portinfo).ok() }
}
#[inline]
pub unsafe fn FaxSetPortW(faxporthandle: super::super::Foundation::HANDLE, portinfo: *const FAX_PORT_INFOW) -> windows_core::Result<()> {
    windows_core::link!("winfax.dll" "system" fn FaxSetPortW(faxporthandle : super::super::Foundation:: HANDLE, portinfo : *const FAX_PORT_INFOW) -> windows_core::BOOL);
    unsafe { FaxSetPortW(faxporthandle, portinfo).ok() }
}
#[inline]
pub unsafe fn FaxSetRoutingInfoA<P1>(faxporthandle: super::super::Foundation::HANDLE, routingguid: P1, routinginfobuffer: *const u8, routinginfobuffersize: u32) -> windows_core::Result<()>
where
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("winfax.dll" "system" fn FaxSetRoutingInfoA(faxporthandle : super::super::Foundation:: HANDLE, routingguid : windows_core::PCSTR, routinginfobuffer : *const u8, routinginfobuffersize : u32) -> windows_core::BOOL);
    unsafe { FaxSetRoutingInfoA(faxporthandle, routingguid.param().abi(), routinginfobuffer, routinginfobuffersize).ok() }
}
#[inline]
pub unsafe fn FaxSetRoutingInfoW<P1>(faxporthandle: super::super::Foundation::HANDLE, routingguid: P1, routinginfobuffer: *const u8, routinginfobuffersize: u32) -> windows_core::Result<()>
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("winfax.dll" "system" fn FaxSetRoutingInfoW(faxporthandle : super::super::Foundation:: HANDLE, routingguid : windows_core::PCWSTR, routinginfobuffer : *const u8, routinginfobuffersize : u32) -> windows_core::BOOL);
    unsafe { FaxSetRoutingInfoW(faxporthandle, routingguid.param().abi(), routinginfobuffer, routinginfobuffersize).ok() }
}
#[cfg(feature = "Win32_Graphics_Gdi")]
#[inline]
pub unsafe fn FaxStartPrintJobA<P0>(printername: P0, printinfo: *const FAX_PRINT_INFOA, faxjobid: *mut u32, faxcontextinfo: *mut FAX_CONTEXT_INFOA) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("winfax.dll" "system" fn FaxStartPrintJobA(printername : windows_core::PCSTR, printinfo : *const FAX_PRINT_INFOA, faxjobid : *mut u32, faxcontextinfo : *mut FAX_CONTEXT_INFOA) -> windows_core::BOOL);
    unsafe { FaxStartPrintJobA(printername.param().abi(), printinfo, faxjobid as _, faxcontextinfo as _).ok() }
}
#[cfg(feature = "Win32_Graphics_Gdi")]
#[inline]
pub unsafe fn FaxStartPrintJobW<P0>(printername: P0, printinfo: *const FAX_PRINT_INFOW, faxjobid: *mut u32, faxcontextinfo: *mut FAX_CONTEXT_INFOW) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("winfax.dll" "system" fn FaxStartPrintJobW(printername : windows_core::PCWSTR, printinfo : *const FAX_PRINT_INFOW, faxjobid : *mut u32, faxcontextinfo : *mut FAX_CONTEXT_INFOW) -> windows_core::BOOL);
    unsafe { FaxStartPrintJobW(printername.param().abi(), printinfo, faxjobid as _, faxcontextinfo as _).ok() }
}
#[inline]
pub unsafe fn FaxUnregisterServiceProviderW<P0>(deviceprovider: P0) -> windows_core::BOOL
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("winfax.dll" "system" fn FaxUnregisterServiceProviderW(deviceprovider : windows_core::PCWSTR) -> windows_core::BOOL);
    unsafe { FaxUnregisterServiceProviderW(deviceprovider.param().abi()) }
}
#[inline]
pub unsafe fn SendToFaxRecipient<P1>(sndmode: SendToMode, lpfilename: P1) -> u32
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("fxsutility.dll" "system" fn SendToFaxRecipient(sndmode : SendToMode, lpfilename : windows_core::PCWSTR) -> u32);
    unsafe { SendToFaxRecipient(sndmode, lpfilename.param().abi()) }
}
#[inline]
pub unsafe fn StiCreateInstanceW<P3>(hinst: super::super::Foundation::HINSTANCE, dwver: u32, ppsti: *mut Option<IStillImageW>, punkouter: P3) -> windows_core::Result<()>
where
    P3: windows_core::Param<windows_core::IUnknown>,
{
    windows_core::link!("sti.dll" "system" fn StiCreateInstanceW(hinst : super::super::Foundation:: HINSTANCE, dwver : u32, ppsti : *mut * mut core::ffi::c_void, punkouter : * mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe { StiCreateInstanceW(hinst, dwver, core::mem::transmute(ppsti), punkouter.param().abi()).ok() }
}
pub const CF_MSFAXSRV_DEVICE_ID: windows_core::PCWSTR = windows_core::w!("FAXSRV_DeviceID");
pub const CF_MSFAXSRV_FSP_GUID: windows_core::PCWSTR = windows_core::w!("FAXSRV_FSPGuid");
pub const CF_MSFAXSRV_ROUTEEXT_NAME: windows_core::PCWSTR = windows_core::w!("FAXSRV_RoutingExtName");
pub const CF_MSFAXSRV_ROUTING_METHOD_GUID: windows_core::PCWSTR = windows_core::w!("FAXSRV_RoutingMethodGuid");
pub const CF_MSFAXSRV_SERVER_NAME: windows_core::PCWSTR = windows_core::w!("FAXSRV_ServerName");
pub const CLSID_Sti: windows_core::GUID = windows_core::GUID::from_u128(0xb323f8e0_2e68_11d0_90ea_00aa0060f86c);
pub const DEVPKEY_WIA_DeviceType: super::super::Foundation::DEVPROPKEY = super::super::Foundation::DEVPROPKEY { fmtid: windows_core::GUID::from_u128(0x6bdd1fc6_810f_11d0_bec7_08002be2092f), pid: 2 };
pub const DEVPKEY_WIA_USDClassId: super::super::Foundation::DEVPROPKEY = super::super::Foundation::DEVPROPKEY { fmtid: windows_core::GUID::from_u128(0x6bdd1fc6_810f_11d0_bec7_08002be2092f), pid: 3 };
pub const DEV_ID_SRC_FAX: FAX_ENUM_DEVICE_ID_SOURCE = FAX_ENUM_DEVICE_ID_SOURCE(0i32);
pub const DEV_ID_SRC_TAPI: FAX_ENUM_DEVICE_ID_SOURCE = FAX_ENUM_DEVICE_ID_SOURCE(1i32);
pub const DRT_EMAIL: FAX_ENUM_DELIVERY_REPORT_TYPES = FAX_ENUM_DELIVERY_REPORT_TYPES(1i32);
pub const DRT_INBOX: FAX_ENUM_DELIVERY_REPORT_TYPES = FAX_ENUM_DELIVERY_REPORT_TYPES(2i32);
pub const DRT_NONE: FAX_ENUM_DELIVERY_REPORT_TYPES = FAX_ENUM_DELIVERY_REPORT_TYPES(0i32);
pub const FAXDEVRECEIVE_SIZE: u32 = 4096u32;
pub const FAXDEVREPORTSTATUS_SIZE: u32 = 4096u32;
pub const FAXLOG_CATEGORY_INBOUND: FAX_ENUM_LOG_CATEGORIES = FAX_ENUM_LOG_CATEGORIES(3i32);
pub const FAXLOG_CATEGORY_INIT: FAX_ENUM_LOG_CATEGORIES = FAX_ENUM_LOG_CATEGORIES(1i32);
pub const FAXLOG_CATEGORY_OUTBOUND: FAX_ENUM_LOG_CATEGORIES = FAX_ENUM_LOG_CATEGORIES(2i32);
pub const FAXLOG_CATEGORY_UNKNOWN: FAX_ENUM_LOG_CATEGORIES = FAX_ENUM_LOG_CATEGORIES(4i32);
pub const FAXLOG_LEVEL_MAX: FAX_ENUM_LOG_LEVELS = FAX_ENUM_LOG_LEVELS(3i32);
pub const FAXLOG_LEVEL_MED: FAX_ENUM_LOG_LEVELS = FAX_ENUM_LOG_LEVELS(2i32);
pub const FAXLOG_LEVEL_MIN: FAX_ENUM_LOG_LEVELS = FAX_ENUM_LOG_LEVELS(1i32);
pub const FAXLOG_LEVEL_NONE: FAX_ENUM_LOG_LEVELS = FAX_ENUM_LOG_LEVELS(0i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct FAXROUTE_ENABLE(pub i32);
pub const FAXSRV_DEVICE_NODETYPE_GUID: windows_core::GUID = windows_core::GUID::from_u128(0x3115a19a_6251_46ac_9425_14782858b8c9);
pub const FAXSRV_DEVICE_PROVIDER_NODETYPE_GUID: windows_core::GUID = windows_core::GUID::from_u128(0xbd38e2ac_b926_4161_8640_0f6956ee2ba3);
pub const FAXSRV_ROUTING_METHOD_NODETYPE_GUID: windows_core::GUID = windows_core::GUID::from_u128(0x220d2cb0_85a9_4a43_b6e8_9d66b44f1af5);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct FAX_ACCESS_RIGHTS_ENUM(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct FAX_ACCESS_RIGHTS_ENUM_2(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct FAX_ACCOUNT_EVENTS_TYPE_ENUM(pub i32);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct FAX_CONFIGURATIONA {
    pub SizeOfStruct: u32,
    pub Retries: u32,
    pub RetryDelay: u32,
    pub DirtyDays: u32,
    pub Branding: windows_core::BOOL,
    pub UseDeviceTsid: windows_core::BOOL,
    pub ServerCp: windows_core::BOOL,
    pub PauseServerQueue: windows_core::BOOL,
    pub StartCheapTime: FAX_TIME,
    pub StopCheapTime: FAX_TIME,
    pub ArchiveOutgoingFaxes: windows_core::BOOL,
    pub ArchiveDirectory: windows_core::PCSTR,
    pub Reserved: windows_core::PCSTR,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct FAX_CONFIGURATIONW {
    pub SizeOfStruct: u32,
    pub Retries: u32,
    pub RetryDelay: u32,
    pub DirtyDays: u32,
    pub Branding: windows_core::BOOL,
    pub UseDeviceTsid: windows_core::BOOL,
    pub ServerCp: windows_core::BOOL,
    pub PauseServerQueue: windows_core::BOOL,
    pub StartCheapTime: FAX_TIME,
    pub StopCheapTime: FAX_TIME,
    pub ArchiveOutgoingFaxes: windows_core::BOOL,
    pub ArchiveDirectory: windows_core::PCWSTR,
    pub Reserved: windows_core::PCWSTR,
}
pub const FAX_CONFIG_QUERY: u32 = 4u32;
pub const FAX_CONFIG_SET: u32 = 8u32;
#[repr(C)]
#[cfg(feature = "Win32_Graphics_Gdi")]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct FAX_CONTEXT_INFOA {
    pub SizeOfStruct: u32,
    pub hDC: super::super::Graphics::Gdi::HDC,
    pub ServerName: [i8; 16],
}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl Default for FAX_CONTEXT_INFOA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Graphics_Gdi")]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct FAX_CONTEXT_INFOW {
    pub SizeOfStruct: u32,
    pub hDC: super::super::Graphics::Gdi::HDC,
    pub ServerName: [u16; 16],
}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl Default for FAX_CONTEXT_INFOW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct FAX_COVERPAGE_INFOA {
    pub SizeOfStruct: u32,
    pub CoverPageName: windows_core::PCSTR,
    pub UseServerCoverPage: windows_core::BOOL,
    pub RecName: windows_core::PCSTR,
    pub RecFaxNumber: windows_core::PCSTR,
    pub RecCompany: windows_core::PCSTR,
    pub RecStreetAddress: windows_core::PCSTR,
    pub RecCity: windows_core::PCSTR,
    pub RecState: windows_core::PCSTR,
    pub RecZip: windows_core::PCSTR,
    pub RecCountry: windows_core::PCSTR,
    pub RecTitle: windows_core::PCSTR,
    pub RecDepartment: windows_core::PCSTR,
    pub RecOfficeLocation: windows_core::PCSTR,
    pub RecHomePhone: windows_core::PCSTR,
    pub RecOfficePhone: windows_core::PCSTR,
    pub SdrName: windows_core::PCSTR,
    pub SdrFaxNumber: windows_core::PCSTR,
    pub SdrCompany: windows_core::PCSTR,
    pub SdrAddress: windows_core::PCSTR,
    pub SdrTitle: windows_core::PCSTR,
    pub SdrDepartment: windows_core::PCSTR,
    pub SdrOfficeLocation: windows_core::PCSTR,
    pub SdrHomePhone: windows_core::PCSTR,
    pub SdrOfficePhone: windows_core::PCSTR,
    pub Note: windows_core::PCSTR,
    pub Subject: windows_core::PCSTR,
    pub TimeSent: super::super::Foundation::SYSTEMTIME,
    pub PageCount: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct FAX_COVERPAGE_INFOW {
    pub SizeOfStruct: u32,
    pub CoverPageName: windows_core::PCWSTR,
    pub UseServerCoverPage: windows_core::BOOL,
    pub RecName: windows_core::PCWSTR,
    pub RecFaxNumber: windows_core::PCWSTR,
    pub RecCompany: windows_core::PCWSTR,
    pub RecStreetAddress: windows_core::PCWSTR,
    pub RecCity: windows_core::PCWSTR,
    pub RecState: windows_core::PCWSTR,
    pub RecZip: windows_core::PCWSTR,
    pub RecCountry: windows_core::PCWSTR,
    pub RecTitle: windows_core::PCWSTR,
    pub RecDepartment: windows_core::PCWSTR,
    pub RecOfficeLocation: windows_core::PCWSTR,
    pub RecHomePhone: windows_core::PCWSTR,
    pub RecOfficePhone: windows_core::PCWSTR,
    pub SdrName: windows_core::PCWSTR,
    pub SdrFaxNumber: windows_core::PCWSTR,
    pub SdrCompany: windows_core::PCWSTR,
    pub SdrAddress: windows_core::PCWSTR,
    pub SdrTitle: windows_core::PCWSTR,
    pub SdrDepartment: windows_core::PCWSTR,
    pub SdrOfficeLocation: windows_core::PCWSTR,
    pub SdrHomePhone: windows_core::PCWSTR,
    pub SdrOfficePhone: windows_core::PCWSTR,
    pub Note: windows_core::PCWSTR,
    pub Subject: windows_core::PCWSTR,
    pub TimeSent: super::super::Foundation::SYSTEMTIME,
    pub PageCount: u32,
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct FAX_COVERPAGE_TYPE_ENUM(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct FAX_DEVICE_RECEIVE_MODE_ENUM(pub i32);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct FAX_DEVICE_STATUSA {
    pub SizeOfStruct: u32,
    pub CallerId: windows_core::PCSTR,
    pub Csid: windows_core::PCSTR,
    pub CurrentPage: u32,
    pub DeviceId: u32,
    pub DeviceName: windows_core::PCSTR,
    pub DocumentName: windows_core::PCSTR,
    pub JobType: u32,
    pub PhoneNumber: windows_core::PCSTR,
    pub RoutingString: windows_core::PCSTR,
    pub SenderName: windows_core::PCSTR,
    pub RecipientName: windows_core::PCSTR,
    pub Size: u32,
    pub StartTime: super::super::Foundation::FILETIME,
    pub Status: u32,
    pub StatusString: windows_core::PCSTR,
    pub SubmittedTime: super::super::Foundation::FILETIME,
    pub TotalPages: u32,
    pub Tsid: windows_core::PCSTR,
    pub UserName: windows_core::PCSTR,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct FAX_DEVICE_STATUSW {
    pub SizeOfStruct: u32,
    pub CallerId: windows_core::PCWSTR,
    pub Csid: windows_core::PCWSTR,
    pub CurrentPage: u32,
    pub DeviceId: u32,
    pub DeviceName: windows_core::PCWSTR,
    pub DocumentName: windows_core::PCWSTR,
    pub JobType: u32,
    pub PhoneNumber: windows_core::PCWSTR,
    pub RoutingString: windows_core::PCWSTR,
    pub SenderName: windows_core::PCWSTR,
    pub RecipientName: windows_core::PCWSTR,
    pub Size: u32,
    pub StartTime: super::super::Foundation::FILETIME,
    pub Status: u32,
    pub StatusString: windows_core::PCWSTR,
    pub SubmittedTime: super::super::Foundation::FILETIME,
    pub TotalPages: u32,
    pub Tsid: windows_core::PCWSTR,
    pub UserName: windows_core::PCWSTR,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct FAX_DEV_STATUS {
    pub SizeOfStruct: u32,
    pub StatusId: u32,
    pub StringId: u32,
    pub PageCount: u32,
    pub CSI: windows_core::PWSTR,
    pub CallerId: windows_core::PWSTR,
    pub RoutingInfo: windows_core::PWSTR,
    pub ErrorCode: u32,
    pub Reserved: [u32; 3],
}
impl Default for FAX_DEV_STATUS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct FAX_ENUM_DELIVERY_REPORT_TYPES(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct FAX_ENUM_DEVICE_ID_SOURCE(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct FAX_ENUM_JOB_COMMANDS(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct FAX_ENUM_JOB_SEND_ATTRIBUTES(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct FAX_ENUM_LOG_CATEGORIES(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct FAX_ENUM_LOG_LEVELS(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct FAX_ENUM_PORT_OPEN_TYPE(pub i32);
pub const FAX_ERR_BAD_GROUP_CONFIGURATION: i32 = 7003i32;
pub const FAX_ERR_DEVICE_NUM_LIMIT_EXCEEDED: i32 = 7010i32;
pub const FAX_ERR_DIRECTORY_IN_USE: i32 = 7007i32;
pub const FAX_ERR_END: i32 = 7013i32;
pub const FAX_ERR_FILE_ACCESS_DENIED: i32 = 7008i32;
pub const FAX_ERR_GROUP_IN_USE: i32 = 7004i32;
pub const FAX_ERR_GROUP_NOT_FOUND: i32 = 7002i32;
pub const FAX_ERR_MESSAGE_NOT_FOUND: i32 = 7009i32;
pub const FAX_ERR_NOT_NTFS: i32 = 7006i32;
pub const FAX_ERR_NOT_SUPPORTED_ON_THIS_SKU: i32 = 7011i32;
pub const FAX_ERR_RECIPIENTS_LIMIT: i32 = 7013i32;
pub const FAX_ERR_RULE_NOT_FOUND: i32 = 7005i32;
pub const FAX_ERR_SRV_OUTOFMEMORY: i32 = 7001i32;
pub const FAX_ERR_START: i32 = 7001i32;
pub const FAX_ERR_VERSION_MISMATCH: i32 = 7012i32;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct FAX_EVENTA {
    pub SizeOfStruct: u32,
    pub TimeStamp: super::super::Foundation::FILETIME,
    pub DeviceId: u32,
    pub EventId: u32,
    pub JobId: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct FAX_EVENTW {
    pub SizeOfStruct: u32,
    pub TimeStamp: super::super::Foundation::FILETIME,
    pub DeviceId: u32,
    pub EventId: u32,
    pub JobId: u32,
}
pub const FAX_E_BAD_GROUP_CONFIGURATION: windows_core::HRESULT = windows_core::HRESULT(0x80041B5B_u32 as _);
pub const FAX_E_DEVICE_NUM_LIMIT_EXCEEDED: windows_core::HRESULT = windows_core::HRESULT(0x80041B62_u32 as _);
pub const FAX_E_DIRECTORY_IN_USE: windows_core::HRESULT = windows_core::HRESULT(0x80041B5F_u32 as _);
pub const FAX_E_FILE_ACCESS_DENIED: windows_core::HRESULT = windows_core::HRESULT(0x80041B60_u32 as _);
pub const FAX_E_GROUP_IN_USE: windows_core::HRESULT = windows_core::HRESULT(0x80041B5C_u32 as _);
pub const FAX_E_GROUP_NOT_FOUND: windows_core::HRESULT = windows_core::HRESULT(0x80041B5A_u32 as _);
pub const FAX_E_MESSAGE_NOT_FOUND: windows_core::HRESULT = windows_core::HRESULT(0x80041B61_u32 as _);
pub const FAX_E_NOT_NTFS: windows_core::HRESULT = windows_core::HRESULT(0x80041B5E_u32 as _);
pub const FAX_E_NOT_SUPPORTED_ON_THIS_SKU: windows_core::HRESULT = windows_core::HRESULT(0x80041B63_u32 as _);
pub const FAX_E_RECIPIENTS_LIMIT: windows_core::HRESULT = windows_core::HRESULT(0x80041B65_u32 as _);
pub const FAX_E_RULE_NOT_FOUND: windows_core::HRESULT = windows_core::HRESULT(0x80041B5D_u32 as _);
pub const FAX_E_SRV_OUTOFMEMORY: windows_core::HRESULT = windows_core::HRESULT(0x80041B59_u32 as _);
pub const FAX_E_VERSION_MISMATCH: windows_core::HRESULT = windows_core::HRESULT(0x80041B64_u32 as _);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct FAX_GLOBAL_ROUTING_INFOA {
    pub SizeOfStruct: u32,
    pub Priority: u32,
    pub Guid: windows_core::PCSTR,
    pub FriendlyName: windows_core::PCSTR,
    pub FunctionName: windows_core::PCSTR,
    pub ExtensionImageName: windows_core::PCSTR,
    pub ExtensionFriendlyName: windows_core::PCSTR,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct FAX_GLOBAL_ROUTING_INFOW {
    pub SizeOfStruct: u32,
    pub Priority: u32,
    pub Guid: windows_core::PCWSTR,
    pub FriendlyName: windows_core::PCWSTR,
    pub FunctionName: windows_core::PCWSTR,
    pub ExtensionImageName: windows_core::PCWSTR,
    pub ExtensionFriendlyName: windows_core::PCWSTR,
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct FAX_GROUP_STATUS_ENUM(pub i32);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct FAX_JOB_ENTRYA {
    pub SizeOfStruct: u32,
    pub JobId: u32,
    pub UserName: windows_core::PCSTR,
    pub JobType: u32,
    pub QueueStatus: u32,
    pub Status: u32,
    pub Size: u32,
    pub PageCount: u32,
    pub RecipientNumber: windows_core::PCSTR,
    pub RecipientName: windows_core::PCSTR,
    pub Tsid: windows_core::PCSTR,
    pub SenderName: windows_core::PCSTR,
    pub SenderCompany: windows_core::PCSTR,
    pub SenderDept: windows_core::PCSTR,
    pub BillingCode: windows_core::PCSTR,
    pub ScheduleAction: u32,
    pub ScheduleTime: super::super::Foundation::SYSTEMTIME,
    pub DeliveryReportType: u32,
    pub DeliveryReportAddress: windows_core::PCSTR,
    pub DocumentName: windows_core::PCSTR,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct FAX_JOB_ENTRYW {
    pub SizeOfStruct: u32,
    pub JobId: u32,
    pub UserName: windows_core::PCWSTR,
    pub JobType: u32,
    pub QueueStatus: u32,
    pub Status: u32,
    pub Size: u32,
    pub PageCount: u32,
    pub RecipientNumber: windows_core::PCWSTR,
    pub RecipientName: windows_core::PCWSTR,
    pub Tsid: windows_core::PCWSTR,
    pub SenderName: windows_core::PCWSTR,
    pub SenderCompany: windows_core::PCWSTR,
    pub SenderDept: windows_core::PCWSTR,
    pub BillingCode: windows_core::PCWSTR,
    pub ScheduleAction: u32,
    pub ScheduleTime: super::super::Foundation::SYSTEMTIME,
    pub DeliveryReportType: u32,
    pub DeliveryReportAddress: windows_core::PCWSTR,
    pub DocumentName: windows_core::PCWSTR,
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct FAX_JOB_EXTENDED_STATUS_ENUM(pub i32);
pub const FAX_JOB_MANAGE: u32 = 64u32;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct FAX_JOB_OPERATIONS_ENUM(pub i32);
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct FAX_JOB_PARAMA {
    pub SizeOfStruct: u32,
    pub RecipientNumber: windows_core::PCSTR,
    pub RecipientName: windows_core::PCSTR,
    pub Tsid: windows_core::PCSTR,
    pub SenderName: windows_core::PCSTR,
    pub SenderCompany: windows_core::PCSTR,
    pub SenderDept: windows_core::PCSTR,
    pub BillingCode: windows_core::PCSTR,
    pub ScheduleAction: u32,
    pub ScheduleTime: super::super::Foundation::SYSTEMTIME,
    pub DeliveryReportType: u32,
    pub DeliveryReportAddress: windows_core::PCSTR,
    pub DocumentName: windows_core::PCSTR,
    pub CallHandle: u32,
    pub Reserved: [usize; 3],
}
impl Default for FAX_JOB_PARAMA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct FAX_JOB_PARAMW {
    pub SizeOfStruct: u32,
    pub RecipientNumber: windows_core::PCWSTR,
    pub RecipientName: windows_core::PCWSTR,
    pub Tsid: windows_core::PCWSTR,
    pub SenderName: windows_core::PCWSTR,
    pub SenderCompany: windows_core::PCWSTR,
    pub SenderDept: windows_core::PCWSTR,
    pub BillingCode: windows_core::PCWSTR,
    pub ScheduleAction: u32,
    pub ScheduleTime: super::super::Foundation::SYSTEMTIME,
    pub DeliveryReportType: u32,
    pub DeliveryReportAddress: windows_core::PCWSTR,
    pub DocumentName: windows_core::PCWSTR,
    pub CallHandle: u32,
    pub Reserved: [usize; 3],
}
impl Default for FAX_JOB_PARAMW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const FAX_JOB_QUERY: u32 = 2u32;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct FAX_JOB_STATUS_ENUM(pub i32);
pub const FAX_JOB_SUBMIT: u32 = 1u32;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct FAX_JOB_TYPE_ENUM(pub i32);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct FAX_LOG_CATEGORYA {
    pub Name: windows_core::PCSTR,
    pub Category: u32,
    pub Level: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct FAX_LOG_CATEGORYW {
    pub Name: windows_core::PCWSTR,
    pub Category: u32,
    pub Level: u32,
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct FAX_LOG_LEVEL_ENUM(pub i32);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct FAX_PORT_INFOA {
    pub SizeOfStruct: u32,
    pub DeviceId: u32,
    pub State: u32,
    pub Flags: u32,
    pub Rings: u32,
    pub Priority: u32,
    pub DeviceName: windows_core::PCSTR,
    pub Tsid: windows_core::PCSTR,
    pub Csid: windows_core::PCSTR,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct FAX_PORT_INFOW {
    pub SizeOfStruct: u32,
    pub DeviceId: u32,
    pub State: u32,
    pub Flags: u32,
    pub Rings: u32,
    pub Priority: u32,
    pub DeviceName: windows_core::PCWSTR,
    pub Tsid: windows_core::PCWSTR,
    pub Csid: windows_core::PCWSTR,
}
pub const FAX_PORT_QUERY: u32 = 16u32;
pub const FAX_PORT_SET: u32 = 32u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct FAX_PRINT_INFOA {
    pub SizeOfStruct: u32,
    pub DocName: windows_core::PCSTR,
    pub RecipientName: windows_core::PCSTR,
    pub RecipientNumber: windows_core::PCSTR,
    pub SenderName: windows_core::PCSTR,
    pub SenderCompany: windows_core::PCSTR,
    pub SenderDept: windows_core::PCSTR,
    pub SenderBillingCode: windows_core::PCSTR,
    pub Reserved: windows_core::PCSTR,
    pub DrEmailAddress: windows_core::PCSTR,
    pub OutputFileName: windows_core::PCSTR,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct FAX_PRINT_INFOW {
    pub SizeOfStruct: u32,
    pub DocName: windows_core::PCWSTR,
    pub RecipientName: windows_core::PCWSTR,
    pub RecipientNumber: windows_core::PCWSTR,
    pub SenderName: windows_core::PCWSTR,
    pub SenderCompany: windows_core::PCWSTR,
    pub SenderDept: windows_core::PCWSTR,
    pub SenderBillingCode: windows_core::PCWSTR,
    pub Reserved: windows_core::PCWSTR,
    pub DrEmailAddress: windows_core::PCWSTR,
    pub OutputFileName: windows_core::PCWSTR,
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct FAX_PRIORITY_TYPE_ENUM(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct FAX_PROVIDER_STATUS_ENUM(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct FAX_RECEIPT_TYPE_ENUM(pub i32);
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct FAX_RECEIVE {
    pub SizeOfStruct: u32,
    pub FileName: windows_core::PWSTR,
    pub ReceiverName: windows_core::PWSTR,
    pub ReceiverNumber: windows_core::PWSTR,
    pub Reserved: [u32; 4],
}
impl Default for FAX_RECEIVE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct FAX_ROUTE {
    pub SizeOfStruct: u32,
    pub JobId: u32,
    pub ElapsedTime: u64,
    pub ReceiveTime: u64,
    pub PageCount: u32,
    pub Csid: windows_core::PCWSTR,
    pub Tsid: windows_core::PCWSTR,
    pub CallerId: windows_core::PCWSTR,
    pub RoutingInfo: windows_core::PCWSTR,
    pub ReceiverName: windows_core::PCWSTR,
    pub ReceiverNumber: windows_core::PCWSTR,
    pub DeviceName: windows_core::PCWSTR,
    pub DeviceId: u32,
    pub RoutingInfoData: *mut u8,
    pub RoutingInfoDataSize: u32,
}
impl Default for FAX_ROUTE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default)]
pub struct FAX_ROUTE_CALLBACKROUTINES {
    pub SizeOfStruct: u32,
    pub FaxRouteAddFile: PFAXROUTEADDFILE,
    pub FaxRouteDeleteFile: PFAXROUTEDELETEFILE,
    pub FaxRouteGetFile: PFAXROUTEGETFILE,
    pub FaxRouteEnumFiles: PFAXROUTEENUMFILES,
    pub FaxRouteModifyRoutingData: PFAXROUTEMODIFYROUTINGDATA,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct FAX_ROUTING_METHODA {
    pub SizeOfStruct: u32,
    pub DeviceId: u32,
    pub Enabled: windows_core::BOOL,
    pub DeviceName: windows_core::PCSTR,
    pub Guid: windows_core::PCSTR,
    pub FriendlyName: windows_core::PCSTR,
    pub FunctionName: windows_core::PCSTR,
    pub ExtensionImageName: windows_core::PCSTR,
    pub ExtensionFriendlyName: windows_core::PCSTR,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct FAX_ROUTING_METHODW {
    pub SizeOfStruct: u32,
    pub DeviceId: u32,
    pub Enabled: windows_core::BOOL,
    pub DeviceName: windows_core::PCWSTR,
    pub Guid: windows_core::PCWSTR,
    pub FriendlyName: windows_core::PCWSTR,
    pub FunctionName: windows_core::PCWSTR,
    pub ExtensionImageName: windows_core::PCWSTR,
    pub ExtensionFriendlyName: windows_core::PCWSTR,
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct FAX_ROUTING_RULE_CODE_ENUM(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct FAX_RULE_STATUS_ENUM(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct FAX_SCHEDULE_TYPE_ENUM(pub i32);
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct FAX_SEND {
    pub SizeOfStruct: u32,
    pub FileName: windows_core::PWSTR,
    pub CallerName: windows_core::PWSTR,
    pub CallerNumber: windows_core::PWSTR,
    pub ReceiverName: windows_core::PWSTR,
    pub ReceiverNumber: windows_core::PWSTR,
    pub Branding: windows_core::BOOL,
    pub CallHandle: u32,
    pub Reserved: [u32; 3],
}
impl Default for FAX_SEND {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct FAX_SERVER_APIVERSION_ENUM(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct FAX_SERVER_EVENTS_TYPE_ENUM(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct FAX_SMTP_AUTHENTICATION_TYPE_ENUM(pub i32);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct FAX_TIME {
    pub Hour: u16,
    pub Minute: u16,
}
pub const FEI_ABORTING: u32 = 15u32;
pub const FEI_ANSWERED: u32 = 21u32;
pub const FEI_BAD_ADDRESS: u32 = 7u32;
pub const FEI_BUSY: u32 = 5u32;
pub const FEI_CALL_BLACKLISTED: u32 = 13u32;
pub const FEI_CALL_DELAYED: u32 = 12u32;
pub const FEI_COMPLETED: u32 = 4u32;
pub const FEI_DELETED: u32 = 23u32;
pub const FEI_DIALING: u32 = 1u32;
pub const FEI_DISCONNECTED: u32 = 9u32;
pub const FEI_FATAL_ERROR: u32 = 10u32;
pub const FEI_FAXSVC_ENDED: u32 = 20u32;
pub const FEI_FAXSVC_STARTED: u32 = 27u32;
pub const FEI_HANDLED: u32 = 26u32;
pub const FEI_IDLE: u32 = 19u32;
pub const FEI_INITIALIZING: u32 = 24u32;
pub const FEI_JOB_QUEUED: u32 = 22u32;
pub const FEI_LINE_UNAVAILABLE: u32 = 25u32;
pub const FEI_MODEM_POWERED_OFF: u32 = 18u32;
pub const FEI_MODEM_POWERED_ON: u32 = 17u32;
pub const FEI_NEVENTS: u32 = 27u32;
pub const FEI_NOT_FAX_CALL: u32 = 11u32;
pub const FEI_NO_ANSWER: u32 = 6u32;
pub const FEI_NO_DIAL_TONE: u32 = 8u32;
pub const FEI_RECEIVING: u32 = 3u32;
pub const FEI_RINGING: u32 = 14u32;
pub const FEI_ROUTING: u32 = 16u32;
pub const FEI_SENDING: u32 = 2u32;
pub const FPF_RECEIVE: u32 = 1u32;
pub const FPF_SEND: u32 = 2u32;
pub const FPF_VIRTUAL: u32 = 4u32;
pub const FPS_ABORTING: u32 = 538968064u32;
pub const FPS_ANSWERED: u32 = 545259520u32;
pub const FPS_AVAILABLE: u32 = 537919488u32;
pub const FPS_BAD_ADDRESS: u32 = 536871168u32;
pub const FPS_BUSY: u32 = 536870976u32;
pub const FPS_CALL_BLACKLISTED: u32 = 536887296u32;
pub const FPS_CALL_DELAYED: u32 = 536879104u32;
pub const FPS_COMPLETED: u32 = 536870920u32;
pub const FPS_DIALING: u32 = 536870913u32;
pub const FPS_DISCONNECTED: u32 = 536871936u32;
pub const FPS_FATAL_ERROR: u32 = 536872960u32;
pub const FPS_HANDLED: u32 = 536870928u32;
pub const FPS_INITIALIZING: u32 = 536903680u32;
pub const FPS_NOT_FAX_CALL: u32 = 536875008u32;
pub const FPS_NO_ANSWER: u32 = 536871040u32;
pub const FPS_NO_DIAL_TONE: u32 = 536871424u32;
pub const FPS_OFFLINE: u32 = 536936448u32;
pub const FPS_RECEIVING: u32 = 536870916u32;
pub const FPS_RINGING: u32 = 537001984u32;
pub const FPS_ROUTING: u32 = 541065216u32;
pub const FPS_SENDING: u32 = 536870914u32;
pub const FPS_UNAVAILABLE: u32 = 536870944u32;
pub const FS_ANSWERED: u32 = 545259520u32;
pub const FS_BAD_ADDRESS: u32 = 536871168u32;
pub const FS_BUSY: u32 = 536870976u32;
pub const FS_CALL_BLACKLISTED: u32 = 536887296u32;
pub const FS_CALL_DELAYED: u32 = 536879104u32;
pub const FS_COMPLETED: u32 = 536870920u32;
pub const FS_DIALING: u32 = 536870913u32;
pub const FS_DISCONNECTED: u32 = 536871936u32;
pub const FS_FATAL_ERROR: u32 = 536872960u32;
pub const FS_HANDLED: u32 = 536870928u32;
pub const FS_INITIALIZING: u32 = 536870912u32;
pub const FS_LINE_UNAVAILABLE: u32 = 536870944u32;
pub const FS_NOT_FAX_CALL: u32 = 536875008u32;
pub const FS_NO_ANSWER: u32 = 536871040u32;
pub const FS_NO_DIAL_TONE: u32 = 536871424u32;
pub const FS_RECEIVING: u32 = 536870916u32;
pub const FS_TRANSMITTING: u32 = 536870914u32;
pub const FS_USER_ABORT: u32 = 538968064u32;
pub const FaxAccount: windows_core::GUID = windows_core::GUID::from_u128(0xa7e0647f_4524_4464_a56d_b9fe666f715e);
pub const FaxAccountFolders: windows_core::GUID = windows_core::GUID::from_u128(0x85398f49_c034_4a3f_821c_db7d685e8129);
pub const FaxAccountIncomingArchive: windows_core::GUID = windows_core::GUID::from_u128(0x14b33db5_4c40_4ecf_9ef8_a360cbe809ed);
pub const FaxAccountIncomingQueue: windows_core::GUID = windows_core::GUID::from_u128(0x9bcf6094_b4da_45f4_b8d6_ddeb2186652c);
pub const FaxAccountOutgoingArchive: windows_core::GUID = windows_core::GUID::from_u128(0x851e7af5_433a_4739_a2df_ad245c2cb98e);
pub const FaxAccountOutgoingQueue: windows_core::GUID = windows_core::GUID::from_u128(0xfeeceefb_c149_48ba_bab8_b791e101f62f);
pub const FaxAccountSet: windows_core::GUID = windows_core::GUID::from_u128(0xfbc23c4b_79e0_4291_bc56_c12e253bbf3a);
pub const FaxAccounts: windows_core::GUID = windows_core::GUID::from_u128(0xda1f94aa_ee2c_47c0_8f4f_2a217075b76e);
pub const FaxActivity: windows_core::GUID = windows_core::GUID::from_u128(0xcfef5d0e_e84d_462e_aabb_87d31eb04fef);
pub const FaxActivityLogging: windows_core::GUID = windows_core::GUID::from_u128(0xf0a0294e_3bbd_48b8_8f13_8c591a55bdbc);
pub const FaxConfiguration: windows_core::GUID = windows_core::GUID::from_u128(0x5857326f_e7b3_41a7_9c19_a91b463e2d56);
pub const FaxDevice: windows_core::GUID = windows_core::GUID::from_u128(0x59e3a5b2_d676_484b_a6de_720bfa89b5af);
pub const FaxDeviceIds: windows_core::GUID = windows_core::GUID::from_u128(0xcdc539ea_7277_460e_8de0_48a0a5760d1f);
pub const FaxDeviceProvider: windows_core::GUID = windows_core::GUID::from_u128(0x17cf1aa3_f5eb_484a_9c9a_4440a5baabfc);
pub const FaxDeviceProviders: windows_core::GUID = windows_core::GUID::from_u128(0xeb8fe768_875a_4f5f_82c5_03f23aac1bd7);
pub const FaxDevices: windows_core::GUID = windows_core::GUID::from_u128(0x5589e28e_23cb_4919_8808_e6101846e80d);
pub const FaxDocument: windows_core::GUID = windows_core::GUID::from_u128(0x0f3f9f91_c838_415e_a4f3_3e828ca445e0);
pub const FaxEventLogging: windows_core::GUID = windows_core::GUID::from_u128(0xa6850930_a0f6_4a6f_95b7_db2ebf3d02e3);
pub const FaxFolders: windows_core::GUID = windows_core::GUID::from_u128(0xc35211d7_5776_48cb_af44_c31be3b2cfe5);
pub const FaxInboundRouting: windows_core::GUID = windows_core::GUID::from_u128(0xe80248ed_ad65_4218_8108_991924d4e7ed);
pub const FaxInboundRoutingExtension: windows_core::GUID = windows_core::GUID::from_u128(0x1d7dfb51_7207_4436_a0d9_24e32ee56988);
pub const FaxInboundRoutingExtensions: windows_core::GUID = windows_core::GUID::from_u128(0x189a48ed_623c_4c0d_80f2_d66c7b9efec2);
pub const FaxInboundRoutingMethod: windows_core::GUID = windows_core::GUID::from_u128(0x4b9fd75c_0194_4b72_9ce5_02a8205ac7d4);
pub const FaxInboundRoutingMethods: windows_core::GUID = windows_core::GUID::from_u128(0x25fcb76a_b750_4b82_9266_fbbbae8922ba);
pub const FaxIncomingArchive: windows_core::GUID = windows_core::GUID::from_u128(0x8426c56a_35a1_4c6f_af93_fc952422e2c2);
pub const FaxIncomingJob: windows_core::GUID = windows_core::GUID::from_u128(0xc47311ec_ae32_41b8_ae4b_3eae0629d0c9);
pub const FaxIncomingJobs: windows_core::GUID = windows_core::GUID::from_u128(0xa1bb8a43_8866_4fb7_a15d_6266c875a5cc);
pub const FaxIncomingMessage: windows_core::GUID = windows_core::GUID::from_u128(0x1932fcf7_9d43_4d5a_89ff_03861b321736);
pub const FaxIncomingMessageIterator: windows_core::GUID = windows_core::GUID::from_u128(0x6088e1d8_3fc8_45c2_87b1_909a29607ea9);
pub const FaxIncomingQueue: windows_core::GUID = windows_core::GUID::from_u128(0x69131717_f3f1_40e3_809d_a6cbf7bd85e5);
pub const FaxJobStatus: windows_core::GUID = windows_core::GUID::from_u128(0x7bf222f4_be8d_442f_841d_6132742423bb);
pub const FaxLoggingOptions: windows_core::GUID = windows_core::GUID::from_u128(0x1bf9eea6_ece0_4785_a18b_de56e9eef96a);
pub const FaxOutboundRouting: windows_core::GUID = windows_core::GUID::from_u128(0xc81b385e_b869_4afd_86c0_616498ed9be2);
pub const FaxOutboundRoutingGroup: windows_core::GUID = windows_core::GUID::from_u128(0x0213f3e0_6791_4d77_a271_04d2357c50d6);
pub const FaxOutboundRoutingGroups: windows_core::GUID = windows_core::GUID::from_u128(0xccbea1a5_e2b4_4b57_9421_b04b6289464b);
pub const FaxOutboundRoutingRule: windows_core::GUID = windows_core::GUID::from_u128(0x6549eebf_08d1_475a_828b_3bf105952fa0);
pub const FaxOutboundRoutingRules: windows_core::GUID = windows_core::GUID::from_u128(0xd385beca_e624_4473_bfaa_9f4000831f54);
pub const FaxOutgoingArchive: windows_core::GUID = windows_core::GUID::from_u128(0x43c28403_e04f_474d_990c_b94669148f59);
pub const FaxOutgoingJob: windows_core::GUID = windows_core::GUID::from_u128(0x71bb429c_0ef9_4915_bec5_a5d897a3e924);
pub const FaxOutgoingJobs: windows_core::GUID = windows_core::GUID::from_u128(0x92bf2a6c_37be_43fa_a37d_cb0e5f753b35);
pub const FaxOutgoingMessage: windows_core::GUID = windows_core::GUID::from_u128(0x91b4a378_4ad8_4aef_a4dc_97d96e939a3a);
pub const FaxOutgoingMessageIterator: windows_core::GUID = windows_core::GUID::from_u128(0x8a3224d0_d30b_49de_9813_cb385790fbbb);
pub const FaxOutgoingQueue: windows_core::GUID = windows_core::GUID::from_u128(0x7421169e_8c43_4b0d_bb16_645c8fa40357);
pub const FaxReceiptOptions: windows_core::GUID = windows_core::GUID::from_u128(0x6982487b_227b_4c96_a61c_248348b05ab6);
pub const FaxRecipient: windows_core::GUID = windows_core::GUID::from_u128(0x60bf3301_7df8_4bd8_9148_7b5801f9efdf);
pub const FaxRecipients: windows_core::GUID = windows_core::GUID::from_u128(0xea9bdf53_10a9_4d4f_a067_63c8f84f01b0);
pub const FaxSecurity: windows_core::GUID = windows_core::GUID::from_u128(0x10c4ddde_abf0_43df_964f_7f3ac21a4c7b);
pub const FaxSecurity2: windows_core::GUID = windows_core::GUID::from_u128(0x735c1248_ec89_4c30_a127_656e92e3c4ea);
pub const FaxSender: windows_core::GUID = windows_core::GUID::from_u128(0x265d84d0_1850_4360_b7c8_758bbb5f0b96);
pub const FaxServer: windows_core::GUID = windows_core::GUID::from_u128(0xcda8acb0_8cf5_4f6c_9ba2_5931d40c8cae);
pub const GUID_DeviceArrivedLaunch: windows_core::GUID = windows_core::GUID::from_u128(0x740d9ee6_70f1_11d1_ad10_00a02438ad48);
pub const GUID_STIUserDefined1: windows_core::GUID = windows_core::GUID::from_u128(0xc00eb795_8c6e_11d2_977a_0000f87a926f);
pub const GUID_STIUserDefined2: windows_core::GUID = windows_core::GUID::from_u128(0xc77ae9c5_8c6e_11d2_977a_0000f87a926f);
pub const GUID_STIUserDefined3: windows_core::GUID = windows_core::GUID::from_u128(0xc77ae9c6_8c6e_11d2_977a_0000f87a926f);
pub const GUID_ScanFaxImage: windows_core::GUID = windows_core::GUID::from_u128(0xc00eb793_8c6e_11d2_977a_0000f87a926f);
pub const GUID_ScanImage: windows_core::GUID = windows_core::GUID::from_u128(0xa6c5a715_8c6e_11d2_977a_0000f87a926f);
pub const GUID_ScanPrintImage: windows_core::GUID = windows_core::GUID::from_u128(0xb441f425_8c6e_11d2_977a_0000f87a926f);
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IFaxAccount, IFaxAccount_Vtbl, 0x68535b33_5dc4_4086_be26_b76f9b711006);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IFaxAccount {
    type Target = super::super::System::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IFaxAccount, windows_core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl IFaxAccount {
    pub unsafe fn AccountName(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).AccountName)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn Folders(&self) -> windows_core::Result<IFaxAccountFolders> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Folders)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn ListenToAccountEvents(&self, eventtypes: FAX_ACCOUNT_EVENTS_TYPE_ENUM) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).ListenToAccountEvents)(windows_core::Interface::as_raw(self), eventtypes).ok() }
    }
    pub unsafe fn RegisteredEvents(&self) -> windows_core::Result<FAX_ACCOUNT_EVENTS_TYPE_ENUM> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).RegisteredEvents)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IFaxAccount_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub AccountName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Folders: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ListenToAccountEvents: unsafe extern "system" fn(*mut core::ffi::c_void, FAX_ACCOUNT_EVENTS_TYPE_ENUM) -> windows_core::HRESULT,
    pub RegisteredEvents: unsafe extern "system" fn(*mut core::ffi::c_void, *mut FAX_ACCOUNT_EVENTS_TYPE_ENUM) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IFaxAccount_Impl: super::super::System::Com::IDispatch_Impl {
    fn AccountName(&self) -> windows_core::Result<windows_core::BSTR>;
    fn Folders(&self) -> windows_core::Result<IFaxAccountFolders>;
    fn ListenToAccountEvents(&self, eventtypes: FAX_ACCOUNT_EVENTS_TYPE_ENUM) -> windows_core::Result<()>;
    fn RegisteredEvents(&self) -> windows_core::Result<FAX_ACCOUNT_EVENTS_TYPE_ENUM>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IFaxAccount_Vtbl {
    pub const fn new<Identity: IFaxAccount_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn AccountName<Identity: IFaxAccount_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstraccountname: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFaxAccount_Impl::AccountName(this) {
                    Ok(ok__) => {
                        pbstraccountname.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Folders<Identity: IFaxAccount_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppfolders: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFaxAccount_Impl::Folders(this) {
                    Ok(ok__) => {
                        ppfolders.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn ListenToAccountEvents<Identity: IFaxAccount_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, eventtypes: FAX_ACCOUNT_EVENTS_TYPE_ENUM) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFaxAccount_Impl::ListenToAccountEvents(this, core::mem::transmute_copy(&eventtypes)).into()
            }
        }
        unsafe extern "system" fn RegisteredEvents<Identity: IFaxAccount_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pregisteredevents: *mut FAX_ACCOUNT_EVENTS_TYPE_ENUM) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFaxAccount_Impl::RegisteredEvents(this) {
                    Ok(ok__) => {
                        pregisteredevents.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            AccountName: AccountName::<Identity, OFFSET>,
            Folders: Folders::<Identity, OFFSET>,
            ListenToAccountEvents: ListenToAccountEvents::<Identity, OFFSET>,
            RegisteredEvents: RegisteredEvents::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IFaxAccount as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for IFaxAccount {}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IFaxAccountFolders, IFaxAccountFolders_Vtbl, 0x6463f89d_23d8_46a9_8f86_c47b77ca7926);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IFaxAccountFolders {
    type Target = super::super::System::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IFaxAccountFolders, windows_core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl IFaxAccountFolders {
    pub unsafe fn OutgoingQueue(&self) -> windows_core::Result<IFaxAccountOutgoingQueue> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).OutgoingQueue)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn IncomingQueue(&self) -> windows_core::Result<IFaxAccountIncomingQueue> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).IncomingQueue)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn IncomingArchive(&self) -> windows_core::Result<IFaxAccountIncomingArchive> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).IncomingArchive)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn OutgoingArchive(&self) -> windows_core::Result<IFaxAccountOutgoingArchive> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).OutgoingArchive)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IFaxAccountFolders_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub OutgoingQueue: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub IncomingQueue: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub IncomingArchive: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub OutgoingArchive: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IFaxAccountFolders_Impl: super::super::System::Com::IDispatch_Impl {
    fn OutgoingQueue(&self) -> windows_core::Result<IFaxAccountOutgoingQueue>;
    fn IncomingQueue(&self) -> windows_core::Result<IFaxAccountIncomingQueue>;
    fn IncomingArchive(&self) -> windows_core::Result<IFaxAccountIncomingArchive>;
    fn OutgoingArchive(&self) -> windows_core::Result<IFaxAccountOutgoingArchive>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IFaxAccountFolders_Vtbl {
    pub const fn new<Identity: IFaxAccountFolders_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn OutgoingQueue<Identity: IFaxAccountFolders_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pfaxoutgoingqueue: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFaxAccountFolders_Impl::OutgoingQueue(this) {
                    Ok(ok__) => {
                        pfaxoutgoingqueue.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn IncomingQueue<Identity: IFaxAccountFolders_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pfaxincomingqueue: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFaxAccountFolders_Impl::IncomingQueue(this) {
                    Ok(ok__) => {
                        pfaxincomingqueue.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn IncomingArchive<Identity: IFaxAccountFolders_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pfaxincomingarchive: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFaxAccountFolders_Impl::IncomingArchive(this) {
                    Ok(ok__) => {
                        pfaxincomingarchive.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn OutgoingArchive<Identity: IFaxAccountFolders_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pfaxoutgoingarchive: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFaxAccountFolders_Impl::OutgoingArchive(this) {
                    Ok(ok__) => {
                        pfaxoutgoingarchive.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            OutgoingQueue: OutgoingQueue::<Identity, OFFSET>,
            IncomingQueue: IncomingQueue::<Identity, OFFSET>,
            IncomingArchive: IncomingArchive::<Identity, OFFSET>,
            OutgoingArchive: OutgoingArchive::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IFaxAccountFolders as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for IFaxAccountFolders {}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IFaxAccountIncomingArchive, IFaxAccountIncomingArchive_Vtbl, 0xa8a5b6ef_e0d6_4aee_955c_91625bec9db4);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IFaxAccountIncomingArchive {
    type Target = super::super::System::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IFaxAccountIncomingArchive, windows_core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl IFaxAccountIncomingArchive {
    pub unsafe fn SizeLow(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).SizeLow)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SizeHigh(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).SizeHigh)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn Refresh(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Refresh)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn GetMessages(&self, lprefetchsize: i32) -> windows_core::Result<IFaxIncomingMessageIterator> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetMessages)(windows_core::Interface::as_raw(self), lprefetchsize, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetMessage(&self, bstrmessageid: &windows_core::BSTR) -> windows_core::Result<IFaxIncomingMessage> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetMessage)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrmessageid), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IFaxAccountIncomingArchive_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub SizeLow: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SizeHigh: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub Refresh: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetMessages: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetMessage: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IFaxAccountIncomingArchive_Impl: super::super::System::Com::IDispatch_Impl {
    fn SizeLow(&self) -> windows_core::Result<i32>;
    fn SizeHigh(&self) -> windows_core::Result<i32>;
    fn Refresh(&self) -> windows_core::Result<()>;
    fn GetMessages(&self, lprefetchsize: i32) -> windows_core::Result<IFaxIncomingMessageIterator>;
    fn GetMessage(&self, bstrmessageid: &windows_core::BSTR) -> windows_core::Result<IFaxIncomingMessage>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IFaxAccountIncomingArchive_Vtbl {
    pub const fn new<Identity: IFaxAccountIncomingArchive_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn SizeLow<Identity: IFaxAccountIncomingArchive_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, plsizelow: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFaxAccountIncomingArchive_Impl::SizeLow(this) {
                    Ok(ok__) => {
                        plsizelow.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SizeHigh<Identity: IFaxAccountIncomingArchive_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, plsizehigh: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFaxAccountIncomingArchive_Impl::SizeHigh(this) {
                    Ok(ok__) => {
                        plsizehigh.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Refresh<Identity: IFaxAccountIncomingArchive_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFaxAccountIncomingArchive_Impl::Refresh(this).into()
            }
        }
        unsafe extern "system" fn GetMessages<Identity: IFaxAccountIncomingArchive_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lprefetchsize: i32, pfaxincomingmessageiterator: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFaxAccountIncomingArchive_Impl::GetMessages(this, core::mem::transmute_copy(&lprefetchsize)) {
                    Ok(ok__) => {
                        pfaxincomingmessageiterator.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetMessage<Identity: IFaxAccountIncomingArchive_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrmessageid: *mut core::ffi::c_void, pfaxincomingmessage: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFaxAccountIncomingArchive_Impl::GetMessage(this, core::mem::transmute(&bstrmessageid)) {
                    Ok(ok__) => {
                        pfaxincomingmessage.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            SizeLow: SizeLow::<Identity, OFFSET>,
            SizeHigh: SizeHigh::<Identity, OFFSET>,
            Refresh: Refresh::<Identity, OFFSET>,
            GetMessages: GetMessages::<Identity, OFFSET>,
            GetMessage: GetMessage::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IFaxAccountIncomingArchive as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for IFaxAccountIncomingArchive {}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IFaxAccountIncomingQueue, IFaxAccountIncomingQueue_Vtbl, 0xdd142d92_0186_4a95_a090_cbc3eadba6b4);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IFaxAccountIncomingQueue {
    type Target = super::super::System::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IFaxAccountIncomingQueue, windows_core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl IFaxAccountIncomingQueue {
    pub unsafe fn GetJobs(&self) -> windows_core::Result<IFaxIncomingJobs> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetJobs)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetJob(&self, bstrjobid: &windows_core::BSTR) -> windows_core::Result<IFaxIncomingJob> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetJob)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrjobid), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IFaxAccountIncomingQueue_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub GetJobs: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetJob: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IFaxAccountIncomingQueue_Impl: super::super::System::Com::IDispatch_Impl {
    fn GetJobs(&self) -> windows_core::Result<IFaxIncomingJobs>;
    fn GetJob(&self, bstrjobid: &windows_core::BSTR) -> windows_core::Result<IFaxIncomingJob>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IFaxAccountIncomingQueue_Vtbl {
    pub const fn new<Identity: IFaxAccountIncomingQueue_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetJobs<Identity: IFaxAccountIncomingQueue_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pfaxincomingjobs: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFaxAccountIncomingQueue_Impl::GetJobs(this) {
                    Ok(ok__) => {
                        pfaxincomingjobs.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetJob<Identity: IFaxAccountIncomingQueue_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrjobid: *mut core::ffi::c_void, pfaxincomingjob: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFaxAccountIncomingQueue_Impl::GetJob(this, core::mem::transmute(&bstrjobid)) {
                    Ok(ok__) => {
                        pfaxincomingjob.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            GetJobs: GetJobs::<Identity, OFFSET>,
            GetJob: GetJob::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IFaxAccountIncomingQueue as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for IFaxAccountIncomingQueue {}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IFaxAccountNotify, IFaxAccountNotify_Vtbl, 0xb9b3bc81_ac1b_46f3_b39d_0adc30e1b788);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IFaxAccountNotify {
    type Target = super::super::System::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IFaxAccountNotify, windows_core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl IFaxAccountNotify {
    pub unsafe fn OnIncomingJobAdded<P0>(&self, pfaxaccount: P0, bstrjobid: &windows_core::BSTR) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IFaxAccount>,
    {
        unsafe { (windows_core::Interface::vtable(self).OnIncomingJobAdded)(windows_core::Interface::as_raw(self), pfaxaccount.param().abi(), core::mem::transmute_copy(bstrjobid)).ok() }
    }
    pub unsafe fn OnIncomingJobRemoved<P0>(&self, pfaxaccount: P0, bstrjobid: &windows_core::BSTR) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IFaxAccount>,
    {
        unsafe { (windows_core::Interface::vtable(self).OnIncomingJobRemoved)(windows_core::Interface::as_raw(self), pfaxaccount.param().abi(), core::mem::transmute_copy(bstrjobid)).ok() }
    }
    pub unsafe fn OnIncomingJobChanged<P0, P2>(&self, pfaxaccount: P0, bstrjobid: &windows_core::BSTR, pjobstatus: P2) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IFaxAccount>,
        P2: windows_core::Param<IFaxJobStatus>,
    {
        unsafe { (windows_core::Interface::vtable(self).OnIncomingJobChanged)(windows_core::Interface::as_raw(self), pfaxaccount.param().abi(), core::mem::transmute_copy(bstrjobid), pjobstatus.param().abi()).ok() }
    }
    pub unsafe fn OnOutgoingJobAdded<P0>(&self, pfaxaccount: P0, bstrjobid: &windows_core::BSTR) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IFaxAccount>,
    {
        unsafe { (windows_core::Interface::vtable(self).OnOutgoingJobAdded)(windows_core::Interface::as_raw(self), pfaxaccount.param().abi(), core::mem::transmute_copy(bstrjobid)).ok() }
    }
    pub unsafe fn OnOutgoingJobRemoved<P0>(&self, pfaxaccount: P0, bstrjobid: &windows_core::BSTR) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IFaxAccount>,
    {
        unsafe { (windows_core::Interface::vtable(self).OnOutgoingJobRemoved)(windows_core::Interface::as_raw(self), pfaxaccount.param().abi(), core::mem::transmute_copy(bstrjobid)).ok() }
    }
    pub unsafe fn OnOutgoingJobChanged<P0, P2>(&self, pfaxaccount: P0, bstrjobid: &windows_core::BSTR, pjobstatus: P2) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IFaxAccount>,
        P2: windows_core::Param<IFaxJobStatus>,
    {
        unsafe { (windows_core::Interface::vtable(self).OnOutgoingJobChanged)(windows_core::Interface::as_raw(self), pfaxaccount.param().abi(), core::mem::transmute_copy(bstrjobid), pjobstatus.param().abi()).ok() }
    }
    pub unsafe fn OnIncomingMessageAdded<P0>(&self, pfaxaccount: P0, bstrmessageid: &windows_core::BSTR, faddedtoreceivefolder: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IFaxAccount>,
    {
        unsafe { (windows_core::Interface::vtable(self).OnIncomingMessageAdded)(windows_core::Interface::as_raw(self), pfaxaccount.param().abi(), core::mem::transmute_copy(bstrmessageid), faddedtoreceivefolder).ok() }
    }
    pub unsafe fn OnIncomingMessageRemoved<P0>(&self, pfaxaccount: P0, bstrmessageid: &windows_core::BSTR, fremovedfromreceivefolder: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IFaxAccount>,
    {
        unsafe { (windows_core::Interface::vtable(self).OnIncomingMessageRemoved)(windows_core::Interface::as_raw(self), pfaxaccount.param().abi(), core::mem::transmute_copy(bstrmessageid), fremovedfromreceivefolder).ok() }
    }
    pub unsafe fn OnOutgoingMessageAdded<P0>(&self, pfaxaccount: P0, bstrmessageid: &windows_core::BSTR) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IFaxAccount>,
    {
        unsafe { (windows_core::Interface::vtable(self).OnOutgoingMessageAdded)(windows_core::Interface::as_raw(self), pfaxaccount.param().abi(), core::mem::transmute_copy(bstrmessageid)).ok() }
    }
    pub unsafe fn OnOutgoingMessageRemoved<P0>(&self, pfaxaccount: P0, bstrmessageid: &windows_core::BSTR) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IFaxAccount>,
    {
        unsafe { (windows_core::Interface::vtable(self).OnOutgoingMessageRemoved)(windows_core::Interface::as_raw(self), pfaxaccount.param().abi(), core::mem::transmute_copy(bstrmessageid)).ok() }
    }
    pub unsafe fn OnServerShutDown<P0>(&self, pfaxserver: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IFaxServer2>,
    {
        unsafe { (windows_core::Interface::vtable(self).OnServerShutDown)(windows_core::Interface::as_raw(self), pfaxserver.param().abi()).ok() }
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IFaxAccountNotify_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub OnIncomingJobAdded: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub OnIncomingJobRemoved: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub OnIncomingJobChanged: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub OnOutgoingJobAdded: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub OnOutgoingJobRemoved: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub OnOutgoingJobChanged: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub OnIncomingMessageAdded: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub OnIncomingMessageRemoved: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub OnOutgoingMessageAdded: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub OnOutgoingMessageRemoved: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub OnServerShutDown: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IFaxAccountNotify_Impl: super::super::System::Com::IDispatch_Impl {
    fn OnIncomingJobAdded(&self, pfaxaccount: windows_core::Ref<IFaxAccount>, bstrjobid: &windows_core::BSTR) -> windows_core::Result<()>;
    fn OnIncomingJobRemoved(&self, pfaxaccount: windows_core::Ref<IFaxAccount>, bstrjobid: &windows_core::BSTR) -> windows_core::Result<()>;
    fn OnIncomingJobChanged(&self, pfaxaccount: windows_core::Ref<IFaxAccount>, bstrjobid: &windows_core::BSTR, pjobstatus: windows_core::Ref<IFaxJobStatus>) -> windows_core::Result<()>;
    fn OnOutgoingJobAdded(&self, pfaxaccount: windows_core::Ref<IFaxAccount>, bstrjobid: &windows_core::BSTR) -> windows_core::Result<()>;
    fn OnOutgoingJobRemoved(&self, pfaxaccount: windows_core::Ref<IFaxAccount>, bstrjobid: &windows_core::BSTR) -> windows_core::Result<()>;
    fn OnOutgoingJobChanged(&self, pfaxaccount: windows_core::Ref<IFaxAccount>, bstrjobid: &windows_core::BSTR, pjobstatus: windows_core::Ref<IFaxJobStatus>) -> windows_core::Result<()>;
    fn OnIncomingMessageAdded(&self, pfaxaccount: windows_core::Ref<IFaxAccount>, bstrmessageid: &windows_core::BSTR, faddedtoreceivefolder: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
    fn OnIncomingMessageRemoved(&self, pfaxaccount: windows_core::Ref<IFaxAccount>, bstrmessageid: &windows_core::BSTR, fremovedfromreceivefolder: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
    fn OnOutgoingMessageAdded(&self, pfaxaccount: windows_core::Ref<IFaxAccount>, bstrmessageid: &windows_core::BSTR) -> windows_core::Result<()>;
    fn OnOutgoingMessageRemoved(&self, pfaxaccount: windows_core::Ref<IFaxAccount>, bstrmessageid: &windows_core::BSTR) -> windows_core::Result<()>;
    fn OnServerShutDown(&self, pfaxserver: windows_core::Ref<IFaxServer2>) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IFaxAccountNotify_Vtbl {
    pub const fn new<Identity: IFaxAccountNotify_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn OnIncomingJobAdded<Identity: IFaxAccountNotify_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pfaxaccount: *mut core::ffi::c_void, bstrjobid: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFaxAccountNotify_Impl::OnIncomingJobAdded(this, core::mem::transmute_copy(&pfaxaccount), core::mem::transmute(&bstrjobid)).into()
            }
        }
        unsafe extern "system" fn OnIncomingJobRemoved<Identity: IFaxAccountNotify_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pfaxaccount: *mut core::ffi::c_void, bstrjobid: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFaxAccountNotify_Impl::OnIncomingJobRemoved(this, core::mem::transmute_copy(&pfaxaccount), core::mem::transmute(&bstrjobid)).into()
            }
        }
        unsafe extern "system" fn OnIncomingJobChanged<Identity: IFaxAccountNotify_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pfaxaccount: *mut core::ffi::c_void, bstrjobid: *mut core::ffi::c_void, pjobstatus: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFaxAccountNotify_Impl::OnIncomingJobChanged(this, core::mem::transmute_copy(&pfaxaccount), core::mem::transmute(&bstrjobid), core::mem::transmute_copy(&pjobstatus)).into()
            }
        }
        unsafe extern "system" fn OnOutgoingJobAdded<Identity: IFaxAccountNotify_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pfaxaccount: *mut core::ffi::c_void, bstrjobid: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFaxAccountNotify_Impl::OnOutgoingJobAdded(this, core::mem::transmute_copy(&pfaxaccount), core::mem::transmute(&bstrjobid)).into()
            }
        }
        unsafe extern "system" fn OnOutgoingJobRemoved<Identity: IFaxAccountNotify_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pfaxaccount: *mut core::ffi::c_void, bstrjobid: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFaxAccountNotify_Impl::OnOutgoingJobRemoved(this, core::mem::transmute_copy(&pfaxaccount), core::mem::transmute(&bstrjobid)).into()
            }
        }
        unsafe extern "system" fn OnOutgoingJobChanged<Identity: IFaxAccountNotify_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pfaxaccount: *mut core::ffi::c_void, bstrjobid: *mut core::ffi::c_void, pjobstatus: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFaxAccountNotify_Impl::OnOutgoingJobChanged(this, core::mem::transmute_copy(&pfaxaccount), core::mem::transmute(&bstrjobid), core::mem::transmute_copy(&pjobstatus)).into()
            }
        }
        unsafe extern "system" fn OnIncomingMessageAdded<Identity: IFaxAccountNotify_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pfaxaccount: *mut core::ffi::c_void, bstrmessageid: *mut core::ffi::c_void, faddedtoreceivefolder: super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFaxAccountNotify_Impl::OnIncomingMessageAdded(this, core::mem::transmute_copy(&pfaxaccount), core::mem::transmute(&bstrmessageid), core::mem::transmute_copy(&faddedtoreceivefolder)).into()
            }
        }
        unsafe extern "system" fn OnIncomingMessageRemoved<Identity: IFaxAccountNotify_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pfaxaccount: *mut core::ffi::c_void, bstrmessageid: *mut core::ffi::c_void, fremovedfromreceivefolder: super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFaxAccountNotify_Impl::OnIncomingMessageRemoved(this, core::mem::transmute_copy(&pfaxaccount), core::mem::transmute(&bstrmessageid), core::mem::transmute_copy(&fremovedfromreceivefolder)).into()
            }
        }
        unsafe extern "system" fn OnOutgoingMessageAdded<Identity: IFaxAccountNotify_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pfaxaccount: *mut core::ffi::c_void, bstrmessageid: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFaxAccountNotify_Impl::OnOutgoingMessageAdded(this, core::mem::transmute_copy(&pfaxaccount), core::mem::transmute(&bstrmessageid)).into()
            }
        }
        unsafe extern "system" fn OnOutgoingMessageRemoved<Identity: IFaxAccountNotify_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pfaxaccount: *mut core::ffi::c_void, bstrmessageid: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFaxAccountNotify_Impl::OnOutgoingMessageRemoved(this, core::mem::transmute_copy(&pfaxaccount), core::mem::transmute(&bstrmessageid)).into()
            }
        }
        unsafe extern "system" fn OnServerShutDown<Identity: IFaxAccountNotify_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pfaxserver: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFaxAccountNotify_Impl::OnServerShutDown(this, core::mem::transmute_copy(&pfaxserver)).into()
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            OnIncomingJobAdded: OnIncomingJobAdded::<Identity, OFFSET>,
            OnIncomingJobRemoved: OnIncomingJobRemoved::<Identity, OFFSET>,
            OnIncomingJobChanged: OnIncomingJobChanged::<Identity, OFFSET>,
            OnOutgoingJobAdded: OnOutgoingJobAdded::<Identity, OFFSET>,
            OnOutgoingJobRemoved: OnOutgoingJobRemoved::<Identity, OFFSET>,
            OnOutgoingJobChanged: OnOutgoingJobChanged::<Identity, OFFSET>,
            OnIncomingMessageAdded: OnIncomingMessageAdded::<Identity, OFFSET>,
            OnIncomingMessageRemoved: OnIncomingMessageRemoved::<Identity, OFFSET>,
            OnOutgoingMessageAdded: OnOutgoingMessageAdded::<Identity, OFFSET>,
            OnOutgoingMessageRemoved: OnOutgoingMessageRemoved::<Identity, OFFSET>,
            OnServerShutDown: OnServerShutDown::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IFaxAccountNotify as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for IFaxAccountNotify {}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IFaxAccountOutgoingArchive, IFaxAccountOutgoingArchive_Vtbl, 0x5463076d_ec14_491f_926e_b3ceda5e5662);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IFaxAccountOutgoingArchive {
    type Target = super::super::System::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IFaxAccountOutgoingArchive, windows_core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl IFaxAccountOutgoingArchive {
    pub unsafe fn SizeLow(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).SizeLow)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SizeHigh(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).SizeHigh)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn Refresh(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Refresh)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn GetMessages(&self, lprefetchsize: i32) -> windows_core::Result<IFaxOutgoingMessageIterator> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetMessages)(windows_core::Interface::as_raw(self), lprefetchsize, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetMessage(&self, bstrmessageid: &windows_core::BSTR) -> windows_core::Result<IFaxOutgoingMessage> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetMessage)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrmessageid), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IFaxAccountOutgoingArchive_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub SizeLow: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SizeHigh: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub Refresh: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetMessages: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetMessage: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IFaxAccountOutgoingArchive_Impl: super::super::System::Com::IDispatch_Impl {
    fn SizeLow(&self) -> windows_core::Result<i32>;
    fn SizeHigh(&self) -> windows_core::Result<i32>;
    fn Refresh(&self) -> windows_core::Result<()>;
    fn GetMessages(&self, lprefetchsize: i32) -> windows_core::Result<IFaxOutgoingMessageIterator>;
    fn GetMessage(&self, bstrmessageid: &windows_core::BSTR) -> windows_core::Result<IFaxOutgoingMessage>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IFaxAccountOutgoingArchive_Vtbl {
    pub const fn new<Identity: IFaxAccountOutgoingArchive_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn SizeLow<Identity: IFaxAccountOutgoingArchive_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, plsizelow: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFaxAccountOutgoingArchive_Impl::SizeLow(this) {
                    Ok(ok__) => {
                        plsizelow.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SizeHigh<Identity: IFaxAccountOutgoingArchive_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, plsizehigh: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFaxAccountOutgoingArchive_Impl::SizeHigh(this) {
                    Ok(ok__) => {
                        plsizehigh.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Refresh<Identity: IFaxAccountOutgoingArchive_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFaxAccountOutgoingArchive_Impl::Refresh(this).into()
            }
        }
        unsafe extern "system" fn GetMessages<Identity: IFaxAccountOutgoingArchive_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lprefetchsize: i32, pfaxoutgoingmessageiterator: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFaxAccountOutgoingArchive_Impl::GetMessages(this, core::mem::transmute_copy(&lprefetchsize)) {
                    Ok(ok__) => {
                        pfaxoutgoingmessageiterator.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetMessage<Identity: IFaxAccountOutgoingArchive_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrmessageid: *mut core::ffi::c_void, pfaxoutgoingmessage: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFaxAccountOutgoingArchive_Impl::GetMessage(this, core::mem::transmute(&bstrmessageid)) {
                    Ok(ok__) => {
                        pfaxoutgoingmessage.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            SizeLow: SizeLow::<Identity, OFFSET>,
            SizeHigh: SizeHigh::<Identity, OFFSET>,
            Refresh: Refresh::<Identity, OFFSET>,
            GetMessages: GetMessages::<Identity, OFFSET>,
            GetMessage: GetMessage::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IFaxAccountOutgoingArchive as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for IFaxAccountOutgoingArchive {}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IFaxAccountOutgoingQueue, IFaxAccountOutgoingQueue_Vtbl, 0x0f1424e9_f22d_4553_b7a5_0d24bd0d7e46);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IFaxAccountOutgoingQueue {
    type Target = super::super::System::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IFaxAccountOutgoingQueue, windows_core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl IFaxAccountOutgoingQueue {
    pub unsafe fn GetJobs(&self) -> windows_core::Result<IFaxOutgoingJobs> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetJobs)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetJob(&self, bstrjobid: &windows_core::BSTR) -> windows_core::Result<IFaxOutgoingJob> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetJob)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrjobid), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IFaxAccountOutgoingQueue_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub GetJobs: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetJob: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IFaxAccountOutgoingQueue_Impl: super::super::System::Com::IDispatch_Impl {
    fn GetJobs(&self) -> windows_core::Result<IFaxOutgoingJobs>;
    fn GetJob(&self, bstrjobid: &windows_core::BSTR) -> windows_core::Result<IFaxOutgoingJob>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IFaxAccountOutgoingQueue_Vtbl {
    pub const fn new<Identity: IFaxAccountOutgoingQueue_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetJobs<Identity: IFaxAccountOutgoingQueue_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pfaxoutgoingjobs: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFaxAccountOutgoingQueue_Impl::GetJobs(this) {
                    Ok(ok__) => {
                        pfaxoutgoingjobs.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetJob<Identity: IFaxAccountOutgoingQueue_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrjobid: *mut core::ffi::c_void, pfaxoutgoingjob: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFaxAccountOutgoingQueue_Impl::GetJob(this, core::mem::transmute(&bstrjobid)) {
                    Ok(ok__) => {
                        pfaxoutgoingjob.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            GetJobs: GetJobs::<Identity, OFFSET>,
            GetJob: GetJob::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IFaxAccountOutgoingQueue as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for IFaxAccountOutgoingQueue {}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IFaxAccountSet, IFaxAccountSet_Vtbl, 0x7428fbae_841e_47b8_86f4_2288946dca1b);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IFaxAccountSet {
    type Target = super::super::System::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IFaxAccountSet, windows_core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl IFaxAccountSet {
    pub unsafe fn GetAccounts(&self) -> windows_core::Result<IFaxAccounts> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetAccounts)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetAccount(&self, bstraccountname: &windows_core::BSTR) -> windows_core::Result<IFaxAccount> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetAccount)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstraccountname), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn AddAccount(&self, bstraccountname: &windows_core::BSTR) -> windows_core::Result<IFaxAccount> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).AddAccount)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstraccountname), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn RemoveAccount(&self, bstraccountname: &windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).RemoveAccount)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstraccountname)).ok() }
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IFaxAccountSet_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub GetAccounts: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetAccount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub AddAccount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub RemoveAccount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IFaxAccountSet_Impl: super::super::System::Com::IDispatch_Impl {
    fn GetAccounts(&self) -> windows_core::Result<IFaxAccounts>;
    fn GetAccount(&self, bstraccountname: &windows_core::BSTR) -> windows_core::Result<IFaxAccount>;
    fn AddAccount(&self, bstraccountname: &windows_core::BSTR) -> windows_core::Result<IFaxAccount>;
    fn RemoveAccount(&self, bstraccountname: &windows_core::BSTR) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IFaxAccountSet_Vtbl {
    pub const fn new<Identity: IFaxAccountSet_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetAccounts<Identity: IFaxAccountSet_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppfaxaccounts: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFaxAccountSet_Impl::GetAccounts(this) {
                    Ok(ok__) => {
                        ppfaxaccounts.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetAccount<Identity: IFaxAccountSet_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstraccountname: *mut core::ffi::c_void, pfaxaccount: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFaxAccountSet_Impl::GetAccount(this, core::mem::transmute(&bstraccountname)) {
                    Ok(ok__) => {
                        pfaxaccount.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn AddAccount<Identity: IFaxAccountSet_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstraccountname: *mut core::ffi::c_void, pfaxaccount: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFaxAccountSet_Impl::AddAccount(this, core::mem::transmute(&bstraccountname)) {
                    Ok(ok__) => {
                        pfaxaccount.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn RemoveAccount<Identity: IFaxAccountSet_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstraccountname: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFaxAccountSet_Impl::RemoveAccount(this, core::mem::transmute(&bstraccountname)).into()
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            GetAccounts: GetAccounts::<Identity, OFFSET>,
            GetAccount: GetAccount::<Identity, OFFSET>,
            AddAccount: AddAccount::<Identity, OFFSET>,
            RemoveAccount: RemoveAccount::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IFaxAccountSet as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for IFaxAccountSet {}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IFaxAccounts, IFaxAccounts_Vtbl, 0x93ea8162_8be7_42d1_ae7b_ec74e2d989da);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IFaxAccounts {
    type Target = super::super::System::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IFaxAccounts, windows_core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl IFaxAccounts {
    pub unsafe fn _NewEnum(&self) -> windows_core::Result<windows_core::IUnknown> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self)._NewEnum)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn get_Item(&self, vindex: &super::super::System::Variant::VARIANT) -> windows_core::Result<IFaxAccount> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_Item)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(vindex), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn Count(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Count)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IFaxAccounts_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub _NewEnum: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub get_Item: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::System::Variant::VARIANT, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    get_Item: usize,
    pub Count: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IFaxAccounts_Impl: super::super::System::Com::IDispatch_Impl {
    fn _NewEnum(&self) -> windows_core::Result<windows_core::IUnknown>;
    fn get_Item(&self, vindex: &super::super::System::Variant::VARIANT) -> windows_core::Result<IFaxAccount>;
    fn Count(&self) -> windows_core::Result<i32>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IFaxAccounts_Vtbl {
    pub const fn new<Identity: IFaxAccounts_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn _NewEnum<Identity: IFaxAccounts_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppunk: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFaxAccounts_Impl::_NewEnum(this) {
                    Ok(ok__) => {
                        ppunk.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_Item<Identity: IFaxAccounts_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, vindex: super::super::System::Variant::VARIANT, pfaxaccount: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFaxAccounts_Impl::get_Item(this, core::mem::transmute(&vindex)) {
                    Ok(ok__) => {
                        pfaxaccount.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Count<Identity: IFaxAccounts_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, plcount: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFaxAccounts_Impl::Count(this) {
                    Ok(ok__) => {
                        plcount.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            _NewEnum: _NewEnum::<Identity, OFFSET>,
            get_Item: get_Item::<Identity, OFFSET>,
            Count: Count::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IFaxAccounts as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for IFaxAccounts {}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IFaxActivity, IFaxActivity_Vtbl, 0x4b106f97_3df5_40f2_bc3c_44cb8115ebdf);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IFaxActivity {
    type Target = super::super::System::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IFaxActivity, windows_core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl IFaxActivity {
    pub unsafe fn IncomingMessages(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).IncomingMessages)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn RoutingMessages(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).RoutingMessages)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn OutgoingMessages(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).OutgoingMessages)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn QueuedMessages(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).QueuedMessages)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn Refresh(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Refresh)(windows_core::Interface::as_raw(self)).ok() }
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IFaxActivity_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub IncomingMessages: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub RoutingMessages: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub OutgoingMessages: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub QueuedMessages: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub Refresh: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IFaxActivity_Impl: super::super::System::Com::IDispatch_Impl {
    fn IncomingMessages(&self) -> windows_core::Result<i32>;
    fn RoutingMessages(&self) -> windows_core::Result<i32>;
    fn OutgoingMessages(&self) -> windows_core::Result<i32>;
    fn QueuedMessages(&self) -> windows_core::Result<i32>;
    fn Refresh(&self) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IFaxActivity_Vtbl {
    pub const fn new<Identity: IFaxActivity_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn IncomingMessages<Identity: IFaxActivity_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, plincomingmessages: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFaxActivity_Impl::IncomingMessages(this) {
                    Ok(ok__) => {
                        plincomingmessages.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn RoutingMessages<Identity: IFaxActivity_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, plroutingmessages: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFaxActivity_Impl::RoutingMessages(this) {
                    Ok(ok__) => {
                        plroutingmessages.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn OutgoingMessages<Identity: IFaxActivity_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ploutgoingmessages: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFaxActivity_Impl::OutgoingMessages(this) {
                    Ok(ok__) => {
                        ploutgoingmessages.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn QueuedMessages<Identity: IFaxActivity_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, plqueuedmessages: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFaxActivity_Impl::QueuedMessages(this) {
                    Ok(ok__) => {
                        plqueuedmessages.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Refresh<Identity: IFaxActivity_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFaxActivity_Impl::Refresh(this).into()
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            IncomingMessages: IncomingMessages::<Identity, OFFSET>,
            RoutingMessages: RoutingMessages::<Identity, OFFSET>,
            OutgoingMessages: OutgoingMessages::<Identity, OFFSET>,
            QueuedMessages: QueuedMessages::<Identity, OFFSET>,
            Refresh: Refresh::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IFaxActivity as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for IFaxActivity {}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IFaxActivityLogging, IFaxActivityLogging_Vtbl, 0x1e29078b_5a69_497b_9592_49b7e7faddb5);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IFaxActivityLogging {
    type Target = super::super::System::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IFaxActivityLogging, windows_core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl IFaxActivityLogging {
    pub unsafe fn LogIncoming(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).LogIncoming)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetLogIncoming(&self, blogincoming: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetLogIncoming)(windows_core::Interface::as_raw(self), blogincoming).ok() }
    }
    pub unsafe fn LogOutgoing(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).LogOutgoing)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetLogOutgoing(&self, blogoutgoing: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetLogOutgoing)(windows_core::Interface::as_raw(self), blogoutgoing).ok() }
    }
    pub unsafe fn DatabasePath(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).DatabasePath)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn SetDatabasePath(&self, bstrdatabasepath: &windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetDatabasePath)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrdatabasepath)).ok() }
    }
    pub unsafe fn Refresh(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Refresh)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn Save(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Save)(windows_core::Interface::as_raw(self)).ok() }
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IFaxActivityLogging_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub LogIncoming: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub SetLogIncoming: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub LogOutgoing: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub SetLogOutgoing: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub DatabasePath: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetDatabasePath: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Refresh: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Save: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IFaxActivityLogging_Impl: super::super::System::Com::IDispatch_Impl {
    fn LogIncoming(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn SetLogIncoming(&self, blogincoming: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
    fn LogOutgoing(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn SetLogOutgoing(&self, blogoutgoing: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
    fn DatabasePath(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetDatabasePath(&self, bstrdatabasepath: &windows_core::BSTR) -> windows_core::Result<()>;
    fn Refresh(&self) -> windows_core::Result<()>;
    fn Save(&self) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IFaxActivityLogging_Vtbl {
    pub const fn new<Identity: IFaxActivityLogging_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn LogIncoming<Identity: IFaxActivityLogging_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pblogincoming: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFaxActivityLogging_Impl::LogIncoming(this) {
                    Ok(ok__) => {
                        pblogincoming.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetLogIncoming<Identity: IFaxActivityLogging_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, blogincoming: super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFaxActivityLogging_Impl::SetLogIncoming(this, core::mem::transmute_copy(&blogincoming)).into()
            }
        }
        unsafe extern "system" fn LogOutgoing<Identity: IFaxActivityLogging_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pblogoutgoing: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFaxActivityLogging_Impl::LogOutgoing(this) {
                    Ok(ok__) => {
                        pblogoutgoing.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetLogOutgoing<Identity: IFaxActivityLogging_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, blogoutgoing: super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFaxActivityLogging_Impl::SetLogOutgoing(this, core::mem::transmute_copy(&blogoutgoing)).into()
            }
        }
        unsafe extern "system" fn DatabasePath<Identity: IFaxActivityLogging_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrdatabasepath: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFaxActivityLogging_Impl::DatabasePath(this) {
                    Ok(ok__) => {
                        pbstrdatabasepath.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetDatabasePath<Identity: IFaxActivityLogging_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrdatabasepath: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFaxActivityLogging_Impl::SetDatabasePath(this, core::mem::transmute(&bstrdatabasepath)).into()
            }
        }
        unsafe extern "system" fn Refresh<Identity: IFaxActivityLogging_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFaxActivityLogging_Impl::Refresh(this).into()
            }
        }
        unsafe extern "system" fn Save<Identity: IFaxActivityLogging_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFaxActivityLogging_Impl::Save(this).into()
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            LogIncoming: LogIncoming::<Identity, OFFSET>,
            SetLogIncoming: SetLogIncoming::<Identity, OFFSET>,
            LogOutgoing: LogOutgoing::<Identity, OFFSET>,
            SetLogOutgoing: SetLogOutgoing::<Identity, OFFSET>,
            DatabasePath: DatabasePath::<Identity, OFFSET>,
            SetDatabasePath: SetDatabasePath::<Identity, OFFSET>,
            Refresh: Refresh::<Identity, OFFSET>,
            Save: Save::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IFaxActivityLogging as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for IFaxActivityLogging {}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IFaxConfiguration, IFaxConfiguration_Vtbl, 0x10f4d0f7_0994_4543_ab6e_506949128c40);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IFaxConfiguration {
    type Target = super::super::System::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IFaxConfiguration, windows_core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl IFaxConfiguration {
    pub unsafe fn UseArchive(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).UseArchive)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetUseArchive(&self, busearchive: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetUseArchive)(windows_core::Interface::as_raw(self), busearchive).ok() }
    }
    pub unsafe fn ArchiveLocation(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ArchiveLocation)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn SetArchiveLocation(&self, bstrarchivelocation: &windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetArchiveLocation)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrarchivelocation)).ok() }
    }
    pub unsafe fn SizeQuotaWarning(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).SizeQuotaWarning)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetSizeQuotaWarning(&self, bsizequotawarning: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetSizeQuotaWarning)(windows_core::Interface::as_raw(self), bsizequotawarning).ok() }
    }
    pub unsafe fn HighQuotaWaterMark(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).HighQuotaWaterMark)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetHighQuotaWaterMark(&self, lhighquotawatermark: i32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetHighQuotaWaterMark)(windows_core::Interface::as_raw(self), lhighquotawatermark).ok() }
    }
    pub unsafe fn LowQuotaWaterMark(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).LowQuotaWaterMark)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetLowQuotaWaterMark(&self, llowquotawatermark: i32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetLowQuotaWaterMark)(windows_core::Interface::as_raw(self), llowquotawatermark).ok() }
    }
    pub unsafe fn ArchiveAgeLimit(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ArchiveAgeLimit)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetArchiveAgeLimit(&self, larchiveagelimit: i32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetArchiveAgeLimit)(windows_core::Interface::as_raw(self), larchiveagelimit).ok() }
    }
    pub unsafe fn ArchiveSizeLow(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ArchiveSizeLow)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn ArchiveSizeHigh(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ArchiveSizeHigh)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn OutgoingQueueBlocked(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).OutgoingQueueBlocked)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetOutgoingQueueBlocked(&self, boutgoingblocked: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetOutgoingQueueBlocked)(windows_core::Interface::as_raw(self), boutgoingblocked).ok() }
    }
    pub unsafe fn OutgoingQueuePaused(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).OutgoingQueuePaused)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetOutgoingQueuePaused(&self, boutgoingpaused: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetOutgoingQueuePaused)(windows_core::Interface::as_raw(self), boutgoingpaused).ok() }
    }
    pub unsafe fn AllowPersonalCoverPages(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).AllowPersonalCoverPages)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetAllowPersonalCoverPages(&self, ballowpersonalcoverpages: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetAllowPersonalCoverPages)(windows_core::Interface::as_raw(self), ballowpersonalcoverpages).ok() }
    }
    pub unsafe fn UseDeviceTSID(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).UseDeviceTSID)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetUseDeviceTSID(&self, busedevicetsid: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetUseDeviceTSID)(windows_core::Interface::as_raw(self), busedevicetsid).ok() }
    }
    pub unsafe fn Retries(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Retries)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetRetries(&self, lretries: i32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetRetries)(windows_core::Interface::as_raw(self), lretries).ok() }
    }
    pub unsafe fn RetryDelay(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).RetryDelay)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetRetryDelay(&self, lretrydelay: i32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetRetryDelay)(windows_core::Interface::as_raw(self), lretrydelay).ok() }
    }
    pub unsafe fn DiscountRateStart(&self) -> windows_core::Result<f64> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).DiscountRateStart)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetDiscountRateStart(&self, datediscountratestart: f64) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetDiscountRateStart)(windows_core::Interface::as_raw(self), datediscountratestart).ok() }
    }
    pub unsafe fn DiscountRateEnd(&self) -> windows_core::Result<f64> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).DiscountRateEnd)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetDiscountRateEnd(&self, datediscountrateend: f64) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetDiscountRateEnd)(windows_core::Interface::as_raw(self), datediscountrateend).ok() }
    }
    pub unsafe fn OutgoingQueueAgeLimit(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).OutgoingQueueAgeLimit)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetOutgoingQueueAgeLimit(&self, loutgoingqueueagelimit: i32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetOutgoingQueueAgeLimit)(windows_core::Interface::as_raw(self), loutgoingqueueagelimit).ok() }
    }
    pub unsafe fn Branding(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Branding)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetBranding(&self, bbranding: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetBranding)(windows_core::Interface::as_raw(self), bbranding).ok() }
    }
    pub unsafe fn IncomingQueueBlocked(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).IncomingQueueBlocked)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetIncomingQueueBlocked(&self, bincomingblocked: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetIncomingQueueBlocked)(windows_core::Interface::as_raw(self), bincomingblocked).ok() }
    }
    pub unsafe fn AutoCreateAccountOnConnect(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).AutoCreateAccountOnConnect)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetAutoCreateAccountOnConnect(&self, bautocreateaccountonconnect: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetAutoCreateAccountOnConnect)(windows_core::Interface::as_raw(self), bautocreateaccountonconnect).ok() }
    }
    pub unsafe fn IncomingFaxesArePublic(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).IncomingFaxesArePublic)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetIncomingFaxesArePublic(&self, bincomingfaxesarepublic: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetIncomingFaxesArePublic)(windows_core::Interface::as_raw(self), bincomingfaxesarepublic).ok() }
    }
    pub unsafe fn Refresh(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Refresh)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn Save(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Save)(windows_core::Interface::as_raw(self)).ok() }
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IFaxConfiguration_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub UseArchive: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub SetUseArchive: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub ArchiveLocation: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetArchiveLocation: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SizeQuotaWarning: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub SetSizeQuotaWarning: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub HighQuotaWaterMark: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetHighQuotaWaterMark: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub LowQuotaWaterMark: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetLowQuotaWaterMark: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub ArchiveAgeLimit: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetArchiveAgeLimit: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub ArchiveSizeLow: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub ArchiveSizeHigh: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub OutgoingQueueBlocked: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub SetOutgoingQueueBlocked: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub OutgoingQueuePaused: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub SetOutgoingQueuePaused: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub AllowPersonalCoverPages: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub SetAllowPersonalCoverPages: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub UseDeviceTSID: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub SetUseDeviceTSID: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub Retries: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetRetries: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub RetryDelay: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetRetryDelay: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub DiscountRateStart: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f64) -> windows_core::HRESULT,
    pub SetDiscountRateStart: unsafe extern "system" fn(*mut core::ffi::c_void, f64) -> windows_core::HRESULT,
    pub DiscountRateEnd: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f64) -> windows_core::HRESULT,
    pub SetDiscountRateEnd: unsafe extern "system" fn(*mut core::ffi::c_void, f64) -> windows_core::HRESULT,
    pub OutgoingQueueAgeLimit: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetOutgoingQueueAgeLimit: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub Branding: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub SetBranding: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub IncomingQueueBlocked: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub SetIncomingQueueBlocked: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub AutoCreateAccountOnConnect: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub SetAutoCreateAccountOnConnect: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub IncomingFaxesArePublic: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub SetIncomingFaxesArePublic: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub Refresh: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Save: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IFaxConfiguration_Impl: super::super::System::Com::IDispatch_Impl {
    fn UseArchive(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn SetUseArchive(&self, busearchive: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
    fn ArchiveLocation(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetArchiveLocation(&self, bstrarchivelocation: &windows_core::BSTR) -> windows_core::Result<()>;
    fn SizeQuotaWarning(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn SetSizeQuotaWarning(&self, bsizequotawarning: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
    fn HighQuotaWaterMark(&self) -> windows_core::Result<i32>;
    fn SetHighQuotaWaterMark(&self, lhighquotawatermark: i32) -> windows_core::Result<()>;
    fn LowQuotaWaterMark(&self) -> windows_core::Result<i32>;
    fn SetLowQuotaWaterMark(&self, llowquotawatermark: i32) -> windows_core::Result<()>;
    fn ArchiveAgeLimit(&self) -> windows_core::Result<i32>;
    fn SetArchiveAgeLimit(&self, larchiveagelimit: i32) -> windows_core::Result<()>;
    fn ArchiveSizeLow(&self) -> windows_core::Result<i32>;
    fn ArchiveSizeHigh(&self) -> windows_core::Result<i32>;
    fn OutgoingQueueBlocked(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn SetOutgoingQueueBlocked(&self, boutgoingblocked: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
    fn OutgoingQueuePaused(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn SetOutgoingQueuePaused(&self, boutgoingpaused: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
    fn AllowPersonalCoverPages(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn SetAllowPersonalCoverPages(&self, ballowpersonalcoverpages: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
    fn UseDeviceTSID(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn SetUseDeviceTSID(&self, busedevicetsid: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
    fn Retries(&self) -> windows_core::Result<i32>;
    fn SetRetries(&self, lretries: i32) -> windows_core::Result<()>;
    fn RetryDelay(&self) -> windows_core::Result<i32>;
    fn SetRetryDelay(&self, lretrydelay: i32) -> windows_core::Result<()>;
    fn DiscountRateStart(&self) -> windows_core::Result<f64>;
    fn SetDiscountRateStart(&self, datediscountratestart: f64) -> windows_core::Result<()>;
    fn DiscountRateEnd(&self) -> windows_core::Result<f64>;
    fn SetDiscountRateEnd(&self, datediscountrateend: f64) -> windows_core::Result<()>;
    fn OutgoingQueueAgeLimit(&self) -> windows_core::Result<i32>;
    fn SetOutgoingQueueAgeLimit(&self, loutgoingqueueagelimit: i32) -> windows_core::Result<()>;
    fn Branding(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn SetBranding(&self, bbranding: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
    fn IncomingQueueBlocked(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn SetIncomingQueueBlocked(&self, bincomingblocked: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
    fn AutoCreateAccountOnConnect(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn SetAutoCreateAccountOnConnect(&self, bautocreateaccountonconnect: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
    fn IncomingFaxesArePublic(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn SetIncomingFaxesArePublic(&self, bincomingfaxesarepublic: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
    fn Refresh(&self) -> windows_core::Result<()>;
    fn Save(&self) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IFaxConfiguration_Vtbl {
    pub const fn new<Identity: IFaxConfiguration_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn UseArchive<Identity: IFaxConfiguration_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbusearchive: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFaxConfiguration_Impl::UseArchive(this) {
                    Ok(ok__) => {
                        pbusearchive.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetUseArchive<Identity: IFaxConfiguration_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, busearchive: super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFaxConfiguration_Impl::SetUseArchive(this, core::mem::transmute_copy(&busearchive)).into()
            }
        }
        unsafe extern "system" fn ArchiveLocation<Identity: IFaxConfiguration_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrarchivelocation: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFaxConfiguration_Impl::ArchiveLocation(this) {
                    Ok(ok__) => {
                        pbstrarchivelocation.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetArchiveLocation<Identity: IFaxConfiguration_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrarchivelocation: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFaxConfiguration_Impl::SetArchiveLocation(this, core::mem::transmute(&bstrarchivelocation)).into()
            }
        }
        unsafe extern "system" fn SizeQuotaWarning<Identity: IFaxConfiguration_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbsizequotawarning: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFaxConfiguration_Impl::SizeQuotaWarning(this) {
                    Ok(ok__) => {
                        pbsizequotawarning.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetSizeQuotaWarning<Identity: IFaxConfiguration_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bsizequotawarning: super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFaxConfiguration_Impl::SetSizeQuotaWarning(this, core::mem::transmute_copy(&bsizequotawarning)).into()
            }
        }
        unsafe extern "system" fn HighQuotaWaterMark<Identity: IFaxConfiguration_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, plhighquotawatermark: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFaxConfiguration_Impl::HighQuotaWaterMark(this) {
                    Ok(ok__) => {
                        plhighquotawatermark.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetHighQuotaWaterMark<Identity: IFaxConfiguration_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lhighquotawatermark: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFaxConfiguration_Impl::SetHighQuotaWaterMark(this, core::mem::transmute_copy(&lhighquotawatermark)).into()
            }
        }
        unsafe extern "system" fn LowQuotaWaterMark<Identity: IFaxConfiguration_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pllowquotawatermark: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFaxConfiguration_Impl::LowQuotaWaterMark(this) {
                    Ok(ok__) => {
                        pllowquotawatermark.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetLowQuotaWaterMark<Identity: IFaxConfiguration_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, llowquotawatermark: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFaxConfiguration_Impl::SetLowQuotaWaterMark(this, core::mem::transmute_copy(&llowquotawatermark)).into()
            }
        }
        unsafe extern "system" fn ArchiveAgeLimit<Identity: IFaxConfiguration_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, plarchiveagelimit: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFaxConfiguration_Impl::ArchiveAgeLimit(this) {
                    Ok(ok__) => {
                        plarchiveagelimit.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetArchiveAgeLimit<Identity: IFaxConfiguration_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, larchiveagelimit: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFaxConfiguration_Impl::SetArchiveAgeLimit(this, core::mem::transmute_copy(&larchiveagelimit)).into()
            }
        }
        unsafe extern "system" fn ArchiveSizeLow<Identity: IFaxConfiguration_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, plsizelow: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFaxConfiguration_Impl::ArchiveSizeLow(this) {
                    Ok(ok__) => {
                        plsizelow.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn ArchiveSizeHigh<Identity: IFaxConfiguration_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, plsizehigh: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFaxConfiguration_Impl::ArchiveSizeHigh(this) {
                    Ok(ok__) => {
                        plsizehigh.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn OutgoingQueueBlocked<Identity: IFaxConfiguration_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pboutgoingblocked: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFaxConfiguration_Impl::OutgoingQueueBlocked(this) {
                    Ok(ok__) => {
                        pboutgoingblocked.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetOutgoingQueueBlocked<Identity: IFaxConfiguration_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, boutgoingblocked: super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFaxConfiguration_Impl::SetOutgoingQueueBlocked(this, core::mem::transmute_copy(&boutgoingblocked)).into()
            }
        }
        unsafe extern "system" fn OutgoingQueuePaused<Identity: IFaxConfiguration_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pboutgoingpaused: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFaxConfiguration_Impl::OutgoingQueuePaused(this) {
                    Ok(ok__) => {
                        pboutgoingpaused.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetOutgoingQueuePaused<Identity: IFaxConfiguration_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, boutgoingpaused: super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFaxConfiguration_Impl::SetOutgoingQueuePaused(this, core::mem::transmute_copy(&boutgoingpaused)).into()
            }
        }
        unsafe extern "system" fn AllowPersonalCoverPages<Identity: IFaxConfiguration_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pballowpersonalcoverpages: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFaxConfiguration_Impl::AllowPersonalCoverPages(this) {
                    Ok(ok__) => {
                        pballowpersonalcoverpages.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetAllowPersonalCoverPages<Identity: IFaxConfiguration_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ballowpersonalcoverpages: super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFaxConfiguration_Impl::SetAllowPersonalCoverPages(this, core::mem::transmute_copy(&ballowpersonalcoverpages)).into()
            }
        }
        unsafe extern "system" fn UseDeviceTSID<Identity: IFaxConfiguration_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbusedevicetsid: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFaxConfiguration_Impl::UseDeviceTSID(this) {
                    Ok(ok__) => {
                        pbusedevicetsid.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetUseDeviceTSID<Identity: IFaxConfiguration_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, busedevicetsid: super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFaxConfiguration_Impl::SetUseDeviceTSID(this, core::mem::transmute_copy(&busedevicetsid)).into()
            }
        }
        unsafe extern "system" fn Retries<Identity: IFaxConfiguration_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, plretries: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFaxConfiguration_Impl::Retries(this) {
                    Ok(ok__) => {
                        plretries.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetRetries<Identity: IFaxConfiguration_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lretries: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFaxConfiguration_Impl::SetRetries(this, core::mem::transmute_copy(&lretries)).into()
            }
        }
        unsafe extern "system" fn RetryDelay<Identity: IFaxConfiguration_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, plretrydelay: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFaxConfiguration_Impl::RetryDelay(this) {
                    Ok(ok__) => {
                        plretrydelay.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetRetryDelay<Identity: IFaxConfiguration_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lretrydelay: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFaxConfiguration_Impl::SetRetryDelay(this, core::mem::transmute_copy(&lretrydelay)).into()
            }
        }
        unsafe extern "system" fn DiscountRateStart<Identity: IFaxConfiguration_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdatediscountratestart: *mut f64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFaxConfiguration_Impl::DiscountRateStart(this) {
                    Ok(ok__) => {
                        pdatediscountratestart.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetDiscountRateStart<Identity: IFaxConfiguration_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, datediscountratestart: f64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFaxConfiguration_Impl::SetDiscountRateStart(this, core::mem::transmute_copy(&datediscountratestart)).into()
            }
        }
        unsafe extern "system" fn DiscountRateEnd<Identity: IFaxConfiguration_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdatediscountrateend: *mut f64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFaxConfiguration_Impl::DiscountRateEnd(this) {
                    Ok(ok__) => {
                        pdatediscountrateend.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetDiscountRateEnd<Identity: IFaxConfiguration_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, datediscountrateend: f64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFaxConfiguration_Impl::SetDiscountRateEnd(this, core::mem::transmute_copy(&datediscountrateend)).into()
            }
        }
        unsafe extern "system" fn OutgoingQueueAgeLimit<Identity: IFaxConfiguration_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ploutgoingqueueagelimit: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFaxConfiguration_Impl::OutgoingQueueAgeLimit(this) {
                    Ok(ok__) => {
                        ploutgoingqueueagelimit.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetOutgoingQueueAgeLimit<Identity: IFaxConfiguration_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, loutgoingqueueagelimit: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFaxConfiguration_Impl::SetOutgoingQueueAgeLimit(this, core::mem::transmute_copy(&loutgoingqueueagelimit)).into()
            }
        }
        unsafe extern "system" fn Branding<Identity: IFaxConfiguration_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbbranding: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFaxConfiguration_Impl::Branding(this) {
                    Ok(ok__) => {
                        pbbranding.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetBranding<Identity: IFaxConfiguration_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bbranding: super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFaxConfiguration_Impl::SetBranding(this, core::mem::transmute_copy(&bbranding)).into()
            }
        }
        unsafe extern "system" fn IncomingQueueBlocked<Identity: IFaxConfiguration_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbincomingblocked: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFaxConfiguration_Impl::IncomingQueueBlocked(this) {
                    Ok(ok__) => {
                        pbincomingblocked.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetIncomingQueueBlocked<Identity: IFaxConfiguration_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bincomingblocked: super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFaxConfiguration_Impl::SetIncomingQueueBlocked(this, core::mem::transmute_copy(&bincomingblocked)).into()
            }
        }
        unsafe extern "system" fn AutoCreateAccountOnConnect<Identity: IFaxConfiguration_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbautocreateaccountonconnect: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFaxConfiguration_Impl::AutoCreateAccountOnConnect(this) {
                    Ok(ok__) => {
                        pbautocreateaccountonconnect.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetAutoCreateAccountOnConnect<Identity: IFaxConfiguration_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bautocreateaccountonconnect: super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFaxConfiguration_Impl::SetAutoCreateAccountOnConnect(this, core::mem::transmute_copy(&bautocreateaccountonconnect)).into()
            }
        }
        unsafe extern "system" fn IncomingFaxesArePublic<Identity: IFaxConfiguration_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbincomingfaxesarepublic: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFaxConfiguration_Impl::IncomingFaxesArePublic(this) {
                    Ok(ok__) => {
                        pbincomingfaxesarepublic.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetIncomingFaxesArePublic<Identity: IFaxConfiguration_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bincomingfaxesarepublic: super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFaxConfiguration_Impl::SetIncomingFaxesArePublic(this, core::mem::transmute_copy(&bincomingfaxesarepublic)).into()
            }
        }
        unsafe extern "system" fn Refresh<Identity: IFaxConfiguration_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFaxConfiguration_Impl::Refresh(this).into()
            }
        }
        unsafe extern "system" fn Save<Identity: IFaxConfiguration_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFaxConfiguration_Impl::Save(this).into()
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            UseArchive: UseArchive::<Identity, OFFSET>,
            SetUseArchive: SetUseArchive::<Identity, OFFSET>,
            ArchiveLocation: ArchiveLocation::<Identity, OFFSET>,
            SetArchiveLocation: SetArchiveLocation::<Identity, OFFSET>,
            SizeQuotaWarning: SizeQuotaWarning::<Identity, OFFSET>,
            SetSizeQuotaWarning: SetSizeQuotaWarning::<Identity, OFFSET>,
            HighQuotaWaterMark: HighQuotaWaterMark::<Identity, OFFSET>,
            SetHighQuotaWaterMark: SetHighQuotaWaterMark::<Identity, OFFSET>,
            LowQuotaWaterMark: LowQuotaWaterMark::<Identity, OFFSET>,
            SetLowQuotaWaterMark: SetLowQuotaWaterMark::<Identity, OFFSET>,
            ArchiveAgeLimit: ArchiveAgeLimit::<Identity, OFFSET>,
            SetArchiveAgeLimit: SetArchiveAgeLimit::<Identity, OFFSET>,
            ArchiveSizeLow: ArchiveSizeLow::<Identity, OFFSET>,
            ArchiveSizeHigh: ArchiveSizeHigh::<Identity, OFFSET>,
            OutgoingQueueBlocked: OutgoingQueueBlocked::<Identity, OFFSET>,
            SetOutgoingQueueBlocked: SetOutgoingQueueBlocked::<Identity, OFFSET>,
            OutgoingQueuePaused: OutgoingQueuePaused::<Identity, OFFSET>,
            SetOutgoingQueuePaused: SetOutgoingQueuePaused::<Identity, OFFSET>,
            AllowPersonalCoverPages: AllowPersonalCoverPages::<Identity, OFFSET>,
            SetAllowPersonalCoverPages: SetAllowPersonalCoverPages::<Identity, OFFSET>,
            UseDeviceTSID: UseDeviceTSID::<Identity, OFFSET>,
            SetUseDeviceTSID: SetUseDeviceTSID::<Identity, OFFSET>,
            Retries: Retries::<Identity, OFFSET>,
            SetRetries: SetRetries::<Identity, OFFSET>,
            RetryDelay: RetryDelay::<Identity, OFFSET>,
            SetRetryDelay: SetRetryDelay::<Identity, OFFSET>,
            DiscountRateStart: DiscountRateStart::<Identity, OFFSET>,
            SetDiscountRateStart: SetDiscountRateStart::<Identity, OFFSET>,
            DiscountRateEnd: DiscountRateEnd::<Identity, OFFSET>,
            SetDiscountRateEnd: SetDiscountRateEnd::<Identity, OFFSET>,
            OutgoingQueueAgeLimit: OutgoingQueueAgeLimit::<Identity, OFFSET>,
            SetOutgoingQueueAgeLimit: SetOutgoingQueueAgeLimit::<Identity, OFFSET>,
            Branding: Branding::<Identity, OFFSET>,
            SetBranding: SetBranding::<Identity, OFFSET>,
            IncomingQueueBlocked: IncomingQueueBlocked::<Identity, OFFSET>,
            SetIncomingQueueBlocked: SetIncomingQueueBlocked::<Identity, OFFSET>,
            AutoCreateAccountOnConnect: AutoCreateAccountOnConnect::<Identity, OFFSET>,
            SetAutoCreateAccountOnConnect: SetAutoCreateAccountOnConnect::<Identity, OFFSET>,
            IncomingFaxesArePublic: IncomingFaxesArePublic::<Identity, OFFSET>,
            SetIncomingFaxesArePublic: SetIncomingFaxesArePublic::<Identity, OFFSET>,
            Refresh: Refresh::<Identity, OFFSET>,
            Save: Save::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IFaxConfiguration as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for IFaxConfiguration {}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IFaxDevice, IFaxDevice_Vtbl, 0x49306c59_b52e_4867_9df4_ca5841c956d0);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IFaxDevice {
    type Target = super::super::System::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IFaxDevice, windows_core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl IFaxDevice {
    pub unsafe fn Id(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Id)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn DeviceName(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).DeviceName)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn ProviderUniqueName(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ProviderUniqueName)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn PoweredOff(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).PoweredOff)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn ReceivingNow(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ReceivingNow)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SendingNow(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).SendingNow)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn UsedRoutingMethods(&self) -> windows_core::Result<super::super::System::Variant::VARIANT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).UsedRoutingMethods)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn Description(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Description)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn SetDescription(&self, bstrdescription: &windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetDescription)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrdescription)).ok() }
    }
    pub unsafe fn SendEnabled(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).SendEnabled)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetSendEnabled(&self, bsendenabled: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetSendEnabled)(windows_core::Interface::as_raw(self), bsendenabled).ok() }
    }
    pub unsafe fn ReceiveMode(&self) -> windows_core::Result<FAX_DEVICE_RECEIVE_MODE_ENUM> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ReceiveMode)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetReceiveMode(&self, receivemode: FAX_DEVICE_RECEIVE_MODE_ENUM) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetReceiveMode)(windows_core::Interface::as_raw(self), receivemode).ok() }
    }
    pub unsafe fn RingsBeforeAnswer(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).RingsBeforeAnswer)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetRingsBeforeAnswer(&self, lringsbeforeanswer: i32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetRingsBeforeAnswer)(windows_core::Interface::as_raw(self), lringsbeforeanswer).ok() }
    }
    pub unsafe fn CSID(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CSID)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn SetCSID(&self, bstrcsid: &windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetCSID)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrcsid)).ok() }
    }
    pub unsafe fn TSID(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).TSID)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn SetTSID(&self, bstrtsid: &windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetTSID)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrtsid)).ok() }
    }
    pub unsafe fn Refresh(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Refresh)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn Save(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Save)(windows_core::Interface::as_raw(self)).ok() }
    }
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn GetExtensionProperty(&self, bstrguid: &windows_core::BSTR) -> windows_core::Result<super::super::System::Variant::VARIANT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetExtensionProperty)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrguid), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn SetExtensionProperty(&self, bstrguid: &windows_core::BSTR, vproperty: &super::super::System::Variant::VARIANT) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetExtensionProperty)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrguid), core::mem::transmute_copy(vproperty)).ok() }
    }
    pub unsafe fn UseRoutingMethod(&self, bstrmethodguid: &windows_core::BSTR, buse: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).UseRoutingMethod)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrmethodguid), buse).ok() }
    }
    pub unsafe fn RingingNow(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).RingingNow)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn AnswerCall(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).AnswerCall)(windows_core::Interface::as_raw(self)).ok() }
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IFaxDevice_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub Id: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub DeviceName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ProviderUniqueName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub PoweredOff: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub ReceivingNow: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub SendingNow: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub UsedRoutingMethods: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::System::Variant::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    UsedRoutingMethods: usize,
    pub Description: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetDescription: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SendEnabled: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub SetSendEnabled: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub ReceiveMode: unsafe extern "system" fn(*mut core::ffi::c_void, *mut FAX_DEVICE_RECEIVE_MODE_ENUM) -> windows_core::HRESULT,
    pub SetReceiveMode: unsafe extern "system" fn(*mut core::ffi::c_void, FAX_DEVICE_RECEIVE_MODE_ENUM) -> windows_core::HRESULT,
    pub RingsBeforeAnswer: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetRingsBeforeAnswer: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub CSID: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetCSID: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub TSID: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetTSID: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Refresh: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Save: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub GetExtensionProperty: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::System::Variant::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    GetExtensionProperty: usize,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub SetExtensionProperty: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, super::super::System::Variant::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    SetExtensionProperty: usize,
    pub UseRoutingMethod: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub RingingNow: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub AnswerCall: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IFaxDevice_Impl: super::super::System::Com::IDispatch_Impl {
    fn Id(&self) -> windows_core::Result<i32>;
    fn DeviceName(&self) -> windows_core::Result<windows_core::BSTR>;
    fn ProviderUniqueName(&self) -> windows_core::Result<windows_core::BSTR>;
    fn PoweredOff(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn ReceivingNow(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn SendingNow(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn UsedRoutingMethods(&self) -> windows_core::Result<super::super::System::Variant::VARIANT>;
    fn Description(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetDescription(&self, bstrdescription: &windows_core::BSTR) -> windows_core::Result<()>;
    fn SendEnabled(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn SetSendEnabled(&self, bsendenabled: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
    fn ReceiveMode(&self) -> windows_core::Result<FAX_DEVICE_RECEIVE_MODE_ENUM>;
    fn SetReceiveMode(&self, receivemode: FAX_DEVICE_RECEIVE_MODE_ENUM) -> windows_core::Result<()>;
    fn RingsBeforeAnswer(&self) -> windows_core::Result<i32>;
    fn SetRingsBeforeAnswer(&self, lringsbeforeanswer: i32) -> windows_core::Result<()>;
    fn CSID(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetCSID(&self, bstrcsid: &windows_core::BSTR) -> windows_core::Result<()>;
    fn TSID(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetTSID(&self, bstrtsid: &windows_core::BSTR) -> windows_core::Result<()>;
    fn Refresh(&self) -> windows_core::Result<()>;
    fn Save(&self) -> windows_core::Result<()>;
    fn GetExtensionProperty(&self, bstrguid: &windows_core::BSTR) -> windows_core::Result<super::super::System::Variant::VARIANT>;
    fn SetExtensionProperty(&self, bstrguid: &windows_core::BSTR, vproperty: &super::super::System::Variant::VARIANT) -> windows_core::Result<()>;
    fn UseRoutingMethod(&self, bstrmethodguid: &windows_core::BSTR, buse: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
    fn RingingNow(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn AnswerCall(&self) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IFaxDevice_Vtbl {
    pub const fn new<Identity: IFaxDevice_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Id<Identity: IFaxDevice_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, plid: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFaxDevice_Impl::Id(this) {
                    Ok(ok__) => {
                        plid.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn DeviceName<Identity: IFaxDevice_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrdevicename: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFaxDevice_Impl::DeviceName(this) {
                    Ok(ok__) => {
                        pbstrdevicename.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn ProviderUniqueName<Identity: IFaxDevice_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrprovideruniquename: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFaxDevice_Impl::ProviderUniqueName(this) {
                    Ok(ok__) => {
                        pbstrprovideruniquename.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn PoweredOff<Identity: IFaxDevice_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbpoweredoff: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFaxDevice_Impl::PoweredOff(this) {
                    Ok(ok__) => {
                        pbpoweredoff.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn ReceivingNow<Identity: IFaxDevice_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbreceivingnow: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFaxDevice_Impl::ReceivingNow(this) {
                    Ok(ok__) => {
                        pbreceivingnow.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SendingNow<Identity: IFaxDevice_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbsendingnow: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFaxDevice_Impl::SendingNow(this) {
                    Ok(ok__) => {
                        pbsendingnow.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn UsedRoutingMethods<Identity: IFaxDevice_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvusedroutingmethods: *mut super::super::System::Variant::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFaxDevice_Impl::UsedRoutingMethods(this) {
                    Ok(ok__) => {
                        pvusedroutingmethods.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Description<Identity: IFaxDevice_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrdescription: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFaxDevice_Impl::Description(this) {
                    Ok(ok__) => {
                        pbstrdescription.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetDescription<Identity: IFaxDevice_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrdescription: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFaxDevice_Impl::SetDescription(this, core::mem::transmute(&bstrdescription)).into()
            }
        }
        unsafe extern "system" fn SendEnabled<Identity: IFaxDevice_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbsendenabled: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFaxDevice_Impl::SendEnabled(this) {
                    Ok(ok__) => {
                        pbsendenabled.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetSendEnabled<Identity: IFaxDevice_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bsendenabled: super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFaxDevice_Impl::SetSendEnabled(this, core::mem::transmute_copy(&bsendenabled)).into()
            }
        }
        unsafe extern "system" fn ReceiveMode<Identity: IFaxDevice_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, preceivemode: *mut FAX_DEVICE_RECEIVE_MODE_ENUM) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFaxDevice_Impl::ReceiveMode(this) {
                    Ok(ok__) => {
                        preceivemode.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetReceiveMode<Identity: IFaxDevice_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, receivemode: FAX_DEVICE_RECEIVE_MODE_ENUM) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFaxDevice_Impl::SetReceiveMode(this, core::mem::transmute_copy(&receivemode)).into()
            }
        }
        unsafe extern "system" fn RingsBeforeAnswer<Identity: IFaxDevice_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, plringsbeforeanswer: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFaxDevice_Impl::RingsBeforeAnswer(this) {
                    Ok(ok__) => {
                        plringsbeforeanswer.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetRingsBeforeAnswer<Identity: IFaxDevice_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lringsbeforeanswer: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFaxDevice_Impl::SetRingsBeforeAnswer(this, core::mem::transmute_copy(&lringsbeforeanswer)).into()
            }
        }
        unsafe extern "system" fn CSID<Identity: IFaxDevice_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrcsid: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFaxDevice_Impl::CSID(this) {
                    Ok(ok__) => {
                        pbstrcsid.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetCSID<Identity: IFaxDevice_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrcsid: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFaxDevice_Impl::SetCSID(this, core::mem::transmute(&bstrcsid)).into()
            }
        }
        unsafe extern "system" fn TSID<Identity: IFaxDevice_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrtsid: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFaxDevice_Impl::TSID(this) {
                    Ok(ok__) => {
                        pbstrtsid.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetTSID<Identity: IFaxDevice_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrtsid: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFaxDevice_Impl::SetTSID(this, core::mem::transmute(&bstrtsid)).into()
            }
        }
        unsafe extern "system" fn Refresh<Identity: IFaxDevice_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFaxDevice_Impl::Refresh(this).into()
            }
        }
        unsafe extern "system" fn Save<Identity: IFaxDevice_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFaxDevice_Impl::Save(this).into()
            }
        }
        unsafe extern "system" fn GetExtensionProperty<Identity: IFaxDevice_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrguid: *mut core::ffi::c_void, pvproperty: *mut super::super::System::Variant::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFaxDevice_Impl::GetExtensionProperty(this, core::mem::transmute(&bstrguid)) {
                    Ok(ok__) => {
                        pvproperty.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetExtensionProperty<Identity: IFaxDevice_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrguid: *mut core::ffi::c_void, vproperty: super::super::System::Variant::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFaxDevice_Impl::SetExtensionProperty(this, core::mem::transmute(&bstrguid), core::mem::transmute(&vproperty)).into()
            }
        }
        unsafe extern "system" fn UseRoutingMethod<Identity: IFaxDevice_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrmethodguid: *mut core::ffi::c_void, buse: super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFaxDevice_Impl::UseRoutingMethod(this, core::mem::transmute(&bstrmethodguid), core::mem::transmute_copy(&buse)).into()
            }
        }
        unsafe extern "system" fn RingingNow<Identity: IFaxDevice_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbringingnow: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFaxDevice_Impl::RingingNow(this) {
                    Ok(ok__) => {
                        pbringingnow.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn AnswerCall<Identity: IFaxDevice_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFaxDevice_Impl::AnswerCall(this).into()
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            Id: Id::<Identity, OFFSET>,
            DeviceName: DeviceName::<Identity, OFFSET>,
            ProviderUniqueName: ProviderUniqueName::<Identity, OFFSET>,
            PoweredOff: PoweredOff::<Identity, OFFSET>,
            ReceivingNow: ReceivingNow::<Identity, OFFSET>,
            SendingNow: SendingNow::<Identity, OFFSET>,
            UsedRoutingMethods: UsedRoutingMethods::<Identity, OFFSET>,
            Description: Description::<Identity, OFFSET>,
            SetDescription: SetDescription::<Identity, OFFSET>,
            SendEnabled: SendEnabled::<Identity, OFFSET>,
            SetSendEnabled: SetSendEnabled::<Identity, OFFSET>,
            ReceiveMode: ReceiveMode::<Identity, OFFSET>,
            SetReceiveMode: SetReceiveMode::<Identity, OFFSET>,
            RingsBeforeAnswer: RingsBeforeAnswer::<Identity, OFFSET>,
            SetRingsBeforeAnswer: SetRingsBeforeAnswer::<Identity, OFFSET>,
            CSID: CSID::<Identity, OFFSET>,
            SetCSID: SetCSID::<Identity, OFFSET>,
            TSID: TSID::<Identity, OFFSET>,
            SetTSID: SetTSID::<Identity, OFFSET>,
            Refresh: Refresh::<Identity, OFFSET>,
            Save: Save::<Identity, OFFSET>,
            GetExtensionProperty: GetExtensionProperty::<Identity, OFFSET>,
            SetExtensionProperty: SetExtensionProperty::<Identity, OFFSET>,
            UseRoutingMethod: UseRoutingMethod::<Identity, OFFSET>,
            RingingNow: RingingNow::<Identity, OFFSET>,
            AnswerCall: AnswerCall::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IFaxDevice as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for IFaxDevice {}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IFaxDeviceIds, IFaxDeviceIds_Vtbl, 0x2f0f813f_4ce9_443e_8ca1_738cfaeee149);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IFaxDeviceIds {
    type Target = super::super::System::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IFaxDeviceIds, windows_core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl IFaxDeviceIds {
    pub unsafe fn _NewEnum(&self) -> windows_core::Result<windows_core::IUnknown> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self)._NewEnum)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn get_Item(&self, lindex: i32) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_Item)(windows_core::Interface::as_raw(self), lindex, &mut result__).map(|| result__)
        }
    }
    pub unsafe fn Count(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Count)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn Add(&self, ldeviceid: i32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Add)(windows_core::Interface::as_raw(self), ldeviceid).ok() }
    }
    pub unsafe fn Remove(&self, lindex: i32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Remove)(windows_core::Interface::as_raw(self), lindex).ok() }
    }
    pub unsafe fn SetOrder(&self, ldeviceid: i32, lneworder: i32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetOrder)(windows_core::Interface::as_raw(self), ldeviceid, lneworder).ok() }
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IFaxDeviceIds_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub _NewEnum: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub get_Item: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut i32) -> windows_core::HRESULT,
    pub Count: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub Add: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub Remove: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub SetOrder: unsafe extern "system" fn(*mut core::ffi::c_void, i32, i32) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IFaxDeviceIds_Impl: super::super::System::Com::IDispatch_Impl {
    fn _NewEnum(&self) -> windows_core::Result<windows_core::IUnknown>;
    fn get_Item(&self, lindex: i32) -> windows_core::Result<i32>;
    fn Count(&self) -> windows_core::Result<i32>;
    fn Add(&self, ldeviceid: i32) -> windows_core::Result<()>;
    fn Remove(&self, lindex: i32) -> windows_core::Result<()>;
    fn SetOrder(&self, ldeviceid: i32, lneworder: i32) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IFaxDeviceIds_Vtbl {
    pub const fn new<Identity: IFaxDeviceIds_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn _NewEnum<Identity: IFaxDeviceIds_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppunk: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFaxDeviceIds_Impl::_NewEnum(this) {
                    Ok(ok__) => {
                        ppunk.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_Item<Identity: IFaxDeviceIds_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lindex: i32, pldeviceid: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFaxDeviceIds_Impl::get_Item(this, core::mem::transmute_copy(&lindex)) {
                    Ok(ok__) => {
                        pldeviceid.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Count<Identity: IFaxDeviceIds_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, plcount: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFaxDeviceIds_Impl::Count(this) {
                    Ok(ok__) => {
                        plcount.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Add<Identity: IFaxDeviceIds_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ldeviceid: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFaxDeviceIds_Impl::Add(this, core::mem::transmute_copy(&ldeviceid)).into()
            }
        }
        unsafe extern "system" fn Remove<Identity: IFaxDeviceIds_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lindex: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFaxDeviceIds_Impl::Remove(this, core::mem::transmute_copy(&lindex)).into()
            }
        }
        unsafe extern "system" fn SetOrder<Identity: IFaxDeviceIds_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ldeviceid: i32, lneworder: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFaxDeviceIds_Impl::SetOrder(this, core::mem::transmute_copy(&ldeviceid), core::mem::transmute_copy(&lneworder)).into()
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            _NewEnum: _NewEnum::<Identity, OFFSET>,
            get_Item: get_Item::<Identity, OFFSET>,
            Count: Count::<Identity, OFFSET>,
            Add: Add::<Identity, OFFSET>,
            Remove: Remove::<Identity, OFFSET>,
            SetOrder: SetOrder::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IFaxDeviceIds as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for IFaxDeviceIds {}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IFaxDeviceProvider, IFaxDeviceProvider_Vtbl, 0x290eac63_83ec_449c_8417_f148df8c682a);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IFaxDeviceProvider {
    type Target = super::super::System::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IFaxDeviceProvider, windows_core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl IFaxDeviceProvider {
    pub unsafe fn FriendlyName(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).FriendlyName)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn ImageName(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ImageName)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn UniqueName(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).UniqueName)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn TapiProviderName(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).TapiProviderName)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn MajorVersion(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).MajorVersion)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn MinorVersion(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).MinorVersion)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn MajorBuild(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).MajorBuild)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn MinorBuild(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).MinorBuild)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn Debug(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Debug)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn Status(&self) -> windows_core::Result<FAX_PROVIDER_STATUS_ENUM> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Status)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn InitErrorCode(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).InitErrorCode)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn DeviceIds(&self) -> windows_core::Result<super::super::System::Variant::VARIANT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).DeviceIds)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IFaxDeviceProvider_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub FriendlyName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ImageName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub UniqueName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub TapiProviderName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub MajorVersion: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub MinorVersion: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub MajorBuild: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub MinorBuild: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub Debug: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub Status: unsafe extern "system" fn(*mut core::ffi::c_void, *mut FAX_PROVIDER_STATUS_ENUM) -> windows_core::HRESULT,
    pub InitErrorCode: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub DeviceIds: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::System::Variant::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    DeviceIds: usize,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IFaxDeviceProvider_Impl: super::super::System::Com::IDispatch_Impl {
    fn FriendlyName(&self) -> windows_core::Result<windows_core::BSTR>;
    fn ImageName(&self) -> windows_core::Result<windows_core::BSTR>;
    fn UniqueName(&self) -> windows_core::Result<windows_core::BSTR>;
    fn TapiProviderName(&self) -> windows_core::Result<windows_core::BSTR>;
    fn MajorVersion(&self) -> windows_core::Result<i32>;
    fn MinorVersion(&self) -> windows_core::Result<i32>;
    fn MajorBuild(&self) -> windows_core::Result<i32>;
    fn MinorBuild(&self) -> windows_core::Result<i32>;
    fn Debug(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn Status(&self) -> windows_core::Result<FAX_PROVIDER_STATUS_ENUM>;
    fn InitErrorCode(&self) -> windows_core::Result<i32>;
    fn DeviceIds(&self) -> windows_core::Result<super::super::System::Variant::VARIANT>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IFaxDeviceProvider_Vtbl {
    pub const fn new<Identity: IFaxDeviceProvider_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn FriendlyName<Identity: IFaxDeviceProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrfriendlyname: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFaxDeviceProvider_Impl::FriendlyName(this) {
                    Ok(ok__) => {
                        pbstrfriendlyname.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn ImageName<Identity: IFaxDeviceProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrimagename: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFaxDeviceProvider_Impl::ImageName(this) {
                    Ok(ok__) => {
                        pbstrimagename.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn UniqueName<Identity: IFaxDeviceProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstruniquename: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFaxDeviceProvider_Impl::UniqueName(this) {
                    Ok(ok__) => {
                        pbstruniquename.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn TapiProviderName<Identity: IFaxDeviceProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrtapiprovidername: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFaxDeviceProvider_Impl::TapiProviderName(this) {
                    Ok(ok__) => {
                        pbstrtapiprovidername.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn MajorVersion<Identity: IFaxDeviceProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, plmajorversion: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFaxDeviceProvider_Impl::MajorVersion(this) {
                    Ok(ok__) => {
                        plmajorversion.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn MinorVersion<Identity: IFaxDeviceProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, plminorversion: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFaxDeviceProvider_Impl::MinorVersion(this) {
                    Ok(ok__) => {
                        plminorversion.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn MajorBuild<Identity: IFaxDeviceProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, plmajorbuild: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFaxDeviceProvider_Impl::MajorBuild(this) {
                    Ok(ok__) => {
                        plmajorbuild.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn MinorBuild<Identity: IFaxDeviceProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, plminorbuild: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFaxDeviceProvider_Impl::MinorBuild(this) {
                    Ok(ok__) => {
                        plminorbuild.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Debug<Identity: IFaxDeviceProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbdebug: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFaxDeviceProvider_Impl::Debug(this) {
                    Ok(ok__) => {
                        pbdebug.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Status<Identity: IFaxDeviceProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pstatus: *mut FAX_PROVIDER_STATUS_ENUM) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFaxDeviceProvider_Impl::Status(this) {
                    Ok(ok__) => {
                        pstatus.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn InitErrorCode<Identity: IFaxDeviceProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pliniterrorcode: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFaxDeviceProvider_Impl::InitErrorCode(this) {
                    Ok(ok__) => {
                        pliniterrorcode.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn DeviceIds<Identity: IFaxDeviceProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvdeviceids: *mut super::super::System::Variant::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFaxDeviceProvider_Impl::DeviceIds(this) {
                    Ok(ok__) => {
                        pvdeviceids.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            FriendlyName: FriendlyName::<Identity, OFFSET>,
            ImageName: ImageName::<Identity, OFFSET>,
            UniqueName: UniqueName::<Identity, OFFSET>,
            TapiProviderName: TapiProviderName::<Identity, OFFSET>,
            MajorVersion: MajorVersion::<Identity, OFFSET>,
            MinorVersion: MinorVersion::<Identity, OFFSET>,
            MajorBuild: MajorBuild::<Identity, OFFSET>,
            MinorBuild: MinorBuild::<Identity, OFFSET>,
            Debug: Debug::<Identity, OFFSET>,
            Status: Status::<Identity, OFFSET>,
            InitErrorCode: InitErrorCode::<Identity, OFFSET>,
            DeviceIds: DeviceIds::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IFaxDeviceProvider as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for IFaxDeviceProvider {}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IFaxDeviceProviders, IFaxDeviceProviders_Vtbl, 0x9fb76f62_4c7e_43a5_b6fd_502893f7e13e);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IFaxDeviceProviders {
    type Target = super::super::System::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IFaxDeviceProviders, windows_core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl IFaxDeviceProviders {
    pub unsafe fn _NewEnum(&self) -> windows_core::Result<windows_core::IUnknown> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self)._NewEnum)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn get_Item(&self, vindex: &super::super::System::Variant::VARIANT) -> windows_core::Result<IFaxDeviceProvider> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_Item)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(vindex), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn Count(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Count)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IFaxDeviceProviders_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub _NewEnum: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub get_Item: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::System::Variant::VARIANT, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    get_Item: usize,
    pub Count: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IFaxDeviceProviders_Impl: super::super::System::Com::IDispatch_Impl {
    fn _NewEnum(&self) -> windows_core::Result<windows_core::IUnknown>;
    fn get_Item(&self, vindex: &super::super::System::Variant::VARIANT) -> windows_core::Result<IFaxDeviceProvider>;
    fn Count(&self) -> windows_core::Result<i32>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IFaxDeviceProviders_Vtbl {
    pub const fn new<Identity: IFaxDeviceProviders_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn _NewEnum<Identity: IFaxDeviceProviders_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppunk: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFaxDeviceProviders_Impl::_NewEnum(this) {
                    Ok(ok__) => {
                        ppunk.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_Item<Identity: IFaxDeviceProviders_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, vindex: super::super::System::Variant::VARIANT, pfaxdeviceprovider: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFaxDeviceProviders_Impl::get_Item(this, core::mem::transmute(&vindex)) {
                    Ok(ok__) => {
                        pfaxdeviceprovider.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Count<Identity: IFaxDeviceProviders_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, plcount: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFaxDeviceProviders_Impl::Count(this) {
                    Ok(ok__) => {
                        plcount.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            _NewEnum: _NewEnum::<Identity, OFFSET>,
            get_Item: get_Item::<Identity, OFFSET>,
            Count: Count::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IFaxDeviceProviders as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for IFaxDeviceProviders {}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IFaxDevices, IFaxDevices_Vtbl, 0x9e46783e_f34f_482e_a360_0416becbbd96);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IFaxDevices {
    type Target = super::super::System::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IFaxDevices, windows_core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl IFaxDevices {
    pub unsafe fn _NewEnum(&self) -> windows_core::Result<windows_core::IUnknown> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self)._NewEnum)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn get_Item(&self, vindex: &super::super::System::Variant::VARIANT) -> windows_core::Result<IFaxDevice> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_Item)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(vindex), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn Count(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Count)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn get_ItemById(&self, lid: i32) -> windows_core::Result<IFaxDevice> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_ItemById)(windows_core::Interface::as_raw(self), lid, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IFaxDevices_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub _NewEnum: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub get_Item: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::System::Variant::VARIANT, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    get_Item: usize,
    pub Count: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub get_ItemById: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IFaxDevices_Impl: super::super::System::Com::IDispatch_Impl {
    fn _NewEnum(&self) -> windows_core::Result<windows_core::IUnknown>;
    fn get_Item(&self, vindex: &super::super::System::Variant::VARIANT) -> windows_core::Result<IFaxDevice>;
    fn Count(&self) -> windows_core::Result<i32>;
    fn get_ItemById(&self, lid: i32) -> windows_core::Result<IFaxDevice>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IFaxDevices_Vtbl {
    pub const fn new<Identity: IFaxDevices_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn _NewEnum<Identity: IFaxDevices_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppunk: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFaxDevices_Impl::_NewEnum(this) {
                    Ok(ok__) => {
                        ppunk.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_Item<Identity: IFaxDevices_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, vindex: super::super::System::Variant::VARIANT, pfaxdevice: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFaxDevices_Impl::get_Item(this, core::mem::transmute(&vindex)) {
                    Ok(ok__) => {
                        pfaxdevice.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Count<Identity: IFaxDevices_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, plcount: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFaxDevices_Impl::Count(this) {
                    Ok(ok__) => {
                        plcount.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_ItemById<Identity: IFaxDevices_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lid: i32, ppfaxdevice: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFaxDevices_Impl::get_ItemById(this, core::mem::transmute_copy(&lid)) {
                    Ok(ok__) => {
                        ppfaxdevice.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            _NewEnum: _NewEnum::<Identity, OFFSET>,
            get_Item: get_Item::<Identity, OFFSET>,
            Count: Count::<Identity, OFFSET>,
            get_ItemById: get_ItemById::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IFaxDevices as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for IFaxDevices {}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IFaxDocument, IFaxDocument_Vtbl, 0xb207a246_09e3_4a4e_a7dc_fea31d29458f);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IFaxDocument {
    type Target = super::super::System::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IFaxDocument, windows_core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl IFaxDocument {
    pub unsafe fn Body(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Body)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn SetBody(&self, bstrbody: &windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetBody)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrbody)).ok() }
    }
    pub unsafe fn Sender(&self) -> windows_core::Result<IFaxSender> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Sender)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn Recipients(&self) -> windows_core::Result<IFaxRecipients> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Recipients)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn CoverPage(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CoverPage)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn SetCoverPage(&self, bstrcoverpage: &windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetCoverPage)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrcoverpage)).ok() }
    }
    pub unsafe fn Subject(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Subject)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn SetSubject(&self, bstrsubject: &windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetSubject)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrsubject)).ok() }
    }
    pub unsafe fn Note(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Note)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn SetNote(&self, bstrnote: &windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetNote)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrnote)).ok() }
    }
    pub unsafe fn ScheduleTime(&self) -> windows_core::Result<f64> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ScheduleTime)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetScheduleTime(&self, datescheduletime: f64) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetScheduleTime)(windows_core::Interface::as_raw(self), datescheduletime).ok() }
    }
    pub unsafe fn ReceiptAddress(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ReceiptAddress)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn SetReceiptAddress(&self, bstrreceiptaddress: &windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetReceiptAddress)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrreceiptaddress)).ok() }
    }
    pub unsafe fn DocumentName(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).DocumentName)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn SetDocumentName(&self, bstrdocumentname: &windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetDocumentName)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrdocumentname)).ok() }
    }
    pub unsafe fn CallHandle(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CallHandle)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetCallHandle(&self, lcallhandle: i32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetCallHandle)(windows_core::Interface::as_raw(self), lcallhandle).ok() }
    }
    pub unsafe fn CoverPageType(&self) -> windows_core::Result<FAX_COVERPAGE_TYPE_ENUM> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CoverPageType)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetCoverPageType(&self, coverpagetype: FAX_COVERPAGE_TYPE_ENUM) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetCoverPageType)(windows_core::Interface::as_raw(self), coverpagetype).ok() }
    }
    pub unsafe fn ScheduleType(&self) -> windows_core::Result<FAX_SCHEDULE_TYPE_ENUM> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ScheduleType)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetScheduleType(&self, scheduletype: FAX_SCHEDULE_TYPE_ENUM) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetScheduleType)(windows_core::Interface::as_raw(self), scheduletype).ok() }
    }
    pub unsafe fn ReceiptType(&self) -> windows_core::Result<FAX_RECEIPT_TYPE_ENUM> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ReceiptType)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetReceiptType(&self, receipttype: FAX_RECEIPT_TYPE_ENUM) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetReceiptType)(windows_core::Interface::as_raw(self), receipttype).ok() }
    }
    pub unsafe fn GroupBroadcastReceipts(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GroupBroadcastReceipts)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetGroupBroadcastReceipts(&self, busegrouping: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetGroupBroadcastReceipts)(windows_core::Interface::as_raw(self), busegrouping).ok() }
    }
    pub unsafe fn Priority(&self) -> windows_core::Result<FAX_PRIORITY_TYPE_ENUM> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Priority)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetPriority(&self, priority: FAX_PRIORITY_TYPE_ENUM) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetPriority)(windows_core::Interface::as_raw(self), priority).ok() }
    }
    pub unsafe fn TapiConnection(&self) -> windows_core::Result<super::super::System::Com::IDispatch> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).TapiConnection)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn putref_TapiConnection<P0>(&self, ptapiconnection: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::System::Com::IDispatch>,
    {
        unsafe { (windows_core::Interface::vtable(self).putref_TapiConnection)(windows_core::Interface::as_raw(self), ptapiconnection.param().abi()).ok() }
    }
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn Submit(&self, bstrfaxservername: &windows_core::BSTR) -> windows_core::Result<super::super::System::Variant::VARIANT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Submit)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrfaxservername), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn ConnectedSubmit<P0>(&self, pfaxserver: P0) -> windows_core::Result<super::super::System::Variant::VARIANT>
    where
        P0: windows_core::Param<IFaxServer>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ConnectedSubmit)(windows_core::Interface::as_raw(self), pfaxserver.param().abi(), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn AttachFaxToReceipt(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).AttachFaxToReceipt)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetAttachFaxToReceipt(&self, battachfax: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetAttachFaxToReceipt)(windows_core::Interface::as_raw(self), battachfax).ok() }
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IFaxDocument_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub Body: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetBody: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Sender: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Recipients: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CoverPage: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetCoverPage: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Subject: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetSubject: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Note: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetNote: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ScheduleTime: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f64) -> windows_core::HRESULT,
    pub SetScheduleTime: unsafe extern "system" fn(*mut core::ffi::c_void, f64) -> windows_core::HRESULT,
    pub ReceiptAddress: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetReceiptAddress: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub DocumentName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetDocumentName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CallHandle: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetCallHandle: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub CoverPageType: unsafe extern "system" fn(*mut core::ffi::c_void, *mut FAX_COVERPAGE_TYPE_ENUM) -> windows_core::HRESULT,
    pub SetCoverPageType: unsafe extern "system" fn(*mut core::ffi::c_void, FAX_COVERPAGE_TYPE_ENUM) -> windows_core::HRESULT,
    pub ScheduleType: unsafe extern "system" fn(*mut core::ffi::c_void, *mut FAX_SCHEDULE_TYPE_ENUM) -> windows_core::HRESULT,
    pub SetScheduleType: unsafe extern "system" fn(*mut core::ffi::c_void, FAX_SCHEDULE_TYPE_ENUM) -> windows_core::HRESULT,
    pub ReceiptType: unsafe extern "system" fn(*mut core::ffi::c_void, *mut FAX_RECEIPT_TYPE_ENUM) -> windows_core::HRESULT,
    pub SetReceiptType: unsafe extern "system" fn(*mut core::ffi::c_void, FAX_RECEIPT_TYPE_ENUM) -> windows_core::HRESULT,
    pub GroupBroadcastReceipts: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub SetGroupBroadcastReceipts: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub Priority: unsafe extern "system" fn(*mut core::ffi::c_void, *mut FAX_PRIORITY_TYPE_ENUM) -> windows_core::HRESULT,
    pub SetPriority: unsafe extern "system" fn(*mut core::ffi::c_void, FAX_PRIORITY_TYPE_ENUM) -> windows_core::HRESULT,
    pub TapiConnection: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub putref_TapiConnection: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub Submit: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::System::Variant::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    Submit: usize,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub ConnectedSubmit: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::System::Variant::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    ConnectedSubmit: usize,
    pub AttachFaxToReceipt: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub SetAttachFaxToReceipt: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IFaxDocument_Impl: super::super::System::Com::IDispatch_Impl {
    fn Body(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetBody(&self, bstrbody: &windows_core::BSTR) -> windows_core::Result<()>;
    fn Sender(&self) -> windows_core::Result<IFaxSender>;
    fn Recipients(&self) -> windows_core::Result<IFaxRecipients>;
    fn CoverPage(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetCoverPage(&self, bstrcoverpage: &windows_core::BSTR) -> windows_core::Result<()>;
    fn Subject(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetSubject(&self, bstrsubject: &windows_core::BSTR) -> windows_core::Result<()>;
    fn Note(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetNote(&self, bstrnote: &windows_core::BSTR) -> windows_core::Result<()>;
    fn ScheduleTime(&self) -> windows_core::Result<f64>;
    fn SetScheduleTime(&self, datescheduletime: f64) -> windows_core::Result<()>;
    fn ReceiptAddress(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetReceiptAddress(&self, bstrreceiptaddress: &windows_core::BSTR) -> windows_core::Result<()>;
    fn DocumentName(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetDocumentName(&self, bstrdocumentname: &windows_core::BSTR) -> windows_core::Result<()>;
    fn CallHandle(&self) -> windows_core::Result<i32>;
    fn SetCallHandle(&self, lcallhandle: i32) -> windows_core::Result<()>;
    fn CoverPageType(&self) -> windows_core::Result<FAX_COVERPAGE_TYPE_ENUM>;
    fn SetCoverPageType(&self, coverpagetype: FAX_COVERPAGE_TYPE_ENUM) -> windows_core::Result<()>;
    fn ScheduleType(&self) -> windows_core::Result<FAX_SCHEDULE_TYPE_ENUM>;
    fn SetScheduleType(&self, scheduletype: FAX_SCHEDULE_TYPE_ENUM) -> windows_core::Result<()>;
    fn ReceiptType(&self) -> windows_core::Result<FAX_RECEIPT_TYPE_ENUM>;
    fn SetReceiptType(&self, receipttype: FAX_RECEIPT_TYPE_ENUM) -> windows_core::Result<()>;
    fn GroupBroadcastReceipts(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn SetGroupBroadcastReceipts(&self, busegrouping: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
    fn Priority(&self) -> windows_core::Result<FAX_PRIORITY_TYPE_ENUM>;
    fn SetPriority(&self, priority: FAX_PRIORITY_TYPE_ENUM) -> windows_core::Result<()>;
    fn TapiConnection(&self) -> windows_core::Result<super::super::System::Com::IDispatch>;
    fn putref_TapiConnection(&self, ptapiconnection: windows_core::Ref<super::super::System::Com::IDispatch>) -> windows_core::Result<()>;
    fn Submit(&self, bstrfaxservername: &windows_core::BSTR) -> windows_core::Result<super::super::System::Variant::VARIANT>;
    fn ConnectedSubmit(&self, pfaxserver: windows_core::Ref<IFaxServer>) -> windows_core::Result<super::super::System::Variant::VARIANT>;
    fn AttachFaxToReceipt(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn SetAttachFaxToReceipt(&self, battachfax: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IFaxDocument_Vtbl {
    pub const fn new<Identity: IFaxDocument_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Body<Identity: IFaxDocument_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrbody: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFaxDocument_Impl::Body(this) {
                    Ok(ok__) => {
                        pbstrbody.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetBody<Identity: IFaxDocument_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrbody: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFaxDocument_Impl::SetBody(this, core::mem::transmute(&bstrbody)).into()
            }
        }
        unsafe extern "system" fn Sender<Identity: IFaxDocument_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppfaxsender: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFaxDocument_Impl::Sender(this) {
                    Ok(ok__) => {
                        ppfaxsender.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Recipients<Identity: IFaxDocument_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppfaxrecipients: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFaxDocument_Impl::Recipients(this) {
                    Ok(ok__) => {
                        ppfaxrecipients.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CoverPage<Identity: IFaxDocument_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrcoverpage: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFaxDocument_Impl::CoverPage(this) {
                    Ok(ok__) => {
                        pbstrcoverpage.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetCoverPage<Identity: IFaxDocument_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrcoverpage: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFaxDocument_Impl::SetCoverPage(this, core::mem::transmute(&bstrcoverpage)).into()
            }
        }
        unsafe extern "system" fn Subject<Identity: IFaxDocument_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrsubject: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFaxDocument_Impl::Subject(this) {
                    Ok(ok__) => {
                        pbstrsubject.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetSubject<Identity: IFaxDocument_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrsubject: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFaxDocument_Impl::SetSubject(this, core::mem::transmute(&bstrsubject)).into()
            }
        }
        unsafe extern "system" fn Note<Identity: IFaxDocument_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrnote: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFaxDocument_Impl::Note(this) {
                    Ok(ok__) => {
                        pbstrnote.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetNote<Identity: IFaxDocument_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrnote: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFaxDocument_Impl::SetNote(this, core::mem::transmute(&bstrnote)).into()
            }
        }
        unsafe extern "system" fn ScheduleTime<Identity: IFaxDocument_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdatescheduletime: *mut f64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFaxDocument_Impl::ScheduleTime(this) {
                    Ok(ok__) => {
                        pdatescheduletime.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetScheduleTime<Identity: IFaxDocument_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, datescheduletime: f64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFaxDocument_Impl::SetScheduleTime(this, core::mem::transmute_copy(&datescheduletime)).into()
            }
        }
        unsafe extern "system" fn ReceiptAddress<Identity: IFaxDocument_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrreceiptaddress: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFaxDocument_Impl::ReceiptAddress(this) {
                    Ok(ok__) => {
                        pbstrreceiptaddress.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetReceiptAddress<Identity: IFaxDocument_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrreceiptaddress: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFaxDocument_Impl::SetReceiptAddress(this, core::mem::transmute(&bstrreceiptaddress)).into()
            }
        }
        unsafe extern "system" fn DocumentName<Identity: IFaxDocument_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrdocumentname: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFaxDocument_Impl::DocumentName(this) {
                    Ok(ok__) => {
                        pbstrdocumentname.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetDocumentName<Identity: IFaxDocument_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrdocumentname: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFaxDocument_Impl::SetDocumentName(this, core::mem::transmute(&bstrdocumentname)).into()
            }
        }
        unsafe extern "system" fn CallHandle<Identity: IFaxDocument_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, plcallhandle: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFaxDocument_Impl::CallHandle(this) {
                    Ok(ok__) => {
                        plcallhandle.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetCallHandle<Identity: IFaxDocument_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lcallhandle: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFaxDocument_Impl::SetCallHandle(this, core::mem::transmute_copy(&lcallhandle)).into()
            }
        }
        unsafe extern "system" fn CoverPageType<Identity: IFaxDocument_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcoverpagetype: *mut FAX_COVERPAGE_TYPE_ENUM) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFaxDocument_Impl::CoverPageType(this) {
                    Ok(ok__) => {
                        pcoverpagetype.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetCoverPageType<Identity: IFaxDocument_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, coverpagetype: FAX_COVERPAGE_TYPE_ENUM) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFaxDocument_Impl::SetCoverPageType(this, core::mem::transmute_copy(&coverpagetype)).into()
            }
        }
        unsafe extern "system" fn ScheduleType<Identity: IFaxDocument_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pscheduletype: *mut FAX_SCHEDULE_TYPE_ENUM) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFaxDocument_Impl::ScheduleType(this) {
                    Ok(ok__) => {
                        pscheduletype.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetScheduleType<Identity: IFaxDocument_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, scheduletype: FAX_SCHEDULE_TYPE_ENUM) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFaxDocument_Impl::SetScheduleType(this, core::mem::transmute_copy(&scheduletype)).into()
            }
        }
        unsafe extern "system" fn ReceiptType<Identity: IFaxDocument_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, preceipttype: *mut FAX_RECEIPT_TYPE_ENUM) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFaxDocument_Impl::ReceiptType(this) {
                    Ok(ok__) => {
                        preceipttype.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetReceiptType<Identity: IFaxDocument_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, receipttype: FAX_RECEIPT_TYPE_ENUM) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFaxDocument_Impl::SetReceiptType(this, core::mem::transmute_copy(&receipttype)).into()
            }
        }
        unsafe extern "system" fn GroupBroadcastReceipts<Identity: IFaxDocument_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbusegrouping: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFaxDocument_Impl::GroupBroadcastReceipts(this) {
                    Ok(ok__) => {
                        pbusegrouping.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetGroupBroadcastReceipts<Identity: IFaxDocument_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, busegrouping: super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFaxDocument_Impl::SetGroupBroadcastReceipts(this, core::mem::transmute_copy(&busegrouping)).into()
            }
        }
        unsafe extern "system" fn Priority<Identity: IFaxDocument_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppriority: *mut FAX_PRIORITY_TYPE_ENUM) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFaxDocument_Impl::Priority(this) {
                    Ok(ok__) => {
                        ppriority.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetPriority<Identity: IFaxDocument_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, priority: FAX_PRIORITY_TYPE_ENUM) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFaxDocument_Impl::SetPriority(this, core::mem::transmute_copy(&priority)).into()
            }
        }
        unsafe extern "system" fn TapiConnection<Identity: IFaxDocument_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pptapiconnection: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFaxDocument_Impl::TapiConnection(this) {
                    Ok(ok__) => {
                        pptapiconnection.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn putref_TapiConnection<Identity: IFaxDocument_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ptapiconnection: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFaxDocument_Impl::putref_TapiConnection(this, core::mem::transmute_copy(&ptapiconnection)).into()
            }
        }
        unsafe extern "system" fn Submit<Identity: IFaxDocument_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrfaxservername: *mut core::ffi::c_void, pvfaxoutgoingjobids: *mut super::super::System::Variant::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFaxDocument_Impl::Submit(this, core::mem::transmute(&bstrfaxservername)) {
                    Ok(ok__) => {
                        pvfaxoutgoingjobids.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn ConnectedSubmit<Identity: IFaxDocument_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pfaxserver: *mut core::ffi::c_void, pvfaxoutgoingjobids: *mut super::super::System::Variant::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFaxDocument_Impl::ConnectedSubmit(this, core::mem::transmute_copy(&pfaxserver)) {
                    Ok(ok__) => {
                        pvfaxoutgoingjobids.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn AttachFaxToReceipt<Identity: IFaxDocument_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbattachfax: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFaxDocument_Impl::AttachFaxToReceipt(this) {
                    Ok(ok__) => {
                        pbattachfax.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetAttachFaxToReceipt<Identity: IFaxDocument_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, battachfax: super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFaxDocument_Impl::SetAttachFaxToReceipt(this, core::mem::transmute_copy(&battachfax)).into()
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            Body: Body::<Identity, OFFSET>,
            SetBody: SetBody::<Identity, OFFSET>,
            Sender: Sender::<Identity, OFFSET>,
            Recipients: Recipients::<Identity, OFFSET>,
            CoverPage: CoverPage::<Identity, OFFSET>,
            SetCoverPage: SetCoverPage::<Identity, OFFSET>,
            Subject: Subject::<Identity, OFFSET>,
            SetSubject: SetSubject::<Identity, OFFSET>,
            Note: Note::<Identity, OFFSET>,
            SetNote: SetNote::<Identity, OFFSET>,
            ScheduleTime: ScheduleTime::<Identity, OFFSET>,
            SetScheduleTime: SetScheduleTime::<Identity, OFFSET>,
            ReceiptAddress: ReceiptAddress::<Identity, OFFSET>,
            SetReceiptAddress: SetReceiptAddress::<Identity, OFFSET>,
            DocumentName: DocumentName::<Identity, OFFSET>,
            SetDocumentName: SetDocumentName::<Identity, OFFSET>,
            CallHandle: CallHandle::<Identity, OFFSET>,
            SetCallHandle: SetCallHandle::<Identity, OFFSET>,
            CoverPageType: CoverPageType::<Identity, OFFSET>,
            SetCoverPageType: SetCoverPageType::<Identity, OFFSET>,
            ScheduleType: ScheduleType::<Identity, OFFSET>,
            SetScheduleType: SetScheduleType::<Identity, OFFSET>,
            ReceiptType: ReceiptType::<Identity, OFFSET>,
            SetReceiptType: SetReceiptType::<Identity, OFFSET>,
            GroupBroadcastReceipts: GroupBroadcastReceipts::<Identity, OFFSET>,
            SetGroupBroadcastReceipts: SetGroupBroadcastReceipts::<Identity, OFFSET>,
            Priority: Priority::<Identity, OFFSET>,
            SetPriority: SetPriority::<Identity, OFFSET>,
            TapiConnection: TapiConnection::<Identity, OFFSET>,
            putref_TapiConnection: putref_TapiConnection::<Identity, OFFSET>,
            Submit: Submit::<Identity, OFFSET>,
            ConnectedSubmit: ConnectedSubmit::<Identity, OFFSET>,
            AttachFaxToReceipt: AttachFaxToReceipt::<Identity, OFFSET>,
            SetAttachFaxToReceipt: SetAttachFaxToReceipt::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IFaxDocument as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for IFaxDocument {}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IFaxDocument2, IFaxDocument2_Vtbl, 0xe1347661_f9ef_4d6d_b4a5_c0a068b65cff);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IFaxDocument2 {
    type Target = IFaxDocument;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IFaxDocument2, windows_core::IUnknown, super::super::System::Com::IDispatch, IFaxDocument);
#[cfg(feature = "Win32_System_Com")]
impl IFaxDocument2 {
    pub unsafe fn SubmissionId(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).SubmissionId)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn Bodies(&self) -> windows_core::Result<super::super::System::Variant::VARIANT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Bodies)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn SetBodies(&self, vbodies: &super::super::System::Variant::VARIANT) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetBodies)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(vbodies)).ok() }
    }
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn Submit2(&self, bstrfaxservername: &windows_core::BSTR, pvfaxoutgoingjobids: *mut super::super::System::Variant::VARIANT, plerrorbodyfile: *mut i32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Submit2)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrfaxservername), core::mem::transmute(pvfaxoutgoingjobids), plerrorbodyfile as _).ok() }
    }
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn ConnectedSubmit2<P0>(&self, pfaxserver: P0, pvfaxoutgoingjobids: *mut super::super::System::Variant::VARIANT, plerrorbodyfile: *mut i32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IFaxServer>,
    {
        unsafe { (windows_core::Interface::vtable(self).ConnectedSubmit2)(windows_core::Interface::as_raw(self), pfaxserver.param().abi(), core::mem::transmute(pvfaxoutgoingjobids), plerrorbodyfile as _).ok() }
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IFaxDocument2_Vtbl {
    pub base__: IFaxDocument_Vtbl,
    pub SubmissionId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub Bodies: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::System::Variant::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    Bodies: usize,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub SetBodies: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::System::Variant::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    SetBodies: usize,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub Submit2: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::System::Variant::VARIANT, *mut i32) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    Submit2: usize,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub ConnectedSubmit2: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::System::Variant::VARIANT, *mut i32) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    ConnectedSubmit2: usize,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IFaxDocument2_Impl: IFaxDocument_Impl {
    fn SubmissionId(&self) -> windows_core::Result<windows_core::BSTR>;
    fn Bodies(&self) -> windows_core::Result<super::super::System::Variant::VARIANT>;
    fn SetBodies(&self, vbodies: &super::super::System::Variant::VARIANT) -> windows_core::Result<()>;
    fn Submit2(&self, bstrfaxservername: &windows_core::BSTR, pvfaxoutgoingjobids: *mut super::super::System::Variant::VARIANT, plerrorbodyfile: *mut i32) -> windows_core::Result<()>;
    fn ConnectedSubmit2(&self, pfaxserver: windows_core::Ref<IFaxServer>, pvfaxoutgoingjobids: *mut super::super::System::Variant::VARIANT, plerrorbodyfile: *mut i32) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IFaxDocument2_Vtbl {
    pub const fn new<Identity: IFaxDocument2_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn SubmissionId<Identity: IFaxDocument2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrsubmissionid: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFaxDocument2_Impl::SubmissionId(this) {
                    Ok(ok__) => {
                        pbstrsubmissionid.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Bodies<Identity: IFaxDocument2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvbodies: *mut super::super::System::Variant::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFaxDocument2_Impl::Bodies(this) {
                    Ok(ok__) => {
                        pvbodies.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetBodies<Identity: IFaxDocument2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, vbodies: super::super::System::Variant::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFaxDocument2_Impl::SetBodies(this, core::mem::transmute(&vbodies)).into()
            }
        }
        unsafe extern "system" fn Submit2<Identity: IFaxDocument2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrfaxservername: *mut core::ffi::c_void, pvfaxoutgoingjobids: *mut super::super::System::Variant::VARIANT, plerrorbodyfile: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFaxDocument2_Impl::Submit2(this, core::mem::transmute(&bstrfaxservername), core::mem::transmute_copy(&pvfaxoutgoingjobids), core::mem::transmute_copy(&plerrorbodyfile)).into()
            }
        }
        unsafe extern "system" fn ConnectedSubmit2<Identity: IFaxDocument2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pfaxserver: *mut core::ffi::c_void, pvfaxoutgoingjobids: *mut super::super::System::Variant::VARIANT, plerrorbodyfile: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFaxDocument2_Impl::ConnectedSubmit2(this, core::mem::transmute_copy(&pfaxserver), core::mem::transmute_copy(&pvfaxoutgoingjobids), core::mem::transmute_copy(&plerrorbodyfile)).into()
            }
        }
        Self {
            base__: IFaxDocument_Vtbl::new::<Identity, OFFSET>(),
            SubmissionId: SubmissionId::<Identity, OFFSET>,
            Bodies: Bodies::<Identity, OFFSET>,
            SetBodies: SetBodies::<Identity, OFFSET>,
            Submit2: Submit2::<Identity, OFFSET>,
            ConnectedSubmit2: ConnectedSubmit2::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IFaxDocument2 as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID || iid == &<IFaxDocument as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for IFaxDocument2 {}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IFaxEventLogging, IFaxEventLogging_Vtbl, 0x0880d965_20e8_42e4_8e17_944f192caad4);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IFaxEventLogging {
    type Target = super::super::System::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IFaxEventLogging, windows_core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl IFaxEventLogging {
    pub unsafe fn InitEventsLevel(&self) -> windows_core::Result<FAX_LOG_LEVEL_ENUM> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).InitEventsLevel)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetInitEventsLevel(&self, initeventlevel: FAX_LOG_LEVEL_ENUM) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetInitEventsLevel)(windows_core::Interface::as_raw(self), initeventlevel).ok() }
    }
    pub unsafe fn InboundEventsLevel(&self) -> windows_core::Result<FAX_LOG_LEVEL_ENUM> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).InboundEventsLevel)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetInboundEventsLevel(&self, inboundeventlevel: FAX_LOG_LEVEL_ENUM) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetInboundEventsLevel)(windows_core::Interface::as_raw(self), inboundeventlevel).ok() }
    }
    pub unsafe fn OutboundEventsLevel(&self) -> windows_core::Result<FAX_LOG_LEVEL_ENUM> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).OutboundEventsLevel)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetOutboundEventsLevel(&self, outboundeventlevel: FAX_LOG_LEVEL_ENUM) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetOutboundEventsLevel)(windows_core::Interface::as_raw(self), outboundeventlevel).ok() }
    }
    pub unsafe fn GeneralEventsLevel(&self) -> windows_core::Result<FAX_LOG_LEVEL_ENUM> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GeneralEventsLevel)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetGeneralEventsLevel(&self, generaleventlevel: FAX_LOG_LEVEL_ENUM) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetGeneralEventsLevel)(windows_core::Interface::as_raw(self), generaleventlevel).ok() }
    }
    pub unsafe fn Refresh(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Refresh)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn Save(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Save)(windows_core::Interface::as_raw(self)).ok() }
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IFaxEventLogging_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub InitEventsLevel: unsafe extern "system" fn(*mut core::ffi::c_void, *mut FAX_LOG_LEVEL_ENUM) -> windows_core::HRESULT,
    pub SetInitEventsLevel: unsafe extern "system" fn(*mut core::ffi::c_void, FAX_LOG_LEVEL_ENUM) -> windows_core::HRESULT,
    pub InboundEventsLevel: unsafe extern "system" fn(*mut core::ffi::c_void, *mut FAX_LOG_LEVEL_ENUM) -> windows_core::HRESULT,
    pub SetInboundEventsLevel: unsafe extern "system" fn(*mut core::ffi::c_void, FAX_LOG_LEVEL_ENUM) -> windows_core::HRESULT,
    pub OutboundEventsLevel: unsafe extern "system" fn(*mut core::ffi::c_void, *mut FAX_LOG_LEVEL_ENUM) -> windows_core::HRESULT,
    pub SetOutboundEventsLevel: unsafe extern "system" fn(*mut core::ffi::c_void, FAX_LOG_LEVEL_ENUM) -> windows_core::HRESULT,
    pub GeneralEventsLevel: unsafe extern "system" fn(*mut core::ffi::c_void, *mut FAX_LOG_LEVEL_ENUM) -> windows_core::HRESULT,
    pub SetGeneralEventsLevel: unsafe extern "system" fn(*mut core::ffi::c_void, FAX_LOG_LEVEL_ENUM) -> windows_core::HRESULT,
    pub Refresh: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Save: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IFaxEventLogging_Impl: super::super::System::Com::IDispatch_Impl {
    fn InitEventsLevel(&self) -> windows_core::Result<FAX_LOG_LEVEL_ENUM>;
    fn SetInitEventsLevel(&self, initeventlevel: FAX_LOG_LEVEL_ENUM) -> windows_core::Result<()>;
    fn InboundEventsLevel(&self) -> windows_core::Result<FAX_LOG_LEVEL_ENUM>;
    fn SetInboundEventsLevel(&self, inboundeventlevel: FAX_LOG_LEVEL_ENUM) -> windows_core::Result<()>;
    fn OutboundEventsLevel(&self) -> windows_core::Result<FAX_LOG_LEVEL_ENUM>;
    fn SetOutboundEventsLevel(&self, outboundeventlevel: FAX_LOG_LEVEL_ENUM) -> windows_core::Result<()>;
    fn GeneralEventsLevel(&self) -> windows_core::Result<FAX_LOG_LEVEL_ENUM>;
    fn SetGeneralEventsLevel(&self, generaleventlevel: FAX_LOG_LEVEL_ENUM) -> windows_core::Result<()>;
    fn Refresh(&self) -> windows_core::Result<()>;
    fn Save(&self) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IFaxEventLogging_Vtbl {
    pub const fn new<Identity: IFaxEventLogging_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn InitEventsLevel<Identity: IFaxEventLogging_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, piniteventlevel: *mut FAX_LOG_LEVEL_ENUM) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFaxEventLogging_Impl::InitEventsLevel(this) {
                    Ok(ok__) => {
                        piniteventlevel.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetInitEventsLevel<Identity: IFaxEventLogging_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, initeventlevel: FAX_LOG_LEVEL_ENUM) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFaxEventLogging_Impl::SetInitEventsLevel(this, core::mem::transmute_copy(&initeventlevel)).into()
            }
        }
        unsafe extern "system" fn InboundEventsLevel<Identity: IFaxEventLogging_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pinboundeventlevel: *mut FAX_LOG_LEVEL_ENUM) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFaxEventLogging_Impl::InboundEventsLevel(this) {
                    Ok(ok__) => {
                        pinboundeventlevel.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetInboundEventsLevel<Identity: IFaxEventLogging_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, inboundeventlevel: FAX_LOG_LEVEL_ENUM) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFaxEventLogging_Impl::SetInboundEventsLevel(this, core::mem::transmute_copy(&inboundeventlevel)).into()
            }
        }
        unsafe extern "system" fn OutboundEventsLevel<Identity: IFaxEventLogging_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, poutboundeventlevel: *mut FAX_LOG_LEVEL_ENUM) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFaxEventLogging_Impl::OutboundEventsLevel(this) {
                    Ok(ok__) => {
                        poutboundeventlevel.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetOutboundEventsLevel<Identity: IFaxEventLogging_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, outboundeventlevel: FAX_LOG_LEVEL_ENUM) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFaxEventLogging_Impl::SetOutboundEventsLevel(this, core::mem::transmute_copy(&outboundeventlevel)).into()
            }
        }
        unsafe extern "system" fn GeneralEventsLevel<Identity: IFaxEventLogging_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pgeneraleventlevel: *mut FAX_LOG_LEVEL_ENUM) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFaxEventLogging_Impl::GeneralEventsLevel(this) {
                    Ok(ok__) => {
                        pgeneraleventlevel.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetGeneralEventsLevel<Identity: IFaxEventLogging_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, generaleventlevel: FAX_LOG_LEVEL_ENUM) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFaxEventLogging_Impl::SetGeneralEventsLevel(this, core::mem::transmute_copy(&generaleventlevel)).into()
            }
        }
        unsafe extern "system" fn Refresh<Identity: IFaxEventLogging_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFaxEventLogging_Impl::Refresh(this).into()
            }
        }
        unsafe extern "system" fn Save<Identity: IFaxEventLogging_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFaxEventLogging_Impl::Save(this).into()
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            InitEventsLevel: InitEventsLevel::<Identity, OFFSET>,
            SetInitEventsLevel: SetInitEventsLevel::<Identity, OFFSET>,
            InboundEventsLevel: InboundEventsLevel::<Identity, OFFSET>,
            SetInboundEventsLevel: SetInboundEventsLevel::<Identity, OFFSET>,
            OutboundEventsLevel: OutboundEventsLevel::<Identity, OFFSET>,
            SetOutboundEventsLevel: SetOutboundEventsLevel::<Identity, OFFSET>,
            GeneralEventsLevel: GeneralEventsLevel::<Identity, OFFSET>,
            SetGeneralEventsLevel: SetGeneralEventsLevel::<Identity, OFFSET>,
            Refresh: Refresh::<Identity, OFFSET>,
            Save: Save::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IFaxEventLogging as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for IFaxEventLogging {}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IFaxFolders, IFaxFolders_Vtbl, 0xdce3b2a8_a7ab_42bc_9d0a_3149457261a0);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IFaxFolders {
    type Target = super::super::System::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IFaxFolders, windows_core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl IFaxFolders {
    pub unsafe fn OutgoingQueue(&self) -> windows_core::Result<IFaxOutgoingQueue> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).OutgoingQueue)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn IncomingQueue(&self) -> windows_core::Result<IFaxIncomingQueue> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).IncomingQueue)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn IncomingArchive(&self) -> windows_core::Result<IFaxIncomingArchive> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).IncomingArchive)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn OutgoingArchive(&self) -> windows_core::Result<IFaxOutgoingArchive> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).OutgoingArchive)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IFaxFolders_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub OutgoingQueue: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub IncomingQueue: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub IncomingArchive: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub OutgoingArchive: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IFaxFolders_Impl: super::super::System::Com::IDispatch_Impl {
    fn OutgoingQueue(&self) -> windows_core::Result<IFaxOutgoingQueue>;
    fn IncomingQueue(&self) -> windows_core::Result<IFaxIncomingQueue>;
    fn IncomingArchive(&self) -> windows_core::Result<IFaxIncomingArchive>;
    fn OutgoingArchive(&self) -> windows_core::Result<IFaxOutgoingArchive>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IFaxFolders_Vtbl {
    pub const fn new<Identity: IFaxFolders_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn OutgoingQueue<Identity: IFaxFolders_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pfaxoutgoingqueue: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFaxFolders_Impl::OutgoingQueue(this) {
                    Ok(ok__) => {
                        pfaxoutgoingqueue.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn IncomingQueue<Identity: IFaxFolders_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pfaxincomingqueue: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFaxFolders_Impl::IncomingQueue(this) {
                    Ok(ok__) => {
                        pfaxincomingqueue.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn IncomingArchive<Identity: IFaxFolders_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pfaxincomingarchive: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFaxFolders_Impl::IncomingArchive(this) {
                    Ok(ok__) => {
                        pfaxincomingarchive.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn OutgoingArchive<Identity: IFaxFolders_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pfaxoutgoingarchive: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFaxFolders_Impl::OutgoingArchive(this) {
                    Ok(ok__) => {
                        pfaxoutgoingarchive.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            OutgoingQueue: OutgoingQueue::<Identity, OFFSET>,
            IncomingQueue: IncomingQueue::<Identity, OFFSET>,
            IncomingArchive: IncomingArchive::<Identity, OFFSET>,
            OutgoingArchive: OutgoingArchive::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IFaxFolders as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for IFaxFolders {}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IFaxInboundRouting, IFaxInboundRouting_Vtbl, 0x8148c20f_9d52_45b1_bf96_38fc12713527);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IFaxInboundRouting {
    type Target = super::super::System::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IFaxInboundRouting, windows_core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl IFaxInboundRouting {
    pub unsafe fn GetExtensions(&self) -> windows_core::Result<IFaxInboundRoutingExtensions> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetExtensions)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetMethods(&self) -> windows_core::Result<IFaxInboundRoutingMethods> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetMethods)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IFaxInboundRouting_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub GetExtensions: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetMethods: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IFaxInboundRouting_Impl: super::super::System::Com::IDispatch_Impl {
    fn GetExtensions(&self) -> windows_core::Result<IFaxInboundRoutingExtensions>;
    fn GetMethods(&self) -> windows_core::Result<IFaxInboundRoutingMethods>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IFaxInboundRouting_Vtbl {
    pub const fn new<Identity: IFaxInboundRouting_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetExtensions<Identity: IFaxInboundRouting_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pfaxinboundroutingextensions: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFaxInboundRouting_Impl::GetExtensions(this) {
                    Ok(ok__) => {
                        pfaxinboundroutingextensions.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetMethods<Identity: IFaxInboundRouting_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pfaxinboundroutingmethods: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFaxInboundRouting_Impl::GetMethods(this) {
                    Ok(ok__) => {
                        pfaxinboundroutingmethods.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            GetExtensions: GetExtensions::<Identity, OFFSET>,
            GetMethods: GetMethods::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IFaxInboundRouting as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for IFaxInboundRouting {}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IFaxInboundRoutingExtension, IFaxInboundRoutingExtension_Vtbl, 0x885b5e08_c26c_4ef9_af83_51580a750be1);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IFaxInboundRoutingExtension {
    type Target = super::super::System::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IFaxInboundRoutingExtension, windows_core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl IFaxInboundRoutingExtension {
    pub unsafe fn FriendlyName(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).FriendlyName)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn ImageName(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ImageName)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn UniqueName(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).UniqueName)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn MajorVersion(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).MajorVersion)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn MinorVersion(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).MinorVersion)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn MajorBuild(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).MajorBuild)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn MinorBuild(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).MinorBuild)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn Debug(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Debug)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn Status(&self) -> windows_core::Result<FAX_PROVIDER_STATUS_ENUM> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Status)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn InitErrorCode(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).InitErrorCode)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn Methods(&self) -> windows_core::Result<super::super::System::Variant::VARIANT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Methods)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IFaxInboundRoutingExtension_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub FriendlyName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ImageName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub UniqueName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub MajorVersion: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub MinorVersion: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub MajorBuild: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub MinorBuild: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub Debug: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub Status: unsafe extern "system" fn(*mut core::ffi::c_void, *mut FAX_PROVIDER_STATUS_ENUM) -> windows_core::HRESULT,
    pub InitErrorCode: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub Methods: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::System::Variant::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    Methods: usize,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IFaxInboundRoutingExtension_Impl: super::super::System::Com::IDispatch_Impl {
    fn FriendlyName(&self) -> windows_core::Result<windows_core::BSTR>;
    fn ImageName(&self) -> windows_core::Result<windows_core::BSTR>;
    fn UniqueName(&self) -> windows_core::Result<windows_core::BSTR>;
    fn MajorVersion(&self) -> windows_core::Result<i32>;
    fn MinorVersion(&self) -> windows_core::Result<i32>;
    fn MajorBuild(&self) -> windows_core::Result<i32>;
    fn MinorBuild(&self) -> windows_core::Result<i32>;
    fn Debug(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn Status(&self) -> windows_core::Result<FAX_PROVIDER_STATUS_ENUM>;
    fn InitErrorCode(&self) -> windows_core::Result<i32>;
    fn Methods(&self) -> windows_core::Result<super::super::System::Variant::VARIANT>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IFaxInboundRoutingExtension_Vtbl {
    pub const fn new<Identity: IFaxInboundRoutingExtension_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn FriendlyName<Identity: IFaxInboundRoutingExtension_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrfriendlyname: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFaxInboundRoutingExtension_Impl::FriendlyName(this) {
                    Ok(ok__) => {
                        pbstrfriendlyname.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn ImageName<Identity: IFaxInboundRoutingExtension_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrimagename: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFaxInboundRoutingExtension_Impl::ImageName(this) {
                    Ok(ok__) => {
                        pbstrimagename.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn UniqueName<Identity: IFaxInboundRoutingExtension_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstruniquename: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFaxInboundRoutingExtension_Impl::UniqueName(this) {
                    Ok(ok__) => {
                        pbstruniquename.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn MajorVersion<Identity: IFaxInboundRoutingExtension_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, plmajorversion: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFaxInboundRoutingExtension_Impl::MajorVersion(this) {
                    Ok(ok__) => {
                        plmajorversion.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn MinorVersion<Identity: IFaxInboundRoutingExtension_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, plminorversion: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFaxInboundRoutingExtension_Impl::MinorVersion(this) {
                    Ok(ok__) => {
                        plminorversion.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn MajorBuild<Identity: IFaxInboundRoutingExtension_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, plmajorbuild: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFaxInboundRoutingExtension_Impl::MajorBuild(this) {
                    Ok(ok__) => {
                        plmajorbuild.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn MinorBuild<Identity: IFaxInboundRoutingExtension_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, plminorbuild: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFaxInboundRoutingExtension_Impl::MinorBuild(this) {
                    Ok(ok__) => {
                        plminorbuild.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Debug<Identity: IFaxInboundRoutingExtension_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbdebug: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFaxInboundRoutingExtension_Impl::Debug(this) {
                    Ok(ok__) => {
                        pbdebug.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Status<Identity: IFaxInboundRoutingExtension_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pstatus: *mut FAX_PROVIDER_STATUS_ENUM) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFaxInboundRoutingExtension_Impl::Status(this) {
                    Ok(ok__) => {
                        pstatus.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn InitErrorCode<Identity: IFaxInboundRoutingExtension_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pliniterrorcode: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFaxInboundRoutingExtension_Impl::InitErrorCode(this) {
                    Ok(ok__) => {
                        pliniterrorcode.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Methods<Identity: IFaxInboundRoutingExtension_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvmethods: *mut super::super::System::Variant::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFaxInboundRoutingExtension_Impl::Methods(this) {
                    Ok(ok__) => {
                        pvmethods.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            FriendlyName: FriendlyName::<Identity, OFFSET>,
            ImageName: ImageName::<Identity, OFFSET>,
            UniqueName: UniqueName::<Identity, OFFSET>,
            MajorVersion: MajorVersion::<Identity, OFFSET>,
            MinorVersion: MinorVersion::<Identity, OFFSET>,
            MajorBuild: MajorBuild::<Identity, OFFSET>,
            MinorBuild: MinorBuild::<Identity, OFFSET>,
            Debug: Debug::<Identity, OFFSET>,
            Status: Status::<Identity, OFFSET>,
            InitErrorCode: InitErrorCode::<Identity, OFFSET>,
            Methods: Methods::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IFaxInboundRoutingExtension as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for IFaxInboundRoutingExtension {}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IFaxInboundRoutingExtensions, IFaxInboundRoutingExtensions_Vtbl, 0x2f6c9673_7b26_42de_8eb0_915dcd2a4f4c);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IFaxInboundRoutingExtensions {
    type Target = super::super::System::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IFaxInboundRoutingExtensions, windows_core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl IFaxInboundRoutingExtensions {
    pub unsafe fn _NewEnum(&self) -> windows_core::Result<windows_core::IUnknown> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self)._NewEnum)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn get_Item(&self, vindex: &super::super::System::Variant::VARIANT) -> windows_core::Result<IFaxInboundRoutingExtension> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_Item)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(vindex), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn Count(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Count)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IFaxInboundRoutingExtensions_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub _NewEnum: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub get_Item: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::System::Variant::VARIANT, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    get_Item: usize,
    pub Count: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IFaxInboundRoutingExtensions_Impl: super::super::System::Com::IDispatch_Impl {
    fn _NewEnum(&self) -> windows_core::Result<windows_core::IUnknown>;
    fn get_Item(&self, vindex: &super::super::System::Variant::VARIANT) -> windows_core::Result<IFaxInboundRoutingExtension>;
    fn Count(&self) -> windows_core::Result<i32>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IFaxInboundRoutingExtensions_Vtbl {
    pub const fn new<Identity: IFaxInboundRoutingExtensions_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn _NewEnum<Identity: IFaxInboundRoutingExtensions_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppunk: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFaxInboundRoutingExtensions_Impl::_NewEnum(this) {
                    Ok(ok__) => {
                        ppunk.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_Item<Identity: IFaxInboundRoutingExtensions_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, vindex: super::super::System::Variant::VARIANT, pfaxinboundroutingextension: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFaxInboundRoutingExtensions_Impl::get_Item(this, core::mem::transmute(&vindex)) {
                    Ok(ok__) => {
                        pfaxinboundroutingextension.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Count<Identity: IFaxInboundRoutingExtensions_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, plcount: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFaxInboundRoutingExtensions_Impl::Count(this) {
                    Ok(ok__) => {
                        plcount.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            _NewEnum: _NewEnum::<Identity, OFFSET>,
            get_Item: get_Item::<Identity, OFFSET>,
            Count: Count::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IFaxInboundRoutingExtensions as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for IFaxInboundRoutingExtensions {}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IFaxInboundRoutingMethod, IFaxInboundRoutingMethod_Vtbl, 0x45700061_ad9d_4776_a8c4_64065492cf4b);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IFaxInboundRoutingMethod {
    type Target = super::super::System::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IFaxInboundRoutingMethod, windows_core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl IFaxInboundRoutingMethod {
    pub unsafe fn Name(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Name)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn GUID(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GUID)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn FunctionName(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).FunctionName)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn ExtensionFriendlyName(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ExtensionFriendlyName)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn ExtensionImageName(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ExtensionImageName)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn Priority(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Priority)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetPriority(&self, lpriority: i32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetPriority)(windows_core::Interface::as_raw(self), lpriority).ok() }
    }
    pub unsafe fn Refresh(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Refresh)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn Save(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Save)(windows_core::Interface::as_raw(self)).ok() }
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IFaxInboundRoutingMethod_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub Name: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GUID: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub FunctionName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ExtensionFriendlyName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ExtensionImageName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Priority: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetPriority: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub Refresh: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Save: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IFaxInboundRoutingMethod_Impl: super::super::System::Com::IDispatch_Impl {
    fn Name(&self) -> windows_core::Result<windows_core::BSTR>;
    fn GUID(&self) -> windows_core::Result<windows_core::BSTR>;
    fn FunctionName(&self) -> windows_core::Result<windows_core::BSTR>;
    fn ExtensionFriendlyName(&self) -> windows_core::Result<windows_core::BSTR>;
    fn ExtensionImageName(&self) -> windows_core::Result<windows_core::BSTR>;
    fn Priority(&self) -> windows_core::Result<i32>;
    fn SetPriority(&self, lpriority: i32) -> windows_core::Result<()>;
    fn Refresh(&self) -> windows_core::Result<()>;
    fn Save(&self) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IFaxInboundRoutingMethod_Vtbl {
    pub const fn new<Identity: IFaxInboundRoutingMethod_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Name<Identity: IFaxInboundRoutingMethod_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrname: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFaxInboundRoutingMethod_Impl::Name(this) {
                    Ok(ok__) => {
                        pbstrname.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GUID<Identity: IFaxInboundRoutingMethod_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrguid: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFaxInboundRoutingMethod_Impl::GUID(this) {
                    Ok(ok__) => {
                        pbstrguid.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn FunctionName<Identity: IFaxInboundRoutingMethod_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrfunctionname: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFaxInboundRoutingMethod_Impl::FunctionName(this) {
                    Ok(ok__) => {
                        pbstrfunctionname.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn ExtensionFriendlyName<Identity: IFaxInboundRoutingMethod_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrextensionfriendlyname: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFaxInboundRoutingMethod_Impl::ExtensionFriendlyName(this) {
                    Ok(ok__) => {
                        pbstrextensionfriendlyname.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn ExtensionImageName<Identity: IFaxInboundRoutingMethod_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrextensionimagename: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFaxInboundRoutingMethod_Impl::ExtensionImageName(this) {
                    Ok(ok__) => {
                        pbstrextensionimagename.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Priority<Identity: IFaxInboundRoutingMethod_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, plpriority: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFaxInboundRoutingMethod_Impl::Priority(this) {
                    Ok(ok__) => {
                        plpriority.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetPriority<Identity: IFaxInboundRoutingMethod_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lpriority: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFaxInboundRoutingMethod_Impl::SetPriority(this, core::mem::transmute_copy(&lpriority)).into()
            }
        }
        unsafe extern "system" fn Refresh<Identity: IFaxInboundRoutingMethod_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFaxInboundRoutingMethod_Impl::Refresh(this).into()
            }
        }
        unsafe extern "system" fn Save<Identity: IFaxInboundRoutingMethod_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFaxInboundRoutingMethod_Impl::Save(this).into()
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            Name: Name::<Identity, OFFSET>,
            GUID: GUID::<Identity, OFFSET>,
            FunctionName: FunctionName::<Identity, OFFSET>,
            ExtensionFriendlyName: ExtensionFriendlyName::<Identity, OFFSET>,
            ExtensionImageName: ExtensionImageName::<Identity, OFFSET>,
            Priority: Priority::<Identity, OFFSET>,
            SetPriority: SetPriority::<Identity, OFFSET>,
            Refresh: Refresh::<Identity, OFFSET>,
            Save: Save::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IFaxInboundRoutingMethod as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for IFaxInboundRoutingMethod {}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IFaxInboundRoutingMethods, IFaxInboundRoutingMethods_Vtbl, 0x783fca10_8908_4473_9d69_f67fbea0c6b9);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IFaxInboundRoutingMethods {
    type Target = super::super::System::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IFaxInboundRoutingMethods, windows_core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl IFaxInboundRoutingMethods {
    pub unsafe fn _NewEnum(&self) -> windows_core::Result<windows_core::IUnknown> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self)._NewEnum)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn get_Item(&self, vindex: &super::super::System::Variant::VARIANT) -> windows_core::Result<IFaxInboundRoutingMethod> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_Item)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(vindex), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn Count(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Count)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IFaxInboundRoutingMethods_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub _NewEnum: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub get_Item: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::System::Variant::VARIANT, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    get_Item: usize,
    pub Count: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IFaxInboundRoutingMethods_Impl: super::super::System::Com::IDispatch_Impl {
    fn _NewEnum(&self) -> windows_core::Result<windows_core::IUnknown>;
    fn get_Item(&self, vindex: &super::super::System::Variant::VARIANT) -> windows_core::Result<IFaxInboundRoutingMethod>;
    fn Count(&self) -> windows_core::Result<i32>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IFaxInboundRoutingMethods_Vtbl {
    pub const fn new<Identity: IFaxInboundRoutingMethods_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn _NewEnum<Identity: IFaxInboundRoutingMethods_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppunk: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFaxInboundRoutingMethods_Impl::_NewEnum(this) {
                    Ok(ok__) => {
                        ppunk.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_Item<Identity: IFaxInboundRoutingMethods_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, vindex: super::super::System::Variant::VARIANT, pfaxinboundroutingmethod: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFaxInboundRoutingMethods_Impl::get_Item(this, core::mem::transmute(&vindex)) {
                    Ok(ok__) => {
                        pfaxinboundroutingmethod.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Count<Identity: IFaxInboundRoutingMethods_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, plcount: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFaxInboundRoutingMethods_Impl::Count(this) {
                    Ok(ok__) => {
                        plcount.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            _NewEnum: _NewEnum::<Identity, OFFSET>,
            get_Item: get_Item::<Identity, OFFSET>,
            Count: Count::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IFaxInboundRoutingMethods as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for IFaxInboundRoutingMethods {}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IFaxIncomingArchive, IFaxIncomingArchive_Vtbl, 0x76062cc7_f714_4fbd_aa06_ed6e4a4b70f3);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IFaxIncomingArchive {
    type Target = super::super::System::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IFaxIncomingArchive, windows_core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl IFaxIncomingArchive {
    pub unsafe fn UseArchive(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).UseArchive)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetUseArchive(&self, busearchive: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetUseArchive)(windows_core::Interface::as_raw(self), busearchive).ok() }
    }
    pub unsafe fn ArchiveFolder(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ArchiveFolder)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn SetArchiveFolder(&self, bstrarchivefolder: &windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetArchiveFolder)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrarchivefolder)).ok() }
    }
    pub unsafe fn SizeQuotaWarning(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).SizeQuotaWarning)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetSizeQuotaWarning(&self, bsizequotawarning: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetSizeQuotaWarning)(windows_core::Interface::as_raw(self), bsizequotawarning).ok() }
    }
    pub unsafe fn HighQuotaWaterMark(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).HighQuotaWaterMark)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetHighQuotaWaterMark(&self, lhighquotawatermark: i32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetHighQuotaWaterMark)(windows_core::Interface::as_raw(self), lhighquotawatermark).ok() }
    }
    pub unsafe fn LowQuotaWaterMark(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).LowQuotaWaterMark)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetLowQuotaWaterMark(&self, llowquotawatermark: i32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetLowQuotaWaterMark)(windows_core::Interface::as_raw(self), llowquotawatermark).ok() }
    }
    pub unsafe fn AgeLimit(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).AgeLimit)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetAgeLimit(&self, lagelimit: i32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetAgeLimit)(windows_core::Interface::as_raw(self), lagelimit).ok() }
    }
    pub unsafe fn SizeLow(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).SizeLow)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SizeHigh(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).SizeHigh)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn Refresh(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Refresh)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn Save(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Save)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn GetMessages(&self, lprefetchsize: i32) -> windows_core::Result<IFaxIncomingMessageIterator> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetMessages)(windows_core::Interface::as_raw(self), lprefetchsize, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetMessage(&self, bstrmessageid: &windows_core::BSTR) -> windows_core::Result<IFaxIncomingMessage> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetMessage)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrmessageid), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IFaxIncomingArchive_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub UseArchive: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub SetUseArchive: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub ArchiveFolder: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetArchiveFolder: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SizeQuotaWarning: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub SetSizeQuotaWarning: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub HighQuotaWaterMark: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetHighQuotaWaterMark: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub LowQuotaWaterMark: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetLowQuotaWaterMark: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub AgeLimit: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetAgeLimit: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub SizeLow: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SizeHigh: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub Refresh: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Save: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetMessages: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetMessage: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IFaxIncomingArchive_Impl: super::super::System::Com::IDispatch_Impl {
    fn UseArchive(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn SetUseArchive(&self, busearchive: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
    fn ArchiveFolder(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetArchiveFolder(&self, bstrarchivefolder: &windows_core::BSTR) -> windows_core::Result<()>;
    fn SizeQuotaWarning(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn SetSizeQuotaWarning(&self, bsizequotawarning: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
    fn HighQuotaWaterMark(&self) -> windows_core::Result<i32>;
    fn SetHighQuotaWaterMark(&self, lhighquotawatermark: i32) -> windows_core::Result<()>;
    fn LowQuotaWaterMark(&self) -> windows_core::Result<i32>;
    fn SetLowQuotaWaterMark(&self, llowquotawatermark: i32) -> windows_core::Result<()>;
    fn AgeLimit(&self) -> windows_core::Result<i32>;
    fn SetAgeLimit(&self, lagelimit: i32) -> windows_core::Result<()>;
    fn SizeLow(&self) -> windows_core::Result<i32>;
    fn SizeHigh(&self) -> windows_core::Result<i32>;
    fn Refresh(&self) -> windows_core::Result<()>;
    fn Save(&self) -> windows_core::Result<()>;
    fn GetMessages(&self, lprefetchsize: i32) -> windows_core::Result<IFaxIncomingMessageIterator>;
    fn GetMessage(&self, bstrmessageid: &windows_core::BSTR) -> windows_core::Result<IFaxIncomingMessage>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IFaxIncomingArchive_Vtbl {
    pub const fn new<Identity: IFaxIncomingArchive_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn UseArchive<Identity: IFaxIncomingArchive_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbusearchive: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFaxIncomingArchive_Impl::UseArchive(this) {
                    Ok(ok__) => {
                        pbusearchive.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetUseArchive<Identity: IFaxIncomingArchive_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, busearchive: super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFaxIncomingArchive_Impl::SetUseArchive(this, core::mem::transmute_copy(&busearchive)).into()
            }
        }
        unsafe extern "system" fn ArchiveFolder<Identity: IFaxIncomingArchive_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrarchivefolder: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFaxIncomingArchive_Impl::ArchiveFolder(this) {
                    Ok(ok__) => {
                        pbstrarchivefolder.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetArchiveFolder<Identity: IFaxIncomingArchive_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrarchivefolder: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFaxIncomingArchive_Impl::SetArchiveFolder(this, core::mem::transmute(&bstrarchivefolder)).into()
            }
        }
        unsafe extern "system" fn SizeQuotaWarning<Identity: IFaxIncomingArchive_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbsizequotawarning: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFaxIncomingArchive_Impl::SizeQuotaWarning(this) {
                    Ok(ok__) => {
                        pbsizequotawarning.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetSizeQuotaWarning<Identity: IFaxIncomingArchive_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bsizequotawarning: super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFaxIncomingArchive_Impl::SetSizeQuotaWarning(this, core::mem::transmute_copy(&bsizequotawarning)).into()
            }
        }
        unsafe extern "system" fn HighQuotaWaterMark<Identity: IFaxIncomingArchive_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, plhighquotawatermark: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFaxIncomingArchive_Impl::HighQuotaWaterMark(this) {
                    Ok(ok__) => {
                        plhighquotawatermark.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetHighQuotaWaterMark<Identity: IFaxIncomingArchive_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lhighquotawatermark: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFaxIncomingArchive_Impl::SetHighQuotaWaterMark(this, core::mem::transmute_copy(&lhighquotawatermark)).into()
            }
        }
        unsafe extern "system" fn LowQuotaWaterMark<Identity: IFaxIncomingArchive_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pllowquotawatermark: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFaxIncomingArchive_Impl::LowQuotaWaterMark(this) {
                    Ok(ok__) => {
                        pllowquotawatermark.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetLowQuotaWaterMark<Identity: IFaxIncomingArchive_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, llowquotawatermark: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFaxIncomingArchive_Impl::SetLowQuotaWaterMark(this, core::mem::transmute_copy(&llowquotawatermark)).into()
            }
        }
        unsafe extern "system" fn AgeLimit<Identity: IFaxIncomingArchive_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, plagelimit: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFaxIncomingArchive_Impl::AgeLimit(this) {
                    Ok(ok__) => {
                        plagelimit.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetAgeLimit<Identity: IFaxIncomingArchive_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lagelimit: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFaxIncomingArchive_Impl::SetAgeLimit(this, core::mem::transmute_copy(&lagelimit)).into()
            }
        }
        unsafe extern "system" fn SizeLow<Identity: IFaxIncomingArchive_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, plsizelow: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFaxIncomingArchive_Impl::SizeLow(this) {
                    Ok(ok__) => {
                        plsizelow.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SizeHigh<Identity: IFaxIncomingArchive_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, plsizehigh: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFaxIncomingArchive_Impl::SizeHigh(this) {
                    Ok(ok__) => {
                        plsizehigh.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Refresh<Identity: IFaxIncomingArchive_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFaxIncomingArchive_Impl::Refresh(this).into()
            }
        }
        unsafe extern "system" fn Save<Identity: IFaxIncomingArchive_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFaxIncomingArchive_Impl::Save(this).into()
            }
        }
        unsafe extern "system" fn GetMessages<Identity: IFaxIncomingArchive_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lprefetchsize: i32, pfaxincomingmessageiterator: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFaxIncomingArchive_Impl::GetMessages(this, core::mem::transmute_copy(&lprefetchsize)) {
                    Ok(ok__) => {
                        pfaxincomingmessageiterator.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetMessage<Identity: IFaxIncomingArchive_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrmessageid: *mut core::ffi::c_void, pfaxincomingmessage: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFaxIncomingArchive_Impl::GetMessage(this, core::mem::transmute(&bstrmessageid)) {
                    Ok(ok__) => {
                        pfaxincomingmessage.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            UseArchive: UseArchive::<Identity, OFFSET>,
            SetUseArchive: SetUseArchive::<Identity, OFFSET>,
            ArchiveFolder: ArchiveFolder::<Identity, OFFSET>,
            SetArchiveFolder: SetArchiveFolder::<Identity, OFFSET>,
            SizeQuotaWarning: SizeQuotaWarning::<Identity, OFFSET>,
            SetSizeQuotaWarning: SetSizeQuotaWarning::<Identity, OFFSET>,
            HighQuotaWaterMark: HighQuotaWaterMark::<Identity, OFFSET>,
            SetHighQuotaWaterMark: SetHighQuotaWaterMark::<Identity, OFFSET>,
            LowQuotaWaterMark: LowQuotaWaterMark::<Identity, OFFSET>,
            SetLowQuotaWaterMark: SetLowQuotaWaterMark::<Identity, OFFSET>,
            AgeLimit: AgeLimit::<Identity, OFFSET>,
            SetAgeLimit: SetAgeLimit::<Identity, OFFSET>,
            SizeLow: SizeLow::<Identity, OFFSET>,
            SizeHigh: SizeHigh::<Identity, OFFSET>,
            Refresh: Refresh::<Identity, OFFSET>,
            Save: Save::<Identity, OFFSET>,
            GetMessages: GetMessages::<Identity, OFFSET>,
            GetMessage: GetMessage::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IFaxIncomingArchive as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for IFaxIncomingArchive {}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IFaxIncomingJob, IFaxIncomingJob_Vtbl, 0x207529e6_654a_4916_9f88_4d232ee8a107);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IFaxIncomingJob {
    type Target = super::super::System::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IFaxIncomingJob, windows_core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl IFaxIncomingJob {
    pub unsafe fn Size(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Size)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn Id(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Id)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn CurrentPage(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CurrentPage)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn DeviceId(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).DeviceId)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn Status(&self) -> windows_core::Result<FAX_JOB_STATUS_ENUM> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Status)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn ExtendedStatusCode(&self) -> windows_core::Result<FAX_JOB_EXTENDED_STATUS_ENUM> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ExtendedStatusCode)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn ExtendedStatus(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ExtendedStatus)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn AvailableOperations(&self) -> windows_core::Result<FAX_JOB_OPERATIONS_ENUM> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).AvailableOperations)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn Retries(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Retries)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn TransmissionStart(&self) -> windows_core::Result<f64> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).TransmissionStart)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn TransmissionEnd(&self) -> windows_core::Result<f64> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).TransmissionEnd)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn CSID(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CSID)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn TSID(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).TSID)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn CallerId(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CallerId)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn RoutingInformation(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).RoutingInformation)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn JobType(&self) -> windows_core::Result<FAX_JOB_TYPE_ENUM> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).JobType)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn Cancel(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Cancel)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn Refresh(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Refresh)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn CopyTiff(&self, bstrtiffpath: &windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).CopyTiff)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrtiffpath)).ok() }
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IFaxIncomingJob_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub Size: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub Id: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CurrentPage: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub DeviceId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub Status: unsafe extern "system" fn(*mut core::ffi::c_void, *mut FAX_JOB_STATUS_ENUM) -> windows_core::HRESULT,
    pub ExtendedStatusCode: unsafe extern "system" fn(*mut core::ffi::c_void, *mut FAX_JOB_EXTENDED_STATUS_ENUM) -> windows_core::HRESULT,
    pub ExtendedStatus: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub AvailableOperations: unsafe extern "system" fn(*mut core::ffi::c_void, *mut FAX_JOB_OPERATIONS_ENUM) -> windows_core::HRESULT,
    pub Retries: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub TransmissionStart: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f64) -> windows_core::HRESULT,
    pub TransmissionEnd: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f64) -> windows_core::HRESULT,
    pub CSID: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub TSID: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CallerId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub RoutingInformation: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub JobType: unsafe extern "system" fn(*mut core::ffi::c_void, *mut FAX_JOB_TYPE_ENUM) -> windows_core::HRESULT,
    pub Cancel: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Refresh: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CopyTiff: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IFaxIncomingJob_Impl: super::super::System::Com::IDispatch_Impl {
    fn Size(&self) -> windows_core::Result<i32>;
    fn Id(&self) -> windows_core::Result<windows_core::BSTR>;
    fn CurrentPage(&self) -> windows_core::Result<i32>;
    fn DeviceId(&self) -> windows_core::Result<i32>;
    fn Status(&self) -> windows_core::Result<FAX_JOB_STATUS_ENUM>;
    fn ExtendedStatusCode(&self) -> windows_core::Result<FAX_JOB_EXTENDED_STATUS_ENUM>;
    fn ExtendedStatus(&self) -> windows_core::Result<windows_core::BSTR>;
    fn AvailableOperations(&self) -> windows_core::Result<FAX_JOB_OPERATIONS_ENUM>;
    fn Retries(&self) -> windows_core::Result<i32>;
    fn TransmissionStart(&self) -> windows_core::Result<f64>;
    fn TransmissionEnd(&self) -> windows_core::Result<f64>;
    fn CSID(&self) -> windows_core::Result<windows_core::BSTR>;
    fn TSID(&self) -> windows_core::Result<windows_core::BSTR>;
    fn CallerId(&self) -> windows_core::Result<windows_core::BSTR>;
    fn RoutingInformation(&self) -> windows_core::Result<windows_core::BSTR>;
    fn JobType(&self) -> windows_core::Result<FAX_JOB_TYPE_ENUM>;
    fn Cancel(&self) -> windows_core::Result<()>;
    fn Refresh(&self) -> windows_core::Result<()>;
    fn CopyTiff(&self, bstrtiffpath: &windows_core::BSTR) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IFaxIncomingJob_Vtbl {
    pub const fn new<Identity: IFaxIncomingJob_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Size<Identity: IFaxIncomingJob_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, plsize: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFaxIncomingJob_Impl::Size(this) {
                    Ok(ok__) => {
                        plsize.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Id<Identity: IFaxIncomingJob_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrid: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFaxIncomingJob_Impl::Id(this) {
                    Ok(ok__) => {
                        pbstrid.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CurrentPage<Identity: IFaxIncomingJob_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, plcurrentpage: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFaxIncomingJob_Impl::CurrentPage(this) {
                    Ok(ok__) => {
                        plcurrentpage.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn DeviceId<Identity: IFaxIncomingJob_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pldeviceid: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFaxIncomingJob_Impl::DeviceId(this) {
                    Ok(ok__) => {
                        pldeviceid.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Status<Identity: IFaxIncomingJob_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pstatus: *mut FAX_JOB_STATUS_ENUM) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFaxIncomingJob_Impl::Status(this) {
                    Ok(ok__) => {
                        pstatus.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn ExtendedStatusCode<Identity: IFaxIncomingJob_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pextendedstatuscode: *mut FAX_JOB_EXTENDED_STATUS_ENUM) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFaxIncomingJob_Impl::ExtendedStatusCode(this) {
                    Ok(ok__) => {
                        pextendedstatuscode.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn ExtendedStatus<Identity: IFaxIncomingJob_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrextendedstatus: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFaxIncomingJob_Impl::ExtendedStatus(this) {
                    Ok(ok__) => {
                        pbstrextendedstatus.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn AvailableOperations<Identity: IFaxIncomingJob_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pavailableoperations: *mut FAX_JOB_OPERATIONS_ENUM) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFaxIncomingJob_Impl::AvailableOperations(this) {
                    Ok(ok__) => {
                        pavailableoperations.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Retries<Identity: IFaxIncomingJob_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, plretries: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFaxIncomingJob_Impl::Retries(this) {
                    Ok(ok__) => {
                        plretries.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn TransmissionStart<Identity: IFaxIncomingJob_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdatetransmissionstart: *mut f64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFaxIncomingJob_Impl::TransmissionStart(this) {
                    Ok(ok__) => {
                        pdatetransmissionstart.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn TransmissionEnd<Identity: IFaxIncomingJob_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdatetransmissionend: *mut f64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFaxIncomingJob_Impl::TransmissionEnd(this) {
                    Ok(ok__) => {
                        pdatetransmissionend.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CSID<Identity: IFaxIncomingJob_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrcsid: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFaxIncomingJob_Impl::CSID(this) {
                    Ok(ok__) => {
                        pbstrcsid.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn TSID<Identity: IFaxIncomingJob_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrtsid: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFaxIncomingJob_Impl::TSID(this) {
                    Ok(ok__) => {
                        pbstrtsid.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CallerId<Identity: IFaxIncomingJob_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrcallerid: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFaxIncomingJob_Impl::CallerId(this) {
                    Ok(ok__) => {
                        pbstrcallerid.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn RoutingInformation<Identity: IFaxIncomingJob_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrroutinginformation: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFaxIncomingJob_Impl::RoutingInformation(this) {
                    Ok(ok__) => {
                        pbstrroutinginformation.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn JobType<Identity: IFaxIncomingJob_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pjobtype: *mut FAX_JOB_TYPE_ENUM) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFaxIncomingJob_Impl::JobType(this) {
                    Ok(ok__) => {
                        pjobtype.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Cancel<Identity: IFaxIncomingJob_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFaxIncomingJob_Impl::Cancel(this).into()
            }
        }
        unsafe extern "system" fn Refresh<Identity: IFaxIncomingJob_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFaxIncomingJob_Impl::Refresh(this).into()
            }
        }
        unsafe extern "system" fn CopyTiff<Identity: IFaxIncomingJob_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrtiffpath: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFaxIncomingJob_Impl::CopyTiff(this, core::mem::transmute(&bstrtiffpath)).into()
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            Size: Size::<Identity, OFFSET>,
            Id: Id::<Identity, OFFSET>,
            CurrentPage: CurrentPage::<Identity, OFFSET>,
            DeviceId: DeviceId::<Identity, OFFSET>,
            Status: Status::<Identity, OFFSET>,
            ExtendedStatusCode: ExtendedStatusCode::<Identity, OFFSET>,
            ExtendedStatus: ExtendedStatus::<Identity, OFFSET>,
            AvailableOperations: AvailableOperations::<Identity, OFFSET>,
            Retries: Retries::<Identity, OFFSET>,
            TransmissionStart: TransmissionStart::<Identity, OFFSET>,
            TransmissionEnd: TransmissionEnd::<Identity, OFFSET>,
            CSID: CSID::<Identity, OFFSET>,
            TSID: TSID::<Identity, OFFSET>,
            CallerId: CallerId::<Identity, OFFSET>,
            RoutingInformation: RoutingInformation::<Identity, OFFSET>,
            JobType: JobType::<Identity, OFFSET>,
            Cancel: Cancel::<Identity, OFFSET>,
            Refresh: Refresh::<Identity, OFFSET>,
            CopyTiff: CopyTiff::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IFaxIncomingJob as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for IFaxIncomingJob {}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IFaxIncomingJobs, IFaxIncomingJobs_Vtbl, 0x011f04e9_4fd6_4c23_9513_b6b66bb26be9);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IFaxIncomingJobs {
    type Target = super::super::System::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IFaxIncomingJobs, windows_core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl IFaxIncomingJobs {
    pub unsafe fn _NewEnum(&self) -> windows_core::Result<windows_core::IUnknown> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self)._NewEnum)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn get_Item(&self, vindex: &super::super::System::Variant::VARIANT) -> windows_core::Result<IFaxIncomingJob> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_Item)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(vindex), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn Count(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Count)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IFaxIncomingJobs_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub _NewEnum: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub get_Item: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::System::Variant::VARIANT, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    get_Item: usize,
    pub Count: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IFaxIncomingJobs_Impl: super::super::System::Com::IDispatch_Impl {
    fn _NewEnum(&self) -> windows_core::Result<windows_core::IUnknown>;
    fn get_Item(&self, vindex: &super::super::System::Variant::VARIANT) -> windows_core::Result<IFaxIncomingJob>;
    fn Count(&self) -> windows_core::Result<i32>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IFaxIncomingJobs_Vtbl {
    pub const fn new<Identity: IFaxIncomingJobs_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn _NewEnum<Identity: IFaxIncomingJobs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppunk: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFaxIncomingJobs_Impl::_NewEnum(this) {
                    Ok(ok__) => {
                        ppunk.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_Item<Identity: IFaxIncomingJobs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, vindex: super::super::System::Variant::VARIANT, pfaxincomingjob: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFaxIncomingJobs_Impl::get_Item(this, core::mem::transmute(&vindex)) {
                    Ok(ok__) => {
                        pfaxincomingjob.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Count<Identity: IFaxIncomingJobs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, plcount: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFaxIncomingJobs_Impl::Count(this) {
                    Ok(ok__) => {
                        plcount.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            _NewEnum: _NewEnum::<Identity, OFFSET>,
            get_Item: get_Item::<Identity, OFFSET>,
            Count: Count::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IFaxIncomingJobs as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for IFaxIncomingJobs {}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IFaxIncomingMessage, IFaxIncomingMessage_Vtbl, 0x7cab88fa_2ef9_4851_b2f3_1d148fed8447);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IFaxIncomingMessage {
    type Target = super::super::System::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IFaxIncomingMessage, windows_core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl IFaxIncomingMessage {
    pub unsafe fn Id(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Id)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn Pages(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Pages)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn Size(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Size)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn DeviceName(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).DeviceName)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn Retries(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Retries)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn TransmissionStart(&self) -> windows_core::Result<f64> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).TransmissionStart)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn TransmissionEnd(&self) -> windows_core::Result<f64> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).TransmissionEnd)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn CSID(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CSID)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn TSID(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).TSID)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn CallerId(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CallerId)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn RoutingInformation(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).RoutingInformation)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn CopyTiff(&self, bstrtiffpath: &windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).CopyTiff)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrtiffpath)).ok() }
    }
    pub unsafe fn Delete(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Delete)(windows_core::Interface::as_raw(self)).ok() }
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IFaxIncomingMessage_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub Id: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Pages: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub Size: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub DeviceName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Retries: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub TransmissionStart: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f64) -> windows_core::HRESULT,
    pub TransmissionEnd: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f64) -> windows_core::HRESULT,
    pub CSID: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub TSID: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CallerId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub RoutingInformation: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CopyTiff: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Delete: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IFaxIncomingMessage_Impl: super::super::System::Com::IDispatch_Impl {
    fn Id(&self) -> windows_core::Result<windows_core::BSTR>;
    fn Pages(&self) -> windows_core::Result<i32>;
    fn Size(&self) -> windows_core::Result<i32>;
    fn DeviceName(&self) -> windows_core::Result<windows_core::BSTR>;
    fn Retries(&self) -> windows_core::Result<i32>;
    fn TransmissionStart(&self) -> windows_core::Result<f64>;
    fn TransmissionEnd(&self) -> windows_core::Result<f64>;
    fn CSID(&self) -> windows_core::Result<windows_core::BSTR>;
    fn TSID(&self) -> windows_core::Result<windows_core::BSTR>;
    fn CallerId(&self) -> windows_core::Result<windows_core::BSTR>;
    fn RoutingInformation(&self) -> windows_core::Result<windows_core::BSTR>;
    fn CopyTiff(&self, bstrtiffpath: &windows_core::BSTR) -> windows_core::Result<()>;
    fn Delete(&self) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IFaxIncomingMessage_Vtbl {
    pub const fn new<Identity: IFaxIncomingMessage_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Id<Identity: IFaxIncomingMessage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrid: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFaxIncomingMessage_Impl::Id(this) {
                    Ok(ok__) => {
                        pbstrid.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Pages<Identity: IFaxIncomingMessage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, plpages: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFaxIncomingMessage_Impl::Pages(this) {
                    Ok(ok__) => {
                        plpages.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Size<Identity: IFaxIncomingMessage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, plsize: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFaxIncomingMessage_Impl::Size(this) {
                    Ok(ok__) => {
                        plsize.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn DeviceName<Identity: IFaxIncomingMessage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrdevicename: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFaxIncomingMessage_Impl::DeviceName(this) {
                    Ok(ok__) => {
                        pbstrdevicename.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Retries<Identity: IFaxIncomingMessage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, plretries: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFaxIncomingMessage_Impl::Retries(this) {
                    Ok(ok__) => {
                        plretries.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn TransmissionStart<Identity: IFaxIncomingMessage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdatetransmissionstart: *mut f64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFaxIncomingMessage_Impl::TransmissionStart(this) {
                    Ok(ok__) => {
                        pdatetransmissionstart.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn TransmissionEnd<Identity: IFaxIncomingMessage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdatetransmissionend: *mut f64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFaxIncomingMessage_Impl::TransmissionEnd(this) {
                    Ok(ok__) => {
                        pdatetransmissionend.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CSID<Identity: IFaxIncomingMessage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrcsid: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFaxIncomingMessage_Impl::CSID(this) {
                    Ok(ok__) => {
                        pbstrcsid.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn TSID<Identity: IFaxIncomingMessage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrtsid: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFaxIncomingMessage_Impl::TSID(this) {
                    Ok(ok__) => {
                        pbstrtsid.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CallerId<Identity: IFaxIncomingMessage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrcallerid: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFaxIncomingMessage_Impl::CallerId(this) {
                    Ok(ok__) => {
                        pbstrcallerid.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn RoutingInformation<Identity: IFaxIncomingMessage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrroutinginformation: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFaxIncomingMessage_Impl::RoutingInformation(this) {
                    Ok(ok__) => {
                        pbstrroutinginformation.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CopyTiff<Identity: IFaxIncomingMessage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrtiffpath: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFaxIncomingMessage_Impl::CopyTiff(this, core::mem::transmute(&bstrtiffpath)).into()
            }
        }
        unsafe extern "system" fn Delete<Identity: IFaxIncomingMessage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFaxIncomingMessage_Impl::Delete(this).into()
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            Id: Id::<Identity, OFFSET>,
            Pages: Pages::<Identity, OFFSET>,
            Size: Size::<Identity, OFFSET>,
            DeviceName: DeviceName::<Identity, OFFSET>,
            Retries: Retries::<Identity, OFFSET>,
            TransmissionStart: TransmissionStart::<Identity, OFFSET>,
            TransmissionEnd: TransmissionEnd::<Identity, OFFSET>,
            CSID: CSID::<Identity, OFFSET>,
            TSID: TSID::<Identity, OFFSET>,
            CallerId: CallerId::<Identity, OFFSET>,
            RoutingInformation: RoutingInformation::<Identity, OFFSET>,
            CopyTiff: CopyTiff::<Identity, OFFSET>,
            Delete: Delete::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IFaxIncomingMessage as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for IFaxIncomingMessage {}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IFaxIncomingMessage2, IFaxIncomingMessage2_Vtbl, 0xf9208503_e2bc_48f3_9ec0_e6236f9b509a);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IFaxIncomingMessage2 {
    type Target = IFaxIncomingMessage;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IFaxIncomingMessage2, windows_core::IUnknown, super::super::System::Com::IDispatch, IFaxIncomingMessage);
#[cfg(feature = "Win32_System_Com")]
impl IFaxIncomingMessage2 {
    pub unsafe fn Subject(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Subject)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn SetSubject(&self, bstrsubject: &windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetSubject)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrsubject)).ok() }
    }
    pub unsafe fn SenderName(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).SenderName)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn SetSenderName(&self, bstrsendername: &windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetSenderName)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrsendername)).ok() }
    }
    pub unsafe fn SenderFaxNumber(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).SenderFaxNumber)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn SetSenderFaxNumber(&self, bstrsenderfaxnumber: &windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetSenderFaxNumber)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrsenderfaxnumber)).ok() }
    }
    pub unsafe fn HasCoverPage(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).HasCoverPage)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetHasCoverPage(&self, bhascoverpage: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetHasCoverPage)(windows_core::Interface::as_raw(self), bhascoverpage).ok() }
    }
    pub unsafe fn Recipients(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Recipients)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn SetRecipients(&self, bstrrecipients: &windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetRecipients)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrrecipients)).ok() }
    }
    pub unsafe fn WasReAssigned(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).WasReAssigned)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn Read(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Read)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetRead(&self, bread: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetRead)(windows_core::Interface::as_raw(self), bread).ok() }
    }
    pub unsafe fn ReAssign(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).ReAssign)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn Save(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Save)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn Refresh(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Refresh)(windows_core::Interface::as_raw(self)).ok() }
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IFaxIncomingMessage2_Vtbl {
    pub base__: IFaxIncomingMessage_Vtbl,
    pub Subject: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetSubject: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SenderName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetSenderName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SenderFaxNumber: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetSenderFaxNumber: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub HasCoverPage: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub SetHasCoverPage: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub Recipients: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetRecipients: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub WasReAssigned: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub Read: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub SetRead: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub ReAssign: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Save: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Refresh: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IFaxIncomingMessage2_Impl: IFaxIncomingMessage_Impl {
    fn Subject(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetSubject(&self, bstrsubject: &windows_core::BSTR) -> windows_core::Result<()>;
    fn SenderName(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetSenderName(&self, bstrsendername: &windows_core::BSTR) -> windows_core::Result<()>;
    fn SenderFaxNumber(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetSenderFaxNumber(&self, bstrsenderfaxnumber: &windows_core::BSTR) -> windows_core::Result<()>;
    fn HasCoverPage(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn SetHasCoverPage(&self, bhascoverpage: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
    fn Recipients(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetRecipients(&self, bstrrecipients: &windows_core::BSTR) -> windows_core::Result<()>;
    fn WasReAssigned(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn Read(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn SetRead(&self, bread: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
    fn ReAssign(&self) -> windows_core::Result<()>;
    fn Save(&self) -> windows_core::Result<()>;
    fn Refresh(&self) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IFaxIncomingMessage2_Vtbl {
    pub const fn new<Identity: IFaxIncomingMessage2_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Subject<Identity: IFaxIncomingMessage2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrsubject: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFaxIncomingMessage2_Impl::Subject(this) {
                    Ok(ok__) => {
                        pbstrsubject.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetSubject<Identity: IFaxIncomingMessage2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrsubject: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFaxIncomingMessage2_Impl::SetSubject(this, core::mem::transmute(&bstrsubject)).into()
            }
        }
        unsafe extern "system" fn SenderName<Identity: IFaxIncomingMessage2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrsendername: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFaxIncomingMessage2_Impl::SenderName(this) {
                    Ok(ok__) => {
                        pbstrsendername.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetSenderName<Identity: IFaxIncomingMessage2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrsendername: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFaxIncomingMessage2_Impl::SetSenderName(this, core::mem::transmute(&bstrsendername)).into()
            }
        }
        unsafe extern "system" fn SenderFaxNumber<Identity: IFaxIncomingMessage2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrsenderfaxnumber: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFaxIncomingMessage2_Impl::SenderFaxNumber(this) {
                    Ok(ok__) => {
                        pbstrsenderfaxnumber.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetSenderFaxNumber<Identity: IFaxIncomingMessage2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrsenderfaxnumber: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFaxIncomingMessage2_Impl::SetSenderFaxNumber(this, core::mem::transmute(&bstrsenderfaxnumber)).into()
            }
        }
        unsafe extern "system" fn HasCoverPage<Identity: IFaxIncomingMessage2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbhascoverpage: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFaxIncomingMessage2_Impl::HasCoverPage(this) {
                    Ok(ok__) => {
                        pbhascoverpage.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetHasCoverPage<Identity: IFaxIncomingMessage2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bhascoverpage: super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFaxIncomingMessage2_Impl::SetHasCoverPage(this, core::mem::transmute_copy(&bhascoverpage)).into()
            }
        }
        unsafe extern "system" fn Recipients<Identity: IFaxIncomingMessage2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrrecipients: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFaxIncomingMessage2_Impl::Recipients(this) {
                    Ok(ok__) => {
                        pbstrrecipients.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetRecipients<Identity: IFaxIncomingMessage2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrrecipients: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFaxIncomingMessage2_Impl::SetRecipients(this, core::mem::transmute(&bstrrecipients)).into()
            }
        }
        unsafe extern "system" fn WasReAssigned<Identity: IFaxIncomingMessage2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbwasreassigned: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFaxIncomingMessage2_Impl::WasReAssigned(this) {
                    Ok(ok__) => {
                        pbwasreassigned.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Read<Identity: IFaxIncomingMessage2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbread: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFaxIncomingMessage2_Impl::Read(this) {
                    Ok(ok__) => {
                        pbread.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetRead<Identity: IFaxIncomingMessage2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bread: super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFaxIncomingMessage2_Impl::SetRead(this, core::mem::transmute_copy(&bread)).into()
            }
        }
        unsafe extern "system" fn ReAssign<Identity: IFaxIncomingMessage2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFaxIncomingMessage2_Impl::ReAssign(this).into()
            }
        }
        unsafe extern "system" fn Save<Identity: IFaxIncomingMessage2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFaxIncomingMessage2_Impl::Save(this).into()
            }
        }
        unsafe extern "system" fn Refresh<Identity: IFaxIncomingMessage2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFaxIncomingMessage2_Impl::Refresh(this).into()
            }
        }
        Self {
            base__: IFaxIncomingMessage_Vtbl::new::<Identity, OFFSET>(),
            Subject: Subject::<Identity, OFFSET>,
            SetSubject: SetSubject::<Identity, OFFSET>,
            SenderName: SenderName::<Identity, OFFSET>,
            SetSenderName: SetSenderName::<Identity, OFFSET>,
            SenderFaxNumber: SenderFaxNumber::<Identity, OFFSET>,
            SetSenderFaxNumber: SetSenderFaxNumber::<Identity, OFFSET>,
            HasCoverPage: HasCoverPage::<Identity, OFFSET>,
            SetHasCoverPage: SetHasCoverPage::<Identity, OFFSET>,
            Recipients: Recipients::<Identity, OFFSET>,
            SetRecipients: SetRecipients::<Identity, OFFSET>,
            WasReAssigned: WasReAssigned::<Identity, OFFSET>,
            Read: Read::<Identity, OFFSET>,
            SetRead: SetRead::<Identity, OFFSET>,
            ReAssign: ReAssign::<Identity, OFFSET>,
            Save: Save::<Identity, OFFSET>,
            Refresh: Refresh::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IFaxIncomingMessage2 as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID || iid == &<IFaxIncomingMessage as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for IFaxIncomingMessage2 {}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IFaxIncomingMessageIterator, IFaxIncomingMessageIterator_Vtbl, 0xfd73ecc4_6f06_4f52_82a8_f7ba06ae3108);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IFaxIncomingMessageIterator {
    type Target = super::super::System::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IFaxIncomingMessageIterator, windows_core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl IFaxIncomingMessageIterator {
    pub unsafe fn Message(&self) -> windows_core::Result<IFaxIncomingMessage> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Message)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn PrefetchSize(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).PrefetchSize)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetPrefetchSize(&self, lprefetchsize: i32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetPrefetchSize)(windows_core::Interface::as_raw(self), lprefetchsize).ok() }
    }
    pub unsafe fn AtEOF(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).AtEOF)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn MoveFirst(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).MoveFirst)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn MoveNext(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).MoveNext)(windows_core::Interface::as_raw(self)).ok() }
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IFaxIncomingMessageIterator_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub Message: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub PrefetchSize: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetPrefetchSize: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub AtEOF: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub MoveFirst: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub MoveNext: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IFaxIncomingMessageIterator_Impl: super::super::System::Com::IDispatch_Impl {
    fn Message(&self) -> windows_core::Result<IFaxIncomingMessage>;
    fn PrefetchSize(&self) -> windows_core::Result<i32>;
    fn SetPrefetchSize(&self, lprefetchsize: i32) -> windows_core::Result<()>;
    fn AtEOF(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn MoveFirst(&self) -> windows_core::Result<()>;
    fn MoveNext(&self) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IFaxIncomingMessageIterator_Vtbl {
    pub const fn new<Identity: IFaxIncomingMessageIterator_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Message<Identity: IFaxIncomingMessageIterator_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pfaxincomingmessage: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFaxIncomingMessageIterator_Impl::Message(this) {
                    Ok(ok__) => {
                        pfaxincomingmessage.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn PrefetchSize<Identity: IFaxIncomingMessageIterator_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, plprefetchsize: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFaxIncomingMessageIterator_Impl::PrefetchSize(this) {
                    Ok(ok__) => {
                        plprefetchsize.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetPrefetchSize<Identity: IFaxIncomingMessageIterator_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lprefetchsize: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFaxIncomingMessageIterator_Impl::SetPrefetchSize(this, core::mem::transmute_copy(&lprefetchsize)).into()
            }
        }
        unsafe extern "system" fn AtEOF<Identity: IFaxIncomingMessageIterator_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbeof: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFaxIncomingMessageIterator_Impl::AtEOF(this) {
                    Ok(ok__) => {
                        pbeof.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn MoveFirst<Identity: IFaxIncomingMessageIterator_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFaxIncomingMessageIterator_Impl::MoveFirst(this).into()
            }
        }
        unsafe extern "system" fn MoveNext<Identity: IFaxIncomingMessageIterator_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFaxIncomingMessageIterator_Impl::MoveNext(this).into()
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            Message: Message::<Identity, OFFSET>,
            PrefetchSize: PrefetchSize::<Identity, OFFSET>,
            SetPrefetchSize: SetPrefetchSize::<Identity, OFFSET>,
            AtEOF: AtEOF::<Identity, OFFSET>,
            MoveFirst: MoveFirst::<Identity, OFFSET>,
            MoveNext: MoveNext::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IFaxIncomingMessageIterator as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for IFaxIncomingMessageIterator {}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IFaxIncomingQueue, IFaxIncomingQueue_Vtbl, 0x902e64ef_8fd8_4b75_9725_6014df161545);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IFaxIncomingQueue {
    type Target = super::super::System::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IFaxIncomingQueue, windows_core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl IFaxIncomingQueue {
    pub unsafe fn Blocked(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Blocked)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetBlocked(&self, bblocked: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetBlocked)(windows_core::Interface::as_raw(self), bblocked).ok() }
    }
    pub unsafe fn Refresh(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Refresh)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn Save(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Save)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn GetJobs(&self) -> windows_core::Result<IFaxIncomingJobs> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetJobs)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetJob(&self, bstrjobid: &windows_core::BSTR) -> windows_core::Result<IFaxIncomingJob> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetJob)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrjobid), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IFaxIncomingQueue_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub Blocked: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub SetBlocked: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub Refresh: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Save: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetJobs: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetJob: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IFaxIncomingQueue_Impl: super::super::System::Com::IDispatch_Impl {
    fn Blocked(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn SetBlocked(&self, bblocked: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
    fn Refresh(&self) -> windows_core::Result<()>;
    fn Save(&self) -> windows_core::Result<()>;
    fn GetJobs(&self) -> windows_core::Result<IFaxIncomingJobs>;
    fn GetJob(&self, bstrjobid: &windows_core::BSTR) -> windows_core::Result<IFaxIncomingJob>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IFaxIncomingQueue_Vtbl {
    pub const fn new<Identity: IFaxIncomingQueue_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Blocked<Identity: IFaxIncomingQueue_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbblocked: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFaxIncomingQueue_Impl::Blocked(this) {
                    Ok(ok__) => {
                        pbblocked.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetBlocked<Identity: IFaxIncomingQueue_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bblocked: super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFaxIncomingQueue_Impl::SetBlocked(this, core::mem::transmute_copy(&bblocked)).into()
            }
        }
        unsafe extern "system" fn Refresh<Identity: IFaxIncomingQueue_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFaxIncomingQueue_Impl::Refresh(this).into()
            }
        }
        unsafe extern "system" fn Save<Identity: IFaxIncomingQueue_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFaxIncomingQueue_Impl::Save(this).into()
            }
        }
        unsafe extern "system" fn GetJobs<Identity: IFaxIncomingQueue_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pfaxincomingjobs: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFaxIncomingQueue_Impl::GetJobs(this) {
                    Ok(ok__) => {
                        pfaxincomingjobs.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetJob<Identity: IFaxIncomingQueue_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrjobid: *mut core::ffi::c_void, pfaxincomingjob: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFaxIncomingQueue_Impl::GetJob(this, core::mem::transmute(&bstrjobid)) {
                    Ok(ok__) => {
                        pfaxincomingjob.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            Blocked: Blocked::<Identity, OFFSET>,
            SetBlocked: SetBlocked::<Identity, OFFSET>,
            Refresh: Refresh::<Identity, OFFSET>,
            Save: Save::<Identity, OFFSET>,
            GetJobs: GetJobs::<Identity, OFFSET>,
            GetJob: GetJob::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IFaxIncomingQueue as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for IFaxIncomingQueue {}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IFaxJobStatus, IFaxJobStatus_Vtbl, 0x8b86f485_fd7f_4824_886b_40c5caa617cc);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IFaxJobStatus {
    type Target = super::super::System::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IFaxJobStatus, windows_core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl IFaxJobStatus {
    pub unsafe fn Status(&self) -> windows_core::Result<FAX_JOB_STATUS_ENUM> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Status)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn Pages(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Pages)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn Size(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Size)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn CurrentPage(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CurrentPage)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn DeviceId(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).DeviceId)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn CSID(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CSID)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn TSID(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).TSID)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn ExtendedStatusCode(&self) -> windows_core::Result<FAX_JOB_EXTENDED_STATUS_ENUM> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ExtendedStatusCode)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn ExtendedStatus(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ExtendedStatus)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn AvailableOperations(&self) -> windows_core::Result<FAX_JOB_OPERATIONS_ENUM> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).AvailableOperations)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn Retries(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Retries)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn JobType(&self) -> windows_core::Result<FAX_JOB_TYPE_ENUM> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).JobType)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn ScheduledTime(&self) -> windows_core::Result<f64> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ScheduledTime)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn TransmissionStart(&self) -> windows_core::Result<f64> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).TransmissionStart)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn TransmissionEnd(&self) -> windows_core::Result<f64> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).TransmissionEnd)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn CallerId(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CallerId)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn RoutingInformation(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).RoutingInformation)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IFaxJobStatus_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub Status: unsafe extern "system" fn(*mut core::ffi::c_void, *mut FAX_JOB_STATUS_ENUM) -> windows_core::HRESULT,
    pub Pages: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub Size: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub CurrentPage: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub DeviceId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub CSID: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub TSID: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ExtendedStatusCode: unsafe extern "system" fn(*mut core::ffi::c_void, *mut FAX_JOB_EXTENDED_STATUS_ENUM) -> windows_core::HRESULT,
    pub ExtendedStatus: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub AvailableOperations: unsafe extern "system" fn(*mut core::ffi::c_void, *mut FAX_JOB_OPERATIONS_ENUM) -> windows_core::HRESULT,
    pub Retries: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub JobType: unsafe extern "system" fn(*mut core::ffi::c_void, *mut FAX_JOB_TYPE_ENUM) -> windows_core::HRESULT,
    pub ScheduledTime: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f64) -> windows_core::HRESULT,
    pub TransmissionStart: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f64) -> windows_core::HRESULT,
    pub TransmissionEnd: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f64) -> windows_core::HRESULT,
    pub CallerId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub RoutingInformation: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IFaxJobStatus_Impl: super::super::System::Com::IDispatch_Impl {
    fn Status(&self) -> windows_core::Result<FAX_JOB_STATUS_ENUM>;
    fn Pages(&self) -> windows_core::Result<i32>;
    fn Size(&self) -> windows_core::Result<i32>;
    fn CurrentPage(&self) -> windows_core::Result<i32>;
    fn DeviceId(&self) -> windows_core::Result<i32>;
    fn CSID(&self) -> windows_core::Result<windows_core::BSTR>;
    fn TSID(&self) -> windows_core::Result<windows_core::BSTR>;
    fn ExtendedStatusCode(&self) -> windows_core::Result<FAX_JOB_EXTENDED_STATUS_ENUM>;
    fn ExtendedStatus(&self) -> windows_core::Result<windows_core::BSTR>;
    fn AvailableOperations(&self) -> windows_core::Result<FAX_JOB_OPERATIONS_ENUM>;
    fn Retries(&self) -> windows_core::Result<i32>;
    fn JobType(&self) -> windows_core::Result<FAX_JOB_TYPE_ENUM>;
    fn ScheduledTime(&self) -> windows_core::Result<f64>;
    fn TransmissionStart(&self) -> windows_core::Result<f64>;
    fn TransmissionEnd(&self) -> windows_core::Result<f64>;
    fn CallerId(&self) -> windows_core::Result<windows_core::BSTR>;
    fn RoutingInformation(&self) -> windows_core::Result<windows_core::BSTR>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IFaxJobStatus_Vtbl {
    pub const fn new<Identity: IFaxJobStatus_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Status<Identity: IFaxJobStatus_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pstatus: *mut FAX_JOB_STATUS_ENUM) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFaxJobStatus_Impl::Status(this) {
                    Ok(ok__) => {
                        pstatus.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Pages<Identity: IFaxJobStatus_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, plpages: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFaxJobStatus_Impl::Pages(this) {
                    Ok(ok__) => {
                        plpages.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Size<Identity: IFaxJobStatus_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, plsize: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFaxJobStatus_Impl::Size(this) {
                    Ok(ok__) => {
                        plsize.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CurrentPage<Identity: IFaxJobStatus_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, plcurrentpage: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFaxJobStatus_Impl::CurrentPage(this) {
                    Ok(ok__) => {
                        plcurrentpage.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn DeviceId<Identity: IFaxJobStatus_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pldeviceid: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFaxJobStatus_Impl::DeviceId(this) {
                    Ok(ok__) => {
                        pldeviceid.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CSID<Identity: IFaxJobStatus_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrcsid: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFaxJobStatus_Impl::CSID(this) {
                    Ok(ok__) => {
                        pbstrcsid.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn TSID<Identity: IFaxJobStatus_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrtsid: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFaxJobStatus_Impl::TSID(this) {
                    Ok(ok__) => {
                        pbstrtsid.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn ExtendedStatusCode<Identity: IFaxJobStatus_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pextendedstatuscode: *mut FAX_JOB_EXTENDED_STATUS_ENUM) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFaxJobStatus_Impl::ExtendedStatusCode(this) {
                    Ok(ok__) => {
                        pextendedstatuscode.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn ExtendedStatus<Identity: IFaxJobStatus_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrextendedstatus: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFaxJobStatus_Impl::ExtendedStatus(this) {
                    Ok(ok__) => {
                        pbstrextendedstatus.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn AvailableOperations<Identity: IFaxJobStatus_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pavailableoperations: *mut FAX_JOB_OPERATIONS_ENUM) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFaxJobStatus_Impl::AvailableOperations(this) {
                    Ok(ok__) => {
                        pavailableoperations.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Retries<Identity: IFaxJobStatus_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, plretries: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFaxJobStatus_Impl::Retries(this) {
                    Ok(ok__) => {
                        plretries.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn JobType<Identity: IFaxJobStatus_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pjobtype: *mut FAX_JOB_TYPE_ENUM) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFaxJobStatus_Impl::JobType(this) {
                    Ok(ok__) => {
                        pjobtype.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn ScheduledTime<Identity: IFaxJobStatus_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdatescheduledtime: *mut f64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFaxJobStatus_Impl::ScheduledTime(this) {
                    Ok(ok__) => {
                        pdatescheduledtime.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn TransmissionStart<Identity: IFaxJobStatus_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdatetransmissionstart: *mut f64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFaxJobStatus_Impl::TransmissionStart(this) {
                    Ok(ok__) => {
                        pdatetransmissionstart.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn TransmissionEnd<Identity: IFaxJobStatus_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdatetransmissionend: *mut f64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFaxJobStatus_Impl::TransmissionEnd(this) {
                    Ok(ok__) => {
                        pdatetransmissionend.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CallerId<Identity: IFaxJobStatus_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrcallerid: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFaxJobStatus_Impl::CallerId(this) {
                    Ok(ok__) => {
                        pbstrcallerid.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn RoutingInformation<Identity: IFaxJobStatus_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrroutinginformation: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFaxJobStatus_Impl::RoutingInformation(this) {
                    Ok(ok__) => {
                        pbstrroutinginformation.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            Status: Status::<Identity, OFFSET>,
            Pages: Pages::<Identity, OFFSET>,
            Size: Size::<Identity, OFFSET>,
            CurrentPage: CurrentPage::<Identity, OFFSET>,
            DeviceId: DeviceId::<Identity, OFFSET>,
            CSID: CSID::<Identity, OFFSET>,
            TSID: TSID::<Identity, OFFSET>,
            ExtendedStatusCode: ExtendedStatusCode::<Identity, OFFSET>,
            ExtendedStatus: ExtendedStatus::<Identity, OFFSET>,
            AvailableOperations: AvailableOperations::<Identity, OFFSET>,
            Retries: Retries::<Identity, OFFSET>,
            JobType: JobType::<Identity, OFFSET>,
            ScheduledTime: ScheduledTime::<Identity, OFFSET>,
            TransmissionStart: TransmissionStart::<Identity, OFFSET>,
            TransmissionEnd: TransmissionEnd::<Identity, OFFSET>,
            CallerId: CallerId::<Identity, OFFSET>,
            RoutingInformation: RoutingInformation::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IFaxJobStatus as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for IFaxJobStatus {}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IFaxLoggingOptions, IFaxLoggingOptions_Vtbl, 0x34e64fb9_6b31_4d32_8b27_d286c0c33606);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IFaxLoggingOptions {
    type Target = super::super::System::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IFaxLoggingOptions, windows_core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl IFaxLoggingOptions {
    pub unsafe fn EventLogging(&self) -> windows_core::Result<IFaxEventLogging> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).EventLogging)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn ActivityLogging(&self) -> windows_core::Result<IFaxActivityLogging> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ActivityLogging)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IFaxLoggingOptions_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub EventLogging: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ActivityLogging: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IFaxLoggingOptions_Impl: super::super::System::Com::IDispatch_Impl {
    fn EventLogging(&self) -> windows_core::Result<IFaxEventLogging>;
    fn ActivityLogging(&self) -> windows_core::Result<IFaxActivityLogging>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IFaxLoggingOptions_Vtbl {
    pub const fn new<Identity: IFaxLoggingOptions_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn EventLogging<Identity: IFaxLoggingOptions_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pfaxeventlogging: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFaxLoggingOptions_Impl::EventLogging(this) {
                    Ok(ok__) => {
                        pfaxeventlogging.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn ActivityLogging<Identity: IFaxLoggingOptions_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pfaxactivitylogging: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFaxLoggingOptions_Impl::ActivityLogging(this) {
                    Ok(ok__) => {
                        pfaxactivitylogging.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            EventLogging: EventLogging::<Identity, OFFSET>,
            ActivityLogging: ActivityLogging::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IFaxLoggingOptions as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for IFaxLoggingOptions {}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IFaxOutboundRouting, IFaxOutboundRouting_Vtbl, 0x25dc05a4_9909_41bd_a95b_7e5d1dec1d43);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IFaxOutboundRouting {
    type Target = super::super::System::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IFaxOutboundRouting, windows_core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl IFaxOutboundRouting {
    pub unsafe fn GetGroups(&self) -> windows_core::Result<IFaxOutboundRoutingGroups> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetGroups)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetRules(&self) -> windows_core::Result<IFaxOutboundRoutingRules> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetRules)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IFaxOutboundRouting_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub GetGroups: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetRules: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IFaxOutboundRouting_Impl: super::super::System::Com::IDispatch_Impl {
    fn GetGroups(&self) -> windows_core::Result<IFaxOutboundRoutingGroups>;
    fn GetRules(&self) -> windows_core::Result<IFaxOutboundRoutingRules>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IFaxOutboundRouting_Vtbl {
    pub const fn new<Identity: IFaxOutboundRouting_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetGroups<Identity: IFaxOutboundRouting_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pfaxoutboundroutinggroups: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFaxOutboundRouting_Impl::GetGroups(this) {
                    Ok(ok__) => {
                        pfaxoutboundroutinggroups.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetRules<Identity: IFaxOutboundRouting_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pfaxoutboundroutingrules: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFaxOutboundRouting_Impl::GetRules(this) {
                    Ok(ok__) => {
                        pfaxoutboundroutingrules.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            GetGroups: GetGroups::<Identity, OFFSET>,
            GetRules: GetRules::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IFaxOutboundRouting as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for IFaxOutboundRouting {}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IFaxOutboundRoutingGroup, IFaxOutboundRoutingGroup_Vtbl, 0xca6289a1_7e25_4f87_9a0b_93365734962c);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IFaxOutboundRoutingGroup {
    type Target = super::super::System::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IFaxOutboundRoutingGroup, windows_core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl IFaxOutboundRoutingGroup {
    pub unsafe fn Name(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Name)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn Status(&self) -> windows_core::Result<FAX_GROUP_STATUS_ENUM> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Status)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn DeviceIds(&self) -> windows_core::Result<IFaxDeviceIds> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).DeviceIds)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IFaxOutboundRoutingGroup_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub Name: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Status: unsafe extern "system" fn(*mut core::ffi::c_void, *mut FAX_GROUP_STATUS_ENUM) -> windows_core::HRESULT,
    pub DeviceIds: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IFaxOutboundRoutingGroup_Impl: super::super::System::Com::IDispatch_Impl {
    fn Name(&self) -> windows_core::Result<windows_core::BSTR>;
    fn Status(&self) -> windows_core::Result<FAX_GROUP_STATUS_ENUM>;
    fn DeviceIds(&self) -> windows_core::Result<IFaxDeviceIds>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IFaxOutboundRoutingGroup_Vtbl {
    pub const fn new<Identity: IFaxOutboundRoutingGroup_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Name<Identity: IFaxOutboundRoutingGroup_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrname: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFaxOutboundRoutingGroup_Impl::Name(this) {
                    Ok(ok__) => {
                        pbstrname.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Status<Identity: IFaxOutboundRoutingGroup_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pstatus: *mut FAX_GROUP_STATUS_ENUM) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFaxOutboundRoutingGroup_Impl::Status(this) {
                    Ok(ok__) => {
                        pstatus.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn DeviceIds<Identity: IFaxOutboundRoutingGroup_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pfaxdeviceids: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFaxOutboundRoutingGroup_Impl::DeviceIds(this) {
                    Ok(ok__) => {
                        pfaxdeviceids.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            Name: Name::<Identity, OFFSET>,
            Status: Status::<Identity, OFFSET>,
            DeviceIds: DeviceIds::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IFaxOutboundRoutingGroup as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for IFaxOutboundRoutingGroup {}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IFaxOutboundRoutingGroups, IFaxOutboundRoutingGroups_Vtbl, 0x235cbef7_c2de_4bfd_b8da_75097c82c87f);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IFaxOutboundRoutingGroups {
    type Target = super::super::System::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IFaxOutboundRoutingGroups, windows_core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl IFaxOutboundRoutingGroups {
    pub unsafe fn _NewEnum(&self) -> windows_core::Result<windows_core::IUnknown> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self)._NewEnum)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn get_Item(&self, vindex: &super::super::System::Variant::VARIANT) -> windows_core::Result<IFaxOutboundRoutingGroup> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_Item)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(vindex), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn Count(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Count)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn Add(&self, bstrname: &windows_core::BSTR) -> windows_core::Result<IFaxOutboundRoutingGroup> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Add)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrname), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn Remove(&self, vindex: &super::super::System::Variant::VARIANT) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Remove)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(vindex)).ok() }
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IFaxOutboundRoutingGroups_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub _NewEnum: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub get_Item: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::System::Variant::VARIANT, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    get_Item: usize,
    pub Count: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub Add: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub Remove: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::System::Variant::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    Remove: usize,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IFaxOutboundRoutingGroups_Impl: super::super::System::Com::IDispatch_Impl {
    fn _NewEnum(&self) -> windows_core::Result<windows_core::IUnknown>;
    fn get_Item(&self, vindex: &super::super::System::Variant::VARIANT) -> windows_core::Result<IFaxOutboundRoutingGroup>;
    fn Count(&self) -> windows_core::Result<i32>;
    fn Add(&self, bstrname: &windows_core::BSTR) -> windows_core::Result<IFaxOutboundRoutingGroup>;
    fn Remove(&self, vindex: &super::super::System::Variant::VARIANT) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IFaxOutboundRoutingGroups_Vtbl {
    pub const fn new<Identity: IFaxOutboundRoutingGroups_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn _NewEnum<Identity: IFaxOutboundRoutingGroups_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppunk: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFaxOutboundRoutingGroups_Impl::_NewEnum(this) {
                    Ok(ok__) => {
                        ppunk.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_Item<Identity: IFaxOutboundRoutingGroups_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, vindex: super::super::System::Variant::VARIANT, pfaxoutboundroutinggroup: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFaxOutboundRoutingGroups_Impl::get_Item(this, core::mem::transmute(&vindex)) {
                    Ok(ok__) => {
                        pfaxoutboundroutinggroup.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Count<Identity: IFaxOutboundRoutingGroups_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, plcount: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFaxOutboundRoutingGroups_Impl::Count(this) {
                    Ok(ok__) => {
                        plcount.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Add<Identity: IFaxOutboundRoutingGroups_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrname: *mut core::ffi::c_void, pfaxoutboundroutinggroup: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFaxOutboundRoutingGroups_Impl::Add(this, core::mem::transmute(&bstrname)) {
                    Ok(ok__) => {
                        pfaxoutboundroutinggroup.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Remove<Identity: IFaxOutboundRoutingGroups_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, vindex: super::super::System::Variant::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFaxOutboundRoutingGroups_Impl::Remove(this, core::mem::transmute(&vindex)).into()
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            _NewEnum: _NewEnum::<Identity, OFFSET>,
            get_Item: get_Item::<Identity, OFFSET>,
            Count: Count::<Identity, OFFSET>,
            Add: Add::<Identity, OFFSET>,
            Remove: Remove::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IFaxOutboundRoutingGroups as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for IFaxOutboundRoutingGroups {}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IFaxOutboundRoutingRule, IFaxOutboundRoutingRule_Vtbl, 0xe1f795d5_07c2_469f_b027_acacc23219da);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IFaxOutboundRoutingRule {
    type Target = super::super::System::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IFaxOutboundRoutingRule, windows_core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl IFaxOutboundRoutingRule {
    pub unsafe fn CountryCode(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CountryCode)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn AreaCode(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).AreaCode)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn Status(&self) -> windows_core::Result<FAX_RULE_STATUS_ENUM> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Status)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn UseDevice(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).UseDevice)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetUseDevice(&self, busedevice: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetUseDevice)(windows_core::Interface::as_raw(self), busedevice).ok() }
    }
    pub unsafe fn DeviceId(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).DeviceId)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetDeviceId(&self, deviceid: i32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetDeviceId)(windows_core::Interface::as_raw(self), deviceid).ok() }
    }
    pub unsafe fn GroupName(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GroupName)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn SetGroupName(&self, bstrgroupname: &windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetGroupName)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrgroupname)).ok() }
    }
    pub unsafe fn Refresh(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Refresh)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn Save(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Save)(windows_core::Interface::as_raw(self)).ok() }
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IFaxOutboundRoutingRule_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub CountryCode: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub AreaCode: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub Status: unsafe extern "system" fn(*mut core::ffi::c_void, *mut FAX_RULE_STATUS_ENUM) -> windows_core::HRESULT,
    pub UseDevice: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub SetUseDevice: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub DeviceId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetDeviceId: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub GroupName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetGroupName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Refresh: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Save: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IFaxOutboundRoutingRule_Impl: super::super::System::Com::IDispatch_Impl {
    fn CountryCode(&self) -> windows_core::Result<i32>;
    fn AreaCode(&self) -> windows_core::Result<i32>;
    fn Status(&self) -> windows_core::Result<FAX_RULE_STATUS_ENUM>;
    fn UseDevice(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn SetUseDevice(&self, busedevice: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
    fn DeviceId(&self) -> windows_core::Result<i32>;
    fn SetDeviceId(&self, deviceid: i32) -> windows_core::Result<()>;
    fn GroupName(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetGroupName(&self, bstrgroupname: &windows_core::BSTR) -> windows_core::Result<()>;
    fn Refresh(&self) -> windows_core::Result<()>;
    fn Save(&self) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IFaxOutboundRoutingRule_Vtbl {
    pub const fn new<Identity: IFaxOutboundRoutingRule_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn CountryCode<Identity: IFaxOutboundRoutingRule_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, plcountrycode: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFaxOutboundRoutingRule_Impl::CountryCode(this) {
                    Ok(ok__) => {
                        plcountrycode.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn AreaCode<Identity: IFaxOutboundRoutingRule_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, plareacode: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFaxOutboundRoutingRule_Impl::AreaCode(this) {
                    Ok(ok__) => {
                        plareacode.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Status<Identity: IFaxOutboundRoutingRule_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pstatus: *mut FAX_RULE_STATUS_ENUM) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFaxOutboundRoutingRule_Impl::Status(this) {
                    Ok(ok__) => {
                        pstatus.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn UseDevice<Identity: IFaxOutboundRoutingRule_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbusedevice: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFaxOutboundRoutingRule_Impl::UseDevice(this) {
                    Ok(ok__) => {
                        pbusedevice.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetUseDevice<Identity: IFaxOutboundRoutingRule_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, busedevice: super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFaxOutboundRoutingRule_Impl::SetUseDevice(this, core::mem::transmute_copy(&busedevice)).into()
            }
        }
        unsafe extern "system" fn DeviceId<Identity: IFaxOutboundRoutingRule_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pldeviceid: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFaxOutboundRoutingRule_Impl::DeviceId(this) {
                    Ok(ok__) => {
                        pldeviceid.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetDeviceId<Identity: IFaxOutboundRoutingRule_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, deviceid: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFaxOutboundRoutingRule_Impl::SetDeviceId(this, core::mem::transmute_copy(&deviceid)).into()
            }
        }
        unsafe extern "system" fn GroupName<Identity: IFaxOutboundRoutingRule_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrgroupname: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFaxOutboundRoutingRule_Impl::GroupName(this) {
                    Ok(ok__) => {
                        pbstrgroupname.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetGroupName<Identity: IFaxOutboundRoutingRule_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrgroupname: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFaxOutboundRoutingRule_Impl::SetGroupName(this, core::mem::transmute(&bstrgroupname)).into()
            }
        }
        unsafe extern "system" fn Refresh<Identity: IFaxOutboundRoutingRule_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFaxOutboundRoutingRule_Impl::Refresh(this).into()
            }
        }
        unsafe extern "system" fn Save<Identity: IFaxOutboundRoutingRule_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFaxOutboundRoutingRule_Impl::Save(this).into()
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            CountryCode: CountryCode::<Identity, OFFSET>,
            AreaCode: AreaCode::<Identity, OFFSET>,
            Status: Status::<Identity, OFFSET>,
            UseDevice: UseDevice::<Identity, OFFSET>,
            SetUseDevice: SetUseDevice::<Identity, OFFSET>,
            DeviceId: DeviceId::<Identity, OFFSET>,
            SetDeviceId: SetDeviceId::<Identity, OFFSET>,
            GroupName: GroupName::<Identity, OFFSET>,
            SetGroupName: SetGroupName::<Identity, OFFSET>,
            Refresh: Refresh::<Identity, OFFSET>,
            Save: Save::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IFaxOutboundRoutingRule as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for IFaxOutboundRoutingRule {}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IFaxOutboundRoutingRules, IFaxOutboundRoutingRules_Vtbl, 0xdcefa1e7_ae7d_4ed6_8521_369edcca5120);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IFaxOutboundRoutingRules {
    type Target = super::super::System::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IFaxOutboundRoutingRules, windows_core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl IFaxOutboundRoutingRules {
    pub unsafe fn _NewEnum(&self) -> windows_core::Result<windows_core::IUnknown> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self)._NewEnum)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn get_Item(&self, lindex: i32) -> windows_core::Result<IFaxOutboundRoutingRule> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_Item)(windows_core::Interface::as_raw(self), lindex, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn Count(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Count)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn ItemByCountryAndArea(&self, lcountrycode: i32, lareacode: i32) -> windows_core::Result<IFaxOutboundRoutingRule> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ItemByCountryAndArea)(windows_core::Interface::as_raw(self), lcountrycode, lareacode, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn RemoveByCountryAndArea(&self, lcountrycode: i32, lareacode: i32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).RemoveByCountryAndArea)(windows_core::Interface::as_raw(self), lcountrycode, lareacode).ok() }
    }
    pub unsafe fn Remove(&self, lindex: i32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Remove)(windows_core::Interface::as_raw(self), lindex).ok() }
    }
    pub unsafe fn Add(&self, lcountrycode: i32, lareacode: i32, busedevice: super::super::Foundation::VARIANT_BOOL, bstrgroupname: &windows_core::BSTR, ldeviceid: i32) -> windows_core::Result<IFaxOutboundRoutingRule> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Add)(windows_core::Interface::as_raw(self), lcountrycode, lareacode, busedevice, core::mem::transmute_copy(bstrgroupname), ldeviceid, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IFaxOutboundRoutingRules_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub _NewEnum: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub get_Item: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Count: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub ItemByCountryAndArea: unsafe extern "system" fn(*mut core::ffi::c_void, i32, i32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub RemoveByCountryAndArea: unsafe extern "system" fn(*mut core::ffi::c_void, i32, i32) -> windows_core::HRESULT,
    pub Remove: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub Add: unsafe extern "system" fn(*mut core::ffi::c_void, i32, i32, super::super::Foundation::VARIANT_BOOL, *mut core::ffi::c_void, i32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IFaxOutboundRoutingRules_Impl: super::super::System::Com::IDispatch_Impl {
    fn _NewEnum(&self) -> windows_core::Result<windows_core::IUnknown>;
    fn get_Item(&self, lindex: i32) -> windows_core::Result<IFaxOutboundRoutingRule>;
    fn Count(&self) -> windows_core::Result<i32>;
    fn ItemByCountryAndArea(&self, lcountrycode: i32, lareacode: i32) -> windows_core::Result<IFaxOutboundRoutingRule>;
    fn RemoveByCountryAndArea(&self, lcountrycode: i32, lareacode: i32) -> windows_core::Result<()>;
    fn Remove(&self, lindex: i32) -> windows_core::Result<()>;
    fn Add(&self, lcountrycode: i32, lareacode: i32, busedevice: super::super::Foundation::VARIANT_BOOL, bstrgroupname: &windows_core::BSTR, ldeviceid: i32) -> windows_core::Result<IFaxOutboundRoutingRule>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IFaxOutboundRoutingRules_Vtbl {
    pub const fn new<Identity: IFaxOutboundRoutingRules_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn _NewEnum<Identity: IFaxOutboundRoutingRules_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppunk: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFaxOutboundRoutingRules_Impl::_NewEnum(this) {
                    Ok(ok__) => {
                        ppunk.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_Item<Identity: IFaxOutboundRoutingRules_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lindex: i32, pfaxoutboundroutingrule: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFaxOutboundRoutingRules_Impl::get_Item(this, core::mem::transmute_copy(&lindex)) {
                    Ok(ok__) => {
                        pfaxoutboundroutingrule.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Count<Identity: IFaxOutboundRoutingRules_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, plcount: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFaxOutboundRoutingRules_Impl::Count(this) {
                    Ok(ok__) => {
                        plcount.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn ItemByCountryAndArea<Identity: IFaxOutboundRoutingRules_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lcountrycode: i32, lareacode: i32, pfaxoutboundroutingrule: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFaxOutboundRoutingRules_Impl::ItemByCountryAndArea(this, core::mem::transmute_copy(&lcountrycode), core::mem::transmute_copy(&lareacode)) {
                    Ok(ok__) => {
                        pfaxoutboundroutingrule.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn RemoveByCountryAndArea<Identity: IFaxOutboundRoutingRules_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lcountrycode: i32, lareacode: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFaxOutboundRoutingRules_Impl::RemoveByCountryAndArea(this, core::mem::transmute_copy(&lcountrycode), core::mem::transmute_copy(&lareacode)).into()
            }
        }
        unsafe extern "system" fn Remove<Identity: IFaxOutboundRoutingRules_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lindex: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFaxOutboundRoutingRules_Impl::Remove(this, core::mem::transmute_copy(&lindex)).into()
            }
        }
        unsafe extern "system" fn Add<Identity: IFaxOutboundRoutingRules_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lcountrycode: i32, lareacode: i32, busedevice: super::super::Foundation::VARIANT_BOOL, bstrgroupname: *mut core::ffi::c_void, ldeviceid: i32, pfaxoutboundroutingrule: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFaxOutboundRoutingRules_Impl::Add(this, core::mem::transmute_copy(&lcountrycode), core::mem::transmute_copy(&lareacode), core::mem::transmute_copy(&busedevice), core::mem::transmute(&bstrgroupname), core::mem::transmute_copy(&ldeviceid)) {
                    Ok(ok__) => {
                        pfaxoutboundroutingrule.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            _NewEnum: _NewEnum::<Identity, OFFSET>,
            get_Item: get_Item::<Identity, OFFSET>,
            Count: Count::<Identity, OFFSET>,
            ItemByCountryAndArea: ItemByCountryAndArea::<Identity, OFFSET>,
            RemoveByCountryAndArea: RemoveByCountryAndArea::<Identity, OFFSET>,
            Remove: Remove::<Identity, OFFSET>,
            Add: Add::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IFaxOutboundRoutingRules as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for IFaxOutboundRoutingRules {}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IFaxOutgoingArchive, IFaxOutgoingArchive_Vtbl, 0xc9c28f40_8d80_4e53_810f_9a79919b49fd);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IFaxOutgoingArchive {
    type Target = super::super::System::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IFaxOutgoingArchive, windows_core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl IFaxOutgoingArchive {
    pub unsafe fn UseArchive(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).UseArchive)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetUseArchive(&self, busearchive: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetUseArchive)(windows_core::Interface::as_raw(self), busearchive).ok() }
    }
    pub unsafe fn ArchiveFolder(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ArchiveFolder)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn SetArchiveFolder(&self, bstrarchivefolder: &windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetArchiveFolder)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrarchivefolder)).ok() }
    }
    pub unsafe fn SizeQuotaWarning(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).SizeQuotaWarning)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetSizeQuotaWarning(&self, bsizequotawarning: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetSizeQuotaWarning)(windows_core::Interface::as_raw(self), bsizequotawarning).ok() }
    }
    pub unsafe fn HighQuotaWaterMark(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).HighQuotaWaterMark)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetHighQuotaWaterMark(&self, lhighquotawatermark: i32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetHighQuotaWaterMark)(windows_core::Interface::as_raw(self), lhighquotawatermark).ok() }
    }
    pub unsafe fn LowQuotaWaterMark(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).LowQuotaWaterMark)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetLowQuotaWaterMark(&self, llowquotawatermark: i32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetLowQuotaWaterMark)(windows_core::Interface::as_raw(self), llowquotawatermark).ok() }
    }
    pub unsafe fn AgeLimit(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).AgeLimit)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetAgeLimit(&self, lagelimit: i32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetAgeLimit)(windows_core::Interface::as_raw(self), lagelimit).ok() }
    }
    pub unsafe fn SizeLow(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).SizeLow)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SizeHigh(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).SizeHigh)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn Refresh(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Refresh)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn Save(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Save)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn GetMessages(&self, lprefetchsize: i32) -> windows_core::Result<IFaxOutgoingMessageIterator> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetMessages)(windows_core::Interface::as_raw(self), lprefetchsize, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetMessage(&self, bstrmessageid: &windows_core::BSTR) -> windows_core::Result<IFaxOutgoingMessage> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetMessage)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrmessageid), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IFaxOutgoingArchive_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub UseArchive: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub SetUseArchive: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub ArchiveFolder: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetArchiveFolder: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SizeQuotaWarning: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub SetSizeQuotaWarning: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub HighQuotaWaterMark: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetHighQuotaWaterMark: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub LowQuotaWaterMark: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetLowQuotaWaterMark: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub AgeLimit: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetAgeLimit: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub SizeLow: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SizeHigh: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub Refresh: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Save: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetMessages: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetMessage: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IFaxOutgoingArchive_Impl: super::super::System::Com::IDispatch_Impl {
    fn UseArchive(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn SetUseArchive(&self, busearchive: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
    fn ArchiveFolder(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetArchiveFolder(&self, bstrarchivefolder: &windows_core::BSTR) -> windows_core::Result<()>;
    fn SizeQuotaWarning(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn SetSizeQuotaWarning(&self, bsizequotawarning: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
    fn HighQuotaWaterMark(&self) -> windows_core::Result<i32>;
    fn SetHighQuotaWaterMark(&self, lhighquotawatermark: i32) -> windows_core::Result<()>;
    fn LowQuotaWaterMark(&self) -> windows_core::Result<i32>;
    fn SetLowQuotaWaterMark(&self, llowquotawatermark: i32) -> windows_core::Result<()>;
    fn AgeLimit(&self) -> windows_core::Result<i32>;
    fn SetAgeLimit(&self, lagelimit: i32) -> windows_core::Result<()>;
    fn SizeLow(&self) -> windows_core::Result<i32>;
    fn SizeHigh(&self) -> windows_core::Result<i32>;
    fn Refresh(&self) -> windows_core::Result<()>;
    fn Save(&self) -> windows_core::Result<()>;
    fn GetMessages(&self, lprefetchsize: i32) -> windows_core::Result<IFaxOutgoingMessageIterator>;
    fn GetMessage(&self, bstrmessageid: &windows_core::BSTR) -> windows_core::Result<IFaxOutgoingMessage>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IFaxOutgoingArchive_Vtbl {
    pub const fn new<Identity: IFaxOutgoingArchive_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn UseArchive<Identity: IFaxOutgoingArchive_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbusearchive: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFaxOutgoingArchive_Impl::UseArchive(this) {
                    Ok(ok__) => {
                        pbusearchive.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetUseArchive<Identity: IFaxOutgoingArchive_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, busearchive: super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFaxOutgoingArchive_Impl::SetUseArchive(this, core::mem::transmute_copy(&busearchive)).into()
            }
        }
        unsafe extern "system" fn ArchiveFolder<Identity: IFaxOutgoingArchive_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrarchivefolder: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFaxOutgoingArchive_Impl::ArchiveFolder(this) {
                    Ok(ok__) => {
                        pbstrarchivefolder.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetArchiveFolder<Identity: IFaxOutgoingArchive_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrarchivefolder: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFaxOutgoingArchive_Impl::SetArchiveFolder(this, core::mem::transmute(&bstrarchivefolder)).into()
            }
        }
        unsafe extern "system" fn SizeQuotaWarning<Identity: IFaxOutgoingArchive_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbsizequotawarning: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFaxOutgoingArchive_Impl::SizeQuotaWarning(this) {
                    Ok(ok__) => {
                        pbsizequotawarning.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetSizeQuotaWarning<Identity: IFaxOutgoingArchive_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bsizequotawarning: super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFaxOutgoingArchive_Impl::SetSizeQuotaWarning(this, core::mem::transmute_copy(&bsizequotawarning)).into()
            }
        }
        unsafe extern "system" fn HighQuotaWaterMark<Identity: IFaxOutgoingArchive_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, plhighquotawatermark: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFaxOutgoingArchive_Impl::HighQuotaWaterMark(this) {
                    Ok(ok__) => {
                        plhighquotawatermark.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetHighQuotaWaterMark<Identity: IFaxOutgoingArchive_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lhighquotawatermark: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFaxOutgoingArchive_Impl::SetHighQuotaWaterMark(this, core::mem::transmute_copy(&lhighquotawatermark)).into()
            }
        }
        unsafe extern "system" fn LowQuotaWaterMark<Identity: IFaxOutgoingArchive_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pllowquotawatermark: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFaxOutgoingArchive_Impl::LowQuotaWaterMark(this) {
                    Ok(ok__) => {
                        pllowquotawatermark.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetLowQuotaWaterMark<Identity: IFaxOutgoingArchive_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, llowquotawatermark: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFaxOutgoingArchive_Impl::SetLowQuotaWaterMark(this, core::mem::transmute_copy(&llowquotawatermark)).into()
            }
        }
        unsafe extern "system" fn AgeLimit<Identity: IFaxOutgoingArchive_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, plagelimit: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFaxOutgoingArchive_Impl::AgeLimit(this) {
                    Ok(ok__) => {
                        plagelimit.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetAgeLimit<Identity: IFaxOutgoingArchive_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lagelimit: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFaxOutgoingArchive_Impl::SetAgeLimit(this, core::mem::transmute_copy(&lagelimit)).into()
            }
        }
        unsafe extern "system" fn SizeLow<Identity: IFaxOutgoingArchive_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, plsizelow: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFaxOutgoingArchive_Impl::SizeLow(this) {
                    Ok(ok__) => {
                        plsizelow.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SizeHigh<Identity: IFaxOutgoingArchive_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, plsizehigh: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFaxOutgoingArchive_Impl::SizeHigh(this) {
                    Ok(ok__) => {
                        plsizehigh.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Refresh<Identity: IFaxOutgoingArchive_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFaxOutgoingArchive_Impl::Refresh(this).into()
            }
        }
        unsafe extern "system" fn Save<Identity: IFaxOutgoingArchive_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFaxOutgoingArchive_Impl::Save(this).into()
            }
        }
        unsafe extern "system" fn GetMessages<Identity: IFaxOutgoingArchive_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lprefetchsize: i32, pfaxoutgoingmessageiterator: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFaxOutgoingArchive_Impl::GetMessages(this, core::mem::transmute_copy(&lprefetchsize)) {
                    Ok(ok__) => {
                        pfaxoutgoingmessageiterator.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetMessage<Identity: IFaxOutgoingArchive_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrmessageid: *mut core::ffi::c_void, pfaxoutgoingmessage: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFaxOutgoingArchive_Impl::GetMessage(this, core::mem::transmute(&bstrmessageid)) {
                    Ok(ok__) => {
                        pfaxoutgoingmessage.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            UseArchive: UseArchive::<Identity, OFFSET>,
            SetUseArchive: SetUseArchive::<Identity, OFFSET>,
            ArchiveFolder: ArchiveFolder::<Identity, OFFSET>,
            SetArchiveFolder: SetArchiveFolder::<Identity, OFFSET>,
            SizeQuotaWarning: SizeQuotaWarning::<Identity, OFFSET>,
            SetSizeQuotaWarning: SetSizeQuotaWarning::<Identity, OFFSET>,
            HighQuotaWaterMark: HighQuotaWaterMark::<Identity, OFFSET>,
            SetHighQuotaWaterMark: SetHighQuotaWaterMark::<Identity, OFFSET>,
            LowQuotaWaterMark: LowQuotaWaterMark::<Identity, OFFSET>,
            SetLowQuotaWaterMark: SetLowQuotaWaterMark::<Identity, OFFSET>,
            AgeLimit: AgeLimit::<Identity, OFFSET>,
            SetAgeLimit: SetAgeLimit::<Identity, OFFSET>,
            SizeLow: SizeLow::<Identity, OFFSET>,
            SizeHigh: SizeHigh::<Identity, OFFSET>,
            Refresh: Refresh::<Identity, OFFSET>,
            Save: Save::<Identity, OFFSET>,
            GetMessages: GetMessages::<Identity, OFFSET>,
            GetMessage: GetMessage::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IFaxOutgoingArchive as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for IFaxOutgoingArchive {}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IFaxOutgoingJob, IFaxOutgoingJob_Vtbl, 0x6356daad_6614_4583_bf7a_3ad67bbfc71c);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IFaxOutgoingJob {
    type Target = super::super::System::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IFaxOutgoingJob, windows_core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl IFaxOutgoingJob {
    pub unsafe fn Subject(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Subject)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn DocumentName(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).DocumentName)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn Pages(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Pages)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn Size(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Size)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SubmissionId(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).SubmissionId)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn Id(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Id)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn OriginalScheduledTime(&self) -> windows_core::Result<f64> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).OriginalScheduledTime)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SubmissionTime(&self) -> windows_core::Result<f64> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).SubmissionTime)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn ReceiptType(&self) -> windows_core::Result<FAX_RECEIPT_TYPE_ENUM> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ReceiptType)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn Priority(&self) -> windows_core::Result<FAX_PRIORITY_TYPE_ENUM> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Priority)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn Sender(&self) -> windows_core::Result<IFaxSender> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Sender)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn Recipient(&self) -> windows_core::Result<IFaxRecipient> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Recipient)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn CurrentPage(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CurrentPage)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn DeviceId(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).DeviceId)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn Status(&self) -> windows_core::Result<FAX_JOB_STATUS_ENUM> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Status)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn ExtendedStatusCode(&self) -> windows_core::Result<FAX_JOB_EXTENDED_STATUS_ENUM> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ExtendedStatusCode)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn ExtendedStatus(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ExtendedStatus)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn AvailableOperations(&self) -> windows_core::Result<FAX_JOB_OPERATIONS_ENUM> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).AvailableOperations)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn Retries(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Retries)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn ScheduledTime(&self) -> windows_core::Result<f64> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ScheduledTime)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn TransmissionStart(&self) -> windows_core::Result<f64> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).TransmissionStart)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn TransmissionEnd(&self) -> windows_core::Result<f64> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).TransmissionEnd)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn CSID(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CSID)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn TSID(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).TSID)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn GroupBroadcastReceipts(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GroupBroadcastReceipts)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn Pause(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Pause)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn Resume(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Resume)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn Restart(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Restart)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn CopyTiff(&self, bstrtiffpath: &windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).CopyTiff)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrtiffpath)).ok() }
    }
    pub unsafe fn Refresh(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Refresh)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn Cancel(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Cancel)(windows_core::Interface::as_raw(self)).ok() }
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IFaxOutgoingJob_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub Subject: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub DocumentName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Pages: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub Size: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SubmissionId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Id: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub OriginalScheduledTime: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f64) -> windows_core::HRESULT,
    pub SubmissionTime: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f64) -> windows_core::HRESULT,
    pub ReceiptType: unsafe extern "system" fn(*mut core::ffi::c_void, *mut FAX_RECEIPT_TYPE_ENUM) -> windows_core::HRESULT,
    pub Priority: unsafe extern "system" fn(*mut core::ffi::c_void, *mut FAX_PRIORITY_TYPE_ENUM) -> windows_core::HRESULT,
    pub Sender: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Recipient: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CurrentPage: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub DeviceId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub Status: unsafe extern "system" fn(*mut core::ffi::c_void, *mut FAX_JOB_STATUS_ENUM) -> windows_core::HRESULT,
    pub ExtendedStatusCode: unsafe extern "system" fn(*mut core::ffi::c_void, *mut FAX_JOB_EXTENDED_STATUS_ENUM) -> windows_core::HRESULT,
    pub ExtendedStatus: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub AvailableOperations: unsafe extern "system" fn(*mut core::ffi::c_void, *mut FAX_JOB_OPERATIONS_ENUM) -> windows_core::HRESULT,
    pub Retries: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub ScheduledTime: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f64) -> windows_core::HRESULT,
    pub TransmissionStart: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f64) -> windows_core::HRESULT,
    pub TransmissionEnd: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f64) -> windows_core::HRESULT,
    pub CSID: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub TSID: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GroupBroadcastReceipts: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub Pause: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Resume: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Restart: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CopyTiff: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Refresh: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Cancel: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IFaxOutgoingJob_Impl: super::super::System::Com::IDispatch_Impl {
    fn Subject(&self) -> windows_core::Result<windows_core::BSTR>;
    fn DocumentName(&self) -> windows_core::Result<windows_core::BSTR>;
    fn Pages(&self) -> windows_core::Result<i32>;
    fn Size(&self) -> windows_core::Result<i32>;
    fn SubmissionId(&self) -> windows_core::Result<windows_core::BSTR>;
    fn Id(&self) -> windows_core::Result<windows_core::BSTR>;
    fn OriginalScheduledTime(&self) -> windows_core::Result<f64>;
    fn SubmissionTime(&self) -> windows_core::Result<f64>;
    fn ReceiptType(&self) -> windows_core::Result<FAX_RECEIPT_TYPE_ENUM>;
    fn Priority(&self) -> windows_core::Result<FAX_PRIORITY_TYPE_ENUM>;
    fn Sender(&self) -> windows_core::Result<IFaxSender>;
    fn Recipient(&self) -> windows_core::Result<IFaxRecipient>;
    fn CurrentPage(&self) -> windows_core::Result<i32>;
    fn DeviceId(&self) -> windows_core::Result<i32>;
    fn Status(&self) -> windows_core::Result<FAX_JOB_STATUS_ENUM>;
    fn ExtendedStatusCode(&self) -> windows_core::Result<FAX_JOB_EXTENDED_STATUS_ENUM>;
    fn ExtendedStatus(&self) -> windows_core::Result<windows_core::BSTR>;
    fn AvailableOperations(&self) -> windows_core::Result<FAX_JOB_OPERATIONS_ENUM>;
    fn Retries(&self) -> windows_core::Result<i32>;
    fn ScheduledTime(&self) -> windows_core::Result<f64>;
    fn TransmissionStart(&self) -> windows_core::Result<f64>;
    fn TransmissionEnd(&self) -> windows_core::Result<f64>;
    fn CSID(&self) -> windows_core::Result<windows_core::BSTR>;
    fn TSID(&self) -> windows_core::Result<windows_core::BSTR>;
    fn GroupBroadcastReceipts(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn Pause(&self) -> windows_core::Result<()>;
    fn Resume(&self) -> windows_core::Result<()>;
    fn Restart(&self) -> windows_core::Result<()>;
    fn CopyTiff(&self, bstrtiffpath: &windows_core::BSTR) -> windows_core::Result<()>;
    fn Refresh(&self) -> windows_core::Result<()>;
    fn Cancel(&self) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IFaxOutgoingJob_Vtbl {
    pub const fn new<Identity: IFaxOutgoingJob_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Subject<Identity: IFaxOutgoingJob_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrsubject: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFaxOutgoingJob_Impl::Subject(this) {
                    Ok(ok__) => {
                        pbstrsubject.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn DocumentName<Identity: IFaxOutgoingJob_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrdocumentname: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFaxOutgoingJob_Impl::DocumentName(this) {
                    Ok(ok__) => {
                        pbstrdocumentname.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Pages<Identity: IFaxOutgoingJob_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, plpages: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFaxOutgoingJob_Impl::Pages(this) {
                    Ok(ok__) => {
                        plpages.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Size<Identity: IFaxOutgoingJob_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, plsize: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFaxOutgoingJob_Impl::Size(this) {
                    Ok(ok__) => {
                        plsize.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SubmissionId<Identity: IFaxOutgoingJob_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrsubmissionid: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFaxOutgoingJob_Impl::SubmissionId(this) {
                    Ok(ok__) => {
                        pbstrsubmissionid.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Id<Identity: IFaxOutgoingJob_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrid: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFaxOutgoingJob_Impl::Id(this) {
                    Ok(ok__) => {
                        pbstrid.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn OriginalScheduledTime<Identity: IFaxOutgoingJob_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdateoriginalscheduledtime: *mut f64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFaxOutgoingJob_Impl::OriginalScheduledTime(this) {
                    Ok(ok__) => {
                        pdateoriginalscheduledtime.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SubmissionTime<Identity: IFaxOutgoingJob_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdatesubmissiontime: *mut f64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFaxOutgoingJob_Impl::SubmissionTime(this) {
                    Ok(ok__) => {
                        pdatesubmissiontime.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn ReceiptType<Identity: IFaxOutgoingJob_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, preceipttype: *mut FAX_RECEIPT_TYPE_ENUM) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFaxOutgoingJob_Impl::ReceiptType(this) {
                    Ok(ok__) => {
                        preceipttype.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Priority<Identity: IFaxOutgoingJob_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppriority: *mut FAX_PRIORITY_TYPE_ENUM) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFaxOutgoingJob_Impl::Priority(this) {
                    Ok(ok__) => {
                        ppriority.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Sender<Identity: IFaxOutgoingJob_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppfaxsender: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFaxOutgoingJob_Impl::Sender(this) {
                    Ok(ok__) => {
                        ppfaxsender.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Recipient<Identity: IFaxOutgoingJob_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppfaxrecipient: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFaxOutgoingJob_Impl::Recipient(this) {
                    Ok(ok__) => {
                        ppfaxrecipient.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CurrentPage<Identity: IFaxOutgoingJob_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, plcurrentpage: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFaxOutgoingJob_Impl::CurrentPage(this) {
                    Ok(ok__) => {
                        plcurrentpage.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn DeviceId<Identity: IFaxOutgoingJob_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pldeviceid: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFaxOutgoingJob_Impl::DeviceId(this) {
                    Ok(ok__) => {
                        pldeviceid.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Status<Identity: IFaxOutgoingJob_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pstatus: *mut FAX_JOB_STATUS_ENUM) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFaxOutgoingJob_Impl::Status(this) {
                    Ok(ok__) => {
                        pstatus.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn ExtendedStatusCode<Identity: IFaxOutgoingJob_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pextendedstatuscode: *mut FAX_JOB_EXTENDED_STATUS_ENUM) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFaxOutgoingJob_Impl::ExtendedStatusCode(this) {
                    Ok(ok__) => {
                        pextendedstatuscode.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn ExtendedStatus<Identity: IFaxOutgoingJob_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrextendedstatus: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFaxOutgoingJob_Impl::ExtendedStatus(this) {
                    Ok(ok__) => {
                        pbstrextendedstatus.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn AvailableOperations<Identity: IFaxOutgoingJob_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pavailableoperations: *mut FAX_JOB_OPERATIONS_ENUM) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFaxOutgoingJob_Impl::AvailableOperations(this) {
                    Ok(ok__) => {
                        pavailableoperations.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Retries<Identity: IFaxOutgoingJob_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, plretries: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFaxOutgoingJob_Impl::Retries(this) {
                    Ok(ok__) => {
                        plretries.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn ScheduledTime<Identity: IFaxOutgoingJob_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdatescheduledtime: *mut f64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFaxOutgoingJob_Impl::ScheduledTime(this) {
                    Ok(ok__) => {
                        pdatescheduledtime.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn TransmissionStart<Identity: IFaxOutgoingJob_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdatetransmissionstart: *mut f64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFaxOutgoingJob_Impl::TransmissionStart(this) {
                    Ok(ok__) => {
                        pdatetransmissionstart.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn TransmissionEnd<Identity: IFaxOutgoingJob_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdatetransmissionend: *mut f64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFaxOutgoingJob_Impl::TransmissionEnd(this) {
                    Ok(ok__) => {
                        pdatetransmissionend.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CSID<Identity: IFaxOutgoingJob_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrcsid: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFaxOutgoingJob_Impl::CSID(this) {
                    Ok(ok__) => {
                        pbstrcsid.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn TSID<Identity: IFaxOutgoingJob_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrtsid: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFaxOutgoingJob_Impl::TSID(this) {
                    Ok(ok__) => {
                        pbstrtsid.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GroupBroadcastReceipts<Identity: IFaxOutgoingJob_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbgroupbroadcastreceipts: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFaxOutgoingJob_Impl::GroupBroadcastReceipts(this) {
                    Ok(ok__) => {
                        pbgroupbroadcastreceipts.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Pause<Identity: IFaxOutgoingJob_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFaxOutgoingJob_Impl::Pause(this).into()
            }
        }
        unsafe extern "system" fn Resume<Identity: IFaxOutgoingJob_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFaxOutgoingJob_Impl::Resume(this).into()
            }
        }
        unsafe extern "system" fn Restart<Identity: IFaxOutgoingJob_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFaxOutgoingJob_Impl::Restart(this).into()
            }
        }
        unsafe extern "system" fn CopyTiff<Identity: IFaxOutgoingJob_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrtiffpath: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFaxOutgoingJob_Impl::CopyTiff(this, core::mem::transmute(&bstrtiffpath)).into()
            }
        }
        unsafe extern "system" fn Refresh<Identity: IFaxOutgoingJob_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFaxOutgoingJob_Impl::Refresh(this).into()
            }
        }
        unsafe extern "system" fn Cancel<Identity: IFaxOutgoingJob_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFaxOutgoingJob_Impl::Cancel(this).into()
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            Subject: Subject::<Identity, OFFSET>,
            DocumentName: DocumentName::<Identity, OFFSET>,
            Pages: Pages::<Identity, OFFSET>,
            Size: Size::<Identity, OFFSET>,
            SubmissionId: SubmissionId::<Identity, OFFSET>,
            Id: Id::<Identity, OFFSET>,
            OriginalScheduledTime: OriginalScheduledTime::<Identity, OFFSET>,
            SubmissionTime: SubmissionTime::<Identity, OFFSET>,
            ReceiptType: ReceiptType::<Identity, OFFSET>,
            Priority: Priority::<Identity, OFFSET>,
            Sender: Sender::<Identity, OFFSET>,
            Recipient: Recipient::<Identity, OFFSET>,
            CurrentPage: CurrentPage::<Identity, OFFSET>,
            DeviceId: DeviceId::<Identity, OFFSET>,
            Status: Status::<Identity, OFFSET>,
            ExtendedStatusCode: ExtendedStatusCode::<Identity, OFFSET>,
            ExtendedStatus: ExtendedStatus::<Identity, OFFSET>,
            AvailableOperations: AvailableOperations::<Identity, OFFSET>,
            Retries: Retries::<Identity, OFFSET>,
            ScheduledTime: ScheduledTime::<Identity, OFFSET>,
            TransmissionStart: TransmissionStart::<Identity, OFFSET>,
            TransmissionEnd: TransmissionEnd::<Identity, OFFSET>,
            CSID: CSID::<Identity, OFFSET>,
            TSID: TSID::<Identity, OFFSET>,
            GroupBroadcastReceipts: GroupBroadcastReceipts::<Identity, OFFSET>,
            Pause: Pause::<Identity, OFFSET>,
            Resume: Resume::<Identity, OFFSET>,
            Restart: Restart::<Identity, OFFSET>,
            CopyTiff: CopyTiff::<Identity, OFFSET>,
            Refresh: Refresh::<Identity, OFFSET>,
            Cancel: Cancel::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IFaxOutgoingJob as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for IFaxOutgoingJob {}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IFaxOutgoingJob2, IFaxOutgoingJob2_Vtbl, 0x418a8d96_59a0_4789_b176_edf3dc8fa8f7);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IFaxOutgoingJob2 {
    type Target = IFaxOutgoingJob;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IFaxOutgoingJob2, windows_core::IUnknown, super::super::System::Com::IDispatch, IFaxOutgoingJob);
#[cfg(feature = "Win32_System_Com")]
impl IFaxOutgoingJob2 {
    pub unsafe fn HasCoverPage(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).HasCoverPage)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn ReceiptAddress(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ReceiptAddress)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn ScheduleType(&self) -> windows_core::Result<FAX_SCHEDULE_TYPE_ENUM> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ScheduleType)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IFaxOutgoingJob2_Vtbl {
    pub base__: IFaxOutgoingJob_Vtbl,
    pub HasCoverPage: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub ReceiptAddress: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ScheduleType: unsafe extern "system" fn(*mut core::ffi::c_void, *mut FAX_SCHEDULE_TYPE_ENUM) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IFaxOutgoingJob2_Impl: IFaxOutgoingJob_Impl {
    fn HasCoverPage(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn ReceiptAddress(&self) -> windows_core::Result<windows_core::BSTR>;
    fn ScheduleType(&self) -> windows_core::Result<FAX_SCHEDULE_TYPE_ENUM>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IFaxOutgoingJob2_Vtbl {
    pub const fn new<Identity: IFaxOutgoingJob2_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn HasCoverPage<Identity: IFaxOutgoingJob2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbhascoverpage: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFaxOutgoingJob2_Impl::HasCoverPage(this) {
                    Ok(ok__) => {
                        pbhascoverpage.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn ReceiptAddress<Identity: IFaxOutgoingJob2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrreceiptaddress: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFaxOutgoingJob2_Impl::ReceiptAddress(this) {
                    Ok(ok__) => {
                        pbstrreceiptaddress.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn ScheduleType<Identity: IFaxOutgoingJob2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pscheduletype: *mut FAX_SCHEDULE_TYPE_ENUM) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFaxOutgoingJob2_Impl::ScheduleType(this) {
                    Ok(ok__) => {
                        pscheduletype.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: IFaxOutgoingJob_Vtbl::new::<Identity, OFFSET>(),
            HasCoverPage: HasCoverPage::<Identity, OFFSET>,
            ReceiptAddress: ReceiptAddress::<Identity, OFFSET>,
            ScheduleType: ScheduleType::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IFaxOutgoingJob2 as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID || iid == &<IFaxOutgoingJob as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for IFaxOutgoingJob2 {}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IFaxOutgoingJobs, IFaxOutgoingJobs_Vtbl, 0x2c56d8e6_8c2f_4573_944c_e505f8f5aeed);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IFaxOutgoingJobs {
    type Target = super::super::System::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IFaxOutgoingJobs, windows_core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl IFaxOutgoingJobs {
    pub unsafe fn _NewEnum(&self) -> windows_core::Result<windows_core::IUnknown> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self)._NewEnum)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn get_Item(&self, vindex: &super::super::System::Variant::VARIANT) -> windows_core::Result<IFaxOutgoingJob> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_Item)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(vindex), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn Count(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Count)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IFaxOutgoingJobs_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub _NewEnum: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub get_Item: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::System::Variant::VARIANT, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    get_Item: usize,
    pub Count: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IFaxOutgoingJobs_Impl: super::super::System::Com::IDispatch_Impl {
    fn _NewEnum(&self) -> windows_core::Result<windows_core::IUnknown>;
    fn get_Item(&self, vindex: &super::super::System::Variant::VARIANT) -> windows_core::Result<IFaxOutgoingJob>;
    fn Count(&self) -> windows_core::Result<i32>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IFaxOutgoingJobs_Vtbl {
    pub const fn new<Identity: IFaxOutgoingJobs_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn _NewEnum<Identity: IFaxOutgoingJobs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppunk: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFaxOutgoingJobs_Impl::_NewEnum(this) {
                    Ok(ok__) => {
                        ppunk.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_Item<Identity: IFaxOutgoingJobs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, vindex: super::super::System::Variant::VARIANT, pfaxoutgoingjob: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFaxOutgoingJobs_Impl::get_Item(this, core::mem::transmute(&vindex)) {
                    Ok(ok__) => {
                        pfaxoutgoingjob.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Count<Identity: IFaxOutgoingJobs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, plcount: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFaxOutgoingJobs_Impl::Count(this) {
                    Ok(ok__) => {
                        plcount.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            _NewEnum: _NewEnum::<Identity, OFFSET>,
            get_Item: get_Item::<Identity, OFFSET>,
            Count: Count::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IFaxOutgoingJobs as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for IFaxOutgoingJobs {}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IFaxOutgoingMessage, IFaxOutgoingMessage_Vtbl, 0xf0ea35de_caa5_4a7c_82c7_2b60ba5f2be2);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IFaxOutgoingMessage {
    type Target = super::super::System::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IFaxOutgoingMessage, windows_core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl IFaxOutgoingMessage {
    pub unsafe fn SubmissionId(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).SubmissionId)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn Id(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Id)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn Subject(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Subject)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn DocumentName(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).DocumentName)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn Retries(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Retries)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn Pages(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Pages)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn Size(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Size)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn OriginalScheduledTime(&self) -> windows_core::Result<f64> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).OriginalScheduledTime)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SubmissionTime(&self) -> windows_core::Result<f64> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).SubmissionTime)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn Priority(&self) -> windows_core::Result<FAX_PRIORITY_TYPE_ENUM> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Priority)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn Sender(&self) -> windows_core::Result<IFaxSender> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Sender)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn Recipient(&self) -> windows_core::Result<IFaxRecipient> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Recipient)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn DeviceName(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).DeviceName)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn TransmissionStart(&self) -> windows_core::Result<f64> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).TransmissionStart)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn TransmissionEnd(&self) -> windows_core::Result<f64> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).TransmissionEnd)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn CSID(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CSID)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn TSID(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).TSID)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn CopyTiff(&self, bstrtiffpath: &windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).CopyTiff)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrtiffpath)).ok() }
    }
    pub unsafe fn Delete(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Delete)(windows_core::Interface::as_raw(self)).ok() }
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IFaxOutgoingMessage_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub SubmissionId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Id: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Subject: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub DocumentName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Retries: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub Pages: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub Size: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub OriginalScheduledTime: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f64) -> windows_core::HRESULT,
    pub SubmissionTime: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f64) -> windows_core::HRESULT,
    pub Priority: unsafe extern "system" fn(*mut core::ffi::c_void, *mut FAX_PRIORITY_TYPE_ENUM) -> windows_core::HRESULT,
    pub Sender: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Recipient: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub DeviceName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub TransmissionStart: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f64) -> windows_core::HRESULT,
    pub TransmissionEnd: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f64) -> windows_core::HRESULT,
    pub CSID: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub TSID: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CopyTiff: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Delete: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IFaxOutgoingMessage_Impl: super::super::System::Com::IDispatch_Impl {
    fn SubmissionId(&self) -> windows_core::Result<windows_core::BSTR>;
    fn Id(&self) -> windows_core::Result<windows_core::BSTR>;
    fn Subject(&self) -> windows_core::Result<windows_core::BSTR>;
    fn DocumentName(&self) -> windows_core::Result<windows_core::BSTR>;
    fn Retries(&self) -> windows_core::Result<i32>;
    fn Pages(&self) -> windows_core::Result<i32>;
    fn Size(&self) -> windows_core::Result<i32>;
    fn OriginalScheduledTime(&self) -> windows_core::Result<f64>;
    fn SubmissionTime(&self) -> windows_core::Result<f64>;
    fn Priority(&self) -> windows_core::Result<FAX_PRIORITY_TYPE_ENUM>;
    fn Sender(&self) -> windows_core::Result<IFaxSender>;
    fn Recipient(&self) -> windows_core::Result<IFaxRecipient>;
    fn DeviceName(&self) -> windows_core::Result<windows_core::BSTR>;
    fn TransmissionStart(&self) -> windows_core::Result<f64>;
    fn TransmissionEnd(&self) -> windows_core::Result<f64>;
    fn CSID(&self) -> windows_core::Result<windows_core::BSTR>;
    fn TSID(&self) -> windows_core::Result<windows_core::BSTR>;
    fn CopyTiff(&self, bstrtiffpath: &windows_core::BSTR) -> windows_core::Result<()>;
    fn Delete(&self) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IFaxOutgoingMessage_Vtbl {
    pub const fn new<Identity: IFaxOutgoingMessage_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn SubmissionId<Identity: IFaxOutgoingMessage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrsubmissionid: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFaxOutgoingMessage_Impl::SubmissionId(this) {
                    Ok(ok__) => {
                        pbstrsubmissionid.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Id<Identity: IFaxOutgoingMessage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrid: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFaxOutgoingMessage_Impl::Id(this) {
                    Ok(ok__) => {
                        pbstrid.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Subject<Identity: IFaxOutgoingMessage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrsubject: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFaxOutgoingMessage_Impl::Subject(this) {
                    Ok(ok__) => {
                        pbstrsubject.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn DocumentName<Identity: IFaxOutgoingMessage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrdocumentname: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFaxOutgoingMessage_Impl::DocumentName(this) {
                    Ok(ok__) => {
                        pbstrdocumentname.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Retries<Identity: IFaxOutgoingMessage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, plretries: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFaxOutgoingMessage_Impl::Retries(this) {
                    Ok(ok__) => {
                        plretries.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Pages<Identity: IFaxOutgoingMessage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, plpages: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFaxOutgoingMessage_Impl::Pages(this) {
                    Ok(ok__) => {
                        plpages.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Size<Identity: IFaxOutgoingMessage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, plsize: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFaxOutgoingMessage_Impl::Size(this) {
                    Ok(ok__) => {
                        plsize.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn OriginalScheduledTime<Identity: IFaxOutgoingMessage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdateoriginalscheduledtime: *mut f64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFaxOutgoingMessage_Impl::OriginalScheduledTime(this) {
                    Ok(ok__) => {
                        pdateoriginalscheduledtime.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SubmissionTime<Identity: IFaxOutgoingMessage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdatesubmissiontime: *mut f64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFaxOutgoingMessage_Impl::SubmissionTime(this) {
                    Ok(ok__) => {
                        pdatesubmissiontime.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Priority<Identity: IFaxOutgoingMessage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppriority: *mut FAX_PRIORITY_TYPE_ENUM) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFaxOutgoingMessage_Impl::Priority(this) {
                    Ok(ok__) => {
                        ppriority.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Sender<Identity: IFaxOutgoingMessage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppfaxsender: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFaxOutgoingMessage_Impl::Sender(this) {
                    Ok(ok__) => {
                        ppfaxsender.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Recipient<Identity: IFaxOutgoingMessage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppfaxrecipient: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFaxOutgoingMessage_Impl::Recipient(this) {
                    Ok(ok__) => {
                        ppfaxrecipient.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn DeviceName<Identity: IFaxOutgoingMessage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrdevicename: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFaxOutgoingMessage_Impl::DeviceName(this) {
                    Ok(ok__) => {
                        pbstrdevicename.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn TransmissionStart<Identity: IFaxOutgoingMessage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdatetransmissionstart: *mut f64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFaxOutgoingMessage_Impl::TransmissionStart(this) {
                    Ok(ok__) => {
                        pdatetransmissionstart.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn TransmissionEnd<Identity: IFaxOutgoingMessage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdatetransmissionend: *mut f64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFaxOutgoingMessage_Impl::TransmissionEnd(this) {
                    Ok(ok__) => {
                        pdatetransmissionend.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CSID<Identity: IFaxOutgoingMessage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrcsid: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFaxOutgoingMessage_Impl::CSID(this) {
                    Ok(ok__) => {
                        pbstrcsid.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn TSID<Identity: IFaxOutgoingMessage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrtsid: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFaxOutgoingMessage_Impl::TSID(this) {
                    Ok(ok__) => {
                        pbstrtsid.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CopyTiff<Identity: IFaxOutgoingMessage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrtiffpath: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFaxOutgoingMessage_Impl::CopyTiff(this, core::mem::transmute(&bstrtiffpath)).into()
            }
        }
        unsafe extern "system" fn Delete<Identity: IFaxOutgoingMessage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFaxOutgoingMessage_Impl::Delete(this).into()
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            SubmissionId: SubmissionId::<Identity, OFFSET>,
            Id: Id::<Identity, OFFSET>,
            Subject: Subject::<Identity, OFFSET>,
            DocumentName: DocumentName::<Identity, OFFSET>,
            Retries: Retries::<Identity, OFFSET>,
            Pages: Pages::<Identity, OFFSET>,
            Size: Size::<Identity, OFFSET>,
            OriginalScheduledTime: OriginalScheduledTime::<Identity, OFFSET>,
            SubmissionTime: SubmissionTime::<Identity, OFFSET>,
            Priority: Priority::<Identity, OFFSET>,
            Sender: Sender::<Identity, OFFSET>,
            Recipient: Recipient::<Identity, OFFSET>,
            DeviceName: DeviceName::<Identity, OFFSET>,
            TransmissionStart: TransmissionStart::<Identity, OFFSET>,
            TransmissionEnd: TransmissionEnd::<Identity, OFFSET>,
            CSID: CSID::<Identity, OFFSET>,
            TSID: TSID::<Identity, OFFSET>,
            CopyTiff: CopyTiff::<Identity, OFFSET>,
            Delete: Delete::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IFaxOutgoingMessage as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for IFaxOutgoingMessage {}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IFaxOutgoingMessage2, IFaxOutgoingMessage2_Vtbl, 0xb37df687_bc88_4b46_b3be_b458b3ea9e7f);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IFaxOutgoingMessage2 {
    type Target = IFaxOutgoingMessage;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IFaxOutgoingMessage2, windows_core::IUnknown, super::super::System::Com::IDispatch, IFaxOutgoingMessage);
#[cfg(feature = "Win32_System_Com")]
impl IFaxOutgoingMessage2 {
    pub unsafe fn HasCoverPage(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).HasCoverPage)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn ReceiptType(&self) -> windows_core::Result<FAX_RECEIPT_TYPE_ENUM> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ReceiptType)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn ReceiptAddress(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ReceiptAddress)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn Read(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Read)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetRead(&self, bread: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetRead)(windows_core::Interface::as_raw(self), bread).ok() }
    }
    pub unsafe fn Save(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Save)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn Refresh(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Refresh)(windows_core::Interface::as_raw(self)).ok() }
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IFaxOutgoingMessage2_Vtbl {
    pub base__: IFaxOutgoingMessage_Vtbl,
    pub HasCoverPage: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub ReceiptType: unsafe extern "system" fn(*mut core::ffi::c_void, *mut FAX_RECEIPT_TYPE_ENUM) -> windows_core::HRESULT,
    pub ReceiptAddress: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Read: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub SetRead: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub Save: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Refresh: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IFaxOutgoingMessage2_Impl: IFaxOutgoingMessage_Impl {
    fn HasCoverPage(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn ReceiptType(&self) -> windows_core::Result<FAX_RECEIPT_TYPE_ENUM>;
    fn ReceiptAddress(&self) -> windows_core::Result<windows_core::BSTR>;
    fn Read(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn SetRead(&self, bread: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
    fn Save(&self) -> windows_core::Result<()>;
    fn Refresh(&self) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IFaxOutgoingMessage2_Vtbl {
    pub const fn new<Identity: IFaxOutgoingMessage2_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn HasCoverPage<Identity: IFaxOutgoingMessage2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbhascoverpage: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFaxOutgoingMessage2_Impl::HasCoverPage(this) {
                    Ok(ok__) => {
                        pbhascoverpage.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn ReceiptType<Identity: IFaxOutgoingMessage2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, preceipttype: *mut FAX_RECEIPT_TYPE_ENUM) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFaxOutgoingMessage2_Impl::ReceiptType(this) {
                    Ok(ok__) => {
                        preceipttype.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn ReceiptAddress<Identity: IFaxOutgoingMessage2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrreceiptaddress: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFaxOutgoingMessage2_Impl::ReceiptAddress(this) {
                    Ok(ok__) => {
                        pbstrreceiptaddress.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Read<Identity: IFaxOutgoingMessage2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbread: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFaxOutgoingMessage2_Impl::Read(this) {
                    Ok(ok__) => {
                        pbread.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetRead<Identity: IFaxOutgoingMessage2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bread: super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFaxOutgoingMessage2_Impl::SetRead(this, core::mem::transmute_copy(&bread)).into()
            }
        }
        unsafe extern "system" fn Save<Identity: IFaxOutgoingMessage2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFaxOutgoingMessage2_Impl::Save(this).into()
            }
        }
        unsafe extern "system" fn Refresh<Identity: IFaxOutgoingMessage2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFaxOutgoingMessage2_Impl::Refresh(this).into()
            }
        }
        Self {
            base__: IFaxOutgoingMessage_Vtbl::new::<Identity, OFFSET>(),
            HasCoverPage: HasCoverPage::<Identity, OFFSET>,
            ReceiptType: ReceiptType::<Identity, OFFSET>,
            ReceiptAddress: ReceiptAddress::<Identity, OFFSET>,
            Read: Read::<Identity, OFFSET>,
            SetRead: SetRead::<Identity, OFFSET>,
            Save: Save::<Identity, OFFSET>,
            Refresh: Refresh::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IFaxOutgoingMessage2 as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID || iid == &<IFaxOutgoingMessage as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for IFaxOutgoingMessage2 {}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IFaxOutgoingMessageIterator, IFaxOutgoingMessageIterator_Vtbl, 0xf5ec5d4f_b840_432f_9980_112fe42a9b7a);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IFaxOutgoingMessageIterator {
    type Target = super::super::System::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IFaxOutgoingMessageIterator, windows_core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl IFaxOutgoingMessageIterator {
    pub unsafe fn Message(&self) -> windows_core::Result<IFaxOutgoingMessage> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Message)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn AtEOF(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).AtEOF)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn PrefetchSize(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).PrefetchSize)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetPrefetchSize(&self, lprefetchsize: i32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetPrefetchSize)(windows_core::Interface::as_raw(self), lprefetchsize).ok() }
    }
    pub unsafe fn MoveFirst(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).MoveFirst)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn MoveNext(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).MoveNext)(windows_core::Interface::as_raw(self)).ok() }
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IFaxOutgoingMessageIterator_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub Message: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub AtEOF: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub PrefetchSize: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetPrefetchSize: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub MoveFirst: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub MoveNext: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IFaxOutgoingMessageIterator_Impl: super::super::System::Com::IDispatch_Impl {
    fn Message(&self) -> windows_core::Result<IFaxOutgoingMessage>;
    fn AtEOF(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn PrefetchSize(&self) -> windows_core::Result<i32>;
    fn SetPrefetchSize(&self, lprefetchsize: i32) -> windows_core::Result<()>;
    fn MoveFirst(&self) -> windows_core::Result<()>;
    fn MoveNext(&self) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IFaxOutgoingMessageIterator_Vtbl {
    pub const fn new<Identity: IFaxOutgoingMessageIterator_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Message<Identity: IFaxOutgoingMessageIterator_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pfaxoutgoingmessage: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFaxOutgoingMessageIterator_Impl::Message(this) {
                    Ok(ok__) => {
                        pfaxoutgoingmessage.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn AtEOF<Identity: IFaxOutgoingMessageIterator_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbeof: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFaxOutgoingMessageIterator_Impl::AtEOF(this) {
                    Ok(ok__) => {
                        pbeof.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn PrefetchSize<Identity: IFaxOutgoingMessageIterator_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, plprefetchsize: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFaxOutgoingMessageIterator_Impl::PrefetchSize(this) {
                    Ok(ok__) => {
                        plprefetchsize.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetPrefetchSize<Identity: IFaxOutgoingMessageIterator_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lprefetchsize: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFaxOutgoingMessageIterator_Impl::SetPrefetchSize(this, core::mem::transmute_copy(&lprefetchsize)).into()
            }
        }
        unsafe extern "system" fn MoveFirst<Identity: IFaxOutgoingMessageIterator_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFaxOutgoingMessageIterator_Impl::MoveFirst(this).into()
            }
        }
        unsafe extern "system" fn MoveNext<Identity: IFaxOutgoingMessageIterator_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFaxOutgoingMessageIterator_Impl::MoveNext(this).into()
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            Message: Message::<Identity, OFFSET>,
            AtEOF: AtEOF::<Identity, OFFSET>,
            PrefetchSize: PrefetchSize::<Identity, OFFSET>,
            SetPrefetchSize: SetPrefetchSize::<Identity, OFFSET>,
            MoveFirst: MoveFirst::<Identity, OFFSET>,
            MoveNext: MoveNext::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IFaxOutgoingMessageIterator as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for IFaxOutgoingMessageIterator {}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IFaxOutgoingQueue, IFaxOutgoingQueue_Vtbl, 0x80b1df24_d9ac_4333_b373_487cedc80ce5);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IFaxOutgoingQueue {
    type Target = super::super::System::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IFaxOutgoingQueue, windows_core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl IFaxOutgoingQueue {
    pub unsafe fn Blocked(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Blocked)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetBlocked(&self, bblocked: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetBlocked)(windows_core::Interface::as_raw(self), bblocked).ok() }
    }
    pub unsafe fn Paused(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Paused)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetPaused(&self, bpaused: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetPaused)(windows_core::Interface::as_raw(self), bpaused).ok() }
    }
    pub unsafe fn AllowPersonalCoverPages(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).AllowPersonalCoverPages)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetAllowPersonalCoverPages(&self, ballowpersonalcoverpages: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetAllowPersonalCoverPages)(windows_core::Interface::as_raw(self), ballowpersonalcoverpages).ok() }
    }
    pub unsafe fn UseDeviceTSID(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).UseDeviceTSID)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetUseDeviceTSID(&self, busedevicetsid: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetUseDeviceTSID)(windows_core::Interface::as_raw(self), busedevicetsid).ok() }
    }
    pub unsafe fn Retries(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Retries)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetRetries(&self, lretries: i32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetRetries)(windows_core::Interface::as_raw(self), lretries).ok() }
    }
    pub unsafe fn RetryDelay(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).RetryDelay)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetRetryDelay(&self, lretrydelay: i32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetRetryDelay)(windows_core::Interface::as_raw(self), lretrydelay).ok() }
    }
    pub unsafe fn DiscountRateStart(&self) -> windows_core::Result<f64> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).DiscountRateStart)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetDiscountRateStart(&self, datediscountratestart: f64) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetDiscountRateStart)(windows_core::Interface::as_raw(self), datediscountratestart).ok() }
    }
    pub unsafe fn DiscountRateEnd(&self) -> windows_core::Result<f64> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).DiscountRateEnd)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetDiscountRateEnd(&self, datediscountrateend: f64) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetDiscountRateEnd)(windows_core::Interface::as_raw(self), datediscountrateend).ok() }
    }
    pub unsafe fn AgeLimit(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).AgeLimit)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetAgeLimit(&self, lagelimit: i32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetAgeLimit)(windows_core::Interface::as_raw(self), lagelimit).ok() }
    }
    pub unsafe fn Branding(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Branding)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetBranding(&self, bbranding: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetBranding)(windows_core::Interface::as_raw(self), bbranding).ok() }
    }
    pub unsafe fn Refresh(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Refresh)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn Save(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Save)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn GetJobs(&self) -> windows_core::Result<IFaxOutgoingJobs> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetJobs)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetJob(&self, bstrjobid: &windows_core::BSTR) -> windows_core::Result<IFaxOutgoingJob> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetJob)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrjobid), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IFaxOutgoingQueue_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub Blocked: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub SetBlocked: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub Paused: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub SetPaused: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub AllowPersonalCoverPages: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub SetAllowPersonalCoverPages: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub UseDeviceTSID: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub SetUseDeviceTSID: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub Retries: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetRetries: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub RetryDelay: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetRetryDelay: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub DiscountRateStart: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f64) -> windows_core::HRESULT,
    pub SetDiscountRateStart: unsafe extern "system" fn(*mut core::ffi::c_void, f64) -> windows_core::HRESULT,
    pub DiscountRateEnd: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f64) -> windows_core::HRESULT,
    pub SetDiscountRateEnd: unsafe extern "system" fn(*mut core::ffi::c_void, f64) -> windows_core::HRESULT,
    pub AgeLimit: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetAgeLimit: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub Branding: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub SetBranding: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub Refresh: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Save: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetJobs: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetJob: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IFaxOutgoingQueue_Impl: super::super::System::Com::IDispatch_Impl {
    fn Blocked(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn SetBlocked(&self, bblocked: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
    fn Paused(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn SetPaused(&self, bpaused: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
    fn AllowPersonalCoverPages(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn SetAllowPersonalCoverPages(&self, ballowpersonalcoverpages: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
    fn UseDeviceTSID(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn SetUseDeviceTSID(&self, busedevicetsid: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
    fn Retries(&self) -> windows_core::Result<i32>;
    fn SetRetries(&self, lretries: i32) -> windows_core::Result<()>;
    fn RetryDelay(&self) -> windows_core::Result<i32>;
    fn SetRetryDelay(&self, lretrydelay: i32) -> windows_core::Result<()>;
    fn DiscountRateStart(&self) -> windows_core::Result<f64>;
    fn SetDiscountRateStart(&self, datediscountratestart: f64) -> windows_core::Result<()>;
    fn DiscountRateEnd(&self) -> windows_core::Result<f64>;
    fn SetDiscountRateEnd(&self, datediscountrateend: f64) -> windows_core::Result<()>;
    fn AgeLimit(&self) -> windows_core::Result<i32>;
    fn SetAgeLimit(&self, lagelimit: i32) -> windows_core::Result<()>;
    fn Branding(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn SetBranding(&self, bbranding: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
    fn Refresh(&self) -> windows_core::Result<()>;
    fn Save(&self) -> windows_core::Result<()>;
    fn GetJobs(&self) -> windows_core::Result<IFaxOutgoingJobs>;
    fn GetJob(&self, bstrjobid: &windows_core::BSTR) -> windows_core::Result<IFaxOutgoingJob>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IFaxOutgoingQueue_Vtbl {
    pub const fn new<Identity: IFaxOutgoingQueue_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Blocked<Identity: IFaxOutgoingQueue_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbblocked: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFaxOutgoingQueue_Impl::Blocked(this) {
                    Ok(ok__) => {
                        pbblocked.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetBlocked<Identity: IFaxOutgoingQueue_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bblocked: super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFaxOutgoingQueue_Impl::SetBlocked(this, core::mem::transmute_copy(&bblocked)).into()
            }
        }
        unsafe extern "system" fn Paused<Identity: IFaxOutgoingQueue_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbpaused: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFaxOutgoingQueue_Impl::Paused(this) {
                    Ok(ok__) => {
                        pbpaused.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetPaused<Identity: IFaxOutgoingQueue_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bpaused: super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFaxOutgoingQueue_Impl::SetPaused(this, core::mem::transmute_copy(&bpaused)).into()
            }
        }
        unsafe extern "system" fn AllowPersonalCoverPages<Identity: IFaxOutgoingQueue_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pballowpersonalcoverpages: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFaxOutgoingQueue_Impl::AllowPersonalCoverPages(this) {
                    Ok(ok__) => {
                        pballowpersonalcoverpages.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetAllowPersonalCoverPages<Identity: IFaxOutgoingQueue_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ballowpersonalcoverpages: super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFaxOutgoingQueue_Impl::SetAllowPersonalCoverPages(this, core::mem::transmute_copy(&ballowpersonalcoverpages)).into()
            }
        }
        unsafe extern "system" fn UseDeviceTSID<Identity: IFaxOutgoingQueue_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbusedevicetsid: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFaxOutgoingQueue_Impl::UseDeviceTSID(this) {
                    Ok(ok__) => {
                        pbusedevicetsid.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetUseDeviceTSID<Identity: IFaxOutgoingQueue_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, busedevicetsid: super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFaxOutgoingQueue_Impl::SetUseDeviceTSID(this, core::mem::transmute_copy(&busedevicetsid)).into()
            }
        }
        unsafe extern "system" fn Retries<Identity: IFaxOutgoingQueue_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, plretries: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFaxOutgoingQueue_Impl::Retries(this) {
                    Ok(ok__) => {
                        plretries.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetRetries<Identity: IFaxOutgoingQueue_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lretries: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFaxOutgoingQueue_Impl::SetRetries(this, core::mem::transmute_copy(&lretries)).into()
            }
        }
        unsafe extern "system" fn RetryDelay<Identity: IFaxOutgoingQueue_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, plretrydelay: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFaxOutgoingQueue_Impl::RetryDelay(this) {
                    Ok(ok__) => {
                        plretrydelay.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetRetryDelay<Identity: IFaxOutgoingQueue_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lretrydelay: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFaxOutgoingQueue_Impl::SetRetryDelay(this, core::mem::transmute_copy(&lretrydelay)).into()
            }
        }
        unsafe extern "system" fn DiscountRateStart<Identity: IFaxOutgoingQueue_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdatediscountratestart: *mut f64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFaxOutgoingQueue_Impl::DiscountRateStart(this) {
                    Ok(ok__) => {
                        pdatediscountratestart.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetDiscountRateStart<Identity: IFaxOutgoingQueue_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, datediscountratestart: f64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFaxOutgoingQueue_Impl::SetDiscountRateStart(this, core::mem::transmute_copy(&datediscountratestart)).into()
            }
        }
        unsafe extern "system" fn DiscountRateEnd<Identity: IFaxOutgoingQueue_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdatediscountrateend: *mut f64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFaxOutgoingQueue_Impl::DiscountRateEnd(this) {
                    Ok(ok__) => {
                        pdatediscountrateend.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetDiscountRateEnd<Identity: IFaxOutgoingQueue_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, datediscountrateend: f64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFaxOutgoingQueue_Impl::SetDiscountRateEnd(this, core::mem::transmute_copy(&datediscountrateend)).into()
            }
        }
        unsafe extern "system" fn AgeLimit<Identity: IFaxOutgoingQueue_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, plagelimit: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFaxOutgoingQueue_Impl::AgeLimit(this) {
                    Ok(ok__) => {
                        plagelimit.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetAgeLimit<Identity: IFaxOutgoingQueue_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lagelimit: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFaxOutgoingQueue_Impl::SetAgeLimit(this, core::mem::transmute_copy(&lagelimit)).into()
            }
        }
        unsafe extern "system" fn Branding<Identity: IFaxOutgoingQueue_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbbranding: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFaxOutgoingQueue_Impl::Branding(this) {
                    Ok(ok__) => {
                        pbbranding.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetBranding<Identity: IFaxOutgoingQueue_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bbranding: super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFaxOutgoingQueue_Impl::SetBranding(this, core::mem::transmute_copy(&bbranding)).into()
            }
        }
        unsafe extern "system" fn Refresh<Identity: IFaxOutgoingQueue_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFaxOutgoingQueue_Impl::Refresh(this).into()
            }
        }
        unsafe extern "system" fn Save<Identity: IFaxOutgoingQueue_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFaxOutgoingQueue_Impl::Save(this).into()
            }
        }
        unsafe extern "system" fn GetJobs<Identity: IFaxOutgoingQueue_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pfaxoutgoingjobs: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFaxOutgoingQueue_Impl::GetJobs(this) {
                    Ok(ok__) => {
                        pfaxoutgoingjobs.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetJob<Identity: IFaxOutgoingQueue_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrjobid: *mut core::ffi::c_void, pfaxoutgoingjob: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFaxOutgoingQueue_Impl::GetJob(this, core::mem::transmute(&bstrjobid)) {
                    Ok(ok__) => {
                        pfaxoutgoingjob.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            Blocked: Blocked::<Identity, OFFSET>,
            SetBlocked: SetBlocked::<Identity, OFFSET>,
            Paused: Paused::<Identity, OFFSET>,
            SetPaused: SetPaused::<Identity, OFFSET>,
            AllowPersonalCoverPages: AllowPersonalCoverPages::<Identity, OFFSET>,
            SetAllowPersonalCoverPages: SetAllowPersonalCoverPages::<Identity, OFFSET>,
            UseDeviceTSID: UseDeviceTSID::<Identity, OFFSET>,
            SetUseDeviceTSID: SetUseDeviceTSID::<Identity, OFFSET>,
            Retries: Retries::<Identity, OFFSET>,
            SetRetries: SetRetries::<Identity, OFFSET>,
            RetryDelay: RetryDelay::<Identity, OFFSET>,
            SetRetryDelay: SetRetryDelay::<Identity, OFFSET>,
            DiscountRateStart: DiscountRateStart::<Identity, OFFSET>,
            SetDiscountRateStart: SetDiscountRateStart::<Identity, OFFSET>,
            DiscountRateEnd: DiscountRateEnd::<Identity, OFFSET>,
            SetDiscountRateEnd: SetDiscountRateEnd::<Identity, OFFSET>,
            AgeLimit: AgeLimit::<Identity, OFFSET>,
            SetAgeLimit: SetAgeLimit::<Identity, OFFSET>,
            Branding: Branding::<Identity, OFFSET>,
            SetBranding: SetBranding::<Identity, OFFSET>,
            Refresh: Refresh::<Identity, OFFSET>,
            Save: Save::<Identity, OFFSET>,
            GetJobs: GetJobs::<Identity, OFFSET>,
            GetJob: GetJob::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IFaxOutgoingQueue as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for IFaxOutgoingQueue {}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IFaxReceiptOptions, IFaxReceiptOptions_Vtbl, 0x378efaeb_5fcb_4afb_b2ee_e16e80614487);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IFaxReceiptOptions {
    type Target = super::super::System::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IFaxReceiptOptions, windows_core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl IFaxReceiptOptions {
    pub unsafe fn AuthenticationType(&self) -> windows_core::Result<FAX_SMTP_AUTHENTICATION_TYPE_ENUM> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).AuthenticationType)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetAuthenticationType(&self, r#type: FAX_SMTP_AUTHENTICATION_TYPE_ENUM) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetAuthenticationType)(windows_core::Interface::as_raw(self), r#type).ok() }
    }
    pub unsafe fn SMTPServer(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).SMTPServer)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn SetSMTPServer(&self, bstrsmtpserver: &windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetSMTPServer)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrsmtpserver)).ok() }
    }
    pub unsafe fn SMTPPort(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).SMTPPort)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetSMTPPort(&self, lsmtpport: i32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetSMTPPort)(windows_core::Interface::as_raw(self), lsmtpport).ok() }
    }
    pub unsafe fn SMTPSender(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).SMTPSender)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn SetSMTPSender(&self, bstrsmtpsender: &windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetSMTPSender)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrsmtpsender)).ok() }
    }
    pub unsafe fn SMTPUser(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).SMTPUser)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn SetSMTPUser(&self, bstrsmtpuser: &windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetSMTPUser)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrsmtpuser)).ok() }
    }
    pub unsafe fn AllowedReceipts(&self) -> windows_core::Result<FAX_RECEIPT_TYPE_ENUM> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).AllowedReceipts)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetAllowedReceipts(&self, allowedreceipts: FAX_RECEIPT_TYPE_ENUM) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetAllowedReceipts)(windows_core::Interface::as_raw(self), allowedreceipts).ok() }
    }
    pub unsafe fn SMTPPassword(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).SMTPPassword)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn SetSMTPPassword(&self, bstrsmtppassword: &windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetSMTPPassword)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrsmtppassword)).ok() }
    }
    pub unsafe fn Refresh(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Refresh)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn Save(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Save)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn UseForInboundRouting(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).UseForInboundRouting)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetUseForInboundRouting(&self, buseforinboundrouting: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetUseForInboundRouting)(windows_core::Interface::as_raw(self), buseforinboundrouting).ok() }
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IFaxReceiptOptions_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub AuthenticationType: unsafe extern "system" fn(*mut core::ffi::c_void, *mut FAX_SMTP_AUTHENTICATION_TYPE_ENUM) -> windows_core::HRESULT,
    pub SetAuthenticationType: unsafe extern "system" fn(*mut core::ffi::c_void, FAX_SMTP_AUTHENTICATION_TYPE_ENUM) -> windows_core::HRESULT,
    pub SMTPServer: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetSMTPServer: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SMTPPort: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetSMTPPort: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub SMTPSender: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetSMTPSender: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SMTPUser: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetSMTPUser: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub AllowedReceipts: unsafe extern "system" fn(*mut core::ffi::c_void, *mut FAX_RECEIPT_TYPE_ENUM) -> windows_core::HRESULT,
    pub SetAllowedReceipts: unsafe extern "system" fn(*mut core::ffi::c_void, FAX_RECEIPT_TYPE_ENUM) -> windows_core::HRESULT,
    pub SMTPPassword: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetSMTPPassword: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Refresh: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Save: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub UseForInboundRouting: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub SetUseForInboundRouting: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IFaxReceiptOptions_Impl: super::super::System::Com::IDispatch_Impl {
    fn AuthenticationType(&self) -> windows_core::Result<FAX_SMTP_AUTHENTICATION_TYPE_ENUM>;
    fn SetAuthenticationType(&self, r#type: FAX_SMTP_AUTHENTICATION_TYPE_ENUM) -> windows_core::Result<()>;
    fn SMTPServer(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetSMTPServer(&self, bstrsmtpserver: &windows_core::BSTR) -> windows_core::Result<()>;
    fn SMTPPort(&self) -> windows_core::Result<i32>;
    fn SetSMTPPort(&self, lsmtpport: i32) -> windows_core::Result<()>;
    fn SMTPSender(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetSMTPSender(&self, bstrsmtpsender: &windows_core::BSTR) -> windows_core::Result<()>;
    fn SMTPUser(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetSMTPUser(&self, bstrsmtpuser: &windows_core::BSTR) -> windows_core::Result<()>;
    fn AllowedReceipts(&self) -> windows_core::Result<FAX_RECEIPT_TYPE_ENUM>;
    fn SetAllowedReceipts(&self, allowedreceipts: FAX_RECEIPT_TYPE_ENUM) -> windows_core::Result<()>;
    fn SMTPPassword(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetSMTPPassword(&self, bstrsmtppassword: &windows_core::BSTR) -> windows_core::Result<()>;
    fn Refresh(&self) -> windows_core::Result<()>;
    fn Save(&self) -> windows_core::Result<()>;
    fn UseForInboundRouting(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn SetUseForInboundRouting(&self, buseforinboundrouting: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IFaxReceiptOptions_Vtbl {
    pub const fn new<Identity: IFaxReceiptOptions_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn AuthenticationType<Identity: IFaxReceiptOptions_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ptype: *mut FAX_SMTP_AUTHENTICATION_TYPE_ENUM) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFaxReceiptOptions_Impl::AuthenticationType(this) {
                    Ok(ok__) => {
                        ptype.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetAuthenticationType<Identity: IFaxReceiptOptions_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, r#type: FAX_SMTP_AUTHENTICATION_TYPE_ENUM) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFaxReceiptOptions_Impl::SetAuthenticationType(this, core::mem::transmute_copy(&r#type)).into()
            }
        }
        unsafe extern "system" fn SMTPServer<Identity: IFaxReceiptOptions_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrsmtpserver: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFaxReceiptOptions_Impl::SMTPServer(this) {
                    Ok(ok__) => {
                        pbstrsmtpserver.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetSMTPServer<Identity: IFaxReceiptOptions_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrsmtpserver: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFaxReceiptOptions_Impl::SetSMTPServer(this, core::mem::transmute(&bstrsmtpserver)).into()
            }
        }
        unsafe extern "system" fn SMTPPort<Identity: IFaxReceiptOptions_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, plsmtpport: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFaxReceiptOptions_Impl::SMTPPort(this) {
                    Ok(ok__) => {
                        plsmtpport.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetSMTPPort<Identity: IFaxReceiptOptions_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lsmtpport: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFaxReceiptOptions_Impl::SetSMTPPort(this, core::mem::transmute_copy(&lsmtpport)).into()
            }
        }
        unsafe extern "system" fn SMTPSender<Identity: IFaxReceiptOptions_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrsmtpsender: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFaxReceiptOptions_Impl::SMTPSender(this) {
                    Ok(ok__) => {
                        pbstrsmtpsender.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetSMTPSender<Identity: IFaxReceiptOptions_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrsmtpsender: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFaxReceiptOptions_Impl::SetSMTPSender(this, core::mem::transmute(&bstrsmtpsender)).into()
            }
        }
        unsafe extern "system" fn SMTPUser<Identity: IFaxReceiptOptions_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrsmtpuser: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFaxReceiptOptions_Impl::SMTPUser(this) {
                    Ok(ok__) => {
                        pbstrsmtpuser.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetSMTPUser<Identity: IFaxReceiptOptions_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrsmtpuser: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFaxReceiptOptions_Impl::SetSMTPUser(this, core::mem::transmute(&bstrsmtpuser)).into()
            }
        }
        unsafe extern "system" fn AllowedReceipts<Identity: IFaxReceiptOptions_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pallowedreceipts: *mut FAX_RECEIPT_TYPE_ENUM) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFaxReceiptOptions_Impl::AllowedReceipts(this) {
                    Ok(ok__) => {
                        pallowedreceipts.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetAllowedReceipts<Identity: IFaxReceiptOptions_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, allowedreceipts: FAX_RECEIPT_TYPE_ENUM) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFaxReceiptOptions_Impl::SetAllowedReceipts(this, core::mem::transmute_copy(&allowedreceipts)).into()
            }
        }
        unsafe extern "system" fn SMTPPassword<Identity: IFaxReceiptOptions_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrsmtppassword: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFaxReceiptOptions_Impl::SMTPPassword(this) {
                    Ok(ok__) => {
                        pbstrsmtppassword.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetSMTPPassword<Identity: IFaxReceiptOptions_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrsmtppassword: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFaxReceiptOptions_Impl::SetSMTPPassword(this, core::mem::transmute(&bstrsmtppassword)).into()
            }
        }
        unsafe extern "system" fn Refresh<Identity: IFaxReceiptOptions_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFaxReceiptOptions_Impl::Refresh(this).into()
            }
        }
        unsafe extern "system" fn Save<Identity: IFaxReceiptOptions_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFaxReceiptOptions_Impl::Save(this).into()
            }
        }
        unsafe extern "system" fn UseForInboundRouting<Identity: IFaxReceiptOptions_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbuseforinboundrouting: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFaxReceiptOptions_Impl::UseForInboundRouting(this) {
                    Ok(ok__) => {
                        pbuseforinboundrouting.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetUseForInboundRouting<Identity: IFaxReceiptOptions_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, buseforinboundrouting: super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFaxReceiptOptions_Impl::SetUseForInboundRouting(this, core::mem::transmute_copy(&buseforinboundrouting)).into()
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            AuthenticationType: AuthenticationType::<Identity, OFFSET>,
            SetAuthenticationType: SetAuthenticationType::<Identity, OFFSET>,
            SMTPServer: SMTPServer::<Identity, OFFSET>,
            SetSMTPServer: SetSMTPServer::<Identity, OFFSET>,
            SMTPPort: SMTPPort::<Identity, OFFSET>,
            SetSMTPPort: SetSMTPPort::<Identity, OFFSET>,
            SMTPSender: SMTPSender::<Identity, OFFSET>,
            SetSMTPSender: SetSMTPSender::<Identity, OFFSET>,
            SMTPUser: SMTPUser::<Identity, OFFSET>,
            SetSMTPUser: SetSMTPUser::<Identity, OFFSET>,
            AllowedReceipts: AllowedReceipts::<Identity, OFFSET>,
            SetAllowedReceipts: SetAllowedReceipts::<Identity, OFFSET>,
            SMTPPassword: SMTPPassword::<Identity, OFFSET>,
            SetSMTPPassword: SetSMTPPassword::<Identity, OFFSET>,
            Refresh: Refresh::<Identity, OFFSET>,
            Save: Save::<Identity, OFFSET>,
            UseForInboundRouting: UseForInboundRouting::<Identity, OFFSET>,
            SetUseForInboundRouting: SetUseForInboundRouting::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IFaxReceiptOptions as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for IFaxReceiptOptions {}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IFaxRecipient, IFaxRecipient_Vtbl, 0x9a3da3a0_538d_42b6_9444_aaa57d0ce2bc);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IFaxRecipient {
    type Target = super::super::System::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IFaxRecipient, windows_core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl IFaxRecipient {
    pub unsafe fn FaxNumber(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).FaxNumber)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn SetFaxNumber(&self, bstrfaxnumber: &windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetFaxNumber)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrfaxnumber)).ok() }
    }
    pub unsafe fn Name(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Name)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn SetName(&self, bstrname: &windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetName)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrname)).ok() }
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IFaxRecipient_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub FaxNumber: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetFaxNumber: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Name: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IFaxRecipient_Impl: super::super::System::Com::IDispatch_Impl {
    fn FaxNumber(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetFaxNumber(&self, bstrfaxnumber: &windows_core::BSTR) -> windows_core::Result<()>;
    fn Name(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetName(&self, bstrname: &windows_core::BSTR) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IFaxRecipient_Vtbl {
    pub const fn new<Identity: IFaxRecipient_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn FaxNumber<Identity: IFaxRecipient_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrfaxnumber: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFaxRecipient_Impl::FaxNumber(this) {
                    Ok(ok__) => {
                        pbstrfaxnumber.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetFaxNumber<Identity: IFaxRecipient_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrfaxnumber: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFaxRecipient_Impl::SetFaxNumber(this, core::mem::transmute(&bstrfaxnumber)).into()
            }
        }
        unsafe extern "system" fn Name<Identity: IFaxRecipient_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrname: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFaxRecipient_Impl::Name(this) {
                    Ok(ok__) => {
                        pbstrname.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetName<Identity: IFaxRecipient_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrname: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFaxRecipient_Impl::SetName(this, core::mem::transmute(&bstrname)).into()
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            FaxNumber: FaxNumber::<Identity, OFFSET>,
            SetFaxNumber: SetFaxNumber::<Identity, OFFSET>,
            Name: Name::<Identity, OFFSET>,
            SetName: SetName::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IFaxRecipient as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for IFaxRecipient {}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IFaxRecipients, IFaxRecipients_Vtbl, 0xb9c9de5a_894e_4492_9fa3_08c627c11d5d);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IFaxRecipients {
    type Target = super::super::System::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IFaxRecipients, windows_core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl IFaxRecipients {
    pub unsafe fn _NewEnum(&self) -> windows_core::Result<windows_core::IUnknown> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self)._NewEnum)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn get_Item(&self, lindex: i32) -> windows_core::Result<IFaxRecipient> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_Item)(windows_core::Interface::as_raw(self), lindex, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn Count(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Count)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn Add(&self, bstrfaxnumber: &windows_core::BSTR, bstrrecipientname: &windows_core::BSTR) -> windows_core::Result<IFaxRecipient> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Add)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrfaxnumber), core::mem::transmute_copy(bstrrecipientname), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn Remove(&self, lindex: i32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Remove)(windows_core::Interface::as_raw(self), lindex).ok() }
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IFaxRecipients_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub _NewEnum: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub get_Item: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Count: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub Add: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Remove: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IFaxRecipients_Impl: super::super::System::Com::IDispatch_Impl {
    fn _NewEnum(&self) -> windows_core::Result<windows_core::IUnknown>;
    fn get_Item(&self, lindex: i32) -> windows_core::Result<IFaxRecipient>;
    fn Count(&self) -> windows_core::Result<i32>;
    fn Add(&self, bstrfaxnumber: &windows_core::BSTR, bstrrecipientname: &windows_core::BSTR) -> windows_core::Result<IFaxRecipient>;
    fn Remove(&self, lindex: i32) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IFaxRecipients_Vtbl {
    pub const fn new<Identity: IFaxRecipients_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn _NewEnum<Identity: IFaxRecipients_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppunk: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFaxRecipients_Impl::_NewEnum(this) {
                    Ok(ok__) => {
                        ppunk.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_Item<Identity: IFaxRecipients_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lindex: i32, ppfaxrecipient: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFaxRecipients_Impl::get_Item(this, core::mem::transmute_copy(&lindex)) {
                    Ok(ok__) => {
                        ppfaxrecipient.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Count<Identity: IFaxRecipients_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, plcount: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFaxRecipients_Impl::Count(this) {
                    Ok(ok__) => {
                        plcount.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Add<Identity: IFaxRecipients_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrfaxnumber: *mut core::ffi::c_void, bstrrecipientname: *mut core::ffi::c_void, ppfaxrecipient: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFaxRecipients_Impl::Add(this, core::mem::transmute(&bstrfaxnumber), core::mem::transmute(&bstrrecipientname)) {
                    Ok(ok__) => {
                        ppfaxrecipient.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Remove<Identity: IFaxRecipients_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lindex: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFaxRecipients_Impl::Remove(this, core::mem::transmute_copy(&lindex)).into()
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            _NewEnum: _NewEnum::<Identity, OFFSET>,
            get_Item: get_Item::<Identity, OFFSET>,
            Count: Count::<Identity, OFFSET>,
            Add: Add::<Identity, OFFSET>,
            Remove: Remove::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IFaxRecipients as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for IFaxRecipients {}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IFaxSecurity, IFaxSecurity_Vtbl, 0x77b508c1_09c0_47a2_91eb_fce7fdf2690e);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IFaxSecurity {
    type Target = super::super::System::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IFaxSecurity, windows_core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl IFaxSecurity {
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn Descriptor(&self) -> windows_core::Result<super::super::System::Variant::VARIANT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Descriptor)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn SetDescriptor(&self, vdescriptor: &super::super::System::Variant::VARIANT) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetDescriptor)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(vdescriptor)).ok() }
    }
    pub unsafe fn GrantedRights(&self) -> windows_core::Result<FAX_ACCESS_RIGHTS_ENUM> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GrantedRights)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn Refresh(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Refresh)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn Save(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Save)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn InformationType(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).InformationType)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetInformationType(&self, linformationtype: i32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetInformationType)(windows_core::Interface::as_raw(self), linformationtype).ok() }
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IFaxSecurity_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub Descriptor: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::System::Variant::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    Descriptor: usize,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub SetDescriptor: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::System::Variant::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    SetDescriptor: usize,
    pub GrantedRights: unsafe extern "system" fn(*mut core::ffi::c_void, *mut FAX_ACCESS_RIGHTS_ENUM) -> windows_core::HRESULT,
    pub Refresh: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Save: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub InformationType: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetInformationType: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IFaxSecurity_Impl: super::super::System::Com::IDispatch_Impl {
    fn Descriptor(&self) -> windows_core::Result<super::super::System::Variant::VARIANT>;
    fn SetDescriptor(&self, vdescriptor: &super::super::System::Variant::VARIANT) -> windows_core::Result<()>;
    fn GrantedRights(&self) -> windows_core::Result<FAX_ACCESS_RIGHTS_ENUM>;
    fn Refresh(&self) -> windows_core::Result<()>;
    fn Save(&self) -> windows_core::Result<()>;
    fn InformationType(&self) -> windows_core::Result<i32>;
    fn SetInformationType(&self, linformationtype: i32) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IFaxSecurity_Vtbl {
    pub const fn new<Identity: IFaxSecurity_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Descriptor<Identity: IFaxSecurity_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvdescriptor: *mut super::super::System::Variant::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFaxSecurity_Impl::Descriptor(this) {
                    Ok(ok__) => {
                        pvdescriptor.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetDescriptor<Identity: IFaxSecurity_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, vdescriptor: super::super::System::Variant::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFaxSecurity_Impl::SetDescriptor(this, core::mem::transmute(&vdescriptor)).into()
            }
        }
        unsafe extern "system" fn GrantedRights<Identity: IFaxSecurity_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pgrantedrights: *mut FAX_ACCESS_RIGHTS_ENUM) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFaxSecurity_Impl::GrantedRights(this) {
                    Ok(ok__) => {
                        pgrantedrights.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Refresh<Identity: IFaxSecurity_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFaxSecurity_Impl::Refresh(this).into()
            }
        }
        unsafe extern "system" fn Save<Identity: IFaxSecurity_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFaxSecurity_Impl::Save(this).into()
            }
        }
        unsafe extern "system" fn InformationType<Identity: IFaxSecurity_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, plinformationtype: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFaxSecurity_Impl::InformationType(this) {
                    Ok(ok__) => {
                        plinformationtype.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetInformationType<Identity: IFaxSecurity_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, linformationtype: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFaxSecurity_Impl::SetInformationType(this, core::mem::transmute_copy(&linformationtype)).into()
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            Descriptor: Descriptor::<Identity, OFFSET>,
            SetDescriptor: SetDescriptor::<Identity, OFFSET>,
            GrantedRights: GrantedRights::<Identity, OFFSET>,
            Refresh: Refresh::<Identity, OFFSET>,
            Save: Save::<Identity, OFFSET>,
            InformationType: InformationType::<Identity, OFFSET>,
            SetInformationType: SetInformationType::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IFaxSecurity as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for IFaxSecurity {}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IFaxSecurity2, IFaxSecurity2_Vtbl, 0x17d851f4_d09b_48fc_99c9_8f24c4db9ab1);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IFaxSecurity2 {
    type Target = super::super::System::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IFaxSecurity2, windows_core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl IFaxSecurity2 {
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn Descriptor(&self) -> windows_core::Result<super::super::System::Variant::VARIANT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Descriptor)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn SetDescriptor(&self, vdescriptor: &super::super::System::Variant::VARIANT) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetDescriptor)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(vdescriptor)).ok() }
    }
    pub unsafe fn GrantedRights(&self) -> windows_core::Result<FAX_ACCESS_RIGHTS_ENUM_2> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GrantedRights)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn Refresh(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Refresh)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn Save(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Save)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn InformationType(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).InformationType)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetInformationType(&self, linformationtype: i32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetInformationType)(windows_core::Interface::as_raw(self), linformationtype).ok() }
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IFaxSecurity2_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub Descriptor: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::System::Variant::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    Descriptor: usize,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub SetDescriptor: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::System::Variant::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    SetDescriptor: usize,
    pub GrantedRights: unsafe extern "system" fn(*mut core::ffi::c_void, *mut FAX_ACCESS_RIGHTS_ENUM_2) -> windows_core::HRESULT,
    pub Refresh: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Save: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub InformationType: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetInformationType: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IFaxSecurity2_Impl: super::super::System::Com::IDispatch_Impl {
    fn Descriptor(&self) -> windows_core::Result<super::super::System::Variant::VARIANT>;
    fn SetDescriptor(&self, vdescriptor: &super::super::System::Variant::VARIANT) -> windows_core::Result<()>;
    fn GrantedRights(&self) -> windows_core::Result<FAX_ACCESS_RIGHTS_ENUM_2>;
    fn Refresh(&self) -> windows_core::Result<()>;
    fn Save(&self) -> windows_core::Result<()>;
    fn InformationType(&self) -> windows_core::Result<i32>;
    fn SetInformationType(&self, linformationtype: i32) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IFaxSecurity2_Vtbl {
    pub const fn new<Identity: IFaxSecurity2_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Descriptor<Identity: IFaxSecurity2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvdescriptor: *mut super::super::System::Variant::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFaxSecurity2_Impl::Descriptor(this) {
                    Ok(ok__) => {
                        pvdescriptor.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetDescriptor<Identity: IFaxSecurity2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, vdescriptor: super::super::System::Variant::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFaxSecurity2_Impl::SetDescriptor(this, core::mem::transmute(&vdescriptor)).into()
            }
        }
        unsafe extern "system" fn GrantedRights<Identity: IFaxSecurity2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pgrantedrights: *mut FAX_ACCESS_RIGHTS_ENUM_2) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFaxSecurity2_Impl::GrantedRights(this) {
                    Ok(ok__) => {
                        pgrantedrights.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Refresh<Identity: IFaxSecurity2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFaxSecurity2_Impl::Refresh(this).into()
            }
        }
        unsafe extern "system" fn Save<Identity: IFaxSecurity2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFaxSecurity2_Impl::Save(this).into()
            }
        }
        unsafe extern "system" fn InformationType<Identity: IFaxSecurity2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, plinformationtype: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFaxSecurity2_Impl::InformationType(this) {
                    Ok(ok__) => {
                        plinformationtype.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetInformationType<Identity: IFaxSecurity2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, linformationtype: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFaxSecurity2_Impl::SetInformationType(this, core::mem::transmute_copy(&linformationtype)).into()
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            Descriptor: Descriptor::<Identity, OFFSET>,
            SetDescriptor: SetDescriptor::<Identity, OFFSET>,
            GrantedRights: GrantedRights::<Identity, OFFSET>,
            Refresh: Refresh::<Identity, OFFSET>,
            Save: Save::<Identity, OFFSET>,
            InformationType: InformationType::<Identity, OFFSET>,
            SetInformationType: SetInformationType::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IFaxSecurity2 as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for IFaxSecurity2 {}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IFaxSender, IFaxSender_Vtbl, 0x0d879d7d_f57a_4cc6_a6f9_3ee5d527b46a);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IFaxSender {
    type Target = super::super::System::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IFaxSender, windows_core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl IFaxSender {
    pub unsafe fn BillingCode(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).BillingCode)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn SetBillingCode(&self, bstrbillingcode: &windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetBillingCode)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrbillingcode)).ok() }
    }
    pub unsafe fn City(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).City)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn SetCity(&self, bstrcity: &windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetCity)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrcity)).ok() }
    }
    pub unsafe fn Company(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Company)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn SetCompany(&self, bstrcompany: &windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetCompany)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrcompany)).ok() }
    }
    pub unsafe fn Country(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Country)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn SetCountry(&self, bstrcountry: &windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetCountry)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrcountry)).ok() }
    }
    pub unsafe fn Department(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Department)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn SetDepartment(&self, bstrdepartment: &windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetDepartment)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrdepartment)).ok() }
    }
    pub unsafe fn Email(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Email)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn SetEmail(&self, bstremail: &windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetEmail)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstremail)).ok() }
    }
    pub unsafe fn FaxNumber(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).FaxNumber)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn SetFaxNumber(&self, bstrfaxnumber: &windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetFaxNumber)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrfaxnumber)).ok() }
    }
    pub unsafe fn HomePhone(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).HomePhone)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn SetHomePhone(&self, bstrhomephone: &windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetHomePhone)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrhomephone)).ok() }
    }
    pub unsafe fn Name(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Name)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn SetName(&self, bstrname: &windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetName)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrname)).ok() }
    }
    pub unsafe fn TSID(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).TSID)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn SetTSID(&self, bstrtsid: &windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetTSID)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrtsid)).ok() }
    }
    pub unsafe fn OfficePhone(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).OfficePhone)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn SetOfficePhone(&self, bstrofficephone: &windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetOfficePhone)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrofficephone)).ok() }
    }
    pub unsafe fn OfficeLocation(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).OfficeLocation)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn SetOfficeLocation(&self, bstrofficelocation: &windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetOfficeLocation)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrofficelocation)).ok() }
    }
    pub unsafe fn State(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).State)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn SetState(&self, bstrstate: &windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetState)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrstate)).ok() }
    }
    pub unsafe fn StreetAddress(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).StreetAddress)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn SetStreetAddress(&self, bstrstreetaddress: &windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetStreetAddress)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrstreetaddress)).ok() }
    }
    pub unsafe fn Title(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Title)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn SetTitle(&self, bstrtitle: &windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetTitle)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrtitle)).ok() }
    }
    pub unsafe fn ZipCode(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ZipCode)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn SetZipCode(&self, bstrzipcode: &windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetZipCode)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrzipcode)).ok() }
    }
    pub unsafe fn LoadDefaultSender(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).LoadDefaultSender)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn SaveDefaultSender(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SaveDefaultSender)(windows_core::Interface::as_raw(self)).ok() }
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IFaxSender_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub BillingCode: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetBillingCode: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub City: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetCity: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Company: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetCompany: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Country: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetCountry: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Department: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetDepartment: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Email: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetEmail: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub FaxNumber: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetFaxNumber: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub HomePhone: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetHomePhone: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Name: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub TSID: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetTSID: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub OfficePhone: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetOfficePhone: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub OfficeLocation: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetOfficeLocation: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub State: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetState: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub StreetAddress: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetStreetAddress: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Title: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetTitle: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ZipCode: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetZipCode: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub LoadDefaultSender: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SaveDefaultSender: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IFaxSender_Impl: super::super::System::Com::IDispatch_Impl {
    fn BillingCode(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetBillingCode(&self, bstrbillingcode: &windows_core::BSTR) -> windows_core::Result<()>;
    fn City(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetCity(&self, bstrcity: &windows_core::BSTR) -> windows_core::Result<()>;
    fn Company(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetCompany(&self, bstrcompany: &windows_core::BSTR) -> windows_core::Result<()>;
    fn Country(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetCountry(&self, bstrcountry: &windows_core::BSTR) -> windows_core::Result<()>;
    fn Department(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetDepartment(&self, bstrdepartment: &windows_core::BSTR) -> windows_core::Result<()>;
    fn Email(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetEmail(&self, bstremail: &windows_core::BSTR) -> windows_core::Result<()>;
    fn FaxNumber(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetFaxNumber(&self, bstrfaxnumber: &windows_core::BSTR) -> windows_core::Result<()>;
    fn HomePhone(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetHomePhone(&self, bstrhomephone: &windows_core::BSTR) -> windows_core::Result<()>;
    fn Name(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetName(&self, bstrname: &windows_core::BSTR) -> windows_core::Result<()>;
    fn TSID(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetTSID(&self, bstrtsid: &windows_core::BSTR) -> windows_core::Result<()>;
    fn OfficePhone(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetOfficePhone(&self, bstrofficephone: &windows_core::BSTR) -> windows_core::Result<()>;
    fn OfficeLocation(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetOfficeLocation(&self, bstrofficelocation: &windows_core::BSTR) -> windows_core::Result<()>;
    fn State(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetState(&self, bstrstate: &windows_core::BSTR) -> windows_core::Result<()>;
    fn StreetAddress(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetStreetAddress(&self, bstrstreetaddress: &windows_core::BSTR) -> windows_core::Result<()>;
    fn Title(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetTitle(&self, bstrtitle: &windows_core::BSTR) -> windows_core::Result<()>;
    fn ZipCode(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetZipCode(&self, bstrzipcode: &windows_core::BSTR) -> windows_core::Result<()>;
    fn LoadDefaultSender(&self) -> windows_core::Result<()>;
    fn SaveDefaultSender(&self) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IFaxSender_Vtbl {
    pub const fn new<Identity: IFaxSender_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn BillingCode<Identity: IFaxSender_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrbillingcode: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFaxSender_Impl::BillingCode(this) {
                    Ok(ok__) => {
                        pbstrbillingcode.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetBillingCode<Identity: IFaxSender_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrbillingcode: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFaxSender_Impl::SetBillingCode(this, core::mem::transmute(&bstrbillingcode)).into()
            }
        }
        unsafe extern "system" fn City<Identity: IFaxSender_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrcity: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFaxSender_Impl::City(this) {
                    Ok(ok__) => {
                        pbstrcity.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetCity<Identity: IFaxSender_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrcity: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFaxSender_Impl::SetCity(this, core::mem::transmute(&bstrcity)).into()
            }
        }
        unsafe extern "system" fn Company<Identity: IFaxSender_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrcompany: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFaxSender_Impl::Company(this) {
                    Ok(ok__) => {
                        pbstrcompany.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetCompany<Identity: IFaxSender_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrcompany: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFaxSender_Impl::SetCompany(this, core::mem::transmute(&bstrcompany)).into()
            }
        }
        unsafe extern "system" fn Country<Identity: IFaxSender_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrcountry: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFaxSender_Impl::Country(this) {
                    Ok(ok__) => {
                        pbstrcountry.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetCountry<Identity: IFaxSender_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrcountry: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFaxSender_Impl::SetCountry(this, core::mem::transmute(&bstrcountry)).into()
            }
        }
        unsafe extern "system" fn Department<Identity: IFaxSender_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrdepartment: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFaxSender_Impl::Department(this) {
                    Ok(ok__) => {
                        pbstrdepartment.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetDepartment<Identity: IFaxSender_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrdepartment: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFaxSender_Impl::SetDepartment(this, core::mem::transmute(&bstrdepartment)).into()
            }
        }
        unsafe extern "system" fn Email<Identity: IFaxSender_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstremail: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFaxSender_Impl::Email(this) {
                    Ok(ok__) => {
                        pbstremail.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetEmail<Identity: IFaxSender_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstremail: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFaxSender_Impl::SetEmail(this, core::mem::transmute(&bstremail)).into()
            }
        }
        unsafe extern "system" fn FaxNumber<Identity: IFaxSender_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrfaxnumber: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFaxSender_Impl::FaxNumber(this) {
                    Ok(ok__) => {
                        pbstrfaxnumber.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetFaxNumber<Identity: IFaxSender_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrfaxnumber: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFaxSender_Impl::SetFaxNumber(this, core::mem::transmute(&bstrfaxnumber)).into()
            }
        }
        unsafe extern "system" fn HomePhone<Identity: IFaxSender_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrhomephone: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFaxSender_Impl::HomePhone(this) {
                    Ok(ok__) => {
                        pbstrhomephone.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetHomePhone<Identity: IFaxSender_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrhomephone: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFaxSender_Impl::SetHomePhone(this, core::mem::transmute(&bstrhomephone)).into()
            }
        }
        unsafe extern "system" fn Name<Identity: IFaxSender_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrname: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFaxSender_Impl::Name(this) {
                    Ok(ok__) => {
                        pbstrname.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetName<Identity: IFaxSender_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrname: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFaxSender_Impl::SetName(this, core::mem::transmute(&bstrname)).into()
            }
        }
        unsafe extern "system" fn TSID<Identity: IFaxSender_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrtsid: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFaxSender_Impl::TSID(this) {
                    Ok(ok__) => {
                        pbstrtsid.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetTSID<Identity: IFaxSender_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrtsid: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFaxSender_Impl::SetTSID(this, core::mem::transmute(&bstrtsid)).into()
            }
        }
        unsafe extern "system" fn OfficePhone<Identity: IFaxSender_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrofficephone: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFaxSender_Impl::OfficePhone(this) {
                    Ok(ok__) => {
                        pbstrofficephone.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetOfficePhone<Identity: IFaxSender_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrofficephone: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFaxSender_Impl::SetOfficePhone(this, core::mem::transmute(&bstrofficephone)).into()
            }
        }
        unsafe extern "system" fn OfficeLocation<Identity: IFaxSender_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrofficelocation: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFaxSender_Impl::OfficeLocation(this) {
                    Ok(ok__) => {
                        pbstrofficelocation.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetOfficeLocation<Identity: IFaxSender_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrofficelocation: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFaxSender_Impl::SetOfficeLocation(this, core::mem::transmute(&bstrofficelocation)).into()
            }
        }
        unsafe extern "system" fn State<Identity: IFaxSender_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrstate: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFaxSender_Impl::State(this) {
                    Ok(ok__) => {
                        pbstrstate.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetState<Identity: IFaxSender_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrstate: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFaxSender_Impl::SetState(this, core::mem::transmute(&bstrstate)).into()
            }
        }
        unsafe extern "system" fn StreetAddress<Identity: IFaxSender_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrstreetaddress: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFaxSender_Impl::StreetAddress(this) {
                    Ok(ok__) => {
                        pbstrstreetaddress.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetStreetAddress<Identity: IFaxSender_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrstreetaddress: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFaxSender_Impl::SetStreetAddress(this, core::mem::transmute(&bstrstreetaddress)).into()
            }
        }
        unsafe extern "system" fn Title<Identity: IFaxSender_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrtitle: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFaxSender_Impl::Title(this) {
                    Ok(ok__) => {
                        pbstrtitle.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetTitle<Identity: IFaxSender_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrtitle: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFaxSender_Impl::SetTitle(this, core::mem::transmute(&bstrtitle)).into()
            }
        }
        unsafe extern "system" fn ZipCode<Identity: IFaxSender_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrzipcode: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFaxSender_Impl::ZipCode(this) {
                    Ok(ok__) => {
                        pbstrzipcode.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetZipCode<Identity: IFaxSender_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrzipcode: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFaxSender_Impl::SetZipCode(this, core::mem::transmute(&bstrzipcode)).into()
            }
        }
        unsafe extern "system" fn LoadDefaultSender<Identity: IFaxSender_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFaxSender_Impl::LoadDefaultSender(this).into()
            }
        }
        unsafe extern "system" fn SaveDefaultSender<Identity: IFaxSender_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFaxSender_Impl::SaveDefaultSender(this).into()
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            BillingCode: BillingCode::<Identity, OFFSET>,
            SetBillingCode: SetBillingCode::<Identity, OFFSET>,
            City: City::<Identity, OFFSET>,
            SetCity: SetCity::<Identity, OFFSET>,
            Company: Company::<Identity, OFFSET>,
            SetCompany: SetCompany::<Identity, OFFSET>,
            Country: Country::<Identity, OFFSET>,
            SetCountry: SetCountry::<Identity, OFFSET>,
            Department: Department::<Identity, OFFSET>,
            SetDepartment: SetDepartment::<Identity, OFFSET>,
            Email: Email::<Identity, OFFSET>,
            SetEmail: SetEmail::<Identity, OFFSET>,
            FaxNumber: FaxNumber::<Identity, OFFSET>,
            SetFaxNumber: SetFaxNumber::<Identity, OFFSET>,
            HomePhone: HomePhone::<Identity, OFFSET>,
            SetHomePhone: SetHomePhone::<Identity, OFFSET>,
            Name: Name::<Identity, OFFSET>,
            SetName: SetName::<Identity, OFFSET>,
            TSID: TSID::<Identity, OFFSET>,
            SetTSID: SetTSID::<Identity, OFFSET>,
            OfficePhone: OfficePhone::<Identity, OFFSET>,
            SetOfficePhone: SetOfficePhone::<Identity, OFFSET>,
            OfficeLocation: OfficeLocation::<Identity, OFFSET>,
            SetOfficeLocation: SetOfficeLocation::<Identity, OFFSET>,
            State: State::<Identity, OFFSET>,
            SetState: SetState::<Identity, OFFSET>,
            StreetAddress: StreetAddress::<Identity, OFFSET>,
            SetStreetAddress: SetStreetAddress::<Identity, OFFSET>,
            Title: Title::<Identity, OFFSET>,
            SetTitle: SetTitle::<Identity, OFFSET>,
            ZipCode: ZipCode::<Identity, OFFSET>,
            SetZipCode: SetZipCode::<Identity, OFFSET>,
            LoadDefaultSender: LoadDefaultSender::<Identity, OFFSET>,
            SaveDefaultSender: SaveDefaultSender::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IFaxSender as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for IFaxSender {}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IFaxServer, IFaxServer_Vtbl, 0x475b6469_90a5_4878_a577_17a86e8e3462);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IFaxServer {
    type Target = super::super::System::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IFaxServer, windows_core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl IFaxServer {
    pub unsafe fn Connect(&self, bstrservername: &windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Connect)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrservername)).ok() }
    }
    pub unsafe fn ServerName(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ServerName)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn GetDeviceProviders(&self) -> windows_core::Result<IFaxDeviceProviders> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetDeviceProviders)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetDevices(&self) -> windows_core::Result<IFaxDevices> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetDevices)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn InboundRouting(&self) -> windows_core::Result<IFaxInboundRouting> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).InboundRouting)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn Folders(&self) -> windows_core::Result<IFaxFolders> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Folders)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn LoggingOptions(&self) -> windows_core::Result<IFaxLoggingOptions> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).LoggingOptions)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn MajorVersion(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).MajorVersion)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn MinorVersion(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).MinorVersion)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn MajorBuild(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).MajorBuild)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn MinorBuild(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).MinorBuild)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn Debug(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Debug)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn Activity(&self) -> windows_core::Result<IFaxActivity> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Activity)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn OutboundRouting(&self) -> windows_core::Result<IFaxOutboundRouting> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).OutboundRouting)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn ReceiptOptions(&self) -> windows_core::Result<IFaxReceiptOptions> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ReceiptOptions)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn Security(&self) -> windows_core::Result<IFaxSecurity> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Security)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn Disconnect(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Disconnect)(windows_core::Interface::as_raw(self)).ok() }
    }
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn GetExtensionProperty(&self, bstrguid: &windows_core::BSTR) -> windows_core::Result<super::super::System::Variant::VARIANT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetExtensionProperty)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrguid), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn SetExtensionProperty(&self, bstrguid: &windows_core::BSTR, vproperty: &super::super::System::Variant::VARIANT) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetExtensionProperty)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrguid), core::mem::transmute_copy(vproperty)).ok() }
    }
    pub unsafe fn ListenToServerEvents(&self, eventtypes: FAX_SERVER_EVENTS_TYPE_ENUM) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).ListenToServerEvents)(windows_core::Interface::as_raw(self), eventtypes).ok() }
    }
    pub unsafe fn RegisterDeviceProvider(&self, bstrguid: &windows_core::BSTR, bstrfriendlyname: &windows_core::BSTR, bstrimagename: &windows_core::BSTR, tspname: &windows_core::BSTR, lfspiversion: i32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).RegisterDeviceProvider)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrguid), core::mem::transmute_copy(bstrfriendlyname), core::mem::transmute_copy(bstrimagename), core::mem::transmute_copy(tspname), lfspiversion).ok() }
    }
    pub unsafe fn UnregisterDeviceProvider(&self, bstruniquename: &windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).UnregisterDeviceProvider)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstruniquename)).ok() }
    }
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn RegisterInboundRoutingExtension(&self, bstrextensionname: &windows_core::BSTR, bstrfriendlyname: &windows_core::BSTR, bstrimagename: &windows_core::BSTR, vmethods: &super::super::System::Variant::VARIANT) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).RegisterInboundRoutingExtension)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrextensionname), core::mem::transmute_copy(bstrfriendlyname), core::mem::transmute_copy(bstrimagename), core::mem::transmute_copy(vmethods)).ok() }
    }
    pub unsafe fn UnregisterInboundRoutingExtension(&self, bstrextensionuniquename: &windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).UnregisterInboundRoutingExtension)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrextensionuniquename)).ok() }
    }
    pub unsafe fn RegisteredEvents(&self) -> windows_core::Result<FAX_SERVER_EVENTS_TYPE_ENUM> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).RegisteredEvents)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn APIVersion(&self) -> windows_core::Result<FAX_SERVER_APIVERSION_ENUM> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).APIVersion)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IFaxServer_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub Connect: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ServerName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetDeviceProviders: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetDevices: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub InboundRouting: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Folders: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub LoggingOptions: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub MajorVersion: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub MinorVersion: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub MajorBuild: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub MinorBuild: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub Debug: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub Activity: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub OutboundRouting: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ReceiptOptions: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Security: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Disconnect: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub GetExtensionProperty: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::System::Variant::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    GetExtensionProperty: usize,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub SetExtensionProperty: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, super::super::System::Variant::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    SetExtensionProperty: usize,
    pub ListenToServerEvents: unsafe extern "system" fn(*mut core::ffi::c_void, FAX_SERVER_EVENTS_TYPE_ENUM) -> windows_core::HRESULT,
    pub RegisterDeviceProvider: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub UnregisterDeviceProvider: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub RegisterInboundRoutingExtension: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, super::super::System::Variant::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    RegisterInboundRoutingExtension: usize,
    pub UnregisterInboundRoutingExtension: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub RegisteredEvents: unsafe extern "system" fn(*mut core::ffi::c_void, *mut FAX_SERVER_EVENTS_TYPE_ENUM) -> windows_core::HRESULT,
    pub APIVersion: unsafe extern "system" fn(*mut core::ffi::c_void, *mut FAX_SERVER_APIVERSION_ENUM) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IFaxServer_Impl: super::super::System::Com::IDispatch_Impl {
    fn Connect(&self, bstrservername: &windows_core::BSTR) -> windows_core::Result<()>;
    fn ServerName(&self) -> windows_core::Result<windows_core::BSTR>;
    fn GetDeviceProviders(&self) -> windows_core::Result<IFaxDeviceProviders>;
    fn GetDevices(&self) -> windows_core::Result<IFaxDevices>;
    fn InboundRouting(&self) -> windows_core::Result<IFaxInboundRouting>;
    fn Folders(&self) -> windows_core::Result<IFaxFolders>;
    fn LoggingOptions(&self) -> windows_core::Result<IFaxLoggingOptions>;
    fn MajorVersion(&self) -> windows_core::Result<i32>;
    fn MinorVersion(&self) -> windows_core::Result<i32>;
    fn MajorBuild(&self) -> windows_core::Result<i32>;
    fn MinorBuild(&self) -> windows_core::Result<i32>;
    fn Debug(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn Activity(&self) -> windows_core::Result<IFaxActivity>;
    fn OutboundRouting(&self) -> windows_core::Result<IFaxOutboundRouting>;
    fn ReceiptOptions(&self) -> windows_core::Result<IFaxReceiptOptions>;
    fn Security(&self) -> windows_core::Result<IFaxSecurity>;
    fn Disconnect(&self) -> windows_core::Result<()>;
    fn GetExtensionProperty(&self, bstrguid: &windows_core::BSTR) -> windows_core::Result<super::super::System::Variant::VARIANT>;
    fn SetExtensionProperty(&self, bstrguid: &windows_core::BSTR, vproperty: &super::super::System::Variant::VARIANT) -> windows_core::Result<()>;
    fn ListenToServerEvents(&self, eventtypes: FAX_SERVER_EVENTS_TYPE_ENUM) -> windows_core::Result<()>;
    fn RegisterDeviceProvider(&self, bstrguid: &windows_core::BSTR, bstrfriendlyname: &windows_core::BSTR, bstrimagename: &windows_core::BSTR, tspname: &windows_core::BSTR, lfspiversion: i32) -> windows_core::Result<()>;
    fn UnregisterDeviceProvider(&self, bstruniquename: &windows_core::BSTR) -> windows_core::Result<()>;
    fn RegisterInboundRoutingExtension(&self, bstrextensionname: &windows_core::BSTR, bstrfriendlyname: &windows_core::BSTR, bstrimagename: &windows_core::BSTR, vmethods: &super::super::System::Variant::VARIANT) -> windows_core::Result<()>;
    fn UnregisterInboundRoutingExtension(&self, bstrextensionuniquename: &windows_core::BSTR) -> windows_core::Result<()>;
    fn RegisteredEvents(&self) -> windows_core::Result<FAX_SERVER_EVENTS_TYPE_ENUM>;
    fn APIVersion(&self) -> windows_core::Result<FAX_SERVER_APIVERSION_ENUM>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IFaxServer_Vtbl {
    pub const fn new<Identity: IFaxServer_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Connect<Identity: IFaxServer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrservername: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFaxServer_Impl::Connect(this, core::mem::transmute(&bstrservername)).into()
            }
        }
        unsafe extern "system" fn ServerName<Identity: IFaxServer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrservername: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFaxServer_Impl::ServerName(this) {
                    Ok(ok__) => {
                        pbstrservername.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetDeviceProviders<Identity: IFaxServer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppfaxdeviceproviders: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFaxServer_Impl::GetDeviceProviders(this) {
                    Ok(ok__) => {
                        ppfaxdeviceproviders.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetDevices<Identity: IFaxServer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppfaxdevices: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFaxServer_Impl::GetDevices(this) {
                    Ok(ok__) => {
                        ppfaxdevices.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn InboundRouting<Identity: IFaxServer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppfaxinboundrouting: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFaxServer_Impl::InboundRouting(this) {
                    Ok(ok__) => {
                        ppfaxinboundrouting.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Folders<Identity: IFaxServer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pfaxfolders: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFaxServer_Impl::Folders(this) {
                    Ok(ok__) => {
                        pfaxfolders.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn LoggingOptions<Identity: IFaxServer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppfaxloggingoptions: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFaxServer_Impl::LoggingOptions(this) {
                    Ok(ok__) => {
                        ppfaxloggingoptions.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn MajorVersion<Identity: IFaxServer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, plmajorversion: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFaxServer_Impl::MajorVersion(this) {
                    Ok(ok__) => {
                        plmajorversion.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn MinorVersion<Identity: IFaxServer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, plminorversion: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFaxServer_Impl::MinorVersion(this) {
                    Ok(ok__) => {
                        plminorversion.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn MajorBuild<Identity: IFaxServer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, plmajorbuild: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFaxServer_Impl::MajorBuild(this) {
                    Ok(ok__) => {
                        plmajorbuild.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn MinorBuild<Identity: IFaxServer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, plminorbuild: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFaxServer_Impl::MinorBuild(this) {
                    Ok(ok__) => {
                        plminorbuild.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Debug<Identity: IFaxServer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbdebug: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFaxServer_Impl::Debug(this) {
                    Ok(ok__) => {
                        pbdebug.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Activity<Identity: IFaxServer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppfaxactivity: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFaxServer_Impl::Activity(this) {
                    Ok(ok__) => {
                        ppfaxactivity.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn OutboundRouting<Identity: IFaxServer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppfaxoutboundrouting: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFaxServer_Impl::OutboundRouting(this) {
                    Ok(ok__) => {
                        ppfaxoutboundrouting.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn ReceiptOptions<Identity: IFaxServer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppfaxreceiptoptions: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFaxServer_Impl::ReceiptOptions(this) {
                    Ok(ok__) => {
                        ppfaxreceiptoptions.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Security<Identity: IFaxServer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppfaxsecurity: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFaxServer_Impl::Security(this) {
                    Ok(ok__) => {
                        ppfaxsecurity.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Disconnect<Identity: IFaxServer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFaxServer_Impl::Disconnect(this).into()
            }
        }
        unsafe extern "system" fn GetExtensionProperty<Identity: IFaxServer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrguid: *mut core::ffi::c_void, pvproperty: *mut super::super::System::Variant::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFaxServer_Impl::GetExtensionProperty(this, core::mem::transmute(&bstrguid)) {
                    Ok(ok__) => {
                        pvproperty.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetExtensionProperty<Identity: IFaxServer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrguid: *mut core::ffi::c_void, vproperty: super::super::System::Variant::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFaxServer_Impl::SetExtensionProperty(this, core::mem::transmute(&bstrguid), core::mem::transmute(&vproperty)).into()
            }
        }
        unsafe extern "system" fn ListenToServerEvents<Identity: IFaxServer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, eventtypes: FAX_SERVER_EVENTS_TYPE_ENUM) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFaxServer_Impl::ListenToServerEvents(this, core::mem::transmute_copy(&eventtypes)).into()
            }
        }
        unsafe extern "system" fn RegisterDeviceProvider<Identity: IFaxServer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrguid: *mut core::ffi::c_void, bstrfriendlyname: *mut core::ffi::c_void, bstrimagename: *mut core::ffi::c_void, tspname: *mut core::ffi::c_void, lfspiversion: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFaxServer_Impl::RegisterDeviceProvider(this, core::mem::transmute(&bstrguid), core::mem::transmute(&bstrfriendlyname), core::mem::transmute(&bstrimagename), core::mem::transmute(&tspname), core::mem::transmute_copy(&lfspiversion)).into()
            }
        }
        unsafe extern "system" fn UnregisterDeviceProvider<Identity: IFaxServer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstruniquename: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFaxServer_Impl::UnregisterDeviceProvider(this, core::mem::transmute(&bstruniquename)).into()
            }
        }
        unsafe extern "system" fn RegisterInboundRoutingExtension<Identity: IFaxServer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrextensionname: *mut core::ffi::c_void, bstrfriendlyname: *mut core::ffi::c_void, bstrimagename: *mut core::ffi::c_void, vmethods: super::super::System::Variant::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFaxServer_Impl::RegisterInboundRoutingExtension(this, core::mem::transmute(&bstrextensionname), core::mem::transmute(&bstrfriendlyname), core::mem::transmute(&bstrimagename), core::mem::transmute(&vmethods)).into()
            }
        }
        unsafe extern "system" fn UnregisterInboundRoutingExtension<Identity: IFaxServer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrextensionuniquename: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFaxServer_Impl::UnregisterInboundRoutingExtension(this, core::mem::transmute(&bstrextensionuniquename)).into()
            }
        }
        unsafe extern "system" fn RegisteredEvents<Identity: IFaxServer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, peventtypes: *mut FAX_SERVER_EVENTS_TYPE_ENUM) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFaxServer_Impl::RegisteredEvents(this) {
                    Ok(ok__) => {
                        peventtypes.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn APIVersion<Identity: IFaxServer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, papiversion: *mut FAX_SERVER_APIVERSION_ENUM) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFaxServer_Impl::APIVersion(this) {
                    Ok(ok__) => {
                        papiversion.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            Connect: Connect::<Identity, OFFSET>,
            ServerName: ServerName::<Identity, OFFSET>,
            GetDeviceProviders: GetDeviceProviders::<Identity, OFFSET>,
            GetDevices: GetDevices::<Identity, OFFSET>,
            InboundRouting: InboundRouting::<Identity, OFFSET>,
            Folders: Folders::<Identity, OFFSET>,
            LoggingOptions: LoggingOptions::<Identity, OFFSET>,
            MajorVersion: MajorVersion::<Identity, OFFSET>,
            MinorVersion: MinorVersion::<Identity, OFFSET>,
            MajorBuild: MajorBuild::<Identity, OFFSET>,
            MinorBuild: MinorBuild::<Identity, OFFSET>,
            Debug: Debug::<Identity, OFFSET>,
            Activity: Activity::<Identity, OFFSET>,
            OutboundRouting: OutboundRouting::<Identity, OFFSET>,
            ReceiptOptions: ReceiptOptions::<Identity, OFFSET>,
            Security: Security::<Identity, OFFSET>,
            Disconnect: Disconnect::<Identity, OFFSET>,
            GetExtensionProperty: GetExtensionProperty::<Identity, OFFSET>,
            SetExtensionProperty: SetExtensionProperty::<Identity, OFFSET>,
            ListenToServerEvents: ListenToServerEvents::<Identity, OFFSET>,
            RegisterDeviceProvider: RegisterDeviceProvider::<Identity, OFFSET>,
            UnregisterDeviceProvider: UnregisterDeviceProvider::<Identity, OFFSET>,
            RegisterInboundRoutingExtension: RegisterInboundRoutingExtension::<Identity, OFFSET>,
            UnregisterInboundRoutingExtension: UnregisterInboundRoutingExtension::<Identity, OFFSET>,
            RegisteredEvents: RegisteredEvents::<Identity, OFFSET>,
            APIVersion: APIVersion::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IFaxServer as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for IFaxServer {}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IFaxServer2, IFaxServer2_Vtbl, 0x571ced0f_5609_4f40_9176_547e3a72ca7c);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IFaxServer2 {
    type Target = IFaxServer;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IFaxServer2, windows_core::IUnknown, super::super::System::Com::IDispatch, IFaxServer);
#[cfg(feature = "Win32_System_Com")]
impl IFaxServer2 {
    pub unsafe fn Configuration(&self) -> windows_core::Result<IFaxConfiguration> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Configuration)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn CurrentAccount(&self) -> windows_core::Result<IFaxAccount> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CurrentAccount)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn FaxAccountSet(&self) -> windows_core::Result<IFaxAccountSet> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).FaxAccountSet)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn Security2(&self) -> windows_core::Result<IFaxSecurity2> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Security2)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IFaxServer2_Vtbl {
    pub base__: IFaxServer_Vtbl,
    pub Configuration: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CurrentAccount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub FaxAccountSet: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Security2: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IFaxServer2_Impl: IFaxServer_Impl {
    fn Configuration(&self) -> windows_core::Result<IFaxConfiguration>;
    fn CurrentAccount(&self) -> windows_core::Result<IFaxAccount>;
    fn FaxAccountSet(&self) -> windows_core::Result<IFaxAccountSet>;
    fn Security2(&self) -> windows_core::Result<IFaxSecurity2>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IFaxServer2_Vtbl {
    pub const fn new<Identity: IFaxServer2_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Configuration<Identity: IFaxServer2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppfaxconfiguration: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFaxServer2_Impl::Configuration(this) {
                    Ok(ok__) => {
                        ppfaxconfiguration.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CurrentAccount<Identity: IFaxServer2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppcurrentaccount: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFaxServer2_Impl::CurrentAccount(this) {
                    Ok(ok__) => {
                        ppcurrentaccount.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn FaxAccountSet<Identity: IFaxServer2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppfaxaccountset: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFaxServer2_Impl::FaxAccountSet(this) {
                    Ok(ok__) => {
                        ppfaxaccountset.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Security2<Identity: IFaxServer2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppfaxsecurity2: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFaxServer2_Impl::Security2(this) {
                    Ok(ok__) => {
                        ppfaxsecurity2.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: IFaxServer_Vtbl::new::<Identity, OFFSET>(),
            Configuration: Configuration::<Identity, OFFSET>,
            CurrentAccount: CurrentAccount::<Identity, OFFSET>,
            FaxAccountSet: FaxAccountSet::<Identity, OFFSET>,
            Security2: Security2::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IFaxServer2 as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID || iid == &<IFaxServer as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for IFaxServer2 {}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IFaxServerNotify, IFaxServerNotify_Vtbl, 0x2e037b27_cf8a_4abd_b1e0_5704943bea6f);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IFaxServerNotify {
    type Target = super::super::System::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IFaxServerNotify, windows_core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IFaxServerNotify_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IFaxServerNotify_Impl: super::super::System::Com::IDispatch_Impl {}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IFaxServerNotify_Vtbl {
    pub const fn new<Identity: IFaxServerNotify_Impl, const OFFSET: isize>() -> Self {
        Self { base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, OFFSET>() }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IFaxServerNotify as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for IFaxServerNotify {}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IFaxServerNotify2, IFaxServerNotify2_Vtbl, 0xec9c69b9_5fe7_4805_9467_82fcd96af903);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IFaxServerNotify2 {
    type Target = super::super::System::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IFaxServerNotify2, windows_core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl IFaxServerNotify2 {
    pub unsafe fn OnIncomingJobAdded<P0>(&self, pfaxserver: P0, bstrjobid: &windows_core::BSTR) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IFaxServer2>,
    {
        unsafe { (windows_core::Interface::vtable(self).OnIncomingJobAdded)(windows_core::Interface::as_raw(self), pfaxserver.param().abi(), core::mem::transmute_copy(bstrjobid)).ok() }
    }
    pub unsafe fn OnIncomingJobRemoved<P0>(&self, pfaxserver: P0, bstrjobid: &windows_core::BSTR) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IFaxServer2>,
    {
        unsafe { (windows_core::Interface::vtable(self).OnIncomingJobRemoved)(windows_core::Interface::as_raw(self), pfaxserver.param().abi(), core::mem::transmute_copy(bstrjobid)).ok() }
    }
    pub unsafe fn OnIncomingJobChanged<P0, P2>(&self, pfaxserver: P0, bstrjobid: &windows_core::BSTR, pjobstatus: P2) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IFaxServer2>,
        P2: windows_core::Param<IFaxJobStatus>,
    {
        unsafe { (windows_core::Interface::vtable(self).OnIncomingJobChanged)(windows_core::Interface::as_raw(self), pfaxserver.param().abi(), core::mem::transmute_copy(bstrjobid), pjobstatus.param().abi()).ok() }
    }
    pub unsafe fn OnOutgoingJobAdded<P0>(&self, pfaxserver: P0, bstrjobid: &windows_core::BSTR) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IFaxServer2>,
    {
        unsafe { (windows_core::Interface::vtable(self).OnOutgoingJobAdded)(windows_core::Interface::as_raw(self), pfaxserver.param().abi(), core::mem::transmute_copy(bstrjobid)).ok() }
    }
    pub unsafe fn OnOutgoingJobRemoved<P0>(&self, pfaxserver: P0, bstrjobid: &windows_core::BSTR) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IFaxServer2>,
    {
        unsafe { (windows_core::Interface::vtable(self).OnOutgoingJobRemoved)(windows_core::Interface::as_raw(self), pfaxserver.param().abi(), core::mem::transmute_copy(bstrjobid)).ok() }
    }
    pub unsafe fn OnOutgoingJobChanged<P0, P2>(&self, pfaxserver: P0, bstrjobid: &windows_core::BSTR, pjobstatus: P2) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IFaxServer2>,
        P2: windows_core::Param<IFaxJobStatus>,
    {
        unsafe { (windows_core::Interface::vtable(self).OnOutgoingJobChanged)(windows_core::Interface::as_raw(self), pfaxserver.param().abi(), core::mem::transmute_copy(bstrjobid), pjobstatus.param().abi()).ok() }
    }
    pub unsafe fn OnIncomingMessageAdded<P0>(&self, pfaxserver: P0, bstrmessageid: &windows_core::BSTR) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IFaxServer2>,
    {
        unsafe { (windows_core::Interface::vtable(self).OnIncomingMessageAdded)(windows_core::Interface::as_raw(self), pfaxserver.param().abi(), core::mem::transmute_copy(bstrmessageid)).ok() }
    }
    pub unsafe fn OnIncomingMessageRemoved<P0>(&self, pfaxserver: P0, bstrmessageid: &windows_core::BSTR) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IFaxServer2>,
    {
        unsafe { (windows_core::Interface::vtable(self).OnIncomingMessageRemoved)(windows_core::Interface::as_raw(self), pfaxserver.param().abi(), core::mem::transmute_copy(bstrmessageid)).ok() }
    }
    pub unsafe fn OnOutgoingMessageAdded<P0>(&self, pfaxserver: P0, bstrmessageid: &windows_core::BSTR) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IFaxServer2>,
    {
        unsafe { (windows_core::Interface::vtable(self).OnOutgoingMessageAdded)(windows_core::Interface::as_raw(self), pfaxserver.param().abi(), core::mem::transmute_copy(bstrmessageid)).ok() }
    }
    pub unsafe fn OnOutgoingMessageRemoved<P0>(&self, pfaxserver: P0, bstrmessageid: &windows_core::BSTR) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IFaxServer2>,
    {
        unsafe { (windows_core::Interface::vtable(self).OnOutgoingMessageRemoved)(windows_core::Interface::as_raw(self), pfaxserver.param().abi(), core::mem::transmute_copy(bstrmessageid)).ok() }
    }
    pub unsafe fn OnReceiptOptionsChange<P0>(&self, pfaxserver: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IFaxServer2>,
    {
        unsafe { (windows_core::Interface::vtable(self).OnReceiptOptionsChange)(windows_core::Interface::as_raw(self), pfaxserver.param().abi()).ok() }
    }
    pub unsafe fn OnActivityLoggingConfigChange<P0>(&self, pfaxserver: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IFaxServer2>,
    {
        unsafe { (windows_core::Interface::vtable(self).OnActivityLoggingConfigChange)(windows_core::Interface::as_raw(self), pfaxserver.param().abi()).ok() }
    }
    pub unsafe fn OnSecurityConfigChange<P0>(&self, pfaxserver: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IFaxServer2>,
    {
        unsafe { (windows_core::Interface::vtable(self).OnSecurityConfigChange)(windows_core::Interface::as_raw(self), pfaxserver.param().abi()).ok() }
    }
    pub unsafe fn OnEventLoggingConfigChange<P0>(&self, pfaxserver: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IFaxServer2>,
    {
        unsafe { (windows_core::Interface::vtable(self).OnEventLoggingConfigChange)(windows_core::Interface::as_raw(self), pfaxserver.param().abi()).ok() }
    }
    pub unsafe fn OnOutgoingQueueConfigChange<P0>(&self, pfaxserver: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IFaxServer2>,
    {
        unsafe { (windows_core::Interface::vtable(self).OnOutgoingQueueConfigChange)(windows_core::Interface::as_raw(self), pfaxserver.param().abi()).ok() }
    }
    pub unsafe fn OnOutgoingArchiveConfigChange<P0>(&self, pfaxserver: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IFaxServer2>,
    {
        unsafe { (windows_core::Interface::vtable(self).OnOutgoingArchiveConfigChange)(windows_core::Interface::as_raw(self), pfaxserver.param().abi()).ok() }
    }
    pub unsafe fn OnIncomingArchiveConfigChange<P0>(&self, pfaxserver: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IFaxServer2>,
    {
        unsafe { (windows_core::Interface::vtable(self).OnIncomingArchiveConfigChange)(windows_core::Interface::as_raw(self), pfaxserver.param().abi()).ok() }
    }
    pub unsafe fn OnDevicesConfigChange<P0>(&self, pfaxserver: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IFaxServer2>,
    {
        unsafe { (windows_core::Interface::vtable(self).OnDevicesConfigChange)(windows_core::Interface::as_raw(self), pfaxserver.param().abi()).ok() }
    }
    pub unsafe fn OnOutboundRoutingGroupsConfigChange<P0>(&self, pfaxserver: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IFaxServer2>,
    {
        unsafe { (windows_core::Interface::vtable(self).OnOutboundRoutingGroupsConfigChange)(windows_core::Interface::as_raw(self), pfaxserver.param().abi()).ok() }
    }
    pub unsafe fn OnOutboundRoutingRulesConfigChange<P0>(&self, pfaxserver: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IFaxServer2>,
    {
        unsafe { (windows_core::Interface::vtable(self).OnOutboundRoutingRulesConfigChange)(windows_core::Interface::as_raw(self), pfaxserver.param().abi()).ok() }
    }
    pub unsafe fn OnServerActivityChange<P0>(&self, pfaxserver: P0, lincomingmessages: i32, lroutingmessages: i32, loutgoingmessages: i32, lqueuedmessages: i32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IFaxServer2>,
    {
        unsafe { (windows_core::Interface::vtable(self).OnServerActivityChange)(windows_core::Interface::as_raw(self), pfaxserver.param().abi(), lincomingmessages, lroutingmessages, loutgoingmessages, lqueuedmessages).ok() }
    }
    pub unsafe fn OnQueuesStatusChange<P0>(&self, pfaxserver: P0, boutgoingqueueblocked: super::super::Foundation::VARIANT_BOOL, boutgoingqueuepaused: super::super::Foundation::VARIANT_BOOL, bincomingqueueblocked: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IFaxServer2>,
    {
        unsafe { (windows_core::Interface::vtable(self).OnQueuesStatusChange)(windows_core::Interface::as_raw(self), pfaxserver.param().abi(), boutgoingqueueblocked, boutgoingqueuepaused, bincomingqueueblocked).ok() }
    }
    pub unsafe fn OnNewCall<P0>(&self, pfaxserver: P0, lcallid: i32, ldeviceid: i32, bstrcallerid: &windows_core::BSTR) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IFaxServer2>,
    {
        unsafe { (windows_core::Interface::vtable(self).OnNewCall)(windows_core::Interface::as_raw(self), pfaxserver.param().abi(), lcallid, ldeviceid, core::mem::transmute_copy(bstrcallerid)).ok() }
    }
    pub unsafe fn OnServerShutDown<P0>(&self, pfaxserver: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IFaxServer2>,
    {
        unsafe { (windows_core::Interface::vtable(self).OnServerShutDown)(windows_core::Interface::as_raw(self), pfaxserver.param().abi()).ok() }
    }
    pub unsafe fn OnDeviceStatusChange<P0>(&self, pfaxserver: P0, ldeviceid: i32, bpoweredoff: super::super::Foundation::VARIANT_BOOL, bsending: super::super::Foundation::VARIANT_BOOL, breceiving: super::super::Foundation::VARIANT_BOOL, bringing: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IFaxServer2>,
    {
        unsafe { (windows_core::Interface::vtable(self).OnDeviceStatusChange)(windows_core::Interface::as_raw(self), pfaxserver.param().abi(), ldeviceid, bpoweredoff, bsending, breceiving, bringing).ok() }
    }
    pub unsafe fn OnGeneralServerConfigChanged<P0>(&self, pfaxserver: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IFaxServer2>,
    {
        unsafe { (windows_core::Interface::vtable(self).OnGeneralServerConfigChanged)(windows_core::Interface::as_raw(self), pfaxserver.param().abi()).ok() }
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IFaxServerNotify2_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub OnIncomingJobAdded: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub OnIncomingJobRemoved: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub OnIncomingJobChanged: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub OnOutgoingJobAdded: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub OnOutgoingJobRemoved: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub OnOutgoingJobChanged: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub OnIncomingMessageAdded: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub OnIncomingMessageRemoved: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub OnOutgoingMessageAdded: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub OnOutgoingMessageRemoved: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub OnReceiptOptionsChange: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub OnActivityLoggingConfigChange: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub OnSecurityConfigChange: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub OnEventLoggingConfigChange: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub OnOutgoingQueueConfigChange: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub OnOutgoingArchiveConfigChange: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub OnIncomingArchiveConfigChange: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub OnDevicesConfigChange: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub OnOutboundRoutingGroupsConfigChange: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub OnOutboundRoutingRulesConfigChange: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub OnServerActivityChange: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, i32, i32, i32, i32) -> windows_core::HRESULT,
    pub OnQueuesStatusChange: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, super::super::Foundation::VARIANT_BOOL, super::super::Foundation::VARIANT_BOOL, super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub OnNewCall: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, i32, i32, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub OnServerShutDown: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub OnDeviceStatusChange: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, i32, super::super::Foundation::VARIANT_BOOL, super::super::Foundation::VARIANT_BOOL, super::super::Foundation::VARIANT_BOOL, super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub OnGeneralServerConfigChanged: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IFaxServerNotify2_Impl: super::super::System::Com::IDispatch_Impl {
    fn OnIncomingJobAdded(&self, pfaxserver: windows_core::Ref<IFaxServer2>, bstrjobid: &windows_core::BSTR) -> windows_core::Result<()>;
    fn OnIncomingJobRemoved(&self, pfaxserver: windows_core::Ref<IFaxServer2>, bstrjobid: &windows_core::BSTR) -> windows_core::Result<()>;
    fn OnIncomingJobChanged(&self, pfaxserver: windows_core::Ref<IFaxServer2>, bstrjobid: &windows_core::BSTR, pjobstatus: windows_core::Ref<IFaxJobStatus>) -> windows_core::Result<()>;
    fn OnOutgoingJobAdded(&self, pfaxserver: windows_core::Ref<IFaxServer2>, bstrjobid: &windows_core::BSTR) -> windows_core::Result<()>;
    fn OnOutgoingJobRemoved(&self, pfaxserver: windows_core::Ref<IFaxServer2>, bstrjobid: &windows_core::BSTR) -> windows_core::Result<()>;
    fn OnOutgoingJobChanged(&self, pfaxserver: windows_core::Ref<IFaxServer2>, bstrjobid: &windows_core::BSTR, pjobstatus: windows_core::Ref<IFaxJobStatus>) -> windows_core::Result<()>;
    fn OnIncomingMessageAdded(&self, pfaxserver: windows_core::Ref<IFaxServer2>, bstrmessageid: &windows_core::BSTR) -> windows_core::Result<()>;
    fn OnIncomingMessageRemoved(&self, pfaxserver: windows_core::Ref<IFaxServer2>, bstrmessageid: &windows_core::BSTR) -> windows_core::Result<()>;
    fn OnOutgoingMessageAdded(&self, pfaxserver: windows_core::Ref<IFaxServer2>, bstrmessageid: &windows_core::BSTR) -> windows_core::Result<()>;
    fn OnOutgoingMessageRemoved(&self, pfaxserver: windows_core::Ref<IFaxServer2>, bstrmessageid: &windows_core::BSTR) -> windows_core::Result<()>;
    fn OnReceiptOptionsChange(&self, pfaxserver: windows_core::Ref<IFaxServer2>) -> windows_core::Result<()>;
    fn OnActivityLoggingConfigChange(&self, pfaxserver: windows_core::Ref<IFaxServer2>) -> windows_core::Result<()>;
    fn OnSecurityConfigChange(&self, pfaxserver: windows_core::Ref<IFaxServer2>) -> windows_core::Result<()>;
    fn OnEventLoggingConfigChange(&self, pfaxserver: windows_core::Ref<IFaxServer2>) -> windows_core::Result<()>;
    fn OnOutgoingQueueConfigChange(&self, pfaxserver: windows_core::Ref<IFaxServer2>) -> windows_core::Result<()>;
    fn OnOutgoingArchiveConfigChange(&self, pfaxserver: windows_core::Ref<IFaxServer2>) -> windows_core::Result<()>;
    fn OnIncomingArchiveConfigChange(&self, pfaxserver: windows_core::Ref<IFaxServer2>) -> windows_core::Result<()>;
    fn OnDevicesConfigChange(&self, pfaxserver: windows_core::Ref<IFaxServer2>) -> windows_core::Result<()>;
    fn OnOutboundRoutingGroupsConfigChange(&self, pfaxserver: windows_core::Ref<IFaxServer2>) -> windows_core::Result<()>;
    fn OnOutboundRoutingRulesConfigChange(&self, pfaxserver: windows_core::Ref<IFaxServer2>) -> windows_core::Result<()>;
    fn OnServerActivityChange(&self, pfaxserver: windows_core::Ref<IFaxServer2>, lincomingmessages: i32, lroutingmessages: i32, loutgoingmessages: i32, lqueuedmessages: i32) -> windows_core::Result<()>;
    fn OnQueuesStatusChange(&self, pfaxserver: windows_core::Ref<IFaxServer2>, boutgoingqueueblocked: super::super::Foundation::VARIANT_BOOL, boutgoingqueuepaused: super::super::Foundation::VARIANT_BOOL, bincomingqueueblocked: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
    fn OnNewCall(&self, pfaxserver: windows_core::Ref<IFaxServer2>, lcallid: i32, ldeviceid: i32, bstrcallerid: &windows_core::BSTR) -> windows_core::Result<()>;
    fn OnServerShutDown(&self, pfaxserver: windows_core::Ref<IFaxServer2>) -> windows_core::Result<()>;
    fn OnDeviceStatusChange(&self, pfaxserver: windows_core::Ref<IFaxServer2>, ldeviceid: i32, bpoweredoff: super::super::Foundation::VARIANT_BOOL, bsending: super::super::Foundation::VARIANT_BOOL, breceiving: super::super::Foundation::VARIANT_BOOL, bringing: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
    fn OnGeneralServerConfigChanged(&self, pfaxserver: windows_core::Ref<IFaxServer2>) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IFaxServerNotify2_Vtbl {
    pub const fn new<Identity: IFaxServerNotify2_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn OnIncomingJobAdded<Identity: IFaxServerNotify2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pfaxserver: *mut core::ffi::c_void, bstrjobid: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFaxServerNotify2_Impl::OnIncomingJobAdded(this, core::mem::transmute_copy(&pfaxserver), core::mem::transmute(&bstrjobid)).into()
            }
        }
        unsafe extern "system" fn OnIncomingJobRemoved<Identity: IFaxServerNotify2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pfaxserver: *mut core::ffi::c_void, bstrjobid: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFaxServerNotify2_Impl::OnIncomingJobRemoved(this, core::mem::transmute_copy(&pfaxserver), core::mem::transmute(&bstrjobid)).into()
            }
        }
        unsafe extern "system" fn OnIncomingJobChanged<Identity: IFaxServerNotify2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pfaxserver: *mut core::ffi::c_void, bstrjobid: *mut core::ffi::c_void, pjobstatus: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFaxServerNotify2_Impl::OnIncomingJobChanged(this, core::mem::transmute_copy(&pfaxserver), core::mem::transmute(&bstrjobid), core::mem::transmute_copy(&pjobstatus)).into()
            }
        }
        unsafe extern "system" fn OnOutgoingJobAdded<Identity: IFaxServerNotify2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pfaxserver: *mut core::ffi::c_void, bstrjobid: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFaxServerNotify2_Impl::OnOutgoingJobAdded(this, core::mem::transmute_copy(&pfaxserver), core::mem::transmute(&bstrjobid)).into()
            }
        }
        unsafe extern "system" fn OnOutgoingJobRemoved<Identity: IFaxServerNotify2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pfaxserver: *mut core::ffi::c_void, bstrjobid: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFaxServerNotify2_Impl::OnOutgoingJobRemoved(this, core::mem::transmute_copy(&pfaxserver), core::mem::transmute(&bstrjobid)).into()
            }
        }
        unsafe extern "system" fn OnOutgoingJobChanged<Identity: IFaxServerNotify2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pfaxserver: *mut core::ffi::c_void, bstrjobid: *mut core::ffi::c_void, pjobstatus: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFaxServerNotify2_Impl::OnOutgoingJobChanged(this, core::mem::transmute_copy(&pfaxserver), core::mem::transmute(&bstrjobid), core::mem::transmute_copy(&pjobstatus)).into()
            }
        }
        unsafe extern "system" fn OnIncomingMessageAdded<Identity: IFaxServerNotify2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pfaxserver: *mut core::ffi::c_void, bstrmessageid: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFaxServerNotify2_Impl::OnIncomingMessageAdded(this, core::mem::transmute_copy(&pfaxserver), core::mem::transmute(&bstrmessageid)).into()
            }
        }
        unsafe extern "system" fn OnIncomingMessageRemoved<Identity: IFaxServerNotify2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pfaxserver: *mut core::ffi::c_void, bstrmessageid: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFaxServerNotify2_Impl::OnIncomingMessageRemoved(this, core::mem::transmute_copy(&pfaxserver), core::mem::transmute(&bstrmessageid)).into()
            }
        }
        unsafe extern "system" fn OnOutgoingMessageAdded<Identity: IFaxServerNotify2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pfaxserver: *mut core::ffi::c_void, bstrmessageid: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFaxServerNotify2_Impl::OnOutgoingMessageAdded(this, core::mem::transmute_copy(&pfaxserver), core::mem::transmute(&bstrmessageid)).into()
            }
        }
        unsafe extern "system" fn OnOutgoingMessageRemoved<Identity: IFaxServerNotify2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pfaxserver: *mut core::ffi::c_void, bstrmessageid: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFaxServerNotify2_Impl::OnOutgoingMessageRemoved(this, core::mem::transmute_copy(&pfaxserver), core::mem::transmute(&bstrmessageid)).into()
            }
        }
        unsafe extern "system" fn OnReceiptOptionsChange<Identity: IFaxServerNotify2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pfaxserver: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFaxServerNotify2_Impl::OnReceiptOptionsChange(this, core::mem::transmute_copy(&pfaxserver)).into()
            }
        }
        unsafe extern "system" fn OnActivityLoggingConfigChange<Identity: IFaxServerNotify2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pfaxserver: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFaxServerNotify2_Impl::OnActivityLoggingConfigChange(this, core::mem::transmute_copy(&pfaxserver)).into()
            }
        }
        unsafe extern "system" fn OnSecurityConfigChange<Identity: IFaxServerNotify2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pfaxserver: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFaxServerNotify2_Impl::OnSecurityConfigChange(this, core::mem::transmute_copy(&pfaxserver)).into()
            }
        }
        unsafe extern "system" fn OnEventLoggingConfigChange<Identity: IFaxServerNotify2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pfaxserver: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFaxServerNotify2_Impl::OnEventLoggingConfigChange(this, core::mem::transmute_copy(&pfaxserver)).into()
            }
        }
        unsafe extern "system" fn OnOutgoingQueueConfigChange<Identity: IFaxServerNotify2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pfaxserver: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFaxServerNotify2_Impl::OnOutgoingQueueConfigChange(this, core::mem::transmute_copy(&pfaxserver)).into()
            }
        }
        unsafe extern "system" fn OnOutgoingArchiveConfigChange<Identity: IFaxServerNotify2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pfaxserver: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFaxServerNotify2_Impl::OnOutgoingArchiveConfigChange(this, core::mem::transmute_copy(&pfaxserver)).into()
            }
        }
        unsafe extern "system" fn OnIncomingArchiveConfigChange<Identity: IFaxServerNotify2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pfaxserver: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFaxServerNotify2_Impl::OnIncomingArchiveConfigChange(this, core::mem::transmute_copy(&pfaxserver)).into()
            }
        }
        unsafe extern "system" fn OnDevicesConfigChange<Identity: IFaxServerNotify2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pfaxserver: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFaxServerNotify2_Impl::OnDevicesConfigChange(this, core::mem::transmute_copy(&pfaxserver)).into()
            }
        }
        unsafe extern "system" fn OnOutboundRoutingGroupsConfigChange<Identity: IFaxServerNotify2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pfaxserver: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFaxServerNotify2_Impl::OnOutboundRoutingGroupsConfigChange(this, core::mem::transmute_copy(&pfaxserver)).into()
            }
        }
        unsafe extern "system" fn OnOutboundRoutingRulesConfigChange<Identity: IFaxServerNotify2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pfaxserver: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFaxServerNotify2_Impl::OnOutboundRoutingRulesConfigChange(this, core::mem::transmute_copy(&pfaxserver)).into()
            }
        }
        unsafe extern "system" fn OnServerActivityChange<Identity: IFaxServerNotify2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pfaxserver: *mut core::ffi::c_void, lincomingmessages: i32, lroutingmessages: i32, loutgoingmessages: i32, lqueuedmessages: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFaxServerNotify2_Impl::OnServerActivityChange(this, core::mem::transmute_copy(&pfaxserver), core::mem::transmute_copy(&lincomingmessages), core::mem::transmute_copy(&lroutingmessages), core::mem::transmute_copy(&loutgoingmessages), core::mem::transmute_copy(&lqueuedmessages)).into()
            }
        }
        unsafe extern "system" fn OnQueuesStatusChange<Identity: IFaxServerNotify2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pfaxserver: *mut core::ffi::c_void, boutgoingqueueblocked: super::super::Foundation::VARIANT_BOOL, boutgoingqueuepaused: super::super::Foundation::VARIANT_BOOL, bincomingqueueblocked: super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFaxServerNotify2_Impl::OnQueuesStatusChange(this, core::mem::transmute_copy(&pfaxserver), core::mem::transmute_copy(&boutgoingqueueblocked), core::mem::transmute_copy(&boutgoingqueuepaused), core::mem::transmute_copy(&bincomingqueueblocked)).into()
            }
        }
        unsafe extern "system" fn OnNewCall<Identity: IFaxServerNotify2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pfaxserver: *mut core::ffi::c_void, lcallid: i32, ldeviceid: i32, bstrcallerid: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFaxServerNotify2_Impl::OnNewCall(this, core::mem::transmute_copy(&pfaxserver), core::mem::transmute_copy(&lcallid), core::mem::transmute_copy(&ldeviceid), core::mem::transmute(&bstrcallerid)).into()
            }
        }
        unsafe extern "system" fn OnServerShutDown<Identity: IFaxServerNotify2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pfaxserver: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFaxServerNotify2_Impl::OnServerShutDown(this, core::mem::transmute_copy(&pfaxserver)).into()
            }
        }
        unsafe extern "system" fn OnDeviceStatusChange<Identity: IFaxServerNotify2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pfaxserver: *mut core::ffi::c_void, ldeviceid: i32, bpoweredoff: super::super::Foundation::VARIANT_BOOL, bsending: super::super::Foundation::VARIANT_BOOL, breceiving: super::super::Foundation::VARIANT_BOOL, bringing: super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFaxServerNotify2_Impl::OnDeviceStatusChange(this, core::mem::transmute_copy(&pfaxserver), core::mem::transmute_copy(&ldeviceid), core::mem::transmute_copy(&bpoweredoff), core::mem::transmute_copy(&bsending), core::mem::transmute_copy(&breceiving), core::mem::transmute_copy(&bringing)).into()
            }
        }
        unsafe extern "system" fn OnGeneralServerConfigChanged<Identity: IFaxServerNotify2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pfaxserver: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFaxServerNotify2_Impl::OnGeneralServerConfigChanged(this, core::mem::transmute_copy(&pfaxserver)).into()
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            OnIncomingJobAdded: OnIncomingJobAdded::<Identity, OFFSET>,
            OnIncomingJobRemoved: OnIncomingJobRemoved::<Identity, OFFSET>,
            OnIncomingJobChanged: OnIncomingJobChanged::<Identity, OFFSET>,
            OnOutgoingJobAdded: OnOutgoingJobAdded::<Identity, OFFSET>,
            OnOutgoingJobRemoved: OnOutgoingJobRemoved::<Identity, OFFSET>,
            OnOutgoingJobChanged: OnOutgoingJobChanged::<Identity, OFFSET>,
            OnIncomingMessageAdded: OnIncomingMessageAdded::<Identity, OFFSET>,
            OnIncomingMessageRemoved: OnIncomingMessageRemoved::<Identity, OFFSET>,
            OnOutgoingMessageAdded: OnOutgoingMessageAdded::<Identity, OFFSET>,
            OnOutgoingMessageRemoved: OnOutgoingMessageRemoved::<Identity, OFFSET>,
            OnReceiptOptionsChange: OnReceiptOptionsChange::<Identity, OFFSET>,
            OnActivityLoggingConfigChange: OnActivityLoggingConfigChange::<Identity, OFFSET>,
            OnSecurityConfigChange: OnSecurityConfigChange::<Identity, OFFSET>,
            OnEventLoggingConfigChange: OnEventLoggingConfigChange::<Identity, OFFSET>,
            OnOutgoingQueueConfigChange: OnOutgoingQueueConfigChange::<Identity, OFFSET>,
            OnOutgoingArchiveConfigChange: OnOutgoingArchiveConfigChange::<Identity, OFFSET>,
            OnIncomingArchiveConfigChange: OnIncomingArchiveConfigChange::<Identity, OFFSET>,
            OnDevicesConfigChange: OnDevicesConfigChange::<Identity, OFFSET>,
            OnOutboundRoutingGroupsConfigChange: OnOutboundRoutingGroupsConfigChange::<Identity, OFFSET>,
            OnOutboundRoutingRulesConfigChange: OnOutboundRoutingRulesConfigChange::<Identity, OFFSET>,
            OnServerActivityChange: OnServerActivityChange::<Identity, OFFSET>,
            OnQueuesStatusChange: OnQueuesStatusChange::<Identity, OFFSET>,
            OnNewCall: OnNewCall::<Identity, OFFSET>,
            OnServerShutDown: OnServerShutDown::<Identity, OFFSET>,
            OnDeviceStatusChange: OnDeviceStatusChange::<Identity, OFFSET>,
            OnGeneralServerConfigChanged: OnGeneralServerConfigChanged::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IFaxServerNotify2 as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for IFaxServerNotify2 {}
pub const IS_DIGITAL_CAMERA_STR: windows_core::PCWSTR = windows_core::w!("IsDigitalCamera");
pub const IS_DIGITAL_CAMERA_VAL: u32 = 1u32;
windows_core::imp::define_interface!(IStiDevice, IStiDevice_Vtbl, 0x6cfa5a80_2dc8_11d0_90ea_00aa0060f86c);
windows_core::imp::interface_hierarchy!(IStiDevice, windows_core::IUnknown);
impl IStiDevice {
    pub unsafe fn Initialize<P1>(&self, hinst: super::super::Foundation::HINSTANCE, pwszdevicename: P1, dwversion: u32, dwmode: u32) -> windows_core::Result<()>
    where
        P1: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).Initialize)(windows_core::Interface::as_raw(self), hinst, pwszdevicename.param().abi(), dwversion, dwmode).ok() }
    }
    pub unsafe fn GetCapabilities(&self, pdevcaps: *mut STI_DEV_CAPS) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetCapabilities)(windows_core::Interface::as_raw(self), pdevcaps as _).ok() }
    }
    pub unsafe fn GetStatus(&self, pdevstatus: *mut STI_DEVICE_STATUS) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetStatus)(windows_core::Interface::as_raw(self), pdevstatus as _).ok() }
    }
    pub unsafe fn DeviceReset(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).DeviceReset)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn Diagnostic(&self, pbuffer: *mut STI_DIAG) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Diagnostic)(windows_core::Interface::as_raw(self), pbuffer as _).ok() }
    }
    pub unsafe fn Escape(&self, escapefunction: u32, lpindata: *const core::ffi::c_void, cbindatasize: u32, poutdata: *mut core::ffi::c_void, dwoutdatasize: u32, pdwactualdata: *mut u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Escape)(windows_core::Interface::as_raw(self), escapefunction, lpindata, cbindatasize, poutdata as _, dwoutdatasize, pdwactualdata as _).ok() }
    }
    pub unsafe fn GetLastError(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetLastError)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn LockDevice(&self, dwtimeout: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).LockDevice)(windows_core::Interface::as_raw(self), dwtimeout).ok() }
    }
    pub unsafe fn UnLockDevice(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).UnLockDevice)(windows_core::Interface::as_raw(self)).ok() }
    }
    #[cfg(feature = "Win32_System_IO")]
    pub unsafe fn RawReadData(&self, lpbuffer: *mut core::ffi::c_void, lpdwnumberofbytes: *mut u32, lpoverlapped: Option<*const super::super::System::IO::OVERLAPPED>) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).RawReadData)(windows_core::Interface::as_raw(self), lpbuffer as _, lpdwnumberofbytes as _, lpoverlapped.unwrap_or(core::mem::zeroed()) as _).ok() }
    }
    #[cfg(feature = "Win32_System_IO")]
    pub unsafe fn RawWriteData(&self, lpbuffer: *const core::ffi::c_void, nnumberofbytes: u32, lpoverlapped: Option<*const super::super::System::IO::OVERLAPPED>) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).RawWriteData)(windows_core::Interface::as_raw(self), lpbuffer, nnumberofbytes, lpoverlapped.unwrap_or(core::mem::zeroed()) as _).ok() }
    }
    #[cfg(feature = "Win32_System_IO")]
    pub unsafe fn RawReadCommand(&self, lpbuffer: *mut core::ffi::c_void, lpdwnumberofbytes: *mut u32, lpoverlapped: Option<*const super::super::System::IO::OVERLAPPED>) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).RawReadCommand)(windows_core::Interface::as_raw(self), lpbuffer as _, lpdwnumberofbytes as _, lpoverlapped.unwrap_or(core::mem::zeroed()) as _).ok() }
    }
    #[cfg(feature = "Win32_System_IO")]
    pub unsafe fn RawWriteCommand(&self, lpbuffer: *const core::ffi::c_void, nnumberofbytes: u32, lpoverlapped: Option<*const super::super::System::IO::OVERLAPPED>) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).RawWriteCommand)(windows_core::Interface::as_raw(self), lpbuffer, nnumberofbytes, lpoverlapped.unwrap_or(core::mem::zeroed()) as _).ok() }
    }
    pub unsafe fn Subscribe(&self, lpsubsribe: *mut STISUBSCRIBE) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Subscribe)(windows_core::Interface::as_raw(self), lpsubsribe as _).ok() }
    }
    pub unsafe fn GetLastNotificationData(&self, lpnotify: *mut STINOTIFY) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetLastNotificationData)(windows_core::Interface::as_raw(self), lpnotify as _).ok() }
    }
    pub unsafe fn UnSubscribe(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).UnSubscribe)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn GetLastErrorInfo(&self, plasterrorinfo: *mut _ERROR_INFOW) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetLastErrorInfo)(windows_core::Interface::as_raw(self), plasterrorinfo as _).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IStiDevice_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Initialize: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::HINSTANCE, windows_core::PCWSTR, u32, u32) -> windows_core::HRESULT,
    pub GetCapabilities: unsafe extern "system" fn(*mut core::ffi::c_void, *mut STI_DEV_CAPS) -> windows_core::HRESULT,
    pub GetStatus: unsafe extern "system" fn(*mut core::ffi::c_void, *mut STI_DEVICE_STATUS) -> windows_core::HRESULT,
    pub DeviceReset: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Diagnostic: unsafe extern "system" fn(*mut core::ffi::c_void, *mut STI_DIAG) -> windows_core::HRESULT,
    pub Escape: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const core::ffi::c_void, u32, *mut core::ffi::c_void, u32, *mut u32) -> windows_core::HRESULT,
    pub GetLastError: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub LockDevice: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub UnLockDevice: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_IO")]
    pub RawReadData: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut u32, *const super::super::System::IO::OVERLAPPED) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_IO"))]
    RawReadData: usize,
    #[cfg(feature = "Win32_System_IO")]
    pub RawWriteData: unsafe extern "system" fn(*mut core::ffi::c_void, *const core::ffi::c_void, u32, *const super::super::System::IO::OVERLAPPED) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_IO"))]
    RawWriteData: usize,
    #[cfg(feature = "Win32_System_IO")]
    pub RawReadCommand: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut u32, *const super::super::System::IO::OVERLAPPED) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_IO"))]
    RawReadCommand: usize,
    #[cfg(feature = "Win32_System_IO")]
    pub RawWriteCommand: unsafe extern "system" fn(*mut core::ffi::c_void, *const core::ffi::c_void, u32, *const super::super::System::IO::OVERLAPPED) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_IO"))]
    RawWriteCommand: usize,
    pub Subscribe: unsafe extern "system" fn(*mut core::ffi::c_void, *mut STISUBSCRIBE) -> windows_core::HRESULT,
    pub GetLastNotificationData: unsafe extern "system" fn(*mut core::ffi::c_void, *mut STINOTIFY) -> windows_core::HRESULT,
    pub UnSubscribe: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetLastErrorInfo: unsafe extern "system" fn(*mut core::ffi::c_void, *mut _ERROR_INFOW) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_IO")]
pub trait IStiDevice_Impl: windows_core::IUnknownImpl {
    fn Initialize(&self, hinst: super::super::Foundation::HINSTANCE, pwszdevicename: &windows_core::PCWSTR, dwversion: u32, dwmode: u32) -> windows_core::Result<()>;
    fn GetCapabilities(&self, pdevcaps: *mut STI_DEV_CAPS) -> windows_core::Result<()>;
    fn GetStatus(&self, pdevstatus: *mut STI_DEVICE_STATUS) -> windows_core::Result<()>;
    fn DeviceReset(&self) -> windows_core::Result<()>;
    fn Diagnostic(&self, pbuffer: *mut STI_DIAG) -> windows_core::Result<()>;
    fn Escape(&self, escapefunction: u32, lpindata: *const core::ffi::c_void, cbindatasize: u32, poutdata: *mut core::ffi::c_void, dwoutdatasize: u32, pdwactualdata: *mut u32) -> windows_core::Result<()>;
    fn GetLastError(&self) -> windows_core::Result<u32>;
    fn LockDevice(&self, dwtimeout: u32) -> windows_core::Result<()>;
    fn UnLockDevice(&self) -> windows_core::Result<()>;
    fn RawReadData(&self, lpbuffer: *mut core::ffi::c_void, lpdwnumberofbytes: *mut u32, lpoverlapped: *const super::super::System::IO::OVERLAPPED) -> windows_core::Result<()>;
    fn RawWriteData(&self, lpbuffer: *const core::ffi::c_void, nnumberofbytes: u32, lpoverlapped: *const super::super::System::IO::OVERLAPPED) -> windows_core::Result<()>;
    fn RawReadCommand(&self, lpbuffer: *mut core::ffi::c_void, lpdwnumberofbytes: *mut u32, lpoverlapped: *const super::super::System::IO::OVERLAPPED) -> windows_core::Result<()>;
    fn RawWriteCommand(&self, lpbuffer: *const core::ffi::c_void, nnumberofbytes: u32, lpoverlapped: *const super::super::System::IO::OVERLAPPED) -> windows_core::Result<()>;
    fn Subscribe(&self, lpsubsribe: *mut STISUBSCRIBE) -> windows_core::Result<()>;
    fn GetLastNotificationData(&self, lpnotify: *mut STINOTIFY) -> windows_core::Result<()>;
    fn UnSubscribe(&self) -> windows_core::Result<()>;
    fn GetLastErrorInfo(&self, plasterrorinfo: *mut _ERROR_INFOW) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_IO")]
impl IStiDevice_Vtbl {
    pub const fn new<Identity: IStiDevice_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Initialize<Identity: IStiDevice_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hinst: super::super::Foundation::HINSTANCE, pwszdevicename: windows_core::PCWSTR, dwversion: u32, dwmode: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IStiDevice_Impl::Initialize(this, core::mem::transmute_copy(&hinst), core::mem::transmute(&pwszdevicename), core::mem::transmute_copy(&dwversion), core::mem::transmute_copy(&dwmode)).into()
            }
        }
        unsafe extern "system" fn GetCapabilities<Identity: IStiDevice_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdevcaps: *mut STI_DEV_CAPS) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IStiDevice_Impl::GetCapabilities(this, core::mem::transmute_copy(&pdevcaps)).into()
            }
        }
        unsafe extern "system" fn GetStatus<Identity: IStiDevice_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdevstatus: *mut STI_DEVICE_STATUS) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IStiDevice_Impl::GetStatus(this, core::mem::transmute_copy(&pdevstatus)).into()
            }
        }
        unsafe extern "system" fn DeviceReset<Identity: IStiDevice_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IStiDevice_Impl::DeviceReset(this).into()
            }
        }
        unsafe extern "system" fn Diagnostic<Identity: IStiDevice_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbuffer: *mut STI_DIAG) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IStiDevice_Impl::Diagnostic(this, core::mem::transmute_copy(&pbuffer)).into()
            }
        }
        unsafe extern "system" fn Escape<Identity: IStiDevice_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, escapefunction: u32, lpindata: *const core::ffi::c_void, cbindatasize: u32, poutdata: *mut core::ffi::c_void, dwoutdatasize: u32, pdwactualdata: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IStiDevice_Impl::Escape(this, core::mem::transmute_copy(&escapefunction), core::mem::transmute_copy(&lpindata), core::mem::transmute_copy(&cbindatasize), core::mem::transmute_copy(&poutdata), core::mem::transmute_copy(&dwoutdatasize), core::mem::transmute_copy(&pdwactualdata)).into()
            }
        }
        unsafe extern "system" fn GetLastError<Identity: IStiDevice_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwlastdeviceerror: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IStiDevice_Impl::GetLastError(this) {
                    Ok(ok__) => {
                        pdwlastdeviceerror.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn LockDevice<Identity: IStiDevice_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwtimeout: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IStiDevice_Impl::LockDevice(this, core::mem::transmute_copy(&dwtimeout)).into()
            }
        }
        unsafe extern "system" fn UnLockDevice<Identity: IStiDevice_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IStiDevice_Impl::UnLockDevice(this).into()
            }
        }
        unsafe extern "system" fn RawReadData<Identity: IStiDevice_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lpbuffer: *mut core::ffi::c_void, lpdwnumberofbytes: *mut u32, lpoverlapped: *const super::super::System::IO::OVERLAPPED) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IStiDevice_Impl::RawReadData(this, core::mem::transmute_copy(&lpbuffer), core::mem::transmute_copy(&lpdwnumberofbytes), core::mem::transmute_copy(&lpoverlapped)).into()
            }
        }
        unsafe extern "system" fn RawWriteData<Identity: IStiDevice_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lpbuffer: *const core::ffi::c_void, nnumberofbytes: u32, lpoverlapped: *const super::super::System::IO::OVERLAPPED) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IStiDevice_Impl::RawWriteData(this, core::mem::transmute_copy(&lpbuffer), core::mem::transmute_copy(&nnumberofbytes), core::mem::transmute_copy(&lpoverlapped)).into()
            }
        }
        unsafe extern "system" fn RawReadCommand<Identity: IStiDevice_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lpbuffer: *mut core::ffi::c_void, lpdwnumberofbytes: *mut u32, lpoverlapped: *const super::super::System::IO::OVERLAPPED) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IStiDevice_Impl::RawReadCommand(this, core::mem::transmute_copy(&lpbuffer), core::mem::transmute_copy(&lpdwnumberofbytes), core::mem::transmute_copy(&lpoverlapped)).into()
            }
        }
        unsafe extern "system" fn RawWriteCommand<Identity: IStiDevice_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lpbuffer: *const core::ffi::c_void, nnumberofbytes: u32, lpoverlapped: *const super::super::System::IO::OVERLAPPED) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IStiDevice_Impl::RawWriteCommand(this, core::mem::transmute_copy(&lpbuffer), core::mem::transmute_copy(&nnumberofbytes), core::mem::transmute_copy(&lpoverlapped)).into()
            }
        }
        unsafe extern "system" fn Subscribe<Identity: IStiDevice_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lpsubsribe: *mut STISUBSCRIBE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IStiDevice_Impl::Subscribe(this, core::mem::transmute_copy(&lpsubsribe)).into()
            }
        }
        unsafe extern "system" fn GetLastNotificationData<Identity: IStiDevice_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lpnotify: *mut STINOTIFY) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IStiDevice_Impl::GetLastNotificationData(this, core::mem::transmute_copy(&lpnotify)).into()
            }
        }
        unsafe extern "system" fn UnSubscribe<Identity: IStiDevice_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IStiDevice_Impl::UnSubscribe(this).into()
            }
        }
        unsafe extern "system" fn GetLastErrorInfo<Identity: IStiDevice_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, plasterrorinfo: *mut _ERROR_INFOW) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IStiDevice_Impl::GetLastErrorInfo(this, core::mem::transmute_copy(&plasterrorinfo)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Initialize: Initialize::<Identity, OFFSET>,
            GetCapabilities: GetCapabilities::<Identity, OFFSET>,
            GetStatus: GetStatus::<Identity, OFFSET>,
            DeviceReset: DeviceReset::<Identity, OFFSET>,
            Diagnostic: Diagnostic::<Identity, OFFSET>,
            Escape: Escape::<Identity, OFFSET>,
            GetLastError: GetLastError::<Identity, OFFSET>,
            LockDevice: LockDevice::<Identity, OFFSET>,
            UnLockDevice: UnLockDevice::<Identity, OFFSET>,
            RawReadData: RawReadData::<Identity, OFFSET>,
            RawWriteData: RawWriteData::<Identity, OFFSET>,
            RawReadCommand: RawReadCommand::<Identity, OFFSET>,
            RawWriteCommand: RawWriteCommand::<Identity, OFFSET>,
            Subscribe: Subscribe::<Identity, OFFSET>,
            GetLastNotificationData: GetLastNotificationData::<Identity, OFFSET>,
            UnSubscribe: UnSubscribe::<Identity, OFFSET>,
            GetLastErrorInfo: GetLastErrorInfo::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IStiDevice as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_IO")]
impl windows_core::RuntimeName for IStiDevice {}
windows_core::imp::define_interface!(IStiDeviceControl, IStiDeviceControl_Vtbl, 0x128a9860_52dc_11d0_9edf_444553540000);
windows_core::imp::interface_hierarchy!(IStiDeviceControl, windows_core::IUnknown);
impl IStiDeviceControl {
    pub unsafe fn Initialize<P2>(&self, dwdevicetype: u32, dwmode: u32, pwszportname: P2, dwflags: u32) -> windows_core::Result<()>
    where
        P2: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).Initialize)(windows_core::Interface::as_raw(self), dwdevicetype, dwmode, pwszportname.param().abi(), dwflags).ok() }
    }
    #[cfg(feature = "Win32_System_IO")]
    pub unsafe fn RawReadData(&self, lpbuffer: *mut core::ffi::c_void, lpdwnumberofbytes: *mut u32, lpoverlapped: *mut super::super::System::IO::OVERLAPPED) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).RawReadData)(windows_core::Interface::as_raw(self), lpbuffer as _, lpdwnumberofbytes as _, lpoverlapped as _).ok() }
    }
    #[cfg(feature = "Win32_System_IO")]
    pub unsafe fn RawWriteData(&self, lpbuffer: *mut core::ffi::c_void, nnumberofbytes: u32, lpoverlapped: *mut super::super::System::IO::OVERLAPPED) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).RawWriteData)(windows_core::Interface::as_raw(self), lpbuffer as _, nnumberofbytes, lpoverlapped as _).ok() }
    }
    #[cfg(feature = "Win32_System_IO")]
    pub unsafe fn RawReadCommand(&self, lpbuffer: *mut core::ffi::c_void, lpdwnumberofbytes: *mut u32, lpoverlapped: *mut super::super::System::IO::OVERLAPPED) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).RawReadCommand)(windows_core::Interface::as_raw(self), lpbuffer as _, lpdwnumberofbytes as _, lpoverlapped as _).ok() }
    }
    #[cfg(feature = "Win32_System_IO")]
    pub unsafe fn RawWriteCommand(&self, lpbuffer: *mut core::ffi::c_void, nnumberofbytes: u32, lpoverlapped: *mut super::super::System::IO::OVERLAPPED) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).RawWriteCommand)(windows_core::Interface::as_raw(self), lpbuffer as _, nnumberofbytes, lpoverlapped as _).ok() }
    }
    pub unsafe fn RawDeviceControl(&self, escapefunction: u32, lpindata: *mut core::ffi::c_void, cbindatasize: u32, poutdata: *mut core::ffi::c_void, dwoutdatasize: u32, pdwactualdata: *mut u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).RawDeviceControl)(windows_core::Interface::as_raw(self), escapefunction, lpindata as _, cbindatasize, poutdata as _, dwoutdatasize, pdwactualdata as _).ok() }
    }
    pub unsafe fn GetLastError(&self, lpdwlasterror: *mut u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetLastError)(windows_core::Interface::as_raw(self), lpdwlasterror as _).ok() }
    }
    pub unsafe fn GetMyDevicePortName(&self, lpszdevicepath: &mut [u16]) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetMyDevicePortName)(windows_core::Interface::as_raw(self), core::mem::transmute(lpszdevicepath.as_ptr()), lpszdevicepath.len().try_into().unwrap()).ok() }
    }
    pub unsafe fn GetMyDeviceHandle(&self, lph: *mut super::super::Foundation::HANDLE) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetMyDeviceHandle)(windows_core::Interface::as_raw(self), lph as _).ok() }
    }
    pub unsafe fn GetMyDeviceOpenMode(&self, pdwopenmode: *mut u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetMyDeviceOpenMode)(windows_core::Interface::as_raw(self), pdwopenmode as _).ok() }
    }
    pub unsafe fn WriteToErrorLog<P1>(&self, dwmessagetype: u32, pszmessage: P1, dwerrorcode: u32) -> windows_core::Result<()>
    where
        P1: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).WriteToErrorLog)(windows_core::Interface::as_raw(self), dwmessagetype, pszmessage.param().abi(), dwerrorcode).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IStiDeviceControl_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Initialize: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, windows_core::PCWSTR, u32) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_IO")]
    pub RawReadData: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut u32, *mut super::super::System::IO::OVERLAPPED) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_IO"))]
    RawReadData: usize,
    #[cfg(feature = "Win32_System_IO")]
    pub RawWriteData: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, u32, *mut super::super::System::IO::OVERLAPPED) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_IO"))]
    RawWriteData: usize,
    #[cfg(feature = "Win32_System_IO")]
    pub RawReadCommand: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut u32, *mut super::super::System::IO::OVERLAPPED) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_IO"))]
    RawReadCommand: usize,
    #[cfg(feature = "Win32_System_IO")]
    pub RawWriteCommand: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, u32, *mut super::super::System::IO::OVERLAPPED) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_IO"))]
    RawWriteCommand: usize,
    pub RawDeviceControl: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut core::ffi::c_void, u32, *mut core::ffi::c_void, u32, *mut u32) -> windows_core::HRESULT,
    pub GetLastError: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub GetMyDevicePortName: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PWSTR, u32) -> windows_core::HRESULT,
    pub GetMyDeviceHandle: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::HANDLE) -> windows_core::HRESULT,
    pub GetMyDeviceOpenMode: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub WriteToErrorLog: unsafe extern "system" fn(*mut core::ffi::c_void, u32, windows_core::PCWSTR, u32) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_IO")]
pub trait IStiDeviceControl_Impl: windows_core::IUnknownImpl {
    fn Initialize(&self, dwdevicetype: u32, dwmode: u32, pwszportname: &windows_core::PCWSTR, dwflags: u32) -> windows_core::Result<()>;
    fn RawReadData(&self, lpbuffer: *mut core::ffi::c_void, lpdwnumberofbytes: *mut u32, lpoverlapped: *mut super::super::System::IO::OVERLAPPED) -> windows_core::Result<()>;
    fn RawWriteData(&self, lpbuffer: *mut core::ffi::c_void, nnumberofbytes: u32, lpoverlapped: *mut super::super::System::IO::OVERLAPPED) -> windows_core::Result<()>;
    fn RawReadCommand(&self, lpbuffer: *mut core::ffi::c_void, lpdwnumberofbytes: *mut u32, lpoverlapped: *mut super::super::System::IO::OVERLAPPED) -> windows_core::Result<()>;
    fn RawWriteCommand(&self, lpbuffer: *mut core::ffi::c_void, nnumberofbytes: u32, lpoverlapped: *mut super::super::System::IO::OVERLAPPED) -> windows_core::Result<()>;
    fn RawDeviceControl(&self, escapefunction: u32, lpindata: *mut core::ffi::c_void, cbindatasize: u32, poutdata: *mut core::ffi::c_void, dwoutdatasize: u32, pdwactualdata: *mut u32) -> windows_core::Result<()>;
    fn GetLastError(&self, lpdwlasterror: *mut u32) -> windows_core::Result<()>;
    fn GetMyDevicePortName(&self, lpszdevicepath: windows_core::PWSTR, cwdevicepathsize: u32) -> windows_core::Result<()>;
    fn GetMyDeviceHandle(&self, lph: *mut super::super::Foundation::HANDLE) -> windows_core::Result<()>;
    fn GetMyDeviceOpenMode(&self, pdwopenmode: *mut u32) -> windows_core::Result<()>;
    fn WriteToErrorLog(&self, dwmessagetype: u32, pszmessage: &windows_core::PCWSTR, dwerrorcode: u32) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_IO")]
impl IStiDeviceControl_Vtbl {
    pub const fn new<Identity: IStiDeviceControl_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Initialize<Identity: IStiDeviceControl_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwdevicetype: u32, dwmode: u32, pwszportname: windows_core::PCWSTR, dwflags: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IStiDeviceControl_Impl::Initialize(this, core::mem::transmute_copy(&dwdevicetype), core::mem::transmute_copy(&dwmode), core::mem::transmute(&pwszportname), core::mem::transmute_copy(&dwflags)).into()
            }
        }
        unsafe extern "system" fn RawReadData<Identity: IStiDeviceControl_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lpbuffer: *mut core::ffi::c_void, lpdwnumberofbytes: *mut u32, lpoverlapped: *mut super::super::System::IO::OVERLAPPED) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IStiDeviceControl_Impl::RawReadData(this, core::mem::transmute_copy(&lpbuffer), core::mem::transmute_copy(&lpdwnumberofbytes), core::mem::transmute_copy(&lpoverlapped)).into()
            }
        }
        unsafe extern "system" fn RawWriteData<Identity: IStiDeviceControl_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lpbuffer: *mut core::ffi::c_void, nnumberofbytes: u32, lpoverlapped: *mut super::super::System::IO::OVERLAPPED) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IStiDeviceControl_Impl::RawWriteData(this, core::mem::transmute_copy(&lpbuffer), core::mem::transmute_copy(&nnumberofbytes), core::mem::transmute_copy(&lpoverlapped)).into()
            }
        }
        unsafe extern "system" fn RawReadCommand<Identity: IStiDeviceControl_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lpbuffer: *mut core::ffi::c_void, lpdwnumberofbytes: *mut u32, lpoverlapped: *mut super::super::System::IO::OVERLAPPED) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IStiDeviceControl_Impl::RawReadCommand(this, core::mem::transmute_copy(&lpbuffer), core::mem::transmute_copy(&lpdwnumberofbytes), core::mem::transmute_copy(&lpoverlapped)).into()
            }
        }
        unsafe extern "system" fn RawWriteCommand<Identity: IStiDeviceControl_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lpbuffer: *mut core::ffi::c_void, nnumberofbytes: u32, lpoverlapped: *mut super::super::System::IO::OVERLAPPED) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IStiDeviceControl_Impl::RawWriteCommand(this, core::mem::transmute_copy(&lpbuffer), core::mem::transmute_copy(&nnumberofbytes), core::mem::transmute_copy(&lpoverlapped)).into()
            }
        }
        unsafe extern "system" fn RawDeviceControl<Identity: IStiDeviceControl_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, escapefunction: u32, lpindata: *mut core::ffi::c_void, cbindatasize: u32, poutdata: *mut core::ffi::c_void, dwoutdatasize: u32, pdwactualdata: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IStiDeviceControl_Impl::RawDeviceControl(this, core::mem::transmute_copy(&escapefunction), core::mem::transmute_copy(&lpindata), core::mem::transmute_copy(&cbindatasize), core::mem::transmute_copy(&poutdata), core::mem::transmute_copy(&dwoutdatasize), core::mem::transmute_copy(&pdwactualdata)).into()
            }
        }
        unsafe extern "system" fn GetLastError<Identity: IStiDeviceControl_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lpdwlasterror: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IStiDeviceControl_Impl::GetLastError(this, core::mem::transmute_copy(&lpdwlasterror)).into()
            }
        }
        unsafe extern "system" fn GetMyDevicePortName<Identity: IStiDeviceControl_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lpszdevicepath: windows_core::PWSTR, cwdevicepathsize: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IStiDeviceControl_Impl::GetMyDevicePortName(this, core::mem::transmute_copy(&lpszdevicepath), core::mem::transmute_copy(&cwdevicepathsize)).into()
            }
        }
        unsafe extern "system" fn GetMyDeviceHandle<Identity: IStiDeviceControl_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lph: *mut super::super::Foundation::HANDLE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IStiDeviceControl_Impl::GetMyDeviceHandle(this, core::mem::transmute_copy(&lph)).into()
            }
        }
        unsafe extern "system" fn GetMyDeviceOpenMode<Identity: IStiDeviceControl_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwopenmode: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IStiDeviceControl_Impl::GetMyDeviceOpenMode(this, core::mem::transmute_copy(&pdwopenmode)).into()
            }
        }
        unsafe extern "system" fn WriteToErrorLog<Identity: IStiDeviceControl_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwmessagetype: u32, pszmessage: windows_core::PCWSTR, dwerrorcode: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IStiDeviceControl_Impl::WriteToErrorLog(this, core::mem::transmute_copy(&dwmessagetype), core::mem::transmute(&pszmessage), core::mem::transmute_copy(&dwerrorcode)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Initialize: Initialize::<Identity, OFFSET>,
            RawReadData: RawReadData::<Identity, OFFSET>,
            RawWriteData: RawWriteData::<Identity, OFFSET>,
            RawReadCommand: RawReadCommand::<Identity, OFFSET>,
            RawWriteCommand: RawWriteCommand::<Identity, OFFSET>,
            RawDeviceControl: RawDeviceControl::<Identity, OFFSET>,
            GetLastError: GetLastError::<Identity, OFFSET>,
            GetMyDevicePortName: GetMyDevicePortName::<Identity, OFFSET>,
            GetMyDeviceHandle: GetMyDeviceHandle::<Identity, OFFSET>,
            GetMyDeviceOpenMode: GetMyDeviceOpenMode::<Identity, OFFSET>,
            WriteToErrorLog: WriteToErrorLog::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IStiDeviceControl as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_IO")]
impl windows_core::RuntimeName for IStiDeviceControl {}
windows_core::imp::define_interface!(IStiUSD, IStiUSD_Vtbl, 0x0c9bb460_51ac_11d0_90ea_00aa0060f86c);
windows_core::imp::interface_hierarchy!(IStiUSD, windows_core::IUnknown);
impl IStiUSD {
    #[cfg(feature = "Win32_System_Registry")]
    pub unsafe fn Initialize<P0>(&self, pheldcb: P0, dwstiversion: u32, hparameterskey: super::super::System::Registry::HKEY) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IStiDeviceControl>,
    {
        unsafe { (windows_core::Interface::vtable(self).Initialize)(windows_core::Interface::as_raw(self), pheldcb.param().abi(), dwstiversion, hparameterskey).ok() }
    }
    pub unsafe fn GetCapabilities(&self) -> windows_core::Result<STI_USD_CAPS> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetCapabilities)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetStatus(&self, pdevstatus: *mut STI_DEVICE_STATUS) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetStatus)(windows_core::Interface::as_raw(self), pdevstatus as _).ok() }
    }
    pub unsafe fn DeviceReset(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).DeviceReset)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn Diagnostic(&self, pbuffer: *mut STI_DIAG) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Diagnostic)(windows_core::Interface::as_raw(self), pbuffer as _).ok() }
    }
    pub unsafe fn Escape(&self, escapefunction: u32, lpindata: *const core::ffi::c_void, cbindatasize: u32, poutdata: *mut core::ffi::c_void, cboutdatasize: u32, pdwactualdata: *mut u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Escape)(windows_core::Interface::as_raw(self), escapefunction, lpindata, cbindatasize, poutdata as _, cboutdatasize, pdwactualdata as _).ok() }
    }
    pub unsafe fn GetLastError(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetLastError)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn LockDevice(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).LockDevice)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn UnLockDevice(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).UnLockDevice)(windows_core::Interface::as_raw(self)).ok() }
    }
    #[cfg(feature = "Win32_System_IO")]
    pub unsafe fn RawReadData(&self, lpbuffer: *mut core::ffi::c_void, lpdwnumberofbytes: *mut u32, lpoverlapped: Option<*const super::super::System::IO::OVERLAPPED>) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).RawReadData)(windows_core::Interface::as_raw(self), lpbuffer as _, lpdwnumberofbytes as _, lpoverlapped.unwrap_or(core::mem::zeroed()) as _).ok() }
    }
    #[cfg(feature = "Win32_System_IO")]
    pub unsafe fn RawWriteData(&self, lpbuffer: *const core::ffi::c_void, nnumberofbytes: u32, lpoverlapped: Option<*const super::super::System::IO::OVERLAPPED>) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).RawWriteData)(windows_core::Interface::as_raw(self), lpbuffer, nnumberofbytes, lpoverlapped.unwrap_or(core::mem::zeroed()) as _).ok() }
    }
    #[cfg(feature = "Win32_System_IO")]
    pub unsafe fn RawReadCommand(&self, lpbuffer: *mut core::ffi::c_void, lpdwnumberofbytes: *mut u32, lpoverlapped: Option<*const super::super::System::IO::OVERLAPPED>) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).RawReadCommand)(windows_core::Interface::as_raw(self), lpbuffer as _, lpdwnumberofbytes as _, lpoverlapped.unwrap_or(core::mem::zeroed()) as _).ok() }
    }
    #[cfg(feature = "Win32_System_IO")]
    pub unsafe fn RawWriteCommand(&self, lpbuffer: *const core::ffi::c_void, nnumberofbytes: u32, lpoverlapped: Option<*const super::super::System::IO::OVERLAPPED>) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).RawWriteCommand)(windows_core::Interface::as_raw(self), lpbuffer, nnumberofbytes, lpoverlapped.unwrap_or(core::mem::zeroed()) as _).ok() }
    }
    pub unsafe fn SetNotificationHandle(&self, hevent: Option<super::super::Foundation::HANDLE>) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetNotificationHandle)(windows_core::Interface::as_raw(self), hevent.unwrap_or(core::mem::zeroed()) as _).ok() }
    }
    pub unsafe fn GetNotificationData(&self, lpnotify: *mut STINOTIFY) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetNotificationData)(windows_core::Interface::as_raw(self), lpnotify as _).ok() }
    }
    pub unsafe fn GetLastErrorInfo(&self, plasterrorinfo: *mut _ERROR_INFOW) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetLastErrorInfo)(windows_core::Interface::as_raw(self), plasterrorinfo as _).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IStiUSD_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_System_Registry")]
    pub Initialize: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, u32, super::super::System::Registry::HKEY) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Registry"))]
    Initialize: usize,
    pub GetCapabilities: unsafe extern "system" fn(*mut core::ffi::c_void, *mut STI_USD_CAPS) -> windows_core::HRESULT,
    pub GetStatus: unsafe extern "system" fn(*mut core::ffi::c_void, *mut STI_DEVICE_STATUS) -> windows_core::HRESULT,
    pub DeviceReset: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Diagnostic: unsafe extern "system" fn(*mut core::ffi::c_void, *mut STI_DIAG) -> windows_core::HRESULT,
    pub Escape: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const core::ffi::c_void, u32, *mut core::ffi::c_void, u32, *mut u32) -> windows_core::HRESULT,
    pub GetLastError: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub LockDevice: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub UnLockDevice: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_IO")]
    pub RawReadData: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut u32, *const super::super::System::IO::OVERLAPPED) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_IO"))]
    RawReadData: usize,
    #[cfg(feature = "Win32_System_IO")]
    pub RawWriteData: unsafe extern "system" fn(*mut core::ffi::c_void, *const core::ffi::c_void, u32, *const super::super::System::IO::OVERLAPPED) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_IO"))]
    RawWriteData: usize,
    #[cfg(feature = "Win32_System_IO")]
    pub RawReadCommand: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut u32, *const super::super::System::IO::OVERLAPPED) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_IO"))]
    RawReadCommand: usize,
    #[cfg(feature = "Win32_System_IO")]
    pub RawWriteCommand: unsafe extern "system" fn(*mut core::ffi::c_void, *const core::ffi::c_void, u32, *const super::super::System::IO::OVERLAPPED) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_IO"))]
    RawWriteCommand: usize,
    pub SetNotificationHandle: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::HANDLE) -> windows_core::HRESULT,
    pub GetNotificationData: unsafe extern "system" fn(*mut core::ffi::c_void, *mut STINOTIFY) -> windows_core::HRESULT,
    pub GetLastErrorInfo: unsafe extern "system" fn(*mut core::ffi::c_void, *mut _ERROR_INFOW) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_System_IO", feature = "Win32_System_Registry"))]
pub trait IStiUSD_Impl: windows_core::IUnknownImpl {
    fn Initialize(&self, pheldcb: windows_core::Ref<IStiDeviceControl>, dwstiversion: u32, hparameterskey: super::super::System::Registry::HKEY) -> windows_core::Result<()>;
    fn GetCapabilities(&self) -> windows_core::Result<STI_USD_CAPS>;
    fn GetStatus(&self, pdevstatus: *mut STI_DEVICE_STATUS) -> windows_core::Result<()>;
    fn DeviceReset(&self) -> windows_core::Result<()>;
    fn Diagnostic(&self, pbuffer: *mut STI_DIAG) -> windows_core::Result<()>;
    fn Escape(&self, escapefunction: u32, lpindata: *const core::ffi::c_void, cbindatasize: u32, poutdata: *mut core::ffi::c_void, cboutdatasize: u32, pdwactualdata: *mut u32) -> windows_core::Result<()>;
    fn GetLastError(&self) -> windows_core::Result<u32>;
    fn LockDevice(&self) -> windows_core::Result<()>;
    fn UnLockDevice(&self) -> windows_core::Result<()>;
    fn RawReadData(&self, lpbuffer: *mut core::ffi::c_void, lpdwnumberofbytes: *mut u32, lpoverlapped: *const super::super::System::IO::OVERLAPPED) -> windows_core::Result<()>;
    fn RawWriteData(&self, lpbuffer: *const core::ffi::c_void, nnumberofbytes: u32, lpoverlapped: *const super::super::System::IO::OVERLAPPED) -> windows_core::Result<()>;
    fn RawReadCommand(&self, lpbuffer: *mut core::ffi::c_void, lpdwnumberofbytes: *mut u32, lpoverlapped: *const super::super::System::IO::OVERLAPPED) -> windows_core::Result<()>;
    fn RawWriteCommand(&self, lpbuffer: *const core::ffi::c_void, nnumberofbytes: u32, lpoverlapped: *const super::super::System::IO::OVERLAPPED) -> windows_core::Result<()>;
    fn SetNotificationHandle(&self, hevent: super::super::Foundation::HANDLE) -> windows_core::Result<()>;
    fn GetNotificationData(&self, lpnotify: *mut STINOTIFY) -> windows_core::Result<()>;
    fn GetLastErrorInfo(&self, plasterrorinfo: *mut _ERROR_INFOW) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_System_IO", feature = "Win32_System_Registry"))]
impl IStiUSD_Vtbl {
    pub const fn new<Identity: IStiUSD_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Initialize<Identity: IStiUSD_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pheldcb: *mut core::ffi::c_void, dwstiversion: u32, hparameterskey: super::super::System::Registry::HKEY) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IStiUSD_Impl::Initialize(this, core::mem::transmute_copy(&pheldcb), core::mem::transmute_copy(&dwstiversion), core::mem::transmute_copy(&hparameterskey)).into()
            }
        }
        unsafe extern "system" fn GetCapabilities<Identity: IStiUSD_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdevcaps: *mut STI_USD_CAPS) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IStiUSD_Impl::GetCapabilities(this) {
                    Ok(ok__) => {
                        pdevcaps.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetStatus<Identity: IStiUSD_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdevstatus: *mut STI_DEVICE_STATUS) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IStiUSD_Impl::GetStatus(this, core::mem::transmute_copy(&pdevstatus)).into()
            }
        }
        unsafe extern "system" fn DeviceReset<Identity: IStiUSD_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IStiUSD_Impl::DeviceReset(this).into()
            }
        }
        unsafe extern "system" fn Diagnostic<Identity: IStiUSD_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbuffer: *mut STI_DIAG) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IStiUSD_Impl::Diagnostic(this, core::mem::transmute_copy(&pbuffer)).into()
            }
        }
        unsafe extern "system" fn Escape<Identity: IStiUSD_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, escapefunction: u32, lpindata: *const core::ffi::c_void, cbindatasize: u32, poutdata: *mut core::ffi::c_void, cboutdatasize: u32, pdwactualdata: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IStiUSD_Impl::Escape(this, core::mem::transmute_copy(&escapefunction), core::mem::transmute_copy(&lpindata), core::mem::transmute_copy(&cbindatasize), core::mem::transmute_copy(&poutdata), core::mem::transmute_copy(&cboutdatasize), core::mem::transmute_copy(&pdwactualdata)).into()
            }
        }
        unsafe extern "system" fn GetLastError<Identity: IStiUSD_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwlastdeviceerror: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IStiUSD_Impl::GetLastError(this) {
                    Ok(ok__) => {
                        pdwlastdeviceerror.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn LockDevice<Identity: IStiUSD_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IStiUSD_Impl::LockDevice(this).into()
            }
        }
        unsafe extern "system" fn UnLockDevice<Identity: IStiUSD_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IStiUSD_Impl::UnLockDevice(this).into()
            }
        }
        unsafe extern "system" fn RawReadData<Identity: IStiUSD_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lpbuffer: *mut core::ffi::c_void, lpdwnumberofbytes: *mut u32, lpoverlapped: *const super::super::System::IO::OVERLAPPED) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IStiUSD_Impl::RawReadData(this, core::mem::transmute_copy(&lpbuffer), core::mem::transmute_copy(&lpdwnumberofbytes), core::mem::transmute_copy(&lpoverlapped)).into()
            }
        }
        unsafe extern "system" fn RawWriteData<Identity: IStiUSD_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lpbuffer: *const core::ffi::c_void, nnumberofbytes: u32, lpoverlapped: *const super::super::System::IO::OVERLAPPED) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IStiUSD_Impl::RawWriteData(this, core::mem::transmute_copy(&lpbuffer), core::mem::transmute_copy(&nnumberofbytes), core::mem::transmute_copy(&lpoverlapped)).into()
            }
        }
        unsafe extern "system" fn RawReadCommand<Identity: IStiUSD_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lpbuffer: *mut core::ffi::c_void, lpdwnumberofbytes: *mut u32, lpoverlapped: *const super::super::System::IO::OVERLAPPED) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IStiUSD_Impl::RawReadCommand(this, core::mem::transmute_copy(&lpbuffer), core::mem::transmute_copy(&lpdwnumberofbytes), core::mem::transmute_copy(&lpoverlapped)).into()
            }
        }
        unsafe extern "system" fn RawWriteCommand<Identity: IStiUSD_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lpbuffer: *const core::ffi::c_void, nnumberofbytes: u32, lpoverlapped: *const super::super::System::IO::OVERLAPPED) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IStiUSD_Impl::RawWriteCommand(this, core::mem::transmute_copy(&lpbuffer), core::mem::transmute_copy(&nnumberofbytes), core::mem::transmute_copy(&lpoverlapped)).into()
            }
        }
        unsafe extern "system" fn SetNotificationHandle<Identity: IStiUSD_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hevent: super::super::Foundation::HANDLE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IStiUSD_Impl::SetNotificationHandle(this, core::mem::transmute_copy(&hevent)).into()
            }
        }
        unsafe extern "system" fn GetNotificationData<Identity: IStiUSD_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lpnotify: *mut STINOTIFY) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IStiUSD_Impl::GetNotificationData(this, core::mem::transmute_copy(&lpnotify)).into()
            }
        }
        unsafe extern "system" fn GetLastErrorInfo<Identity: IStiUSD_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, plasterrorinfo: *mut _ERROR_INFOW) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IStiUSD_Impl::GetLastErrorInfo(this, core::mem::transmute_copy(&plasterrorinfo)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Initialize: Initialize::<Identity, OFFSET>,
            GetCapabilities: GetCapabilities::<Identity, OFFSET>,
            GetStatus: GetStatus::<Identity, OFFSET>,
            DeviceReset: DeviceReset::<Identity, OFFSET>,
            Diagnostic: Diagnostic::<Identity, OFFSET>,
            Escape: Escape::<Identity, OFFSET>,
            GetLastError: GetLastError::<Identity, OFFSET>,
            LockDevice: LockDevice::<Identity, OFFSET>,
            UnLockDevice: UnLockDevice::<Identity, OFFSET>,
            RawReadData: RawReadData::<Identity, OFFSET>,
            RawWriteData: RawWriteData::<Identity, OFFSET>,
            RawReadCommand: RawReadCommand::<Identity, OFFSET>,
            RawWriteCommand: RawWriteCommand::<Identity, OFFSET>,
            SetNotificationHandle: SetNotificationHandle::<Identity, OFFSET>,
            GetNotificationData: GetNotificationData::<Identity, OFFSET>,
            GetLastErrorInfo: GetLastErrorInfo::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IStiUSD as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_IO", feature = "Win32_System_Registry"))]
impl windows_core::RuntimeName for IStiUSD {}
windows_core::imp::define_interface!(IStillImageW, IStillImageW_Vtbl, 0x641bd880_2dc8_11d0_90ea_00aa0060f86c);
windows_core::imp::interface_hierarchy!(IStillImageW, windows_core::IUnknown);
impl IStillImageW {
    pub unsafe fn Initialize(&self, hinst: super::super::Foundation::HINSTANCE, dwversion: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Initialize)(windows_core::Interface::as_raw(self), hinst, dwversion).ok() }
    }
    pub unsafe fn GetDeviceList(&self, dwtype: u32, dwflags: u32, pdwitemsreturned: *mut u32, ppbuffer: *mut *mut core::ffi::c_void) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetDeviceList)(windows_core::Interface::as_raw(self), dwtype, dwflags, pdwitemsreturned as _, ppbuffer as _).ok() }
    }
    pub unsafe fn GetDeviceInfo<P0>(&self, pwszdevicename: P0, ppbuffer: *mut *mut core::ffi::c_void) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).GetDeviceInfo)(windows_core::Interface::as_raw(self), pwszdevicename.param().abi(), ppbuffer as _).ok() }
    }
    pub unsafe fn CreateDevice<P0, P3>(&self, pwszdevicename: P0, dwmode: u32, pdevice: *mut Option<IStiDevice>, punkouter: P3) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        P3: windows_core::Param<windows_core::IUnknown>,
    {
        unsafe { (windows_core::Interface::vtable(self).CreateDevice)(windows_core::Interface::as_raw(self), pwszdevicename.param().abi(), dwmode, core::mem::transmute(pdevice), punkouter.param().abi()).ok() }
    }
    pub unsafe fn GetDeviceValue<P0, P1>(&self, pwszdevicename: P0, pvaluename: P1, ptype: *mut u32, pdata: *mut u8, cbdata: *mut u32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        P1: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).GetDeviceValue)(windows_core::Interface::as_raw(self), pwszdevicename.param().abi(), pvaluename.param().abi(), ptype as _, pdata as _, cbdata as _).ok() }
    }
    pub unsafe fn SetDeviceValue<P0, P1>(&self, pwszdevicename: P0, pvaluename: P1, r#type: u32, pdata: &[u8]) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        P1: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetDeviceValue)(windows_core::Interface::as_raw(self), pwszdevicename.param().abi(), pvaluename.param().abi(), r#type, core::mem::transmute(pdata.as_ptr()), pdata.len().try_into().unwrap()).ok() }
    }
    pub unsafe fn GetSTILaunchInformation(&self, pwszdevicename: &mut [u16; 128], pdweventcode: Option<*mut u32>, pwszeventname: &mut [u16; 128]) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetSTILaunchInformation)(windows_core::Interface::as_raw(self), core::mem::transmute(pwszdevicename.as_ptr()), pdweventcode.unwrap_or(core::mem::zeroed()) as _, core::mem::transmute(pwszeventname.as_ptr())).ok() }
    }
    pub unsafe fn RegisterLaunchApplication<P0, P1>(&self, pwszappname: P0, pwszcommandline: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        P1: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).RegisterLaunchApplication)(windows_core::Interface::as_raw(self), pwszappname.param().abi(), pwszcommandline.param().abi()).ok() }
    }
    pub unsafe fn UnregisterLaunchApplication<P0>(&self, pwszappname: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).UnregisterLaunchApplication)(windows_core::Interface::as_raw(self), pwszappname.param().abi()).ok() }
    }
    pub unsafe fn EnableHwNotifications<P0>(&self, pwszdevicename: P0, bnewstate: bool) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).EnableHwNotifications)(windows_core::Interface::as_raw(self), pwszdevicename.param().abi(), bnewstate.into()).ok() }
    }
    pub unsafe fn GetHwNotificationState<P0>(&self, pwszdevicename: P0) -> windows_core::Result<windows_core::BOOL>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetHwNotificationState)(windows_core::Interface::as_raw(self), pwszdevicename.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn RefreshDeviceBus<P0>(&self, pwszdevicename: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).RefreshDeviceBus)(windows_core::Interface::as_raw(self), pwszdevicename.param().abi()).ok() }
    }
    pub unsafe fn LaunchApplicationForDevice<P0, P1>(&self, pwszdevicename: P0, pwszappname: P1, pstinotify: *const STINOTIFY) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        P1: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).LaunchApplicationForDevice)(windows_core::Interface::as_raw(self), pwszdevicename.param().abi(), pwszappname.param().abi(), pstinotify).ok() }
    }
    pub unsafe fn SetupDeviceParameters(&self, param0: *mut STI_DEVICE_INFORMATIONW) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetupDeviceParameters)(windows_core::Interface::as_raw(self), param0 as _).ok() }
    }
    pub unsafe fn WriteToErrorLog<P1>(&self, dwmessagetype: u32, pszmessage: P1) -> windows_core::Result<()>
    where
        P1: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).WriteToErrorLog)(windows_core::Interface::as_raw(self), dwmessagetype, pszmessage.param().abi()).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IStillImageW_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Initialize: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::HINSTANCE, u32) -> windows_core::HRESULT,
    pub GetDeviceList: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, *mut u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetDeviceInfo: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateDevice: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, u32, *mut *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetDeviceValue: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, windows_core::PCWSTR, *mut u32, *mut u8, *mut u32) -> windows_core::HRESULT,
    pub SetDeviceValue: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, windows_core::PCWSTR, u32, *const u8, u32) -> windows_core::HRESULT,
    pub GetSTILaunchInformation: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PWSTR, *mut u32, windows_core::PWSTR) -> windows_core::HRESULT,
    pub RegisterLaunchApplication: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, windows_core::PCWSTR) -> windows_core::HRESULT,
    pub UnregisterLaunchApplication: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR) -> windows_core::HRESULT,
    pub EnableHwNotifications: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, windows_core::BOOL) -> windows_core::HRESULT,
    pub GetHwNotificationState: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, *mut windows_core::BOOL) -> windows_core::HRESULT,
    pub RefreshDeviceBus: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR) -> windows_core::HRESULT,
    pub LaunchApplicationForDevice: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, windows_core::PCWSTR, *const STINOTIFY) -> windows_core::HRESULT,
    pub SetupDeviceParameters: unsafe extern "system" fn(*mut core::ffi::c_void, *mut STI_DEVICE_INFORMATIONW) -> windows_core::HRESULT,
    pub WriteToErrorLog: unsafe extern "system" fn(*mut core::ffi::c_void, u32, windows_core::PCWSTR) -> windows_core::HRESULT,
}
pub trait IStillImageW_Impl: windows_core::IUnknownImpl {
    fn Initialize(&self, hinst: super::super::Foundation::HINSTANCE, dwversion: u32) -> windows_core::Result<()>;
    fn GetDeviceList(&self, dwtype: u32, dwflags: u32, pdwitemsreturned: *mut u32, ppbuffer: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
    fn GetDeviceInfo(&self, pwszdevicename: &windows_core::PCWSTR, ppbuffer: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
    fn CreateDevice(&self, pwszdevicename: &windows_core::PCWSTR, dwmode: u32, pdevice: windows_core::OutRef<IStiDevice>, punkouter: windows_core::Ref<windows_core::IUnknown>) -> windows_core::Result<()>;
    fn GetDeviceValue(&self, pwszdevicename: &windows_core::PCWSTR, pvaluename: &windows_core::PCWSTR, ptype: *mut u32, pdata: *mut u8, cbdata: *mut u32) -> windows_core::Result<()>;
    fn SetDeviceValue(&self, pwszdevicename: &windows_core::PCWSTR, pvaluename: &windows_core::PCWSTR, r#type: u32, pdata: *const u8, cbdata: u32) -> windows_core::Result<()>;
    fn GetSTILaunchInformation(&self, pwszdevicename: windows_core::PWSTR, pdweventcode: *mut u32, pwszeventname: windows_core::PWSTR) -> windows_core::Result<()>;
    fn RegisterLaunchApplication(&self, pwszappname: &windows_core::PCWSTR, pwszcommandline: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn UnregisterLaunchApplication(&self, pwszappname: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn EnableHwNotifications(&self, pwszdevicename: &windows_core::PCWSTR, bnewstate: windows_core::BOOL) -> windows_core::Result<()>;
    fn GetHwNotificationState(&self, pwszdevicename: &windows_core::PCWSTR) -> windows_core::Result<windows_core::BOOL>;
    fn RefreshDeviceBus(&self, pwszdevicename: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn LaunchApplicationForDevice(&self, pwszdevicename: &windows_core::PCWSTR, pwszappname: &windows_core::PCWSTR, pstinotify: *const STINOTIFY) -> windows_core::Result<()>;
    fn SetupDeviceParameters(&self, param0: *mut STI_DEVICE_INFORMATIONW) -> windows_core::Result<()>;
    fn WriteToErrorLog(&self, dwmessagetype: u32, pszmessage: &windows_core::PCWSTR) -> windows_core::Result<()>;
}
impl IStillImageW_Vtbl {
    pub const fn new<Identity: IStillImageW_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Initialize<Identity: IStillImageW_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hinst: super::super::Foundation::HINSTANCE, dwversion: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IStillImageW_Impl::Initialize(this, core::mem::transmute_copy(&hinst), core::mem::transmute_copy(&dwversion)).into()
            }
        }
        unsafe extern "system" fn GetDeviceList<Identity: IStillImageW_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwtype: u32, dwflags: u32, pdwitemsreturned: *mut u32, ppbuffer: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IStillImageW_Impl::GetDeviceList(this, core::mem::transmute_copy(&dwtype), core::mem::transmute_copy(&dwflags), core::mem::transmute_copy(&pdwitemsreturned), core::mem::transmute_copy(&ppbuffer)).into()
            }
        }
        unsafe extern "system" fn GetDeviceInfo<Identity: IStillImageW_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwszdevicename: windows_core::PCWSTR, ppbuffer: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IStillImageW_Impl::GetDeviceInfo(this, core::mem::transmute(&pwszdevicename), core::mem::transmute_copy(&ppbuffer)).into()
            }
        }
        unsafe extern "system" fn CreateDevice<Identity: IStillImageW_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwszdevicename: windows_core::PCWSTR, dwmode: u32, pdevice: *mut *mut core::ffi::c_void, punkouter: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IStillImageW_Impl::CreateDevice(this, core::mem::transmute(&pwszdevicename), core::mem::transmute_copy(&dwmode), core::mem::transmute_copy(&pdevice), core::mem::transmute_copy(&punkouter)).into()
            }
        }
        unsafe extern "system" fn GetDeviceValue<Identity: IStillImageW_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwszdevicename: windows_core::PCWSTR, pvaluename: windows_core::PCWSTR, ptype: *mut u32, pdata: *mut u8, cbdata: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IStillImageW_Impl::GetDeviceValue(this, core::mem::transmute(&pwszdevicename), core::mem::transmute(&pvaluename), core::mem::transmute_copy(&ptype), core::mem::transmute_copy(&pdata), core::mem::transmute_copy(&cbdata)).into()
            }
        }
        unsafe extern "system" fn SetDeviceValue<Identity: IStillImageW_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwszdevicename: windows_core::PCWSTR, pvaluename: windows_core::PCWSTR, r#type: u32, pdata: *const u8, cbdata: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IStillImageW_Impl::SetDeviceValue(this, core::mem::transmute(&pwszdevicename), core::mem::transmute(&pvaluename), core::mem::transmute_copy(&r#type), core::mem::transmute_copy(&pdata), core::mem::transmute_copy(&cbdata)).into()
            }
        }
        unsafe extern "system" fn GetSTILaunchInformation<Identity: IStillImageW_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwszdevicename: windows_core::PWSTR, pdweventcode: *mut u32, pwszeventname: windows_core::PWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IStillImageW_Impl::GetSTILaunchInformation(this, core::mem::transmute_copy(&pwszdevicename), core::mem::transmute_copy(&pdweventcode), core::mem::transmute_copy(&pwszeventname)).into()
            }
        }
        unsafe extern "system" fn RegisterLaunchApplication<Identity: IStillImageW_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwszappname: windows_core::PCWSTR, pwszcommandline: windows_core::PCWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IStillImageW_Impl::RegisterLaunchApplication(this, core::mem::transmute(&pwszappname), core::mem::transmute(&pwszcommandline)).into()
            }
        }
        unsafe extern "system" fn UnregisterLaunchApplication<Identity: IStillImageW_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwszappname: windows_core::PCWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IStillImageW_Impl::UnregisterLaunchApplication(this, core::mem::transmute(&pwszappname)).into()
            }
        }
        unsafe extern "system" fn EnableHwNotifications<Identity: IStillImageW_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwszdevicename: windows_core::PCWSTR, bnewstate: windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IStillImageW_Impl::EnableHwNotifications(this, core::mem::transmute(&pwszdevicename), core::mem::transmute_copy(&bnewstate)).into()
            }
        }
        unsafe extern "system" fn GetHwNotificationState<Identity: IStillImageW_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwszdevicename: windows_core::PCWSTR, pbcurrentstate: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IStillImageW_Impl::GetHwNotificationState(this, core::mem::transmute(&pwszdevicename)) {
                    Ok(ok__) => {
                        pbcurrentstate.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn RefreshDeviceBus<Identity: IStillImageW_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwszdevicename: windows_core::PCWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IStillImageW_Impl::RefreshDeviceBus(this, core::mem::transmute(&pwszdevicename)).into()
            }
        }
        unsafe extern "system" fn LaunchApplicationForDevice<Identity: IStillImageW_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwszdevicename: windows_core::PCWSTR, pwszappname: windows_core::PCWSTR, pstinotify: *const STINOTIFY) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IStillImageW_Impl::LaunchApplicationForDevice(this, core::mem::transmute(&pwszdevicename), core::mem::transmute(&pwszappname), core::mem::transmute_copy(&pstinotify)).into()
            }
        }
        unsafe extern "system" fn SetupDeviceParameters<Identity: IStillImageW_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: *mut STI_DEVICE_INFORMATIONW) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IStillImageW_Impl::SetupDeviceParameters(this, core::mem::transmute_copy(&param0)).into()
            }
        }
        unsafe extern "system" fn WriteToErrorLog<Identity: IStillImageW_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwmessagetype: u32, pszmessage: windows_core::PCWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IStillImageW_Impl::WriteToErrorLog(this, core::mem::transmute_copy(&dwmessagetype), core::mem::transmute(&pszmessage)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Initialize: Initialize::<Identity, OFFSET>,
            GetDeviceList: GetDeviceList::<Identity, OFFSET>,
            GetDeviceInfo: GetDeviceInfo::<Identity, OFFSET>,
            CreateDevice: CreateDevice::<Identity, OFFSET>,
            GetDeviceValue: GetDeviceValue::<Identity, OFFSET>,
            SetDeviceValue: SetDeviceValue::<Identity, OFFSET>,
            GetSTILaunchInformation: GetSTILaunchInformation::<Identity, OFFSET>,
            RegisterLaunchApplication: RegisterLaunchApplication::<Identity, OFFSET>,
            UnregisterLaunchApplication: UnregisterLaunchApplication::<Identity, OFFSET>,
            EnableHwNotifications: EnableHwNotifications::<Identity, OFFSET>,
            GetHwNotificationState: GetHwNotificationState::<Identity, OFFSET>,
            RefreshDeviceBus: RefreshDeviceBus::<Identity, OFFSET>,
            LaunchApplicationForDevice: LaunchApplicationForDevice::<Identity, OFFSET>,
            SetupDeviceParameters: SetupDeviceParameters::<Identity, OFFSET>,
            WriteToErrorLog: WriteToErrorLog::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IStillImageW as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IStillImageW {}
pub const JC_DELETE: FAX_ENUM_JOB_COMMANDS = FAX_ENUM_JOB_COMMANDS(1i32);
pub const JC_PAUSE: FAX_ENUM_JOB_COMMANDS = FAX_ENUM_JOB_COMMANDS(2i32);
pub const JC_RESUME: FAX_ENUM_JOB_COMMANDS = FAX_ENUM_JOB_COMMANDS(3i32);
pub const JC_UNKNOWN: FAX_ENUM_JOB_COMMANDS = FAX_ENUM_JOB_COMMANDS(0i32);
pub const JSA_DISCOUNT_PERIOD: FAX_ENUM_JOB_SEND_ATTRIBUTES = FAX_ENUM_JOB_SEND_ATTRIBUTES(2i32);
pub const JSA_NOW: FAX_ENUM_JOB_SEND_ATTRIBUTES = FAX_ENUM_JOB_SEND_ATTRIBUTES(0i32);
pub const JSA_SPECIFIC_TIME: FAX_ENUM_JOB_SEND_ATTRIBUTES = FAX_ENUM_JOB_SEND_ATTRIBUTES(1i32);
pub const JS_DELETING: u32 = 2u32;
pub const JS_FAILED: u32 = 4u32;
pub const JS_INPROGRESS: u32 = 1u32;
pub const JS_NOLINE: u32 = 16u32;
pub const JS_PAUSED: u32 = 8u32;
pub const JS_PENDING: u32 = 0u32;
pub const JS_RETRIES_EXCEEDED: u32 = 64u32;
pub const JS_RETRYING: u32 = 32u32;
pub const JT_FAIL_RECEIVE: u32 = 4u32;
pub const JT_RECEIVE: u32 = 2u32;
pub const JT_ROUTING: u32 = 3u32;
pub const JT_SEND: u32 = 1u32;
pub const JT_UNKNOWN: u32 = 0u32;
pub const MAX_NOTIFICATION_DATA: u32 = 64u32;
pub const MS_FAXROUTE_EMAIL_GUID: windows_core::PCWSTR = windows_core::w!("{6bbf7bfe-9af2-11d0-abf7-00c04fd91a4e}");
pub const MS_FAXROUTE_FOLDER_GUID: windows_core::PCWSTR = windows_core::w!("{92041a90-9af2-11d0-abf7-00c04fd91a4e}");
pub const MS_FAXROUTE_PRINTING_GUID: windows_core::PCWSTR = windows_core::w!("{aec1b37c-9af2-11d0-abf7-00c04fd91a4e}");
pub type PFAXABORT = Option<unsafe extern "system" fn(faxhandle: super::super::Foundation::HANDLE, jobid: u32) -> windows_core::BOOL>;
pub type PFAXACCESSCHECK = Option<unsafe extern "system" fn(faxhandle: super::super::Foundation::HANDLE, accessmask: u32) -> windows_core::BOOL>;
pub type PFAXCLOSE = Option<unsafe extern "system" fn(faxhandle: super::super::Foundation::HANDLE) -> windows_core::BOOL>;
pub type PFAXCOMPLETEJOBPARAMSA = Option<unsafe extern "system" fn(jobparams: *mut *mut FAX_JOB_PARAMA, coverpageinfo: *mut *mut FAX_COVERPAGE_INFOA) -> windows_core::BOOL>;
pub type PFAXCOMPLETEJOBPARAMSW = Option<unsafe extern "system" fn(jobparams: *mut *mut FAX_JOB_PARAMW, coverpageinfo: *mut *mut FAX_COVERPAGE_INFOW) -> windows_core::BOOL>;
pub type PFAXCONNECTFAXSERVERA = Option<unsafe extern "system" fn(machinename: windows_core::PCSTR, faxhandle: *mut super::super::Foundation::HANDLE) -> windows_core::BOOL>;
pub type PFAXCONNECTFAXSERVERW = Option<unsafe extern "system" fn(machinename: windows_core::PCWSTR, faxhandle: *mut super::super::Foundation::HANDLE) -> windows_core::BOOL>;
pub type PFAXDEVABORTOPERATION = Option<unsafe extern "system" fn(param0: super::super::Foundation::HANDLE) -> windows_core::BOOL>;
#[cfg(feature = "Win32_UI_Controls")]
pub type PFAXDEVCONFIGURE = Option<unsafe extern "system" fn(param0: *mut super::super::UI::Controls::HPROPSHEETPAGE) -> windows_core::BOOL>;
pub type PFAXDEVENDJOB = Option<unsafe extern "system" fn(param0: super::super::Foundation::HANDLE) -> windows_core::BOOL>;
pub type PFAXDEVINITIALIZE = Option<unsafe extern "system" fn(param0: u32, param1: super::super::Foundation::HANDLE, param2: *mut PFAX_LINECALLBACK, param3: PFAX_SERVICE_CALLBACK) -> windows_core::BOOL>;
pub type PFAXDEVRECEIVE = Option<unsafe extern "system" fn(param0: super::super::Foundation::HANDLE, param1: u32, param2: *mut FAX_RECEIVE) -> windows_core::BOOL>;
pub type PFAXDEVREPORTSTATUS = Option<unsafe extern "system" fn(param0: super::super::Foundation::HANDLE, param1: *mut FAX_DEV_STATUS, param2: u32, param3: *mut u32) -> windows_core::BOOL>;
pub type PFAXDEVSEND = Option<unsafe extern "system" fn(param0: super::super::Foundation::HANDLE, param1: *mut FAX_SEND, param2: PFAX_SEND_CALLBACK) -> windows_core::BOOL>;
pub type PFAXDEVSHUTDOWN = Option<unsafe extern "system" fn() -> windows_core::HRESULT>;
pub type PFAXDEVSTARTJOB = Option<unsafe extern "system" fn(param0: u32, param1: u32, param2: *mut super::super::Foundation::HANDLE, param3: super::super::Foundation::HANDLE, param4: usize) -> windows_core::BOOL>;
pub type PFAXDEVVIRTUALDEVICECREATION = Option<unsafe extern "system" fn(devicecount: *mut u32, devicenameprefix: windows_core::PWSTR, deviceidprefix: *mut u32, completionport: super::super::Foundation::HANDLE, completionkey: usize) -> windows_core::BOOL>;
pub type PFAXENABLEROUTINGMETHODA = Option<unsafe extern "system" fn(faxporthandle: super::super::Foundation::HANDLE, routingguid: windows_core::PCSTR, enabled: windows_core::BOOL) -> windows_core::BOOL>;
pub type PFAXENABLEROUTINGMETHODW = Option<unsafe extern "system" fn(faxporthandle: super::super::Foundation::HANDLE, routingguid: windows_core::PCWSTR, enabled: windows_core::BOOL) -> windows_core::BOOL>;
pub type PFAXENUMGLOBALROUTINGINFOA = Option<unsafe extern "system" fn(faxhandle: super::super::Foundation::HANDLE, routinginfo: *mut *mut FAX_GLOBAL_ROUTING_INFOA, methodsreturned: *mut u32) -> windows_core::BOOL>;
pub type PFAXENUMGLOBALROUTINGINFOW = Option<unsafe extern "system" fn(faxhandle: super::super::Foundation::HANDLE, routinginfo: *mut *mut FAX_GLOBAL_ROUTING_INFOW, methodsreturned: *mut u32) -> windows_core::BOOL>;
pub type PFAXENUMJOBSA = Option<unsafe extern "system" fn(faxhandle: super::super::Foundation::HANDLE, jobentry: *mut *mut FAX_JOB_ENTRYA, jobsreturned: *mut u32) -> windows_core::BOOL>;
pub type PFAXENUMJOBSW = Option<unsafe extern "system" fn(faxhandle: super::super::Foundation::HANDLE, jobentry: *mut *mut FAX_JOB_ENTRYW, jobsreturned: *mut u32) -> windows_core::BOOL>;
pub type PFAXENUMPORTSA = Option<unsafe extern "system" fn(faxhandle: super::super::Foundation::HANDLE, portinfo: *mut *mut FAX_PORT_INFOA, portsreturned: *mut u32) -> windows_core::BOOL>;
pub type PFAXENUMPORTSW = Option<unsafe extern "system" fn(faxhandle: super::super::Foundation::HANDLE, portinfo: *mut *mut FAX_PORT_INFOW, portsreturned: *mut u32) -> windows_core::BOOL>;
pub type PFAXENUMROUTINGMETHODSA = Option<unsafe extern "system" fn(faxporthandle: super::super::Foundation::HANDLE, routingmethod: *mut *mut FAX_ROUTING_METHODA, methodsreturned: *mut u32) -> windows_core::BOOL>;
pub type PFAXENUMROUTINGMETHODSW = Option<unsafe extern "system" fn(faxporthandle: super::super::Foundation::HANDLE, routingmethod: *mut *mut FAX_ROUTING_METHODW, methodsreturned: *mut u32) -> windows_core::BOOL>;
pub type PFAXFREEBUFFER = Option<unsafe extern "system" fn(buffer: *mut core::ffi::c_void)>;
pub type PFAXGETCONFIGURATIONA = Option<unsafe extern "system" fn(faxhandle: super::super::Foundation::HANDLE, faxconfig: *mut *mut FAX_CONFIGURATIONA) -> windows_core::BOOL>;
pub type PFAXGETCONFIGURATIONW = Option<unsafe extern "system" fn(faxhandle: super::super::Foundation::HANDLE, faxconfig: *mut *mut FAX_CONFIGURATIONW) -> windows_core::BOOL>;
pub type PFAXGETDEVICESTATUSA = Option<unsafe extern "system" fn(faxporthandle: super::super::Foundation::HANDLE, devicestatus: *mut *mut FAX_DEVICE_STATUSA) -> windows_core::BOOL>;
pub type PFAXGETDEVICESTATUSW = Option<unsafe extern "system" fn(faxporthandle: super::super::Foundation::HANDLE, devicestatus: *mut *mut FAX_DEVICE_STATUSW) -> windows_core::BOOL>;
pub type PFAXGETJOBA = Option<unsafe extern "system" fn(faxhandle: super::super::Foundation::HANDLE, jobid: u32, jobentry: *mut *mut FAX_JOB_ENTRYA) -> windows_core::BOOL>;
pub type PFAXGETJOBW = Option<unsafe extern "system" fn(faxhandle: super::super::Foundation::HANDLE, jobid: u32, jobentry: *mut *mut FAX_JOB_ENTRYW) -> windows_core::BOOL>;
pub type PFAXGETLOGGINGCATEGORIESA = Option<unsafe extern "system" fn(faxhandle: super::super::Foundation::HANDLE, categories: *mut *mut FAX_LOG_CATEGORYA, numbercategories: *mut u32) -> windows_core::BOOL>;
pub type PFAXGETLOGGINGCATEGORIESW = Option<unsafe extern "system" fn(faxhandle: super::super::Foundation::HANDLE, categories: *mut *mut FAX_LOG_CATEGORYW, numbercategories: *mut u32) -> windows_core::BOOL>;
pub type PFAXGETPAGEDATA = Option<unsafe extern "system" fn(faxhandle: super::super::Foundation::HANDLE, jobid: u32, buffer: *mut *mut u8, buffersize: *mut u32, imagewidth: *mut u32, imageheight: *mut u32) -> windows_core::BOOL>;
pub type PFAXGETPORTA = Option<unsafe extern "system" fn(faxporthandle: super::super::Foundation::HANDLE, portinfo: *mut *mut FAX_PORT_INFOA) -> windows_core::BOOL>;
pub type PFAXGETPORTW = Option<unsafe extern "system" fn(faxporthandle: super::super::Foundation::HANDLE, portinfo: *mut *mut FAX_PORT_INFOW) -> windows_core::BOOL>;
pub type PFAXGETROUTINGINFOA = Option<unsafe extern "system" fn(faxporthandle: super::super::Foundation::HANDLE, routingguid: windows_core::PCSTR, routinginfobuffer: *mut *mut u8, routinginfobuffersize: *mut u32) -> windows_core::BOOL>;
pub type PFAXGETROUTINGINFOW = Option<unsafe extern "system" fn(faxporthandle: super::super::Foundation::HANDLE, routingguid: windows_core::PCWSTR, routinginfobuffer: *mut *mut u8, routinginfobuffersize: *mut u32) -> windows_core::BOOL>;
pub type PFAXINITIALIZEEVENTQUEUE = Option<unsafe extern "system" fn(faxhandle: super::super::Foundation::HANDLE, completionport: super::super::Foundation::HANDLE, completionkey: usize, hwnd: super::super::Foundation::HWND, messagestart: u32) -> windows_core::BOOL>;
pub type PFAXOPENPORT = Option<unsafe extern "system" fn(faxhandle: super::super::Foundation::HANDLE, deviceid: u32, flags: u32, faxporthandle: *mut super::super::Foundation::HANDLE) -> windows_core::BOOL>;
#[cfg(feature = "Win32_Graphics_Gdi")]
pub type PFAXPRINTCOVERPAGEA = Option<unsafe extern "system" fn(faxcontextinfo: *const FAX_CONTEXT_INFOA, coverpageinfo: *const FAX_COVERPAGE_INFOA) -> windows_core::BOOL>;
#[cfg(feature = "Win32_Graphics_Gdi")]
pub type PFAXPRINTCOVERPAGEW = Option<unsafe extern "system" fn(faxcontextinfo: *const FAX_CONTEXT_INFOW, coverpageinfo: *const FAX_COVERPAGE_INFOW) -> windows_core::BOOL>;
pub type PFAXREGISTERROUTINGEXTENSIONW = Option<unsafe extern "system" fn(faxhandle: super::super::Foundation::HANDLE, extensionname: windows_core::PCWSTR, friendlyname: windows_core::PCWSTR, imagename: windows_core::PCWSTR, callback: PFAX_ROUTING_INSTALLATION_CALLBACKW, context: *const core::ffi::c_void) -> windows_core::BOOL>;
pub type PFAXREGISTERSERVICEPROVIDERW = Option<unsafe extern "system" fn(deviceprovider: windows_core::PCWSTR, friendlyname: windows_core::PCWSTR, imagename: windows_core::PCWSTR, tspname: windows_core::PCWSTR) -> windows_core::BOOL>;
pub type PFAXROUTEADDFILE = Option<unsafe extern "system" fn(jobid: u32, filename: windows_core::PCWSTR, guid: *mut windows_core::GUID) -> i32>;
pub type PFAXROUTEDELETEFILE = Option<unsafe extern "system" fn(jobid: u32, filename: windows_core::PCWSTR) -> i32>;
pub type PFAXROUTEDEVICECHANGENOTIFICATION = Option<unsafe extern "system" fn(param0: u32, param1: windows_core::BOOL) -> windows_core::BOOL>;
pub type PFAXROUTEDEVICEENABLE = Option<unsafe extern "system" fn(param0: windows_core::PCWSTR, param1: u32, param2: i32) -> windows_core::BOOL>;
pub type PFAXROUTEENUMFILE = Option<unsafe extern "system" fn(jobid: u32, guidowner: *mut windows_core::GUID, guidcaller: *mut windows_core::GUID, filename: windows_core::PCWSTR, context: *mut core::ffi::c_void) -> windows_core::BOOL>;
pub type PFAXROUTEENUMFILES = Option<unsafe extern "system" fn(jobid: u32, guid: *mut windows_core::GUID, fileenumerator: PFAXROUTEENUMFILE, context: *mut core::ffi::c_void) -> windows_core::BOOL>;
pub type PFAXROUTEGETFILE = Option<unsafe extern "system" fn(jobid: u32, index: u32, filenamebuffer: windows_core::PWSTR, requiredsize: *mut u32) -> windows_core::BOOL>;
pub type PFAXROUTEGETROUTINGINFO = Option<unsafe extern "system" fn(param0: windows_core::PCWSTR, param1: u32, param2: *mut u8, param3: *mut u32) -> windows_core::BOOL>;
pub type PFAXROUTEINITIALIZE = Option<unsafe extern "system" fn(param0: super::super::Foundation::HANDLE, param1: *mut FAX_ROUTE_CALLBACKROUTINES) -> windows_core::BOOL>;
pub type PFAXROUTEMETHOD = Option<unsafe extern "system" fn(param0: *const FAX_ROUTE, param1: *mut *mut core::ffi::c_void, param2: *mut u32) -> windows_core::BOOL>;
pub type PFAXROUTEMODIFYROUTINGDATA = Option<unsafe extern "system" fn(jobid: u32, routingguid: windows_core::PCWSTR, routingdata: *mut u8, routingdatasize: u32) -> windows_core::BOOL>;
pub type PFAXROUTESETROUTINGINFO = Option<unsafe extern "system" fn(param0: windows_core::PCWSTR, param1: u32, param2: *const u8, param3: u32) -> windows_core::BOOL>;
pub type PFAXSENDDOCUMENTA = Option<unsafe extern "system" fn(faxhandle: super::super::Foundation::HANDLE, filename: windows_core::PCSTR, jobparams: *const FAX_JOB_PARAMA, coverpageinfo: *const FAX_COVERPAGE_INFOA, faxjobid: *mut u32) -> windows_core::BOOL>;
pub type PFAXSENDDOCUMENTFORBROADCASTA = Option<unsafe extern "system" fn(faxhandle: super::super::Foundation::HANDLE, filename: windows_core::PCSTR, faxjobid: *mut u32, faxrecipientcallback: PFAX_RECIPIENT_CALLBACKA, context: *const core::ffi::c_void) -> windows_core::BOOL>;
pub type PFAXSENDDOCUMENTFORBROADCASTW = Option<unsafe extern "system" fn(faxhandle: super::super::Foundation::HANDLE, filename: windows_core::PCWSTR, faxjobid: *mut u32, faxrecipientcallback: PFAX_RECIPIENT_CALLBACKW, context: *const core::ffi::c_void) -> windows_core::BOOL>;
pub type PFAXSENDDOCUMENTW = Option<unsafe extern "system" fn(faxhandle: super::super::Foundation::HANDLE, filename: windows_core::PCWSTR, jobparams: *const FAX_JOB_PARAMW, coverpageinfo: *const FAX_COVERPAGE_INFOW, faxjobid: *mut u32) -> windows_core::BOOL>;
pub type PFAXSETCONFIGURATIONA = Option<unsafe extern "system" fn(faxhandle: super::super::Foundation::HANDLE, faxconfig: *const FAX_CONFIGURATIONA) -> windows_core::BOOL>;
pub type PFAXSETCONFIGURATIONW = Option<unsafe extern "system" fn(faxhandle: super::super::Foundation::HANDLE, faxconfig: *const FAX_CONFIGURATIONW) -> windows_core::BOOL>;
pub type PFAXSETGLOBALROUTINGINFOA = Option<unsafe extern "system" fn(faxporthandle: super::super::Foundation::HANDLE, routinginfo: *const FAX_GLOBAL_ROUTING_INFOA) -> windows_core::BOOL>;
pub type PFAXSETGLOBALROUTINGINFOW = Option<unsafe extern "system" fn(faxporthandle: super::super::Foundation::HANDLE, routinginfo: *const FAX_GLOBAL_ROUTING_INFOW) -> windows_core::BOOL>;
pub type PFAXSETJOBA = Option<unsafe extern "system" fn(faxhandle: super::super::Foundation::HANDLE, jobid: u32, command: u32, jobentry: *const FAX_JOB_ENTRYA) -> windows_core::BOOL>;
pub type PFAXSETJOBW = Option<unsafe extern "system" fn(faxhandle: super::super::Foundation::HANDLE, jobid: u32, command: u32, jobentry: *const FAX_JOB_ENTRYW) -> windows_core::BOOL>;
pub type PFAXSETLOGGINGCATEGORIESA = Option<unsafe extern "system" fn(faxhandle: super::super::Foundation::HANDLE, categories: *const FAX_LOG_CATEGORYA, numbercategories: u32) -> windows_core::BOOL>;
pub type PFAXSETLOGGINGCATEGORIESW = Option<unsafe extern "system" fn(faxhandle: super::super::Foundation::HANDLE, categories: *const FAX_LOG_CATEGORYW, numbercategories: u32) -> windows_core::BOOL>;
pub type PFAXSETPORTA = Option<unsafe extern "system" fn(faxporthandle: super::super::Foundation::HANDLE, portinfo: *const FAX_PORT_INFOA) -> windows_core::BOOL>;
pub type PFAXSETPORTW = Option<unsafe extern "system" fn(faxporthandle: super::super::Foundation::HANDLE, portinfo: *const FAX_PORT_INFOW) -> windows_core::BOOL>;
pub type PFAXSETROUTINGINFOA = Option<unsafe extern "system" fn(faxporthandle: super::super::Foundation::HANDLE, routingguid: windows_core::PCSTR, routinginfobuffer: *const u8, routinginfobuffersize: u32) -> windows_core::BOOL>;
pub type PFAXSETROUTINGINFOW = Option<unsafe extern "system" fn(faxporthandle: super::super::Foundation::HANDLE, routingguid: windows_core::PCWSTR, routinginfobuffer: *const u8, routinginfobuffersize: u32) -> windows_core::BOOL>;
#[cfg(feature = "Win32_Graphics_Gdi")]
pub type PFAXSTARTPRINTJOBA = Option<unsafe extern "system" fn(printername: windows_core::PCSTR, printinfo: *const FAX_PRINT_INFOA, faxjobid: *mut u32, faxcontextinfo: *mut FAX_CONTEXT_INFOA) -> windows_core::BOOL>;
#[cfg(feature = "Win32_Graphics_Gdi")]
pub type PFAXSTARTPRINTJOBW = Option<unsafe extern "system" fn(printername: windows_core::PCWSTR, printinfo: *const FAX_PRINT_INFOW, faxjobid: *mut u32, faxcontextinfo: *mut FAX_CONTEXT_INFOW) -> windows_core::BOOL>;
pub type PFAXUNREGISTERSERVICEPROVIDERW = Option<unsafe extern "system" fn(deviceprovider: windows_core::PCWSTR) -> windows_core::BOOL>;
pub type PFAX_EXT_CONFIG_CHANGE = Option<unsafe extern "system" fn(param0: u32, param1: windows_core::PCWSTR, param2: *mut u8, param3: u32) -> windows_core::HRESULT>;
pub type PFAX_EXT_FREE_BUFFER = Option<unsafe extern "system" fn(param0: *mut core::ffi::c_void)>;
pub type PFAX_EXT_GET_DATA = Option<unsafe extern "system" fn(param0: u32, param1: FAX_ENUM_DEVICE_ID_SOURCE, param2: windows_core::PCWSTR, param3: *mut *mut u8, param4: *mut u32) -> u32>;
pub type PFAX_EXT_INITIALIZE_CONFIG = Option<unsafe extern "system" fn(param0: PFAX_EXT_GET_DATA, param1: PFAX_EXT_SET_DATA, param2: PFAX_EXT_REGISTER_FOR_EVENTS, param3: PFAX_EXT_UNREGISTER_FOR_EVENTS, param4: PFAX_EXT_FREE_BUFFER) -> windows_core::HRESULT>;
pub type PFAX_EXT_REGISTER_FOR_EVENTS = Option<unsafe extern "system" fn(param0: super::super::Foundation::HINSTANCE, param1: u32, param2: FAX_ENUM_DEVICE_ID_SOURCE, param3: windows_core::PCWSTR, param4: PFAX_EXT_CONFIG_CHANGE) -> super::super::Foundation::HANDLE>;
pub type PFAX_EXT_SET_DATA = Option<unsafe extern "system" fn(param0: super::super::Foundation::HINSTANCE, param1: u32, param2: FAX_ENUM_DEVICE_ID_SOURCE, param3: windows_core::PCWSTR, param4: *mut u8, param5: u32) -> u32>;
pub type PFAX_EXT_UNREGISTER_FOR_EVENTS = Option<unsafe extern "system" fn(param0: super::super::Foundation::HANDLE) -> u32>;
pub type PFAX_LINECALLBACK = Option<unsafe extern "system" fn(faxhandle: super::super::Foundation::HANDLE, hdevice: u32, dwmessage: u32, dwinstance: usize, dwparam1: usize, dwparam2: usize, dwparam3: usize)>;
pub type PFAX_RECIPIENT_CALLBACKA = Option<unsafe extern "system" fn(faxhandle: super::super::Foundation::HANDLE, recipientnumber: u32, context: *const core::ffi::c_void, jobparams: *mut FAX_JOB_PARAMA, coverpageinfo: *mut FAX_COVERPAGE_INFOA) -> windows_core::BOOL>;
pub type PFAX_RECIPIENT_CALLBACKW = Option<unsafe extern "system" fn(faxhandle: super::super::Foundation::HANDLE, recipientnumber: u32, context: *const core::ffi::c_void, jobparams: *mut FAX_JOB_PARAMW, coverpageinfo: *mut FAX_COVERPAGE_INFOW) -> windows_core::BOOL>;
pub type PFAX_ROUTING_INSTALLATION_CALLBACKW = Option<unsafe extern "system" fn(faxhandle: super::super::Foundation::HANDLE, context: *const core::ffi::c_void, methodname: windows_core::PWSTR, friendlyname: windows_core::PWSTR, functionname: windows_core::PWSTR, guid: windows_core::PWSTR) -> windows_core::BOOL>;
pub type PFAX_SEND_CALLBACK = Option<unsafe extern "system" fn(faxhandle: super::super::Foundation::HANDLE, callhandle: u32, reserved1: u32, reserved2: u32) -> windows_core::BOOL>;
pub type PFAX_SERVICE_CALLBACK = Option<unsafe extern "system" fn(faxhandle: super::super::Foundation::HANDLE, deviceid: u32, param1: usize, param2: usize, param3: usize) -> windows_core::BOOL>;
pub const PORT_OPEN_MODIFY: FAX_ENUM_PORT_OPEN_TYPE = FAX_ENUM_PORT_OPEN_TYPE(2i32);
pub const PORT_OPEN_QUERY: FAX_ENUM_PORT_OPEN_TYPE = FAX_ENUM_PORT_OPEN_TYPE(1i32);
pub const QUERY_STATUS: FAXROUTE_ENABLE = FAXROUTE_ENABLE(-1i32);
pub const REGSTR_VAL_BAUDRATE: windows_core::PCWSTR = windows_core::w!("BaudRate");
pub const REGSTR_VAL_BAUDRATE_A: windows_core::PCSTR = windows_core::s!("BaudRate");
pub const REGSTR_VAL_DATA_W: windows_core::PCWSTR = windows_core::w!("DeviceData");
pub const REGSTR_VAL_DEVICESUBTYPE_W: windows_core::PCWSTR = windows_core::w!("DeviceSubType");
pub const REGSTR_VAL_DEVICETYPE_W: windows_core::PCWSTR = windows_core::w!("DeviceType");
pub const REGSTR_VAL_DEVICE_NAME_W: windows_core::PCWSTR = windows_core::w!("DriverDesc");
pub const REGSTR_VAL_DEV_NAME_W: windows_core::PCWSTR = windows_core::w!("DeviceName");
pub const REGSTR_VAL_DRIVER_DESC_W: windows_core::PCWSTR = windows_core::w!("DriverDesc");
pub const REGSTR_VAL_FRIENDLY_NAME_W: windows_core::PCWSTR = windows_core::w!("FriendlyName");
pub const REGSTR_VAL_GENERIC_CAPS_W: windows_core::PCWSTR = windows_core::w!("Capabilities");
pub const REGSTR_VAL_GUID: windows_core::PCWSTR = windows_core::w!("GUID");
pub const REGSTR_VAL_GUID_W: windows_core::PCWSTR = windows_core::w!("GUID");
pub const REGSTR_VAL_HARDWARE: windows_core::PCWSTR = windows_core::w!("HardwareConfig");
pub const REGSTR_VAL_HARDWARE_W: windows_core::PCWSTR = windows_core::w!("HardwareConfig");
pub const REGSTR_VAL_LAUNCHABLE: windows_core::PCWSTR = windows_core::w!("Launchable");
pub const REGSTR_VAL_LAUNCHABLE_W: windows_core::PCWSTR = windows_core::w!("Launchable");
pub const REGSTR_VAL_LAUNCH_APPS: windows_core::PCWSTR = windows_core::w!("LaunchApplications");
pub const REGSTR_VAL_LAUNCH_APPS_W: windows_core::PCWSTR = windows_core::w!("LaunchApplications");
pub const REGSTR_VAL_SHUTDOWNDELAY: windows_core::PCWSTR = windows_core::w!("ShutdownIfUnusedDelay");
pub const REGSTR_VAL_SHUTDOWNDELAY_W: windows_core::PCWSTR = windows_core::w!("ShutdownIfUnusedDelay");
pub const REGSTR_VAL_TYPE_W: windows_core::PCWSTR = windows_core::w!("Type");
pub const REGSTR_VAL_VENDOR_NAME_W: windows_core::PCWSTR = windows_core::w!("Vendor");
pub const SEND_TO_FAX_RECIPIENT_ATTACHMENT: SendToMode = SendToMode(0i32);
pub const STATUS_DISABLE: FAXROUTE_ENABLE = FAXROUTE_ENABLE(0i32);
pub const STATUS_ENABLE: FAXROUTE_ENABLE = FAXROUTE_ENABLE(1i32);
pub const STIEDFL_ALLDEVICES: u32 = 0u32;
pub const STIEDFL_ATTACHEDONLY: u32 = 1u32;
pub const STIERR_ALREADY_INITIALIZED: windows_core::HRESULT = windows_core::HRESULT(0x800704DF_u32 as _);
pub const STIERR_BADDRIVER: windows_core::HRESULT = windows_core::HRESULT(0x80070077_u32 as _);
pub const STIERR_BETA_VERSION: windows_core::HRESULT = windows_core::HRESULT(0x80070481_u32 as _);
pub const STIERR_DEVICENOTREG: i32 = -2147221164i32;
pub const STIERR_DEVICE_LOCKED: windows_core::HRESULT = windows_core::HRESULT(0x80070021_u32 as _);
pub const STIERR_DEVICE_NOTREADY: windows_core::HRESULT = windows_core::HRESULT(0x80070015_u32 as _);
pub const STIERR_GENERIC: i32 = -2147467259i32;
pub const STIERR_HANDLEEXISTS: windows_core::HRESULT = windows_core::HRESULT(0x800700B7_u32 as _);
pub const STIERR_INVALID_DEVICE_NAME: windows_core::HRESULT = windows_core::HRESULT(0x8007007B_u32 as _);
pub const STIERR_INVALID_HW_TYPE: windows_core::HRESULT = windows_core::HRESULT(0x8007000D_u32 as _);
pub const STIERR_INVALID_PARAM: i32 = -2147024809i32;
pub const STIERR_NEEDS_LOCK: windows_core::HRESULT = windows_core::HRESULT(0x8007009E_u32 as _);
pub const STIERR_NOEVENTS: windows_core::HRESULT = windows_core::HRESULT(0x80070103_u32 as _);
pub const STIERR_NOINTERFACE: i32 = -2147467262i32;
pub const STIERR_NOTINITIALIZED: i32 = -2147024891i32;
pub const STIERR_NOT_INITIALIZED: windows_core::HRESULT = windows_core::HRESULT(0x80070015_u32 as _);
pub const STIERR_OBJECTNOTFOUND: windows_core::HRESULT = windows_core::HRESULT(0x80070002_u32 as _);
pub const STIERR_OLD_VERSION: windows_core::HRESULT = windows_core::HRESULT(0x8007047E_u32 as _);
pub const STIERR_OUTOFMEMORY: i32 = -2147024882i32;
pub const STIERR_READONLY: i32 = -2147024891i32;
pub const STIERR_SHARING_VIOLATION: windows_core::HRESULT = windows_core::HRESULT(0x80070020_u32 as _);
pub const STIERR_UNSUPPORTED: i32 = -2147467263i32;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct STINOTIFY {
    pub dwSize: u32,
    pub guidNotificationCode: windows_core::GUID,
    pub abNotificationData: [u8; 64],
}
impl Default for STINOTIFY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct STISUBSCRIBE {
    pub dwSize: u32,
    pub dwFlags: u32,
    pub dwFilter: u32,
    pub hWndNotify: super::super::Foundation::HWND,
    pub hEvent: super::super::Foundation::HANDLE,
    pub uiNotificationMessage: u32,
}
pub const STI_ADD_DEVICE_BROADCAST_ACTION: windows_core::PCSTR = windows_core::s!("Arrival");
pub const STI_ADD_DEVICE_BROADCAST_STRING: windows_core::PCSTR = windows_core::s!("STI\\");
pub const STI_CHANGENOEFFECT: i32 = 1i32;
pub const STI_DEVICE_CREATE_BOTH: u32 = 3u32;
pub const STI_DEVICE_CREATE_DATA: u32 = 2u32;
pub const STI_DEVICE_CREATE_FOR_MONITOR: u32 = 16777216u32;
pub const STI_DEVICE_CREATE_MASK: u32 = 65535u32;
pub const STI_DEVICE_CREATE_STATUS: u32 = 1u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct STI_DEVICE_INFORMATIONW {
    pub dwSize: u32,
    pub DeviceType: u32,
    pub szDeviceInternalName: [u16; 128],
    pub DeviceCapabilitiesA: STI_DEV_CAPS,
    pub dwHardwareConfiguration: u32,
    pub pszVendorDescription: windows_core::PWSTR,
    pub pszDeviceDescription: windows_core::PWSTR,
    pub pszPortName: windows_core::PWSTR,
    pub pszPropProvider: windows_core::PWSTR,
    pub pszLocalName: windows_core::PWSTR,
}
impl Default for STI_DEVICE_INFORMATIONW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct STI_DEVICE_MJ_TYPE(pub i32);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct STI_DEVICE_STATUS {
    pub dwSize: u32,
    pub StatusMask: u32,
    pub dwOnlineState: u32,
    pub dwHardwareStatusCode: u32,
    pub dwEventHandlingState: u32,
    pub dwPollingInterval: u32,
}
pub const STI_DEVICE_VALUE_DEFAULT_LAUNCHAPP: windows_core::PCWSTR = windows_core::w!("DefaultLaunchApp");
pub const STI_DEVICE_VALUE_DEFAULT_LAUNCHAPP_A: windows_core::PCSTR = windows_core::s!("DefaultLaunchApp");
pub const STI_DEVICE_VALUE_DISABLE_NOTIFICATIONS: windows_core::PCWSTR = windows_core::w!("DisableNotifications");
pub const STI_DEVICE_VALUE_DISABLE_NOTIFICATIONS_A: windows_core::PCSTR = windows_core::s!("DisableNotifications");
pub const STI_DEVICE_VALUE_ICM_PROFILE: windows_core::PCWSTR = windows_core::w!("ICMProfile");
pub const STI_DEVICE_VALUE_ICM_PROFILE_A: windows_core::PCSTR = windows_core::s!("ICMProfile");
pub const STI_DEVICE_VALUE_ISIS_NAME: windows_core::PCWSTR = windows_core::w!("ISISDriverName");
pub const STI_DEVICE_VALUE_ISIS_NAME_A: windows_core::PCSTR = windows_core::s!("ISISDriverName");
pub const STI_DEVICE_VALUE_TIMEOUT: windows_core::PCWSTR = windows_core::w!("PollTimeout");
pub const STI_DEVICE_VALUE_TIMEOUT_A: windows_core::PCSTR = windows_core::s!("PollTimeout");
pub const STI_DEVICE_VALUE_TWAIN_NAME: windows_core::PCWSTR = windows_core::w!("TwainDS");
pub const STI_DEVICE_VALUE_TWAIN_NAME_A: windows_core::PCSTR = windows_core::s!("TwainDS");
pub const STI_DEVSTATUS_EVENTS_STATE: u32 = 2u32;
pub const STI_DEVSTATUS_ONLINE_STATE: u32 = 1u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct STI_DEV_CAPS {
    pub dwGeneric: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct STI_DIAG {
    pub dwSize: u32,
    pub dwBasicDiagCode: u32,
    pub dwVendorDiagCode: u32,
    pub dwStatusMask: u32,
    pub sErrorInfo: _ERROR_INFOW,
}
pub const STI_DIAGCODE_HWPRESENCE: u32 = 1u32;
pub const STI_ERROR_NO_ERROR: i32 = 0i32;
pub const STI_EVENTHANDLING_ENABLED: u32 = 1u32;
pub const STI_EVENTHANDLING_PENDING: u32 = 4u32;
pub const STI_EVENTHANDLING_POLLING: u32 = 2u32;
pub const STI_GENCAP_AUTO_PORTSELECT: u32 = 8u32;
pub const STI_GENCAP_COMMON_MASK: u32 = 255u32;
pub const STI_GENCAP_GENERATE_ARRIVALEVENT: u32 = 4u32;
pub const STI_GENCAP_NOTIFICATIONS: u32 = 1u32;
pub const STI_GENCAP_POLLING_NEEDED: u32 = 2u32;
pub const STI_GENCAP_SUBSET: u32 = 32u32;
pub const STI_GENCAP_WIA: u32 = 16u32;
pub const STI_HW_CONFIG_PARALLEL: u32 = 16u32;
pub const STI_HW_CONFIG_SCSI: u32 = 2u32;
pub const STI_HW_CONFIG_SERIAL: u32 = 8u32;
pub const STI_HW_CONFIG_UNKNOWN: u32 = 1u32;
pub const STI_HW_CONFIG_USB: u32 = 4u32;
pub const STI_MAX_INTERNAL_NAME_LENGTH: u32 = 128u32;
pub const STI_NOTCONNECTED: i32 = 1i32;
pub const STI_OK: i32 = 0i32;
pub const STI_ONLINESTATE_BUSY: u32 = 256u32;
pub const STI_ONLINESTATE_ERROR: u32 = 4u32;
pub const STI_ONLINESTATE_INITIALIZING: u32 = 1024u32;
pub const STI_ONLINESTATE_IO_ACTIVE: u32 = 128u32;
pub const STI_ONLINESTATE_OFFLINE: u32 = 64u32;
pub const STI_ONLINESTATE_OPERATIONAL: u32 = 1u32;
pub const STI_ONLINESTATE_PAPER_JAM: u32 = 16u32;
pub const STI_ONLINESTATE_PAPER_PROBLEM: u32 = 32u32;
pub const STI_ONLINESTATE_PAUSED: u32 = 8u32;
pub const STI_ONLINESTATE_PENDING: u32 = 2u32;
pub const STI_ONLINESTATE_POWER_SAVE: u32 = 8192u32;
pub const STI_ONLINESTATE_TRANSFERRING: u32 = 512u32;
pub const STI_ONLINESTATE_USER_INTERVENTION: u32 = 4096u32;
pub const STI_ONLINESTATE_WARMING_UP: u32 = 2048u32;
pub const STI_RAW_RESERVED: u32 = 4096u32;
pub const STI_REMOVE_DEVICE_BROADCAST_ACTION: windows_core::PCSTR = windows_core::s!("Removal");
pub const STI_REMOVE_DEVICE_BROADCAST_STRING: windows_core::PCSTR = windows_core::s!("STI\\");
pub const STI_SUBSCRIBE_FLAG_EVENT: u32 = 2u32;
pub const STI_SUBSCRIBE_FLAG_WINDOW: u32 = 1u32;
pub const STI_TRACE_ERROR: u32 = 4u32;
pub const STI_TRACE_INFORMATION: u32 = 1u32;
pub const STI_TRACE_WARNING: u32 = 2u32;
pub const STI_UNICODE: u32 = 1u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct STI_USD_CAPS {
    pub dwVersion: u32,
    pub dwGenericCaps: u32,
}
pub const STI_USD_GENCAP_NATIVE_PUSHSUPPORT: u32 = 1u32;
pub const STI_VERSION: u32 = 2u32;
pub const STI_VERSION_FLAG_MASK: u32 = 4278190080u32;
pub const STI_VERSION_FLAG_UNICODE: u32 = 16777216u32;
pub const STI_VERSION_MIN_ALLOWED: u32 = 2u32;
pub const STI_VERSION_REAL: u32 = 2u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct STI_WIA_DEVICE_INFORMATIONW {
    pub dwSize: u32,
    pub DeviceType: u32,
    pub szDeviceInternalName: [u16; 128],
    pub DeviceCapabilitiesA: STI_DEV_CAPS,
    pub dwHardwareConfiguration: u32,
    pub pszVendorDescription: windows_core::PWSTR,
    pub pszDeviceDescription: windows_core::PWSTR,
    pub pszPortName: windows_core::PWSTR,
    pub pszPropProvider: windows_core::PWSTR,
    pub pszLocalName: windows_core::PWSTR,
    pub pszUiDll: windows_core::PWSTR,
    pub pszServer: windows_core::PWSTR,
}
impl Default for STI_WIA_DEVICE_INFORMATIONW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const SUPPORTS_MSCPLUS_STR: windows_core::PCWSTR = windows_core::w!("SupportsMSCPlus");
pub const SUPPORTS_MSCPLUS_VAL: u32 = 1u32;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct SendToMode(pub i32);
pub const StiDeviceTypeDefault: STI_DEVICE_MJ_TYPE = STI_DEVICE_MJ_TYPE(0i32);
pub const StiDeviceTypeDigitalCamera: STI_DEVICE_MJ_TYPE = STI_DEVICE_MJ_TYPE(2i32);
pub const StiDeviceTypeScanner: STI_DEVICE_MJ_TYPE = STI_DEVICE_MJ_TYPE(1i32);
pub const StiDeviceTypeStreamingVideo: STI_DEVICE_MJ_TYPE = STI_DEVICE_MJ_TYPE(3i32);
pub const WIA_INCOMPAT_XP: u32 = 1u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct _ERROR_INFOW {
    pub dwSize: u32,
    pub dwGenericError: u32,
    pub dwVendorError: u32,
    pub szExtendedErrorText: [u16; 255],
}
impl Default for _ERROR_INFOW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const faetFXSSVC_ENDED: FAX_ACCOUNT_EVENTS_TYPE_ENUM = FAX_ACCOUNT_EVENTS_TYPE_ENUM(16i32);
pub const faetIN_ARCHIVE: FAX_ACCOUNT_EVENTS_TYPE_ENUM = FAX_ACCOUNT_EVENTS_TYPE_ENUM(4i32);
pub const faetIN_QUEUE: FAX_ACCOUNT_EVENTS_TYPE_ENUM = FAX_ACCOUNT_EVENTS_TYPE_ENUM(1i32);
pub const faetNONE: FAX_ACCOUNT_EVENTS_TYPE_ENUM = FAX_ACCOUNT_EVENTS_TYPE_ENUM(0i32);
pub const faetOUT_ARCHIVE: FAX_ACCOUNT_EVENTS_TYPE_ENUM = FAX_ACCOUNT_EVENTS_TYPE_ENUM(8i32);
pub const faetOUT_QUEUE: FAX_ACCOUNT_EVENTS_TYPE_ENUM = FAX_ACCOUNT_EVENTS_TYPE_ENUM(2i32);
pub const far2MANAGE_ARCHIVES: FAX_ACCESS_RIGHTS_ENUM_2 = FAX_ACCESS_RIGHTS_ENUM_2(256i32);
pub const far2MANAGE_CONFIG: FAX_ACCESS_RIGHTS_ENUM_2 = FAX_ACCESS_RIGHTS_ENUM_2(64i32);
pub const far2MANAGE_OUT_JOBS: FAX_ACCESS_RIGHTS_ENUM_2 = FAX_ACCESS_RIGHTS_ENUM_2(16i32);
pub const far2MANAGE_RECEIVE_FOLDER: FAX_ACCESS_RIGHTS_ENUM_2 = FAX_ACCESS_RIGHTS_ENUM_2(512i32);
pub const far2QUERY_ARCHIVES: FAX_ACCESS_RIGHTS_ENUM_2 = FAX_ACCESS_RIGHTS_ENUM_2(128i32);
pub const far2QUERY_CONFIG: FAX_ACCESS_RIGHTS_ENUM_2 = FAX_ACCESS_RIGHTS_ENUM_2(32i32);
pub const far2QUERY_OUT_JOBS: FAX_ACCESS_RIGHTS_ENUM_2 = FAX_ACCESS_RIGHTS_ENUM_2(8i32);
pub const far2SUBMIT_HIGH: FAX_ACCESS_RIGHTS_ENUM_2 = FAX_ACCESS_RIGHTS_ENUM_2(4i32);
pub const far2SUBMIT_LOW: FAX_ACCESS_RIGHTS_ENUM_2 = FAX_ACCESS_RIGHTS_ENUM_2(1i32);
pub const far2SUBMIT_NORMAL: FAX_ACCESS_RIGHTS_ENUM_2 = FAX_ACCESS_RIGHTS_ENUM_2(2i32);
pub const farMANAGE_CONFIG: FAX_ACCESS_RIGHTS_ENUM = FAX_ACCESS_RIGHTS_ENUM(64i32);
pub const farMANAGE_IN_ARCHIVE: FAX_ACCESS_RIGHTS_ENUM = FAX_ACCESS_RIGHTS_ENUM(256i32);
pub const farMANAGE_JOBS: FAX_ACCESS_RIGHTS_ENUM = FAX_ACCESS_RIGHTS_ENUM(16i32);
pub const farMANAGE_OUT_ARCHIVE: FAX_ACCESS_RIGHTS_ENUM = FAX_ACCESS_RIGHTS_ENUM(1024i32);
pub const farQUERY_CONFIG: FAX_ACCESS_RIGHTS_ENUM = FAX_ACCESS_RIGHTS_ENUM(32i32);
pub const farQUERY_IN_ARCHIVE: FAX_ACCESS_RIGHTS_ENUM = FAX_ACCESS_RIGHTS_ENUM(128i32);
pub const farQUERY_JOBS: FAX_ACCESS_RIGHTS_ENUM = FAX_ACCESS_RIGHTS_ENUM(8i32);
pub const farQUERY_OUT_ARCHIVE: FAX_ACCESS_RIGHTS_ENUM = FAX_ACCESS_RIGHTS_ENUM(512i32);
pub const farSUBMIT_HIGH: FAX_ACCESS_RIGHTS_ENUM = FAX_ACCESS_RIGHTS_ENUM(4i32);
pub const farSUBMIT_LOW: FAX_ACCESS_RIGHTS_ENUM = FAX_ACCESS_RIGHTS_ENUM(1i32);
pub const farSUBMIT_NORMAL: FAX_ACCESS_RIGHTS_ENUM = FAX_ACCESS_RIGHTS_ENUM(2i32);
pub const fcptLOCAL: FAX_COVERPAGE_TYPE_ENUM = FAX_COVERPAGE_TYPE_ENUM(1i32);
pub const fcptNONE: FAX_COVERPAGE_TYPE_ENUM = FAX_COVERPAGE_TYPE_ENUM(0i32);
pub const fcptSERVER: FAX_COVERPAGE_TYPE_ENUM = FAX_COVERPAGE_TYPE_ENUM(2i32);
pub const fdrmAUTO_ANSWER: FAX_DEVICE_RECEIVE_MODE_ENUM = FAX_DEVICE_RECEIVE_MODE_ENUM(1i32);
pub const fdrmMANUAL_ANSWER: FAX_DEVICE_RECEIVE_MODE_ENUM = FAX_DEVICE_RECEIVE_MODE_ENUM(2i32);
pub const fdrmNO_ANSWER: FAX_DEVICE_RECEIVE_MODE_ENUM = FAX_DEVICE_RECEIVE_MODE_ENUM(0i32);
pub const fgsALL_DEV_NOT_VALID: FAX_GROUP_STATUS_ENUM = FAX_GROUP_STATUS_ENUM(2i32);
pub const fgsALL_DEV_VALID: FAX_GROUP_STATUS_ENUM = FAX_GROUP_STATUS_ENUM(0i32);
pub const fgsEMPTY: FAX_GROUP_STATUS_ENUM = FAX_GROUP_STATUS_ENUM(1i32);
pub const fgsSOME_DEV_NOT_VALID: FAX_GROUP_STATUS_ENUM = FAX_GROUP_STATUS_ENUM(3i32);
pub const fjesANSWERED: FAX_JOB_EXTENDED_STATUS_ENUM = FAX_JOB_EXTENDED_STATUS_ENUM(5i32);
pub const fjesBAD_ADDRESS: FAX_JOB_EXTENDED_STATUS_ENUM = FAX_JOB_EXTENDED_STATUS_ENUM(10i32);
pub const fjesBUSY: FAX_JOB_EXTENDED_STATUS_ENUM = FAX_JOB_EXTENDED_STATUS_ENUM(8i32);
pub const fjesCALL_ABORTED: FAX_JOB_EXTENDED_STATUS_ENUM = FAX_JOB_EXTENDED_STATUS_ENUM(19i32);
pub const fjesCALL_BLACKLISTED: FAX_JOB_EXTENDED_STATUS_ENUM = FAX_JOB_EXTENDED_STATUS_ENUM(14i32);
pub const fjesCALL_COMPLETED: FAX_JOB_EXTENDED_STATUS_ENUM = FAX_JOB_EXTENDED_STATUS_ENUM(18i32);
pub const fjesCALL_DELAYED: FAX_JOB_EXTENDED_STATUS_ENUM = FAX_JOB_EXTENDED_STATUS_ENUM(13i32);
pub const fjesDIALING: FAX_JOB_EXTENDED_STATUS_ENUM = FAX_JOB_EXTENDED_STATUS_ENUM(3i32);
pub const fjesDISCONNECTED: FAX_JOB_EXTENDED_STATUS_ENUM = FAX_JOB_EXTENDED_STATUS_ENUM(1i32);
pub const fjesFATAL_ERROR: FAX_JOB_EXTENDED_STATUS_ENUM = FAX_JOB_EXTENDED_STATUS_ENUM(12i32);
pub const fjesHANDLED: FAX_JOB_EXTENDED_STATUS_ENUM = FAX_JOB_EXTENDED_STATUS_ENUM(17i32);
pub const fjesINITIALIZING: FAX_JOB_EXTENDED_STATUS_ENUM = FAX_JOB_EXTENDED_STATUS_ENUM(2i32);
pub const fjesLINE_UNAVAILABLE: FAX_JOB_EXTENDED_STATUS_ENUM = FAX_JOB_EXTENDED_STATUS_ENUM(7i32);
pub const fjesNONE: FAX_JOB_EXTENDED_STATUS_ENUM = FAX_JOB_EXTENDED_STATUS_ENUM(0i32);
pub const fjesNOT_FAX_CALL: FAX_JOB_EXTENDED_STATUS_ENUM = FAX_JOB_EXTENDED_STATUS_ENUM(15i32);
pub const fjesNO_ANSWER: FAX_JOB_EXTENDED_STATUS_ENUM = FAX_JOB_EXTENDED_STATUS_ENUM(9i32);
pub const fjesNO_DIAL_TONE: FAX_JOB_EXTENDED_STATUS_ENUM = FAX_JOB_EXTENDED_STATUS_ENUM(11i32);
pub const fjesPARTIALLY_RECEIVED: FAX_JOB_EXTENDED_STATUS_ENUM = FAX_JOB_EXTENDED_STATUS_ENUM(16i32);
pub const fjesPROPRIETARY: FAX_JOB_EXTENDED_STATUS_ENUM = FAX_JOB_EXTENDED_STATUS_ENUM(16777216i32);
pub const fjesRECEIVING: FAX_JOB_EXTENDED_STATUS_ENUM = FAX_JOB_EXTENDED_STATUS_ENUM(6i32);
pub const fjesTRANSMITTING: FAX_JOB_EXTENDED_STATUS_ENUM = FAX_JOB_EXTENDED_STATUS_ENUM(4i32);
pub const fjoDELETE: FAX_JOB_OPERATIONS_ENUM = FAX_JOB_OPERATIONS_ENUM(16i32);
pub const fjoPAUSE: FAX_JOB_OPERATIONS_ENUM = FAX_JOB_OPERATIONS_ENUM(2i32);
pub const fjoRECIPIENT_INFO: FAX_JOB_OPERATIONS_ENUM = FAX_JOB_OPERATIONS_ENUM(32i32);
pub const fjoRESTART: FAX_JOB_OPERATIONS_ENUM = FAX_JOB_OPERATIONS_ENUM(8i32);
pub const fjoRESUME: FAX_JOB_OPERATIONS_ENUM = FAX_JOB_OPERATIONS_ENUM(4i32);
pub const fjoSENDER_INFO: FAX_JOB_OPERATIONS_ENUM = FAX_JOB_OPERATIONS_ENUM(64i32);
pub const fjoVIEW: FAX_JOB_OPERATIONS_ENUM = FAX_JOB_OPERATIONS_ENUM(1i32);
pub const fjsCANCELED: FAX_JOB_STATUS_ENUM = FAX_JOB_STATUS_ENUM(512i32);
pub const fjsCANCELING: FAX_JOB_STATUS_ENUM = FAX_JOB_STATUS_ENUM(1024i32);
pub const fjsCOMPLETED: FAX_JOB_STATUS_ENUM = FAX_JOB_STATUS_ENUM(256i32);
pub const fjsFAILED: FAX_JOB_STATUS_ENUM = FAX_JOB_STATUS_ENUM(8i32);
pub const fjsINPROGRESS: FAX_JOB_STATUS_ENUM = FAX_JOB_STATUS_ENUM(2i32);
pub const fjsNOLINE: FAX_JOB_STATUS_ENUM = FAX_JOB_STATUS_ENUM(32i32);
pub const fjsPAUSED: FAX_JOB_STATUS_ENUM = FAX_JOB_STATUS_ENUM(16i32);
pub const fjsPENDING: FAX_JOB_STATUS_ENUM = FAX_JOB_STATUS_ENUM(1i32);
pub const fjsRETRIES_EXCEEDED: FAX_JOB_STATUS_ENUM = FAX_JOB_STATUS_ENUM(128i32);
pub const fjsRETRYING: FAX_JOB_STATUS_ENUM = FAX_JOB_STATUS_ENUM(64i32);
pub const fjsROUTING: FAX_JOB_STATUS_ENUM = FAX_JOB_STATUS_ENUM(2048i32);
pub const fjtRECEIVE: FAX_JOB_TYPE_ENUM = FAX_JOB_TYPE_ENUM(1i32);
pub const fjtROUTING: FAX_JOB_TYPE_ENUM = FAX_JOB_TYPE_ENUM(2i32);
pub const fjtSEND: FAX_JOB_TYPE_ENUM = FAX_JOB_TYPE_ENUM(0i32);
pub const fllMAX: FAX_LOG_LEVEL_ENUM = FAX_LOG_LEVEL_ENUM(3i32);
pub const fllMED: FAX_LOG_LEVEL_ENUM = FAX_LOG_LEVEL_ENUM(2i32);
pub const fllMIN: FAX_LOG_LEVEL_ENUM = FAX_LOG_LEVEL_ENUM(1i32);
pub const fllNONE: FAX_LOG_LEVEL_ENUM = FAX_LOG_LEVEL_ENUM(0i32);
pub const fpsBAD_GUID: FAX_PROVIDER_STATUS_ENUM = FAX_PROVIDER_STATUS_ENUM(2i32);
pub const fpsBAD_VERSION: FAX_PROVIDER_STATUS_ENUM = FAX_PROVIDER_STATUS_ENUM(3i32);
pub const fpsCANT_INIT: FAX_PROVIDER_STATUS_ENUM = FAX_PROVIDER_STATUS_ENUM(6i32);
pub const fpsCANT_LINK: FAX_PROVIDER_STATUS_ENUM = FAX_PROVIDER_STATUS_ENUM(5i32);
pub const fpsCANT_LOAD: FAX_PROVIDER_STATUS_ENUM = FAX_PROVIDER_STATUS_ENUM(4i32);
pub const fpsSERVER_ERROR: FAX_PROVIDER_STATUS_ENUM = FAX_PROVIDER_STATUS_ENUM(1i32);
pub const fpsSUCCESS: FAX_PROVIDER_STATUS_ENUM = FAX_PROVIDER_STATUS_ENUM(0i32);
pub const fptHIGH: FAX_PRIORITY_TYPE_ENUM = FAX_PRIORITY_TYPE_ENUM(2i32);
pub const fptLOW: FAX_PRIORITY_TYPE_ENUM = FAX_PRIORITY_TYPE_ENUM(0i32);
pub const fptNORMAL: FAX_PRIORITY_TYPE_ENUM = FAX_PRIORITY_TYPE_ENUM(1i32);
pub const frrcANY_CODE: FAX_ROUTING_RULE_CODE_ENUM = FAX_ROUTING_RULE_CODE_ENUM(0i32);
pub const frsALL_GROUP_DEV_NOT_VALID: FAX_RULE_STATUS_ENUM = FAX_RULE_STATUS_ENUM(2i32);
pub const frsBAD_DEVICE: FAX_RULE_STATUS_ENUM = FAX_RULE_STATUS_ENUM(4i32);
pub const frsEMPTY_GROUP: FAX_RULE_STATUS_ENUM = FAX_RULE_STATUS_ENUM(1i32);
pub const frsSOME_GROUP_DEV_NOT_VALID: FAX_RULE_STATUS_ENUM = FAX_RULE_STATUS_ENUM(3i32);
pub const frsVALID: FAX_RULE_STATUS_ENUM = FAX_RULE_STATUS_ENUM(0i32);
pub const frtMAIL: FAX_RECEIPT_TYPE_ENUM = FAX_RECEIPT_TYPE_ENUM(1i32);
pub const frtMSGBOX: FAX_RECEIPT_TYPE_ENUM = FAX_RECEIPT_TYPE_ENUM(4i32);
pub const frtNONE: FAX_RECEIPT_TYPE_ENUM = FAX_RECEIPT_TYPE_ENUM(0i32);
pub const fsAPI_VERSION_0: FAX_SERVER_APIVERSION_ENUM = FAX_SERVER_APIVERSION_ENUM(0i32);
pub const fsAPI_VERSION_1: FAX_SERVER_APIVERSION_ENUM = FAX_SERVER_APIVERSION_ENUM(65536i32);
pub const fsAPI_VERSION_2: FAX_SERVER_APIVERSION_ENUM = FAX_SERVER_APIVERSION_ENUM(131072i32);
pub const fsAPI_VERSION_3: FAX_SERVER_APIVERSION_ENUM = FAX_SERVER_APIVERSION_ENUM(196608i32);
pub const fsatANONYMOUS: FAX_SMTP_AUTHENTICATION_TYPE_ENUM = FAX_SMTP_AUTHENTICATION_TYPE_ENUM(0i32);
pub const fsatBASIC: FAX_SMTP_AUTHENTICATION_TYPE_ENUM = FAX_SMTP_AUTHENTICATION_TYPE_ENUM(1i32);
pub const fsatNTLM: FAX_SMTP_AUTHENTICATION_TYPE_ENUM = FAX_SMTP_AUTHENTICATION_TYPE_ENUM(2i32);
pub const fsetACTIVITY: FAX_SERVER_EVENTS_TYPE_ENUM = FAX_SERVER_EVENTS_TYPE_ENUM(8i32);
pub const fsetCONFIG: FAX_SERVER_EVENTS_TYPE_ENUM = FAX_SERVER_EVENTS_TYPE_ENUM(4i32);
pub const fsetDEVICE_STATUS: FAX_SERVER_EVENTS_TYPE_ENUM = FAX_SERVER_EVENTS_TYPE_ENUM(256i32);
pub const fsetFXSSVC_ENDED: FAX_SERVER_EVENTS_TYPE_ENUM = FAX_SERVER_EVENTS_TYPE_ENUM(128i32);
pub const fsetINCOMING_CALL: FAX_SERVER_EVENTS_TYPE_ENUM = FAX_SERVER_EVENTS_TYPE_ENUM(512i32);
pub const fsetIN_ARCHIVE: FAX_SERVER_EVENTS_TYPE_ENUM = FAX_SERVER_EVENTS_TYPE_ENUM(32i32);
pub const fsetIN_QUEUE: FAX_SERVER_EVENTS_TYPE_ENUM = FAX_SERVER_EVENTS_TYPE_ENUM(1i32);
pub const fsetNONE: FAX_SERVER_EVENTS_TYPE_ENUM = FAX_SERVER_EVENTS_TYPE_ENUM(0i32);
pub const fsetOUT_ARCHIVE: FAX_SERVER_EVENTS_TYPE_ENUM = FAX_SERVER_EVENTS_TYPE_ENUM(64i32);
pub const fsetOUT_QUEUE: FAX_SERVER_EVENTS_TYPE_ENUM = FAX_SERVER_EVENTS_TYPE_ENUM(2i32);
pub const fsetQUEUE_STATE: FAX_SERVER_EVENTS_TYPE_ENUM = FAX_SERVER_EVENTS_TYPE_ENUM(16i32);
pub const fstDISCOUNT_PERIOD: FAX_SCHEDULE_TYPE_ENUM = FAX_SCHEDULE_TYPE_ENUM(2i32);
pub const fstNOW: FAX_SCHEDULE_TYPE_ENUM = FAX_SCHEDULE_TYPE_ENUM(0i32);
pub const fstSPECIFIC_TIME: FAX_SCHEDULE_TYPE_ENUM = FAX_SCHEDULE_TYPE_ENUM(1i32);
pub const lDEFAULT_PREFETCH_SIZE: i32 = 100i32;
pub const prv_DEFAULT_PREFETCH_SIZE: u32 = 100u32;
pub const wcharREASSIGN_RECIPIENTS_DELIMITER: u16 = 59u16;
