#[inline]
pub unsafe fn ApplyLocalManagementSyncML<P0>(syncmlrequest: P0, syncmlresult: Option<*mut windows_core::PWSTR>) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("mdmlocalmanagement.dll" "system" fn ApplyLocalManagementSyncML(syncmlrequest : windows_core::PCWSTR, syncmlresult : *mut windows_core::PWSTR) -> windows_core::HRESULT);
    ApplyLocalManagementSyncML(syncmlrequest.param().abi(), core::mem::transmute(syncmlresult.unwrap_or(std::ptr::null_mut()))).ok()
}
#[inline]
pub unsafe fn DiscoverManagementService<P0>(pszupn: P0) -> windows_core::Result<*mut MANAGEMENT_SERVICE_INFO>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("mdmregistration.dll" "system" fn DiscoverManagementService(pszupn : windows_core::PCWSTR, ppmgmtinfo : *mut *mut MANAGEMENT_SERVICE_INFO) -> windows_core::HRESULT);
    let mut result__ = core::mem::zeroed();
    DiscoverManagementService(pszupn.param().abi(), &mut result__).map(|| result__)
}
#[inline]
pub unsafe fn DiscoverManagementServiceEx<P0, P1>(pszupn: P0, pszdiscoveryservicecandidate: P1) -> windows_core::Result<*mut MANAGEMENT_SERVICE_INFO>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("mdmregistration.dll" "system" fn DiscoverManagementServiceEx(pszupn : windows_core::PCWSTR, pszdiscoveryservicecandidate : windows_core::PCWSTR, ppmgmtinfo : *mut *mut MANAGEMENT_SERVICE_INFO) -> windows_core::HRESULT);
    let mut result__ = core::mem::zeroed();
    DiscoverManagementServiceEx(pszupn.param().abi(), pszdiscoveryservicecandidate.param().abi(), &mut result__).map(|| result__)
}
#[inline]
pub unsafe fn GetDeviceManagementConfigInfo<P0>(providerid: P0, configstringbufferlength: *mut u32, configstring: windows_core::PWSTR) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("mdmregistration.dll" "system" fn GetDeviceManagementConfigInfo(providerid : windows_core::PCWSTR, configstringbufferlength : *mut u32, configstring : windows_core::PWSTR) -> windows_core::HRESULT);
    GetDeviceManagementConfigInfo(providerid.param().abi(), configstringbufferlength, core::mem::transmute(configstring)).ok()
}
#[inline]
pub unsafe fn GetDeviceRegistrationInfo(deviceinformationclass: REGISTRATION_INFORMATION_CLASS, ppdeviceregistrationinfo: *mut *mut core::ffi::c_void) -> windows_core::Result<()> {
    windows_targets::link!("mdmregistration.dll" "system" fn GetDeviceRegistrationInfo(deviceinformationclass : REGISTRATION_INFORMATION_CLASS, ppdeviceregistrationinfo : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    GetDeviceRegistrationInfo(deviceinformationclass, ppdeviceregistrationinfo).ok()
}
#[inline]
pub unsafe fn GetManagementAppHyperlink(pszhyperlink: &mut [u16]) -> windows_core::Result<()> {
    windows_targets::link!("mdmregistration.dll" "system" fn GetManagementAppHyperlink(cchhyperlink : u32, pszhyperlink : windows_core::PWSTR) -> windows_core::HRESULT);
    GetManagementAppHyperlink(pszhyperlink.len().try_into().unwrap(), core::mem::transmute(pszhyperlink.as_ptr())).ok()
}
#[inline]
pub unsafe fn IsDeviceRegisteredWithManagement(pfisdeviceregisteredwithmanagement: *mut super::super::Foundation::BOOL, pszupn: Option<&mut [u16]>) -> windows_core::Result<()> {
    windows_targets::link!("mdmregistration.dll" "system" fn IsDeviceRegisteredWithManagement(pfisdeviceregisteredwithmanagement : *mut super::super::Foundation:: BOOL, cchupn : u32, pszupn : windows_core::PWSTR) -> windows_core::HRESULT);
    IsDeviceRegisteredWithManagement(pfisdeviceregisteredwithmanagement, pszupn.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), core::mem::transmute(pszupn.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr()))).ok()
}
#[inline]
pub unsafe fn IsManagementRegistrationAllowed() -> windows_core::Result<super::super::Foundation::BOOL> {
    windows_targets::link!("mdmregistration.dll" "system" fn IsManagementRegistrationAllowed(pfismanagementregistrationallowed : *mut super::super::Foundation:: BOOL) -> windows_core::HRESULT);
    let mut result__ = core::mem::zeroed();
    IsManagementRegistrationAllowed(&mut result__).map(|| result__)
}
#[inline]
pub unsafe fn IsMdmUxWithoutAadAllowed() -> windows_core::Result<super::super::Foundation::BOOL> {
    windows_targets::link!("mdmregistration.dll" "system" fn IsMdmUxWithoutAadAllowed(isenrollmentallowed : *mut super::super::Foundation:: BOOL) -> windows_core::HRESULT);
    let mut result__ = core::mem::zeroed();
    IsMdmUxWithoutAadAllowed(&mut result__).map(|| result__)
}
#[inline]
pub unsafe fn RegisterDeviceWithLocalManagement(alreadyregistered: Option<*mut super::super::Foundation::BOOL>) -> windows_core::Result<()> {
    windows_targets::link!("mdmlocalmanagement.dll" "system" fn RegisterDeviceWithLocalManagement(alreadyregistered : *mut super::super::Foundation:: BOOL) -> windows_core::HRESULT);
    RegisterDeviceWithLocalManagement(core::mem::transmute(alreadyregistered.unwrap_or(std::ptr::null_mut()))).ok()
}
#[inline]
pub unsafe fn RegisterDeviceWithManagement<P0, P1, P2>(pszupn: P0, ppszmdmserviceuri: P1, ppzsaccesstoken: P2) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("mdmregistration.dll" "system" fn RegisterDeviceWithManagement(pszupn : windows_core::PCWSTR, ppszmdmserviceuri : windows_core::PCWSTR, ppzsaccesstoken : windows_core::PCWSTR) -> windows_core::HRESULT);
    RegisterDeviceWithManagement(pszupn.param().abi(), ppszmdmserviceuri.param().abi(), ppzsaccesstoken.param().abi()).ok()
}
#[inline]
pub unsafe fn RegisterDeviceWithManagementUsingAADCredentials<P0>(usertoken: P0) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::super::Foundation::HANDLE>,
{
    windows_targets::link!("mdmregistration.dll" "system" fn RegisterDeviceWithManagementUsingAADCredentials(usertoken : super::super::Foundation:: HANDLE) -> windows_core::HRESULT);
    RegisterDeviceWithManagementUsingAADCredentials(usertoken.param().abi()).ok()
}
#[inline]
pub unsafe fn RegisterDeviceWithManagementUsingAADDeviceCredentials() -> windows_core::Result<()> {
    windows_targets::link!("mdmregistration.dll" "system" fn RegisterDeviceWithManagementUsingAADDeviceCredentials() -> windows_core::HRESULT);
    RegisterDeviceWithManagementUsingAADDeviceCredentials().ok()
}
#[inline]
pub unsafe fn RegisterDeviceWithManagementUsingAADDeviceCredentials2<P0>(mdmapplicationid: P0) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("mdmregistration.dll" "system" fn RegisterDeviceWithManagementUsingAADDeviceCredentials2(mdmapplicationid : windows_core::PCWSTR) -> windows_core::HRESULT);
    RegisterDeviceWithManagementUsingAADDeviceCredentials2(mdmapplicationid.param().abi()).ok()
}
#[inline]
pub unsafe fn SetDeviceManagementConfigInfo<P0, P1>(providerid: P0, configstring: P1) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("mdmregistration.dll" "system" fn SetDeviceManagementConfigInfo(providerid : windows_core::PCWSTR, configstring : windows_core::PCWSTR) -> windows_core::HRESULT);
    SetDeviceManagementConfigInfo(providerid.param().abi(), configstring.param().abi()).ok()
}
#[inline]
pub unsafe fn SetManagedExternally<P0>(ismanagedexternally: P0) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::super::Foundation::BOOL>,
{
    windows_targets::link!("mdmregistration.dll" "system" fn SetManagedExternally(ismanagedexternally : super::super::Foundation:: BOOL) -> windows_core::HRESULT);
    SetManagedExternally(ismanagedexternally.param().abi()).ok()
}
#[inline]
pub unsafe fn UnregisterDeviceWithLocalManagement() -> windows_core::Result<()> {
    windows_targets::link!("mdmlocalmanagement.dll" "system" fn UnregisterDeviceWithLocalManagement() -> windows_core::HRESULT);
    UnregisterDeviceWithLocalManagement().ok()
}
#[inline]
pub unsafe fn UnregisterDeviceWithManagement<P0>(enrollmentid: P0) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("mdmregistration.dll" "system" fn UnregisterDeviceWithManagement(enrollmentid : windows_core::PCWSTR) -> windows_core::HRESULT);
    UnregisterDeviceWithManagement(enrollmentid.param().abi()).ok()
}
pub const DEVICEREGISTRATIONTYPE_MAM: u32 = 5u32;
pub const DEVICEREGISTRATIONTYPE_MDM_DEVICEWIDE_WITH_AAD: u32 = 6u32;
pub const DEVICEREGISTRATIONTYPE_MDM_ONLY: u32 = 0u32;
pub const DEVICEREGISTRATIONTYPE_MDM_USERSPECIFIC_WITH_AAD: u32 = 13u32;
pub const DEVICE_ENROLLER_FACILITY_CODE: u32 = 24u32;
pub const DeviceRegistrationBasicInfo: REGISTRATION_INFORMATION_CLASS = REGISTRATION_INFORMATION_CLASS(1i32);
pub const MDM_REGISTRATION_FACILITY_CODE: u32 = 25u32;
pub const MENROLL_E_CERTAUTH_FAILED_TO_FIND_CERT: windows_core::HRESULT = windows_core::HRESULT(0x80180028_u32 as _);
pub const MENROLL_E_CERTPOLICY_PRIVATEKEYCREATION_FAILED: windows_core::HRESULT = windows_core::HRESULT(0x80180027_u32 as _);
pub const MENROLL_E_CONNECTIVITY: windows_core::HRESULT = windows_core::HRESULT(0x80180010_u32 as _);
pub const MENROLL_E_CUSTOMSERVERERROR: windows_core::HRESULT = windows_core::HRESULT(0x80180032_u32 as _);
pub const MENROLL_E_DEVICECAPREACHED: windows_core::HRESULT = windows_core::HRESULT(0x80180013_u32 as _);
pub const MENROLL_E_DEVICENOTSUPPORTED: windows_core::HRESULT = windows_core::HRESULT(0x80180014_u32 as _);
pub const MENROLL_E_DEVICE_ALREADY_ENROLLED: windows_core::HRESULT = windows_core::HRESULT(0x8018000A_u32 as _);
pub const MENROLL_E_DEVICE_AUTHENTICATION_ERROR: windows_core::HRESULT = windows_core::HRESULT(0x80180002_u32 as _);
pub const MENROLL_E_DEVICE_AUTHORIZATION_ERROR: windows_core::HRESULT = windows_core::HRESULT(0x80180003_u32 as _);
pub const MENROLL_E_DEVICE_CERTIFCATEREQUEST_ERROR: windows_core::HRESULT = windows_core::HRESULT(0x80180004_u32 as _);
pub const MENROLL_E_DEVICE_CERTIFICATEREQUEST_ERROR: windows_core::HRESULT = windows_core::HRESULT(0x80180004_u32 as _);
pub const MENROLL_E_DEVICE_CONFIGMGRSERVER_ERROR: windows_core::HRESULT = windows_core::HRESULT(0x80180005_u32 as _);
pub const MENROLL_E_DEVICE_INTERNALSERVICE_ERROR: windows_core::HRESULT = windows_core::HRESULT(0x80180006_u32 as _);
pub const MENROLL_E_DEVICE_INVALIDSECURITY_ERROR: windows_core::HRESULT = windows_core::HRESULT(0x80180007_u32 as _);
pub const MENROLL_E_DEVICE_MANAGEMENT_BLOCKED: windows_core::HRESULT = windows_core::HRESULT(0x80180026_u32 as _);
pub const MENROLL_E_DEVICE_MESSAGE_FORMAT_ERROR: windows_core::HRESULT = windows_core::HRESULT(0x80180001_u32 as _);
pub const MENROLL_E_DEVICE_NOT_ENROLLED: windows_core::HRESULT = windows_core::HRESULT(0x8018000B_u32 as _);
pub const MENROLL_E_DEVICE_UNKNOWN_ERROR: windows_core::HRESULT = windows_core::HRESULT(0x80180008_u32 as _);
pub const MENROLL_E_DISCOVERY_SEC_CERT_DATE_INVALID: windows_core::HRESULT = windows_core::HRESULT(0x8018000D_u32 as _);
pub const MENROLL_E_EMPTY_MESSAGE: windows_core::HRESULT = windows_core::HRESULT(0x80180029_u32 as _);
pub const MENROLL_E_ENROLLMENTDATAINVALID: windows_core::HRESULT = windows_core::HRESULT(0x80180019_u32 as _);
pub const MENROLL_E_ENROLLMENT_IN_PROGRESS: windows_core::HRESULT = windows_core::HRESULT(0x80180009_u32 as _);
pub const MENROLL_E_INMAINTENANCE: windows_core::HRESULT = windows_core::HRESULT(0x80180017_u32 as _);
pub const MENROLL_E_INSECUREREDIRECT: windows_core::HRESULT = windows_core::HRESULT(0x8018001A_u32 as _);
pub const MENROLL_E_INVALIDSSLCERT: windows_core::HRESULT = windows_core::HRESULT(0x80180012_u32 as _);
pub const MENROLL_E_MDM_NOT_CONFIGURED: windows_core::HRESULT = windows_core::HRESULT(0x80180031_u32 as _);
pub const MENROLL_E_NOTELIGIBLETORENEW: windows_core::HRESULT = windows_core::HRESULT(0x80180016_u32 as _);
pub const MENROLL_E_NOTSUPPORTED: windows_core::HRESULT = windows_core::HRESULT(0x80180015_u32 as _);
pub const MENROLL_E_NOT_SUPPORTED: windows_core::HRESULT = windows_core::HRESULT(0x80180015_u32 as _);
pub const MENROLL_E_PASSWORD_NEEDED: windows_core::HRESULT = windows_core::HRESULT(0x8018000E_u32 as _);
pub const MENROLL_E_PLATFORM_LICENSE_ERROR: windows_core::HRESULT = windows_core::HRESULT(0x8018001C_u32 as _);
pub const MENROLL_E_PLATFORM_UNKNOWN_ERROR: windows_core::HRESULT = windows_core::HRESULT(0x8018001D_u32 as _);
pub const MENROLL_E_PLATFORM_WRONG_STATE: windows_core::HRESULT = windows_core::HRESULT(0x8018001B_u32 as _);
pub const MENROLL_E_PROV_CSP_APPMGMT: windows_core::HRESULT = windows_core::HRESULT(0x80180025_u32 as _);
pub const MENROLL_E_PROV_CSP_CERTSTORE: windows_core::HRESULT = windows_core::HRESULT(0x8018001E_u32 as _);
pub const MENROLL_E_PROV_CSP_DMCLIENT: windows_core::HRESULT = windows_core::HRESULT(0x80180020_u32 as _);
pub const MENROLL_E_PROV_CSP_MISC: windows_core::HRESULT = windows_core::HRESULT(0x80180022_u32 as _);
pub const MENROLL_E_PROV_CSP_PFW: windows_core::HRESULT = windows_core::HRESULT(0x80180021_u32 as _);
pub const MENROLL_E_PROV_CSP_W7: windows_core::HRESULT = windows_core::HRESULT(0x8018001F_u32 as _);
pub const MENROLL_E_PROV_SSLCERTNOTFOUND: windows_core::HRESULT = windows_core::HRESULT(0x80180024_u32 as _);
pub const MENROLL_E_PROV_UNKNOWN: windows_core::HRESULT = windows_core::HRESULT(0x80180023_u32 as _);
pub const MENROLL_E_USERLICENSE: windows_core::HRESULT = windows_core::HRESULT(0x80180018_u32 as _);
pub const MENROLL_E_USER_CANCELED: windows_core::HRESULT = windows_core::HRESULT(0x8018002A_u32 as _);
pub const MENROLL_E_USER_CANCELLED: windows_core::HRESULT = windows_core::HRESULT(0x80180030_u32 as _);
pub const MENROLL_E_USER_LICENSE: windows_core::HRESULT = windows_core::HRESULT(0x80180018_u32 as _);
pub const MENROLL_E_WAB_ERROR: windows_core::HRESULT = windows_core::HRESULT(0x8018000F_u32 as _);
pub const MREGISTER_E_DEVICE_ALREADY_REGISTERED: windows_core::HRESULT = windows_core::HRESULT(0x8019000A_u32 as _);
pub const MREGISTER_E_DEVICE_AUTHENTICATION_ERROR: windows_core::HRESULT = windows_core::HRESULT(0x80190002_u32 as _);
pub const MREGISTER_E_DEVICE_AUTHORIZATION_ERROR: windows_core::HRESULT = windows_core::HRESULT(0x80190003_u32 as _);
pub const MREGISTER_E_DEVICE_CERTIFCATEREQUEST_ERROR: windows_core::HRESULT = windows_core::HRESULT(0x80190004_u32 as _);
pub const MREGISTER_E_DEVICE_CONFIGMGRSERVER_ERROR: windows_core::HRESULT = windows_core::HRESULT(0x80190005_u32 as _);
pub const MREGISTER_E_DEVICE_INTERNALSERVICE_ERROR: windows_core::HRESULT = windows_core::HRESULT(0x80190006_u32 as _);
pub const MREGISTER_E_DEVICE_INVALIDSECURITY_ERROR: windows_core::HRESULT = windows_core::HRESULT(0x80190007_u32 as _);
pub const MREGISTER_E_DEVICE_MESSAGE_FORMAT_ERROR: windows_core::HRESULT = windows_core::HRESULT(0x80190001_u32 as _);
pub const MREGISTER_E_DEVICE_NOT_AD_REGISTERED_ERROR: windows_core::HRESULT = windows_core::HRESULT(0x8019000D_u32 as _);
pub const MREGISTER_E_DEVICE_NOT_REGISTERED: windows_core::HRESULT = windows_core::HRESULT(0x8019000B_u32 as _);
pub const MREGISTER_E_DEVICE_UNKNOWN_ERROR: windows_core::HRESULT = windows_core::HRESULT(0x80190008_u32 as _);
pub const MREGISTER_E_DISCOVERY_FAILED: windows_core::HRESULT = windows_core::HRESULT(0x8019000E_u32 as _);
pub const MREGISTER_E_DISCOVERY_REDIRECTED: windows_core::HRESULT = windows_core::HRESULT(0x8019000C_u32 as _);
pub const MREGISTER_E_REGISTRATION_IN_PROGRESS: windows_core::HRESULT = windows_core::HRESULT(0x80190009_u32 as _);
pub const MaxDeviceInfoClass: REGISTRATION_INFORMATION_CLASS = REGISTRATION_INFORMATION_CLASS(2i32);
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct REGISTRATION_INFORMATION_CLASS(pub i32);
impl windows_core::TypeKind for REGISTRATION_INFORMATION_CLASS {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for REGISTRATION_INFORMATION_CLASS {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("REGISTRATION_INFORMATION_CLASS").field(&self.0).finish()
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct MANAGEMENT_REGISTRATION_INFO {
    pub fDeviceRegisteredWithManagement: super::super::Foundation::BOOL,
    pub dwDeviceRegistionKind: u32,
    pub pszUPN: windows_core::PWSTR,
    pub pszMDMServiceUri: windows_core::PWSTR,
}
impl windows_core::TypeKind for MANAGEMENT_REGISTRATION_INFO {
    type TypeKind = windows_core::CopyType;
}
impl Default for MANAGEMENT_REGISTRATION_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct MANAGEMENT_SERVICE_INFO {
    pub pszMDMServiceUri: windows_core::PWSTR,
    pub pszAuthenticationUri: windows_core::PWSTR,
}
impl windows_core::TypeKind for MANAGEMENT_SERVICE_INFO {
    type TypeKind = windows_core::CopyType;
}
impl Default for MANAGEMENT_SERVICE_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
