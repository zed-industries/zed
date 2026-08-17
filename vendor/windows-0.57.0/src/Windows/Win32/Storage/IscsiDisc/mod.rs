#[inline]
pub unsafe fn AddISNSServerA<P0>(address: P0) -> u32
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("iscsidsc.dll" "system" fn AddISNSServerA(address : windows_core::PCSTR) -> u32);
    AddISNSServerA(address.param().abi())
}
#[inline]
pub unsafe fn AddISNSServerW<P0>(address: P0) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("iscsidsc.dll" "system" fn AddISNSServerW(address : windows_core::PCWSTR) -> u32);
    AddISNSServerW(address.param().abi())
}
#[inline]
pub unsafe fn AddIScsiConnectionA(uniquesessionid: *mut ISCSI_UNIQUE_SESSION_ID, reserved: *mut core::ffi::c_void, initiatorportnumber: u32, targetportal: *mut ISCSI_TARGET_PORTALA, securityflags: u64, loginoptions: *mut ISCSI_LOGIN_OPTIONS, key: Option<&[u8]>, connectionid: *mut ISCSI_UNIQUE_SESSION_ID) -> u32 {
    windows_targets::link!("iscsidsc.dll" "system" fn AddIScsiConnectionA(uniquesessionid : *mut ISCSI_UNIQUE_SESSION_ID, reserved : *mut core::ffi::c_void, initiatorportnumber : u32, targetportal : *mut ISCSI_TARGET_PORTALA, securityflags : u64, loginoptions : *mut ISCSI_LOGIN_OPTIONS, keysize : u32, key : windows_core::PCSTR, connectionid : *mut ISCSI_UNIQUE_SESSION_ID) -> u32);
    AddIScsiConnectionA(uniquesessionid, reserved, initiatorportnumber, targetportal, securityflags, loginoptions, key.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), core::mem::transmute(key.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), connectionid)
}
#[inline]
pub unsafe fn AddIScsiConnectionW(uniquesessionid: *mut ISCSI_UNIQUE_SESSION_ID, reserved: *mut core::ffi::c_void, initiatorportnumber: u32, targetportal: *mut ISCSI_TARGET_PORTALW, securityflags: u64, loginoptions: *mut ISCSI_LOGIN_OPTIONS, key: Option<&[u8]>, connectionid: *mut ISCSI_UNIQUE_SESSION_ID) -> u32 {
    windows_targets::link!("iscsidsc.dll" "system" fn AddIScsiConnectionW(uniquesessionid : *mut ISCSI_UNIQUE_SESSION_ID, reserved : *mut core::ffi::c_void, initiatorportnumber : u32, targetportal : *mut ISCSI_TARGET_PORTALW, securityflags : u64, loginoptions : *mut ISCSI_LOGIN_OPTIONS, keysize : u32, key : windows_core::PCSTR, connectionid : *mut ISCSI_UNIQUE_SESSION_ID) -> u32);
    AddIScsiConnectionW(uniquesessionid, reserved, initiatorportnumber, targetportal, securityflags, loginoptions, key.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), core::mem::transmute(key.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), connectionid)
}
#[inline]
pub unsafe fn AddIScsiSendTargetPortalA<P0>(initiatorinstance: P0, initiatorportnumber: u32, loginoptions: *mut ISCSI_LOGIN_OPTIONS, securityflags: u64, portal: *mut ISCSI_TARGET_PORTALA) -> u32
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("iscsidsc.dll" "system" fn AddIScsiSendTargetPortalA(initiatorinstance : windows_core::PCSTR, initiatorportnumber : u32, loginoptions : *mut ISCSI_LOGIN_OPTIONS, securityflags : u64, portal : *mut ISCSI_TARGET_PORTALA) -> u32);
    AddIScsiSendTargetPortalA(initiatorinstance.param().abi(), initiatorportnumber, loginoptions, securityflags, portal)
}
#[inline]
pub unsafe fn AddIScsiSendTargetPortalW<P0>(initiatorinstance: P0, initiatorportnumber: u32, loginoptions: *mut ISCSI_LOGIN_OPTIONS, securityflags: u64, portal: *mut ISCSI_TARGET_PORTALW) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("iscsidsc.dll" "system" fn AddIScsiSendTargetPortalW(initiatorinstance : windows_core::PCWSTR, initiatorportnumber : u32, loginoptions : *mut ISCSI_LOGIN_OPTIONS, securityflags : u64, portal : *mut ISCSI_TARGET_PORTALW) -> u32);
    AddIScsiSendTargetPortalW(initiatorinstance.param().abi(), initiatorportnumber, loginoptions, securityflags, portal)
}
#[inline]
pub unsafe fn AddIScsiStaticTargetA<P0, P1, P2>(targetname: P0, targetalias: P1, targetflags: u32, persist: P2, mappings: *mut ISCSI_TARGET_MAPPINGA, loginoptions: *mut ISCSI_LOGIN_OPTIONS, portalgroup: *mut ISCSI_TARGET_PORTAL_GROUPA) -> u32
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P1: windows_core::Param<windows_core::PCSTR>,
    P2: windows_core::Param<super::super::Foundation::BOOLEAN>,
{
    windows_targets::link!("iscsidsc.dll" "system" fn AddIScsiStaticTargetA(targetname : windows_core::PCSTR, targetalias : windows_core::PCSTR, targetflags : u32, persist : super::super::Foundation:: BOOLEAN, mappings : *mut ISCSI_TARGET_MAPPINGA, loginoptions : *mut ISCSI_LOGIN_OPTIONS, portalgroup : *mut ISCSI_TARGET_PORTAL_GROUPA) -> u32);
    AddIScsiStaticTargetA(targetname.param().abi(), targetalias.param().abi(), targetflags, persist.param().abi(), mappings, loginoptions, portalgroup)
}
#[inline]
pub unsafe fn AddIScsiStaticTargetW<P0, P1, P2>(targetname: P0, targetalias: P1, targetflags: u32, persist: P2, mappings: *mut ISCSI_TARGET_MAPPINGW, loginoptions: *mut ISCSI_LOGIN_OPTIONS, portalgroup: *mut ISCSI_TARGET_PORTAL_GROUPW) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<super::super::Foundation::BOOLEAN>,
{
    windows_targets::link!("iscsidsc.dll" "system" fn AddIScsiStaticTargetW(targetname : windows_core::PCWSTR, targetalias : windows_core::PCWSTR, targetflags : u32, persist : super::super::Foundation:: BOOLEAN, mappings : *mut ISCSI_TARGET_MAPPINGW, loginoptions : *mut ISCSI_LOGIN_OPTIONS, portalgroup : *mut ISCSI_TARGET_PORTAL_GROUPW) -> u32);
    AddIScsiStaticTargetW(targetname.param().abi(), targetalias.param().abi(), targetflags, persist.param().abi(), mappings, loginoptions, portalgroup)
}
#[inline]
pub unsafe fn AddPersistentIScsiDeviceA<P0>(devicepath: P0) -> u32
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("iscsidsc.dll" "system" fn AddPersistentIScsiDeviceA(devicepath : windows_core::PCSTR) -> u32);
    AddPersistentIScsiDeviceA(devicepath.param().abi())
}
#[inline]
pub unsafe fn AddPersistentIScsiDeviceW<P0>(devicepath: P0) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("iscsidsc.dll" "system" fn AddPersistentIScsiDeviceW(devicepath : windows_core::PCWSTR) -> u32);
    AddPersistentIScsiDeviceW(devicepath.param().abi())
}
#[inline]
pub unsafe fn AddRadiusServerA<P0>(address: P0) -> u32
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("iscsidsc.dll" "system" fn AddRadiusServerA(address : windows_core::PCSTR) -> u32);
    AddRadiusServerA(address.param().abi())
}
#[inline]
pub unsafe fn AddRadiusServerW<P0>(address: P0) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("iscsidsc.dll" "system" fn AddRadiusServerW(address : windows_core::PCWSTR) -> u32);
    AddRadiusServerW(address.param().abi())
}
#[inline]
pub unsafe fn ClearPersistentIScsiDevices() -> u32 {
    windows_targets::link!("iscsidsc.dll" "system" fn ClearPersistentIScsiDevices() -> u32);
    ClearPersistentIScsiDevices()
}
#[cfg(feature = "Win32_System_Ioctl")]
#[inline]
pub unsafe fn GetDevicesForIScsiSessionA(uniquesessionid: *mut ISCSI_UNIQUE_SESSION_ID, devicecount: *mut u32, devices: *mut ISCSI_DEVICE_ON_SESSIONA) -> u32 {
    windows_targets::link!("iscsidsc.dll" "system" fn GetDevicesForIScsiSessionA(uniquesessionid : *mut ISCSI_UNIQUE_SESSION_ID, devicecount : *mut u32, devices : *mut ISCSI_DEVICE_ON_SESSIONA) -> u32);
    GetDevicesForIScsiSessionA(uniquesessionid, devicecount, devices)
}
#[cfg(feature = "Win32_System_Ioctl")]
#[inline]
pub unsafe fn GetDevicesForIScsiSessionW(uniquesessionid: *mut ISCSI_UNIQUE_SESSION_ID, devicecount: *mut u32, devices: *mut ISCSI_DEVICE_ON_SESSIONW) -> u32 {
    windows_targets::link!("iscsidsc.dll" "system" fn GetDevicesForIScsiSessionW(uniquesessionid : *mut ISCSI_UNIQUE_SESSION_ID, devicecount : *mut u32, devices : *mut ISCSI_DEVICE_ON_SESSIONW) -> u32);
    GetDevicesForIScsiSessionW(uniquesessionid, devicecount, devices)
}
#[inline]
pub unsafe fn GetIScsiIKEInfoA<P0>(initiatorname: P0, initiatorportnumber: u32, reserved: *mut u32, authinfo: *mut IKE_AUTHENTICATION_INFORMATION) -> u32
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("iscsidsc.dll" "system" fn GetIScsiIKEInfoA(initiatorname : windows_core::PCSTR, initiatorportnumber : u32, reserved : *mut u32, authinfo : *mut IKE_AUTHENTICATION_INFORMATION) -> u32);
    GetIScsiIKEInfoA(initiatorname.param().abi(), initiatorportnumber, reserved, authinfo)
}
#[inline]
pub unsafe fn GetIScsiIKEInfoW<P0>(initiatorname: P0, initiatorportnumber: u32, reserved: *mut u32, authinfo: *mut IKE_AUTHENTICATION_INFORMATION) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("iscsidsc.dll" "system" fn GetIScsiIKEInfoW(initiatorname : windows_core::PCWSTR, initiatorportnumber : u32, reserved : *mut u32, authinfo : *mut IKE_AUTHENTICATION_INFORMATION) -> u32);
    GetIScsiIKEInfoW(initiatorname.param().abi(), initiatorportnumber, reserved, authinfo)
}
#[inline]
pub unsafe fn GetIScsiInitiatorNodeNameA(initiatornodename: windows_core::PSTR) -> u32 {
    windows_targets::link!("iscsidsc.dll" "system" fn GetIScsiInitiatorNodeNameA(initiatornodename : windows_core::PSTR) -> u32);
    GetIScsiInitiatorNodeNameA(core::mem::transmute(initiatornodename))
}
#[inline]
pub unsafe fn GetIScsiInitiatorNodeNameW(initiatornodename: windows_core::PWSTR) -> u32 {
    windows_targets::link!("iscsidsc.dll" "system" fn GetIScsiInitiatorNodeNameW(initiatornodename : windows_core::PWSTR) -> u32);
    GetIScsiInitiatorNodeNameW(core::mem::transmute(initiatornodename))
}
#[inline]
pub unsafe fn GetIScsiSessionListA(buffersize: *mut u32, sessioncount: *mut u32, sessioninfo: *mut ISCSI_SESSION_INFOA) -> u32 {
    windows_targets::link!("iscsidsc.dll" "system" fn GetIScsiSessionListA(buffersize : *mut u32, sessioncount : *mut u32, sessioninfo : *mut ISCSI_SESSION_INFOA) -> u32);
    GetIScsiSessionListA(buffersize, sessioncount, sessioninfo)
}
#[inline]
pub unsafe fn GetIScsiSessionListEx(buffersize: *mut u32, sessioncountptr: *mut u32, sessioninfo: *mut ISCSI_SESSION_INFO_EX) -> u32 {
    windows_targets::link!("iscsidsc.dll" "system" fn GetIScsiSessionListEx(buffersize : *mut u32, sessioncountptr : *mut u32, sessioninfo : *mut ISCSI_SESSION_INFO_EX) -> u32);
    GetIScsiSessionListEx(buffersize, sessioncountptr, sessioninfo)
}
#[inline]
pub unsafe fn GetIScsiSessionListW(buffersize: *mut u32, sessioncount: *mut u32, sessioninfo: *mut ISCSI_SESSION_INFOW) -> u32 {
    windows_targets::link!("iscsidsc.dll" "system" fn GetIScsiSessionListW(buffersize : *mut u32, sessioncount : *mut u32, sessioninfo : *mut ISCSI_SESSION_INFOW) -> u32);
    GetIScsiSessionListW(buffersize, sessioncount, sessioninfo)
}
#[inline]
pub unsafe fn GetIScsiTargetInformationA<P0, P1>(targetname: P0, discoverymechanism: P1, infoclass: TARGET_INFORMATION_CLASS, buffersize: *mut u32, buffer: *mut core::ffi::c_void) -> u32
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("iscsidsc.dll" "system" fn GetIScsiTargetInformationA(targetname : windows_core::PCSTR, discoverymechanism : windows_core::PCSTR, infoclass : TARGET_INFORMATION_CLASS, buffersize : *mut u32, buffer : *mut core::ffi::c_void) -> u32);
    GetIScsiTargetInformationA(targetname.param().abi(), discoverymechanism.param().abi(), infoclass, buffersize, buffer)
}
#[inline]
pub unsafe fn GetIScsiTargetInformationW<P0, P1>(targetname: P0, discoverymechanism: P1, infoclass: TARGET_INFORMATION_CLASS, buffersize: *mut u32, buffer: *mut core::ffi::c_void) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("iscsidsc.dll" "system" fn GetIScsiTargetInformationW(targetname : windows_core::PCWSTR, discoverymechanism : windows_core::PCWSTR, infoclass : TARGET_INFORMATION_CLASS, buffersize : *mut u32, buffer : *mut core::ffi::c_void) -> u32);
    GetIScsiTargetInformationW(targetname.param().abi(), discoverymechanism.param().abi(), infoclass, buffersize, buffer)
}
#[inline]
pub unsafe fn GetIScsiVersionInformation(versioninfo: *mut ISCSI_VERSION_INFO) -> u32 {
    windows_targets::link!("iscsidsc.dll" "system" fn GetIScsiVersionInformation(versioninfo : *mut ISCSI_VERSION_INFO) -> u32);
    GetIScsiVersionInformation(versioninfo)
}
#[inline]
pub unsafe fn LoginIScsiTargetA<P0, P1, P2, P3>(targetname: P0, isinformationalsession: P1, initiatorinstance: P2, initiatorportnumber: u32, targetportal: *mut ISCSI_TARGET_PORTALA, securityflags: u64, mappings: *mut ISCSI_TARGET_MAPPINGA, loginoptions: *mut ISCSI_LOGIN_OPTIONS, key: Option<&[u8]>, ispersistent: P3, uniquesessionid: *mut ISCSI_UNIQUE_SESSION_ID, uniqueconnectionid: *mut ISCSI_UNIQUE_SESSION_ID) -> u32
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P1: windows_core::Param<super::super::Foundation::BOOLEAN>,
    P2: windows_core::Param<windows_core::PCSTR>,
    P3: windows_core::Param<super::super::Foundation::BOOLEAN>,
{
    windows_targets::link!("iscsidsc.dll" "system" fn LoginIScsiTargetA(targetname : windows_core::PCSTR, isinformationalsession : super::super::Foundation:: BOOLEAN, initiatorinstance : windows_core::PCSTR, initiatorportnumber : u32, targetportal : *mut ISCSI_TARGET_PORTALA, securityflags : u64, mappings : *mut ISCSI_TARGET_MAPPINGA, loginoptions : *mut ISCSI_LOGIN_OPTIONS, keysize : u32, key : windows_core::PCSTR, ispersistent : super::super::Foundation:: BOOLEAN, uniquesessionid : *mut ISCSI_UNIQUE_SESSION_ID, uniqueconnectionid : *mut ISCSI_UNIQUE_SESSION_ID) -> u32);
    LoginIScsiTargetA(targetname.param().abi(), isinformationalsession.param().abi(), initiatorinstance.param().abi(), initiatorportnumber, targetportal, securityflags, mappings, loginoptions, key.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), core::mem::transmute(key.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), ispersistent.param().abi(), uniquesessionid, uniqueconnectionid)
}
#[inline]
pub unsafe fn LoginIScsiTargetW<P0, P1, P2, P3>(targetname: P0, isinformationalsession: P1, initiatorinstance: P2, initiatorportnumber: u32, targetportal: *mut ISCSI_TARGET_PORTALW, securityflags: u64, mappings: *mut ISCSI_TARGET_MAPPINGW, loginoptions: *mut ISCSI_LOGIN_OPTIONS, key: Option<&[u8]>, ispersistent: P3, uniquesessionid: *mut ISCSI_UNIQUE_SESSION_ID, uniqueconnectionid: *mut ISCSI_UNIQUE_SESSION_ID) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<super::super::Foundation::BOOLEAN>,
    P2: windows_core::Param<windows_core::PCWSTR>,
    P3: windows_core::Param<super::super::Foundation::BOOLEAN>,
{
    windows_targets::link!("iscsidsc.dll" "system" fn LoginIScsiTargetW(targetname : windows_core::PCWSTR, isinformationalsession : super::super::Foundation:: BOOLEAN, initiatorinstance : windows_core::PCWSTR, initiatorportnumber : u32, targetportal : *mut ISCSI_TARGET_PORTALW, securityflags : u64, mappings : *mut ISCSI_TARGET_MAPPINGW, loginoptions : *mut ISCSI_LOGIN_OPTIONS, keysize : u32, key : windows_core::PCSTR, ispersistent : super::super::Foundation:: BOOLEAN, uniquesessionid : *mut ISCSI_UNIQUE_SESSION_ID, uniqueconnectionid : *mut ISCSI_UNIQUE_SESSION_ID) -> u32);
    LoginIScsiTargetW(targetname.param().abi(), isinformationalsession.param().abi(), initiatorinstance.param().abi(), initiatorportnumber, targetportal, securityflags, mappings, loginoptions, key.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), core::mem::transmute(key.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), ispersistent.param().abi(), uniquesessionid, uniqueconnectionid)
}
#[inline]
pub unsafe fn LogoutIScsiTarget(uniquesessionid: *mut ISCSI_UNIQUE_SESSION_ID) -> u32 {
    windows_targets::link!("iscsidsc.dll" "system" fn LogoutIScsiTarget(uniquesessionid : *mut ISCSI_UNIQUE_SESSION_ID) -> u32);
    LogoutIScsiTarget(uniquesessionid)
}
#[inline]
pub unsafe fn RefreshISNSServerA<P0>(address: P0) -> u32
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("iscsidsc.dll" "system" fn RefreshISNSServerA(address : windows_core::PCSTR) -> u32);
    RefreshISNSServerA(address.param().abi())
}
#[inline]
pub unsafe fn RefreshISNSServerW<P0>(address: P0) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("iscsidsc.dll" "system" fn RefreshISNSServerW(address : windows_core::PCWSTR) -> u32);
    RefreshISNSServerW(address.param().abi())
}
#[inline]
pub unsafe fn RefreshIScsiSendTargetPortalA<P0>(initiatorinstance: P0, initiatorportnumber: u32, portal: *mut ISCSI_TARGET_PORTALA) -> u32
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("iscsidsc.dll" "system" fn RefreshIScsiSendTargetPortalA(initiatorinstance : windows_core::PCSTR, initiatorportnumber : u32, portal : *mut ISCSI_TARGET_PORTALA) -> u32);
    RefreshIScsiSendTargetPortalA(initiatorinstance.param().abi(), initiatorportnumber, portal)
}
#[inline]
pub unsafe fn RefreshIScsiSendTargetPortalW<P0>(initiatorinstance: P0, initiatorportnumber: u32, portal: *mut ISCSI_TARGET_PORTALW) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("iscsidsc.dll" "system" fn RefreshIScsiSendTargetPortalW(initiatorinstance : windows_core::PCWSTR, initiatorportnumber : u32, portal : *mut ISCSI_TARGET_PORTALW) -> u32);
    RefreshIScsiSendTargetPortalW(initiatorinstance.param().abi(), initiatorportnumber, portal)
}
#[inline]
pub unsafe fn RemoveISNSServerA<P0>(address: P0) -> u32
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("iscsidsc.dll" "system" fn RemoveISNSServerA(address : windows_core::PCSTR) -> u32);
    RemoveISNSServerA(address.param().abi())
}
#[inline]
pub unsafe fn RemoveISNSServerW<P0>(address: P0) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("iscsidsc.dll" "system" fn RemoveISNSServerW(address : windows_core::PCWSTR) -> u32);
    RemoveISNSServerW(address.param().abi())
}
#[inline]
pub unsafe fn RemoveIScsiConnection(uniquesessionid: *mut ISCSI_UNIQUE_SESSION_ID, connectionid: *mut ISCSI_UNIQUE_SESSION_ID) -> u32 {
    windows_targets::link!("iscsidsc.dll" "system" fn RemoveIScsiConnection(uniquesessionid : *mut ISCSI_UNIQUE_SESSION_ID, connectionid : *mut ISCSI_UNIQUE_SESSION_ID) -> u32);
    RemoveIScsiConnection(uniquesessionid, connectionid)
}
#[inline]
pub unsafe fn RemoveIScsiPersistentTargetA<P0, P1>(initiatorinstance: P0, initiatorportnumber: u32, targetname: P1, portal: *mut ISCSI_TARGET_PORTALA) -> u32
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("iscsidsc.dll" "system" fn RemoveIScsiPersistentTargetA(initiatorinstance : windows_core::PCSTR, initiatorportnumber : u32, targetname : windows_core::PCSTR, portal : *mut ISCSI_TARGET_PORTALA) -> u32);
    RemoveIScsiPersistentTargetA(initiatorinstance.param().abi(), initiatorportnumber, targetname.param().abi(), portal)
}
#[inline]
pub unsafe fn RemoveIScsiPersistentTargetW<P0, P1>(initiatorinstance: P0, initiatorportnumber: u32, targetname: P1, portal: *mut ISCSI_TARGET_PORTALW) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("iscsidsc.dll" "system" fn RemoveIScsiPersistentTargetW(initiatorinstance : windows_core::PCWSTR, initiatorportnumber : u32, targetname : windows_core::PCWSTR, portal : *mut ISCSI_TARGET_PORTALW) -> u32);
    RemoveIScsiPersistentTargetW(initiatorinstance.param().abi(), initiatorportnumber, targetname.param().abi(), portal)
}
#[inline]
pub unsafe fn RemoveIScsiSendTargetPortalA<P0>(initiatorinstance: P0, initiatorportnumber: u32, portal: *mut ISCSI_TARGET_PORTALA) -> u32
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("iscsidsc.dll" "system" fn RemoveIScsiSendTargetPortalA(initiatorinstance : windows_core::PCSTR, initiatorportnumber : u32, portal : *mut ISCSI_TARGET_PORTALA) -> u32);
    RemoveIScsiSendTargetPortalA(initiatorinstance.param().abi(), initiatorportnumber, portal)
}
#[inline]
pub unsafe fn RemoveIScsiSendTargetPortalW<P0>(initiatorinstance: P0, initiatorportnumber: u32, portal: *mut ISCSI_TARGET_PORTALW) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("iscsidsc.dll" "system" fn RemoveIScsiSendTargetPortalW(initiatorinstance : windows_core::PCWSTR, initiatorportnumber : u32, portal : *mut ISCSI_TARGET_PORTALW) -> u32);
    RemoveIScsiSendTargetPortalW(initiatorinstance.param().abi(), initiatorportnumber, portal)
}
#[inline]
pub unsafe fn RemoveIScsiStaticTargetA<P0>(targetname: P0) -> u32
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("iscsidsc.dll" "system" fn RemoveIScsiStaticTargetA(targetname : windows_core::PCSTR) -> u32);
    RemoveIScsiStaticTargetA(targetname.param().abi())
}
#[inline]
pub unsafe fn RemoveIScsiStaticTargetW<P0>(targetname: P0) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("iscsidsc.dll" "system" fn RemoveIScsiStaticTargetW(targetname : windows_core::PCWSTR) -> u32);
    RemoveIScsiStaticTargetW(targetname.param().abi())
}
#[inline]
pub unsafe fn RemovePersistentIScsiDeviceA<P0>(devicepath: P0) -> u32
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("iscsidsc.dll" "system" fn RemovePersistentIScsiDeviceA(devicepath : windows_core::PCSTR) -> u32);
    RemovePersistentIScsiDeviceA(devicepath.param().abi())
}
#[inline]
pub unsafe fn RemovePersistentIScsiDeviceW<P0>(devicepath: P0) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("iscsidsc.dll" "system" fn RemovePersistentIScsiDeviceW(devicepath : windows_core::PCWSTR) -> u32);
    RemovePersistentIScsiDeviceW(devicepath.param().abi())
}
#[inline]
pub unsafe fn RemoveRadiusServerA<P0>(address: P0) -> u32
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("iscsidsc.dll" "system" fn RemoveRadiusServerA(address : windows_core::PCSTR) -> u32);
    RemoveRadiusServerA(address.param().abi())
}
#[inline]
pub unsafe fn RemoveRadiusServerW<P0>(address: P0) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("iscsidsc.dll" "system" fn RemoveRadiusServerW(address : windows_core::PCWSTR) -> u32);
    RemoveRadiusServerW(address.param().abi())
}
#[inline]
pub unsafe fn ReportActiveIScsiTargetMappingsA(buffersize: *mut u32, mappingcount: *mut u32, mappings: *mut ISCSI_TARGET_MAPPINGA) -> u32 {
    windows_targets::link!("iscsidsc.dll" "system" fn ReportActiveIScsiTargetMappingsA(buffersize : *mut u32, mappingcount : *mut u32, mappings : *mut ISCSI_TARGET_MAPPINGA) -> u32);
    ReportActiveIScsiTargetMappingsA(buffersize, mappingcount, mappings)
}
#[inline]
pub unsafe fn ReportActiveIScsiTargetMappingsW(buffersize: *mut u32, mappingcount: *mut u32, mappings: *mut ISCSI_TARGET_MAPPINGW) -> u32 {
    windows_targets::link!("iscsidsc.dll" "system" fn ReportActiveIScsiTargetMappingsW(buffersize : *mut u32, mappingcount : *mut u32, mappings : *mut ISCSI_TARGET_MAPPINGW) -> u32);
    ReportActiveIScsiTargetMappingsW(buffersize, mappingcount, mappings)
}
#[inline]
pub unsafe fn ReportISNSServerListA(buffersizeinchar: *mut u32, buffer: windows_core::PSTR) -> u32 {
    windows_targets::link!("iscsidsc.dll" "system" fn ReportISNSServerListA(buffersizeinchar : *mut u32, buffer : windows_core::PSTR) -> u32);
    ReportISNSServerListA(buffersizeinchar, core::mem::transmute(buffer))
}
#[inline]
pub unsafe fn ReportISNSServerListW(buffersizeinchar: *mut u32, buffer: windows_core::PWSTR) -> u32 {
    windows_targets::link!("iscsidsc.dll" "system" fn ReportISNSServerListW(buffersizeinchar : *mut u32, buffer : windows_core::PWSTR) -> u32);
    ReportISNSServerListW(buffersizeinchar, core::mem::transmute(buffer))
}
#[inline]
pub unsafe fn ReportIScsiInitiatorListA(buffersize: *mut u32, buffer: windows_core::PSTR) -> u32 {
    windows_targets::link!("iscsidsc.dll" "system" fn ReportIScsiInitiatorListA(buffersize : *mut u32, buffer : windows_core::PSTR) -> u32);
    ReportIScsiInitiatorListA(buffersize, core::mem::transmute(buffer))
}
#[inline]
pub unsafe fn ReportIScsiInitiatorListW(buffersize: *mut u32, buffer: windows_core::PWSTR) -> u32 {
    windows_targets::link!("iscsidsc.dll" "system" fn ReportIScsiInitiatorListW(buffersize : *mut u32, buffer : windows_core::PWSTR) -> u32);
    ReportIScsiInitiatorListW(buffersize, core::mem::transmute(buffer))
}
#[inline]
pub unsafe fn ReportIScsiPersistentLoginsA(count: *mut u32, persistentlogininfo: *mut PERSISTENT_ISCSI_LOGIN_INFOA, buffersizeinbytes: *mut u32) -> u32 {
    windows_targets::link!("iscsidsc.dll" "system" fn ReportIScsiPersistentLoginsA(count : *mut u32, persistentlogininfo : *mut PERSISTENT_ISCSI_LOGIN_INFOA, buffersizeinbytes : *mut u32) -> u32);
    ReportIScsiPersistentLoginsA(count, persistentlogininfo, buffersizeinbytes)
}
#[inline]
pub unsafe fn ReportIScsiPersistentLoginsW(count: *mut u32, persistentlogininfo: *mut PERSISTENT_ISCSI_LOGIN_INFOW, buffersizeinbytes: *mut u32) -> u32 {
    windows_targets::link!("iscsidsc.dll" "system" fn ReportIScsiPersistentLoginsW(count : *mut u32, persistentlogininfo : *mut PERSISTENT_ISCSI_LOGIN_INFOW, buffersizeinbytes : *mut u32) -> u32);
    ReportIScsiPersistentLoginsW(count, persistentlogininfo, buffersizeinbytes)
}
#[inline]
pub unsafe fn ReportIScsiSendTargetPortalsA(portalcount: *mut u32, portalinfo: *mut ISCSI_TARGET_PORTAL_INFOA) -> u32 {
    windows_targets::link!("iscsidsc.dll" "system" fn ReportIScsiSendTargetPortalsA(portalcount : *mut u32, portalinfo : *mut ISCSI_TARGET_PORTAL_INFOA) -> u32);
    ReportIScsiSendTargetPortalsA(portalcount, portalinfo)
}
#[inline]
pub unsafe fn ReportIScsiSendTargetPortalsExA(portalcount: *mut u32, portalinfosize: *mut u32, portalinfo: *mut ISCSI_TARGET_PORTAL_INFO_EXA) -> u32 {
    windows_targets::link!("iscsidsc.dll" "system" fn ReportIScsiSendTargetPortalsExA(portalcount : *mut u32, portalinfosize : *mut u32, portalinfo : *mut ISCSI_TARGET_PORTAL_INFO_EXA) -> u32);
    ReportIScsiSendTargetPortalsExA(portalcount, portalinfosize, portalinfo)
}
#[inline]
pub unsafe fn ReportIScsiSendTargetPortalsExW(portalcount: *mut u32, portalinfosize: *mut u32, portalinfo: *mut ISCSI_TARGET_PORTAL_INFO_EXW) -> u32 {
    windows_targets::link!("iscsidsc.dll" "system" fn ReportIScsiSendTargetPortalsExW(portalcount : *mut u32, portalinfosize : *mut u32, portalinfo : *mut ISCSI_TARGET_PORTAL_INFO_EXW) -> u32);
    ReportIScsiSendTargetPortalsExW(portalcount, portalinfosize, portalinfo)
}
#[inline]
pub unsafe fn ReportIScsiSendTargetPortalsW(portalcount: *mut u32, portalinfo: *mut ISCSI_TARGET_PORTAL_INFOW) -> u32 {
    windows_targets::link!("iscsidsc.dll" "system" fn ReportIScsiSendTargetPortalsW(portalcount : *mut u32, portalinfo : *mut ISCSI_TARGET_PORTAL_INFOW) -> u32);
    ReportIScsiSendTargetPortalsW(portalcount, portalinfo)
}
#[inline]
pub unsafe fn ReportIScsiTargetPortalsA<P0, P1>(initiatorname: P0, targetname: P1, targetportaltag: *mut u16, elementcount: *mut u32, portals: *mut ISCSI_TARGET_PORTALA) -> u32
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("iscsidsc.dll" "system" fn ReportIScsiTargetPortalsA(initiatorname : windows_core::PCSTR, targetname : windows_core::PCSTR, targetportaltag : *mut u16, elementcount : *mut u32, portals : *mut ISCSI_TARGET_PORTALA) -> u32);
    ReportIScsiTargetPortalsA(initiatorname.param().abi(), targetname.param().abi(), targetportaltag, elementcount, portals)
}
#[inline]
pub unsafe fn ReportIScsiTargetPortalsW<P0, P1>(initiatorname: P0, targetname: P1, targetportaltag: *mut u16, elementcount: *mut u32, portals: *mut ISCSI_TARGET_PORTALW) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("iscsidsc.dll" "system" fn ReportIScsiTargetPortalsW(initiatorname : windows_core::PCWSTR, targetname : windows_core::PCWSTR, targetportaltag : *mut u16, elementcount : *mut u32, portals : *mut ISCSI_TARGET_PORTALW) -> u32);
    ReportIScsiTargetPortalsW(initiatorname.param().abi(), targetname.param().abi(), targetportaltag, elementcount, portals)
}
#[inline]
pub unsafe fn ReportIScsiTargetsA<P0>(forceupdate: P0, buffersize: *mut u32, buffer: windows_core::PSTR) -> u32
where
    P0: windows_core::Param<super::super::Foundation::BOOLEAN>,
{
    windows_targets::link!("iscsidsc.dll" "system" fn ReportIScsiTargetsA(forceupdate : super::super::Foundation:: BOOLEAN, buffersize : *mut u32, buffer : windows_core::PSTR) -> u32);
    ReportIScsiTargetsA(forceupdate.param().abi(), buffersize, core::mem::transmute(buffer))
}
#[inline]
pub unsafe fn ReportIScsiTargetsW<P0>(forceupdate: P0, buffersize: *mut u32, buffer: windows_core::PWSTR) -> u32
where
    P0: windows_core::Param<super::super::Foundation::BOOLEAN>,
{
    windows_targets::link!("iscsidsc.dll" "system" fn ReportIScsiTargetsW(forceupdate : super::super::Foundation:: BOOLEAN, buffersize : *mut u32, buffer : windows_core::PWSTR) -> u32);
    ReportIScsiTargetsW(forceupdate.param().abi(), buffersize, core::mem::transmute(buffer))
}
#[inline]
pub unsafe fn ReportPersistentIScsiDevicesA(buffersizeinchar: *mut u32, buffer: windows_core::PSTR) -> u32 {
    windows_targets::link!("iscsidsc.dll" "system" fn ReportPersistentIScsiDevicesA(buffersizeinchar : *mut u32, buffer : windows_core::PSTR) -> u32);
    ReportPersistentIScsiDevicesA(buffersizeinchar, core::mem::transmute(buffer))
}
#[inline]
pub unsafe fn ReportPersistentIScsiDevicesW(buffersizeinchar: *mut u32, buffer: windows_core::PWSTR) -> u32 {
    windows_targets::link!("iscsidsc.dll" "system" fn ReportPersistentIScsiDevicesW(buffersizeinchar : *mut u32, buffer : windows_core::PWSTR) -> u32);
    ReportPersistentIScsiDevicesW(buffersizeinchar, core::mem::transmute(buffer))
}
#[inline]
pub unsafe fn ReportRadiusServerListA(buffersizeinchar: *mut u32, buffer: windows_core::PSTR) -> u32 {
    windows_targets::link!("iscsidsc.dll" "system" fn ReportRadiusServerListA(buffersizeinchar : *mut u32, buffer : windows_core::PSTR) -> u32);
    ReportRadiusServerListA(buffersizeinchar, core::mem::transmute(buffer))
}
#[inline]
pub unsafe fn ReportRadiusServerListW(buffersizeinchar: *mut u32, buffer: windows_core::PWSTR) -> u32 {
    windows_targets::link!("iscsidsc.dll" "system" fn ReportRadiusServerListW(buffersizeinchar : *mut u32, buffer : windows_core::PWSTR) -> u32);
    ReportRadiusServerListW(buffersizeinchar, core::mem::transmute(buffer))
}
#[inline]
pub unsafe fn SendScsiInquiry(uniquesessionid: *mut ISCSI_UNIQUE_SESSION_ID, lun: u64, evpdcmddt: u8, pagecode: u8, scsistatus: *mut u8, responsesize: *mut u32, responsebuffer: *mut u8, sensesize: *mut u32, sensebuffer: *mut u8) -> u32 {
    windows_targets::link!("iscsidsc.dll" "system" fn SendScsiInquiry(uniquesessionid : *mut ISCSI_UNIQUE_SESSION_ID, lun : u64, evpdcmddt : u8, pagecode : u8, scsistatus : *mut u8, responsesize : *mut u32, responsebuffer : *mut u8, sensesize : *mut u32, sensebuffer : *mut u8) -> u32);
    SendScsiInquiry(uniquesessionid, lun, evpdcmddt, pagecode, scsistatus, responsesize, responsebuffer, sensesize, sensebuffer)
}
#[inline]
pub unsafe fn SendScsiReadCapacity(uniquesessionid: *mut ISCSI_UNIQUE_SESSION_ID, lun: u64, scsistatus: *mut u8, responsesize: *mut u32, responsebuffer: *mut u8, sensesize: *mut u32, sensebuffer: *mut u8) -> u32 {
    windows_targets::link!("iscsidsc.dll" "system" fn SendScsiReadCapacity(uniquesessionid : *mut ISCSI_UNIQUE_SESSION_ID, lun : u64, scsistatus : *mut u8, responsesize : *mut u32, responsebuffer : *mut u8, sensesize : *mut u32, sensebuffer : *mut u8) -> u32);
    SendScsiReadCapacity(uniquesessionid, lun, scsistatus, responsesize, responsebuffer, sensesize, sensebuffer)
}
#[inline]
pub unsafe fn SendScsiReportLuns(uniquesessionid: *mut ISCSI_UNIQUE_SESSION_ID, scsistatus: *mut u8, responsesize: *mut u32, responsebuffer: *mut u8, sensesize: *mut u32, sensebuffer: *mut u8) -> u32 {
    windows_targets::link!("iscsidsc.dll" "system" fn SendScsiReportLuns(uniquesessionid : *mut ISCSI_UNIQUE_SESSION_ID, scsistatus : *mut u8, responsesize : *mut u32, responsebuffer : *mut u8, sensesize : *mut u32, sensebuffer : *mut u8) -> u32);
    SendScsiReportLuns(uniquesessionid, scsistatus, responsesize, responsebuffer, sensesize, sensebuffer)
}
#[inline]
pub unsafe fn SetIScsiGroupPresharedKey<P0>(keylength: u32, key: *mut u8, persist: P0) -> u32
where
    P0: windows_core::Param<super::super::Foundation::BOOLEAN>,
{
    windows_targets::link!("iscsidsc.dll" "system" fn SetIScsiGroupPresharedKey(keylength : u32, key : *mut u8, persist : super::super::Foundation:: BOOLEAN) -> u32);
    SetIScsiGroupPresharedKey(keylength, key, persist.param().abi())
}
#[inline]
pub unsafe fn SetIScsiIKEInfoA<P0, P1>(initiatorname: P0, initiatorportnumber: u32, authinfo: *mut IKE_AUTHENTICATION_INFORMATION, persist: P1) -> u32
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P1: windows_core::Param<super::super::Foundation::BOOLEAN>,
{
    windows_targets::link!("iscsidsc.dll" "system" fn SetIScsiIKEInfoA(initiatorname : windows_core::PCSTR, initiatorportnumber : u32, authinfo : *mut IKE_AUTHENTICATION_INFORMATION, persist : super::super::Foundation:: BOOLEAN) -> u32);
    SetIScsiIKEInfoA(initiatorname.param().abi(), initiatorportnumber, authinfo, persist.param().abi())
}
#[inline]
pub unsafe fn SetIScsiIKEInfoW<P0, P1>(initiatorname: P0, initiatorportnumber: u32, authinfo: *mut IKE_AUTHENTICATION_INFORMATION, persist: P1) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<super::super::Foundation::BOOLEAN>,
{
    windows_targets::link!("iscsidsc.dll" "system" fn SetIScsiIKEInfoW(initiatorname : windows_core::PCWSTR, initiatorportnumber : u32, authinfo : *mut IKE_AUTHENTICATION_INFORMATION, persist : super::super::Foundation:: BOOLEAN) -> u32);
    SetIScsiIKEInfoW(initiatorname.param().abi(), initiatorportnumber, authinfo, persist.param().abi())
}
#[inline]
pub unsafe fn SetIScsiInitiatorCHAPSharedSecret(sharedsecretlength: u32, sharedsecret: *mut u8) -> u32 {
    windows_targets::link!("iscsidsc.dll" "system" fn SetIScsiInitiatorCHAPSharedSecret(sharedsecretlength : u32, sharedsecret : *mut u8) -> u32);
    SetIScsiInitiatorCHAPSharedSecret(sharedsecretlength, sharedsecret)
}
#[inline]
pub unsafe fn SetIScsiInitiatorNodeNameA<P0>(initiatornodename: P0) -> u32
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("iscsidsc.dll" "system" fn SetIScsiInitiatorNodeNameA(initiatornodename : windows_core::PCSTR) -> u32);
    SetIScsiInitiatorNodeNameA(initiatornodename.param().abi())
}
#[inline]
pub unsafe fn SetIScsiInitiatorNodeNameW<P0>(initiatornodename: P0) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("iscsidsc.dll" "system" fn SetIScsiInitiatorNodeNameW(initiatornodename : windows_core::PCWSTR) -> u32);
    SetIScsiInitiatorNodeNameW(initiatornodename.param().abi())
}
#[inline]
pub unsafe fn SetIScsiInitiatorRADIUSSharedSecret(sharedsecretlength: u32, sharedsecret: *mut u8) -> u32 {
    windows_targets::link!("iscsidsc.dll" "system" fn SetIScsiInitiatorRADIUSSharedSecret(sharedsecretlength : u32, sharedsecret : *mut u8) -> u32);
    SetIScsiInitiatorRADIUSSharedSecret(sharedsecretlength, sharedsecret)
}
#[inline]
pub unsafe fn SetIScsiTunnelModeOuterAddressA<P0, P1, P2, P3>(initiatorname: P0, initiatorportnumber: u32, destinationaddress: P1, outermodeaddress: P2, persist: P3) -> u32
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P1: windows_core::Param<windows_core::PCSTR>,
    P2: windows_core::Param<windows_core::PCSTR>,
    P3: windows_core::Param<super::super::Foundation::BOOLEAN>,
{
    windows_targets::link!("iscsidsc.dll" "system" fn SetIScsiTunnelModeOuterAddressA(initiatorname : windows_core::PCSTR, initiatorportnumber : u32, destinationaddress : windows_core::PCSTR, outermodeaddress : windows_core::PCSTR, persist : super::super::Foundation:: BOOLEAN) -> u32);
    SetIScsiTunnelModeOuterAddressA(initiatorname.param().abi(), initiatorportnumber, destinationaddress.param().abi(), outermodeaddress.param().abi(), persist.param().abi())
}
#[inline]
pub unsafe fn SetIScsiTunnelModeOuterAddressW<P0, P1, P2, P3>(initiatorname: P0, initiatorportnumber: u32, destinationaddress: P1, outermodeaddress: P2, persist: P3) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
    P3: windows_core::Param<super::super::Foundation::BOOLEAN>,
{
    windows_targets::link!("iscsidsc.dll" "system" fn SetIScsiTunnelModeOuterAddressW(initiatorname : windows_core::PCWSTR, initiatorportnumber : u32, destinationaddress : windows_core::PCWSTR, outermodeaddress : windows_core::PCWSTR, persist : super::super::Foundation:: BOOLEAN) -> u32);
    SetIScsiTunnelModeOuterAddressW(initiatorname.param().abi(), initiatorportnumber, destinationaddress.param().abi(), outermodeaddress.param().abi(), persist.param().abi())
}
#[inline]
pub unsafe fn SetupPersistentIScsiDevices() -> u32 {
    windows_targets::link!("iscsidsc.dll" "system" fn SetupPersistentIScsiDevices() -> u32);
    SetupPersistentIScsiDevices()
}
#[inline]
pub unsafe fn SetupPersistentIScsiVolumes() -> u32 {
    windows_targets::link!("iscsidsc.dll" "system" fn SetupPersistentIScsiVolumes() -> u32);
    SetupPersistentIScsiVolumes()
}
pub const ATA_FLAGS_48BIT_COMMAND: u32 = 8u32;
pub const ATA_FLAGS_DATA_IN: u32 = 2u32;
pub const ATA_FLAGS_DATA_OUT: u32 = 4u32;
pub const ATA_FLAGS_DRDY_REQUIRED: u32 = 1u32;
pub const ATA_FLAGS_NO_MULTIPLE: u32 = 32u32;
pub const ATA_FLAGS_USE_DMA: u32 = 16u32;
pub const DD_SCSI_DEVICE_NAME: windows_core::PCSTR = windows_core::s!("\\Device\\ScsiPort");
pub const DUMP_DRIVER_NAME_LENGTH: u32 = 15u32;
pub const DUMP_EX_FLAG_DRIVER_FULL_PATH_SUPPORT: u32 = 8u32;
pub const DUMP_EX_FLAG_RESUME_SUPPORT: u32 = 4u32;
pub const DUMP_EX_FLAG_SUPPORT_64BITMEMORY: u32 = 1u32;
pub const DUMP_EX_FLAG_SUPPORT_DD_TELEMETRY: u32 = 2u32;
pub const DUMP_POINTERS_VERSION_1: u32 = 1u32;
pub const DUMP_POINTERS_VERSION_2: u32 = 2u32;
pub const DUMP_POINTERS_VERSION_3: u32 = 3u32;
pub const DUMP_POINTERS_VERSION_4: u32 = 4u32;
pub const DiscoveryMechanisms: TARGET_INFORMATION_CLASS = TARGET_INFORMATION_CLASS(2i32);
pub const FILE_DEVICE_SCSI: u32 = 27u32;
pub const FIRMWARE_FUNCTION_ACTIVATE: u32 = 3u32;
pub const FIRMWARE_FUNCTION_DOWNLOAD: u32 = 2u32;
pub const FIRMWARE_FUNCTION_GET_INFO: u32 = 1u32;
pub const FIRMWARE_REQUEST_BLOCK_STRUCTURE_VERSION: u32 = 1u32;
pub const FIRMWARE_REQUEST_FLAG_CONTROLLER: u32 = 1u32;
pub const FIRMWARE_REQUEST_FLAG_FIRST_SEGMENT: u32 = 4u32;
pub const FIRMWARE_REQUEST_FLAG_LAST_SEGMENT: u32 = 2u32;
pub const FIRMWARE_REQUEST_FLAG_REPLACE_EXISTING_IMAGE: u32 = 1073741824u32;
pub const FIRMWARE_REQUEST_FLAG_SWITCH_TO_EXISTING_FIRMWARE: u32 = 2147483648u32;
pub const FIRMWARE_STATUS_COMMAND_ABORT: u32 = 133u32;
pub const FIRMWARE_STATUS_CONTROLLER_ERROR: u32 = 16u32;
pub const FIRMWARE_STATUS_DEVICE_ERROR: u32 = 64u32;
pub const FIRMWARE_STATUS_END_OF_MEDIA: u32 = 134u32;
pub const FIRMWARE_STATUS_ERROR: u32 = 1u32;
pub const FIRMWARE_STATUS_ID_NOT_FOUND: u32 = 131u32;
pub const FIRMWARE_STATUS_ILLEGAL_LENGTH: u32 = 135u32;
pub const FIRMWARE_STATUS_ILLEGAL_REQUEST: u32 = 2u32;
pub const FIRMWARE_STATUS_INPUT_BUFFER_TOO_BIG: u32 = 4u32;
pub const FIRMWARE_STATUS_INTERFACE_CRC_ERROR: u32 = 128u32;
pub const FIRMWARE_STATUS_INVALID_IMAGE: u32 = 7u32;
pub const FIRMWARE_STATUS_INVALID_PARAMETER: u32 = 3u32;
pub const FIRMWARE_STATUS_INVALID_SLOT: u32 = 6u32;
pub const FIRMWARE_STATUS_MEDIA_CHANGE: u32 = 130u32;
pub const FIRMWARE_STATUS_MEDIA_CHANGE_REQUEST: u32 = 132u32;
pub const FIRMWARE_STATUS_OUTPUT_BUFFER_TOO_SMALL: u32 = 5u32;
pub const FIRMWARE_STATUS_POWER_CYCLE_REQUIRED: u32 = 32u32;
pub const FIRMWARE_STATUS_SUCCESS: u32 = 0u32;
pub const FIRMWARE_STATUS_UNCORRECTABLE_DATA_ERROR: u32 = 129u32;
pub const HYBRID_FUNCTION_DEMOTE_BY_SIZE: u32 = 19u32;
pub const HYBRID_FUNCTION_DISABLE_CACHING_MEDIUM: u32 = 16u32;
pub const HYBRID_FUNCTION_ENABLE_CACHING_MEDIUM: u32 = 17u32;
pub const HYBRID_FUNCTION_GET_INFO: u32 = 1u32;
pub const HYBRID_FUNCTION_SET_DIRTY_THRESHOLD: u32 = 18u32;
pub const HYBRID_REQUEST_BLOCK_STRUCTURE_VERSION: u32 = 1u32;
pub const HYBRID_REQUEST_INFO_STRUCTURE_VERSION: u32 = 1u32;
pub const HYBRID_STATUS_ENABLE_REFCOUNT_HOLD: u32 = 16u32;
pub const HYBRID_STATUS_ILLEGAL_REQUEST: u32 = 1u32;
pub const HYBRID_STATUS_INVALID_PARAMETER: u32 = 2u32;
pub const HYBRID_STATUS_OUTPUT_BUFFER_TOO_SMALL: u32 = 3u32;
pub const HYBRID_STATUS_SUCCESS: u32 = 0u32;
pub const ID_FQDN: windows_core::PCSTR = windows_core::s!("2");
pub const ID_IPV4_ADDR: windows_core::PCSTR = windows_core::s!("1");
pub const ID_IPV6_ADDR: windows_core::PCSTR = windows_core::s!("5");
pub const ID_USER_FQDN: windows_core::PCSTR = windows_core::s!("3");
pub const IKE_AUTHENTICATION_PRESHARED_KEY_METHOD: IKE_AUTHENTICATION_METHOD = IKE_AUTHENTICATION_METHOD(1i32);
pub const IOCTL_ATA_MINIPORT: u32 = 315444u32;
pub const IOCTL_ATA_PASS_THROUGH: u32 = 315436u32;
pub const IOCTL_ATA_PASS_THROUGH_DIRECT: u32 = 315440u32;
pub const IOCTL_IDE_PASS_THROUGH: u32 = 315432u32;
pub const IOCTL_MINIPORT_PROCESS_SERVICE_IRP: u32 = 315448u32;
pub const IOCTL_MINIPORT_SIGNATURE_DSM_GENERAL: windows_core::PCSTR = windows_core::s!("MPDSMGEN");
pub const IOCTL_MINIPORT_SIGNATURE_DSM_NOTIFICATION: windows_core::PCSTR = windows_core::s!("MPDSM   ");
pub const IOCTL_MINIPORT_SIGNATURE_ENDURANCE_INFO: windows_core::PCSTR = windows_core::s!("ENDURINF");
pub const IOCTL_MINIPORT_SIGNATURE_FIRMWARE: windows_core::PCSTR = windows_core::s!("FIRMWARE");
pub const IOCTL_MINIPORT_SIGNATURE_HYBRDISK: windows_core::PCSTR = windows_core::s!("HYBRDISK");
pub const IOCTL_MINIPORT_SIGNATURE_QUERY_PHYSICAL_TOPOLOGY: windows_core::PCSTR = windows_core::s!("TOPOLOGY");
pub const IOCTL_MINIPORT_SIGNATURE_QUERY_PROTOCOL: windows_core::PCSTR = windows_core::s!("PROTOCOL");
pub const IOCTL_MINIPORT_SIGNATURE_QUERY_TEMPERATURE: windows_core::PCSTR = windows_core::s!("TEMPERAT");
pub const IOCTL_MINIPORT_SIGNATURE_SCSIDISK: windows_core::PCSTR = windows_core::s!("SCSIDISK");
pub const IOCTL_MINIPORT_SIGNATURE_SET_PROTOCOL: windows_core::PCSTR = windows_core::s!("SETPROTO");
pub const IOCTL_MINIPORT_SIGNATURE_SET_TEMPERATURE_THRESHOLD: windows_core::PCSTR = windows_core::s!("SETTEMPT");
pub const IOCTL_MPIO_PASS_THROUGH_PATH: u32 = 315452u32;
pub const IOCTL_MPIO_PASS_THROUGH_PATH_DIRECT: u32 = 315456u32;
pub const IOCTL_MPIO_PASS_THROUGH_PATH_DIRECT_EX: u32 = 315472u32;
pub const IOCTL_MPIO_PASS_THROUGH_PATH_EX: u32 = 315468u32;
pub const IOCTL_SCSI_BASE: u32 = 4u32;
pub const IOCTL_SCSI_FREE_DUMP_POINTERS: u32 = 266276u32;
pub const IOCTL_SCSI_GET_ADDRESS: u32 = 266264u32;
pub const IOCTL_SCSI_GET_CAPABILITIES: u32 = 266256u32;
pub const IOCTL_SCSI_GET_DUMP_POINTERS: u32 = 266272u32;
pub const IOCTL_SCSI_GET_INQUIRY_DATA: u32 = 266252u32;
pub const IOCTL_SCSI_MINIPORT: u32 = 315400u32;
pub const IOCTL_SCSI_PASS_THROUGH: u32 = 315396u32;
pub const IOCTL_SCSI_PASS_THROUGH_DIRECT: u32 = 315412u32;
pub const IOCTL_SCSI_PASS_THROUGH_DIRECT_EX: u32 = 315464u32;
pub const IOCTL_SCSI_PASS_THROUGH_EX: u32 = 315460u32;
pub const IOCTL_SCSI_RESCAN_BUS: u32 = 266268u32;
pub const ISCSI_CHAP_AUTH_TYPE: ISCSI_AUTH_TYPES = ISCSI_AUTH_TYPES(1i32);
pub const ISCSI_DIGEST_TYPE_CRC32C: ISCSI_DIGEST_TYPES = ISCSI_DIGEST_TYPES(1i32);
pub const ISCSI_DIGEST_TYPE_NONE: ISCSI_DIGEST_TYPES = ISCSI_DIGEST_TYPES(0i32);
pub const ISCSI_LOGIN_FLAG_ALLOW_PORTAL_HOPPING: u32 = 8u32;
pub const ISCSI_LOGIN_FLAG_MULTIPATH_ENABLED: u32 = 2u32;
pub const ISCSI_LOGIN_FLAG_REQUIRE_IPSEC: u32 = 1u32;
pub const ISCSI_LOGIN_FLAG_RESERVED1: u32 = 4u32;
pub const ISCSI_LOGIN_FLAG_USE_RADIUS_RESPONSE: u32 = 16u32;
pub const ISCSI_LOGIN_FLAG_USE_RADIUS_VERIFICATION: u32 = 32u32;
pub const ISCSI_LOGIN_OPTIONS_AUTH_TYPE: windows_core::PCSTR = windows_core::s!("0x00000080");
pub const ISCSI_LOGIN_OPTIONS_DATA_DIGEST: windows_core::PCSTR = windows_core::s!("0x00000002");
pub const ISCSI_LOGIN_OPTIONS_DEFAULT_TIME_2_RETAIN: windows_core::PCSTR = windows_core::s!("0x00000010");
pub const ISCSI_LOGIN_OPTIONS_DEFAULT_TIME_2_WAIT: windows_core::PCSTR = windows_core::s!("0x00000008");
pub const ISCSI_LOGIN_OPTIONS_HEADER_DIGEST: windows_core::PCSTR = windows_core::s!("0x00000001");
pub const ISCSI_LOGIN_OPTIONS_MAXIMUM_CONNECTIONS: windows_core::PCSTR = windows_core::s!("0x00000004");
pub const ISCSI_LOGIN_OPTIONS_PASSWORD: windows_core::PCSTR = windows_core::s!("0x00000040");
pub const ISCSI_LOGIN_OPTIONS_USERNAME: windows_core::PCSTR = windows_core::s!("0x00000020");
pub const ISCSI_LOGIN_OPTIONS_VERSION: u32 = 0u32;
pub const ISCSI_MUTUAL_CHAP_AUTH_TYPE: ISCSI_AUTH_TYPES = ISCSI_AUTH_TYPES(2i32);
pub const ISCSI_NO_AUTH_TYPE: ISCSI_AUTH_TYPES = ISCSI_AUTH_TYPES(0i32);
pub const ISCSI_SECURITY_FLAG_AGGRESSIVE_MODE_ENABLED: windows_core::PCSTR = windows_core::s!("0x00000008");
pub const ISCSI_SECURITY_FLAG_IKE_IPSEC_ENABLED: windows_core::PCSTR = windows_core::s!("0x00000002");
pub const ISCSI_SECURITY_FLAG_MAIN_MODE_ENABLED: windows_core::PCSTR = windows_core::s!("0x00000004");
pub const ISCSI_SECURITY_FLAG_PFS_ENABLED: windows_core::PCSTR = windows_core::s!("0x00000010");
pub const ISCSI_SECURITY_FLAG_TRANSPORT_MODE_PREFERRED: windows_core::PCSTR = windows_core::s!("0x00000020");
pub const ISCSI_SECURITY_FLAG_TUNNEL_MODE_PREFERRED: windows_core::PCSTR = windows_core::s!("0x00000040");
pub const ISCSI_SECURITY_FLAG_VALID: windows_core::PCSTR = windows_core::s!("0x00000001");
pub const ISCSI_TARGET_FLAG_HIDE_STATIC_TARGET: u32 = 2u32;
pub const ISCSI_TARGET_FLAG_MERGE_TARGET_INFORMATION: u32 = 4u32;
pub const ISCSI_TCP_PROTOCOL_TYPE: TARGETPROTOCOLTYPE = TARGETPROTOCOLTYPE(0i32);
pub const InitiatorName: TARGET_INFORMATION_CLASS = TARGET_INFORMATION_CLASS(5i32);
pub const LoginOptions: TARGET_INFORMATION_CLASS = TARGET_INFORMATION_CLASS(7i32);
pub const MAX_ISCSI_ALIAS_LEN: u32 = 255u32;
pub const MAX_ISCSI_DISCOVERY_DOMAIN_LEN: u32 = 256u32;
pub const MAX_ISCSI_HBANAME_LEN: u32 = 256u32;
pub const MAX_ISCSI_NAME_LEN: u32 = 223u32;
pub const MAX_ISCSI_PORTAL_ADDRESS_LEN: u32 = 256u32;
pub const MAX_ISCSI_PORTAL_ALIAS_LEN: u32 = 256u32;
pub const MAX_ISCSI_PORTAL_NAME_LEN: u32 = 256u32;
pub const MAX_ISCSI_TEXT_ADDRESS_LEN: u32 = 256u32;
pub const MAX_RADIUS_ADDRESS_LEN: u32 = 41u32;
pub const MINIPORT_DSM_NOTIFICATION_VERSION: u32 = 1u32;
pub const MINIPORT_DSM_NOTIFICATION_VERSION_1: u32 = 1u32;
pub const MINIPORT_DSM_NOTIFY_FLAG_BEGIN: u32 = 1u32;
pub const MINIPORT_DSM_NOTIFY_FLAG_END: u32 = 2u32;
pub const MINIPORT_DSM_PROFILE_CRASHDUMP_FILE: u32 = 3u32;
pub const MINIPORT_DSM_PROFILE_HIBERNATION_FILE: u32 = 2u32;
pub const MINIPORT_DSM_PROFILE_PAGE_FILE: u32 = 1u32;
pub const MINIPORT_DSM_PROFILE_UNKNOWN: u32 = 0u32;
pub const MPIO_IOCTL_FLAG_INVOLVE_DSM: u32 = 4u32;
pub const MPIO_IOCTL_FLAG_USE_PATHID: u32 = 1u32;
pub const MPIO_IOCTL_FLAG_USE_SCSIADDRESS: u32 = 2u32;
pub const MpStorageDiagnosticLevelDefault: MP_STORAGE_DIAGNOSTIC_LEVEL = MP_STORAGE_DIAGNOSTIC_LEVEL(0i32);
pub const MpStorageDiagnosticLevelMax: MP_STORAGE_DIAGNOSTIC_LEVEL = MP_STORAGE_DIAGNOSTIC_LEVEL(1i32);
pub const MpStorageDiagnosticTargetTypeHbaFirmware: MP_STORAGE_DIAGNOSTIC_TARGET_TYPE = MP_STORAGE_DIAGNOSTIC_TARGET_TYPE(3i32);
pub const MpStorageDiagnosticTargetTypeMax: MP_STORAGE_DIAGNOSTIC_TARGET_TYPE = MP_STORAGE_DIAGNOSTIC_TARGET_TYPE(4i32);
pub const MpStorageDiagnosticTargetTypeMiniport: MP_STORAGE_DIAGNOSTIC_TARGET_TYPE = MP_STORAGE_DIAGNOSTIC_TARGET_TYPE(2i32);
pub const MpStorageDiagnosticTargetTypeUndefined: MP_STORAGE_DIAGNOSTIC_TARGET_TYPE = MP_STORAGE_DIAGNOSTIC_TARGET_TYPE(0i32);
pub const NRB_FUNCTION_ADD_LBAS_PINNED_SET: u32 = 16u32;
pub const NRB_FUNCTION_FLUSH_NVCACHE: u32 = 20u32;
pub const NRB_FUNCTION_NVCACHE_INFO: u32 = 236u32;
pub const NRB_FUNCTION_NVCACHE_POWER_MODE_RETURN: u32 = 1u32;
pub const NRB_FUNCTION_NVCACHE_POWER_MODE_SET: u32 = 0u32;
pub const NRB_FUNCTION_NVSEPARATED_FLUSH: u32 = 193u32;
pub const NRB_FUNCTION_NVSEPARATED_INFO: u32 = 192u32;
pub const NRB_FUNCTION_NVSEPARATED_WB_DISABLE: u32 = 194u32;
pub const NRB_FUNCTION_NVSEPARATED_WB_REVERT_DEFAULT: u32 = 195u32;
pub const NRB_FUNCTION_PASS_HINT_PAYLOAD: u32 = 224u32;
pub const NRB_FUNCTION_QUERY_ASCENDER_STATUS: u32 = 208u32;
pub const NRB_FUNCTION_QUERY_CACHE_MISS: u32 = 19u32;
pub const NRB_FUNCTION_QUERY_HYBRID_DISK_STATUS: u32 = 209u32;
pub const NRB_FUNCTION_QUERY_PINNED_SET: u32 = 18u32;
pub const NRB_FUNCTION_REMOVE_LBAS_PINNED_SET: u32 = 17u32;
pub const NRB_FUNCTION_SPINDLE_STATUS: u32 = 229u32;
pub const NRB_ILLEGAL_REQUEST: u32 = 1u32;
pub const NRB_INPUT_DATA_OVERRUN: u32 = 3u32;
pub const NRB_INPUT_DATA_UNDERRUN: u32 = 4u32;
pub const NRB_INVALID_PARAMETER: u32 = 2u32;
pub const NRB_OUTPUT_DATA_OVERRUN: u32 = 5u32;
pub const NRB_OUTPUT_DATA_UNDERRUN: u32 = 6u32;
pub const NRB_SUCCESS: u32 = 0u32;
pub const NVSEPWriteCacheTypeNone: NV_SEP_WRITE_CACHE_TYPE = NV_SEP_WRITE_CACHE_TYPE(1i32);
pub const NVSEPWriteCacheTypeUnknown: NV_SEP_WRITE_CACHE_TYPE = NV_SEP_WRITE_CACHE_TYPE(0i32);
pub const NVSEPWriteCacheTypeWriteBack: NV_SEP_WRITE_CACHE_TYPE = NV_SEP_WRITE_CACHE_TYPE(2i32);
pub const NVSEPWriteCacheTypeWriteThrough: NV_SEP_WRITE_CACHE_TYPE = NV_SEP_WRITE_CACHE_TYPE(3i32);
pub const NV_SEP_CACHE_PARAMETER_VERSION: u32 = 1u32;
pub const NV_SEP_CACHE_PARAMETER_VERSION_1: u32 = 1u32;
pub const NvCacheStatusDisabled: NVCACHE_STATUS = NVCACHE_STATUS(2i32);
pub const NvCacheStatusDisabling: NVCACHE_STATUS = NVCACHE_STATUS(1i32);
pub const NvCacheStatusEnabled: NVCACHE_STATUS = NVCACHE_STATUS(3i32);
pub const NvCacheStatusUnknown: NVCACHE_STATUS = NVCACHE_STATUS(0i32);
pub const NvCacheTypeNone: NVCACHE_TYPE = NVCACHE_TYPE(1i32);
pub const NvCacheTypeUnknown: NVCACHE_TYPE = NVCACHE_TYPE(0i32);
pub const NvCacheTypeWriteBack: NVCACHE_TYPE = NVCACHE_TYPE(2i32);
pub const NvCacheTypeWriteThrough: NVCACHE_TYPE = NVCACHE_TYPE(3i32);
pub const PersistentTargetMappings: TARGET_INFORMATION_CLASS = TARGET_INFORMATION_CLASS(4i32);
pub const PortalGroups: TARGET_INFORMATION_CLASS = TARGET_INFORMATION_CLASS(3i32);
pub const ProtocolType: TARGET_INFORMATION_CLASS = TARGET_INFORMATION_CLASS(0i32);
pub const SCSI_IOCTL_DATA_BIDIRECTIONAL: u32 = 3u32;
pub const SCSI_IOCTL_DATA_IN: u32 = 1u32;
pub const SCSI_IOCTL_DATA_OUT: u32 = 0u32;
pub const SCSI_IOCTL_DATA_UNSPECIFIED: u32 = 2u32;
pub const STORAGE_DIAGNOSTIC_STATUS_BUFFER_TOO_SMALL: u32 = 1u32;
pub const STORAGE_DIAGNOSTIC_STATUS_INVALID_PARAMETER: u32 = 3u32;
pub const STORAGE_DIAGNOSTIC_STATUS_INVALID_SIGNATURE: u32 = 4u32;
pub const STORAGE_DIAGNOSTIC_STATUS_INVALID_TARGET_TYPE: u32 = 5u32;
pub const STORAGE_DIAGNOSTIC_STATUS_MORE_DATA: u32 = 6u32;
pub const STORAGE_DIAGNOSTIC_STATUS_SUCCESS: u32 = 0u32;
pub const STORAGE_DIAGNOSTIC_STATUS_UNSUPPORTED_VERSION: u32 = 2u32;
pub const STORAGE_FIRMWARE_ACTIVATE_STRUCTURE_VERSION: u32 = 1u32;
pub const STORAGE_FIRMWARE_DOWNLOAD_STRUCTURE_VERSION: u32 = 1u32;
pub const STORAGE_FIRMWARE_DOWNLOAD_STRUCTURE_VERSION_V2: u32 = 2u32;
pub const STORAGE_FIRMWARE_INFO_INVALID_SLOT: u32 = 255u32;
pub const STORAGE_FIRMWARE_INFO_STRUCTURE_VERSION: u32 = 1u32;
pub const STORAGE_FIRMWARE_INFO_STRUCTURE_VERSION_V2: u32 = 2u32;
pub const STORAGE_FIRMWARE_SLOT_INFO_V2_REVISION_LENGTH: u32 = 16u32;
pub const ScsiRawInterfaceGuid: windows_core::GUID = windows_core::GUID::from_u128(0x53f56309_b6bf_11d0_94f2_00a0c91efb8b);
pub const TargetAlias: TARGET_INFORMATION_CLASS = TARGET_INFORMATION_CLASS(1i32);
pub const TargetFlags: TARGET_INFORMATION_CLASS = TARGET_INFORMATION_CLASS(6i32);
pub const WmiScsiAddressGuid: windows_core::GUID = windows_core::GUID::from_u128(0x53f5630f_b6bf_11d0_94f2_00a0c91efb8b);
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct IKE_AUTHENTICATION_METHOD(pub i32);
impl windows_core::TypeKind for IKE_AUTHENTICATION_METHOD {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for IKE_AUTHENTICATION_METHOD {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("IKE_AUTHENTICATION_METHOD").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct ISCSI_AUTH_TYPES(pub i32);
impl windows_core::TypeKind for ISCSI_AUTH_TYPES {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for ISCSI_AUTH_TYPES {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("ISCSI_AUTH_TYPES").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct ISCSI_DIGEST_TYPES(pub i32);
impl windows_core::TypeKind for ISCSI_DIGEST_TYPES {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for ISCSI_DIGEST_TYPES {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("ISCSI_DIGEST_TYPES").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct MP_STORAGE_DIAGNOSTIC_LEVEL(pub i32);
impl windows_core::TypeKind for MP_STORAGE_DIAGNOSTIC_LEVEL {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for MP_STORAGE_DIAGNOSTIC_LEVEL {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("MP_STORAGE_DIAGNOSTIC_LEVEL").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct MP_STORAGE_DIAGNOSTIC_TARGET_TYPE(pub i32);
impl windows_core::TypeKind for MP_STORAGE_DIAGNOSTIC_TARGET_TYPE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for MP_STORAGE_DIAGNOSTIC_TARGET_TYPE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("MP_STORAGE_DIAGNOSTIC_TARGET_TYPE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct NVCACHE_STATUS(pub i32);
impl windows_core::TypeKind for NVCACHE_STATUS {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for NVCACHE_STATUS {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("NVCACHE_STATUS").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct NVCACHE_TYPE(pub i32);
impl windows_core::TypeKind for NVCACHE_TYPE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for NVCACHE_TYPE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("NVCACHE_TYPE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct NV_SEP_WRITE_CACHE_TYPE(pub i32);
impl windows_core::TypeKind for NV_SEP_WRITE_CACHE_TYPE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for NV_SEP_WRITE_CACHE_TYPE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("NV_SEP_WRITE_CACHE_TYPE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct TARGETPROTOCOLTYPE(pub i32);
impl windows_core::TypeKind for TARGETPROTOCOLTYPE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for TARGETPROTOCOLTYPE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("TARGETPROTOCOLTYPE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct TARGET_INFORMATION_CLASS(pub i32);
impl windows_core::TypeKind for TARGET_INFORMATION_CLASS {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for TARGET_INFORMATION_CLASS {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("TARGET_INFORMATION_CLASS").field(&self.0).finish()
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ATA_PASS_THROUGH_DIRECT {
    pub Length: u16,
    pub AtaFlags: u16,
    pub PathId: u8,
    pub TargetId: u8,
    pub Lun: u8,
    pub ReservedAsUchar: u8,
    pub DataTransferLength: u32,
    pub TimeOutValue: u32,
    pub ReservedAsUlong: u32,
    pub DataBuffer: *mut core::ffi::c_void,
    pub PreviousTaskFile: [u8; 8],
    pub CurrentTaskFile: [u8; 8],
}
impl windows_core::TypeKind for ATA_PASS_THROUGH_DIRECT {
    type TypeKind = windows_core::CopyType;
}
impl Default for ATA_PASS_THROUGH_DIRECT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ATA_PASS_THROUGH_DIRECT32 {
    pub Length: u16,
    pub AtaFlags: u16,
    pub PathId: u8,
    pub TargetId: u8,
    pub Lun: u8,
    pub ReservedAsUchar: u8,
    pub DataTransferLength: u32,
    pub TimeOutValue: u32,
    pub ReservedAsUlong: u32,
    pub DataBuffer: *mut core::ffi::c_void,
    pub PreviousTaskFile: [u8; 8],
    pub CurrentTaskFile: [u8; 8],
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
impl windows_core::TypeKind for ATA_PASS_THROUGH_DIRECT32 {
    type TypeKind = windows_core::CopyType;
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
impl Default for ATA_PASS_THROUGH_DIRECT32 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ATA_PASS_THROUGH_EX {
    pub Length: u16,
    pub AtaFlags: u16,
    pub PathId: u8,
    pub TargetId: u8,
    pub Lun: u8,
    pub ReservedAsUchar: u8,
    pub DataTransferLength: u32,
    pub TimeOutValue: u32,
    pub ReservedAsUlong: u32,
    pub DataBufferOffset: usize,
    pub PreviousTaskFile: [u8; 8],
    pub CurrentTaskFile: [u8; 8],
}
impl windows_core::TypeKind for ATA_PASS_THROUGH_EX {
    type TypeKind = windows_core::CopyType;
}
impl Default for ATA_PASS_THROUGH_EX {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ATA_PASS_THROUGH_EX32 {
    pub Length: u16,
    pub AtaFlags: u16,
    pub PathId: u8,
    pub TargetId: u8,
    pub Lun: u8,
    pub ReservedAsUchar: u8,
    pub DataTransferLength: u32,
    pub TimeOutValue: u32,
    pub ReservedAsUlong: u32,
    pub DataBufferOffset: u32,
    pub PreviousTaskFile: [u8; 8],
    pub CurrentTaskFile: [u8; 8],
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
impl windows_core::TypeKind for ATA_PASS_THROUGH_EX32 {
    type TypeKind = windows_core::CopyType;
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
impl Default for ATA_PASS_THROUGH_EX32 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DSM_NOTIFICATION_REQUEST_BLOCK {
    pub Size: u32,
    pub Version: u32,
    pub NotifyFlags: u32,
    pub DataSetProfile: u32,
    pub Reserved: [u32; 3],
    pub DataSetRangesCount: u32,
    pub DataSetRanges: [MP_DEVICE_DATA_SET_RANGE; 1],
}
impl windows_core::TypeKind for DSM_NOTIFICATION_REQUEST_BLOCK {
    type TypeKind = windows_core::CopyType;
}
impl Default for DSM_NOTIFICATION_REQUEST_BLOCK {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DUMP_DRIVER {
    pub DumpDriverList: *mut core::ffi::c_void,
    pub DriverName: [u16; 15],
    pub BaseName: [u16; 15],
}
impl windows_core::TypeKind for DUMP_DRIVER {
    type TypeKind = windows_core::CopyType;
}
impl Default for DUMP_DRIVER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DUMP_DRIVER_EX {
    pub DumpDriverList: *mut core::ffi::c_void,
    pub DriverName: [u16; 15],
    pub BaseName: [u16; 15],
    pub DriverFullPath: NTSCSI_UNICODE_STRING,
}
impl windows_core::TypeKind for DUMP_DRIVER_EX {
    type TypeKind = windows_core::CopyType;
}
impl Default for DUMP_DRIVER_EX {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DUMP_POINTERS {
    pub AdapterObject: *mut _ADAPTER_OBJECT,
    pub MappedRegisterBase: *mut core::ffi::c_void,
    pub DumpData: *mut core::ffi::c_void,
    pub CommonBufferVa: *mut core::ffi::c_void,
    pub CommonBufferPa: i64,
    pub CommonBufferSize: u32,
    pub AllocateCommonBuffers: super::super::Foundation::BOOLEAN,
    pub UseDiskDump: super::super::Foundation::BOOLEAN,
    pub Spare1: [u8; 2],
    pub DeviceObject: *mut core::ffi::c_void,
}
impl windows_core::TypeKind for DUMP_POINTERS {
    type TypeKind = windows_core::CopyType;
}
impl Default for DUMP_POINTERS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct DUMP_POINTERS_EX {
    pub Header: DUMP_POINTERS_VERSION,
    pub DumpData: *mut core::ffi::c_void,
    pub CommonBufferVa: *mut core::ffi::c_void,
    pub CommonBufferSize: u32,
    pub AllocateCommonBuffers: super::super::Foundation::BOOLEAN,
    pub DeviceObject: *mut core::ffi::c_void,
    pub DriverList: *mut core::ffi::c_void,
    pub dwPortFlags: u32,
    pub MaxDeviceDumpSectionSize: u32,
    pub MaxDeviceDumpLevel: u32,
    pub MaxTransferSize: u32,
    pub AdapterObject: *mut core::ffi::c_void,
    pub MappedRegisterBase: *mut core::ffi::c_void,
    pub DeviceReady: *mut super::super::Foundation::BOOLEAN,
    pub DumpDevicePowerOn: PDUMP_DEVICE_POWERON_ROUTINE,
    pub DumpDevicePowerOnContext: *mut core::ffi::c_void,
}
impl windows_core::TypeKind for DUMP_POINTERS_EX {
    type TypeKind = windows_core::CopyType;
}
impl Default for DUMP_POINTERS_EX {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DUMP_POINTERS_VERSION {
    pub Version: u32,
    pub Size: u32,
}
impl windows_core::TypeKind for DUMP_POINTERS_VERSION {
    type TypeKind = windows_core::CopyType;
}
impl Default for DUMP_POINTERS_VERSION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FIRMWARE_REQUEST_BLOCK {
    pub Version: u32,
    pub Size: u32,
    pub Function: u32,
    pub Flags: u32,
    pub DataBufferOffset: u32,
    pub DataBufferLength: u32,
}
impl windows_core::TypeKind for FIRMWARE_REQUEST_BLOCK {
    type TypeKind = windows_core::CopyType;
}
impl Default for FIRMWARE_REQUEST_BLOCK {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct HYBRID_DEMOTE_BY_SIZE {
    pub Version: u32,
    pub Size: u32,
    pub SourcePriority: u8,
    pub TargetPriority: u8,
    pub Reserved0: u16,
    pub Reserved1: u32,
    pub LbaCount: u64,
}
impl windows_core::TypeKind for HYBRID_DEMOTE_BY_SIZE {
    type TypeKind = windows_core::CopyType;
}
impl Default for HYBRID_DEMOTE_BY_SIZE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct HYBRID_DIRTY_THRESHOLDS {
    pub Version: u32,
    pub Size: u32,
    pub DirtyLowThreshold: u32,
    pub DirtyHighThreshold: u32,
}
impl windows_core::TypeKind for HYBRID_DIRTY_THRESHOLDS {
    type TypeKind = windows_core::CopyType;
}
impl Default for HYBRID_DIRTY_THRESHOLDS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct HYBRID_INFORMATION {
    pub Version: u32,
    pub Size: u32,
    pub HybridSupported: super::super::Foundation::BOOLEAN,
    pub Status: NVCACHE_STATUS,
    pub CacheTypeEffective: NVCACHE_TYPE,
    pub CacheTypeDefault: NVCACHE_TYPE,
    pub FractionBase: u32,
    pub CacheSize: u64,
    pub Attributes: HYBRID_INFORMATION_0,
    pub Priorities: HYBRID_INFORMATION_1,
}
impl windows_core::TypeKind for HYBRID_INFORMATION {
    type TypeKind = windows_core::CopyType;
}
impl Default for HYBRID_INFORMATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct HYBRID_INFORMATION_0 {
    pub _bitfield: u32,
}
impl windows_core::TypeKind for HYBRID_INFORMATION_0 {
    type TypeKind = windows_core::CopyType;
}
impl Default for HYBRID_INFORMATION_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct HYBRID_INFORMATION_1 {
    pub PriorityLevelCount: u8,
    pub MaxPriorityBehavior: super::super::Foundation::BOOLEAN,
    pub OptimalWriteGranularity: u8,
    pub Reserved: u8,
    pub DirtyThresholdLow: u32,
    pub DirtyThresholdHigh: u32,
    pub SupportedCommands: HYBRID_INFORMATION_1_0,
    pub Priority: [NVCACHE_PRIORITY_LEVEL_DESCRIPTOR; 1],
}
impl windows_core::TypeKind for HYBRID_INFORMATION_1 {
    type TypeKind = windows_core::CopyType;
}
impl Default for HYBRID_INFORMATION_1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct HYBRID_INFORMATION_1_0 {
    pub _bitfield: u32,
    pub MaxEvictCommands: u32,
    pub MaxLbaRangeCountForEvict: u32,
    pub MaxLbaRangeCountForChangeLba: u32,
}
impl windows_core::TypeKind for HYBRID_INFORMATION_1_0 {
    type TypeKind = windows_core::CopyType;
}
impl Default for HYBRID_INFORMATION_1_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct HYBRID_REQUEST_BLOCK {
    pub Version: u32,
    pub Size: u32,
    pub Function: u32,
    pub Flags: u32,
    pub DataBufferOffset: u32,
    pub DataBufferLength: u32,
}
impl windows_core::TypeKind for HYBRID_REQUEST_BLOCK {
    type TypeKind = windows_core::CopyType;
}
impl Default for HYBRID_REQUEST_BLOCK {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct IDE_IO_CONTROL {
    pub HeaderLength: u32,
    pub Signature: [u8; 8],
    pub Timeout: u32,
    pub ControlCode: u32,
    pub ReturnStatus: u32,
    pub DataLength: u32,
}
impl windows_core::TypeKind for IDE_IO_CONTROL {
    type TypeKind = windows_core::CopyType;
}
impl Default for IDE_IO_CONTROL {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct IKE_AUTHENTICATION_INFORMATION {
    pub AuthMethod: IKE_AUTHENTICATION_METHOD,
    pub Anonymous: IKE_AUTHENTICATION_INFORMATION_0,
}
impl windows_core::TypeKind for IKE_AUTHENTICATION_INFORMATION {
    type TypeKind = windows_core::CopyType;
}
impl Default for IKE_AUTHENTICATION_INFORMATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union IKE_AUTHENTICATION_INFORMATION_0 {
    pub PsKey: IKE_AUTHENTICATION_PRESHARED_KEY,
}
impl windows_core::TypeKind for IKE_AUTHENTICATION_INFORMATION_0 {
    type TypeKind = windows_core::CopyType;
}
impl Default for IKE_AUTHENTICATION_INFORMATION_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct IKE_AUTHENTICATION_PRESHARED_KEY {
    pub SecurityFlags: u64,
    pub IdType: u8,
    pub IdLengthInBytes: u32,
    pub Id: *mut u8,
    pub KeyLengthInBytes: u32,
    pub Key: *mut u8,
}
impl windows_core::TypeKind for IKE_AUTHENTICATION_PRESHARED_KEY {
    type TypeKind = windows_core::CopyType;
}
impl Default for IKE_AUTHENTICATION_PRESHARED_KEY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct IO_SCSI_CAPABILITIES {
    pub Length: u32,
    pub MaximumTransferLength: u32,
    pub MaximumPhysicalPages: u32,
    pub SupportedAsynchronousEvents: u32,
    pub AlignmentMask: u32,
    pub TaggedQueuing: super::super::Foundation::BOOLEAN,
    pub AdapterScansDown: super::super::Foundation::BOOLEAN,
    pub AdapterUsesPio: super::super::Foundation::BOOLEAN,
}
impl windows_core::TypeKind for IO_SCSI_CAPABILITIES {
    type TypeKind = windows_core::CopyType;
}
impl Default for IO_SCSI_CAPABILITIES {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ISCSI_CONNECTION_INFOA {
    pub ConnectionId: ISCSI_UNIQUE_SESSION_ID,
    pub InitiatorAddress: windows_core::PSTR,
    pub TargetAddress: windows_core::PSTR,
    pub InitiatorSocket: u16,
    pub TargetSocket: u16,
    pub CID: [u8; 2],
}
impl windows_core::TypeKind for ISCSI_CONNECTION_INFOA {
    type TypeKind = windows_core::CopyType;
}
impl Default for ISCSI_CONNECTION_INFOA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ISCSI_CONNECTION_INFOW {
    pub ConnectionId: ISCSI_UNIQUE_SESSION_ID,
    pub InitiatorAddress: windows_core::PWSTR,
    pub TargetAddress: windows_core::PWSTR,
    pub InitiatorSocket: u16,
    pub TargetSocket: u16,
    pub CID: [u8; 2],
}
impl windows_core::TypeKind for ISCSI_CONNECTION_INFOW {
    type TypeKind = windows_core::CopyType;
}
impl Default for ISCSI_CONNECTION_INFOW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ISCSI_CONNECTION_INFO_EX {
    pub ConnectionId: ISCSI_UNIQUE_SESSION_ID,
    pub State: u8,
    pub Protocol: u8,
    pub HeaderDigest: u8,
    pub DataDigest: u8,
    pub MaxRecvDataSegmentLength: u32,
    pub AuthType: ISCSI_AUTH_TYPES,
    pub EstimatedThroughput: u64,
    pub MaxDatagramSize: u32,
}
impl windows_core::TypeKind for ISCSI_CONNECTION_INFO_EX {
    type TypeKind = windows_core::CopyType;
}
impl Default for ISCSI_CONNECTION_INFO_EX {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_System_Ioctl")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ISCSI_DEVICE_ON_SESSIONA {
    pub InitiatorName: [i8; 256],
    pub TargetName: [i8; 224],
    pub ScsiAddress: SCSI_ADDRESS,
    pub DeviceInterfaceType: windows_core::GUID,
    pub DeviceInterfaceName: [i8; 260],
    pub LegacyName: [i8; 260],
    pub StorageDeviceNumber: super::super::System::Ioctl::STORAGE_DEVICE_NUMBER,
    pub DeviceInstance: u32,
}
#[cfg(feature = "Win32_System_Ioctl")]
impl windows_core::TypeKind for ISCSI_DEVICE_ON_SESSIONA {
    type TypeKind = windows_core::CopyType;
}
#[cfg(feature = "Win32_System_Ioctl")]
impl Default for ISCSI_DEVICE_ON_SESSIONA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_System_Ioctl")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ISCSI_DEVICE_ON_SESSIONW {
    pub InitiatorName: [u16; 256],
    pub TargetName: [u16; 224],
    pub ScsiAddress: SCSI_ADDRESS,
    pub DeviceInterfaceType: windows_core::GUID,
    pub DeviceInterfaceName: [u16; 260],
    pub LegacyName: [u16; 260],
    pub StorageDeviceNumber: super::super::System::Ioctl::STORAGE_DEVICE_NUMBER,
    pub DeviceInstance: u32,
}
#[cfg(feature = "Win32_System_Ioctl")]
impl windows_core::TypeKind for ISCSI_DEVICE_ON_SESSIONW {
    type TypeKind = windows_core::CopyType;
}
#[cfg(feature = "Win32_System_Ioctl")]
impl Default for ISCSI_DEVICE_ON_SESSIONW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ISCSI_LOGIN_OPTIONS {
    pub Version: u32,
    pub InformationSpecified: u32,
    pub LoginFlags: u32,
    pub AuthType: ISCSI_AUTH_TYPES,
    pub HeaderDigest: ISCSI_DIGEST_TYPES,
    pub DataDigest: ISCSI_DIGEST_TYPES,
    pub MaximumConnections: u32,
    pub DefaultTime2Wait: u32,
    pub DefaultTime2Retain: u32,
    pub UsernameLength: u32,
    pub PasswordLength: u32,
    pub Username: *mut u8,
    pub Password: *mut u8,
}
impl windows_core::TypeKind for ISCSI_LOGIN_OPTIONS {
    type TypeKind = windows_core::CopyType;
}
impl Default for ISCSI_LOGIN_OPTIONS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ISCSI_SESSION_INFOA {
    pub SessionId: ISCSI_UNIQUE_SESSION_ID,
    pub InitiatorName: windows_core::PSTR,
    pub TargetNodeName: windows_core::PSTR,
    pub TargetName: windows_core::PSTR,
    pub ISID: [u8; 6],
    pub TSID: [u8; 2],
    pub ConnectionCount: u32,
    pub Connections: *mut ISCSI_CONNECTION_INFOA,
}
impl windows_core::TypeKind for ISCSI_SESSION_INFOA {
    type TypeKind = windows_core::CopyType;
}
impl Default for ISCSI_SESSION_INFOA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ISCSI_SESSION_INFOW {
    pub SessionId: ISCSI_UNIQUE_SESSION_ID,
    pub InitiatorName: windows_core::PWSTR,
    pub TargetNodeName: windows_core::PWSTR,
    pub TargetName: windows_core::PWSTR,
    pub ISID: [u8; 6],
    pub TSID: [u8; 2],
    pub ConnectionCount: u32,
    pub Connections: *mut ISCSI_CONNECTION_INFOW,
}
impl windows_core::TypeKind for ISCSI_SESSION_INFOW {
    type TypeKind = windows_core::CopyType;
}
impl Default for ISCSI_SESSION_INFOW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ISCSI_SESSION_INFO_EX {
    pub SessionId: ISCSI_UNIQUE_SESSION_ID,
    pub InitialR2t: super::super::Foundation::BOOLEAN,
    pub ImmediateData: super::super::Foundation::BOOLEAN,
    pub Type: u8,
    pub DataSequenceInOrder: super::super::Foundation::BOOLEAN,
    pub DataPduInOrder: super::super::Foundation::BOOLEAN,
    pub ErrorRecoveryLevel: u8,
    pub MaxOutstandingR2t: u32,
    pub FirstBurstLength: u32,
    pub MaxBurstLength: u32,
    pub MaximumConnections: u32,
    pub ConnectionCount: u32,
    pub Connections: *mut ISCSI_CONNECTION_INFO_EX,
}
impl windows_core::TypeKind for ISCSI_SESSION_INFO_EX {
    type TypeKind = windows_core::CopyType;
}
impl Default for ISCSI_SESSION_INFO_EX {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ISCSI_TARGET_MAPPINGA {
    pub InitiatorName: [i8; 256],
    pub TargetName: [i8; 224],
    pub OSDeviceName: [i8; 260],
    pub SessionId: ISCSI_UNIQUE_SESSION_ID,
    pub OSBusNumber: u32,
    pub OSTargetNumber: u32,
    pub LUNCount: u32,
    pub LUNList: *mut SCSI_LUN_LIST,
}
impl windows_core::TypeKind for ISCSI_TARGET_MAPPINGA {
    type TypeKind = windows_core::CopyType;
}
impl Default for ISCSI_TARGET_MAPPINGA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ISCSI_TARGET_MAPPINGW {
    pub InitiatorName: [u16; 256],
    pub TargetName: [u16; 224],
    pub OSDeviceName: [u16; 260],
    pub SessionId: ISCSI_UNIQUE_SESSION_ID,
    pub OSBusNumber: u32,
    pub OSTargetNumber: u32,
    pub LUNCount: u32,
    pub LUNList: *mut SCSI_LUN_LIST,
}
impl windows_core::TypeKind for ISCSI_TARGET_MAPPINGW {
    type TypeKind = windows_core::CopyType;
}
impl Default for ISCSI_TARGET_MAPPINGW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ISCSI_TARGET_PORTALA {
    pub SymbolicName: [i8; 256],
    pub Address: [i8; 256],
    pub Socket: u16,
}
impl windows_core::TypeKind for ISCSI_TARGET_PORTALA {
    type TypeKind = windows_core::CopyType;
}
impl Default for ISCSI_TARGET_PORTALA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ISCSI_TARGET_PORTALW {
    pub SymbolicName: [u16; 256],
    pub Address: [u16; 256],
    pub Socket: u16,
}
impl windows_core::TypeKind for ISCSI_TARGET_PORTALW {
    type TypeKind = windows_core::CopyType;
}
impl Default for ISCSI_TARGET_PORTALW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ISCSI_TARGET_PORTAL_GROUPA {
    pub Count: u32,
    pub Portals: [ISCSI_TARGET_PORTALA; 1],
}
impl windows_core::TypeKind for ISCSI_TARGET_PORTAL_GROUPA {
    type TypeKind = windows_core::CopyType;
}
impl Default for ISCSI_TARGET_PORTAL_GROUPA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ISCSI_TARGET_PORTAL_GROUPW {
    pub Count: u32,
    pub Portals: [ISCSI_TARGET_PORTALW; 1],
}
impl windows_core::TypeKind for ISCSI_TARGET_PORTAL_GROUPW {
    type TypeKind = windows_core::CopyType;
}
impl Default for ISCSI_TARGET_PORTAL_GROUPW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ISCSI_TARGET_PORTAL_INFOA {
    pub InitiatorName: [i8; 256],
    pub InitiatorPortNumber: u32,
    pub SymbolicName: [i8; 256],
    pub Address: [i8; 256],
    pub Socket: u16,
}
impl windows_core::TypeKind for ISCSI_TARGET_PORTAL_INFOA {
    type TypeKind = windows_core::CopyType;
}
impl Default for ISCSI_TARGET_PORTAL_INFOA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ISCSI_TARGET_PORTAL_INFOW {
    pub InitiatorName: [u16; 256],
    pub InitiatorPortNumber: u32,
    pub SymbolicName: [u16; 256],
    pub Address: [u16; 256],
    pub Socket: u16,
}
impl windows_core::TypeKind for ISCSI_TARGET_PORTAL_INFOW {
    type TypeKind = windows_core::CopyType;
}
impl Default for ISCSI_TARGET_PORTAL_INFOW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ISCSI_TARGET_PORTAL_INFO_EXA {
    pub InitiatorName: [i8; 256],
    pub InitiatorPortNumber: u32,
    pub SymbolicName: [i8; 256],
    pub Address: [i8; 256],
    pub Socket: u16,
    pub SecurityFlags: u64,
    pub LoginOptions: ISCSI_LOGIN_OPTIONS,
}
impl windows_core::TypeKind for ISCSI_TARGET_PORTAL_INFO_EXA {
    type TypeKind = windows_core::CopyType;
}
impl Default for ISCSI_TARGET_PORTAL_INFO_EXA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ISCSI_TARGET_PORTAL_INFO_EXW {
    pub InitiatorName: [u16; 256],
    pub InitiatorPortNumber: u32,
    pub SymbolicName: [u16; 256],
    pub Address: [u16; 256],
    pub Socket: u16,
    pub SecurityFlags: u64,
    pub LoginOptions: ISCSI_LOGIN_OPTIONS,
}
impl windows_core::TypeKind for ISCSI_TARGET_PORTAL_INFO_EXW {
    type TypeKind = windows_core::CopyType;
}
impl Default for ISCSI_TARGET_PORTAL_INFO_EXW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ISCSI_UNIQUE_SESSION_ID {
    pub AdapterUnique: u64,
    pub AdapterSpecific: u64,
}
impl windows_core::TypeKind for ISCSI_UNIQUE_SESSION_ID {
    type TypeKind = windows_core::CopyType;
}
impl Default for ISCSI_UNIQUE_SESSION_ID {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ISCSI_VERSION_INFO {
    pub MajorVersion: u32,
    pub MinorVersion: u32,
    pub BuildNumber: u32,
}
impl windows_core::TypeKind for ISCSI_VERSION_INFO {
    type TypeKind = windows_core::CopyType;
}
impl Default for ISCSI_VERSION_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct MPIO_PASS_THROUGH_PATH {
    pub PassThrough: SCSI_PASS_THROUGH,
    pub Version: u32,
    pub Length: u16,
    pub Flags: u8,
    pub PortNumber: u8,
    pub MpioPathId: u64,
}
impl windows_core::TypeKind for MPIO_PASS_THROUGH_PATH {
    type TypeKind = windows_core::CopyType;
}
impl Default for MPIO_PASS_THROUGH_PATH {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct MPIO_PASS_THROUGH_PATH32 {
    pub PassThrough: SCSI_PASS_THROUGH32,
    pub Version: u32,
    pub Length: u16,
    pub Flags: u8,
    pub PortNumber: u8,
    pub MpioPathId: u64,
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
impl windows_core::TypeKind for MPIO_PASS_THROUGH_PATH32 {
    type TypeKind = windows_core::CopyType;
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
impl Default for MPIO_PASS_THROUGH_PATH32 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct MPIO_PASS_THROUGH_PATH32_EX {
    pub PassThroughOffset: u32,
    pub Version: u32,
    pub Length: u16,
    pub Flags: u8,
    pub PortNumber: u8,
    pub MpioPathId: u64,
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
impl windows_core::TypeKind for MPIO_PASS_THROUGH_PATH32_EX {
    type TypeKind = windows_core::CopyType;
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
impl Default for MPIO_PASS_THROUGH_PATH32_EX {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct MPIO_PASS_THROUGH_PATH_DIRECT {
    pub PassThrough: SCSI_PASS_THROUGH_DIRECT,
    pub Version: u32,
    pub Length: u16,
    pub Flags: u8,
    pub PortNumber: u8,
    pub MpioPathId: u64,
}
impl windows_core::TypeKind for MPIO_PASS_THROUGH_PATH_DIRECT {
    type TypeKind = windows_core::CopyType;
}
impl Default for MPIO_PASS_THROUGH_PATH_DIRECT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct MPIO_PASS_THROUGH_PATH_DIRECT32 {
    pub PassThrough: SCSI_PASS_THROUGH_DIRECT32,
    pub Version: u32,
    pub Length: u16,
    pub Flags: u8,
    pub PortNumber: u8,
    pub MpioPathId: u64,
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
impl windows_core::TypeKind for MPIO_PASS_THROUGH_PATH_DIRECT32 {
    type TypeKind = windows_core::CopyType;
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
impl Default for MPIO_PASS_THROUGH_PATH_DIRECT32 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct MPIO_PASS_THROUGH_PATH_DIRECT32_EX {
    pub PassThroughOffset: u32,
    pub Version: u32,
    pub Length: u16,
    pub Flags: u8,
    pub PortNumber: u8,
    pub MpioPathId: u64,
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
impl windows_core::TypeKind for MPIO_PASS_THROUGH_PATH_DIRECT32_EX {
    type TypeKind = windows_core::CopyType;
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
impl Default for MPIO_PASS_THROUGH_PATH_DIRECT32_EX {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct MPIO_PASS_THROUGH_PATH_DIRECT_EX {
    pub PassThroughOffset: u32,
    pub Version: u32,
    pub Length: u16,
    pub Flags: u8,
    pub PortNumber: u8,
    pub MpioPathId: u64,
}
impl windows_core::TypeKind for MPIO_PASS_THROUGH_PATH_DIRECT_EX {
    type TypeKind = windows_core::CopyType;
}
impl Default for MPIO_PASS_THROUGH_PATH_DIRECT_EX {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct MPIO_PASS_THROUGH_PATH_EX {
    pub PassThroughOffset: u32,
    pub Version: u32,
    pub Length: u16,
    pub Flags: u8,
    pub PortNumber: u8,
    pub MpioPathId: u64,
}
impl windows_core::TypeKind for MPIO_PASS_THROUGH_PATH_EX {
    type TypeKind = windows_core::CopyType;
}
impl Default for MPIO_PASS_THROUGH_PATH_EX {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct MP_DEVICE_DATA_SET_RANGE {
    pub StartingOffset: i64,
    pub LengthInBytes: u64,
}
impl windows_core::TypeKind for MP_DEVICE_DATA_SET_RANGE {
    type TypeKind = windows_core::CopyType;
}
impl Default for MP_DEVICE_DATA_SET_RANGE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct NTSCSI_UNICODE_STRING {
    pub Length: u16,
    pub MaximumLength: u16,
    pub Buffer: windows_core::PWSTR,
}
impl windows_core::TypeKind for NTSCSI_UNICODE_STRING {
    type TypeKind = windows_core::CopyType;
}
impl Default for NTSCSI_UNICODE_STRING {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct NVCACHE_HINT_PAYLOAD {
    pub Command: u8,
    pub Feature7_0: u8,
    pub Feature15_8: u8,
    pub Count15_8: u8,
    pub LBA7_0: u8,
    pub LBA15_8: u8,
    pub LBA23_16: u8,
    pub LBA31_24: u8,
    pub LBA39_32: u8,
    pub LBA47_40: u8,
    pub Auxiliary7_0: u8,
    pub Auxiliary23_16: u8,
    pub Reserved: [u8; 4],
}
impl windows_core::TypeKind for NVCACHE_HINT_PAYLOAD {
    type TypeKind = windows_core::CopyType;
}
impl Default for NVCACHE_HINT_PAYLOAD {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct NVCACHE_PRIORITY_LEVEL_DESCRIPTOR {
    pub PriorityLevel: u8,
    pub Reserved0: [u8; 3],
    pub ConsumedNVMSizeFraction: u32,
    pub ConsumedMappingResourcesFraction: u32,
    pub ConsumedNVMSizeForDirtyDataFraction: u32,
    pub ConsumedMappingResourcesForDirtyDataFraction: u32,
    pub Reserved1: u32,
}
impl windows_core::TypeKind for NVCACHE_PRIORITY_LEVEL_DESCRIPTOR {
    type TypeKind = windows_core::CopyType;
}
impl Default for NVCACHE_PRIORITY_LEVEL_DESCRIPTOR {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct NVCACHE_REQUEST_BLOCK {
    pub NRBSize: u32,
    pub Function: u16,
    pub NRBFlags: u32,
    pub NRBStatus: u32,
    pub Count: u32,
    pub LBA: u64,
    pub DataBufSize: u32,
    pub NVCacheStatus: u32,
    pub NVCacheSubStatus: u32,
}
impl windows_core::TypeKind for NVCACHE_REQUEST_BLOCK {
    type TypeKind = windows_core::CopyType;
}
impl Default for NVCACHE_REQUEST_BLOCK {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct NV_FEATURE_PARAMETER {
    pub NVPowerModeEnabled: u16,
    pub NVParameterReserv1: u16,
    pub NVCmdEnabled: u16,
    pub NVParameterReserv2: u16,
    pub NVPowerModeVer: u16,
    pub NVCmdVer: u16,
    pub NVSize: u32,
    pub NVReadSpeed: u16,
    pub NVWrtSpeed: u16,
    pub DeviceSpinUpTime: u32,
}
impl windows_core::TypeKind for NV_FEATURE_PARAMETER {
    type TypeKind = windows_core::CopyType;
}
impl Default for NV_FEATURE_PARAMETER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct NV_SEP_CACHE_PARAMETER {
    pub Version: u32,
    pub Size: u32,
    pub Flags: NV_SEP_CACHE_PARAMETER_0,
    pub WriteCacheType: u8,
    pub WriteCacheTypeEffective: u8,
    pub ParameterReserve1: [u8; 3],
}
impl windows_core::TypeKind for NV_SEP_CACHE_PARAMETER {
    type TypeKind = windows_core::CopyType;
}
impl Default for NV_SEP_CACHE_PARAMETER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union NV_SEP_CACHE_PARAMETER_0 {
    pub CacheFlags: NV_SEP_CACHE_PARAMETER_0_0,
    pub CacheFlagsSet: u8,
}
impl windows_core::TypeKind for NV_SEP_CACHE_PARAMETER_0 {
    type TypeKind = windows_core::CopyType;
}
impl Default for NV_SEP_CACHE_PARAMETER_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct NV_SEP_CACHE_PARAMETER_0_0 {
    pub _bitfield: u8,
}
impl windows_core::TypeKind for NV_SEP_CACHE_PARAMETER_0_0 {
    type TypeKind = windows_core::CopyType;
}
impl Default for NV_SEP_CACHE_PARAMETER_0_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct PERSISTENT_ISCSI_LOGIN_INFOA {
    pub TargetName: [i8; 224],
    pub IsInformationalSession: super::super::Foundation::BOOLEAN,
    pub InitiatorInstance: [i8; 256],
    pub InitiatorPortNumber: u32,
    pub TargetPortal: ISCSI_TARGET_PORTALA,
    pub SecurityFlags: u64,
    pub Mappings: *mut ISCSI_TARGET_MAPPINGA,
    pub LoginOptions: ISCSI_LOGIN_OPTIONS,
}
impl windows_core::TypeKind for PERSISTENT_ISCSI_LOGIN_INFOA {
    type TypeKind = windows_core::CopyType;
}
impl Default for PERSISTENT_ISCSI_LOGIN_INFOA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct PERSISTENT_ISCSI_LOGIN_INFOW {
    pub TargetName: [u16; 224],
    pub IsInformationalSession: super::super::Foundation::BOOLEAN,
    pub InitiatorInstance: [u16; 256],
    pub InitiatorPortNumber: u32,
    pub TargetPortal: ISCSI_TARGET_PORTALW,
    pub SecurityFlags: u64,
    pub Mappings: *mut ISCSI_TARGET_MAPPINGW,
    pub LoginOptions: ISCSI_LOGIN_OPTIONS,
}
impl windows_core::TypeKind for PERSISTENT_ISCSI_LOGIN_INFOW {
    type TypeKind = windows_core::CopyType;
}
impl Default for PERSISTENT_ISCSI_LOGIN_INFOW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SCSI_ADAPTER_BUS_INFO {
    pub NumberOfBuses: u8,
    pub BusData: [SCSI_BUS_DATA; 1],
}
impl windows_core::TypeKind for SCSI_ADAPTER_BUS_INFO {
    type TypeKind = windows_core::CopyType;
}
impl Default for SCSI_ADAPTER_BUS_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SCSI_ADDRESS {
    pub Length: u32,
    pub PortNumber: u8,
    pub PathId: u8,
    pub TargetId: u8,
    pub Lun: u8,
}
impl windows_core::TypeKind for SCSI_ADDRESS {
    type TypeKind = windows_core::CopyType;
}
impl Default for SCSI_ADDRESS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SCSI_BUS_DATA {
    pub NumberOfLogicalUnits: u8,
    pub InitiatorBusId: u8,
    pub InquiryDataOffset: u32,
}
impl windows_core::TypeKind for SCSI_BUS_DATA {
    type TypeKind = windows_core::CopyType;
}
impl Default for SCSI_BUS_DATA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SCSI_INQUIRY_DATA {
    pub PathId: u8,
    pub TargetId: u8,
    pub Lun: u8,
    pub DeviceClaimed: super::super::Foundation::BOOLEAN,
    pub InquiryDataLength: u32,
    pub NextInquiryDataOffset: u32,
    pub InquiryData: [u8; 1],
}
impl windows_core::TypeKind for SCSI_INQUIRY_DATA {
    type TypeKind = windows_core::CopyType;
}
impl Default for SCSI_INQUIRY_DATA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SCSI_LUN_LIST {
    pub OSLUN: u32,
    pub TargetLUN: u64,
}
impl windows_core::TypeKind for SCSI_LUN_LIST {
    type TypeKind = windows_core::CopyType;
}
impl Default for SCSI_LUN_LIST {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SCSI_PASS_THROUGH {
    pub Length: u16,
    pub ScsiStatus: u8,
    pub PathId: u8,
    pub TargetId: u8,
    pub Lun: u8,
    pub CdbLength: u8,
    pub SenseInfoLength: u8,
    pub DataIn: u8,
    pub DataTransferLength: u32,
    pub TimeOutValue: u32,
    pub DataBufferOffset: usize,
    pub SenseInfoOffset: u32,
    pub Cdb: [u8; 16],
}
impl windows_core::TypeKind for SCSI_PASS_THROUGH {
    type TypeKind = windows_core::CopyType;
}
impl Default for SCSI_PASS_THROUGH {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SCSI_PASS_THROUGH32 {
    pub Length: u16,
    pub ScsiStatus: u8,
    pub PathId: u8,
    pub TargetId: u8,
    pub Lun: u8,
    pub CdbLength: u8,
    pub SenseInfoLength: u8,
    pub DataIn: u8,
    pub DataTransferLength: u32,
    pub TimeOutValue: u32,
    pub DataBufferOffset: u32,
    pub SenseInfoOffset: u32,
    pub Cdb: [u8; 16],
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
impl windows_core::TypeKind for SCSI_PASS_THROUGH32 {
    type TypeKind = windows_core::CopyType;
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
impl Default for SCSI_PASS_THROUGH32 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SCSI_PASS_THROUGH32_EX {
    pub Version: u32,
    pub Length: u32,
    pub CdbLength: u32,
    pub StorAddressLength: u32,
    pub ScsiStatus: u8,
    pub SenseInfoLength: u8,
    pub DataDirection: u8,
    pub Reserved: u8,
    pub TimeOutValue: u32,
    pub StorAddressOffset: u32,
    pub SenseInfoOffset: u32,
    pub DataOutTransferLength: u32,
    pub DataInTransferLength: u32,
    pub DataOutBufferOffset: u32,
    pub DataInBufferOffset: u32,
    pub Cdb: [u8; 1],
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
impl windows_core::TypeKind for SCSI_PASS_THROUGH32_EX {
    type TypeKind = windows_core::CopyType;
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
impl Default for SCSI_PASS_THROUGH32_EX {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SCSI_PASS_THROUGH_DIRECT {
    pub Length: u16,
    pub ScsiStatus: u8,
    pub PathId: u8,
    pub TargetId: u8,
    pub Lun: u8,
    pub CdbLength: u8,
    pub SenseInfoLength: u8,
    pub DataIn: u8,
    pub DataTransferLength: u32,
    pub TimeOutValue: u32,
    pub DataBuffer: *mut core::ffi::c_void,
    pub SenseInfoOffset: u32,
    pub Cdb: [u8; 16],
}
impl windows_core::TypeKind for SCSI_PASS_THROUGH_DIRECT {
    type TypeKind = windows_core::CopyType;
}
impl Default for SCSI_PASS_THROUGH_DIRECT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SCSI_PASS_THROUGH_DIRECT32 {
    pub Length: u16,
    pub ScsiStatus: u8,
    pub PathId: u8,
    pub TargetId: u8,
    pub Lun: u8,
    pub CdbLength: u8,
    pub SenseInfoLength: u8,
    pub DataIn: u8,
    pub DataTransferLength: u32,
    pub TimeOutValue: u32,
    pub DataBuffer: *mut core::ffi::c_void,
    pub SenseInfoOffset: u32,
    pub Cdb: [u8; 16],
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
impl windows_core::TypeKind for SCSI_PASS_THROUGH_DIRECT32 {
    type TypeKind = windows_core::CopyType;
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
impl Default for SCSI_PASS_THROUGH_DIRECT32 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SCSI_PASS_THROUGH_DIRECT32_EX {
    pub Version: u32,
    pub Length: u32,
    pub CdbLength: u32,
    pub StorAddressLength: u32,
    pub ScsiStatus: u8,
    pub SenseInfoLength: u8,
    pub DataDirection: u8,
    pub Reserved: u8,
    pub TimeOutValue: u32,
    pub StorAddressOffset: u32,
    pub SenseInfoOffset: u32,
    pub DataOutTransferLength: u32,
    pub DataInTransferLength: u32,
    pub DataOutBuffer: *mut core::ffi::c_void,
    pub DataInBuffer: *mut core::ffi::c_void,
    pub Cdb: [u8; 1],
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
impl windows_core::TypeKind for SCSI_PASS_THROUGH_DIRECT32_EX {
    type TypeKind = windows_core::CopyType;
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
impl Default for SCSI_PASS_THROUGH_DIRECT32_EX {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SCSI_PASS_THROUGH_DIRECT_EX {
    pub Version: u32,
    pub Length: u32,
    pub CdbLength: u32,
    pub StorAddressLength: u32,
    pub ScsiStatus: u8,
    pub SenseInfoLength: u8,
    pub DataDirection: u8,
    pub Reserved: u8,
    pub TimeOutValue: u32,
    pub StorAddressOffset: u32,
    pub SenseInfoOffset: u32,
    pub DataOutTransferLength: u32,
    pub DataInTransferLength: u32,
    pub DataOutBuffer: *mut core::ffi::c_void,
    pub DataInBuffer: *mut core::ffi::c_void,
    pub Cdb: [u8; 1],
}
impl windows_core::TypeKind for SCSI_PASS_THROUGH_DIRECT_EX {
    type TypeKind = windows_core::CopyType;
}
impl Default for SCSI_PASS_THROUGH_DIRECT_EX {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SCSI_PASS_THROUGH_EX {
    pub Version: u32,
    pub Length: u32,
    pub CdbLength: u32,
    pub StorAddressLength: u32,
    pub ScsiStatus: u8,
    pub SenseInfoLength: u8,
    pub DataDirection: u8,
    pub Reserved: u8,
    pub TimeOutValue: u32,
    pub StorAddressOffset: u32,
    pub SenseInfoOffset: u32,
    pub DataOutTransferLength: u32,
    pub DataInTransferLength: u32,
    pub DataOutBufferOffset: usize,
    pub DataInBufferOffset: usize,
    pub Cdb: [u8; 1],
}
impl windows_core::TypeKind for SCSI_PASS_THROUGH_EX {
    type TypeKind = windows_core::CopyType;
}
impl Default for SCSI_PASS_THROUGH_EX {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SRB_IO_CONTROL {
    pub HeaderLength: u32,
    pub Signature: [u8; 8],
    pub Timeout: u32,
    pub ControlCode: u32,
    pub ReturnCode: u32,
    pub Length: u32,
}
impl windows_core::TypeKind for SRB_IO_CONTROL {
    type TypeKind = windows_core::CopyType;
}
impl Default for SRB_IO_CONTROL {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct STORAGE_DIAGNOSTIC_MP_REQUEST {
    pub Version: u32,
    pub Size: u32,
    pub TargetType: MP_STORAGE_DIAGNOSTIC_TARGET_TYPE,
    pub Level: MP_STORAGE_DIAGNOSTIC_LEVEL,
    pub ProviderId: windows_core::GUID,
    pub BufferSize: u32,
    pub Reserved: u32,
    pub DataBuffer: [u8; 1],
}
impl windows_core::TypeKind for STORAGE_DIAGNOSTIC_MP_REQUEST {
    type TypeKind = windows_core::CopyType;
}
impl Default for STORAGE_DIAGNOSTIC_MP_REQUEST {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct STORAGE_ENDURANCE_DATA_DESCRIPTOR {
    pub Version: u32,
    pub Size: u32,
    pub EnduranceInfo: STORAGE_ENDURANCE_INFO,
}
impl windows_core::TypeKind for STORAGE_ENDURANCE_DATA_DESCRIPTOR {
    type TypeKind = windows_core::CopyType;
}
impl Default for STORAGE_ENDURANCE_DATA_DESCRIPTOR {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct STORAGE_ENDURANCE_INFO {
    pub ValidFields: u32,
    pub GroupId: u32,
    pub Flags: STORAGE_ENDURANCE_INFO_0,
    pub LifePercentage: u32,
    pub BytesReadCount: [u8; 16],
    pub ByteWriteCount: [u8; 16],
}
impl windows_core::TypeKind for STORAGE_ENDURANCE_INFO {
    type TypeKind = windows_core::CopyType;
}
impl Default for STORAGE_ENDURANCE_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct STORAGE_ENDURANCE_INFO_0 {
    pub _bitfield: u32,
}
impl windows_core::TypeKind for STORAGE_ENDURANCE_INFO_0 {
    type TypeKind = windows_core::CopyType;
}
impl Default for STORAGE_ENDURANCE_INFO_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct STORAGE_FIRMWARE_ACTIVATE {
    pub Version: u32,
    pub Size: u32,
    pub SlotToActivate: u8,
    pub Reserved0: [u8; 3],
}
impl windows_core::TypeKind for STORAGE_FIRMWARE_ACTIVATE {
    type TypeKind = windows_core::CopyType;
}
impl Default for STORAGE_FIRMWARE_ACTIVATE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct STORAGE_FIRMWARE_DOWNLOAD {
    pub Version: u32,
    pub Size: u32,
    pub Offset: u64,
    pub BufferSize: u64,
    pub ImageBuffer: [u8; 1],
}
impl windows_core::TypeKind for STORAGE_FIRMWARE_DOWNLOAD {
    type TypeKind = windows_core::CopyType;
}
impl Default for STORAGE_FIRMWARE_DOWNLOAD {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct STORAGE_FIRMWARE_DOWNLOAD_V2 {
    pub Version: u32,
    pub Size: u32,
    pub Offset: u64,
    pub BufferSize: u64,
    pub Slot: u8,
    pub Reserved: [u8; 3],
    pub ImageSize: u32,
    pub ImageBuffer: [u8; 1],
}
impl windows_core::TypeKind for STORAGE_FIRMWARE_DOWNLOAD_V2 {
    type TypeKind = windows_core::CopyType;
}
impl Default for STORAGE_FIRMWARE_DOWNLOAD_V2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct STORAGE_FIRMWARE_INFO {
    pub Version: u32,
    pub Size: u32,
    pub UpgradeSupport: super::super::Foundation::BOOLEAN,
    pub SlotCount: u8,
    pub ActiveSlot: u8,
    pub PendingActivateSlot: u8,
    pub Reserved: u32,
    pub Slot: [STORAGE_FIRMWARE_SLOT_INFO; 1],
}
impl windows_core::TypeKind for STORAGE_FIRMWARE_INFO {
    type TypeKind = windows_core::CopyType;
}
impl Default for STORAGE_FIRMWARE_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct STORAGE_FIRMWARE_INFO_V2 {
    pub Version: u32,
    pub Size: u32,
    pub UpgradeSupport: super::super::Foundation::BOOLEAN,
    pub SlotCount: u8,
    pub ActiveSlot: u8,
    pub PendingActivateSlot: u8,
    pub FirmwareShared: super::super::Foundation::BOOLEAN,
    pub Reserved: [u8; 3],
    pub ImagePayloadAlignment: u32,
    pub ImagePayloadMaxSize: u32,
    pub Slot: [STORAGE_FIRMWARE_SLOT_INFO_V2; 1],
}
impl windows_core::TypeKind for STORAGE_FIRMWARE_INFO_V2 {
    type TypeKind = windows_core::CopyType;
}
impl Default for STORAGE_FIRMWARE_INFO_V2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct STORAGE_FIRMWARE_SLOT_INFO {
    pub SlotNumber: u8,
    pub ReadOnly: super::super::Foundation::BOOLEAN,
    pub Reserved: [u8; 6],
    pub Revision: STORAGE_FIRMWARE_SLOT_INFO_0,
}
impl windows_core::TypeKind for STORAGE_FIRMWARE_SLOT_INFO {
    type TypeKind = windows_core::CopyType;
}
impl Default for STORAGE_FIRMWARE_SLOT_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union STORAGE_FIRMWARE_SLOT_INFO_0 {
    pub Info: [u8; 8],
    pub AsUlonglong: u64,
}
impl windows_core::TypeKind for STORAGE_FIRMWARE_SLOT_INFO_0 {
    type TypeKind = windows_core::CopyType;
}
impl Default for STORAGE_FIRMWARE_SLOT_INFO_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct STORAGE_FIRMWARE_SLOT_INFO_V2 {
    pub SlotNumber: u8,
    pub ReadOnly: super::super::Foundation::BOOLEAN,
    pub Reserved: [u8; 6],
    pub Revision: [u8; 16],
}
impl windows_core::TypeKind for STORAGE_FIRMWARE_SLOT_INFO_V2 {
    type TypeKind = windows_core::CopyType;
}
impl Default for STORAGE_FIRMWARE_SLOT_INFO_V2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct _ADAPTER_OBJECT(pub isize);
impl Default for _ADAPTER_OBJECT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for _ADAPTER_OBJECT {
    type TypeKind = windows_core::CopyType;
}
pub type PDUMP_DEVICE_POWERON_ROUTINE = Option<unsafe extern "system" fn(context: *const core::ffi::c_void) -> i32>;
