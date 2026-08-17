windows_link::link!("mdmlocalmanagement.dll" "system" fn ApplyLocalManagementSyncML(syncmlrequest : windows_sys::core::PCWSTR, syncmlresult : *mut windows_sys::core::PWSTR) -> windows_sys::core::HRESULT);
windows_link::link!("mdmregistration.dll" "system" fn DiscoverManagementService(pszupn : windows_sys::core::PCWSTR, ppmgmtinfo : *mut *mut MANAGEMENT_SERVICE_INFO) -> windows_sys::core::HRESULT);
windows_link::link!("mdmregistration.dll" "system" fn DiscoverManagementServiceEx(pszupn : windows_sys::core::PCWSTR, pszdiscoveryservicecandidate : windows_sys::core::PCWSTR, ppmgmtinfo : *mut *mut MANAGEMENT_SERVICE_INFO) -> windows_sys::core::HRESULT);
windows_link::link!("mdmregistration.dll" "system" fn GetDeviceManagementConfigInfo(providerid : windows_sys::core::PCWSTR, configstringbufferlength : *mut u32, configstring : windows_sys::core::PWSTR) -> windows_sys::core::HRESULT);
windows_link::link!("mdmregistration.dll" "system" fn GetDeviceRegistrationInfo(deviceinformationclass : REGISTRATION_INFORMATION_CLASS, ppdeviceregistrationinfo : *mut *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
windows_link::link!("mdmregistration.dll" "system" fn GetManagementAppHyperlink(cchhyperlink : u32, pszhyperlink : windows_sys::core::PWSTR) -> windows_sys::core::HRESULT);
windows_link::link!("mdmregistration.dll" "system" fn IsDeviceRegisteredWithManagement(pfisdeviceregisteredwithmanagement : *mut windows_sys::core::BOOL, cchupn : u32, pszupn : windows_sys::core::PWSTR) -> windows_sys::core::HRESULT);
windows_link::link!("mdmregistration.dll" "system" fn IsManagementRegistrationAllowed(pfismanagementregistrationallowed : *mut windows_sys::core::BOOL) -> windows_sys::core::HRESULT);
windows_link::link!("mdmregistration.dll" "system" fn IsMdmUxWithoutAadAllowed(isenrollmentallowed : *mut windows_sys::core::BOOL) -> windows_sys::core::HRESULT);
windows_link::link!("mdmlocalmanagement.dll" "system" fn RegisterDeviceWithLocalManagement(alreadyregistered : *mut windows_sys::core::BOOL) -> windows_sys::core::HRESULT);
windows_link::link!("mdmregistration.dll" "system" fn RegisterDeviceWithManagement(pszupn : windows_sys::core::PCWSTR, ppszmdmserviceuri : windows_sys::core::PCWSTR, ppzsaccesstoken : windows_sys::core::PCWSTR) -> windows_sys::core::HRESULT);
windows_link::link!("mdmregistration.dll" "system" fn RegisterDeviceWithManagementUsingAADCredentials(usertoken : super::super::Foundation:: HANDLE) -> windows_sys::core::HRESULT);
windows_link::link!("mdmregistration.dll" "system" fn RegisterDeviceWithManagementUsingAADDeviceCredentials() -> windows_sys::core::HRESULT);
windows_link::link!("mdmregistration.dll" "system" fn RegisterDeviceWithManagementUsingAADDeviceCredentials2(mdmapplicationid : windows_sys::core::PCWSTR) -> windows_sys::core::HRESULT);
windows_link::link!("mdmregistration.dll" "system" fn SetDeviceManagementConfigInfo(providerid : windows_sys::core::PCWSTR, configstring : windows_sys::core::PCWSTR) -> windows_sys::core::HRESULT);
windows_link::link!("mdmregistration.dll" "system" fn SetManagedExternally(ismanagedexternally : windows_sys::core::BOOL) -> windows_sys::core::HRESULT);
windows_link::link!("mdmlocalmanagement.dll" "system" fn UnregisterDeviceWithLocalManagement() -> windows_sys::core::HRESULT);
windows_link::link!("mdmregistration.dll" "system" fn UnregisterDeviceWithManagement(enrollmentid : windows_sys::core::PCWSTR) -> windows_sys::core::HRESULT);
pub const DEVICEREGISTRATIONTYPE_MAM: u32 = 5u32;
pub const DEVICEREGISTRATIONTYPE_MDM_DEVICEWIDE_WITH_AAD: u32 = 6u32;
pub const DEVICEREGISTRATIONTYPE_MDM_ONLY: u32 = 0u32;
pub const DEVICEREGISTRATIONTYPE_MDM_USERSPECIFIC_WITH_AAD: u32 = 13u32;
pub const DEVICE_ENROLLER_FACILITY_CODE: u32 = 24u32;
pub const DeviceRegistrationBasicInfo: REGISTRATION_INFORMATION_CLASS = 1i32;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct MANAGEMENT_REGISTRATION_INFO {
    pub fDeviceRegisteredWithManagement: windows_sys::core::BOOL,
    pub dwDeviceRegistionKind: u32,
    pub pszUPN: windows_sys::core::PWSTR,
    pub pszMDMServiceUri: windows_sys::core::PWSTR,
}
impl Default for MANAGEMENT_REGISTRATION_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct MANAGEMENT_SERVICE_INFO {
    pub pszMDMServiceUri: windows_sys::core::PWSTR,
    pub pszAuthenticationUri: windows_sys::core::PWSTR,
}
impl Default for MANAGEMENT_SERVICE_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const MDM_REGISTRATION_FACILITY_CODE: u32 = 25u32;
pub const MENROLL_E_CERTAUTH_FAILED_TO_FIND_CERT: windows_sys::core::HRESULT = 0x80180028_u32 as _;
pub const MENROLL_E_CERTPOLICY_PRIVATEKEYCREATION_FAILED: windows_sys::core::HRESULT = 0x80180027_u32 as _;
pub const MENROLL_E_CONNECTIVITY: windows_sys::core::HRESULT = 0x80180010_u32 as _;
pub const MENROLL_E_CUSTOMSERVERERROR: windows_sys::core::HRESULT = 0x80180032_u32 as _;
pub const MENROLL_E_DEVICECAPREACHED: windows_sys::core::HRESULT = 0x80180013_u32 as _;
pub const MENROLL_E_DEVICENOTSUPPORTED: windows_sys::core::HRESULT = 0x80180014_u32 as _;
pub const MENROLL_E_DEVICE_ALREADY_ENROLLED: windows_sys::core::HRESULT = 0x8018000A_u32 as _;
pub const MENROLL_E_DEVICE_AUTHENTICATION_ERROR: windows_sys::core::HRESULT = 0x80180002_u32 as _;
pub const MENROLL_E_DEVICE_AUTHORIZATION_ERROR: windows_sys::core::HRESULT = 0x80180003_u32 as _;
pub const MENROLL_E_DEVICE_CERTIFCATEREQUEST_ERROR: windows_sys::core::HRESULT = 0x80180004_u32 as _;
pub const MENROLL_E_DEVICE_CERTIFICATEREQUEST_ERROR: windows_sys::core::HRESULT = 0x80180004_u32 as _;
pub const MENROLL_E_DEVICE_CONFIGMGRSERVER_ERROR: windows_sys::core::HRESULT = 0x80180005_u32 as _;
pub const MENROLL_E_DEVICE_INTERNALSERVICE_ERROR: windows_sys::core::HRESULT = 0x80180006_u32 as _;
pub const MENROLL_E_DEVICE_INVALIDSECURITY_ERROR: windows_sys::core::HRESULT = 0x80180007_u32 as _;
pub const MENROLL_E_DEVICE_MANAGEMENT_BLOCKED: windows_sys::core::HRESULT = 0x80180026_u32 as _;
pub const MENROLL_E_DEVICE_MESSAGE_FORMAT_ERROR: windows_sys::core::HRESULT = 0x80180001_u32 as _;
pub const MENROLL_E_DEVICE_NOT_ENROLLED: windows_sys::core::HRESULT = 0x8018000B_u32 as _;
pub const MENROLL_E_DEVICE_UNKNOWN_ERROR: windows_sys::core::HRESULT = 0x80180008_u32 as _;
pub const MENROLL_E_DISCOVERY_SEC_CERT_DATE_INVALID: windows_sys::core::HRESULT = 0x8018000D_u32 as _;
pub const MENROLL_E_EMPTY_MESSAGE: windows_sys::core::HRESULT = 0x80180029_u32 as _;
pub const MENROLL_E_ENROLLMENTDATAINVALID: windows_sys::core::HRESULT = 0x80180019_u32 as _;
pub const MENROLL_E_ENROLLMENT_IN_PROGRESS: windows_sys::core::HRESULT = 0x80180009_u32 as _;
pub const MENROLL_E_INMAINTENANCE: windows_sys::core::HRESULT = 0x80180017_u32 as _;
pub const MENROLL_E_INSECUREREDIRECT: windows_sys::core::HRESULT = 0x8018001A_u32 as _;
pub const MENROLL_E_INVALIDSSLCERT: windows_sys::core::HRESULT = 0x80180012_u32 as _;
pub const MENROLL_E_MDM_NOT_CONFIGURED: windows_sys::core::HRESULT = 0x80180031_u32 as _;
pub const MENROLL_E_NOTELIGIBLETORENEW: windows_sys::core::HRESULT = 0x80180016_u32 as _;
pub const MENROLL_E_NOTSUPPORTED: windows_sys::core::HRESULT = 0x80180015_u32 as _;
pub const MENROLL_E_NOT_SUPPORTED: windows_sys::core::HRESULT = 0x80180015_u32 as _;
pub const MENROLL_E_PASSWORD_NEEDED: windows_sys::core::HRESULT = 0x8018000E_u32 as _;
pub const MENROLL_E_PLATFORM_LICENSE_ERROR: windows_sys::core::HRESULT = 0x8018001C_u32 as _;
pub const MENROLL_E_PLATFORM_UNKNOWN_ERROR: windows_sys::core::HRESULT = 0x8018001D_u32 as _;
pub const MENROLL_E_PLATFORM_WRONG_STATE: windows_sys::core::HRESULT = 0x8018001B_u32 as _;
pub const MENROLL_E_PROV_CSP_APPMGMT: windows_sys::core::HRESULT = 0x80180025_u32 as _;
pub const MENROLL_E_PROV_CSP_CERTSTORE: windows_sys::core::HRESULT = 0x8018001E_u32 as _;
pub const MENROLL_E_PROV_CSP_DMCLIENT: windows_sys::core::HRESULT = 0x80180020_u32 as _;
pub const MENROLL_E_PROV_CSP_MISC: windows_sys::core::HRESULT = 0x80180022_u32 as _;
pub const MENROLL_E_PROV_CSP_PFW: windows_sys::core::HRESULT = 0x80180021_u32 as _;
pub const MENROLL_E_PROV_CSP_W7: windows_sys::core::HRESULT = 0x8018001F_u32 as _;
pub const MENROLL_E_PROV_SSLCERTNOTFOUND: windows_sys::core::HRESULT = 0x80180024_u32 as _;
pub const MENROLL_E_PROV_UNKNOWN: windows_sys::core::HRESULT = 0x80180023_u32 as _;
pub const MENROLL_E_USERLICENSE: windows_sys::core::HRESULT = 0x80180018_u32 as _;
pub const MENROLL_E_USER_CANCELED: windows_sys::core::HRESULT = 0x8018002A_u32 as _;
pub const MENROLL_E_USER_CANCELLED: windows_sys::core::HRESULT = 0x80180030_u32 as _;
pub const MENROLL_E_USER_LICENSE: windows_sys::core::HRESULT = 0x80180018_u32 as _;
pub const MENROLL_E_WAB_ERROR: windows_sys::core::HRESULT = 0x8018000F_u32 as _;
pub const MREGISTER_E_DEVICE_ALREADY_REGISTERED: windows_sys::core::HRESULT = 0x8019000A_u32 as _;
pub const MREGISTER_E_DEVICE_AUTHENTICATION_ERROR: windows_sys::core::HRESULT = 0x80190002_u32 as _;
pub const MREGISTER_E_DEVICE_AUTHORIZATION_ERROR: windows_sys::core::HRESULT = 0x80190003_u32 as _;
pub const MREGISTER_E_DEVICE_CERTIFCATEREQUEST_ERROR: windows_sys::core::HRESULT = 0x80190004_u32 as _;
pub const MREGISTER_E_DEVICE_CONFIGMGRSERVER_ERROR: windows_sys::core::HRESULT = 0x80190005_u32 as _;
pub const MREGISTER_E_DEVICE_INTERNALSERVICE_ERROR: windows_sys::core::HRESULT = 0x80190006_u32 as _;
pub const MREGISTER_E_DEVICE_INVALIDSECURITY_ERROR: windows_sys::core::HRESULT = 0x80190007_u32 as _;
pub const MREGISTER_E_DEVICE_MESSAGE_FORMAT_ERROR: windows_sys::core::HRESULT = 0x80190001_u32 as _;
pub const MREGISTER_E_DEVICE_NOT_AD_REGISTERED_ERROR: windows_sys::core::HRESULT = 0x8019000D_u32 as _;
pub const MREGISTER_E_DEVICE_NOT_REGISTERED: windows_sys::core::HRESULT = 0x8019000B_u32 as _;
pub const MREGISTER_E_DEVICE_UNKNOWN_ERROR: windows_sys::core::HRESULT = 0x80190008_u32 as _;
pub const MREGISTER_E_DISCOVERY_FAILED: windows_sys::core::HRESULT = 0x8019000E_u32 as _;
pub const MREGISTER_E_DISCOVERY_REDIRECTED: windows_sys::core::HRESULT = 0x8019000C_u32 as _;
pub const MREGISTER_E_REGISTRATION_IN_PROGRESS: windows_sys::core::HRESULT = 0x80190009_u32 as _;
pub const MaxDeviceInfoClass: REGISTRATION_INFORMATION_CLASS = 2i32;
pub type REGISTRATION_INFORMATION_CLASS = i32;
