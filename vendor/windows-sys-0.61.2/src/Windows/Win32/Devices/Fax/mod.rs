windows_link::link!("fxsutility.dll" "system" fn CanSendToFaxRecipient() -> windows_sys::core::BOOL);
windows_link::link!("winfax.dll" "system" fn FaxAbort(faxhandle : super::super::Foundation:: HANDLE, jobid : u32) -> windows_sys::core::BOOL);
windows_link::link!("winfax.dll" "system" fn FaxAccessCheck(faxhandle : super::super::Foundation:: HANDLE, accessmask : u32) -> windows_sys::core::BOOL);
windows_link::link!("winfax.dll" "system" fn FaxClose(faxhandle : super::super::Foundation:: HANDLE) -> windows_sys::core::BOOL);
windows_link::link!("winfax.dll" "system" fn FaxCompleteJobParamsA(jobparams : *mut *mut FAX_JOB_PARAMA, coverpageinfo : *mut *mut FAX_COVERPAGE_INFOA) -> windows_sys::core::BOOL);
windows_link::link!("winfax.dll" "system" fn FaxCompleteJobParamsW(jobparams : *mut *mut FAX_JOB_PARAMW, coverpageinfo : *mut *mut FAX_COVERPAGE_INFOW) -> windows_sys::core::BOOL);
windows_link::link!("winfax.dll" "system" fn FaxConnectFaxServerA(machinename : windows_sys::core::PCSTR, faxhandle : *mut super::super::Foundation:: HANDLE) -> windows_sys::core::BOOL);
windows_link::link!("winfax.dll" "system" fn FaxConnectFaxServerW(machinename : windows_sys::core::PCWSTR, faxhandle : *mut super::super::Foundation:: HANDLE) -> windows_sys::core::BOOL);
windows_link::link!("winfax.dll" "system" fn FaxEnableRoutingMethodA(faxporthandle : super::super::Foundation:: HANDLE, routingguid : windows_sys::core::PCSTR, enabled : windows_sys::core::BOOL) -> windows_sys::core::BOOL);
windows_link::link!("winfax.dll" "system" fn FaxEnableRoutingMethodW(faxporthandle : super::super::Foundation:: HANDLE, routingguid : windows_sys::core::PCWSTR, enabled : windows_sys::core::BOOL) -> windows_sys::core::BOOL);
windows_link::link!("winfax.dll" "system" fn FaxEnumGlobalRoutingInfoA(faxhandle : super::super::Foundation:: HANDLE, routinginfo : *mut *mut FAX_GLOBAL_ROUTING_INFOA, methodsreturned : *mut u32) -> windows_sys::core::BOOL);
windows_link::link!("winfax.dll" "system" fn FaxEnumGlobalRoutingInfoW(faxhandle : super::super::Foundation:: HANDLE, routinginfo : *mut *mut FAX_GLOBAL_ROUTING_INFOW, methodsreturned : *mut u32) -> windows_sys::core::BOOL);
windows_link::link!("winfax.dll" "system" fn FaxEnumJobsA(faxhandle : super::super::Foundation:: HANDLE, jobentry : *mut *mut FAX_JOB_ENTRYA, jobsreturned : *mut u32) -> windows_sys::core::BOOL);
windows_link::link!("winfax.dll" "system" fn FaxEnumJobsW(faxhandle : super::super::Foundation:: HANDLE, jobentry : *mut *mut FAX_JOB_ENTRYW, jobsreturned : *mut u32) -> windows_sys::core::BOOL);
windows_link::link!("winfax.dll" "system" fn FaxEnumPortsA(faxhandle : super::super::Foundation:: HANDLE, portinfo : *mut *mut FAX_PORT_INFOA, portsreturned : *mut u32) -> windows_sys::core::BOOL);
windows_link::link!("winfax.dll" "system" fn FaxEnumPortsW(faxhandle : super::super::Foundation:: HANDLE, portinfo : *mut *mut FAX_PORT_INFOW, portsreturned : *mut u32) -> windows_sys::core::BOOL);
windows_link::link!("winfax.dll" "system" fn FaxEnumRoutingMethodsA(faxporthandle : super::super::Foundation:: HANDLE, routingmethod : *mut *mut FAX_ROUTING_METHODA, methodsreturned : *mut u32) -> windows_sys::core::BOOL);
windows_link::link!("winfax.dll" "system" fn FaxEnumRoutingMethodsW(faxporthandle : super::super::Foundation:: HANDLE, routingmethod : *mut *mut FAX_ROUTING_METHODW, methodsreturned : *mut u32) -> windows_sys::core::BOOL);
windows_link::link!("winfax.dll" "system" fn FaxFreeBuffer(buffer : *mut core::ffi::c_void));
windows_link::link!("winfax.dll" "system" fn FaxGetConfigurationA(faxhandle : super::super::Foundation:: HANDLE, faxconfig : *mut *mut FAX_CONFIGURATIONA) -> windows_sys::core::BOOL);
windows_link::link!("winfax.dll" "system" fn FaxGetConfigurationW(faxhandle : super::super::Foundation:: HANDLE, faxconfig : *mut *mut FAX_CONFIGURATIONW) -> windows_sys::core::BOOL);
windows_link::link!("winfax.dll" "system" fn FaxGetDeviceStatusA(faxporthandle : super::super::Foundation:: HANDLE, devicestatus : *mut *mut FAX_DEVICE_STATUSA) -> windows_sys::core::BOOL);
windows_link::link!("winfax.dll" "system" fn FaxGetDeviceStatusW(faxporthandle : super::super::Foundation:: HANDLE, devicestatus : *mut *mut FAX_DEVICE_STATUSW) -> windows_sys::core::BOOL);
windows_link::link!("winfax.dll" "system" fn FaxGetJobA(faxhandle : super::super::Foundation:: HANDLE, jobid : u32, jobentry : *mut *mut FAX_JOB_ENTRYA) -> windows_sys::core::BOOL);
windows_link::link!("winfax.dll" "system" fn FaxGetJobW(faxhandle : super::super::Foundation:: HANDLE, jobid : u32, jobentry : *mut *mut FAX_JOB_ENTRYW) -> windows_sys::core::BOOL);
windows_link::link!("winfax.dll" "system" fn FaxGetLoggingCategoriesA(faxhandle : super::super::Foundation:: HANDLE, categories : *mut *mut FAX_LOG_CATEGORYA, numbercategories : *mut u32) -> windows_sys::core::BOOL);
windows_link::link!("winfax.dll" "system" fn FaxGetLoggingCategoriesW(faxhandle : super::super::Foundation:: HANDLE, categories : *mut *mut FAX_LOG_CATEGORYW, numbercategories : *mut u32) -> windows_sys::core::BOOL);
windows_link::link!("winfax.dll" "system" fn FaxGetPageData(faxhandle : super::super::Foundation:: HANDLE, jobid : u32, buffer : *mut *mut u8, buffersize : *mut u32, imagewidth : *mut u32, imageheight : *mut u32) -> windows_sys::core::BOOL);
windows_link::link!("winfax.dll" "system" fn FaxGetPortA(faxporthandle : super::super::Foundation:: HANDLE, portinfo : *mut *mut FAX_PORT_INFOA) -> windows_sys::core::BOOL);
windows_link::link!("winfax.dll" "system" fn FaxGetPortW(faxporthandle : super::super::Foundation:: HANDLE, portinfo : *mut *mut FAX_PORT_INFOW) -> windows_sys::core::BOOL);
windows_link::link!("winfax.dll" "system" fn FaxGetRoutingInfoA(faxporthandle : super::super::Foundation:: HANDLE, routingguid : windows_sys::core::PCSTR, routinginfobuffer : *mut *mut u8, routinginfobuffersize : *mut u32) -> windows_sys::core::BOOL);
windows_link::link!("winfax.dll" "system" fn FaxGetRoutingInfoW(faxporthandle : super::super::Foundation:: HANDLE, routingguid : windows_sys::core::PCWSTR, routinginfobuffer : *mut *mut u8, routinginfobuffersize : *mut u32) -> windows_sys::core::BOOL);
windows_link::link!("winfax.dll" "system" fn FaxInitializeEventQueue(faxhandle : super::super::Foundation:: HANDLE, completionport : super::super::Foundation:: HANDLE, completionkey : usize, hwnd : super::super::Foundation:: HWND, messagestart : u32) -> windows_sys::core::BOOL);
windows_link::link!("winfax.dll" "system" fn FaxOpenPort(faxhandle : super::super::Foundation:: HANDLE, deviceid : u32, flags : u32, faxporthandle : *mut super::super::Foundation:: HANDLE) -> windows_sys::core::BOOL);
#[cfg(feature = "Win32_Graphics_Gdi")]
windows_link::link!("winfax.dll" "system" fn FaxPrintCoverPageA(faxcontextinfo : *const FAX_CONTEXT_INFOA, coverpageinfo : *const FAX_COVERPAGE_INFOA) -> windows_sys::core::BOOL);
#[cfg(feature = "Win32_Graphics_Gdi")]
windows_link::link!("winfax.dll" "system" fn FaxPrintCoverPageW(faxcontextinfo : *const FAX_CONTEXT_INFOW, coverpageinfo : *const FAX_COVERPAGE_INFOW) -> windows_sys::core::BOOL);
windows_link::link!("winfax.dll" "system" fn FaxRegisterRoutingExtensionW(faxhandle : super::super::Foundation:: HANDLE, extensionname : windows_sys::core::PCWSTR, friendlyname : windows_sys::core::PCWSTR, imagename : windows_sys::core::PCWSTR, callback : PFAX_ROUTING_INSTALLATION_CALLBACKW, context : *const core::ffi::c_void) -> windows_sys::core::BOOL);
windows_link::link!("winfax.dll" "system" fn FaxRegisterServiceProviderW(deviceprovider : windows_sys::core::PCWSTR, friendlyname : windows_sys::core::PCWSTR, imagename : windows_sys::core::PCWSTR, tspname : windows_sys::core::PCWSTR) -> windows_sys::core::BOOL);
windows_link::link!("winfax.dll" "system" fn FaxSendDocumentA(faxhandle : super::super::Foundation:: HANDLE, filename : windows_sys::core::PCSTR, jobparams : *const FAX_JOB_PARAMA, coverpageinfo : *const FAX_COVERPAGE_INFOA, faxjobid : *mut u32) -> windows_sys::core::BOOL);
windows_link::link!("winfax.dll" "system" fn FaxSendDocumentForBroadcastA(faxhandle : super::super::Foundation:: HANDLE, filename : windows_sys::core::PCSTR, faxjobid : *mut u32, faxrecipientcallback : PFAX_RECIPIENT_CALLBACKA, context : *const core::ffi::c_void) -> windows_sys::core::BOOL);
windows_link::link!("winfax.dll" "system" fn FaxSendDocumentForBroadcastW(faxhandle : super::super::Foundation:: HANDLE, filename : windows_sys::core::PCWSTR, faxjobid : *mut u32, faxrecipientcallback : PFAX_RECIPIENT_CALLBACKW, context : *const core::ffi::c_void) -> windows_sys::core::BOOL);
windows_link::link!("winfax.dll" "system" fn FaxSendDocumentW(faxhandle : super::super::Foundation:: HANDLE, filename : windows_sys::core::PCWSTR, jobparams : *const FAX_JOB_PARAMW, coverpageinfo : *const FAX_COVERPAGE_INFOW, faxjobid : *mut u32) -> windows_sys::core::BOOL);
windows_link::link!("winfax.dll" "system" fn FaxSetConfigurationA(faxhandle : super::super::Foundation:: HANDLE, faxconfig : *const FAX_CONFIGURATIONA) -> windows_sys::core::BOOL);
windows_link::link!("winfax.dll" "system" fn FaxSetConfigurationW(faxhandle : super::super::Foundation:: HANDLE, faxconfig : *const FAX_CONFIGURATIONW) -> windows_sys::core::BOOL);
windows_link::link!("winfax.dll" "system" fn FaxSetGlobalRoutingInfoA(faxhandle : super::super::Foundation:: HANDLE, routinginfo : *const FAX_GLOBAL_ROUTING_INFOA) -> windows_sys::core::BOOL);
windows_link::link!("winfax.dll" "system" fn FaxSetGlobalRoutingInfoW(faxhandle : super::super::Foundation:: HANDLE, routinginfo : *const FAX_GLOBAL_ROUTING_INFOW) -> windows_sys::core::BOOL);
windows_link::link!("winfax.dll" "system" fn FaxSetJobA(faxhandle : super::super::Foundation:: HANDLE, jobid : u32, command : u32, jobentry : *const FAX_JOB_ENTRYA) -> windows_sys::core::BOOL);
windows_link::link!("winfax.dll" "system" fn FaxSetJobW(faxhandle : super::super::Foundation:: HANDLE, jobid : u32, command : u32, jobentry : *const FAX_JOB_ENTRYW) -> windows_sys::core::BOOL);
windows_link::link!("winfax.dll" "system" fn FaxSetLoggingCategoriesA(faxhandle : super::super::Foundation:: HANDLE, categories : *const FAX_LOG_CATEGORYA, numbercategories : u32) -> windows_sys::core::BOOL);
windows_link::link!("winfax.dll" "system" fn FaxSetLoggingCategoriesW(faxhandle : super::super::Foundation:: HANDLE, categories : *const FAX_LOG_CATEGORYW, numbercategories : u32) -> windows_sys::core::BOOL);
windows_link::link!("winfax.dll" "system" fn FaxSetPortA(faxporthandle : super::super::Foundation:: HANDLE, portinfo : *const FAX_PORT_INFOA) -> windows_sys::core::BOOL);
windows_link::link!("winfax.dll" "system" fn FaxSetPortW(faxporthandle : super::super::Foundation:: HANDLE, portinfo : *const FAX_PORT_INFOW) -> windows_sys::core::BOOL);
windows_link::link!("winfax.dll" "system" fn FaxSetRoutingInfoA(faxporthandle : super::super::Foundation:: HANDLE, routingguid : windows_sys::core::PCSTR, routinginfobuffer : *const u8, routinginfobuffersize : u32) -> windows_sys::core::BOOL);
windows_link::link!("winfax.dll" "system" fn FaxSetRoutingInfoW(faxporthandle : super::super::Foundation:: HANDLE, routingguid : windows_sys::core::PCWSTR, routinginfobuffer : *const u8, routinginfobuffersize : u32) -> windows_sys::core::BOOL);
#[cfg(feature = "Win32_Graphics_Gdi")]
windows_link::link!("winfax.dll" "system" fn FaxStartPrintJobA(printername : windows_sys::core::PCSTR, printinfo : *const FAX_PRINT_INFOA, faxjobid : *mut u32, faxcontextinfo : *mut FAX_CONTEXT_INFOA) -> windows_sys::core::BOOL);
#[cfg(feature = "Win32_Graphics_Gdi")]
windows_link::link!("winfax.dll" "system" fn FaxStartPrintJobW(printername : windows_sys::core::PCWSTR, printinfo : *const FAX_PRINT_INFOW, faxjobid : *mut u32, faxcontextinfo : *mut FAX_CONTEXT_INFOW) -> windows_sys::core::BOOL);
windows_link::link!("winfax.dll" "system" fn FaxUnregisterServiceProviderW(deviceprovider : windows_sys::core::PCWSTR) -> windows_sys::core::BOOL);
windows_link::link!("fxsutility.dll" "system" fn SendToFaxRecipient(sndmode : SendToMode, lpfilename : windows_sys::core::PCWSTR) -> u32);
windows_link::link!("sti.dll" "system" fn StiCreateInstanceW(hinst : super::super::Foundation:: HINSTANCE, dwver : u32, ppsti : *mut * mut core::ffi::c_void, punkouter : * mut core::ffi::c_void) -> windows_sys::core::HRESULT);
pub const CF_MSFAXSRV_DEVICE_ID: windows_sys::core::PCWSTR = windows_sys::core::w!("FAXSRV_DeviceID");
pub const CF_MSFAXSRV_FSP_GUID: windows_sys::core::PCWSTR = windows_sys::core::w!("FAXSRV_FSPGuid");
pub const CF_MSFAXSRV_ROUTEEXT_NAME: windows_sys::core::PCWSTR = windows_sys::core::w!("FAXSRV_RoutingExtName");
pub const CF_MSFAXSRV_ROUTING_METHOD_GUID: windows_sys::core::PCWSTR = windows_sys::core::w!("FAXSRV_RoutingMethodGuid");
pub const CF_MSFAXSRV_SERVER_NAME: windows_sys::core::PCWSTR = windows_sys::core::w!("FAXSRV_ServerName");
pub const CLSID_Sti: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xb323f8e0_2e68_11d0_90ea_00aa0060f86c);
pub const DEVPKEY_WIA_DeviceType: super::super::Foundation::DEVPROPKEY = super::super::Foundation::DEVPROPKEY { fmtid: windows_sys::core::GUID::from_u128(0x6bdd1fc6_810f_11d0_bec7_08002be2092f), pid: 2 };
pub const DEVPKEY_WIA_USDClassId: super::super::Foundation::DEVPROPKEY = super::super::Foundation::DEVPROPKEY { fmtid: windows_sys::core::GUID::from_u128(0x6bdd1fc6_810f_11d0_bec7_08002be2092f), pid: 3 };
pub const DEV_ID_SRC_FAX: FAX_ENUM_DEVICE_ID_SOURCE = 0i32;
pub const DEV_ID_SRC_TAPI: FAX_ENUM_DEVICE_ID_SOURCE = 1i32;
pub const DRT_EMAIL: FAX_ENUM_DELIVERY_REPORT_TYPES = 1i32;
pub const DRT_INBOX: FAX_ENUM_DELIVERY_REPORT_TYPES = 2i32;
pub const DRT_NONE: FAX_ENUM_DELIVERY_REPORT_TYPES = 0i32;
pub const FAXDEVRECEIVE_SIZE: u32 = 4096u32;
pub const FAXDEVREPORTSTATUS_SIZE: u32 = 4096u32;
pub const FAXLOG_CATEGORY_INBOUND: FAX_ENUM_LOG_CATEGORIES = 3i32;
pub const FAXLOG_CATEGORY_INIT: FAX_ENUM_LOG_CATEGORIES = 1i32;
pub const FAXLOG_CATEGORY_OUTBOUND: FAX_ENUM_LOG_CATEGORIES = 2i32;
pub const FAXLOG_CATEGORY_UNKNOWN: FAX_ENUM_LOG_CATEGORIES = 4i32;
pub const FAXLOG_LEVEL_MAX: FAX_ENUM_LOG_LEVELS = 3i32;
pub const FAXLOG_LEVEL_MED: FAX_ENUM_LOG_LEVELS = 2i32;
pub const FAXLOG_LEVEL_MIN: FAX_ENUM_LOG_LEVELS = 1i32;
pub const FAXLOG_LEVEL_NONE: FAX_ENUM_LOG_LEVELS = 0i32;
pub type FAXROUTE_ENABLE = i32;
pub const FAXSRV_DEVICE_NODETYPE_GUID: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x3115a19a_6251_46ac_9425_14782858b8c9);
pub const FAXSRV_DEVICE_PROVIDER_NODETYPE_GUID: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xbd38e2ac_b926_4161_8640_0f6956ee2ba3);
pub const FAXSRV_ROUTING_METHOD_NODETYPE_GUID: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x220d2cb0_85a9_4a43_b6e8_9d66b44f1af5);
pub type FAX_ACCESS_RIGHTS_ENUM = i32;
pub type FAX_ACCESS_RIGHTS_ENUM_2 = i32;
pub type FAX_ACCOUNT_EVENTS_TYPE_ENUM = i32;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct FAX_CONFIGURATIONA {
    pub SizeOfStruct: u32,
    pub Retries: u32,
    pub RetryDelay: u32,
    pub DirtyDays: u32,
    pub Branding: windows_sys::core::BOOL,
    pub UseDeviceTsid: windows_sys::core::BOOL,
    pub ServerCp: windows_sys::core::BOOL,
    pub PauseServerQueue: windows_sys::core::BOOL,
    pub StartCheapTime: FAX_TIME,
    pub StopCheapTime: FAX_TIME,
    pub ArchiveOutgoingFaxes: windows_sys::core::BOOL,
    pub ArchiveDirectory: windows_sys::core::PCSTR,
    pub Reserved: windows_sys::core::PCSTR,
}
impl Default for FAX_CONFIGURATIONA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct FAX_CONFIGURATIONW {
    pub SizeOfStruct: u32,
    pub Retries: u32,
    pub RetryDelay: u32,
    pub DirtyDays: u32,
    pub Branding: windows_sys::core::BOOL,
    pub UseDeviceTsid: windows_sys::core::BOOL,
    pub ServerCp: windows_sys::core::BOOL,
    pub PauseServerQueue: windows_sys::core::BOOL,
    pub StartCheapTime: FAX_TIME,
    pub StopCheapTime: FAX_TIME,
    pub ArchiveOutgoingFaxes: windows_sys::core::BOOL,
    pub ArchiveDirectory: windows_sys::core::PCWSTR,
    pub Reserved: windows_sys::core::PCWSTR,
}
impl Default for FAX_CONFIGURATIONW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const FAX_CONFIG_QUERY: u32 = 4u32;
pub const FAX_CONFIG_SET: u32 = 8u32;
#[repr(C)]
#[cfg(feature = "Win32_Graphics_Gdi")]
#[derive(Clone, Copy)]
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
#[derive(Clone, Copy)]
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
#[derive(Clone, Copy)]
pub struct FAX_COVERPAGE_INFOA {
    pub SizeOfStruct: u32,
    pub CoverPageName: windows_sys::core::PCSTR,
    pub UseServerCoverPage: windows_sys::core::BOOL,
    pub RecName: windows_sys::core::PCSTR,
    pub RecFaxNumber: windows_sys::core::PCSTR,
    pub RecCompany: windows_sys::core::PCSTR,
    pub RecStreetAddress: windows_sys::core::PCSTR,
    pub RecCity: windows_sys::core::PCSTR,
    pub RecState: windows_sys::core::PCSTR,
    pub RecZip: windows_sys::core::PCSTR,
    pub RecCountry: windows_sys::core::PCSTR,
    pub RecTitle: windows_sys::core::PCSTR,
    pub RecDepartment: windows_sys::core::PCSTR,
    pub RecOfficeLocation: windows_sys::core::PCSTR,
    pub RecHomePhone: windows_sys::core::PCSTR,
    pub RecOfficePhone: windows_sys::core::PCSTR,
    pub SdrName: windows_sys::core::PCSTR,
    pub SdrFaxNumber: windows_sys::core::PCSTR,
    pub SdrCompany: windows_sys::core::PCSTR,
    pub SdrAddress: windows_sys::core::PCSTR,
    pub SdrTitle: windows_sys::core::PCSTR,
    pub SdrDepartment: windows_sys::core::PCSTR,
    pub SdrOfficeLocation: windows_sys::core::PCSTR,
    pub SdrHomePhone: windows_sys::core::PCSTR,
    pub SdrOfficePhone: windows_sys::core::PCSTR,
    pub Note: windows_sys::core::PCSTR,
    pub Subject: windows_sys::core::PCSTR,
    pub TimeSent: super::super::Foundation::SYSTEMTIME,
    pub PageCount: u32,
}
impl Default for FAX_COVERPAGE_INFOA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct FAX_COVERPAGE_INFOW {
    pub SizeOfStruct: u32,
    pub CoverPageName: windows_sys::core::PCWSTR,
    pub UseServerCoverPage: windows_sys::core::BOOL,
    pub RecName: windows_sys::core::PCWSTR,
    pub RecFaxNumber: windows_sys::core::PCWSTR,
    pub RecCompany: windows_sys::core::PCWSTR,
    pub RecStreetAddress: windows_sys::core::PCWSTR,
    pub RecCity: windows_sys::core::PCWSTR,
    pub RecState: windows_sys::core::PCWSTR,
    pub RecZip: windows_sys::core::PCWSTR,
    pub RecCountry: windows_sys::core::PCWSTR,
    pub RecTitle: windows_sys::core::PCWSTR,
    pub RecDepartment: windows_sys::core::PCWSTR,
    pub RecOfficeLocation: windows_sys::core::PCWSTR,
    pub RecHomePhone: windows_sys::core::PCWSTR,
    pub RecOfficePhone: windows_sys::core::PCWSTR,
    pub SdrName: windows_sys::core::PCWSTR,
    pub SdrFaxNumber: windows_sys::core::PCWSTR,
    pub SdrCompany: windows_sys::core::PCWSTR,
    pub SdrAddress: windows_sys::core::PCWSTR,
    pub SdrTitle: windows_sys::core::PCWSTR,
    pub SdrDepartment: windows_sys::core::PCWSTR,
    pub SdrOfficeLocation: windows_sys::core::PCWSTR,
    pub SdrHomePhone: windows_sys::core::PCWSTR,
    pub SdrOfficePhone: windows_sys::core::PCWSTR,
    pub Note: windows_sys::core::PCWSTR,
    pub Subject: windows_sys::core::PCWSTR,
    pub TimeSent: super::super::Foundation::SYSTEMTIME,
    pub PageCount: u32,
}
impl Default for FAX_COVERPAGE_INFOW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type FAX_COVERPAGE_TYPE_ENUM = i32;
pub type FAX_DEVICE_RECEIVE_MODE_ENUM = i32;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct FAX_DEVICE_STATUSA {
    pub SizeOfStruct: u32,
    pub CallerId: windows_sys::core::PCSTR,
    pub Csid: windows_sys::core::PCSTR,
    pub CurrentPage: u32,
    pub DeviceId: u32,
    pub DeviceName: windows_sys::core::PCSTR,
    pub DocumentName: windows_sys::core::PCSTR,
    pub JobType: u32,
    pub PhoneNumber: windows_sys::core::PCSTR,
    pub RoutingString: windows_sys::core::PCSTR,
    pub SenderName: windows_sys::core::PCSTR,
    pub RecipientName: windows_sys::core::PCSTR,
    pub Size: u32,
    pub StartTime: super::super::Foundation::FILETIME,
    pub Status: u32,
    pub StatusString: windows_sys::core::PCSTR,
    pub SubmittedTime: super::super::Foundation::FILETIME,
    pub TotalPages: u32,
    pub Tsid: windows_sys::core::PCSTR,
    pub UserName: windows_sys::core::PCSTR,
}
impl Default for FAX_DEVICE_STATUSA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct FAX_DEVICE_STATUSW {
    pub SizeOfStruct: u32,
    pub CallerId: windows_sys::core::PCWSTR,
    pub Csid: windows_sys::core::PCWSTR,
    pub CurrentPage: u32,
    pub DeviceId: u32,
    pub DeviceName: windows_sys::core::PCWSTR,
    pub DocumentName: windows_sys::core::PCWSTR,
    pub JobType: u32,
    pub PhoneNumber: windows_sys::core::PCWSTR,
    pub RoutingString: windows_sys::core::PCWSTR,
    pub SenderName: windows_sys::core::PCWSTR,
    pub RecipientName: windows_sys::core::PCWSTR,
    pub Size: u32,
    pub StartTime: super::super::Foundation::FILETIME,
    pub Status: u32,
    pub StatusString: windows_sys::core::PCWSTR,
    pub SubmittedTime: super::super::Foundation::FILETIME,
    pub TotalPages: u32,
    pub Tsid: windows_sys::core::PCWSTR,
    pub UserName: windows_sys::core::PCWSTR,
}
impl Default for FAX_DEVICE_STATUSW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct FAX_DEV_STATUS {
    pub SizeOfStruct: u32,
    pub StatusId: u32,
    pub StringId: u32,
    pub PageCount: u32,
    pub CSI: windows_sys::core::PWSTR,
    pub CallerId: windows_sys::core::PWSTR,
    pub RoutingInfo: windows_sys::core::PWSTR,
    pub ErrorCode: u32,
    pub Reserved: [u32; 3],
}
impl Default for FAX_DEV_STATUS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type FAX_ENUM_DELIVERY_REPORT_TYPES = i32;
pub type FAX_ENUM_DEVICE_ID_SOURCE = i32;
pub type FAX_ENUM_JOB_COMMANDS = i32;
pub type FAX_ENUM_JOB_SEND_ATTRIBUTES = i32;
pub type FAX_ENUM_LOG_CATEGORIES = i32;
pub type FAX_ENUM_LOG_LEVELS = i32;
pub type FAX_ENUM_PORT_OPEN_TYPE = i32;
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
#[derive(Clone, Copy, Default)]
pub struct FAX_EVENTA {
    pub SizeOfStruct: u32,
    pub TimeStamp: super::super::Foundation::FILETIME,
    pub DeviceId: u32,
    pub EventId: u32,
    pub JobId: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct FAX_EVENTW {
    pub SizeOfStruct: u32,
    pub TimeStamp: super::super::Foundation::FILETIME,
    pub DeviceId: u32,
    pub EventId: u32,
    pub JobId: u32,
}
pub const FAX_E_BAD_GROUP_CONFIGURATION: windows_sys::core::HRESULT = 0x80041B5B_u32 as _;
pub const FAX_E_DEVICE_NUM_LIMIT_EXCEEDED: windows_sys::core::HRESULT = 0x80041B62_u32 as _;
pub const FAX_E_DIRECTORY_IN_USE: windows_sys::core::HRESULT = 0x80041B5F_u32 as _;
pub const FAX_E_FILE_ACCESS_DENIED: windows_sys::core::HRESULT = 0x80041B60_u32 as _;
pub const FAX_E_GROUP_IN_USE: windows_sys::core::HRESULT = 0x80041B5C_u32 as _;
pub const FAX_E_GROUP_NOT_FOUND: windows_sys::core::HRESULT = 0x80041B5A_u32 as _;
pub const FAX_E_MESSAGE_NOT_FOUND: windows_sys::core::HRESULT = 0x80041B61_u32 as _;
pub const FAX_E_NOT_NTFS: windows_sys::core::HRESULT = 0x80041B5E_u32 as _;
pub const FAX_E_NOT_SUPPORTED_ON_THIS_SKU: windows_sys::core::HRESULT = 0x80041B63_u32 as _;
pub const FAX_E_RECIPIENTS_LIMIT: windows_sys::core::HRESULT = 0x80041B65_u32 as _;
pub const FAX_E_RULE_NOT_FOUND: windows_sys::core::HRESULT = 0x80041B5D_u32 as _;
pub const FAX_E_SRV_OUTOFMEMORY: windows_sys::core::HRESULT = 0x80041B59_u32 as _;
pub const FAX_E_VERSION_MISMATCH: windows_sys::core::HRESULT = 0x80041B64_u32 as _;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct FAX_GLOBAL_ROUTING_INFOA {
    pub SizeOfStruct: u32,
    pub Priority: u32,
    pub Guid: windows_sys::core::PCSTR,
    pub FriendlyName: windows_sys::core::PCSTR,
    pub FunctionName: windows_sys::core::PCSTR,
    pub ExtensionImageName: windows_sys::core::PCSTR,
    pub ExtensionFriendlyName: windows_sys::core::PCSTR,
}
impl Default for FAX_GLOBAL_ROUTING_INFOA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct FAX_GLOBAL_ROUTING_INFOW {
    pub SizeOfStruct: u32,
    pub Priority: u32,
    pub Guid: windows_sys::core::PCWSTR,
    pub FriendlyName: windows_sys::core::PCWSTR,
    pub FunctionName: windows_sys::core::PCWSTR,
    pub ExtensionImageName: windows_sys::core::PCWSTR,
    pub ExtensionFriendlyName: windows_sys::core::PCWSTR,
}
impl Default for FAX_GLOBAL_ROUTING_INFOW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type FAX_GROUP_STATUS_ENUM = i32;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct FAX_JOB_ENTRYA {
    pub SizeOfStruct: u32,
    pub JobId: u32,
    pub UserName: windows_sys::core::PCSTR,
    pub JobType: u32,
    pub QueueStatus: u32,
    pub Status: u32,
    pub Size: u32,
    pub PageCount: u32,
    pub RecipientNumber: windows_sys::core::PCSTR,
    pub RecipientName: windows_sys::core::PCSTR,
    pub Tsid: windows_sys::core::PCSTR,
    pub SenderName: windows_sys::core::PCSTR,
    pub SenderCompany: windows_sys::core::PCSTR,
    pub SenderDept: windows_sys::core::PCSTR,
    pub BillingCode: windows_sys::core::PCSTR,
    pub ScheduleAction: u32,
    pub ScheduleTime: super::super::Foundation::SYSTEMTIME,
    pub DeliveryReportType: u32,
    pub DeliveryReportAddress: windows_sys::core::PCSTR,
    pub DocumentName: windows_sys::core::PCSTR,
}
impl Default for FAX_JOB_ENTRYA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct FAX_JOB_ENTRYW {
    pub SizeOfStruct: u32,
    pub JobId: u32,
    pub UserName: windows_sys::core::PCWSTR,
    pub JobType: u32,
    pub QueueStatus: u32,
    pub Status: u32,
    pub Size: u32,
    pub PageCount: u32,
    pub RecipientNumber: windows_sys::core::PCWSTR,
    pub RecipientName: windows_sys::core::PCWSTR,
    pub Tsid: windows_sys::core::PCWSTR,
    pub SenderName: windows_sys::core::PCWSTR,
    pub SenderCompany: windows_sys::core::PCWSTR,
    pub SenderDept: windows_sys::core::PCWSTR,
    pub BillingCode: windows_sys::core::PCWSTR,
    pub ScheduleAction: u32,
    pub ScheduleTime: super::super::Foundation::SYSTEMTIME,
    pub DeliveryReportType: u32,
    pub DeliveryReportAddress: windows_sys::core::PCWSTR,
    pub DocumentName: windows_sys::core::PCWSTR,
}
impl Default for FAX_JOB_ENTRYW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type FAX_JOB_EXTENDED_STATUS_ENUM = i32;
pub const FAX_JOB_MANAGE: u32 = 64u32;
pub type FAX_JOB_OPERATIONS_ENUM = i32;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct FAX_JOB_PARAMA {
    pub SizeOfStruct: u32,
    pub RecipientNumber: windows_sys::core::PCSTR,
    pub RecipientName: windows_sys::core::PCSTR,
    pub Tsid: windows_sys::core::PCSTR,
    pub SenderName: windows_sys::core::PCSTR,
    pub SenderCompany: windows_sys::core::PCSTR,
    pub SenderDept: windows_sys::core::PCSTR,
    pub BillingCode: windows_sys::core::PCSTR,
    pub ScheduleAction: u32,
    pub ScheduleTime: super::super::Foundation::SYSTEMTIME,
    pub DeliveryReportType: u32,
    pub DeliveryReportAddress: windows_sys::core::PCSTR,
    pub DocumentName: windows_sys::core::PCSTR,
    pub CallHandle: u32,
    pub Reserved: [usize; 3],
}
impl Default for FAX_JOB_PARAMA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct FAX_JOB_PARAMW {
    pub SizeOfStruct: u32,
    pub RecipientNumber: windows_sys::core::PCWSTR,
    pub RecipientName: windows_sys::core::PCWSTR,
    pub Tsid: windows_sys::core::PCWSTR,
    pub SenderName: windows_sys::core::PCWSTR,
    pub SenderCompany: windows_sys::core::PCWSTR,
    pub SenderDept: windows_sys::core::PCWSTR,
    pub BillingCode: windows_sys::core::PCWSTR,
    pub ScheduleAction: u32,
    pub ScheduleTime: super::super::Foundation::SYSTEMTIME,
    pub DeliveryReportType: u32,
    pub DeliveryReportAddress: windows_sys::core::PCWSTR,
    pub DocumentName: windows_sys::core::PCWSTR,
    pub CallHandle: u32,
    pub Reserved: [usize; 3],
}
impl Default for FAX_JOB_PARAMW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const FAX_JOB_QUERY: u32 = 2u32;
pub type FAX_JOB_STATUS_ENUM = i32;
pub const FAX_JOB_SUBMIT: u32 = 1u32;
pub type FAX_JOB_TYPE_ENUM = i32;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct FAX_LOG_CATEGORYA {
    pub Name: windows_sys::core::PCSTR,
    pub Category: u32,
    pub Level: u32,
}
impl Default for FAX_LOG_CATEGORYA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct FAX_LOG_CATEGORYW {
    pub Name: windows_sys::core::PCWSTR,
    pub Category: u32,
    pub Level: u32,
}
impl Default for FAX_LOG_CATEGORYW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type FAX_LOG_LEVEL_ENUM = i32;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct FAX_PORT_INFOA {
    pub SizeOfStruct: u32,
    pub DeviceId: u32,
    pub State: u32,
    pub Flags: u32,
    pub Rings: u32,
    pub Priority: u32,
    pub DeviceName: windows_sys::core::PCSTR,
    pub Tsid: windows_sys::core::PCSTR,
    pub Csid: windows_sys::core::PCSTR,
}
impl Default for FAX_PORT_INFOA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct FAX_PORT_INFOW {
    pub SizeOfStruct: u32,
    pub DeviceId: u32,
    pub State: u32,
    pub Flags: u32,
    pub Rings: u32,
    pub Priority: u32,
    pub DeviceName: windows_sys::core::PCWSTR,
    pub Tsid: windows_sys::core::PCWSTR,
    pub Csid: windows_sys::core::PCWSTR,
}
impl Default for FAX_PORT_INFOW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const FAX_PORT_QUERY: u32 = 16u32;
pub const FAX_PORT_SET: u32 = 32u32;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct FAX_PRINT_INFOA {
    pub SizeOfStruct: u32,
    pub DocName: windows_sys::core::PCSTR,
    pub RecipientName: windows_sys::core::PCSTR,
    pub RecipientNumber: windows_sys::core::PCSTR,
    pub SenderName: windows_sys::core::PCSTR,
    pub SenderCompany: windows_sys::core::PCSTR,
    pub SenderDept: windows_sys::core::PCSTR,
    pub SenderBillingCode: windows_sys::core::PCSTR,
    pub Reserved: windows_sys::core::PCSTR,
    pub DrEmailAddress: windows_sys::core::PCSTR,
    pub OutputFileName: windows_sys::core::PCSTR,
}
impl Default for FAX_PRINT_INFOA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct FAX_PRINT_INFOW {
    pub SizeOfStruct: u32,
    pub DocName: windows_sys::core::PCWSTR,
    pub RecipientName: windows_sys::core::PCWSTR,
    pub RecipientNumber: windows_sys::core::PCWSTR,
    pub SenderName: windows_sys::core::PCWSTR,
    pub SenderCompany: windows_sys::core::PCWSTR,
    pub SenderDept: windows_sys::core::PCWSTR,
    pub SenderBillingCode: windows_sys::core::PCWSTR,
    pub Reserved: windows_sys::core::PCWSTR,
    pub DrEmailAddress: windows_sys::core::PCWSTR,
    pub OutputFileName: windows_sys::core::PCWSTR,
}
impl Default for FAX_PRINT_INFOW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type FAX_PRIORITY_TYPE_ENUM = i32;
pub type FAX_PROVIDER_STATUS_ENUM = i32;
pub type FAX_RECEIPT_TYPE_ENUM = i32;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct FAX_RECEIVE {
    pub SizeOfStruct: u32,
    pub FileName: windows_sys::core::PWSTR,
    pub ReceiverName: windows_sys::core::PWSTR,
    pub ReceiverNumber: windows_sys::core::PWSTR,
    pub Reserved: [u32; 4],
}
impl Default for FAX_RECEIVE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct FAX_ROUTE {
    pub SizeOfStruct: u32,
    pub JobId: u32,
    pub ElapsedTime: u64,
    pub ReceiveTime: u64,
    pub PageCount: u32,
    pub Csid: windows_sys::core::PCWSTR,
    pub Tsid: windows_sys::core::PCWSTR,
    pub CallerId: windows_sys::core::PCWSTR,
    pub RoutingInfo: windows_sys::core::PCWSTR,
    pub ReceiverName: windows_sys::core::PCWSTR,
    pub ReceiverNumber: windows_sys::core::PCWSTR,
    pub DeviceName: windows_sys::core::PCWSTR,
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
#[derive(Clone, Copy, Default)]
pub struct FAX_ROUTE_CALLBACKROUTINES {
    pub SizeOfStruct: u32,
    pub FaxRouteAddFile: PFAXROUTEADDFILE,
    pub FaxRouteDeleteFile: PFAXROUTEDELETEFILE,
    pub FaxRouteGetFile: PFAXROUTEGETFILE,
    pub FaxRouteEnumFiles: PFAXROUTEENUMFILES,
    pub FaxRouteModifyRoutingData: PFAXROUTEMODIFYROUTINGDATA,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct FAX_ROUTING_METHODA {
    pub SizeOfStruct: u32,
    pub DeviceId: u32,
    pub Enabled: windows_sys::core::BOOL,
    pub DeviceName: windows_sys::core::PCSTR,
    pub Guid: windows_sys::core::PCSTR,
    pub FriendlyName: windows_sys::core::PCSTR,
    pub FunctionName: windows_sys::core::PCSTR,
    pub ExtensionImageName: windows_sys::core::PCSTR,
    pub ExtensionFriendlyName: windows_sys::core::PCSTR,
}
impl Default for FAX_ROUTING_METHODA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct FAX_ROUTING_METHODW {
    pub SizeOfStruct: u32,
    pub DeviceId: u32,
    pub Enabled: windows_sys::core::BOOL,
    pub DeviceName: windows_sys::core::PCWSTR,
    pub Guid: windows_sys::core::PCWSTR,
    pub FriendlyName: windows_sys::core::PCWSTR,
    pub FunctionName: windows_sys::core::PCWSTR,
    pub ExtensionImageName: windows_sys::core::PCWSTR,
    pub ExtensionFriendlyName: windows_sys::core::PCWSTR,
}
impl Default for FAX_ROUTING_METHODW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type FAX_ROUTING_RULE_CODE_ENUM = i32;
pub type FAX_RULE_STATUS_ENUM = i32;
pub type FAX_SCHEDULE_TYPE_ENUM = i32;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct FAX_SEND {
    pub SizeOfStruct: u32,
    pub FileName: windows_sys::core::PWSTR,
    pub CallerName: windows_sys::core::PWSTR,
    pub CallerNumber: windows_sys::core::PWSTR,
    pub ReceiverName: windows_sys::core::PWSTR,
    pub ReceiverNumber: windows_sys::core::PWSTR,
    pub Branding: windows_sys::core::BOOL,
    pub CallHandle: u32,
    pub Reserved: [u32; 3],
}
impl Default for FAX_SEND {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type FAX_SERVER_APIVERSION_ENUM = i32;
pub type FAX_SERVER_EVENTS_TYPE_ENUM = i32;
pub type FAX_SMTP_AUTHENTICATION_TYPE_ENUM = i32;
#[repr(C)]
#[derive(Clone, Copy, Default)]
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
pub const FaxAccount: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xa7e0647f_4524_4464_a56d_b9fe666f715e);
pub const FaxAccountFolders: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x85398f49_c034_4a3f_821c_db7d685e8129);
pub const FaxAccountIncomingArchive: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x14b33db5_4c40_4ecf_9ef8_a360cbe809ed);
pub const FaxAccountIncomingQueue: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x9bcf6094_b4da_45f4_b8d6_ddeb2186652c);
pub const FaxAccountOutgoingArchive: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x851e7af5_433a_4739_a2df_ad245c2cb98e);
pub const FaxAccountOutgoingQueue: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xfeeceefb_c149_48ba_bab8_b791e101f62f);
pub const FaxAccountSet: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xfbc23c4b_79e0_4291_bc56_c12e253bbf3a);
pub const FaxAccounts: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xda1f94aa_ee2c_47c0_8f4f_2a217075b76e);
pub const FaxActivity: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xcfef5d0e_e84d_462e_aabb_87d31eb04fef);
pub const FaxActivityLogging: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xf0a0294e_3bbd_48b8_8f13_8c591a55bdbc);
pub const FaxConfiguration: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x5857326f_e7b3_41a7_9c19_a91b463e2d56);
pub const FaxDevice: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x59e3a5b2_d676_484b_a6de_720bfa89b5af);
pub const FaxDeviceIds: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xcdc539ea_7277_460e_8de0_48a0a5760d1f);
pub const FaxDeviceProvider: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x17cf1aa3_f5eb_484a_9c9a_4440a5baabfc);
pub const FaxDeviceProviders: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xeb8fe768_875a_4f5f_82c5_03f23aac1bd7);
pub const FaxDevices: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x5589e28e_23cb_4919_8808_e6101846e80d);
pub const FaxDocument: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x0f3f9f91_c838_415e_a4f3_3e828ca445e0);
pub const FaxEventLogging: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xa6850930_a0f6_4a6f_95b7_db2ebf3d02e3);
pub const FaxFolders: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xc35211d7_5776_48cb_af44_c31be3b2cfe5);
pub const FaxInboundRouting: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xe80248ed_ad65_4218_8108_991924d4e7ed);
pub const FaxInboundRoutingExtension: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x1d7dfb51_7207_4436_a0d9_24e32ee56988);
pub const FaxInboundRoutingExtensions: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x189a48ed_623c_4c0d_80f2_d66c7b9efec2);
pub const FaxInboundRoutingMethod: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x4b9fd75c_0194_4b72_9ce5_02a8205ac7d4);
pub const FaxInboundRoutingMethods: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x25fcb76a_b750_4b82_9266_fbbbae8922ba);
pub const FaxIncomingArchive: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x8426c56a_35a1_4c6f_af93_fc952422e2c2);
pub const FaxIncomingJob: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xc47311ec_ae32_41b8_ae4b_3eae0629d0c9);
pub const FaxIncomingJobs: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xa1bb8a43_8866_4fb7_a15d_6266c875a5cc);
pub const FaxIncomingMessage: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x1932fcf7_9d43_4d5a_89ff_03861b321736);
pub const FaxIncomingMessageIterator: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x6088e1d8_3fc8_45c2_87b1_909a29607ea9);
pub const FaxIncomingQueue: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x69131717_f3f1_40e3_809d_a6cbf7bd85e5);
pub const FaxJobStatus: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x7bf222f4_be8d_442f_841d_6132742423bb);
pub const FaxLoggingOptions: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x1bf9eea6_ece0_4785_a18b_de56e9eef96a);
pub const FaxOutboundRouting: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xc81b385e_b869_4afd_86c0_616498ed9be2);
pub const FaxOutboundRoutingGroup: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x0213f3e0_6791_4d77_a271_04d2357c50d6);
pub const FaxOutboundRoutingGroups: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xccbea1a5_e2b4_4b57_9421_b04b6289464b);
pub const FaxOutboundRoutingRule: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x6549eebf_08d1_475a_828b_3bf105952fa0);
pub const FaxOutboundRoutingRules: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xd385beca_e624_4473_bfaa_9f4000831f54);
pub const FaxOutgoingArchive: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x43c28403_e04f_474d_990c_b94669148f59);
pub const FaxOutgoingJob: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x71bb429c_0ef9_4915_bec5_a5d897a3e924);
pub const FaxOutgoingJobs: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x92bf2a6c_37be_43fa_a37d_cb0e5f753b35);
pub const FaxOutgoingMessage: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x91b4a378_4ad8_4aef_a4dc_97d96e939a3a);
pub const FaxOutgoingMessageIterator: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x8a3224d0_d30b_49de_9813_cb385790fbbb);
pub const FaxOutgoingQueue: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x7421169e_8c43_4b0d_bb16_645c8fa40357);
pub const FaxReceiptOptions: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x6982487b_227b_4c96_a61c_248348b05ab6);
pub const FaxRecipient: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x60bf3301_7df8_4bd8_9148_7b5801f9efdf);
pub const FaxRecipients: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xea9bdf53_10a9_4d4f_a067_63c8f84f01b0);
pub const FaxSecurity: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x10c4ddde_abf0_43df_964f_7f3ac21a4c7b);
pub const FaxSecurity2: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x735c1248_ec89_4c30_a127_656e92e3c4ea);
pub const FaxSender: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x265d84d0_1850_4360_b7c8_758bbb5f0b96);
pub const FaxServer: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xcda8acb0_8cf5_4f6c_9ba2_5931d40c8cae);
pub const GUID_DeviceArrivedLaunch: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x740d9ee6_70f1_11d1_ad10_00a02438ad48);
pub const GUID_STIUserDefined1: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xc00eb795_8c6e_11d2_977a_0000f87a926f);
pub const GUID_STIUserDefined2: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xc77ae9c5_8c6e_11d2_977a_0000f87a926f);
pub const GUID_STIUserDefined3: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xc77ae9c6_8c6e_11d2_977a_0000f87a926f);
pub const GUID_ScanFaxImage: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xc00eb793_8c6e_11d2_977a_0000f87a926f);
pub const GUID_ScanImage: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xa6c5a715_8c6e_11d2_977a_0000f87a926f);
pub const GUID_ScanPrintImage: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xb441f425_8c6e_11d2_977a_0000f87a926f);
pub const IS_DIGITAL_CAMERA_STR: windows_sys::core::PCWSTR = windows_sys::core::w!("IsDigitalCamera");
pub const IS_DIGITAL_CAMERA_VAL: u32 = 1u32;
pub const JC_DELETE: FAX_ENUM_JOB_COMMANDS = 1i32;
pub const JC_PAUSE: FAX_ENUM_JOB_COMMANDS = 2i32;
pub const JC_RESUME: FAX_ENUM_JOB_COMMANDS = 3i32;
pub const JC_UNKNOWN: FAX_ENUM_JOB_COMMANDS = 0i32;
pub const JSA_DISCOUNT_PERIOD: FAX_ENUM_JOB_SEND_ATTRIBUTES = 2i32;
pub const JSA_NOW: FAX_ENUM_JOB_SEND_ATTRIBUTES = 0i32;
pub const JSA_SPECIFIC_TIME: FAX_ENUM_JOB_SEND_ATTRIBUTES = 1i32;
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
pub const MS_FAXROUTE_EMAIL_GUID: windows_sys::core::PCWSTR = windows_sys::core::w!("{6bbf7bfe-9af2-11d0-abf7-00c04fd91a4e}");
pub const MS_FAXROUTE_FOLDER_GUID: windows_sys::core::PCWSTR = windows_sys::core::w!("{92041a90-9af2-11d0-abf7-00c04fd91a4e}");
pub const MS_FAXROUTE_PRINTING_GUID: windows_sys::core::PCWSTR = windows_sys::core::w!("{aec1b37c-9af2-11d0-abf7-00c04fd91a4e}");
pub type PFAXABORT = Option<unsafe extern "system" fn(faxhandle: super::super::Foundation::HANDLE, jobid: u32) -> windows_sys::core::BOOL>;
pub type PFAXACCESSCHECK = Option<unsafe extern "system" fn(faxhandle: super::super::Foundation::HANDLE, accessmask: u32) -> windows_sys::core::BOOL>;
pub type PFAXCLOSE = Option<unsafe extern "system" fn(faxhandle: super::super::Foundation::HANDLE) -> windows_sys::core::BOOL>;
pub type PFAXCOMPLETEJOBPARAMSA = Option<unsafe extern "system" fn(jobparams: *mut *mut FAX_JOB_PARAMA, coverpageinfo: *mut *mut FAX_COVERPAGE_INFOA) -> windows_sys::core::BOOL>;
pub type PFAXCOMPLETEJOBPARAMSW = Option<unsafe extern "system" fn(jobparams: *mut *mut FAX_JOB_PARAMW, coverpageinfo: *mut *mut FAX_COVERPAGE_INFOW) -> windows_sys::core::BOOL>;
pub type PFAXCONNECTFAXSERVERA = Option<unsafe extern "system" fn(machinename: windows_sys::core::PCSTR, faxhandle: *mut super::super::Foundation::HANDLE) -> windows_sys::core::BOOL>;
pub type PFAXCONNECTFAXSERVERW = Option<unsafe extern "system" fn(machinename: windows_sys::core::PCWSTR, faxhandle: *mut super::super::Foundation::HANDLE) -> windows_sys::core::BOOL>;
pub type PFAXDEVABORTOPERATION = Option<unsafe extern "system" fn(param0: super::super::Foundation::HANDLE) -> windows_sys::core::BOOL>;
#[cfg(feature = "Win32_UI_Controls")]
pub type PFAXDEVCONFIGURE = Option<unsafe extern "system" fn(param0: *mut super::super::UI::Controls::HPROPSHEETPAGE) -> windows_sys::core::BOOL>;
pub type PFAXDEVENDJOB = Option<unsafe extern "system" fn(param0: super::super::Foundation::HANDLE) -> windows_sys::core::BOOL>;
pub type PFAXDEVINITIALIZE = Option<unsafe extern "system" fn(param0: u32, param1: super::super::Foundation::HANDLE, param2: *mut PFAX_LINECALLBACK, param3: PFAX_SERVICE_CALLBACK) -> windows_sys::core::BOOL>;
pub type PFAXDEVRECEIVE = Option<unsafe extern "system" fn(param0: super::super::Foundation::HANDLE, param1: u32, param2: *mut FAX_RECEIVE) -> windows_sys::core::BOOL>;
pub type PFAXDEVREPORTSTATUS = Option<unsafe extern "system" fn(param0: super::super::Foundation::HANDLE, param1: *mut FAX_DEV_STATUS, param2: u32, param3: *mut u32) -> windows_sys::core::BOOL>;
pub type PFAXDEVSEND = Option<unsafe extern "system" fn(param0: super::super::Foundation::HANDLE, param1: *mut FAX_SEND, param2: PFAX_SEND_CALLBACK) -> windows_sys::core::BOOL>;
pub type PFAXDEVSHUTDOWN = Option<unsafe extern "system" fn() -> windows_sys::core::HRESULT>;
pub type PFAXDEVSTARTJOB = Option<unsafe extern "system" fn(param0: u32, param1: u32, param2: *mut super::super::Foundation::HANDLE, param3: super::super::Foundation::HANDLE, param4: usize) -> windows_sys::core::BOOL>;
pub type PFAXDEVVIRTUALDEVICECREATION = Option<unsafe extern "system" fn(devicecount: *mut u32, devicenameprefix: windows_sys::core::PWSTR, deviceidprefix: *mut u32, completionport: super::super::Foundation::HANDLE, completionkey: usize) -> windows_sys::core::BOOL>;
pub type PFAXENABLEROUTINGMETHODA = Option<unsafe extern "system" fn(faxporthandle: super::super::Foundation::HANDLE, routingguid: windows_sys::core::PCSTR, enabled: windows_sys::core::BOOL) -> windows_sys::core::BOOL>;
pub type PFAXENABLEROUTINGMETHODW = Option<unsafe extern "system" fn(faxporthandle: super::super::Foundation::HANDLE, routingguid: windows_sys::core::PCWSTR, enabled: windows_sys::core::BOOL) -> windows_sys::core::BOOL>;
pub type PFAXENUMGLOBALROUTINGINFOA = Option<unsafe extern "system" fn(faxhandle: super::super::Foundation::HANDLE, routinginfo: *mut *mut FAX_GLOBAL_ROUTING_INFOA, methodsreturned: *mut u32) -> windows_sys::core::BOOL>;
pub type PFAXENUMGLOBALROUTINGINFOW = Option<unsafe extern "system" fn(faxhandle: super::super::Foundation::HANDLE, routinginfo: *mut *mut FAX_GLOBAL_ROUTING_INFOW, methodsreturned: *mut u32) -> windows_sys::core::BOOL>;
pub type PFAXENUMJOBSA = Option<unsafe extern "system" fn(faxhandle: super::super::Foundation::HANDLE, jobentry: *mut *mut FAX_JOB_ENTRYA, jobsreturned: *mut u32) -> windows_sys::core::BOOL>;
pub type PFAXENUMJOBSW = Option<unsafe extern "system" fn(faxhandle: super::super::Foundation::HANDLE, jobentry: *mut *mut FAX_JOB_ENTRYW, jobsreturned: *mut u32) -> windows_sys::core::BOOL>;
pub type PFAXENUMPORTSA = Option<unsafe extern "system" fn(faxhandle: super::super::Foundation::HANDLE, portinfo: *mut *mut FAX_PORT_INFOA, portsreturned: *mut u32) -> windows_sys::core::BOOL>;
pub type PFAXENUMPORTSW = Option<unsafe extern "system" fn(faxhandle: super::super::Foundation::HANDLE, portinfo: *mut *mut FAX_PORT_INFOW, portsreturned: *mut u32) -> windows_sys::core::BOOL>;
pub type PFAXENUMROUTINGMETHODSA = Option<unsafe extern "system" fn(faxporthandle: super::super::Foundation::HANDLE, routingmethod: *mut *mut FAX_ROUTING_METHODA, methodsreturned: *mut u32) -> windows_sys::core::BOOL>;
pub type PFAXENUMROUTINGMETHODSW = Option<unsafe extern "system" fn(faxporthandle: super::super::Foundation::HANDLE, routingmethod: *mut *mut FAX_ROUTING_METHODW, methodsreturned: *mut u32) -> windows_sys::core::BOOL>;
pub type PFAXFREEBUFFER = Option<unsafe extern "system" fn(buffer: *mut core::ffi::c_void)>;
pub type PFAXGETCONFIGURATIONA = Option<unsafe extern "system" fn(faxhandle: super::super::Foundation::HANDLE, faxconfig: *mut *mut FAX_CONFIGURATIONA) -> windows_sys::core::BOOL>;
pub type PFAXGETCONFIGURATIONW = Option<unsafe extern "system" fn(faxhandle: super::super::Foundation::HANDLE, faxconfig: *mut *mut FAX_CONFIGURATIONW) -> windows_sys::core::BOOL>;
pub type PFAXGETDEVICESTATUSA = Option<unsafe extern "system" fn(faxporthandle: super::super::Foundation::HANDLE, devicestatus: *mut *mut FAX_DEVICE_STATUSA) -> windows_sys::core::BOOL>;
pub type PFAXGETDEVICESTATUSW = Option<unsafe extern "system" fn(faxporthandle: super::super::Foundation::HANDLE, devicestatus: *mut *mut FAX_DEVICE_STATUSW) -> windows_sys::core::BOOL>;
pub type PFAXGETJOBA = Option<unsafe extern "system" fn(faxhandle: super::super::Foundation::HANDLE, jobid: u32, jobentry: *mut *mut FAX_JOB_ENTRYA) -> windows_sys::core::BOOL>;
pub type PFAXGETJOBW = Option<unsafe extern "system" fn(faxhandle: super::super::Foundation::HANDLE, jobid: u32, jobentry: *mut *mut FAX_JOB_ENTRYW) -> windows_sys::core::BOOL>;
pub type PFAXGETLOGGINGCATEGORIESA = Option<unsafe extern "system" fn(faxhandle: super::super::Foundation::HANDLE, categories: *mut *mut FAX_LOG_CATEGORYA, numbercategories: *mut u32) -> windows_sys::core::BOOL>;
pub type PFAXGETLOGGINGCATEGORIESW = Option<unsafe extern "system" fn(faxhandle: super::super::Foundation::HANDLE, categories: *mut *mut FAX_LOG_CATEGORYW, numbercategories: *mut u32) -> windows_sys::core::BOOL>;
pub type PFAXGETPAGEDATA = Option<unsafe extern "system" fn(faxhandle: super::super::Foundation::HANDLE, jobid: u32, buffer: *mut *mut u8, buffersize: *mut u32, imagewidth: *mut u32, imageheight: *mut u32) -> windows_sys::core::BOOL>;
pub type PFAXGETPORTA = Option<unsafe extern "system" fn(faxporthandle: super::super::Foundation::HANDLE, portinfo: *mut *mut FAX_PORT_INFOA) -> windows_sys::core::BOOL>;
pub type PFAXGETPORTW = Option<unsafe extern "system" fn(faxporthandle: super::super::Foundation::HANDLE, portinfo: *mut *mut FAX_PORT_INFOW) -> windows_sys::core::BOOL>;
pub type PFAXGETROUTINGINFOA = Option<unsafe extern "system" fn(faxporthandle: super::super::Foundation::HANDLE, routingguid: windows_sys::core::PCSTR, routinginfobuffer: *mut *mut u8, routinginfobuffersize: *mut u32) -> windows_sys::core::BOOL>;
pub type PFAXGETROUTINGINFOW = Option<unsafe extern "system" fn(faxporthandle: super::super::Foundation::HANDLE, routingguid: windows_sys::core::PCWSTR, routinginfobuffer: *mut *mut u8, routinginfobuffersize: *mut u32) -> windows_sys::core::BOOL>;
pub type PFAXINITIALIZEEVENTQUEUE = Option<unsafe extern "system" fn(faxhandle: super::super::Foundation::HANDLE, completionport: super::super::Foundation::HANDLE, completionkey: usize, hwnd: super::super::Foundation::HWND, messagestart: u32) -> windows_sys::core::BOOL>;
pub type PFAXOPENPORT = Option<unsafe extern "system" fn(faxhandle: super::super::Foundation::HANDLE, deviceid: u32, flags: u32, faxporthandle: *mut super::super::Foundation::HANDLE) -> windows_sys::core::BOOL>;
#[cfg(feature = "Win32_Graphics_Gdi")]
pub type PFAXPRINTCOVERPAGEA = Option<unsafe extern "system" fn(faxcontextinfo: *const FAX_CONTEXT_INFOA, coverpageinfo: *const FAX_COVERPAGE_INFOA) -> windows_sys::core::BOOL>;
#[cfg(feature = "Win32_Graphics_Gdi")]
pub type PFAXPRINTCOVERPAGEW = Option<unsafe extern "system" fn(faxcontextinfo: *const FAX_CONTEXT_INFOW, coverpageinfo: *const FAX_COVERPAGE_INFOW) -> windows_sys::core::BOOL>;
pub type PFAXREGISTERROUTINGEXTENSIONW = Option<unsafe extern "system" fn(faxhandle: super::super::Foundation::HANDLE, extensionname: windows_sys::core::PCWSTR, friendlyname: windows_sys::core::PCWSTR, imagename: windows_sys::core::PCWSTR, callback: PFAX_ROUTING_INSTALLATION_CALLBACKW, context: *const core::ffi::c_void) -> windows_sys::core::BOOL>;
pub type PFAXREGISTERSERVICEPROVIDERW = Option<unsafe extern "system" fn(deviceprovider: windows_sys::core::PCWSTR, friendlyname: windows_sys::core::PCWSTR, imagename: windows_sys::core::PCWSTR, tspname: windows_sys::core::PCWSTR) -> windows_sys::core::BOOL>;
pub type PFAXROUTEADDFILE = Option<unsafe extern "system" fn(jobid: u32, filename: windows_sys::core::PCWSTR, guid: *mut windows_sys::core::GUID) -> i32>;
pub type PFAXROUTEDELETEFILE = Option<unsafe extern "system" fn(jobid: u32, filename: windows_sys::core::PCWSTR) -> i32>;
pub type PFAXROUTEDEVICECHANGENOTIFICATION = Option<unsafe extern "system" fn(param0: u32, param1: windows_sys::core::BOOL) -> windows_sys::core::BOOL>;
pub type PFAXROUTEDEVICEENABLE = Option<unsafe extern "system" fn(param0: windows_sys::core::PCWSTR, param1: u32, param2: i32) -> windows_sys::core::BOOL>;
pub type PFAXROUTEENUMFILE = Option<unsafe extern "system" fn(jobid: u32, guidowner: *mut windows_sys::core::GUID, guidcaller: *mut windows_sys::core::GUID, filename: windows_sys::core::PCWSTR, context: *mut core::ffi::c_void) -> windows_sys::core::BOOL>;
pub type PFAXROUTEENUMFILES = Option<unsafe extern "system" fn(jobid: u32, guid: *mut windows_sys::core::GUID, fileenumerator: PFAXROUTEENUMFILE, context: *mut core::ffi::c_void) -> windows_sys::core::BOOL>;
pub type PFAXROUTEGETFILE = Option<unsafe extern "system" fn(jobid: u32, index: u32, filenamebuffer: windows_sys::core::PWSTR, requiredsize: *mut u32) -> windows_sys::core::BOOL>;
pub type PFAXROUTEGETROUTINGINFO = Option<unsafe extern "system" fn(param0: windows_sys::core::PCWSTR, param1: u32, param2: *mut u8, param3: *mut u32) -> windows_sys::core::BOOL>;
pub type PFAXROUTEINITIALIZE = Option<unsafe extern "system" fn(param0: super::super::Foundation::HANDLE, param1: *mut FAX_ROUTE_CALLBACKROUTINES) -> windows_sys::core::BOOL>;
pub type PFAXROUTEMETHOD = Option<unsafe extern "system" fn(param0: *const FAX_ROUTE, param1: *mut *mut core::ffi::c_void, param2: *mut u32) -> windows_sys::core::BOOL>;
pub type PFAXROUTEMODIFYROUTINGDATA = Option<unsafe extern "system" fn(jobid: u32, routingguid: windows_sys::core::PCWSTR, routingdata: *mut u8, routingdatasize: u32) -> windows_sys::core::BOOL>;
pub type PFAXROUTESETROUTINGINFO = Option<unsafe extern "system" fn(param0: windows_sys::core::PCWSTR, param1: u32, param2: *const u8, param3: u32) -> windows_sys::core::BOOL>;
pub type PFAXSENDDOCUMENTA = Option<unsafe extern "system" fn(faxhandle: super::super::Foundation::HANDLE, filename: windows_sys::core::PCSTR, jobparams: *const FAX_JOB_PARAMA, coverpageinfo: *const FAX_COVERPAGE_INFOA, faxjobid: *mut u32) -> windows_sys::core::BOOL>;
pub type PFAXSENDDOCUMENTFORBROADCASTA = Option<unsafe extern "system" fn(faxhandle: super::super::Foundation::HANDLE, filename: windows_sys::core::PCSTR, faxjobid: *mut u32, faxrecipientcallback: PFAX_RECIPIENT_CALLBACKA, context: *const core::ffi::c_void) -> windows_sys::core::BOOL>;
pub type PFAXSENDDOCUMENTFORBROADCASTW = Option<unsafe extern "system" fn(faxhandle: super::super::Foundation::HANDLE, filename: windows_sys::core::PCWSTR, faxjobid: *mut u32, faxrecipientcallback: PFAX_RECIPIENT_CALLBACKW, context: *const core::ffi::c_void) -> windows_sys::core::BOOL>;
pub type PFAXSENDDOCUMENTW = Option<unsafe extern "system" fn(faxhandle: super::super::Foundation::HANDLE, filename: windows_sys::core::PCWSTR, jobparams: *const FAX_JOB_PARAMW, coverpageinfo: *const FAX_COVERPAGE_INFOW, faxjobid: *mut u32) -> windows_sys::core::BOOL>;
pub type PFAXSETCONFIGURATIONA = Option<unsafe extern "system" fn(faxhandle: super::super::Foundation::HANDLE, faxconfig: *const FAX_CONFIGURATIONA) -> windows_sys::core::BOOL>;
pub type PFAXSETCONFIGURATIONW = Option<unsafe extern "system" fn(faxhandle: super::super::Foundation::HANDLE, faxconfig: *const FAX_CONFIGURATIONW) -> windows_sys::core::BOOL>;
pub type PFAXSETGLOBALROUTINGINFOA = Option<unsafe extern "system" fn(faxporthandle: super::super::Foundation::HANDLE, routinginfo: *const FAX_GLOBAL_ROUTING_INFOA) -> windows_sys::core::BOOL>;
pub type PFAXSETGLOBALROUTINGINFOW = Option<unsafe extern "system" fn(faxporthandle: super::super::Foundation::HANDLE, routinginfo: *const FAX_GLOBAL_ROUTING_INFOW) -> windows_sys::core::BOOL>;
pub type PFAXSETJOBA = Option<unsafe extern "system" fn(faxhandle: super::super::Foundation::HANDLE, jobid: u32, command: u32, jobentry: *const FAX_JOB_ENTRYA) -> windows_sys::core::BOOL>;
pub type PFAXSETJOBW = Option<unsafe extern "system" fn(faxhandle: super::super::Foundation::HANDLE, jobid: u32, command: u32, jobentry: *const FAX_JOB_ENTRYW) -> windows_sys::core::BOOL>;
pub type PFAXSETLOGGINGCATEGORIESA = Option<unsafe extern "system" fn(faxhandle: super::super::Foundation::HANDLE, categories: *const FAX_LOG_CATEGORYA, numbercategories: u32) -> windows_sys::core::BOOL>;
pub type PFAXSETLOGGINGCATEGORIESW = Option<unsafe extern "system" fn(faxhandle: super::super::Foundation::HANDLE, categories: *const FAX_LOG_CATEGORYW, numbercategories: u32) -> windows_sys::core::BOOL>;
pub type PFAXSETPORTA = Option<unsafe extern "system" fn(faxporthandle: super::super::Foundation::HANDLE, portinfo: *const FAX_PORT_INFOA) -> windows_sys::core::BOOL>;
pub type PFAXSETPORTW = Option<unsafe extern "system" fn(faxporthandle: super::super::Foundation::HANDLE, portinfo: *const FAX_PORT_INFOW) -> windows_sys::core::BOOL>;
pub type PFAXSETROUTINGINFOA = Option<unsafe extern "system" fn(faxporthandle: super::super::Foundation::HANDLE, routingguid: windows_sys::core::PCSTR, routinginfobuffer: *const u8, routinginfobuffersize: u32) -> windows_sys::core::BOOL>;
pub type PFAXSETROUTINGINFOW = Option<unsafe extern "system" fn(faxporthandle: super::super::Foundation::HANDLE, routingguid: windows_sys::core::PCWSTR, routinginfobuffer: *const u8, routinginfobuffersize: u32) -> windows_sys::core::BOOL>;
#[cfg(feature = "Win32_Graphics_Gdi")]
pub type PFAXSTARTPRINTJOBA = Option<unsafe extern "system" fn(printername: windows_sys::core::PCSTR, printinfo: *const FAX_PRINT_INFOA, faxjobid: *mut u32, faxcontextinfo: *mut FAX_CONTEXT_INFOA) -> windows_sys::core::BOOL>;
#[cfg(feature = "Win32_Graphics_Gdi")]
pub type PFAXSTARTPRINTJOBW = Option<unsafe extern "system" fn(printername: windows_sys::core::PCWSTR, printinfo: *const FAX_PRINT_INFOW, faxjobid: *mut u32, faxcontextinfo: *mut FAX_CONTEXT_INFOW) -> windows_sys::core::BOOL>;
pub type PFAXUNREGISTERSERVICEPROVIDERW = Option<unsafe extern "system" fn(deviceprovider: windows_sys::core::PCWSTR) -> windows_sys::core::BOOL>;
pub type PFAX_EXT_CONFIG_CHANGE = Option<unsafe extern "system" fn(param0: u32, param1: windows_sys::core::PCWSTR, param2: *mut u8, param3: u32) -> windows_sys::core::HRESULT>;
pub type PFAX_EXT_FREE_BUFFER = Option<unsafe extern "system" fn(param0: *mut core::ffi::c_void)>;
pub type PFAX_EXT_GET_DATA = Option<unsafe extern "system" fn(param0: u32, param1: FAX_ENUM_DEVICE_ID_SOURCE, param2: windows_sys::core::PCWSTR, param3: *mut *mut u8, param4: *mut u32) -> u32>;
pub type PFAX_EXT_INITIALIZE_CONFIG = Option<unsafe extern "system" fn(param0: PFAX_EXT_GET_DATA, param1: PFAX_EXT_SET_DATA, param2: PFAX_EXT_REGISTER_FOR_EVENTS, param3: PFAX_EXT_UNREGISTER_FOR_EVENTS, param4: PFAX_EXT_FREE_BUFFER) -> windows_sys::core::HRESULT>;
pub type PFAX_EXT_REGISTER_FOR_EVENTS = Option<unsafe extern "system" fn(param0: super::super::Foundation::HINSTANCE, param1: u32, param2: FAX_ENUM_DEVICE_ID_SOURCE, param3: windows_sys::core::PCWSTR, param4: PFAX_EXT_CONFIG_CHANGE) -> super::super::Foundation::HANDLE>;
pub type PFAX_EXT_SET_DATA = Option<unsafe extern "system" fn(param0: super::super::Foundation::HINSTANCE, param1: u32, param2: FAX_ENUM_DEVICE_ID_SOURCE, param3: windows_sys::core::PCWSTR, param4: *mut u8, param5: u32) -> u32>;
pub type PFAX_EXT_UNREGISTER_FOR_EVENTS = Option<unsafe extern "system" fn(param0: super::super::Foundation::HANDLE) -> u32>;
pub type PFAX_LINECALLBACK = Option<unsafe extern "system" fn(faxhandle: super::super::Foundation::HANDLE, hdevice: u32, dwmessage: u32, dwinstance: usize, dwparam1: usize, dwparam2: usize, dwparam3: usize)>;
pub type PFAX_RECIPIENT_CALLBACKA = Option<unsafe extern "system" fn(faxhandle: super::super::Foundation::HANDLE, recipientnumber: u32, context: *const core::ffi::c_void, jobparams: *mut FAX_JOB_PARAMA, coverpageinfo: *mut FAX_COVERPAGE_INFOA) -> windows_sys::core::BOOL>;
pub type PFAX_RECIPIENT_CALLBACKW = Option<unsafe extern "system" fn(faxhandle: super::super::Foundation::HANDLE, recipientnumber: u32, context: *const core::ffi::c_void, jobparams: *mut FAX_JOB_PARAMW, coverpageinfo: *mut FAX_COVERPAGE_INFOW) -> windows_sys::core::BOOL>;
pub type PFAX_ROUTING_INSTALLATION_CALLBACKW = Option<unsafe extern "system" fn(faxhandle: super::super::Foundation::HANDLE, context: *const core::ffi::c_void, methodname: windows_sys::core::PWSTR, friendlyname: windows_sys::core::PWSTR, functionname: windows_sys::core::PWSTR, guid: windows_sys::core::PWSTR) -> windows_sys::core::BOOL>;
pub type PFAX_SEND_CALLBACK = Option<unsafe extern "system" fn(faxhandle: super::super::Foundation::HANDLE, callhandle: u32, reserved1: u32, reserved2: u32) -> windows_sys::core::BOOL>;
pub type PFAX_SERVICE_CALLBACK = Option<unsafe extern "system" fn(faxhandle: super::super::Foundation::HANDLE, deviceid: u32, param1: usize, param2: usize, param3: usize) -> windows_sys::core::BOOL>;
pub const PORT_OPEN_MODIFY: FAX_ENUM_PORT_OPEN_TYPE = 2i32;
pub const PORT_OPEN_QUERY: FAX_ENUM_PORT_OPEN_TYPE = 1i32;
pub const QUERY_STATUS: FAXROUTE_ENABLE = -1i32;
pub const REGSTR_VAL_BAUDRATE: windows_sys::core::PCWSTR = windows_sys::core::w!("BaudRate");
pub const REGSTR_VAL_BAUDRATE_A: windows_sys::core::PCSTR = windows_sys::core::s!("BaudRate");
pub const REGSTR_VAL_DATA_W: windows_sys::core::PCWSTR = windows_sys::core::w!("DeviceData");
pub const REGSTR_VAL_DEVICESUBTYPE_W: windows_sys::core::PCWSTR = windows_sys::core::w!("DeviceSubType");
pub const REGSTR_VAL_DEVICETYPE_W: windows_sys::core::PCWSTR = windows_sys::core::w!("DeviceType");
pub const REGSTR_VAL_DEVICE_NAME_W: windows_sys::core::PCWSTR = windows_sys::core::w!("DriverDesc");
pub const REGSTR_VAL_DEV_NAME_W: windows_sys::core::PCWSTR = windows_sys::core::w!("DeviceName");
pub const REGSTR_VAL_DRIVER_DESC_W: windows_sys::core::PCWSTR = windows_sys::core::w!("DriverDesc");
pub const REGSTR_VAL_FRIENDLY_NAME_W: windows_sys::core::PCWSTR = windows_sys::core::w!("FriendlyName");
pub const REGSTR_VAL_GENERIC_CAPS_W: windows_sys::core::PCWSTR = windows_sys::core::w!("Capabilities");
pub const REGSTR_VAL_GUID: windows_sys::core::PCWSTR = windows_sys::core::w!("GUID");
pub const REGSTR_VAL_GUID_W: windows_sys::core::PCWSTR = windows_sys::core::w!("GUID");
pub const REGSTR_VAL_HARDWARE: windows_sys::core::PCWSTR = windows_sys::core::w!("HardwareConfig");
pub const REGSTR_VAL_HARDWARE_W: windows_sys::core::PCWSTR = windows_sys::core::w!("HardwareConfig");
pub const REGSTR_VAL_LAUNCHABLE: windows_sys::core::PCWSTR = windows_sys::core::w!("Launchable");
pub const REGSTR_VAL_LAUNCHABLE_W: windows_sys::core::PCWSTR = windows_sys::core::w!("Launchable");
pub const REGSTR_VAL_LAUNCH_APPS: windows_sys::core::PCWSTR = windows_sys::core::w!("LaunchApplications");
pub const REGSTR_VAL_LAUNCH_APPS_W: windows_sys::core::PCWSTR = windows_sys::core::w!("LaunchApplications");
pub const REGSTR_VAL_SHUTDOWNDELAY: windows_sys::core::PCWSTR = windows_sys::core::w!("ShutdownIfUnusedDelay");
pub const REGSTR_VAL_SHUTDOWNDELAY_W: windows_sys::core::PCWSTR = windows_sys::core::w!("ShutdownIfUnusedDelay");
pub const REGSTR_VAL_TYPE_W: windows_sys::core::PCWSTR = windows_sys::core::w!("Type");
pub const REGSTR_VAL_VENDOR_NAME_W: windows_sys::core::PCWSTR = windows_sys::core::w!("Vendor");
pub const SEND_TO_FAX_RECIPIENT_ATTACHMENT: SendToMode = 0i32;
pub const STATUS_DISABLE: FAXROUTE_ENABLE = 0i32;
pub const STATUS_ENABLE: FAXROUTE_ENABLE = 1i32;
pub const STIEDFL_ALLDEVICES: u32 = 0u32;
pub const STIEDFL_ATTACHEDONLY: u32 = 1u32;
pub const STIERR_ALREADY_INITIALIZED: windows_sys::core::HRESULT = 0x800704DF_u32 as _;
pub const STIERR_BADDRIVER: windows_sys::core::HRESULT = 0x80070077_u32 as _;
pub const STIERR_BETA_VERSION: windows_sys::core::HRESULT = 0x80070481_u32 as _;
pub const STIERR_DEVICENOTREG: i32 = -2147221164i32;
pub const STIERR_DEVICE_LOCKED: windows_sys::core::HRESULT = 0x80070021_u32 as _;
pub const STIERR_DEVICE_NOTREADY: windows_sys::core::HRESULT = 0x80070015_u32 as _;
pub const STIERR_GENERIC: i32 = -2147467259i32;
pub const STIERR_HANDLEEXISTS: windows_sys::core::HRESULT = 0x800700B7_u32 as _;
pub const STIERR_INVALID_DEVICE_NAME: windows_sys::core::HRESULT = 0x8007007B_u32 as _;
pub const STIERR_INVALID_HW_TYPE: windows_sys::core::HRESULT = 0x8007000D_u32 as _;
pub const STIERR_INVALID_PARAM: i32 = -2147024809i32;
pub const STIERR_NEEDS_LOCK: windows_sys::core::HRESULT = 0x8007009E_u32 as _;
pub const STIERR_NOEVENTS: windows_sys::core::HRESULT = 0x80070103_u32 as _;
pub const STIERR_NOINTERFACE: i32 = -2147467262i32;
pub const STIERR_NOTINITIALIZED: i32 = -2147024891i32;
pub const STIERR_NOT_INITIALIZED: windows_sys::core::HRESULT = 0x80070015_u32 as _;
pub const STIERR_OBJECTNOTFOUND: windows_sys::core::HRESULT = 0x80070002_u32 as _;
pub const STIERR_OLD_VERSION: windows_sys::core::HRESULT = 0x8007047E_u32 as _;
pub const STIERR_OUTOFMEMORY: i32 = -2147024882i32;
pub const STIERR_READONLY: i32 = -2147024891i32;
pub const STIERR_SHARING_VIOLATION: windows_sys::core::HRESULT = 0x80070020_u32 as _;
pub const STIERR_UNSUPPORTED: i32 = -2147467263i32;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct STINOTIFY {
    pub dwSize: u32,
    pub guidNotificationCode: windows_sys::core::GUID,
    pub abNotificationData: [u8; 64],
}
impl Default for STINOTIFY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct STISUBSCRIBE {
    pub dwSize: u32,
    pub dwFlags: u32,
    pub dwFilter: u32,
    pub hWndNotify: super::super::Foundation::HWND,
    pub hEvent: super::super::Foundation::HANDLE,
    pub uiNotificationMessage: u32,
}
impl Default for STISUBSCRIBE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const STI_ADD_DEVICE_BROADCAST_ACTION: windows_sys::core::PCSTR = windows_sys::core::s!("Arrival");
pub const STI_ADD_DEVICE_BROADCAST_STRING: windows_sys::core::PCSTR = windows_sys::core::s!("STI\\");
pub const STI_CHANGENOEFFECT: i32 = 1i32;
pub const STI_DEVICE_CREATE_BOTH: u32 = 3u32;
pub const STI_DEVICE_CREATE_DATA: u32 = 2u32;
pub const STI_DEVICE_CREATE_FOR_MONITOR: u32 = 16777216u32;
pub const STI_DEVICE_CREATE_MASK: u32 = 65535u32;
pub const STI_DEVICE_CREATE_STATUS: u32 = 1u32;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct STI_DEVICE_INFORMATIONW {
    pub dwSize: u32,
    pub DeviceType: u32,
    pub szDeviceInternalName: [u16; 128],
    pub DeviceCapabilitiesA: STI_DEV_CAPS,
    pub dwHardwareConfiguration: u32,
    pub pszVendorDescription: windows_sys::core::PWSTR,
    pub pszDeviceDescription: windows_sys::core::PWSTR,
    pub pszPortName: windows_sys::core::PWSTR,
    pub pszPropProvider: windows_sys::core::PWSTR,
    pub pszLocalName: windows_sys::core::PWSTR,
}
impl Default for STI_DEVICE_INFORMATIONW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type STI_DEVICE_MJ_TYPE = i32;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct STI_DEVICE_STATUS {
    pub dwSize: u32,
    pub StatusMask: u32,
    pub dwOnlineState: u32,
    pub dwHardwareStatusCode: u32,
    pub dwEventHandlingState: u32,
    pub dwPollingInterval: u32,
}
pub const STI_DEVICE_VALUE_DEFAULT_LAUNCHAPP: windows_sys::core::PCWSTR = windows_sys::core::w!("DefaultLaunchApp");
pub const STI_DEVICE_VALUE_DEFAULT_LAUNCHAPP_A: windows_sys::core::PCSTR = windows_sys::core::s!("DefaultLaunchApp");
pub const STI_DEVICE_VALUE_DISABLE_NOTIFICATIONS: windows_sys::core::PCWSTR = windows_sys::core::w!("DisableNotifications");
pub const STI_DEVICE_VALUE_DISABLE_NOTIFICATIONS_A: windows_sys::core::PCSTR = windows_sys::core::s!("DisableNotifications");
pub const STI_DEVICE_VALUE_ICM_PROFILE: windows_sys::core::PCWSTR = windows_sys::core::w!("ICMProfile");
pub const STI_DEVICE_VALUE_ICM_PROFILE_A: windows_sys::core::PCSTR = windows_sys::core::s!("ICMProfile");
pub const STI_DEVICE_VALUE_ISIS_NAME: windows_sys::core::PCWSTR = windows_sys::core::w!("ISISDriverName");
pub const STI_DEVICE_VALUE_ISIS_NAME_A: windows_sys::core::PCSTR = windows_sys::core::s!("ISISDriverName");
pub const STI_DEVICE_VALUE_TIMEOUT: windows_sys::core::PCWSTR = windows_sys::core::w!("PollTimeout");
pub const STI_DEVICE_VALUE_TIMEOUT_A: windows_sys::core::PCSTR = windows_sys::core::s!("PollTimeout");
pub const STI_DEVICE_VALUE_TWAIN_NAME: windows_sys::core::PCWSTR = windows_sys::core::w!("TwainDS");
pub const STI_DEVICE_VALUE_TWAIN_NAME_A: windows_sys::core::PCSTR = windows_sys::core::s!("TwainDS");
pub const STI_DEVSTATUS_EVENTS_STATE: u32 = 2u32;
pub const STI_DEVSTATUS_ONLINE_STATE: u32 = 1u32;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct STI_DEV_CAPS {
    pub dwGeneric: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
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
pub const STI_REMOVE_DEVICE_BROADCAST_ACTION: windows_sys::core::PCSTR = windows_sys::core::s!("Removal");
pub const STI_REMOVE_DEVICE_BROADCAST_STRING: windows_sys::core::PCSTR = windows_sys::core::s!("STI\\");
pub const STI_SUBSCRIBE_FLAG_EVENT: u32 = 2u32;
pub const STI_SUBSCRIBE_FLAG_WINDOW: u32 = 1u32;
pub const STI_TRACE_ERROR: u32 = 4u32;
pub const STI_TRACE_INFORMATION: u32 = 1u32;
pub const STI_TRACE_WARNING: u32 = 2u32;
pub const STI_UNICODE: u32 = 1u32;
#[repr(C)]
#[derive(Clone, Copy, Default)]
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
#[derive(Clone, Copy)]
pub struct STI_WIA_DEVICE_INFORMATIONW {
    pub dwSize: u32,
    pub DeviceType: u32,
    pub szDeviceInternalName: [u16; 128],
    pub DeviceCapabilitiesA: STI_DEV_CAPS,
    pub dwHardwareConfiguration: u32,
    pub pszVendorDescription: windows_sys::core::PWSTR,
    pub pszDeviceDescription: windows_sys::core::PWSTR,
    pub pszPortName: windows_sys::core::PWSTR,
    pub pszPropProvider: windows_sys::core::PWSTR,
    pub pszLocalName: windows_sys::core::PWSTR,
    pub pszUiDll: windows_sys::core::PWSTR,
    pub pszServer: windows_sys::core::PWSTR,
}
impl Default for STI_WIA_DEVICE_INFORMATIONW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const SUPPORTS_MSCPLUS_STR: windows_sys::core::PCWSTR = windows_sys::core::w!("SupportsMSCPlus");
pub const SUPPORTS_MSCPLUS_VAL: u32 = 1u32;
pub type SendToMode = i32;
pub const StiDeviceTypeDefault: STI_DEVICE_MJ_TYPE = 0i32;
pub const StiDeviceTypeDigitalCamera: STI_DEVICE_MJ_TYPE = 2i32;
pub const StiDeviceTypeScanner: STI_DEVICE_MJ_TYPE = 1i32;
pub const StiDeviceTypeStreamingVideo: STI_DEVICE_MJ_TYPE = 3i32;
pub const WIA_INCOMPAT_XP: u32 = 1u32;
#[repr(C)]
#[derive(Clone, Copy)]
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
pub const faetFXSSVC_ENDED: FAX_ACCOUNT_EVENTS_TYPE_ENUM = 16i32;
pub const faetIN_ARCHIVE: FAX_ACCOUNT_EVENTS_TYPE_ENUM = 4i32;
pub const faetIN_QUEUE: FAX_ACCOUNT_EVENTS_TYPE_ENUM = 1i32;
pub const faetNONE: FAX_ACCOUNT_EVENTS_TYPE_ENUM = 0i32;
pub const faetOUT_ARCHIVE: FAX_ACCOUNT_EVENTS_TYPE_ENUM = 8i32;
pub const faetOUT_QUEUE: FAX_ACCOUNT_EVENTS_TYPE_ENUM = 2i32;
pub const far2MANAGE_ARCHIVES: FAX_ACCESS_RIGHTS_ENUM_2 = 256i32;
pub const far2MANAGE_CONFIG: FAX_ACCESS_RIGHTS_ENUM_2 = 64i32;
pub const far2MANAGE_OUT_JOBS: FAX_ACCESS_RIGHTS_ENUM_2 = 16i32;
pub const far2MANAGE_RECEIVE_FOLDER: FAX_ACCESS_RIGHTS_ENUM_2 = 512i32;
pub const far2QUERY_ARCHIVES: FAX_ACCESS_RIGHTS_ENUM_2 = 128i32;
pub const far2QUERY_CONFIG: FAX_ACCESS_RIGHTS_ENUM_2 = 32i32;
pub const far2QUERY_OUT_JOBS: FAX_ACCESS_RIGHTS_ENUM_2 = 8i32;
pub const far2SUBMIT_HIGH: FAX_ACCESS_RIGHTS_ENUM_2 = 4i32;
pub const far2SUBMIT_LOW: FAX_ACCESS_RIGHTS_ENUM_2 = 1i32;
pub const far2SUBMIT_NORMAL: FAX_ACCESS_RIGHTS_ENUM_2 = 2i32;
pub const farMANAGE_CONFIG: FAX_ACCESS_RIGHTS_ENUM = 64i32;
pub const farMANAGE_IN_ARCHIVE: FAX_ACCESS_RIGHTS_ENUM = 256i32;
pub const farMANAGE_JOBS: FAX_ACCESS_RIGHTS_ENUM = 16i32;
pub const farMANAGE_OUT_ARCHIVE: FAX_ACCESS_RIGHTS_ENUM = 1024i32;
pub const farQUERY_CONFIG: FAX_ACCESS_RIGHTS_ENUM = 32i32;
pub const farQUERY_IN_ARCHIVE: FAX_ACCESS_RIGHTS_ENUM = 128i32;
pub const farQUERY_JOBS: FAX_ACCESS_RIGHTS_ENUM = 8i32;
pub const farQUERY_OUT_ARCHIVE: FAX_ACCESS_RIGHTS_ENUM = 512i32;
pub const farSUBMIT_HIGH: FAX_ACCESS_RIGHTS_ENUM = 4i32;
pub const farSUBMIT_LOW: FAX_ACCESS_RIGHTS_ENUM = 1i32;
pub const farSUBMIT_NORMAL: FAX_ACCESS_RIGHTS_ENUM = 2i32;
pub const fcptLOCAL: FAX_COVERPAGE_TYPE_ENUM = 1i32;
pub const fcptNONE: FAX_COVERPAGE_TYPE_ENUM = 0i32;
pub const fcptSERVER: FAX_COVERPAGE_TYPE_ENUM = 2i32;
pub const fdrmAUTO_ANSWER: FAX_DEVICE_RECEIVE_MODE_ENUM = 1i32;
pub const fdrmMANUAL_ANSWER: FAX_DEVICE_RECEIVE_MODE_ENUM = 2i32;
pub const fdrmNO_ANSWER: FAX_DEVICE_RECEIVE_MODE_ENUM = 0i32;
pub const fgsALL_DEV_NOT_VALID: FAX_GROUP_STATUS_ENUM = 2i32;
pub const fgsALL_DEV_VALID: FAX_GROUP_STATUS_ENUM = 0i32;
pub const fgsEMPTY: FAX_GROUP_STATUS_ENUM = 1i32;
pub const fgsSOME_DEV_NOT_VALID: FAX_GROUP_STATUS_ENUM = 3i32;
pub const fjesANSWERED: FAX_JOB_EXTENDED_STATUS_ENUM = 5i32;
pub const fjesBAD_ADDRESS: FAX_JOB_EXTENDED_STATUS_ENUM = 10i32;
pub const fjesBUSY: FAX_JOB_EXTENDED_STATUS_ENUM = 8i32;
pub const fjesCALL_ABORTED: FAX_JOB_EXTENDED_STATUS_ENUM = 19i32;
pub const fjesCALL_BLACKLISTED: FAX_JOB_EXTENDED_STATUS_ENUM = 14i32;
pub const fjesCALL_COMPLETED: FAX_JOB_EXTENDED_STATUS_ENUM = 18i32;
pub const fjesCALL_DELAYED: FAX_JOB_EXTENDED_STATUS_ENUM = 13i32;
pub const fjesDIALING: FAX_JOB_EXTENDED_STATUS_ENUM = 3i32;
pub const fjesDISCONNECTED: FAX_JOB_EXTENDED_STATUS_ENUM = 1i32;
pub const fjesFATAL_ERROR: FAX_JOB_EXTENDED_STATUS_ENUM = 12i32;
pub const fjesHANDLED: FAX_JOB_EXTENDED_STATUS_ENUM = 17i32;
pub const fjesINITIALIZING: FAX_JOB_EXTENDED_STATUS_ENUM = 2i32;
pub const fjesLINE_UNAVAILABLE: FAX_JOB_EXTENDED_STATUS_ENUM = 7i32;
pub const fjesNONE: FAX_JOB_EXTENDED_STATUS_ENUM = 0i32;
pub const fjesNOT_FAX_CALL: FAX_JOB_EXTENDED_STATUS_ENUM = 15i32;
pub const fjesNO_ANSWER: FAX_JOB_EXTENDED_STATUS_ENUM = 9i32;
pub const fjesNO_DIAL_TONE: FAX_JOB_EXTENDED_STATUS_ENUM = 11i32;
pub const fjesPARTIALLY_RECEIVED: FAX_JOB_EXTENDED_STATUS_ENUM = 16i32;
pub const fjesPROPRIETARY: FAX_JOB_EXTENDED_STATUS_ENUM = 16777216i32;
pub const fjesRECEIVING: FAX_JOB_EXTENDED_STATUS_ENUM = 6i32;
pub const fjesTRANSMITTING: FAX_JOB_EXTENDED_STATUS_ENUM = 4i32;
pub const fjoDELETE: FAX_JOB_OPERATIONS_ENUM = 16i32;
pub const fjoPAUSE: FAX_JOB_OPERATIONS_ENUM = 2i32;
pub const fjoRECIPIENT_INFO: FAX_JOB_OPERATIONS_ENUM = 32i32;
pub const fjoRESTART: FAX_JOB_OPERATIONS_ENUM = 8i32;
pub const fjoRESUME: FAX_JOB_OPERATIONS_ENUM = 4i32;
pub const fjoSENDER_INFO: FAX_JOB_OPERATIONS_ENUM = 64i32;
pub const fjoVIEW: FAX_JOB_OPERATIONS_ENUM = 1i32;
pub const fjsCANCELED: FAX_JOB_STATUS_ENUM = 512i32;
pub const fjsCANCELING: FAX_JOB_STATUS_ENUM = 1024i32;
pub const fjsCOMPLETED: FAX_JOB_STATUS_ENUM = 256i32;
pub const fjsFAILED: FAX_JOB_STATUS_ENUM = 8i32;
pub const fjsINPROGRESS: FAX_JOB_STATUS_ENUM = 2i32;
pub const fjsNOLINE: FAX_JOB_STATUS_ENUM = 32i32;
pub const fjsPAUSED: FAX_JOB_STATUS_ENUM = 16i32;
pub const fjsPENDING: FAX_JOB_STATUS_ENUM = 1i32;
pub const fjsRETRIES_EXCEEDED: FAX_JOB_STATUS_ENUM = 128i32;
pub const fjsRETRYING: FAX_JOB_STATUS_ENUM = 64i32;
pub const fjsROUTING: FAX_JOB_STATUS_ENUM = 2048i32;
pub const fjtRECEIVE: FAX_JOB_TYPE_ENUM = 1i32;
pub const fjtROUTING: FAX_JOB_TYPE_ENUM = 2i32;
pub const fjtSEND: FAX_JOB_TYPE_ENUM = 0i32;
pub const fllMAX: FAX_LOG_LEVEL_ENUM = 3i32;
pub const fllMED: FAX_LOG_LEVEL_ENUM = 2i32;
pub const fllMIN: FAX_LOG_LEVEL_ENUM = 1i32;
pub const fllNONE: FAX_LOG_LEVEL_ENUM = 0i32;
pub const fpsBAD_GUID: FAX_PROVIDER_STATUS_ENUM = 2i32;
pub const fpsBAD_VERSION: FAX_PROVIDER_STATUS_ENUM = 3i32;
pub const fpsCANT_INIT: FAX_PROVIDER_STATUS_ENUM = 6i32;
pub const fpsCANT_LINK: FAX_PROVIDER_STATUS_ENUM = 5i32;
pub const fpsCANT_LOAD: FAX_PROVIDER_STATUS_ENUM = 4i32;
pub const fpsSERVER_ERROR: FAX_PROVIDER_STATUS_ENUM = 1i32;
pub const fpsSUCCESS: FAX_PROVIDER_STATUS_ENUM = 0i32;
pub const fptHIGH: FAX_PRIORITY_TYPE_ENUM = 2i32;
pub const fptLOW: FAX_PRIORITY_TYPE_ENUM = 0i32;
pub const fptNORMAL: FAX_PRIORITY_TYPE_ENUM = 1i32;
pub const frrcANY_CODE: FAX_ROUTING_RULE_CODE_ENUM = 0i32;
pub const frsALL_GROUP_DEV_NOT_VALID: FAX_RULE_STATUS_ENUM = 2i32;
pub const frsBAD_DEVICE: FAX_RULE_STATUS_ENUM = 4i32;
pub const frsEMPTY_GROUP: FAX_RULE_STATUS_ENUM = 1i32;
pub const frsSOME_GROUP_DEV_NOT_VALID: FAX_RULE_STATUS_ENUM = 3i32;
pub const frsVALID: FAX_RULE_STATUS_ENUM = 0i32;
pub const frtMAIL: FAX_RECEIPT_TYPE_ENUM = 1i32;
pub const frtMSGBOX: FAX_RECEIPT_TYPE_ENUM = 4i32;
pub const frtNONE: FAX_RECEIPT_TYPE_ENUM = 0i32;
pub const fsAPI_VERSION_0: FAX_SERVER_APIVERSION_ENUM = 0i32;
pub const fsAPI_VERSION_1: FAX_SERVER_APIVERSION_ENUM = 65536i32;
pub const fsAPI_VERSION_2: FAX_SERVER_APIVERSION_ENUM = 131072i32;
pub const fsAPI_VERSION_3: FAX_SERVER_APIVERSION_ENUM = 196608i32;
pub const fsatANONYMOUS: FAX_SMTP_AUTHENTICATION_TYPE_ENUM = 0i32;
pub const fsatBASIC: FAX_SMTP_AUTHENTICATION_TYPE_ENUM = 1i32;
pub const fsatNTLM: FAX_SMTP_AUTHENTICATION_TYPE_ENUM = 2i32;
pub const fsetACTIVITY: FAX_SERVER_EVENTS_TYPE_ENUM = 8i32;
pub const fsetCONFIG: FAX_SERVER_EVENTS_TYPE_ENUM = 4i32;
pub const fsetDEVICE_STATUS: FAX_SERVER_EVENTS_TYPE_ENUM = 256i32;
pub const fsetFXSSVC_ENDED: FAX_SERVER_EVENTS_TYPE_ENUM = 128i32;
pub const fsetINCOMING_CALL: FAX_SERVER_EVENTS_TYPE_ENUM = 512i32;
pub const fsetIN_ARCHIVE: FAX_SERVER_EVENTS_TYPE_ENUM = 32i32;
pub const fsetIN_QUEUE: FAX_SERVER_EVENTS_TYPE_ENUM = 1i32;
pub const fsetNONE: FAX_SERVER_EVENTS_TYPE_ENUM = 0i32;
pub const fsetOUT_ARCHIVE: FAX_SERVER_EVENTS_TYPE_ENUM = 64i32;
pub const fsetOUT_QUEUE: FAX_SERVER_EVENTS_TYPE_ENUM = 2i32;
pub const fsetQUEUE_STATE: FAX_SERVER_EVENTS_TYPE_ENUM = 16i32;
pub const fstDISCOUNT_PERIOD: FAX_SCHEDULE_TYPE_ENUM = 2i32;
pub const fstNOW: FAX_SCHEDULE_TYPE_ENUM = 0i32;
pub const fstSPECIFIC_TIME: FAX_SCHEDULE_TYPE_ENUM = 1i32;
pub const lDEFAULT_PREFETCH_SIZE: i32 = 100i32;
pub const prv_DEFAULT_PREFETCH_SIZE: u32 = 100u32;
pub const wcharREASSIGN_RECIPIENTS_DELIMITER: u16 = 59u16;
