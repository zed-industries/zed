#[cfg(feature = "Win32_Security")]
#[inline]
pub unsafe fn ChangeServiceConfig2A<P0>(hservice: P0, dwinfolevel: SERVICE_CONFIG, lpinfo: Option<*const core::ffi::c_void>) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::super::Security::SC_HANDLE>,
{
    windows_targets::link!("advapi32.dll" "system" fn ChangeServiceConfig2A(hservice : super::super::Security:: SC_HANDLE, dwinfolevel : SERVICE_CONFIG, lpinfo : *const core::ffi::c_void) -> super::super::Foundation:: BOOL);
    ChangeServiceConfig2A(hservice.param().abi(), dwinfolevel, core::mem::transmute(lpinfo.unwrap_or(std::ptr::null()))).ok()
}
#[cfg(feature = "Win32_Security")]
#[inline]
pub unsafe fn ChangeServiceConfig2W<P0>(hservice: P0, dwinfolevel: SERVICE_CONFIG, lpinfo: Option<*const core::ffi::c_void>) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::super::Security::SC_HANDLE>,
{
    windows_targets::link!("advapi32.dll" "system" fn ChangeServiceConfig2W(hservice : super::super::Security:: SC_HANDLE, dwinfolevel : SERVICE_CONFIG, lpinfo : *const core::ffi::c_void) -> super::super::Foundation:: BOOL);
    ChangeServiceConfig2W(hservice.param().abi(), dwinfolevel, core::mem::transmute(lpinfo.unwrap_or(std::ptr::null()))).ok()
}
#[cfg(feature = "Win32_Security")]
#[inline]
pub unsafe fn ChangeServiceConfigA<P0, P1, P2, P3, P4, P5, P6>(hservice: P0, dwservicetype: ENUM_SERVICE_TYPE, dwstarttype: SERVICE_START_TYPE, dwerrorcontrol: SERVICE_ERROR, lpbinarypathname: P1, lploadordergroup: P2, lpdwtagid: Option<*mut u32>, lpdependencies: P3, lpservicestartname: P4, lppassword: P5, lpdisplayname: P6) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::super::Security::SC_HANDLE>,
    P1: windows_core::Param<windows_core::PCSTR>,
    P2: windows_core::Param<windows_core::PCSTR>,
    P3: windows_core::Param<windows_core::PCSTR>,
    P4: windows_core::Param<windows_core::PCSTR>,
    P5: windows_core::Param<windows_core::PCSTR>,
    P6: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("advapi32.dll" "system" fn ChangeServiceConfigA(hservice : super::super::Security:: SC_HANDLE, dwservicetype : ENUM_SERVICE_TYPE, dwstarttype : SERVICE_START_TYPE, dwerrorcontrol : SERVICE_ERROR, lpbinarypathname : windows_core::PCSTR, lploadordergroup : windows_core::PCSTR, lpdwtagid : *mut u32, lpdependencies : windows_core::PCSTR, lpservicestartname : windows_core::PCSTR, lppassword : windows_core::PCSTR, lpdisplayname : windows_core::PCSTR) -> super::super::Foundation:: BOOL);
    ChangeServiceConfigA(hservice.param().abi(), dwservicetype, dwstarttype, dwerrorcontrol, lpbinarypathname.param().abi(), lploadordergroup.param().abi(), core::mem::transmute(lpdwtagid.unwrap_or(std::ptr::null_mut())), lpdependencies.param().abi(), lpservicestartname.param().abi(), lppassword.param().abi(), lpdisplayname.param().abi()).ok()
}
#[cfg(feature = "Win32_Security")]
#[inline]
pub unsafe fn ChangeServiceConfigW<P0, P1, P2, P3, P4, P5, P6>(hservice: P0, dwservicetype: ENUM_SERVICE_TYPE, dwstarttype: SERVICE_START_TYPE, dwerrorcontrol: SERVICE_ERROR, lpbinarypathname: P1, lploadordergroup: P2, lpdwtagid: Option<*mut u32>, lpdependencies: P3, lpservicestartname: P4, lppassword: P5, lpdisplayname: P6) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::super::Security::SC_HANDLE>,
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
    P3: windows_core::Param<windows_core::PCWSTR>,
    P4: windows_core::Param<windows_core::PCWSTR>,
    P5: windows_core::Param<windows_core::PCWSTR>,
    P6: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("advapi32.dll" "system" fn ChangeServiceConfigW(hservice : super::super::Security:: SC_HANDLE, dwservicetype : ENUM_SERVICE_TYPE, dwstarttype : SERVICE_START_TYPE, dwerrorcontrol : SERVICE_ERROR, lpbinarypathname : windows_core::PCWSTR, lploadordergroup : windows_core::PCWSTR, lpdwtagid : *mut u32, lpdependencies : windows_core::PCWSTR, lpservicestartname : windows_core::PCWSTR, lppassword : windows_core::PCWSTR, lpdisplayname : windows_core::PCWSTR) -> super::super::Foundation:: BOOL);
    ChangeServiceConfigW(hservice.param().abi(), dwservicetype, dwstarttype, dwerrorcontrol, lpbinarypathname.param().abi(), lploadordergroup.param().abi(), core::mem::transmute(lpdwtagid.unwrap_or(std::ptr::null_mut())), lpdependencies.param().abi(), lpservicestartname.param().abi(), lppassword.param().abi(), lpdisplayname.param().abi()).ok()
}
#[cfg(feature = "Win32_Security")]
#[inline]
pub unsafe fn CloseServiceHandle<P0>(hscobject: P0) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::super::Security::SC_HANDLE>,
{
    windows_targets::link!("advapi32.dll" "system" fn CloseServiceHandle(hscobject : super::super::Security:: SC_HANDLE) -> super::super::Foundation:: BOOL);
    CloseServiceHandle(hscobject.param().abi()).ok()
}
#[cfg(feature = "Win32_Security")]
#[inline]
pub unsafe fn ControlService<P0>(hservice: P0, dwcontrol: u32, lpservicestatus: *mut SERVICE_STATUS) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::super::Security::SC_HANDLE>,
{
    windows_targets::link!("advapi32.dll" "system" fn ControlService(hservice : super::super::Security:: SC_HANDLE, dwcontrol : u32, lpservicestatus : *mut SERVICE_STATUS) -> super::super::Foundation:: BOOL);
    ControlService(hservice.param().abi(), dwcontrol, lpservicestatus).ok()
}
#[cfg(feature = "Win32_Security")]
#[inline]
pub unsafe fn ControlServiceExA<P0>(hservice: P0, dwcontrol: u32, dwinfolevel: u32, pcontrolparams: *mut core::ffi::c_void) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::super::Security::SC_HANDLE>,
{
    windows_targets::link!("advapi32.dll" "system" fn ControlServiceExA(hservice : super::super::Security:: SC_HANDLE, dwcontrol : u32, dwinfolevel : u32, pcontrolparams : *mut core::ffi::c_void) -> super::super::Foundation:: BOOL);
    ControlServiceExA(hservice.param().abi(), dwcontrol, dwinfolevel, pcontrolparams).ok()
}
#[cfg(feature = "Win32_Security")]
#[inline]
pub unsafe fn ControlServiceExW<P0>(hservice: P0, dwcontrol: u32, dwinfolevel: u32, pcontrolparams: *mut core::ffi::c_void) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::super::Security::SC_HANDLE>,
{
    windows_targets::link!("advapi32.dll" "system" fn ControlServiceExW(hservice : super::super::Security:: SC_HANDLE, dwcontrol : u32, dwinfolevel : u32, pcontrolparams : *mut core::ffi::c_void) -> super::super::Foundation:: BOOL);
    ControlServiceExW(hservice.param().abi(), dwcontrol, dwinfolevel, pcontrolparams).ok()
}
#[cfg(feature = "Win32_Security")]
#[inline]
pub unsafe fn CreateServiceA<P0, P1, P2, P3, P4, P5, P6, P7>(hscmanager: P0, lpservicename: P1, lpdisplayname: P2, dwdesiredaccess: u32, dwservicetype: ENUM_SERVICE_TYPE, dwstarttype: SERVICE_START_TYPE, dwerrorcontrol: SERVICE_ERROR, lpbinarypathname: P3, lploadordergroup: P4, lpdwtagid: Option<*mut u32>, lpdependencies: P5, lpservicestartname: P6, lppassword: P7) -> windows_core::Result<super::super::Security::SC_HANDLE>
where
    P0: windows_core::Param<super::super::Security::SC_HANDLE>,
    P1: windows_core::Param<windows_core::PCSTR>,
    P2: windows_core::Param<windows_core::PCSTR>,
    P3: windows_core::Param<windows_core::PCSTR>,
    P4: windows_core::Param<windows_core::PCSTR>,
    P5: windows_core::Param<windows_core::PCSTR>,
    P6: windows_core::Param<windows_core::PCSTR>,
    P7: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("advapi32.dll" "system" fn CreateServiceA(hscmanager : super::super::Security:: SC_HANDLE, lpservicename : windows_core::PCSTR, lpdisplayname : windows_core::PCSTR, dwdesiredaccess : u32, dwservicetype : ENUM_SERVICE_TYPE, dwstarttype : SERVICE_START_TYPE, dwerrorcontrol : SERVICE_ERROR, lpbinarypathname : windows_core::PCSTR, lploadordergroup : windows_core::PCSTR, lpdwtagid : *mut u32, lpdependencies : windows_core::PCSTR, lpservicestartname : windows_core::PCSTR, lppassword : windows_core::PCSTR) -> super::super::Security:: SC_HANDLE);
    let result__ = CreateServiceA(hscmanager.param().abi(), lpservicename.param().abi(), lpdisplayname.param().abi(), dwdesiredaccess, dwservicetype, dwstarttype, dwerrorcontrol, lpbinarypathname.param().abi(), lploadordergroup.param().abi(), core::mem::transmute(lpdwtagid.unwrap_or(std::ptr::null_mut())), lpdependencies.param().abi(), lpservicestartname.param().abi(), lppassword.param().abi());
    (!result__.is_invalid()).then(|| result__).ok_or_else(windows_core::Error::from_win32)
}
#[cfg(feature = "Win32_Security")]
#[inline]
pub unsafe fn CreateServiceW<P0, P1, P2, P3, P4, P5, P6, P7>(hscmanager: P0, lpservicename: P1, lpdisplayname: P2, dwdesiredaccess: u32, dwservicetype: ENUM_SERVICE_TYPE, dwstarttype: SERVICE_START_TYPE, dwerrorcontrol: SERVICE_ERROR, lpbinarypathname: P3, lploadordergroup: P4, lpdwtagid: Option<*mut u32>, lpdependencies: P5, lpservicestartname: P6, lppassword: P7) -> windows_core::Result<super::super::Security::SC_HANDLE>
where
    P0: windows_core::Param<super::super::Security::SC_HANDLE>,
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
    P3: windows_core::Param<windows_core::PCWSTR>,
    P4: windows_core::Param<windows_core::PCWSTR>,
    P5: windows_core::Param<windows_core::PCWSTR>,
    P6: windows_core::Param<windows_core::PCWSTR>,
    P7: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("advapi32.dll" "system" fn CreateServiceW(hscmanager : super::super::Security:: SC_HANDLE, lpservicename : windows_core::PCWSTR, lpdisplayname : windows_core::PCWSTR, dwdesiredaccess : u32, dwservicetype : ENUM_SERVICE_TYPE, dwstarttype : SERVICE_START_TYPE, dwerrorcontrol : SERVICE_ERROR, lpbinarypathname : windows_core::PCWSTR, lploadordergroup : windows_core::PCWSTR, lpdwtagid : *mut u32, lpdependencies : windows_core::PCWSTR, lpservicestartname : windows_core::PCWSTR, lppassword : windows_core::PCWSTR) -> super::super::Security:: SC_HANDLE);
    let result__ = CreateServiceW(hscmanager.param().abi(), lpservicename.param().abi(), lpdisplayname.param().abi(), dwdesiredaccess, dwservicetype, dwstarttype, dwerrorcontrol, lpbinarypathname.param().abi(), lploadordergroup.param().abi(), core::mem::transmute(lpdwtagid.unwrap_or(std::ptr::null_mut())), lpdependencies.param().abi(), lpservicestartname.param().abi(), lppassword.param().abi());
    (!result__.is_invalid()).then(|| result__).ok_or_else(windows_core::Error::from_win32)
}
#[cfg(feature = "Win32_Security")]
#[inline]
pub unsafe fn DeleteService<P0>(hservice: P0) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::super::Security::SC_HANDLE>,
{
    windows_targets::link!("advapi32.dll" "system" fn DeleteService(hservice : super::super::Security:: SC_HANDLE) -> super::super::Foundation:: BOOL);
    DeleteService(hservice.param().abi()).ok()
}
#[cfg(feature = "Win32_Security")]
#[inline]
pub unsafe fn EnumDependentServicesA<P0>(hservice: P0, dwservicestate: ENUM_SERVICE_STATE, lpservices: Option<*mut ENUM_SERVICE_STATUSA>, cbbufsize: u32, pcbbytesneeded: *mut u32, lpservicesreturned: *mut u32) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::super::Security::SC_HANDLE>,
{
    windows_targets::link!("advapi32.dll" "system" fn EnumDependentServicesA(hservice : super::super::Security:: SC_HANDLE, dwservicestate : ENUM_SERVICE_STATE, lpservices : *mut ENUM_SERVICE_STATUSA, cbbufsize : u32, pcbbytesneeded : *mut u32, lpservicesreturned : *mut u32) -> super::super::Foundation:: BOOL);
    EnumDependentServicesA(hservice.param().abi(), dwservicestate, core::mem::transmute(lpservices.unwrap_or(std::ptr::null_mut())), cbbufsize, pcbbytesneeded, lpservicesreturned).ok()
}
#[cfg(feature = "Win32_Security")]
#[inline]
pub unsafe fn EnumDependentServicesW<P0>(hservice: P0, dwservicestate: ENUM_SERVICE_STATE, lpservices: Option<*mut ENUM_SERVICE_STATUSW>, cbbufsize: u32, pcbbytesneeded: *mut u32, lpservicesreturned: *mut u32) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::super::Security::SC_HANDLE>,
{
    windows_targets::link!("advapi32.dll" "system" fn EnumDependentServicesW(hservice : super::super::Security:: SC_HANDLE, dwservicestate : ENUM_SERVICE_STATE, lpservices : *mut ENUM_SERVICE_STATUSW, cbbufsize : u32, pcbbytesneeded : *mut u32, lpservicesreturned : *mut u32) -> super::super::Foundation:: BOOL);
    EnumDependentServicesW(hservice.param().abi(), dwservicestate, core::mem::transmute(lpservices.unwrap_or(std::ptr::null_mut())), cbbufsize, pcbbytesneeded, lpservicesreturned).ok()
}
#[cfg(feature = "Win32_Security")]
#[inline]
pub unsafe fn EnumServicesStatusA<P0>(hscmanager: P0, dwservicetype: ENUM_SERVICE_TYPE, dwservicestate: ENUM_SERVICE_STATE, lpservices: Option<*mut ENUM_SERVICE_STATUSA>, cbbufsize: u32, pcbbytesneeded: *mut u32, lpservicesreturned: *mut u32, lpresumehandle: Option<*mut u32>) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::super::Security::SC_HANDLE>,
{
    windows_targets::link!("advapi32.dll" "system" fn EnumServicesStatusA(hscmanager : super::super::Security:: SC_HANDLE, dwservicetype : ENUM_SERVICE_TYPE, dwservicestate : ENUM_SERVICE_STATE, lpservices : *mut ENUM_SERVICE_STATUSA, cbbufsize : u32, pcbbytesneeded : *mut u32, lpservicesreturned : *mut u32, lpresumehandle : *mut u32) -> super::super::Foundation:: BOOL);
    EnumServicesStatusA(hscmanager.param().abi(), dwservicetype, dwservicestate, core::mem::transmute(lpservices.unwrap_or(std::ptr::null_mut())), cbbufsize, pcbbytesneeded, lpservicesreturned, core::mem::transmute(lpresumehandle.unwrap_or(std::ptr::null_mut()))).ok()
}
#[cfg(feature = "Win32_Security")]
#[inline]
pub unsafe fn EnumServicesStatusExA<P0, P1>(hscmanager: P0, infolevel: SC_ENUM_TYPE, dwservicetype: ENUM_SERVICE_TYPE, dwservicestate: ENUM_SERVICE_STATE, lpservices: Option<&mut [u8]>, pcbbytesneeded: *mut u32, lpservicesreturned: *mut u32, lpresumehandle: Option<*mut u32>, pszgroupname: P1) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::super::Security::SC_HANDLE>,
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("advapi32.dll" "system" fn EnumServicesStatusExA(hscmanager : super::super::Security:: SC_HANDLE, infolevel : SC_ENUM_TYPE, dwservicetype : ENUM_SERVICE_TYPE, dwservicestate : ENUM_SERVICE_STATE, lpservices : *mut u8, cbbufsize : u32, pcbbytesneeded : *mut u32, lpservicesreturned : *mut u32, lpresumehandle : *mut u32, pszgroupname : windows_core::PCSTR) -> super::super::Foundation:: BOOL);
    EnumServicesStatusExA(hscmanager.param().abi(), infolevel, dwservicetype, dwservicestate, core::mem::transmute(lpservices.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), lpservices.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), pcbbytesneeded, lpservicesreturned, core::mem::transmute(lpresumehandle.unwrap_or(std::ptr::null_mut())), pszgroupname.param().abi()).ok()
}
#[cfg(feature = "Win32_Security")]
#[inline]
pub unsafe fn EnumServicesStatusExW<P0, P1>(hscmanager: P0, infolevel: SC_ENUM_TYPE, dwservicetype: ENUM_SERVICE_TYPE, dwservicestate: ENUM_SERVICE_STATE, lpservices: Option<&mut [u8]>, pcbbytesneeded: *mut u32, lpservicesreturned: *mut u32, lpresumehandle: Option<*mut u32>, pszgroupname: P1) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::super::Security::SC_HANDLE>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("advapi32.dll" "system" fn EnumServicesStatusExW(hscmanager : super::super::Security:: SC_HANDLE, infolevel : SC_ENUM_TYPE, dwservicetype : ENUM_SERVICE_TYPE, dwservicestate : ENUM_SERVICE_STATE, lpservices : *mut u8, cbbufsize : u32, pcbbytesneeded : *mut u32, lpservicesreturned : *mut u32, lpresumehandle : *mut u32, pszgroupname : windows_core::PCWSTR) -> super::super::Foundation:: BOOL);
    EnumServicesStatusExW(hscmanager.param().abi(), infolevel, dwservicetype, dwservicestate, core::mem::transmute(lpservices.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), lpservices.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), pcbbytesneeded, lpservicesreturned, core::mem::transmute(lpresumehandle.unwrap_or(std::ptr::null_mut())), pszgroupname.param().abi()).ok()
}
#[cfg(feature = "Win32_Security")]
#[inline]
pub unsafe fn EnumServicesStatusW<P0>(hscmanager: P0, dwservicetype: ENUM_SERVICE_TYPE, dwservicestate: ENUM_SERVICE_STATE, lpservices: Option<*mut ENUM_SERVICE_STATUSW>, cbbufsize: u32, pcbbytesneeded: *mut u32, lpservicesreturned: *mut u32, lpresumehandle: Option<*mut u32>) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::super::Security::SC_HANDLE>,
{
    windows_targets::link!("advapi32.dll" "system" fn EnumServicesStatusW(hscmanager : super::super::Security:: SC_HANDLE, dwservicetype : ENUM_SERVICE_TYPE, dwservicestate : ENUM_SERVICE_STATE, lpservices : *mut ENUM_SERVICE_STATUSW, cbbufsize : u32, pcbbytesneeded : *mut u32, lpservicesreturned : *mut u32, lpresumehandle : *mut u32) -> super::super::Foundation:: BOOL);
    EnumServicesStatusW(hscmanager.param().abi(), dwservicetype, dwservicestate, core::mem::transmute(lpservices.unwrap_or(std::ptr::null_mut())), cbbufsize, pcbbytesneeded, lpservicesreturned, core::mem::transmute(lpresumehandle.unwrap_or(std::ptr::null_mut()))).ok()
}
#[inline]
pub unsafe fn GetServiceDirectory<P0>(hservicestatus: P0, edirectorytype: SERVICE_DIRECTORY_TYPE, lppathbuffer: Option<&mut [u16]>, lpcchrequiredbufferlength: *mut u32) -> u32
where
    P0: windows_core::Param<SERVICE_STATUS_HANDLE>,
{
    windows_targets::link!("api-ms-win-service-core-l1-1-4.dll" "system" fn GetServiceDirectory(hservicestatus : SERVICE_STATUS_HANDLE, edirectorytype : SERVICE_DIRECTORY_TYPE, lppathbuffer : windows_core::PWSTR, cchpathbufferlength : u32, lpcchrequiredbufferlength : *mut u32) -> u32);
    GetServiceDirectory(hservicestatus.param().abi(), edirectorytype, core::mem::transmute(lppathbuffer.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), lppathbuffer.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), lpcchrequiredbufferlength)
}
#[cfg(feature = "Win32_Security")]
#[inline]
pub unsafe fn GetServiceDisplayNameA<P0, P1>(hscmanager: P0, lpservicename: P1, lpdisplayname: windows_core::PSTR, lpcchbuffer: *mut u32) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::super::Security::SC_HANDLE>,
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("advapi32.dll" "system" fn GetServiceDisplayNameA(hscmanager : super::super::Security:: SC_HANDLE, lpservicename : windows_core::PCSTR, lpdisplayname : windows_core::PSTR, lpcchbuffer : *mut u32) -> super::super::Foundation:: BOOL);
    GetServiceDisplayNameA(hscmanager.param().abi(), lpservicename.param().abi(), core::mem::transmute(lpdisplayname), lpcchbuffer).ok()
}
#[cfg(feature = "Win32_Security")]
#[inline]
pub unsafe fn GetServiceDisplayNameW<P0, P1>(hscmanager: P0, lpservicename: P1, lpdisplayname: windows_core::PWSTR, lpcchbuffer: *mut u32) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::super::Security::SC_HANDLE>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("advapi32.dll" "system" fn GetServiceDisplayNameW(hscmanager : super::super::Security:: SC_HANDLE, lpservicename : windows_core::PCWSTR, lpdisplayname : windows_core::PWSTR, lpcchbuffer : *mut u32) -> super::super::Foundation:: BOOL);
    GetServiceDisplayNameW(hscmanager.param().abi(), lpservicename.param().abi(), core::mem::transmute(lpdisplayname), lpcchbuffer).ok()
}
#[cfg(feature = "Win32_Security")]
#[inline]
pub unsafe fn GetServiceKeyNameA<P0, P1>(hscmanager: P0, lpdisplayname: P1, lpservicename: windows_core::PSTR, lpcchbuffer: *mut u32) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::super::Security::SC_HANDLE>,
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("advapi32.dll" "system" fn GetServiceKeyNameA(hscmanager : super::super::Security:: SC_HANDLE, lpdisplayname : windows_core::PCSTR, lpservicename : windows_core::PSTR, lpcchbuffer : *mut u32) -> super::super::Foundation:: BOOL);
    GetServiceKeyNameA(hscmanager.param().abi(), lpdisplayname.param().abi(), core::mem::transmute(lpservicename), lpcchbuffer).ok()
}
#[cfg(feature = "Win32_Security")]
#[inline]
pub unsafe fn GetServiceKeyNameW<P0, P1>(hscmanager: P0, lpdisplayname: P1, lpservicename: windows_core::PWSTR, lpcchbuffer: *mut u32) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::super::Security::SC_HANDLE>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("advapi32.dll" "system" fn GetServiceKeyNameW(hscmanager : super::super::Security:: SC_HANDLE, lpdisplayname : windows_core::PCWSTR, lpservicename : windows_core::PWSTR, lpcchbuffer : *mut u32) -> super::super::Foundation:: BOOL);
    GetServiceKeyNameW(hscmanager.param().abi(), lpdisplayname.param().abi(), core::mem::transmute(lpservicename), lpcchbuffer).ok()
}
#[cfg(feature = "Win32_System_Registry")]
#[inline]
pub unsafe fn GetServiceRegistryStateKey<P0>(servicestatushandle: P0, statetype: SERVICE_REGISTRY_STATE_TYPE, accessmask: u32, servicestatekey: *mut super::Registry::HKEY) -> u32
where
    P0: windows_core::Param<SERVICE_STATUS_HANDLE>,
{
    windows_targets::link!("api-ms-win-service-core-l1-1-3.dll" "system" fn GetServiceRegistryStateKey(servicestatushandle : SERVICE_STATUS_HANDLE, statetype : SERVICE_REGISTRY_STATE_TYPE, accessmask : u32, servicestatekey : *mut super::Registry:: HKEY) -> u32);
    GetServiceRegistryStateKey(servicestatushandle.param().abi(), statetype, accessmask, servicestatekey)
}
#[cfg(feature = "Win32_Security")]
#[inline]
pub unsafe fn GetSharedServiceDirectory<P0>(servicehandle: P0, directorytype: SERVICE_SHARED_DIRECTORY_TYPE, pathbuffer: Option<&mut [u16]>, requiredbufferlength: *mut u32) -> u32
where
    P0: windows_core::Param<super::super::Security::SC_HANDLE>,
{
    windows_targets::link!("api-ms-win-service-core-l1-1-5.dll" "system" fn GetSharedServiceDirectory(servicehandle : super::super::Security:: SC_HANDLE, directorytype : SERVICE_SHARED_DIRECTORY_TYPE, pathbuffer : windows_core::PWSTR, pathbufferlength : u32, requiredbufferlength : *mut u32) -> u32);
    GetSharedServiceDirectory(servicehandle.param().abi(), directorytype, core::mem::transmute(pathbuffer.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), pathbuffer.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), requiredbufferlength)
}
#[cfg(all(feature = "Win32_Security", feature = "Win32_System_Registry"))]
#[inline]
pub unsafe fn GetSharedServiceRegistryStateKey<P0>(servicehandle: P0, statetype: SERVICE_SHARED_REGISTRY_STATE_TYPE, accessmask: u32, servicestatekey: *mut super::Registry::HKEY) -> u32
where
    P0: windows_core::Param<super::super::Security::SC_HANDLE>,
{
    windows_targets::link!("api-ms-win-service-core-l1-1-5.dll" "system" fn GetSharedServiceRegistryStateKey(servicehandle : super::super::Security:: SC_HANDLE, statetype : SERVICE_SHARED_REGISTRY_STATE_TYPE, accessmask : u32, servicestatekey : *mut super::Registry:: HKEY) -> u32);
    GetSharedServiceRegistryStateKey(servicehandle.param().abi(), statetype, accessmask, servicestatekey)
}
#[cfg(feature = "Win32_Security")]
#[inline]
pub unsafe fn LockServiceDatabase<P0>(hscmanager: P0) -> *mut core::ffi::c_void
where
    P0: windows_core::Param<super::super::Security::SC_HANDLE>,
{
    windows_targets::link!("advapi32.dll" "system" fn LockServiceDatabase(hscmanager : super::super::Security:: SC_HANDLE) -> *mut core::ffi::c_void);
    LockServiceDatabase(hscmanager.param().abi())
}
#[inline]
pub unsafe fn NotifyBootConfigStatus<P0>(bootacceptable: P0) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::super::Foundation::BOOL>,
{
    windows_targets::link!("advapi32.dll" "system" fn NotifyBootConfigStatus(bootacceptable : super::super::Foundation:: BOOL) -> super::super::Foundation:: BOOL);
    NotifyBootConfigStatus(bootacceptable.param().abi()).ok()
}
#[cfg(feature = "Win32_Security")]
#[inline]
pub unsafe fn NotifyServiceStatusChangeA<P0>(hservice: P0, dwnotifymask: SERVICE_NOTIFY, pnotifybuffer: *const SERVICE_NOTIFY_2A) -> u32
where
    P0: windows_core::Param<super::super::Security::SC_HANDLE>,
{
    windows_targets::link!("advapi32.dll" "system" fn NotifyServiceStatusChangeA(hservice : super::super::Security:: SC_HANDLE, dwnotifymask : SERVICE_NOTIFY, pnotifybuffer : *const SERVICE_NOTIFY_2A) -> u32);
    NotifyServiceStatusChangeA(hservice.param().abi(), dwnotifymask, pnotifybuffer)
}
#[cfg(feature = "Win32_Security")]
#[inline]
pub unsafe fn NotifyServiceStatusChangeW<P0>(hservice: P0, dwnotifymask: SERVICE_NOTIFY, pnotifybuffer: *const SERVICE_NOTIFY_2W) -> u32
where
    P0: windows_core::Param<super::super::Security::SC_HANDLE>,
{
    windows_targets::link!("advapi32.dll" "system" fn NotifyServiceStatusChangeW(hservice : super::super::Security:: SC_HANDLE, dwnotifymask : SERVICE_NOTIFY, pnotifybuffer : *const SERVICE_NOTIFY_2W) -> u32);
    NotifyServiceStatusChangeW(hservice.param().abi(), dwnotifymask, pnotifybuffer)
}
#[cfg(feature = "Win32_Security")]
#[inline]
pub unsafe fn OpenSCManagerA<P0, P1>(lpmachinename: P0, lpdatabasename: P1, dwdesiredaccess: u32) -> windows_core::Result<super::super::Security::SC_HANDLE>
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("advapi32.dll" "system" fn OpenSCManagerA(lpmachinename : windows_core::PCSTR, lpdatabasename : windows_core::PCSTR, dwdesiredaccess : u32) -> super::super::Security:: SC_HANDLE);
    let result__ = OpenSCManagerA(lpmachinename.param().abi(), lpdatabasename.param().abi(), dwdesiredaccess);
    (!result__.is_invalid()).then(|| result__).ok_or_else(windows_core::Error::from_win32)
}
#[cfg(feature = "Win32_Security")]
#[inline]
pub unsafe fn OpenSCManagerW<P0, P1>(lpmachinename: P0, lpdatabasename: P1, dwdesiredaccess: u32) -> windows_core::Result<super::super::Security::SC_HANDLE>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("advapi32.dll" "system" fn OpenSCManagerW(lpmachinename : windows_core::PCWSTR, lpdatabasename : windows_core::PCWSTR, dwdesiredaccess : u32) -> super::super::Security:: SC_HANDLE);
    let result__ = OpenSCManagerW(lpmachinename.param().abi(), lpdatabasename.param().abi(), dwdesiredaccess);
    (!result__.is_invalid()).then(|| result__).ok_or_else(windows_core::Error::from_win32)
}
#[cfg(feature = "Win32_Security")]
#[inline]
pub unsafe fn OpenServiceA<P0, P1>(hscmanager: P0, lpservicename: P1, dwdesiredaccess: u32) -> windows_core::Result<super::super::Security::SC_HANDLE>
where
    P0: windows_core::Param<super::super::Security::SC_HANDLE>,
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("advapi32.dll" "system" fn OpenServiceA(hscmanager : super::super::Security:: SC_HANDLE, lpservicename : windows_core::PCSTR, dwdesiredaccess : u32) -> super::super::Security:: SC_HANDLE);
    let result__ = OpenServiceA(hscmanager.param().abi(), lpservicename.param().abi(), dwdesiredaccess);
    (!result__.is_invalid()).then(|| result__).ok_or_else(windows_core::Error::from_win32)
}
#[cfg(feature = "Win32_Security")]
#[inline]
pub unsafe fn OpenServiceW<P0, P1>(hscmanager: P0, lpservicename: P1, dwdesiredaccess: u32) -> windows_core::Result<super::super::Security::SC_HANDLE>
where
    P0: windows_core::Param<super::super::Security::SC_HANDLE>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("advapi32.dll" "system" fn OpenServiceW(hscmanager : super::super::Security:: SC_HANDLE, lpservicename : windows_core::PCWSTR, dwdesiredaccess : u32) -> super::super::Security:: SC_HANDLE);
    let result__ = OpenServiceW(hscmanager.param().abi(), lpservicename.param().abi(), dwdesiredaccess);
    (!result__.is_invalid()).then(|| result__).ok_or_else(windows_core::Error::from_win32)
}
#[cfg(feature = "Win32_Security")]
#[inline]
pub unsafe fn QueryServiceConfig2A<P0>(hservice: P0, dwinfolevel: SERVICE_CONFIG, lpbuffer: Option<&mut [u8]>, pcbbytesneeded: *mut u32) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::super::Security::SC_HANDLE>,
{
    windows_targets::link!("advapi32.dll" "system" fn QueryServiceConfig2A(hservice : super::super::Security:: SC_HANDLE, dwinfolevel : SERVICE_CONFIG, lpbuffer : *mut u8, cbbufsize : u32, pcbbytesneeded : *mut u32) -> super::super::Foundation:: BOOL);
    QueryServiceConfig2A(hservice.param().abi(), dwinfolevel, core::mem::transmute(lpbuffer.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), lpbuffer.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), pcbbytesneeded).ok()
}
#[cfg(feature = "Win32_Security")]
#[inline]
pub unsafe fn QueryServiceConfig2W<P0>(hservice: P0, dwinfolevel: SERVICE_CONFIG, lpbuffer: Option<&mut [u8]>, pcbbytesneeded: *mut u32) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::super::Security::SC_HANDLE>,
{
    windows_targets::link!("advapi32.dll" "system" fn QueryServiceConfig2W(hservice : super::super::Security:: SC_HANDLE, dwinfolevel : SERVICE_CONFIG, lpbuffer : *mut u8, cbbufsize : u32, pcbbytesneeded : *mut u32) -> super::super::Foundation:: BOOL);
    QueryServiceConfig2W(hservice.param().abi(), dwinfolevel, core::mem::transmute(lpbuffer.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), lpbuffer.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), pcbbytesneeded).ok()
}
#[cfg(feature = "Win32_Security")]
#[inline]
pub unsafe fn QueryServiceConfigA<P0>(hservice: P0, lpserviceconfig: Option<*mut QUERY_SERVICE_CONFIGA>, cbbufsize: u32, pcbbytesneeded: *mut u32) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::super::Security::SC_HANDLE>,
{
    windows_targets::link!("advapi32.dll" "system" fn QueryServiceConfigA(hservice : super::super::Security:: SC_HANDLE, lpserviceconfig : *mut QUERY_SERVICE_CONFIGA, cbbufsize : u32, pcbbytesneeded : *mut u32) -> super::super::Foundation:: BOOL);
    QueryServiceConfigA(hservice.param().abi(), core::mem::transmute(lpserviceconfig.unwrap_or(std::ptr::null_mut())), cbbufsize, pcbbytesneeded).ok()
}
#[cfg(feature = "Win32_Security")]
#[inline]
pub unsafe fn QueryServiceConfigW<P0>(hservice: P0, lpserviceconfig: Option<*mut QUERY_SERVICE_CONFIGW>, cbbufsize: u32, pcbbytesneeded: *mut u32) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::super::Security::SC_HANDLE>,
{
    windows_targets::link!("advapi32.dll" "system" fn QueryServiceConfigW(hservice : super::super::Security:: SC_HANDLE, lpserviceconfig : *mut QUERY_SERVICE_CONFIGW, cbbufsize : u32, pcbbytesneeded : *mut u32) -> super::super::Foundation:: BOOL);
    QueryServiceConfigW(hservice.param().abi(), core::mem::transmute(lpserviceconfig.unwrap_or(std::ptr::null_mut())), cbbufsize, pcbbytesneeded).ok()
}
#[inline]
pub unsafe fn QueryServiceDynamicInformation<P0>(hservicestatus: P0, dwinfolevel: u32, ppdynamicinfo: *mut *mut core::ffi::c_void) -> windows_core::Result<()>
where
    P0: windows_core::Param<SERVICE_STATUS_HANDLE>,
{
    windows_targets::link!("advapi32.dll" "system" fn QueryServiceDynamicInformation(hservicestatus : SERVICE_STATUS_HANDLE, dwinfolevel : u32, ppdynamicinfo : *mut *mut core::ffi::c_void) -> super::super::Foundation:: BOOL);
    QueryServiceDynamicInformation(hservicestatus.param().abi(), dwinfolevel, ppdynamicinfo).ok()
}
#[cfg(feature = "Win32_Security")]
#[inline]
pub unsafe fn QueryServiceLockStatusA<P0>(hscmanager: P0, lplockstatus: Option<*mut QUERY_SERVICE_LOCK_STATUSA>, cbbufsize: u32, pcbbytesneeded: *mut u32) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::super::Security::SC_HANDLE>,
{
    windows_targets::link!("advapi32.dll" "system" fn QueryServiceLockStatusA(hscmanager : super::super::Security:: SC_HANDLE, lplockstatus : *mut QUERY_SERVICE_LOCK_STATUSA, cbbufsize : u32, pcbbytesneeded : *mut u32) -> super::super::Foundation:: BOOL);
    QueryServiceLockStatusA(hscmanager.param().abi(), core::mem::transmute(lplockstatus.unwrap_or(std::ptr::null_mut())), cbbufsize, pcbbytesneeded).ok()
}
#[cfg(feature = "Win32_Security")]
#[inline]
pub unsafe fn QueryServiceLockStatusW<P0>(hscmanager: P0, lplockstatus: Option<*mut QUERY_SERVICE_LOCK_STATUSW>, cbbufsize: u32, pcbbytesneeded: *mut u32) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::super::Security::SC_HANDLE>,
{
    windows_targets::link!("advapi32.dll" "system" fn QueryServiceLockStatusW(hscmanager : super::super::Security:: SC_HANDLE, lplockstatus : *mut QUERY_SERVICE_LOCK_STATUSW, cbbufsize : u32, pcbbytesneeded : *mut u32) -> super::super::Foundation:: BOOL);
    QueryServiceLockStatusW(hscmanager.param().abi(), core::mem::transmute(lplockstatus.unwrap_or(std::ptr::null_mut())), cbbufsize, pcbbytesneeded).ok()
}
#[cfg(feature = "Win32_Security")]
#[inline]
pub unsafe fn QueryServiceObjectSecurity<P0>(hservice: P0, dwsecurityinformation: u32, lpsecuritydescriptor: super::super::Security::PSECURITY_DESCRIPTOR, cbbufsize: u32, pcbbytesneeded: *mut u32) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::super::Security::SC_HANDLE>,
{
    windows_targets::link!("advapi32.dll" "system" fn QueryServiceObjectSecurity(hservice : super::super::Security:: SC_HANDLE, dwsecurityinformation : u32, lpsecuritydescriptor : super::super::Security:: PSECURITY_DESCRIPTOR, cbbufsize : u32, pcbbytesneeded : *mut u32) -> super::super::Foundation:: BOOL);
    QueryServiceObjectSecurity(hservice.param().abi(), dwsecurityinformation, lpsecuritydescriptor, cbbufsize, pcbbytesneeded).ok()
}
#[cfg(feature = "Win32_Security")]
#[inline]
pub unsafe fn QueryServiceStatus<P0>(hservice: P0, lpservicestatus: *mut SERVICE_STATUS) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::super::Security::SC_HANDLE>,
{
    windows_targets::link!("advapi32.dll" "system" fn QueryServiceStatus(hservice : super::super::Security:: SC_HANDLE, lpservicestatus : *mut SERVICE_STATUS) -> super::super::Foundation:: BOOL);
    QueryServiceStatus(hservice.param().abi(), lpservicestatus).ok()
}
#[cfg(feature = "Win32_Security")]
#[inline]
pub unsafe fn QueryServiceStatusEx<P0>(hservice: P0, infolevel: SC_STATUS_TYPE, lpbuffer: Option<&mut [u8]>, pcbbytesneeded: *mut u32) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::super::Security::SC_HANDLE>,
{
    windows_targets::link!("advapi32.dll" "system" fn QueryServiceStatusEx(hservice : super::super::Security:: SC_HANDLE, infolevel : SC_STATUS_TYPE, lpbuffer : *mut u8, cbbufsize : u32, pcbbytesneeded : *mut u32) -> super::super::Foundation:: BOOL);
    QueryServiceStatusEx(hservice.param().abi(), infolevel, core::mem::transmute(lpbuffer.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), lpbuffer.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), pcbbytesneeded).ok()
}
#[inline]
pub unsafe fn RegisterServiceCtrlHandlerA<P0>(lpservicename: P0, lphandlerproc: LPHANDLER_FUNCTION) -> windows_core::Result<SERVICE_STATUS_HANDLE>
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("advapi32.dll" "system" fn RegisterServiceCtrlHandlerA(lpservicename : windows_core::PCSTR, lphandlerproc : LPHANDLER_FUNCTION) -> SERVICE_STATUS_HANDLE);
    let result__ = RegisterServiceCtrlHandlerA(lpservicename.param().abi(), lphandlerproc);
    (!result__.is_invalid()).then(|| result__).ok_or_else(windows_core::Error::from_win32)
}
#[inline]
pub unsafe fn RegisterServiceCtrlHandlerExA<P0>(lpservicename: P0, lphandlerproc: LPHANDLER_FUNCTION_EX, lpcontext: Option<*const core::ffi::c_void>) -> windows_core::Result<SERVICE_STATUS_HANDLE>
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("advapi32.dll" "system" fn RegisterServiceCtrlHandlerExA(lpservicename : windows_core::PCSTR, lphandlerproc : LPHANDLER_FUNCTION_EX, lpcontext : *const core::ffi::c_void) -> SERVICE_STATUS_HANDLE);
    let result__ = RegisterServiceCtrlHandlerExA(lpservicename.param().abi(), lphandlerproc, core::mem::transmute(lpcontext.unwrap_or(std::ptr::null())));
    (!result__.is_invalid()).then(|| result__).ok_or_else(windows_core::Error::from_win32)
}
#[inline]
pub unsafe fn RegisterServiceCtrlHandlerExW<P0>(lpservicename: P0, lphandlerproc: LPHANDLER_FUNCTION_EX, lpcontext: Option<*const core::ffi::c_void>) -> windows_core::Result<SERVICE_STATUS_HANDLE>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("advapi32.dll" "system" fn RegisterServiceCtrlHandlerExW(lpservicename : windows_core::PCWSTR, lphandlerproc : LPHANDLER_FUNCTION_EX, lpcontext : *const core::ffi::c_void) -> SERVICE_STATUS_HANDLE);
    let result__ = RegisterServiceCtrlHandlerExW(lpservicename.param().abi(), lphandlerproc, core::mem::transmute(lpcontext.unwrap_or(std::ptr::null())));
    (!result__.is_invalid()).then(|| result__).ok_or_else(windows_core::Error::from_win32)
}
#[inline]
pub unsafe fn RegisterServiceCtrlHandlerW<P0>(lpservicename: P0, lphandlerproc: LPHANDLER_FUNCTION) -> windows_core::Result<SERVICE_STATUS_HANDLE>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("advapi32.dll" "system" fn RegisterServiceCtrlHandlerW(lpservicename : windows_core::PCWSTR, lphandlerproc : LPHANDLER_FUNCTION) -> SERVICE_STATUS_HANDLE);
    let result__ = RegisterServiceCtrlHandlerW(lpservicename.param().abi(), lphandlerproc);
    (!result__.is_invalid()).then(|| result__).ok_or_else(windows_core::Error::from_win32)
}
#[inline]
pub unsafe fn SetServiceBits<P0, P1, P2>(hservicestatus: P0, dwservicebits: u32, bsetbitson: P1, bupdateimmediately: P2) -> windows_core::Result<()>
where
    P0: windows_core::Param<SERVICE_STATUS_HANDLE>,
    P1: windows_core::Param<super::super::Foundation::BOOL>,
    P2: windows_core::Param<super::super::Foundation::BOOL>,
{
    windows_targets::link!("advapi32.dll" "system" fn SetServiceBits(hservicestatus : SERVICE_STATUS_HANDLE, dwservicebits : u32, bsetbitson : super::super::Foundation:: BOOL, bupdateimmediately : super::super::Foundation:: BOOL) -> super::super::Foundation:: BOOL);
    SetServiceBits(hservicestatus.param().abi(), dwservicebits, bsetbitson.param().abi(), bupdateimmediately.param().abi()).ok()
}
#[cfg(feature = "Win32_Security")]
#[inline]
pub unsafe fn SetServiceObjectSecurity<P0, P1>(hservice: P0, dwsecurityinformation: super::super::Security::OBJECT_SECURITY_INFORMATION, lpsecuritydescriptor: P1) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::super::Security::SC_HANDLE>,
    P1: windows_core::Param<super::super::Security::PSECURITY_DESCRIPTOR>,
{
    windows_targets::link!("advapi32.dll" "system" fn SetServiceObjectSecurity(hservice : super::super::Security:: SC_HANDLE, dwsecurityinformation : super::super::Security:: OBJECT_SECURITY_INFORMATION, lpsecuritydescriptor : super::super::Security:: PSECURITY_DESCRIPTOR) -> super::super::Foundation:: BOOL);
    SetServiceObjectSecurity(hservice.param().abi(), dwsecurityinformation, lpsecuritydescriptor.param().abi()).ok()
}
#[inline]
pub unsafe fn SetServiceStatus<P0>(hservicestatus: P0, lpservicestatus: *const SERVICE_STATUS) -> windows_core::Result<()>
where
    P0: windows_core::Param<SERVICE_STATUS_HANDLE>,
{
    windows_targets::link!("advapi32.dll" "system" fn SetServiceStatus(hservicestatus : SERVICE_STATUS_HANDLE, lpservicestatus : *const SERVICE_STATUS) -> super::super::Foundation:: BOOL);
    SetServiceStatus(hservicestatus.param().abi(), lpservicestatus).ok()
}
#[cfg(feature = "Win32_Security")]
#[inline]
pub unsafe fn StartServiceA<P0>(hservice: P0, lpserviceargvectors: Option<&[windows_core::PCSTR]>) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::super::Security::SC_HANDLE>,
{
    windows_targets::link!("advapi32.dll" "system" fn StartServiceA(hservice : super::super::Security:: SC_HANDLE, dwnumserviceargs : u32, lpserviceargvectors : *const windows_core::PCSTR) -> super::super::Foundation:: BOOL);
    StartServiceA(hservice.param().abi(), lpserviceargvectors.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), core::mem::transmute(lpserviceargvectors.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr()))).ok()
}
#[inline]
pub unsafe fn StartServiceCtrlDispatcherA(lpservicestarttable: *const SERVICE_TABLE_ENTRYA) -> windows_core::Result<()> {
    windows_targets::link!("advapi32.dll" "system" fn StartServiceCtrlDispatcherA(lpservicestarttable : *const SERVICE_TABLE_ENTRYA) -> super::super::Foundation:: BOOL);
    StartServiceCtrlDispatcherA(lpservicestarttable).ok()
}
#[inline]
pub unsafe fn StartServiceCtrlDispatcherW(lpservicestarttable: *const SERVICE_TABLE_ENTRYW) -> windows_core::Result<()> {
    windows_targets::link!("advapi32.dll" "system" fn StartServiceCtrlDispatcherW(lpservicestarttable : *const SERVICE_TABLE_ENTRYW) -> super::super::Foundation:: BOOL);
    StartServiceCtrlDispatcherW(lpservicestarttable).ok()
}
#[cfg(feature = "Win32_Security")]
#[inline]
pub unsafe fn StartServiceW<P0>(hservice: P0, lpserviceargvectors: Option<&[windows_core::PCWSTR]>) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::super::Security::SC_HANDLE>,
{
    windows_targets::link!("advapi32.dll" "system" fn StartServiceW(hservice : super::super::Security:: SC_HANDLE, dwnumserviceargs : u32, lpserviceargvectors : *const windows_core::PCWSTR) -> super::super::Foundation:: BOOL);
    StartServiceW(hservice.param().abi(), lpserviceargvectors.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), core::mem::transmute(lpserviceargvectors.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr()))).ok()
}
#[cfg(feature = "Win32_Security")]
#[inline]
pub unsafe fn SubscribeServiceChangeNotifications<P0>(hservice: P0, eeventtype: SC_EVENT_TYPE, pcallback: PSC_NOTIFICATION_CALLBACK, pcallbackcontext: Option<*const core::ffi::c_void>, psubscription: *mut PSC_NOTIFICATION_REGISTRATION) -> u32
where
    P0: windows_core::Param<super::super::Security::SC_HANDLE>,
{
    windows_targets::link!("sechost.dll" "system" fn SubscribeServiceChangeNotifications(hservice : super::super::Security:: SC_HANDLE, eeventtype : SC_EVENT_TYPE, pcallback : PSC_NOTIFICATION_CALLBACK, pcallbackcontext : *const core::ffi::c_void, psubscription : *mut PSC_NOTIFICATION_REGISTRATION) -> u32);
    SubscribeServiceChangeNotifications(hservice.param().abi(), eeventtype, pcallback, core::mem::transmute(pcallbackcontext.unwrap_or(std::ptr::null())), psubscription)
}
#[inline]
pub unsafe fn UnlockServiceDatabase(sclock: *const core::ffi::c_void) -> windows_core::Result<()> {
    windows_targets::link!("advapi32.dll" "system" fn UnlockServiceDatabase(sclock : *const core::ffi::c_void) -> super::super::Foundation:: BOOL);
    UnlockServiceDatabase(sclock).ok()
}
#[inline]
pub unsafe fn UnsubscribeServiceChangeNotifications<P0>(psubscription: P0)
where
    P0: windows_core::Param<PSC_NOTIFICATION_REGISTRATION>,
{
    windows_targets::link!("sechost.dll" "system" fn UnsubscribeServiceChangeNotifications(psubscription : PSC_NOTIFICATION_REGISTRATION));
    UnsubscribeServiceChangeNotifications(psubscription.param().abi())
}
#[cfg(feature = "Win32_Security")]
#[inline]
pub unsafe fn WaitServiceState<P0, P1>(hservice: P0, dwnotify: u32, dwtimeout: u32, hcancelevent: P1) -> u32
where
    P0: windows_core::Param<super::super::Security::SC_HANDLE>,
    P1: windows_core::Param<super::super::Foundation::HANDLE>,
{
    windows_targets::link!("advapi32.dll" "system" fn WaitServiceState(hservice : super::super::Security:: SC_HANDLE, dwnotify : u32, dwtimeout : u32, hcancelevent : super::super::Foundation:: HANDLE) -> u32);
    WaitServiceState(hservice.param().abi(), dwnotify, dwtimeout, hcancelevent.param().abi())
}
pub const CUSTOM_SYSTEM_STATE_CHANGE_EVENT_GUID: windows_core::GUID = windows_core::GUID::from_u128(0x2d7a2816_0c5e_45fc_9ce7_570e5ecde9c9);
pub const DOMAIN_JOIN_GUID: windows_core::GUID = windows_core::GUID::from_u128(0x1ce20aba_9851_4421_9430_1ddeb766e809);
pub const DOMAIN_LEAVE_GUID: windows_core::GUID = windows_core::GUID::from_u128(0xddaf516e_58c2_4866_9574_c3b615d42ea1);
pub const FIREWALL_PORT_CLOSE_GUID: windows_core::GUID = windows_core::GUID::from_u128(0xa144ed38_8e12_4de4_9d96_e64740b1a524);
pub const FIREWALL_PORT_OPEN_GUID: windows_core::GUID = windows_core::GUID::from_u128(0xb7569e07_8421_4ee0_ad10_86915afdad09);
pub const MACHINE_POLICY_PRESENT_GUID: windows_core::GUID = windows_core::GUID::from_u128(0x659fcae6_5bdb_4da9_b1ff_ca2a178d46e0);
pub const MaxServiceRegistryStateType: SERVICE_REGISTRY_STATE_TYPE = SERVICE_REGISTRY_STATE_TYPE(2i32);
pub const NAMED_PIPE_EVENT_GUID: windows_core::GUID = windows_core::GUID::from_u128(0x1f81d131_3fac_4537_9e0c_7e7b0c2f4b55);
pub const NETWORK_MANAGER_FIRST_IP_ADDRESS_ARRIVAL_GUID: windows_core::GUID = windows_core::GUID::from_u128(0x4f27f2de_14e2_430b_a549_7cd48cbc8245);
pub const NETWORK_MANAGER_LAST_IP_ADDRESS_REMOVAL_GUID: windows_core::GUID = windows_core::GUID::from_u128(0xcc4ba62a_162e_4648_847a_b6bdf993e335);
pub const RPC_INTERFACE_EVENT_GUID: windows_core::GUID = windows_core::GUID::from_u128(0xbc90d167_9470_4139_a9ba_be0bbbf5b74d);
pub const SC_ACTION_NONE: SC_ACTION_TYPE = SC_ACTION_TYPE(0i32);
pub const SC_ACTION_OWN_RESTART: SC_ACTION_TYPE = SC_ACTION_TYPE(4i32);
pub const SC_ACTION_REBOOT: SC_ACTION_TYPE = SC_ACTION_TYPE(2i32);
pub const SC_ACTION_RESTART: SC_ACTION_TYPE = SC_ACTION_TYPE(1i32);
pub const SC_ACTION_RUN_COMMAND: SC_ACTION_TYPE = SC_ACTION_TYPE(3i32);
pub const SC_AGGREGATE_STORAGE_KEY: windows_core::PCWSTR = windows_core::w!("System\\CurrentControlSet\\Control\\ServiceAggregatedEvents");
pub const SC_ENUM_PROCESS_INFO: SC_ENUM_TYPE = SC_ENUM_TYPE(0i32);
pub const SC_EVENT_DATABASE_CHANGE: SC_EVENT_TYPE = SC_EVENT_TYPE(0i32);
pub const SC_EVENT_PROPERTY_CHANGE: SC_EVENT_TYPE = SC_EVENT_TYPE(1i32);
pub const SC_EVENT_STATUS_CHANGE: SC_EVENT_TYPE = SC_EVENT_TYPE(2i32);
pub const SC_MANAGER_ALL_ACCESS: u32 = 983103u32;
pub const SC_MANAGER_CONNECT: u32 = 1u32;
pub const SC_MANAGER_CREATE_SERVICE: u32 = 2u32;
pub const SC_MANAGER_ENUMERATE_SERVICE: u32 = 4u32;
pub const SC_MANAGER_LOCK: u32 = 8u32;
pub const SC_MANAGER_MODIFY_BOOT_CONFIG: u32 = 32u32;
pub const SC_MANAGER_QUERY_LOCK_STATUS: u32 = 16u32;
pub const SC_STATUS_PROCESS_INFO: SC_STATUS_TYPE = SC_STATUS_TYPE(0i32);
pub const SERVICES_ACTIVE_DATABASE: windows_core::PCWSTR = windows_core::w!("ServicesActive");
pub const SERVICES_ACTIVE_DATABASEA: windows_core::PCSTR = windows_core::s!("ServicesActive");
pub const SERVICES_ACTIVE_DATABASEW: windows_core::PCWSTR = windows_core::w!("ServicesActive");
pub const SERVICES_FAILED_DATABASE: windows_core::PCWSTR = windows_core::w!("ServicesFailed");
pub const SERVICES_FAILED_DATABASEA: windows_core::PCSTR = windows_core::s!("ServicesFailed");
pub const SERVICES_FAILED_DATABASEW: windows_core::PCWSTR = windows_core::w!("ServicesFailed");
pub const SERVICE_ACCEPT_HARDWAREPROFILECHANGE: u32 = 32u32;
pub const SERVICE_ACCEPT_LOWRESOURCES: u32 = 8192u32;
pub const SERVICE_ACCEPT_NETBINDCHANGE: u32 = 16u32;
pub const SERVICE_ACCEPT_PARAMCHANGE: u32 = 8u32;
pub const SERVICE_ACCEPT_PAUSE_CONTINUE: u32 = 2u32;
pub const SERVICE_ACCEPT_POWEREVENT: u32 = 64u32;
pub const SERVICE_ACCEPT_PRESHUTDOWN: u32 = 256u32;
pub const SERVICE_ACCEPT_SESSIONCHANGE: u32 = 128u32;
pub const SERVICE_ACCEPT_SHUTDOWN: u32 = 4u32;
pub const SERVICE_ACCEPT_STOP: u32 = 1u32;
pub const SERVICE_ACCEPT_SYSTEMLOWRESOURCES: u32 = 16384u32;
pub const SERVICE_ACCEPT_TIMECHANGE: u32 = 512u32;
pub const SERVICE_ACCEPT_TRIGGEREVENT: u32 = 1024u32;
pub const SERVICE_ACCEPT_USER_LOGOFF: u32 = 2048u32;
pub const SERVICE_ACTIVE: ENUM_SERVICE_STATE = ENUM_SERVICE_STATE(1u32);
pub const SERVICE_ADAPTER: ENUM_SERVICE_TYPE = ENUM_SERVICE_TYPE(4u32);
pub const SERVICE_ALL_ACCESS: u32 = 983551u32;
pub const SERVICE_AUTO_START: SERVICE_START_TYPE = SERVICE_START_TYPE(2u32);
pub const SERVICE_BOOT_START: SERVICE_START_TYPE = SERVICE_START_TYPE(0u32);
pub const SERVICE_CHANGE_CONFIG: u32 = 2u32;
pub const SERVICE_CONFIG_DELAYED_AUTO_START_INFO: SERVICE_CONFIG = SERVICE_CONFIG(3u32);
pub const SERVICE_CONFIG_DESCRIPTION: SERVICE_CONFIG = SERVICE_CONFIG(1u32);
pub const SERVICE_CONFIG_FAILURE_ACTIONS: SERVICE_CONFIG = SERVICE_CONFIG(2u32);
pub const SERVICE_CONFIG_FAILURE_ACTIONS_FLAG: SERVICE_CONFIG = SERVICE_CONFIG(4u32);
pub const SERVICE_CONFIG_LAUNCH_PROTECTED: SERVICE_CONFIG = SERVICE_CONFIG(12u32);
pub const SERVICE_CONFIG_PREFERRED_NODE: SERVICE_CONFIG = SERVICE_CONFIG(9u32);
pub const SERVICE_CONFIG_PRESHUTDOWN_INFO: SERVICE_CONFIG = SERVICE_CONFIG(7u32);
pub const SERVICE_CONFIG_REQUIRED_PRIVILEGES_INFO: SERVICE_CONFIG = SERVICE_CONFIG(6u32);
pub const SERVICE_CONFIG_SERVICE_SID_INFO: SERVICE_CONFIG = SERVICE_CONFIG(5u32);
pub const SERVICE_CONFIG_TRIGGER_INFO: SERVICE_CONFIG = SERVICE_CONFIG(8u32);
pub const SERVICE_CONTINUE_PENDING: SERVICE_STATUS_CURRENT_STATE = SERVICE_STATUS_CURRENT_STATE(5u32);
pub const SERVICE_CONTROL_CONTINUE: u32 = 3u32;
pub const SERVICE_CONTROL_DEVICEEVENT: u32 = 11u32;
pub const SERVICE_CONTROL_HARDWAREPROFILECHANGE: u32 = 12u32;
pub const SERVICE_CONTROL_INTERROGATE: u32 = 4u32;
pub const SERVICE_CONTROL_LOWRESOURCES: u32 = 96u32;
pub const SERVICE_CONTROL_NETBINDADD: u32 = 7u32;
pub const SERVICE_CONTROL_NETBINDDISABLE: u32 = 10u32;
pub const SERVICE_CONTROL_NETBINDENABLE: u32 = 9u32;
pub const SERVICE_CONTROL_NETBINDREMOVE: u32 = 8u32;
pub const SERVICE_CONTROL_PARAMCHANGE: u32 = 6u32;
pub const SERVICE_CONTROL_PAUSE: u32 = 2u32;
pub const SERVICE_CONTROL_POWEREVENT: u32 = 13u32;
pub const SERVICE_CONTROL_PRESHUTDOWN: u32 = 15u32;
pub const SERVICE_CONTROL_SESSIONCHANGE: u32 = 14u32;
pub const SERVICE_CONTROL_SHUTDOWN: u32 = 5u32;
pub const SERVICE_CONTROL_STATUS_REASON_INFO: u32 = 1u32;
pub const SERVICE_CONTROL_STOP: u32 = 1u32;
pub const SERVICE_CONTROL_SYSTEMLOWRESOURCES: u32 = 97u32;
pub const SERVICE_CONTROL_TIMECHANGE: u32 = 16u32;
pub const SERVICE_CONTROL_TRIGGEREVENT: u32 = 32u32;
pub const SERVICE_DEMAND_START: SERVICE_START_TYPE = SERVICE_START_TYPE(3u32);
pub const SERVICE_DISABLED: SERVICE_START_TYPE = SERVICE_START_TYPE(4u32);
pub const SERVICE_DRIVER: ENUM_SERVICE_TYPE = ENUM_SERVICE_TYPE(11u32);
pub const SERVICE_DYNAMIC_INFORMATION_LEVEL_START_REASON: u32 = 1u32;
pub const SERVICE_ENUMERATE_DEPENDENTS: u32 = 8u32;
pub const SERVICE_ERROR_CRITICAL: SERVICE_ERROR = SERVICE_ERROR(3u32);
pub const SERVICE_ERROR_IGNORE: SERVICE_ERROR = SERVICE_ERROR(0u32);
pub const SERVICE_ERROR_NORMAL: SERVICE_ERROR = SERVICE_ERROR(1u32);
pub const SERVICE_ERROR_SEVERE: SERVICE_ERROR = SERVICE_ERROR(2u32);
pub const SERVICE_FILE_SYSTEM_DRIVER: ENUM_SERVICE_TYPE = ENUM_SERVICE_TYPE(2u32);
pub const SERVICE_INACTIVE: ENUM_SERVICE_STATE = ENUM_SERVICE_STATE(2u32);
pub const SERVICE_INTERROGATE: u32 = 128u32;
pub const SERVICE_KERNEL_DRIVER: ENUM_SERVICE_TYPE = ENUM_SERVICE_TYPE(1u32);
pub const SERVICE_LAUNCH_PROTECTED_ANTIMALWARE_LIGHT: u32 = 3u32;
pub const SERVICE_LAUNCH_PROTECTED_NONE: u32 = 0u32;
pub const SERVICE_LAUNCH_PROTECTED_WINDOWS: u32 = 1u32;
pub const SERVICE_LAUNCH_PROTECTED_WINDOWS_LIGHT: u32 = 2u32;
pub const SERVICE_NOTIFY_CONTINUE_PENDING: SERVICE_NOTIFY = SERVICE_NOTIFY(16u32);
pub const SERVICE_NOTIFY_CREATED: SERVICE_NOTIFY = SERVICE_NOTIFY(128u32);
pub const SERVICE_NOTIFY_DELETED: SERVICE_NOTIFY = SERVICE_NOTIFY(256u32);
pub const SERVICE_NOTIFY_DELETE_PENDING: SERVICE_NOTIFY = SERVICE_NOTIFY(512u32);
pub const SERVICE_NOTIFY_PAUSED: SERVICE_NOTIFY = SERVICE_NOTIFY(64u32);
pub const SERVICE_NOTIFY_PAUSE_PENDING: SERVICE_NOTIFY = SERVICE_NOTIFY(32u32);
pub const SERVICE_NOTIFY_RUNNING: SERVICE_NOTIFY = SERVICE_NOTIFY(8u32);
pub const SERVICE_NOTIFY_START_PENDING: SERVICE_NOTIFY = SERVICE_NOTIFY(2u32);
pub const SERVICE_NOTIFY_STATUS_CHANGE: u32 = 2u32;
pub const SERVICE_NOTIFY_STATUS_CHANGE_1: u32 = 1u32;
pub const SERVICE_NOTIFY_STATUS_CHANGE_2: u32 = 2u32;
pub const SERVICE_NOTIFY_STOPPED: SERVICE_NOTIFY = SERVICE_NOTIFY(1u32);
pub const SERVICE_NOTIFY_STOP_PENDING: SERVICE_NOTIFY = SERVICE_NOTIFY(4u32);
pub const SERVICE_NO_CHANGE: u32 = 4294967295u32;
pub const SERVICE_PAUSED: SERVICE_STATUS_CURRENT_STATE = SERVICE_STATUS_CURRENT_STATE(7u32);
pub const SERVICE_PAUSE_CONTINUE: u32 = 64u32;
pub const SERVICE_PAUSE_PENDING: SERVICE_STATUS_CURRENT_STATE = SERVICE_STATUS_CURRENT_STATE(6u32);
pub const SERVICE_QUERY_CONFIG: u32 = 1u32;
pub const SERVICE_QUERY_STATUS: u32 = 4u32;
pub const SERVICE_RECOGNIZER_DRIVER: ENUM_SERVICE_TYPE = ENUM_SERVICE_TYPE(8u32);
pub const SERVICE_RUNNING: SERVICE_STATUS_CURRENT_STATE = SERVICE_STATUS_CURRENT_STATE(4u32);
pub const SERVICE_RUNS_IN_NON_SYSTEM_OR_NOT_RUNNING: SERVICE_RUNS_IN_PROCESS = SERVICE_RUNS_IN_PROCESS(0u32);
pub const SERVICE_RUNS_IN_SYSTEM_PROCESS: SERVICE_RUNS_IN_PROCESS = SERVICE_RUNS_IN_PROCESS(1u32);
pub const SERVICE_SID_TYPE_NONE: u32 = 0u32;
pub const SERVICE_SID_TYPE_UNRESTRICTED: u32 = 1u32;
pub const SERVICE_START: u32 = 16u32;
pub const SERVICE_START_PENDING: SERVICE_STATUS_CURRENT_STATE = SERVICE_STATUS_CURRENT_STATE(2u32);
pub const SERVICE_START_REASON_AUTO: u32 = 2u32;
pub const SERVICE_START_REASON_DELAYEDAUTO: u32 = 16u32;
pub const SERVICE_START_REASON_DEMAND: u32 = 1u32;
pub const SERVICE_START_REASON_RESTART_ON_FAILURE: u32 = 8u32;
pub const SERVICE_START_REASON_TRIGGER: u32 = 4u32;
pub const SERVICE_STATE_ALL: ENUM_SERVICE_STATE = ENUM_SERVICE_STATE(3u32);
pub const SERVICE_STOP: u32 = 32u32;
pub const SERVICE_STOPPED: SERVICE_STATUS_CURRENT_STATE = SERVICE_STATUS_CURRENT_STATE(1u32);
pub const SERVICE_STOP_PENDING: SERVICE_STATUS_CURRENT_STATE = SERVICE_STATUS_CURRENT_STATE(3u32);
pub const SERVICE_STOP_REASON_FLAG_CUSTOM: u32 = 536870912u32;
pub const SERVICE_STOP_REASON_FLAG_MAX: u32 = 2147483648u32;
pub const SERVICE_STOP_REASON_FLAG_MIN: u32 = 0u32;
pub const SERVICE_STOP_REASON_FLAG_PLANNED: u32 = 1073741824u32;
pub const SERVICE_STOP_REASON_FLAG_UNPLANNED: u32 = 268435456u32;
pub const SERVICE_STOP_REASON_MAJOR_APPLICATION: u32 = 327680u32;
pub const SERVICE_STOP_REASON_MAJOR_HARDWARE: u32 = 131072u32;
pub const SERVICE_STOP_REASON_MAJOR_MAX: u32 = 458752u32;
pub const SERVICE_STOP_REASON_MAJOR_MAX_CUSTOM: u32 = 16711680u32;
pub const SERVICE_STOP_REASON_MAJOR_MIN: u32 = 0u32;
pub const SERVICE_STOP_REASON_MAJOR_MIN_CUSTOM: u32 = 4194304u32;
pub const SERVICE_STOP_REASON_MAJOR_NONE: u32 = 393216u32;
pub const SERVICE_STOP_REASON_MAJOR_OPERATINGSYSTEM: u32 = 196608u32;
pub const SERVICE_STOP_REASON_MAJOR_OTHER: u32 = 65536u32;
pub const SERVICE_STOP_REASON_MAJOR_SOFTWARE: u32 = 262144u32;
pub const SERVICE_STOP_REASON_MINOR_DISK: u32 = 8u32;
pub const SERVICE_STOP_REASON_MINOR_ENVIRONMENT: u32 = 10u32;
pub const SERVICE_STOP_REASON_MINOR_HARDWARE_DRIVER: u32 = 11u32;
pub const SERVICE_STOP_REASON_MINOR_HUNG: u32 = 6u32;
pub const SERVICE_STOP_REASON_MINOR_INSTALLATION: u32 = 3u32;
pub const SERVICE_STOP_REASON_MINOR_MAINTENANCE: u32 = 2u32;
pub const SERVICE_STOP_REASON_MINOR_MAX: u32 = 25u32;
pub const SERVICE_STOP_REASON_MINOR_MAX_CUSTOM: u32 = 65535u32;
pub const SERVICE_STOP_REASON_MINOR_MEMOTYLIMIT: u32 = 24u32;
pub const SERVICE_STOP_REASON_MINOR_MIN: u32 = 0u32;
pub const SERVICE_STOP_REASON_MINOR_MIN_CUSTOM: u32 = 256u32;
pub const SERVICE_STOP_REASON_MINOR_MMC: u32 = 22u32;
pub const SERVICE_STOP_REASON_MINOR_NETWORKCARD: u32 = 9u32;
pub const SERVICE_STOP_REASON_MINOR_NETWORK_CONNECTIVITY: u32 = 17u32;
pub const SERVICE_STOP_REASON_MINOR_NONE: u32 = 23u32;
pub const SERVICE_STOP_REASON_MINOR_OTHER: u32 = 1u32;
pub const SERVICE_STOP_REASON_MINOR_OTHERDRIVER: u32 = 12u32;
pub const SERVICE_STOP_REASON_MINOR_RECONFIG: u32 = 5u32;
pub const SERVICE_STOP_REASON_MINOR_SECURITY: u32 = 16u32;
pub const SERVICE_STOP_REASON_MINOR_SECURITYFIX: u32 = 15u32;
pub const SERVICE_STOP_REASON_MINOR_SECURITYFIX_UNINSTALL: u32 = 21u32;
pub const SERVICE_STOP_REASON_MINOR_SERVICEPACK: u32 = 13u32;
pub const SERVICE_STOP_REASON_MINOR_SERVICEPACK_UNINSTALL: u32 = 19u32;
pub const SERVICE_STOP_REASON_MINOR_SOFTWARE_UPDATE: u32 = 14u32;
pub const SERVICE_STOP_REASON_MINOR_SOFTWARE_UPDATE_UNINSTALL: u32 = 20u32;
pub const SERVICE_STOP_REASON_MINOR_UNSTABLE: u32 = 7u32;
pub const SERVICE_STOP_REASON_MINOR_UPGRADE: u32 = 4u32;
pub const SERVICE_STOP_REASON_MINOR_WMI: u32 = 18u32;
pub const SERVICE_SYSTEM_START: SERVICE_START_TYPE = SERVICE_START_TYPE(1u32);
pub const SERVICE_TRIGGER_ACTION_SERVICE_START: SERVICE_TRIGGER_ACTION = SERVICE_TRIGGER_ACTION(1u32);
pub const SERVICE_TRIGGER_ACTION_SERVICE_STOP: SERVICE_TRIGGER_ACTION = SERVICE_TRIGGER_ACTION(2u32);
pub const SERVICE_TRIGGER_DATA_TYPE_BINARY: SERVICE_TRIGGER_SPECIFIC_DATA_ITEM_DATA_TYPE = SERVICE_TRIGGER_SPECIFIC_DATA_ITEM_DATA_TYPE(1u32);
pub const SERVICE_TRIGGER_DATA_TYPE_KEYWORD_ALL: SERVICE_TRIGGER_SPECIFIC_DATA_ITEM_DATA_TYPE = SERVICE_TRIGGER_SPECIFIC_DATA_ITEM_DATA_TYPE(5u32);
pub const SERVICE_TRIGGER_DATA_TYPE_KEYWORD_ANY: SERVICE_TRIGGER_SPECIFIC_DATA_ITEM_DATA_TYPE = SERVICE_TRIGGER_SPECIFIC_DATA_ITEM_DATA_TYPE(4u32);
pub const SERVICE_TRIGGER_DATA_TYPE_LEVEL: SERVICE_TRIGGER_SPECIFIC_DATA_ITEM_DATA_TYPE = SERVICE_TRIGGER_SPECIFIC_DATA_ITEM_DATA_TYPE(3u32);
pub const SERVICE_TRIGGER_DATA_TYPE_STRING: SERVICE_TRIGGER_SPECIFIC_DATA_ITEM_DATA_TYPE = SERVICE_TRIGGER_SPECIFIC_DATA_ITEM_DATA_TYPE(2u32);
pub const SERVICE_TRIGGER_STARTED_ARGUMENT: windows_core::PCWSTR = windows_core::w!("TriggerStarted");
pub const SERVICE_TRIGGER_TYPE_AGGREGATE: u32 = 30u32;
pub const SERVICE_TRIGGER_TYPE_CUSTOM: SERVICE_TRIGGER_TYPE = SERVICE_TRIGGER_TYPE(20u32);
pub const SERVICE_TRIGGER_TYPE_CUSTOM_SYSTEM_STATE_CHANGE: u32 = 7u32;
pub const SERVICE_TRIGGER_TYPE_DEVICE_INTERFACE_ARRIVAL: SERVICE_TRIGGER_TYPE = SERVICE_TRIGGER_TYPE(1u32);
pub const SERVICE_TRIGGER_TYPE_DOMAIN_JOIN: SERVICE_TRIGGER_TYPE = SERVICE_TRIGGER_TYPE(3u32);
pub const SERVICE_TRIGGER_TYPE_FIREWALL_PORT_EVENT: SERVICE_TRIGGER_TYPE = SERVICE_TRIGGER_TYPE(4u32);
pub const SERVICE_TRIGGER_TYPE_GROUP_POLICY: SERVICE_TRIGGER_TYPE = SERVICE_TRIGGER_TYPE(5u32);
pub const SERVICE_TRIGGER_TYPE_IP_ADDRESS_AVAILABILITY: SERVICE_TRIGGER_TYPE = SERVICE_TRIGGER_TYPE(2u32);
pub const SERVICE_TRIGGER_TYPE_NETWORK_ENDPOINT: SERVICE_TRIGGER_TYPE = SERVICE_TRIGGER_TYPE(6u32);
pub const SERVICE_USER_DEFINED_CONTROL: u32 = 256u32;
pub const SERVICE_USER_OWN_PROCESS: ENUM_SERVICE_TYPE = ENUM_SERVICE_TYPE(80u32);
pub const SERVICE_USER_SHARE_PROCESS: ENUM_SERVICE_TYPE = ENUM_SERVICE_TYPE(96u32);
pub const SERVICE_WIN32: ENUM_SERVICE_TYPE = ENUM_SERVICE_TYPE(48u32);
pub const SERVICE_WIN32_OWN_PROCESS: ENUM_SERVICE_TYPE = ENUM_SERVICE_TYPE(16u32);
pub const SERVICE_WIN32_SHARE_PROCESS: ENUM_SERVICE_TYPE = ENUM_SERVICE_TYPE(32u32);
pub const ServiceDirectoryPersistentState: SERVICE_DIRECTORY_TYPE = SERVICE_DIRECTORY_TYPE(0i32);
pub const ServiceDirectoryTypeMax: SERVICE_DIRECTORY_TYPE = SERVICE_DIRECTORY_TYPE(1i32);
pub const ServiceRegistryStateParameters: SERVICE_REGISTRY_STATE_TYPE = SERVICE_REGISTRY_STATE_TYPE(0i32);
pub const ServiceRegistryStatePersistent: SERVICE_REGISTRY_STATE_TYPE = SERVICE_REGISTRY_STATE_TYPE(1i32);
pub const ServiceSharedDirectoryPersistentState: SERVICE_SHARED_DIRECTORY_TYPE = SERVICE_SHARED_DIRECTORY_TYPE(0i32);
pub const ServiceSharedRegistryPersistentState: SERVICE_SHARED_REGISTRY_STATE_TYPE = SERVICE_SHARED_REGISTRY_STATE_TYPE(0i32);
pub const USER_POLICY_PRESENT_GUID: windows_core::GUID = windows_core::GUID::from_u128(0x54fb46c8_f089_464c_b1fd_59d1b62c3b50);
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct ENUM_SERVICE_STATE(pub u32);
impl windows_core::TypeKind for ENUM_SERVICE_STATE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for ENUM_SERVICE_STATE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("ENUM_SERVICE_STATE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct ENUM_SERVICE_TYPE(pub u32);
impl windows_core::TypeKind for ENUM_SERVICE_TYPE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for ENUM_SERVICE_TYPE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("ENUM_SERVICE_TYPE").field(&self.0).finish()
    }
}
impl ENUM_SERVICE_TYPE {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for ENUM_SERVICE_TYPE {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for ENUM_SERVICE_TYPE {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for ENUM_SERVICE_TYPE {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for ENUM_SERVICE_TYPE {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for ENUM_SERVICE_TYPE {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct SC_ACTION_TYPE(pub i32);
impl windows_core::TypeKind for SC_ACTION_TYPE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for SC_ACTION_TYPE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("SC_ACTION_TYPE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct SC_ENUM_TYPE(pub i32);
impl windows_core::TypeKind for SC_ENUM_TYPE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for SC_ENUM_TYPE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("SC_ENUM_TYPE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct SC_EVENT_TYPE(pub i32);
impl windows_core::TypeKind for SC_EVENT_TYPE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for SC_EVENT_TYPE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("SC_EVENT_TYPE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct SC_STATUS_TYPE(pub i32);
impl windows_core::TypeKind for SC_STATUS_TYPE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for SC_STATUS_TYPE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("SC_STATUS_TYPE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct SERVICE_CONFIG(pub u32);
impl windows_core::TypeKind for SERVICE_CONFIG {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for SERVICE_CONFIG {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("SERVICE_CONFIG").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct SERVICE_DIRECTORY_TYPE(pub i32);
impl windows_core::TypeKind for SERVICE_DIRECTORY_TYPE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for SERVICE_DIRECTORY_TYPE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("SERVICE_DIRECTORY_TYPE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct SERVICE_ERROR(pub u32);
impl windows_core::TypeKind for SERVICE_ERROR {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for SERVICE_ERROR {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("SERVICE_ERROR").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct SERVICE_NOTIFY(pub u32);
impl windows_core::TypeKind for SERVICE_NOTIFY {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for SERVICE_NOTIFY {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("SERVICE_NOTIFY").field(&self.0).finish()
    }
}
impl SERVICE_NOTIFY {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for SERVICE_NOTIFY {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for SERVICE_NOTIFY {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for SERVICE_NOTIFY {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for SERVICE_NOTIFY {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for SERVICE_NOTIFY {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct SERVICE_REGISTRY_STATE_TYPE(pub i32);
impl windows_core::TypeKind for SERVICE_REGISTRY_STATE_TYPE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for SERVICE_REGISTRY_STATE_TYPE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("SERVICE_REGISTRY_STATE_TYPE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct SERVICE_RUNS_IN_PROCESS(pub u32);
impl windows_core::TypeKind for SERVICE_RUNS_IN_PROCESS {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for SERVICE_RUNS_IN_PROCESS {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("SERVICE_RUNS_IN_PROCESS").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct SERVICE_SHARED_DIRECTORY_TYPE(pub i32);
impl windows_core::TypeKind for SERVICE_SHARED_DIRECTORY_TYPE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for SERVICE_SHARED_DIRECTORY_TYPE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("SERVICE_SHARED_DIRECTORY_TYPE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct SERVICE_SHARED_REGISTRY_STATE_TYPE(pub i32);
impl windows_core::TypeKind for SERVICE_SHARED_REGISTRY_STATE_TYPE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for SERVICE_SHARED_REGISTRY_STATE_TYPE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("SERVICE_SHARED_REGISTRY_STATE_TYPE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct SERVICE_START_TYPE(pub u32);
impl windows_core::TypeKind for SERVICE_START_TYPE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for SERVICE_START_TYPE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("SERVICE_START_TYPE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct SERVICE_STATUS_CURRENT_STATE(pub u32);
impl windows_core::TypeKind for SERVICE_STATUS_CURRENT_STATE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for SERVICE_STATUS_CURRENT_STATE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("SERVICE_STATUS_CURRENT_STATE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct SERVICE_TRIGGER_ACTION(pub u32);
impl windows_core::TypeKind for SERVICE_TRIGGER_ACTION {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for SERVICE_TRIGGER_ACTION {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("SERVICE_TRIGGER_ACTION").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct SERVICE_TRIGGER_SPECIFIC_DATA_ITEM_DATA_TYPE(pub u32);
impl windows_core::TypeKind for SERVICE_TRIGGER_SPECIFIC_DATA_ITEM_DATA_TYPE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for SERVICE_TRIGGER_SPECIFIC_DATA_ITEM_DATA_TYPE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("SERVICE_TRIGGER_SPECIFIC_DATA_ITEM_DATA_TYPE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct SERVICE_TRIGGER_TYPE(pub u32);
impl windows_core::TypeKind for SERVICE_TRIGGER_TYPE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for SERVICE_TRIGGER_TYPE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("SERVICE_TRIGGER_TYPE").field(&self.0).finish()
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ENUM_SERVICE_STATUSA {
    pub lpServiceName: windows_core::PSTR,
    pub lpDisplayName: windows_core::PSTR,
    pub ServiceStatus: SERVICE_STATUS,
}
impl windows_core::TypeKind for ENUM_SERVICE_STATUSA {
    type TypeKind = windows_core::CopyType;
}
impl Default for ENUM_SERVICE_STATUSA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ENUM_SERVICE_STATUSW {
    pub lpServiceName: windows_core::PWSTR,
    pub lpDisplayName: windows_core::PWSTR,
    pub ServiceStatus: SERVICE_STATUS,
}
impl windows_core::TypeKind for ENUM_SERVICE_STATUSW {
    type TypeKind = windows_core::CopyType;
}
impl Default for ENUM_SERVICE_STATUSW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ENUM_SERVICE_STATUS_PROCESSA {
    pub lpServiceName: windows_core::PSTR,
    pub lpDisplayName: windows_core::PSTR,
    pub ServiceStatusProcess: SERVICE_STATUS_PROCESS,
}
impl windows_core::TypeKind for ENUM_SERVICE_STATUS_PROCESSA {
    type TypeKind = windows_core::CopyType;
}
impl Default for ENUM_SERVICE_STATUS_PROCESSA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ENUM_SERVICE_STATUS_PROCESSW {
    pub lpServiceName: windows_core::PWSTR,
    pub lpDisplayName: windows_core::PWSTR,
    pub ServiceStatusProcess: SERVICE_STATUS_PROCESS,
}
impl windows_core::TypeKind for ENUM_SERVICE_STATUS_PROCESSW {
    type TypeKind = windows_core::CopyType;
}
impl Default for ENUM_SERVICE_STATUS_PROCESSW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PSC_NOTIFICATION_REGISTRATION(pub isize);
impl Default for PSC_NOTIFICATION_REGISTRATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for PSC_NOTIFICATION_REGISTRATION {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct QUERY_SERVICE_CONFIGA {
    pub dwServiceType: ENUM_SERVICE_TYPE,
    pub dwStartType: SERVICE_START_TYPE,
    pub dwErrorControl: SERVICE_ERROR,
    pub lpBinaryPathName: windows_core::PSTR,
    pub lpLoadOrderGroup: windows_core::PSTR,
    pub dwTagId: u32,
    pub lpDependencies: windows_core::PSTR,
    pub lpServiceStartName: windows_core::PSTR,
    pub lpDisplayName: windows_core::PSTR,
}
impl windows_core::TypeKind for QUERY_SERVICE_CONFIGA {
    type TypeKind = windows_core::CopyType;
}
impl Default for QUERY_SERVICE_CONFIGA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct QUERY_SERVICE_CONFIGW {
    pub dwServiceType: ENUM_SERVICE_TYPE,
    pub dwStartType: SERVICE_START_TYPE,
    pub dwErrorControl: SERVICE_ERROR,
    pub lpBinaryPathName: windows_core::PWSTR,
    pub lpLoadOrderGroup: windows_core::PWSTR,
    pub dwTagId: u32,
    pub lpDependencies: windows_core::PWSTR,
    pub lpServiceStartName: windows_core::PWSTR,
    pub lpDisplayName: windows_core::PWSTR,
}
impl windows_core::TypeKind for QUERY_SERVICE_CONFIGW {
    type TypeKind = windows_core::CopyType;
}
impl Default for QUERY_SERVICE_CONFIGW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct QUERY_SERVICE_LOCK_STATUSA {
    pub fIsLocked: u32,
    pub lpLockOwner: windows_core::PSTR,
    pub dwLockDuration: u32,
}
impl windows_core::TypeKind for QUERY_SERVICE_LOCK_STATUSA {
    type TypeKind = windows_core::CopyType;
}
impl Default for QUERY_SERVICE_LOCK_STATUSA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct QUERY_SERVICE_LOCK_STATUSW {
    pub fIsLocked: u32,
    pub lpLockOwner: windows_core::PWSTR,
    pub dwLockDuration: u32,
}
impl windows_core::TypeKind for QUERY_SERVICE_LOCK_STATUSW {
    type TypeKind = windows_core::CopyType;
}
impl Default for QUERY_SERVICE_LOCK_STATUSW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SC_ACTION {
    pub Type: SC_ACTION_TYPE,
    pub Delay: u32,
}
impl windows_core::TypeKind for SC_ACTION {
    type TypeKind = windows_core::CopyType;
}
impl Default for SC_ACTION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SERVICE_CONTROL_STATUS_REASON_PARAMSA {
    pub dwReason: u32,
    pub pszComment: windows_core::PSTR,
    pub ServiceStatus: SERVICE_STATUS_PROCESS,
}
impl windows_core::TypeKind for SERVICE_CONTROL_STATUS_REASON_PARAMSA {
    type TypeKind = windows_core::CopyType;
}
impl Default for SERVICE_CONTROL_STATUS_REASON_PARAMSA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SERVICE_CONTROL_STATUS_REASON_PARAMSW {
    pub dwReason: u32,
    pub pszComment: windows_core::PWSTR,
    pub ServiceStatus: SERVICE_STATUS_PROCESS,
}
impl windows_core::TypeKind for SERVICE_CONTROL_STATUS_REASON_PARAMSW {
    type TypeKind = windows_core::CopyType;
}
impl Default for SERVICE_CONTROL_STATUS_REASON_PARAMSW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct SERVICE_CUSTOM_SYSTEM_STATE_CHANGE_DATA_ITEM {
    pub u: SERVICE_CUSTOM_SYSTEM_STATE_CHANGE_DATA_ITEM_0,
}
impl windows_core::TypeKind for SERVICE_CUSTOM_SYSTEM_STATE_CHANGE_DATA_ITEM {
    type TypeKind = windows_core::CopyType;
}
impl Default for SERVICE_CUSTOM_SYSTEM_STATE_CHANGE_DATA_ITEM {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union SERVICE_CUSTOM_SYSTEM_STATE_CHANGE_DATA_ITEM_0 {
    pub CustomStateId: SERVICE_TRIGGER_CUSTOM_STATE_ID,
    pub s: SERVICE_CUSTOM_SYSTEM_STATE_CHANGE_DATA_ITEM_0_0,
}
impl windows_core::TypeKind for SERVICE_CUSTOM_SYSTEM_STATE_CHANGE_DATA_ITEM_0 {
    type TypeKind = windows_core::CopyType;
}
impl Default for SERVICE_CUSTOM_SYSTEM_STATE_CHANGE_DATA_ITEM_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SERVICE_CUSTOM_SYSTEM_STATE_CHANGE_DATA_ITEM_0_0 {
    pub DataOffset: u32,
    pub Data: [u8; 1],
}
impl windows_core::TypeKind for SERVICE_CUSTOM_SYSTEM_STATE_CHANGE_DATA_ITEM_0_0 {
    type TypeKind = windows_core::CopyType;
}
impl Default for SERVICE_CUSTOM_SYSTEM_STATE_CHANGE_DATA_ITEM_0_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SERVICE_DELAYED_AUTO_START_INFO {
    pub fDelayedAutostart: super::super::Foundation::BOOL,
}
impl windows_core::TypeKind for SERVICE_DELAYED_AUTO_START_INFO {
    type TypeKind = windows_core::CopyType;
}
impl Default for SERVICE_DELAYED_AUTO_START_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SERVICE_DESCRIPTIONA {
    pub lpDescription: windows_core::PSTR,
}
impl windows_core::TypeKind for SERVICE_DESCRIPTIONA {
    type TypeKind = windows_core::CopyType;
}
impl Default for SERVICE_DESCRIPTIONA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SERVICE_DESCRIPTIONW {
    pub lpDescription: windows_core::PWSTR,
}
impl windows_core::TypeKind for SERVICE_DESCRIPTIONW {
    type TypeKind = windows_core::CopyType;
}
impl Default for SERVICE_DESCRIPTIONW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SERVICE_FAILURE_ACTIONSA {
    pub dwResetPeriod: u32,
    pub lpRebootMsg: windows_core::PSTR,
    pub lpCommand: windows_core::PSTR,
    pub cActions: u32,
    pub lpsaActions: *mut SC_ACTION,
}
impl windows_core::TypeKind for SERVICE_FAILURE_ACTIONSA {
    type TypeKind = windows_core::CopyType;
}
impl Default for SERVICE_FAILURE_ACTIONSA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SERVICE_FAILURE_ACTIONSW {
    pub dwResetPeriod: u32,
    pub lpRebootMsg: windows_core::PWSTR,
    pub lpCommand: windows_core::PWSTR,
    pub cActions: u32,
    pub lpsaActions: *mut SC_ACTION,
}
impl windows_core::TypeKind for SERVICE_FAILURE_ACTIONSW {
    type TypeKind = windows_core::CopyType;
}
impl Default for SERVICE_FAILURE_ACTIONSW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SERVICE_FAILURE_ACTIONS_FLAG {
    pub fFailureActionsOnNonCrashFailures: super::super::Foundation::BOOL,
}
impl windows_core::TypeKind for SERVICE_FAILURE_ACTIONS_FLAG {
    type TypeKind = windows_core::CopyType;
}
impl Default for SERVICE_FAILURE_ACTIONS_FLAG {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SERVICE_LAUNCH_PROTECTED_INFO {
    pub dwLaunchProtected: u32,
}
impl windows_core::TypeKind for SERVICE_LAUNCH_PROTECTED_INFO {
    type TypeKind = windows_core::CopyType;
}
impl Default for SERVICE_LAUNCH_PROTECTED_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct SERVICE_NOTIFY_1 {
    pub dwVersion: u32,
    pub pfnNotifyCallback: PFN_SC_NOTIFY_CALLBACK,
    pub pContext: *mut core::ffi::c_void,
    pub dwNotificationStatus: u32,
    pub ServiceStatus: SERVICE_STATUS_PROCESS,
}
impl windows_core::TypeKind for SERVICE_NOTIFY_1 {
    type TypeKind = windows_core::CopyType;
}
impl Default for SERVICE_NOTIFY_1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct SERVICE_NOTIFY_2A {
    pub dwVersion: u32,
    pub pfnNotifyCallback: PFN_SC_NOTIFY_CALLBACK,
    pub pContext: *mut core::ffi::c_void,
    pub dwNotificationStatus: u32,
    pub ServiceStatus: SERVICE_STATUS_PROCESS,
    pub dwNotificationTriggered: u32,
    pub pszServiceNames: windows_core::PSTR,
}
impl windows_core::TypeKind for SERVICE_NOTIFY_2A {
    type TypeKind = windows_core::CopyType;
}
impl Default for SERVICE_NOTIFY_2A {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct SERVICE_NOTIFY_2W {
    pub dwVersion: u32,
    pub pfnNotifyCallback: PFN_SC_NOTIFY_CALLBACK,
    pub pContext: *mut core::ffi::c_void,
    pub dwNotificationStatus: u32,
    pub ServiceStatus: SERVICE_STATUS_PROCESS,
    pub dwNotificationTriggered: u32,
    pub pszServiceNames: windows_core::PWSTR,
}
impl windows_core::TypeKind for SERVICE_NOTIFY_2W {
    type TypeKind = windows_core::CopyType;
}
impl Default for SERVICE_NOTIFY_2W {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SERVICE_PREFERRED_NODE_INFO {
    pub usPreferredNode: u16,
    pub fDelete: super::super::Foundation::BOOLEAN,
}
impl windows_core::TypeKind for SERVICE_PREFERRED_NODE_INFO {
    type TypeKind = windows_core::CopyType;
}
impl Default for SERVICE_PREFERRED_NODE_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SERVICE_PRESHUTDOWN_INFO {
    pub dwPreshutdownTimeout: u32,
}
impl windows_core::TypeKind for SERVICE_PRESHUTDOWN_INFO {
    type TypeKind = windows_core::CopyType;
}
impl Default for SERVICE_PRESHUTDOWN_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SERVICE_REQUIRED_PRIVILEGES_INFOA {
    pub pmszRequiredPrivileges: windows_core::PSTR,
}
impl windows_core::TypeKind for SERVICE_REQUIRED_PRIVILEGES_INFOA {
    type TypeKind = windows_core::CopyType;
}
impl Default for SERVICE_REQUIRED_PRIVILEGES_INFOA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SERVICE_REQUIRED_PRIVILEGES_INFOW {
    pub pmszRequiredPrivileges: windows_core::PWSTR,
}
impl windows_core::TypeKind for SERVICE_REQUIRED_PRIVILEGES_INFOW {
    type TypeKind = windows_core::CopyType;
}
impl Default for SERVICE_REQUIRED_PRIVILEGES_INFOW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SERVICE_SID_INFO {
    pub dwServiceSidType: u32,
}
impl windows_core::TypeKind for SERVICE_SID_INFO {
    type TypeKind = windows_core::CopyType;
}
impl Default for SERVICE_SID_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SERVICE_START_REASON {
    pub dwReason: u32,
}
impl windows_core::TypeKind for SERVICE_START_REASON {
    type TypeKind = windows_core::CopyType;
}
impl Default for SERVICE_START_REASON {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SERVICE_STATUS {
    pub dwServiceType: ENUM_SERVICE_TYPE,
    pub dwCurrentState: SERVICE_STATUS_CURRENT_STATE,
    pub dwControlsAccepted: u32,
    pub dwWin32ExitCode: u32,
    pub dwServiceSpecificExitCode: u32,
    pub dwCheckPoint: u32,
    pub dwWaitHint: u32,
}
impl windows_core::TypeKind for SERVICE_STATUS {
    type TypeKind = windows_core::CopyType;
}
impl Default for SERVICE_STATUS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SERVICE_STATUS_HANDLE(pub isize);
impl SERVICE_STATUS_HANDLE {
    pub fn is_invalid(&self) -> bool {
        self.0 == -1 || self.0 == 0
    }
}
impl Default for SERVICE_STATUS_HANDLE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for SERVICE_STATUS_HANDLE {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SERVICE_STATUS_PROCESS {
    pub dwServiceType: ENUM_SERVICE_TYPE,
    pub dwCurrentState: SERVICE_STATUS_CURRENT_STATE,
    pub dwControlsAccepted: u32,
    pub dwWin32ExitCode: u32,
    pub dwServiceSpecificExitCode: u32,
    pub dwCheckPoint: u32,
    pub dwWaitHint: u32,
    pub dwProcessId: u32,
    pub dwServiceFlags: SERVICE_RUNS_IN_PROCESS,
}
impl windows_core::TypeKind for SERVICE_STATUS_PROCESS {
    type TypeKind = windows_core::CopyType;
}
impl Default for SERVICE_STATUS_PROCESS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct SERVICE_TABLE_ENTRYA {
    pub lpServiceName: windows_core::PSTR,
    pub lpServiceProc: LPSERVICE_MAIN_FUNCTIONA,
}
impl windows_core::TypeKind for SERVICE_TABLE_ENTRYA {
    type TypeKind = windows_core::CopyType;
}
impl Default for SERVICE_TABLE_ENTRYA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct SERVICE_TABLE_ENTRYW {
    pub lpServiceName: windows_core::PWSTR,
    pub lpServiceProc: LPSERVICE_MAIN_FUNCTIONW,
}
impl windows_core::TypeKind for SERVICE_TABLE_ENTRYW {
    type TypeKind = windows_core::CopyType;
}
impl Default for SERVICE_TABLE_ENTRYW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SERVICE_TIMECHANGE_INFO {
    pub liNewTime: i64,
    pub liOldTime: i64,
}
impl windows_core::TypeKind for SERVICE_TIMECHANGE_INFO {
    type TypeKind = windows_core::CopyType;
}
impl Default for SERVICE_TIMECHANGE_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SERVICE_TRIGGER {
    pub dwTriggerType: SERVICE_TRIGGER_TYPE,
    pub dwAction: SERVICE_TRIGGER_ACTION,
    pub pTriggerSubtype: *mut windows_core::GUID,
    pub cDataItems: u32,
    pub pDataItems: *mut SERVICE_TRIGGER_SPECIFIC_DATA_ITEM,
}
impl windows_core::TypeKind for SERVICE_TRIGGER {
    type TypeKind = windows_core::CopyType;
}
impl Default for SERVICE_TRIGGER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SERVICE_TRIGGER_CUSTOM_STATE_ID {
    pub Data: [u32; 2],
}
impl windows_core::TypeKind for SERVICE_TRIGGER_CUSTOM_STATE_ID {
    type TypeKind = windows_core::CopyType;
}
impl Default for SERVICE_TRIGGER_CUSTOM_STATE_ID {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SERVICE_TRIGGER_INFO {
    pub cTriggers: u32,
    pub pTriggers: *mut SERVICE_TRIGGER,
    pub pReserved: *mut u8,
}
impl windows_core::TypeKind for SERVICE_TRIGGER_INFO {
    type TypeKind = windows_core::CopyType;
}
impl Default for SERVICE_TRIGGER_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SERVICE_TRIGGER_SPECIFIC_DATA_ITEM {
    pub dwDataType: SERVICE_TRIGGER_SPECIFIC_DATA_ITEM_DATA_TYPE,
    pub cbData: u32,
    pub pData: *mut u8,
}
impl windows_core::TypeKind for SERVICE_TRIGGER_SPECIFIC_DATA_ITEM {
    type TypeKind = windows_core::CopyType;
}
impl Default for SERVICE_TRIGGER_SPECIFIC_DATA_ITEM {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type HANDLER_FUNCTION = Option<unsafe extern "system" fn(dwcontrol: u32)>;
pub type HANDLER_FUNCTION_EX = Option<unsafe extern "system" fn(dwcontrol: u32, dweventtype: u32, lpeventdata: *mut core::ffi::c_void, lpcontext: *mut core::ffi::c_void) -> u32>;
pub type LPHANDLER_FUNCTION = Option<unsafe extern "system" fn(dwcontrol: u32)>;
pub type LPHANDLER_FUNCTION_EX = Option<unsafe extern "system" fn(dwcontrol: u32, dweventtype: u32, lpeventdata: *mut core::ffi::c_void, lpcontext: *mut core::ffi::c_void) -> u32>;
pub type LPSERVICE_MAIN_FUNCTIONA = Option<unsafe extern "system" fn(dwnumservicesargs: u32, lpserviceargvectors: *mut windows_core::PSTR)>;
pub type LPSERVICE_MAIN_FUNCTIONW = Option<unsafe extern "system" fn(dwnumservicesargs: u32, lpserviceargvectors: *mut windows_core::PWSTR)>;
pub type PFN_SC_NOTIFY_CALLBACK = Option<unsafe extern "system" fn(pparameter: *const core::ffi::c_void)>;
pub type PSC_NOTIFICATION_CALLBACK = Option<unsafe extern "system" fn(dwnotify: u32, pcallbackcontext: *const core::ffi::c_void)>;
pub type SERVICE_MAIN_FUNCTIONA = Option<unsafe extern "system" fn(dwnumservicesargs: u32, lpserviceargvectors: *mut *mut i8)>;
pub type SERVICE_MAIN_FUNCTIONW = Option<unsafe extern "system" fn(dwnumservicesargs: u32, lpserviceargvectors: *mut windows_core::PWSTR)>;
