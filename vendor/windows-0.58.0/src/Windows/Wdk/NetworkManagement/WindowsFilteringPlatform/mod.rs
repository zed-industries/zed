#[cfg(feature = "Win32_NetworkManagement_WindowsFilteringPlatform")]
#[inline]
pub unsafe fn FwpmBfeStateGet0() -> super::super::super::Win32::NetworkManagement::WindowsFilteringPlatform::FWPM_SERVICE_STATE {
    windows_targets::link!("fwpkclnt.sys" "system" fn FwpmBfeStateGet0() -> super::super::super::Win32::NetworkManagement::WindowsFilteringPlatform:: FWPM_SERVICE_STATE);
    FwpmBfeStateGet0()
}
#[cfg(feature = "Win32_NetworkManagement_WindowsFilteringPlatform")]
#[inline]
pub unsafe fn FwpmBfeStateSubscribeChanges0(deviceobject: *mut core::ffi::c_void, callback: FWPM_SERVICE_STATE_CHANGE_CALLBACK0, context: Option<*const core::ffi::c_void>, changehandle: *mut super::super::super::Win32::Foundation::HANDLE) -> super::super::super::Win32::Foundation::NTSTATUS {
    windows_targets::link!("fwpkclnt.sys" "system" fn FwpmBfeStateSubscribeChanges0(deviceobject : *mut core::ffi::c_void, callback : FWPM_SERVICE_STATE_CHANGE_CALLBACK0, context : *const core::ffi::c_void, changehandle : *mut super::super::super::Win32::Foundation:: HANDLE) -> super::super::super::Win32::Foundation:: NTSTATUS);
    FwpmBfeStateSubscribeChanges0(deviceobject, callback, core::mem::transmute(context.unwrap_or(std::ptr::null())), changehandle)
}
#[inline]
pub unsafe fn FwpmBfeStateUnsubscribeChanges0(changehandle: super::super::super::Win32::Foundation::HANDLE) -> super::super::super::Win32::Foundation::NTSTATUS {
    windows_targets::link!("fwpkclnt.sys" "system" fn FwpmBfeStateUnsubscribeChanges0(changehandle : super::super::super::Win32::Foundation:: HANDLE) -> super::super::super::Win32::Foundation:: NTSTATUS);
    FwpmBfeStateUnsubscribeChanges0(changehandle)
}
#[cfg(all(feature = "Win32_NetworkManagement_WindowsFilteringPlatform", feature = "Win32_Security"))]
#[inline]
pub unsafe fn FwpmCalloutAdd0<P0, P1>(enginehandle: P0, callout: *const super::super::super::Win32::NetworkManagement::WindowsFilteringPlatform::FWPM_CALLOUT0, sd: P1, id: Option<*mut u32>) -> super::super::super::Win32::Foundation::NTSTATUS
where
    P0: windows_core::Param<super::super::super::Win32::Foundation::HANDLE>,
    P1: windows_core::Param<super::super::super::Win32::Security::PSECURITY_DESCRIPTOR>,
{
    windows_targets::link!("fwpuclnt.dll" "system" fn FwpmCalloutAdd0(enginehandle : super::super::super::Win32::Foundation:: HANDLE, callout : *const super::super::super::Win32::NetworkManagement::WindowsFilteringPlatform:: FWPM_CALLOUT0, sd : super::super::super::Win32::Security:: PSECURITY_DESCRIPTOR, id : *mut u32) -> super::super::super::Win32::Foundation:: NTSTATUS);
    FwpmCalloutAdd0(enginehandle.param().abi(), callout, sd.param().abi(), core::mem::transmute(id.unwrap_or(std::ptr::null_mut())))
}
#[cfg(feature = "Win32_NetworkManagement_WindowsFilteringPlatform")]
#[inline]
pub unsafe fn FwpmCalloutCreateEnumHandle0<P0>(enginehandle: P0, enumtemplate: Option<*const super::super::super::Win32::NetworkManagement::WindowsFilteringPlatform::FWPM_CALLOUT_ENUM_TEMPLATE0>, enumhandle: *mut super::super::super::Win32::Foundation::HANDLE) -> super::super::super::Win32::Foundation::NTSTATUS
where
    P0: windows_core::Param<super::super::super::Win32::Foundation::HANDLE>,
{
    windows_targets::link!("fwpuclnt.dll" "system" fn FwpmCalloutCreateEnumHandle0(enginehandle : super::super::super::Win32::Foundation:: HANDLE, enumtemplate : *const super::super::super::Win32::NetworkManagement::WindowsFilteringPlatform:: FWPM_CALLOUT_ENUM_TEMPLATE0, enumhandle : *mut super::super::super::Win32::Foundation:: HANDLE) -> super::super::super::Win32::Foundation:: NTSTATUS);
    FwpmCalloutCreateEnumHandle0(enginehandle.param().abi(), core::mem::transmute(enumtemplate.unwrap_or(std::ptr::null())), enumhandle)
}
#[inline]
pub unsafe fn FwpmCalloutDeleteById0<P0>(enginehandle: P0, id: u32) -> super::super::super::Win32::Foundation::NTSTATUS
where
    P0: windows_core::Param<super::super::super::Win32::Foundation::HANDLE>,
{
    windows_targets::link!("fwpuclnt.dll" "system" fn FwpmCalloutDeleteById0(enginehandle : super::super::super::Win32::Foundation:: HANDLE, id : u32) -> super::super::super::Win32::Foundation:: NTSTATUS);
    FwpmCalloutDeleteById0(enginehandle.param().abi(), id)
}
#[inline]
pub unsafe fn FwpmCalloutDeleteByKey0<P0>(enginehandle: P0, key: *const windows_core::GUID) -> super::super::super::Win32::Foundation::NTSTATUS
where
    P0: windows_core::Param<super::super::super::Win32::Foundation::HANDLE>,
{
    windows_targets::link!("fwpuclnt.dll" "system" fn FwpmCalloutDeleteByKey0(enginehandle : super::super::super::Win32::Foundation:: HANDLE, key : *const windows_core::GUID) -> super::super::super::Win32::Foundation:: NTSTATUS);
    FwpmCalloutDeleteByKey0(enginehandle.param().abi(), key)
}
#[inline]
pub unsafe fn FwpmCalloutDestroyEnumHandle0<P0>(enginehandle: P0, enumhandle: super::super::super::Win32::Foundation::HANDLE) -> super::super::super::Win32::Foundation::NTSTATUS
where
    P0: windows_core::Param<super::super::super::Win32::Foundation::HANDLE>,
{
    windows_targets::link!("fwpuclnt.dll" "system" fn FwpmCalloutDestroyEnumHandle0(enginehandle : super::super::super::Win32::Foundation:: HANDLE, enumhandle : super::super::super::Win32::Foundation:: HANDLE) -> super::super::super::Win32::Foundation:: NTSTATUS);
    FwpmCalloutDestroyEnumHandle0(enginehandle.param().abi(), enumhandle)
}
#[cfg(feature = "Win32_NetworkManagement_WindowsFilteringPlatform")]
#[inline]
pub unsafe fn FwpmCalloutEnum0<P0, P1>(enginehandle: P0, enumhandle: P1, numentriesrequested: u32, entries: *mut *mut *mut super::super::super::Win32::NetworkManagement::WindowsFilteringPlatform::FWPM_CALLOUT0, numentriesreturned: *mut u32) -> super::super::super::Win32::Foundation::NTSTATUS
where
    P0: windows_core::Param<super::super::super::Win32::Foundation::HANDLE>,
    P1: windows_core::Param<super::super::super::Win32::Foundation::HANDLE>,
{
    windows_targets::link!("fwpuclnt.dll" "system" fn FwpmCalloutEnum0(enginehandle : super::super::super::Win32::Foundation:: HANDLE, enumhandle : super::super::super::Win32::Foundation:: HANDLE, numentriesrequested : u32, entries : *mut *mut *mut super::super::super::Win32::NetworkManagement::WindowsFilteringPlatform:: FWPM_CALLOUT0, numentriesreturned : *mut u32) -> super::super::super::Win32::Foundation:: NTSTATUS);
    FwpmCalloutEnum0(enginehandle.param().abi(), enumhandle.param().abi(), numentriesrequested, entries, numentriesreturned)
}
#[cfg(feature = "Win32_NetworkManagement_WindowsFilteringPlatform")]
#[inline]
pub unsafe fn FwpmCalloutGetById0<P0>(enginehandle: P0, id: u32, callout: *mut *mut super::super::super::Win32::NetworkManagement::WindowsFilteringPlatform::FWPM_CALLOUT0) -> super::super::super::Win32::Foundation::NTSTATUS
where
    P0: windows_core::Param<super::super::super::Win32::Foundation::HANDLE>,
{
    windows_targets::link!("fwpuclnt.dll" "system" fn FwpmCalloutGetById0(enginehandle : super::super::super::Win32::Foundation:: HANDLE, id : u32, callout : *mut *mut super::super::super::Win32::NetworkManagement::WindowsFilteringPlatform:: FWPM_CALLOUT0) -> super::super::super::Win32::Foundation:: NTSTATUS);
    FwpmCalloutGetById0(enginehandle.param().abi(), id, callout)
}
#[cfg(feature = "Win32_NetworkManagement_WindowsFilteringPlatform")]
#[inline]
pub unsafe fn FwpmCalloutGetByKey0<P0>(enginehandle: P0, key: *const windows_core::GUID, callout: *mut *mut super::super::super::Win32::NetworkManagement::WindowsFilteringPlatform::FWPM_CALLOUT0) -> super::super::super::Win32::Foundation::NTSTATUS
where
    P0: windows_core::Param<super::super::super::Win32::Foundation::HANDLE>,
{
    windows_targets::link!("fwpuclnt.dll" "system" fn FwpmCalloutGetByKey0(enginehandle : super::super::super::Win32::Foundation:: HANDLE, key : *const windows_core::GUID, callout : *mut *mut super::super::super::Win32::NetworkManagement::WindowsFilteringPlatform:: FWPM_CALLOUT0) -> super::super::super::Win32::Foundation:: NTSTATUS);
    FwpmCalloutGetByKey0(enginehandle.param().abi(), key, callout)
}
#[cfg(feature = "Win32_Security")]
#[inline]
pub unsafe fn FwpmCalloutGetSecurityInfoByKey0<P0>(enginehandle: P0, key: Option<*const windows_core::GUID>, securityinfo: u32, sidowner: *mut super::super::super::Win32::Security::PSID, sidgroup: *mut super::super::super::Win32::Security::PSID, dacl: *mut *mut super::super::super::Win32::Security::ACL, sacl: *mut *mut super::super::super::Win32::Security::ACL, securitydescriptor: *mut super::super::super::Win32::Security::PSECURITY_DESCRIPTOR) -> super::super::super::Win32::Foundation::NTSTATUS
where
    P0: windows_core::Param<super::super::super::Win32::Foundation::HANDLE>,
{
    windows_targets::link!("fwpuclnt.dll" "system" fn FwpmCalloutGetSecurityInfoByKey0(enginehandle : super::super::super::Win32::Foundation:: HANDLE, key : *const windows_core::GUID, securityinfo : u32, sidowner : *mut super::super::super::Win32::Security:: PSID, sidgroup : *mut super::super::super::Win32::Security:: PSID, dacl : *mut *mut super::super::super::Win32::Security:: ACL, sacl : *mut *mut super::super::super::Win32::Security:: ACL, securitydescriptor : *mut super::super::super::Win32::Security:: PSECURITY_DESCRIPTOR) -> super::super::super::Win32::Foundation:: NTSTATUS);
    FwpmCalloutGetSecurityInfoByKey0(enginehandle.param().abi(), core::mem::transmute(key.unwrap_or(std::ptr::null())), securityinfo, sidowner, sidgroup, dacl, sacl, securitydescriptor)
}
#[cfg(feature = "Win32_Security")]
#[inline]
pub unsafe fn FwpmCalloutSetSecurityInfoByKey0<P0>(enginehandle: P0, key: Option<*const windows_core::GUID>, securityinfo: u32, sidowner: Option<*const super::super::super::Win32::Security::SID>, sidgroup: Option<*const super::super::super::Win32::Security::SID>, dacl: Option<*const super::super::super::Win32::Security::ACL>, sacl: Option<*const super::super::super::Win32::Security::ACL>) -> super::super::super::Win32::Foundation::NTSTATUS
where
    P0: windows_core::Param<super::super::super::Win32::Foundation::HANDLE>,
{
    windows_targets::link!("fwpuclnt.dll" "system" fn FwpmCalloutSetSecurityInfoByKey0(enginehandle : super::super::super::Win32::Foundation:: HANDLE, key : *const windows_core::GUID, securityinfo : u32, sidowner : *const super::super::super::Win32::Security:: SID, sidgroup : *const super::super::super::Win32::Security:: SID, dacl : *const super::super::super::Win32::Security:: ACL, sacl : *const super::super::super::Win32::Security:: ACL) -> super::super::super::Win32::Foundation:: NTSTATUS);
    FwpmCalloutSetSecurityInfoByKey0(enginehandle.param().abi(), core::mem::transmute(key.unwrap_or(std::ptr::null())), securityinfo, core::mem::transmute(sidowner.unwrap_or(std::ptr::null())), core::mem::transmute(sidgroup.unwrap_or(std::ptr::null())), core::mem::transmute(dacl.unwrap_or(std::ptr::null())), core::mem::transmute(sacl.unwrap_or(std::ptr::null())))
}
#[cfg(feature = "Win32_NetworkManagement_WindowsFilteringPlatform")]
#[inline]
pub unsafe fn FwpmConnectionCreateEnumHandle0<P0>(enginehandle: P0, enumtemplate: Option<*const super::super::super::Win32::NetworkManagement::WindowsFilteringPlatform::FWPM_CONNECTION_ENUM_TEMPLATE0>, enumhandle: *mut super::super::super::Win32::Foundation::HANDLE) -> super::super::super::Win32::Foundation::NTSTATUS
where
    P0: windows_core::Param<super::super::super::Win32::Foundation::HANDLE>,
{
    windows_targets::link!("fwpuclnt.dll" "system" fn FwpmConnectionCreateEnumHandle0(enginehandle : super::super::super::Win32::Foundation:: HANDLE, enumtemplate : *const super::super::super::Win32::NetworkManagement::WindowsFilteringPlatform:: FWPM_CONNECTION_ENUM_TEMPLATE0, enumhandle : *mut super::super::super::Win32::Foundation:: HANDLE) -> super::super::super::Win32::Foundation:: NTSTATUS);
    FwpmConnectionCreateEnumHandle0(enginehandle.param().abi(), core::mem::transmute(enumtemplate.unwrap_or(std::ptr::null())), enumhandle)
}
#[inline]
pub unsafe fn FwpmConnectionDestroyEnumHandle0<P0, P1>(enginehandle: P0, enumhandle: P1) -> super::super::super::Win32::Foundation::NTSTATUS
where
    P0: windows_core::Param<super::super::super::Win32::Foundation::HANDLE>,
    P1: windows_core::Param<super::super::super::Win32::Foundation::HANDLE>,
{
    windows_targets::link!("fwpuclnt.dll" "system" fn FwpmConnectionDestroyEnumHandle0(enginehandle : super::super::super::Win32::Foundation:: HANDLE, enumhandle : super::super::super::Win32::Foundation:: HANDLE) -> super::super::super::Win32::Foundation:: NTSTATUS);
    FwpmConnectionDestroyEnumHandle0(enginehandle.param().abi(), enumhandle.param().abi())
}
#[cfg(feature = "Win32_NetworkManagement_WindowsFilteringPlatform")]
#[inline]
pub unsafe fn FwpmConnectionEnum0<P0, P1>(enginehandle: P0, enumhandle: P1, numentriesrequested: u32, entries: *mut *mut *mut super::super::super::Win32::NetworkManagement::WindowsFilteringPlatform::FWPM_CONNECTION0, numentriesreturned: *mut u32) -> super::super::super::Win32::Foundation::NTSTATUS
where
    P0: windows_core::Param<super::super::super::Win32::Foundation::HANDLE>,
    P1: windows_core::Param<super::super::super::Win32::Foundation::HANDLE>,
{
    windows_targets::link!("fwpuclnt.dll" "system" fn FwpmConnectionEnum0(enginehandle : super::super::super::Win32::Foundation:: HANDLE, enumhandle : super::super::super::Win32::Foundation:: HANDLE, numentriesrequested : u32, entries : *mut *mut *mut super::super::super::Win32::NetworkManagement::WindowsFilteringPlatform:: FWPM_CONNECTION0, numentriesreturned : *mut u32) -> super::super::super::Win32::Foundation:: NTSTATUS);
    FwpmConnectionEnum0(enginehandle.param().abi(), enumhandle.param().abi(), numentriesrequested, entries, numentriesreturned)
}
#[cfg(feature = "Win32_NetworkManagement_WindowsFilteringPlatform")]
#[inline]
pub unsafe fn FwpmConnectionGetById0<P0>(enginehandle: P0, id: u64, connection: *mut *mut super::super::super::Win32::NetworkManagement::WindowsFilteringPlatform::FWPM_CONNECTION0) -> super::super::super::Win32::Foundation::NTSTATUS
where
    P0: windows_core::Param<super::super::super::Win32::Foundation::HANDLE>,
{
    windows_targets::link!("fwpuclnt.dll" "system" fn FwpmConnectionGetById0(enginehandle : super::super::super::Win32::Foundation:: HANDLE, id : u64, connection : *mut *mut super::super::super::Win32::NetworkManagement::WindowsFilteringPlatform:: FWPM_CONNECTION0) -> super::super::super::Win32::Foundation:: NTSTATUS);
    FwpmConnectionGetById0(enginehandle.param().abi(), id, connection)
}
#[cfg(feature = "Win32_Security")]
#[inline]
pub unsafe fn FwpmConnectionGetSecurityInfo0<P0>(enginehandle: P0, securityinfo: u32, sidowner: *mut super::super::super::Win32::Security::PSID, sidgroup: *mut super::super::super::Win32::Security::PSID, dacl: *mut *mut super::super::super::Win32::Security::ACL, sacl: *mut *mut super::super::super::Win32::Security::ACL, securitydescriptor: *mut super::super::super::Win32::Security::PSECURITY_DESCRIPTOR) -> super::super::super::Win32::Foundation::NTSTATUS
where
    P0: windows_core::Param<super::super::super::Win32::Foundation::HANDLE>,
{
    windows_targets::link!("fwpuclnt.dll" "system" fn FwpmConnectionGetSecurityInfo0(enginehandle : super::super::super::Win32::Foundation:: HANDLE, securityinfo : u32, sidowner : *mut super::super::super::Win32::Security:: PSID, sidgroup : *mut super::super::super::Win32::Security:: PSID, dacl : *mut *mut super::super::super::Win32::Security:: ACL, sacl : *mut *mut super::super::super::Win32::Security:: ACL, securitydescriptor : *mut super::super::super::Win32::Security:: PSECURITY_DESCRIPTOR) -> super::super::super::Win32::Foundation:: NTSTATUS);
    FwpmConnectionGetSecurityInfo0(enginehandle.param().abi(), securityinfo, sidowner, sidgroup, dacl, sacl, securitydescriptor)
}
#[cfg(feature = "Win32_Security")]
#[inline]
pub unsafe fn FwpmConnectionSetSecurityInfo0<P0>(enginehandle: P0, securityinfo: u32, sidowner: Option<*const super::super::super::Win32::Security::SID>, sidgroup: Option<*const super::super::super::Win32::Security::SID>, dacl: Option<*const super::super::super::Win32::Security::ACL>, sacl: Option<*const super::super::super::Win32::Security::ACL>) -> super::super::super::Win32::Foundation::NTSTATUS
where
    P0: windows_core::Param<super::super::super::Win32::Foundation::HANDLE>,
{
    windows_targets::link!("fwpuclnt.dll" "system" fn FwpmConnectionSetSecurityInfo0(enginehandle : super::super::super::Win32::Foundation:: HANDLE, securityinfo : u32, sidowner : *const super::super::super::Win32::Security:: SID, sidgroup : *const super::super::super::Win32::Security:: SID, dacl : *const super::super::super::Win32::Security:: ACL, sacl : *const super::super::super::Win32::Security:: ACL) -> super::super::super::Win32::Foundation:: NTSTATUS);
    FwpmConnectionSetSecurityInfo0(enginehandle.param().abi(), securityinfo, core::mem::transmute(sidowner.unwrap_or(std::ptr::null())), core::mem::transmute(sidgroup.unwrap_or(std::ptr::null())), core::mem::transmute(dacl.unwrap_or(std::ptr::null())), core::mem::transmute(sacl.unwrap_or(std::ptr::null())))
}
#[inline]
pub unsafe fn FwpmEngineClose0(enginehandle: super::super::super::Win32::Foundation::HANDLE) -> super::super::super::Win32::Foundation::NTSTATUS {
    windows_targets::link!("fwpuclnt.dll" "system" fn FwpmEngineClose0(enginehandle : super::super::super::Win32::Foundation:: HANDLE) -> super::super::super::Win32::Foundation:: NTSTATUS);
    FwpmEngineClose0(enginehandle)
}
#[cfg(all(feature = "Win32_NetworkManagement_WindowsFilteringPlatform", feature = "Win32_Security"))]
#[inline]
pub unsafe fn FwpmEngineGetOption0<P0>(enginehandle: P0, option: super::super::super::Win32::NetworkManagement::WindowsFilteringPlatform::FWPM_ENGINE_OPTION, value: *mut *mut super::super::super::Win32::NetworkManagement::WindowsFilteringPlatform::FWP_VALUE0) -> super::super::super::Win32::Foundation::NTSTATUS
where
    P0: windows_core::Param<super::super::super::Win32::Foundation::HANDLE>,
{
    windows_targets::link!("fwpuclnt.dll" "system" fn FwpmEngineGetOption0(enginehandle : super::super::super::Win32::Foundation:: HANDLE, option : super::super::super::Win32::NetworkManagement::WindowsFilteringPlatform:: FWPM_ENGINE_OPTION, value : *mut *mut super::super::super::Win32::NetworkManagement::WindowsFilteringPlatform:: FWP_VALUE0) -> super::super::super::Win32::Foundation:: NTSTATUS);
    FwpmEngineGetOption0(enginehandle.param().abi(), option, value)
}
#[cfg(feature = "Win32_Security")]
#[inline]
pub unsafe fn FwpmEngineGetSecurityInfo0<P0>(enginehandle: P0, securityinfo: u32, sidowner: *mut super::super::super::Win32::Security::PSID, sidgroup: *mut super::super::super::Win32::Security::PSID, dacl: *mut *mut super::super::super::Win32::Security::ACL, sacl: *mut *mut super::super::super::Win32::Security::ACL, securitydescriptor: *mut super::super::super::Win32::Security::PSECURITY_DESCRIPTOR) -> super::super::super::Win32::Foundation::NTSTATUS
where
    P0: windows_core::Param<super::super::super::Win32::Foundation::HANDLE>,
{
    windows_targets::link!("fwpuclnt.dll" "system" fn FwpmEngineGetSecurityInfo0(enginehandle : super::super::super::Win32::Foundation:: HANDLE, securityinfo : u32, sidowner : *mut super::super::super::Win32::Security:: PSID, sidgroup : *mut super::super::super::Win32::Security:: PSID, dacl : *mut *mut super::super::super::Win32::Security:: ACL, sacl : *mut *mut super::super::super::Win32::Security:: ACL, securitydescriptor : *mut super::super::super::Win32::Security:: PSECURITY_DESCRIPTOR) -> super::super::super::Win32::Foundation:: NTSTATUS);
    FwpmEngineGetSecurityInfo0(enginehandle.param().abi(), securityinfo, sidowner, sidgroup, dacl, sacl, securitydescriptor)
}
#[cfg(all(feature = "Win32_NetworkManagement_WindowsFilteringPlatform", feature = "Win32_Security", feature = "Win32_System_Rpc"))]
#[inline]
pub unsafe fn FwpmEngineOpen0<P0>(servername: P0, authnservice: u32, authidentity: Option<*const super::super::super::Win32::System::Rpc::SEC_WINNT_AUTH_IDENTITY_W>, session: Option<*const super::super::super::Win32::NetworkManagement::WindowsFilteringPlatform::FWPM_SESSION0>, enginehandle: *mut super::super::super::Win32::Foundation::HANDLE) -> super::super::super::Win32::Foundation::NTSTATUS
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("fwpuclnt.dll" "system" fn FwpmEngineOpen0(servername : windows_core::PCWSTR, authnservice : u32, authidentity : *const super::super::super::Win32::System::Rpc:: SEC_WINNT_AUTH_IDENTITY_W, session : *const super::super::super::Win32::NetworkManagement::WindowsFilteringPlatform:: FWPM_SESSION0, enginehandle : *mut super::super::super::Win32::Foundation:: HANDLE) -> super::super::super::Win32::Foundation:: NTSTATUS);
    FwpmEngineOpen0(servername.param().abi(), authnservice, core::mem::transmute(authidentity.unwrap_or(std::ptr::null())), core::mem::transmute(session.unwrap_or(std::ptr::null())), enginehandle)
}
#[cfg(all(feature = "Win32_NetworkManagement_WindowsFilteringPlatform", feature = "Win32_Security"))]
#[inline]
pub unsafe fn FwpmEngineSetOption0<P0>(enginehandle: P0, option: super::super::super::Win32::NetworkManagement::WindowsFilteringPlatform::FWPM_ENGINE_OPTION, newvalue: *const super::super::super::Win32::NetworkManagement::WindowsFilteringPlatform::FWP_VALUE0) -> super::super::super::Win32::Foundation::NTSTATUS
where
    P0: windows_core::Param<super::super::super::Win32::Foundation::HANDLE>,
{
    windows_targets::link!("fwpuclnt.dll" "system" fn FwpmEngineSetOption0(enginehandle : super::super::super::Win32::Foundation:: HANDLE, option : super::super::super::Win32::NetworkManagement::WindowsFilteringPlatform:: FWPM_ENGINE_OPTION, newvalue : *const super::super::super::Win32::NetworkManagement::WindowsFilteringPlatform:: FWP_VALUE0) -> super::super::super::Win32::Foundation:: NTSTATUS);
    FwpmEngineSetOption0(enginehandle.param().abi(), option, newvalue)
}
#[cfg(feature = "Win32_Security")]
#[inline]
pub unsafe fn FwpmEngineSetSecurityInfo0<P0>(enginehandle: P0, securityinfo: u32, sidowner: Option<*const super::super::super::Win32::Security::SID>, sidgroup: Option<*const super::super::super::Win32::Security::SID>, dacl: Option<*const super::super::super::Win32::Security::ACL>, sacl: Option<*const super::super::super::Win32::Security::ACL>) -> super::super::super::Win32::Foundation::NTSTATUS
where
    P0: windows_core::Param<super::super::super::Win32::Foundation::HANDLE>,
{
    windows_targets::link!("fwpuclnt.dll" "system" fn FwpmEngineSetSecurityInfo0(enginehandle : super::super::super::Win32::Foundation:: HANDLE, securityinfo : u32, sidowner : *const super::super::super::Win32::Security:: SID, sidgroup : *const super::super::super::Win32::Security:: SID, dacl : *const super::super::super::Win32::Security:: ACL, sacl : *const super::super::super::Win32::Security:: ACL) -> super::super::super::Win32::Foundation:: NTSTATUS);
    FwpmEngineSetSecurityInfo0(enginehandle.param().abi(), securityinfo, core::mem::transmute(sidowner.unwrap_or(std::ptr::null())), core::mem::transmute(sidgroup.unwrap_or(std::ptr::null())), core::mem::transmute(dacl.unwrap_or(std::ptr::null())), core::mem::transmute(sacl.unwrap_or(std::ptr::null())))
}
#[cfg(all(feature = "Win32_NetworkManagement_WindowsFilteringPlatform", feature = "Win32_Security"))]
#[inline]
pub unsafe fn FwpmFilterAdd0<P0, P1>(enginehandle: P0, filter: *const super::super::super::Win32::NetworkManagement::WindowsFilteringPlatform::FWPM_FILTER0, sd: P1, id: Option<*mut u64>) -> super::super::super::Win32::Foundation::NTSTATUS
where
    P0: windows_core::Param<super::super::super::Win32::Foundation::HANDLE>,
    P1: windows_core::Param<super::super::super::Win32::Security::PSECURITY_DESCRIPTOR>,
{
    windows_targets::link!("fwpuclnt.dll" "system" fn FwpmFilterAdd0(enginehandle : super::super::super::Win32::Foundation:: HANDLE, filter : *const super::super::super::Win32::NetworkManagement::WindowsFilteringPlatform:: FWPM_FILTER0, sd : super::super::super::Win32::Security:: PSECURITY_DESCRIPTOR, id : *mut u64) -> super::super::super::Win32::Foundation:: NTSTATUS);
    FwpmFilterAdd0(enginehandle.param().abi(), filter, sd.param().abi(), core::mem::transmute(id.unwrap_or(std::ptr::null_mut())))
}
#[cfg(all(feature = "Win32_NetworkManagement_WindowsFilteringPlatform", feature = "Win32_Security"))]
#[inline]
pub unsafe fn FwpmFilterCreateEnumHandle0<P0>(enginehandle: P0, enumtemplate: Option<*const super::super::super::Win32::NetworkManagement::WindowsFilteringPlatform::FWPM_FILTER_ENUM_TEMPLATE0>, enumhandle: *mut super::super::super::Win32::Foundation::HANDLE) -> super::super::super::Win32::Foundation::NTSTATUS
where
    P0: windows_core::Param<super::super::super::Win32::Foundation::HANDLE>,
{
    windows_targets::link!("fwpuclnt.dll" "system" fn FwpmFilterCreateEnumHandle0(enginehandle : super::super::super::Win32::Foundation:: HANDLE, enumtemplate : *const super::super::super::Win32::NetworkManagement::WindowsFilteringPlatform:: FWPM_FILTER_ENUM_TEMPLATE0, enumhandle : *mut super::super::super::Win32::Foundation:: HANDLE) -> super::super::super::Win32::Foundation:: NTSTATUS);
    FwpmFilterCreateEnumHandle0(enginehandle.param().abi(), core::mem::transmute(enumtemplate.unwrap_or(std::ptr::null())), enumhandle)
}
#[inline]
pub unsafe fn FwpmFilterDeleteById0<P0>(enginehandle: P0, id: u64) -> super::super::super::Win32::Foundation::NTSTATUS
where
    P0: windows_core::Param<super::super::super::Win32::Foundation::HANDLE>,
{
    windows_targets::link!("fwpuclnt.dll" "system" fn FwpmFilterDeleteById0(enginehandle : super::super::super::Win32::Foundation:: HANDLE, id : u64) -> super::super::super::Win32::Foundation:: NTSTATUS);
    FwpmFilterDeleteById0(enginehandle.param().abi(), id)
}
#[inline]
pub unsafe fn FwpmFilterDeleteByKey0<P0>(enginehandle: P0, key: *const windows_core::GUID) -> super::super::super::Win32::Foundation::NTSTATUS
where
    P0: windows_core::Param<super::super::super::Win32::Foundation::HANDLE>,
{
    windows_targets::link!("fwpuclnt.dll" "system" fn FwpmFilterDeleteByKey0(enginehandle : super::super::super::Win32::Foundation:: HANDLE, key : *const windows_core::GUID) -> super::super::super::Win32::Foundation:: NTSTATUS);
    FwpmFilterDeleteByKey0(enginehandle.param().abi(), key)
}
#[inline]
pub unsafe fn FwpmFilterDestroyEnumHandle0<P0>(enginehandle: P0, enumhandle: super::super::super::Win32::Foundation::HANDLE) -> super::super::super::Win32::Foundation::NTSTATUS
where
    P0: windows_core::Param<super::super::super::Win32::Foundation::HANDLE>,
{
    windows_targets::link!("fwpuclnt.dll" "system" fn FwpmFilterDestroyEnumHandle0(enginehandle : super::super::super::Win32::Foundation:: HANDLE, enumhandle : super::super::super::Win32::Foundation:: HANDLE) -> super::super::super::Win32::Foundation:: NTSTATUS);
    FwpmFilterDestroyEnumHandle0(enginehandle.param().abi(), enumhandle)
}
#[cfg(all(feature = "Win32_NetworkManagement_WindowsFilteringPlatform", feature = "Win32_Security"))]
#[inline]
pub unsafe fn FwpmFilterEnum0<P0, P1>(enginehandle: P0, enumhandle: P1, numentriesrequested: u32, entries: *mut *mut *mut super::super::super::Win32::NetworkManagement::WindowsFilteringPlatform::FWPM_FILTER0, numentriesreturned: *mut u32) -> super::super::super::Win32::Foundation::NTSTATUS
where
    P0: windows_core::Param<super::super::super::Win32::Foundation::HANDLE>,
    P1: windows_core::Param<super::super::super::Win32::Foundation::HANDLE>,
{
    windows_targets::link!("fwpuclnt.dll" "system" fn FwpmFilterEnum0(enginehandle : super::super::super::Win32::Foundation:: HANDLE, enumhandle : super::super::super::Win32::Foundation:: HANDLE, numentriesrequested : u32, entries : *mut *mut *mut super::super::super::Win32::NetworkManagement::WindowsFilteringPlatform:: FWPM_FILTER0, numentriesreturned : *mut u32) -> super::super::super::Win32::Foundation:: NTSTATUS);
    FwpmFilterEnum0(enginehandle.param().abi(), enumhandle.param().abi(), numentriesrequested, entries, numentriesreturned)
}
#[cfg(all(feature = "Win32_NetworkManagement_WindowsFilteringPlatform", feature = "Win32_Security"))]
#[inline]
pub unsafe fn FwpmFilterGetById0<P0>(enginehandle: P0, id: u64, filter: *mut *mut super::super::super::Win32::NetworkManagement::WindowsFilteringPlatform::FWPM_FILTER0) -> super::super::super::Win32::Foundation::NTSTATUS
where
    P0: windows_core::Param<super::super::super::Win32::Foundation::HANDLE>,
{
    windows_targets::link!("fwpuclnt.dll" "system" fn FwpmFilterGetById0(enginehandle : super::super::super::Win32::Foundation:: HANDLE, id : u64, filter : *mut *mut super::super::super::Win32::NetworkManagement::WindowsFilteringPlatform:: FWPM_FILTER0) -> super::super::super::Win32::Foundation:: NTSTATUS);
    FwpmFilterGetById0(enginehandle.param().abi(), id, filter)
}
#[cfg(all(feature = "Win32_NetworkManagement_WindowsFilteringPlatform", feature = "Win32_Security"))]
#[inline]
pub unsafe fn FwpmFilterGetByKey0<P0>(enginehandle: P0, key: *const windows_core::GUID, filter: *mut *mut super::super::super::Win32::NetworkManagement::WindowsFilteringPlatform::FWPM_FILTER0) -> super::super::super::Win32::Foundation::NTSTATUS
where
    P0: windows_core::Param<super::super::super::Win32::Foundation::HANDLE>,
{
    windows_targets::link!("fwpuclnt.dll" "system" fn FwpmFilterGetByKey0(enginehandle : super::super::super::Win32::Foundation:: HANDLE, key : *const windows_core::GUID, filter : *mut *mut super::super::super::Win32::NetworkManagement::WindowsFilteringPlatform:: FWPM_FILTER0) -> super::super::super::Win32::Foundation:: NTSTATUS);
    FwpmFilterGetByKey0(enginehandle.param().abi(), key, filter)
}
#[cfg(feature = "Win32_Security")]
#[inline]
pub unsafe fn FwpmFilterGetSecurityInfoByKey0<P0>(enginehandle: P0, key: Option<*const windows_core::GUID>, securityinfo: u32, sidowner: *mut super::super::super::Win32::Security::PSID, sidgroup: *mut super::super::super::Win32::Security::PSID, dacl: *mut *mut super::super::super::Win32::Security::ACL, sacl: *mut *mut super::super::super::Win32::Security::ACL, securitydescriptor: *mut super::super::super::Win32::Security::PSECURITY_DESCRIPTOR) -> super::super::super::Win32::Foundation::NTSTATUS
where
    P0: windows_core::Param<super::super::super::Win32::Foundation::HANDLE>,
{
    windows_targets::link!("fwpuclnt.dll" "system" fn FwpmFilterGetSecurityInfoByKey0(enginehandle : super::super::super::Win32::Foundation:: HANDLE, key : *const windows_core::GUID, securityinfo : u32, sidowner : *mut super::super::super::Win32::Security:: PSID, sidgroup : *mut super::super::super::Win32::Security:: PSID, dacl : *mut *mut super::super::super::Win32::Security:: ACL, sacl : *mut *mut super::super::super::Win32::Security:: ACL, securitydescriptor : *mut super::super::super::Win32::Security:: PSECURITY_DESCRIPTOR) -> super::super::super::Win32::Foundation:: NTSTATUS);
    FwpmFilterGetSecurityInfoByKey0(enginehandle.param().abi(), core::mem::transmute(key.unwrap_or(std::ptr::null())), securityinfo, sidowner, sidgroup, dacl, sacl, securitydescriptor)
}
#[cfg(feature = "Win32_Security")]
#[inline]
pub unsafe fn FwpmFilterSetSecurityInfoByKey0<P0>(enginehandle: P0, key: Option<*const windows_core::GUID>, securityinfo: u32, sidowner: Option<*const super::super::super::Win32::Security::SID>, sidgroup: Option<*const super::super::super::Win32::Security::SID>, dacl: Option<*const super::super::super::Win32::Security::ACL>, sacl: Option<*const super::super::super::Win32::Security::ACL>) -> super::super::super::Win32::Foundation::NTSTATUS
where
    P0: windows_core::Param<super::super::super::Win32::Foundation::HANDLE>,
{
    windows_targets::link!("fwpuclnt.dll" "system" fn FwpmFilterSetSecurityInfoByKey0(enginehandle : super::super::super::Win32::Foundation:: HANDLE, key : *const windows_core::GUID, securityinfo : u32, sidowner : *const super::super::super::Win32::Security:: SID, sidgroup : *const super::super::super::Win32::Security:: SID, dacl : *const super::super::super::Win32::Security:: ACL, sacl : *const super::super::super::Win32::Security:: ACL) -> super::super::super::Win32::Foundation:: NTSTATUS);
    FwpmFilterSetSecurityInfoByKey0(enginehandle.param().abi(), core::mem::transmute(key.unwrap_or(std::ptr::null())), securityinfo, core::mem::transmute(sidowner.unwrap_or(std::ptr::null())), core::mem::transmute(sidgroup.unwrap_or(std::ptr::null())), core::mem::transmute(dacl.unwrap_or(std::ptr::null())), core::mem::transmute(sacl.unwrap_or(std::ptr::null())))
}
#[inline]
pub unsafe fn FwpmFreeMemory0(p: *mut *mut core::ffi::c_void) {
    windows_targets::link!("fwpuclnt.dll" "system" fn FwpmFreeMemory0(p : *mut *mut core::ffi::c_void));
    FwpmFreeMemory0(p)
}
#[cfg(all(feature = "Win32_NetworkManagement_WindowsFilteringPlatform", feature = "Win32_Security"))]
#[inline]
pub unsafe fn FwpmIPsecTunnelAdd0<P0, P1>(enginehandle: P0, flags: u32, mainmodepolicy: Option<*const super::super::super::Win32::NetworkManagement::WindowsFilteringPlatform::FWPM_PROVIDER_CONTEXT0>, tunnelpolicy: *const super::super::super::Win32::NetworkManagement::WindowsFilteringPlatform::FWPM_PROVIDER_CONTEXT0, filterconditions: &[super::super::super::Win32::NetworkManagement::WindowsFilteringPlatform::FWPM_FILTER_CONDITION0], sd: P1) -> super::super::super::Win32::Foundation::NTSTATUS
where
    P0: windows_core::Param<super::super::super::Win32::Foundation::HANDLE>,
    P1: windows_core::Param<super::super::super::Win32::Security::PSECURITY_DESCRIPTOR>,
{
    windows_targets::link!("fwpuclnt.dll" "system" fn FwpmIPsecTunnelAdd0(enginehandle : super::super::super::Win32::Foundation:: HANDLE, flags : u32, mainmodepolicy : *const super::super::super::Win32::NetworkManagement::WindowsFilteringPlatform:: FWPM_PROVIDER_CONTEXT0, tunnelpolicy : *const super::super::super::Win32::NetworkManagement::WindowsFilteringPlatform:: FWPM_PROVIDER_CONTEXT0, numfilterconditions : u32, filterconditions : *const super::super::super::Win32::NetworkManagement::WindowsFilteringPlatform:: FWPM_FILTER_CONDITION0, sd : super::super::super::Win32::Security:: PSECURITY_DESCRIPTOR) -> super::super::super::Win32::Foundation:: NTSTATUS);
    FwpmIPsecTunnelAdd0(enginehandle.param().abi(), flags, core::mem::transmute(mainmodepolicy.unwrap_or(std::ptr::null())), tunnelpolicy, filterconditions.len().try_into().unwrap(), core::mem::transmute(filterconditions.as_ptr()), sd.param().abi())
}
#[cfg(all(feature = "Win32_NetworkManagement_WindowsFilteringPlatform", feature = "Win32_Security"))]
#[inline]
pub unsafe fn FwpmIPsecTunnelAdd1<P0, P1>(enginehandle: P0, flags: u32, mainmodepolicy: Option<*const super::super::super::Win32::NetworkManagement::WindowsFilteringPlatform::FWPM_PROVIDER_CONTEXT1>, tunnelpolicy: *const super::super::super::Win32::NetworkManagement::WindowsFilteringPlatform::FWPM_PROVIDER_CONTEXT1, filterconditions: &[super::super::super::Win32::NetworkManagement::WindowsFilteringPlatform::FWPM_FILTER_CONDITION0], keymodkey: Option<*const windows_core::GUID>, sd: P1) -> super::super::super::Win32::Foundation::NTSTATUS
where
    P0: windows_core::Param<super::super::super::Win32::Foundation::HANDLE>,
    P1: windows_core::Param<super::super::super::Win32::Security::PSECURITY_DESCRIPTOR>,
{
    windows_targets::link!("fwpuclnt.dll" "system" fn FwpmIPsecTunnelAdd1(enginehandle : super::super::super::Win32::Foundation:: HANDLE, flags : u32, mainmodepolicy : *const super::super::super::Win32::NetworkManagement::WindowsFilteringPlatform:: FWPM_PROVIDER_CONTEXT1, tunnelpolicy : *const super::super::super::Win32::NetworkManagement::WindowsFilteringPlatform:: FWPM_PROVIDER_CONTEXT1, numfilterconditions : u32, filterconditions : *const super::super::super::Win32::NetworkManagement::WindowsFilteringPlatform:: FWPM_FILTER_CONDITION0, keymodkey : *const windows_core::GUID, sd : super::super::super::Win32::Security:: PSECURITY_DESCRIPTOR) -> super::super::super::Win32::Foundation:: NTSTATUS);
    FwpmIPsecTunnelAdd1(enginehandle.param().abi(), flags, core::mem::transmute(mainmodepolicy.unwrap_or(std::ptr::null())), tunnelpolicy, filterconditions.len().try_into().unwrap(), core::mem::transmute(filterconditions.as_ptr()), core::mem::transmute(keymodkey.unwrap_or(std::ptr::null())), sd.param().abi())
}
#[cfg(all(feature = "Win32_NetworkManagement_WindowsFilteringPlatform", feature = "Win32_Security"))]
#[inline]
pub unsafe fn FwpmIPsecTunnelAdd2<P0, P1>(enginehandle: P0, flags: u32, mainmodepolicy: Option<*const super::super::super::Win32::NetworkManagement::WindowsFilteringPlatform::FWPM_PROVIDER_CONTEXT2>, tunnelpolicy: *const super::super::super::Win32::NetworkManagement::WindowsFilteringPlatform::FWPM_PROVIDER_CONTEXT2, filterconditions: &[super::super::super::Win32::NetworkManagement::WindowsFilteringPlatform::FWPM_FILTER_CONDITION0], keymodkey: Option<*const windows_core::GUID>, sd: P1) -> super::super::super::Win32::Foundation::NTSTATUS
where
    P0: windows_core::Param<super::super::super::Win32::Foundation::HANDLE>,
    P1: windows_core::Param<super::super::super::Win32::Security::PSECURITY_DESCRIPTOR>,
{
    windows_targets::link!("fwpuclnt.dll" "system" fn FwpmIPsecTunnelAdd2(enginehandle : super::super::super::Win32::Foundation:: HANDLE, flags : u32, mainmodepolicy : *const super::super::super::Win32::NetworkManagement::WindowsFilteringPlatform:: FWPM_PROVIDER_CONTEXT2, tunnelpolicy : *const super::super::super::Win32::NetworkManagement::WindowsFilteringPlatform:: FWPM_PROVIDER_CONTEXT2, numfilterconditions : u32, filterconditions : *const super::super::super::Win32::NetworkManagement::WindowsFilteringPlatform:: FWPM_FILTER_CONDITION0, keymodkey : *const windows_core::GUID, sd : super::super::super::Win32::Security:: PSECURITY_DESCRIPTOR) -> super::super::super::Win32::Foundation:: NTSTATUS);
    FwpmIPsecTunnelAdd2(enginehandle.param().abi(), flags, core::mem::transmute(mainmodepolicy.unwrap_or(std::ptr::null())), tunnelpolicy, filterconditions.len().try_into().unwrap(), core::mem::transmute(filterconditions.as_ptr()), core::mem::transmute(keymodkey.unwrap_or(std::ptr::null())), sd.param().abi())
}
#[cfg(all(feature = "Win32_NetworkManagement_WindowsFilteringPlatform", feature = "Win32_Security"))]
#[inline]
pub unsafe fn FwpmIPsecTunnelAdd3<P0, P1>(enginehandle: P0, flags: u32, mainmodepolicy: Option<*const super::super::super::Win32::NetworkManagement::WindowsFilteringPlatform::FWPM_PROVIDER_CONTEXT3>, tunnelpolicy: *const super::super::super::Win32::NetworkManagement::WindowsFilteringPlatform::FWPM_PROVIDER_CONTEXT3, filterconditions: &[super::super::super::Win32::NetworkManagement::WindowsFilteringPlatform::FWPM_FILTER_CONDITION0], keymodkey: Option<*const windows_core::GUID>, sd: P1) -> super::super::super::Win32::Foundation::NTSTATUS
where
    P0: windows_core::Param<super::super::super::Win32::Foundation::HANDLE>,
    P1: windows_core::Param<super::super::super::Win32::Security::PSECURITY_DESCRIPTOR>,
{
    windows_targets::link!("fwpuclnt.dll" "system" fn FwpmIPsecTunnelAdd3(enginehandle : super::super::super::Win32::Foundation:: HANDLE, flags : u32, mainmodepolicy : *const super::super::super::Win32::NetworkManagement::WindowsFilteringPlatform:: FWPM_PROVIDER_CONTEXT3, tunnelpolicy : *const super::super::super::Win32::NetworkManagement::WindowsFilteringPlatform:: FWPM_PROVIDER_CONTEXT3, numfilterconditions : u32, filterconditions : *const super::super::super::Win32::NetworkManagement::WindowsFilteringPlatform:: FWPM_FILTER_CONDITION0, keymodkey : *const windows_core::GUID, sd : super::super::super::Win32::Security:: PSECURITY_DESCRIPTOR) -> super::super::super::Win32::Foundation:: NTSTATUS);
    FwpmIPsecTunnelAdd3(enginehandle.param().abi(), flags, core::mem::transmute(mainmodepolicy.unwrap_or(std::ptr::null())), tunnelpolicy, filterconditions.len().try_into().unwrap(), core::mem::transmute(filterconditions.as_ptr()), core::mem::transmute(keymodkey.unwrap_or(std::ptr::null())), sd.param().abi())
}
#[inline]
pub unsafe fn FwpmIPsecTunnelDeleteByKey0<P0>(enginehandle: P0, key: *const windows_core::GUID) -> super::super::super::Win32::Foundation::NTSTATUS
where
    P0: windows_core::Param<super::super::super::Win32::Foundation::HANDLE>,
{
    windows_targets::link!("fwpuclnt.dll" "system" fn FwpmIPsecTunnelDeleteByKey0(enginehandle : super::super::super::Win32::Foundation:: HANDLE, key : *const windows_core::GUID) -> super::super::super::Win32::Foundation:: NTSTATUS);
    FwpmIPsecTunnelDeleteByKey0(enginehandle.param().abi(), key)
}
#[cfg(feature = "Win32_NetworkManagement_WindowsFilteringPlatform")]
#[inline]
pub unsafe fn FwpmLayerCreateEnumHandle0<P0>(enginehandle: P0, enumtemplate: Option<*const super::super::super::Win32::NetworkManagement::WindowsFilteringPlatform::FWPM_LAYER_ENUM_TEMPLATE0>, enumhandle: *mut super::super::super::Win32::Foundation::HANDLE) -> super::super::super::Win32::Foundation::NTSTATUS
where
    P0: windows_core::Param<super::super::super::Win32::Foundation::HANDLE>,
{
    windows_targets::link!("fwpuclnt.dll" "system" fn FwpmLayerCreateEnumHandle0(enginehandle : super::super::super::Win32::Foundation:: HANDLE, enumtemplate : *const super::super::super::Win32::NetworkManagement::WindowsFilteringPlatform:: FWPM_LAYER_ENUM_TEMPLATE0, enumhandle : *mut super::super::super::Win32::Foundation:: HANDLE) -> super::super::super::Win32::Foundation:: NTSTATUS);
    FwpmLayerCreateEnumHandle0(enginehandle.param().abi(), core::mem::transmute(enumtemplate.unwrap_or(std::ptr::null())), enumhandle)
}
#[inline]
pub unsafe fn FwpmLayerDestroyEnumHandle0<P0>(enginehandle: P0, enumhandle: super::super::super::Win32::Foundation::HANDLE) -> super::super::super::Win32::Foundation::NTSTATUS
where
    P0: windows_core::Param<super::super::super::Win32::Foundation::HANDLE>,
{
    windows_targets::link!("fwpuclnt.dll" "system" fn FwpmLayerDestroyEnumHandle0(enginehandle : super::super::super::Win32::Foundation:: HANDLE, enumhandle : super::super::super::Win32::Foundation:: HANDLE) -> super::super::super::Win32::Foundation:: NTSTATUS);
    FwpmLayerDestroyEnumHandle0(enginehandle.param().abi(), enumhandle)
}
#[cfg(feature = "Win32_NetworkManagement_WindowsFilteringPlatform")]
#[inline]
pub unsafe fn FwpmLayerEnum0<P0, P1>(enginehandle: P0, enumhandle: P1, numentriesrequested: u32, entries: *mut *mut *mut super::super::super::Win32::NetworkManagement::WindowsFilteringPlatform::FWPM_LAYER0, numentriesreturned: *mut u32) -> super::super::super::Win32::Foundation::NTSTATUS
where
    P0: windows_core::Param<super::super::super::Win32::Foundation::HANDLE>,
    P1: windows_core::Param<super::super::super::Win32::Foundation::HANDLE>,
{
    windows_targets::link!("fwpuclnt.dll" "system" fn FwpmLayerEnum0(enginehandle : super::super::super::Win32::Foundation:: HANDLE, enumhandle : super::super::super::Win32::Foundation:: HANDLE, numentriesrequested : u32, entries : *mut *mut *mut super::super::super::Win32::NetworkManagement::WindowsFilteringPlatform:: FWPM_LAYER0, numentriesreturned : *mut u32) -> super::super::super::Win32::Foundation:: NTSTATUS);
    FwpmLayerEnum0(enginehandle.param().abi(), enumhandle.param().abi(), numentriesrequested, entries, numentriesreturned)
}
#[cfg(feature = "Win32_NetworkManagement_WindowsFilteringPlatform")]
#[inline]
pub unsafe fn FwpmLayerGetById0<P0>(enginehandle: P0, id: u16, layer: *mut *mut super::super::super::Win32::NetworkManagement::WindowsFilteringPlatform::FWPM_LAYER0) -> super::super::super::Win32::Foundation::NTSTATUS
where
    P0: windows_core::Param<super::super::super::Win32::Foundation::HANDLE>,
{
    windows_targets::link!("fwpuclnt.dll" "system" fn FwpmLayerGetById0(enginehandle : super::super::super::Win32::Foundation:: HANDLE, id : u16, layer : *mut *mut super::super::super::Win32::NetworkManagement::WindowsFilteringPlatform:: FWPM_LAYER0) -> super::super::super::Win32::Foundation:: NTSTATUS);
    FwpmLayerGetById0(enginehandle.param().abi(), id, layer)
}
#[cfg(feature = "Win32_NetworkManagement_WindowsFilteringPlatform")]
#[inline]
pub unsafe fn FwpmLayerGetByKey0<P0>(enginehandle: P0, key: *const windows_core::GUID, layer: *mut *mut super::super::super::Win32::NetworkManagement::WindowsFilteringPlatform::FWPM_LAYER0) -> super::super::super::Win32::Foundation::NTSTATUS
where
    P0: windows_core::Param<super::super::super::Win32::Foundation::HANDLE>,
{
    windows_targets::link!("fwpuclnt.dll" "system" fn FwpmLayerGetByKey0(enginehandle : super::super::super::Win32::Foundation:: HANDLE, key : *const windows_core::GUID, layer : *mut *mut super::super::super::Win32::NetworkManagement::WindowsFilteringPlatform:: FWPM_LAYER0) -> super::super::super::Win32::Foundation:: NTSTATUS);
    FwpmLayerGetByKey0(enginehandle.param().abi(), key, layer)
}
#[cfg(feature = "Win32_Security")]
#[inline]
pub unsafe fn FwpmLayerGetSecurityInfoByKey0<P0>(enginehandle: P0, key: Option<*const windows_core::GUID>, securityinfo: u32, sidowner: *mut super::super::super::Win32::Security::PSID, sidgroup: *mut super::super::super::Win32::Security::PSID, dacl: *mut *mut super::super::super::Win32::Security::ACL, sacl: *mut *mut super::super::super::Win32::Security::ACL, securitydescriptor: *mut super::super::super::Win32::Security::PSECURITY_DESCRIPTOR) -> super::super::super::Win32::Foundation::NTSTATUS
where
    P0: windows_core::Param<super::super::super::Win32::Foundation::HANDLE>,
{
    windows_targets::link!("fwpuclnt.dll" "system" fn FwpmLayerGetSecurityInfoByKey0(enginehandle : super::super::super::Win32::Foundation:: HANDLE, key : *const windows_core::GUID, securityinfo : u32, sidowner : *mut super::super::super::Win32::Security:: PSID, sidgroup : *mut super::super::super::Win32::Security:: PSID, dacl : *mut *mut super::super::super::Win32::Security:: ACL, sacl : *mut *mut super::super::super::Win32::Security:: ACL, securitydescriptor : *mut super::super::super::Win32::Security:: PSECURITY_DESCRIPTOR) -> super::super::super::Win32::Foundation:: NTSTATUS);
    FwpmLayerGetSecurityInfoByKey0(enginehandle.param().abi(), core::mem::transmute(key.unwrap_or(std::ptr::null())), securityinfo, sidowner, sidgroup, dacl, sacl, securitydescriptor)
}
#[cfg(feature = "Win32_Security")]
#[inline]
pub unsafe fn FwpmLayerSetSecurityInfoByKey0<P0>(enginehandle: P0, key: Option<*const windows_core::GUID>, securityinfo: u32, sidowner: Option<*const super::super::super::Win32::Security::SID>, sidgroup: Option<*const super::super::super::Win32::Security::SID>, dacl: Option<*const super::super::super::Win32::Security::ACL>, sacl: Option<*const super::super::super::Win32::Security::ACL>) -> super::super::super::Win32::Foundation::NTSTATUS
where
    P0: windows_core::Param<super::super::super::Win32::Foundation::HANDLE>,
{
    windows_targets::link!("fwpuclnt.dll" "system" fn FwpmLayerSetSecurityInfoByKey0(enginehandle : super::super::super::Win32::Foundation:: HANDLE, key : *const windows_core::GUID, securityinfo : u32, sidowner : *const super::super::super::Win32::Security:: SID, sidgroup : *const super::super::super::Win32::Security:: SID, dacl : *const super::super::super::Win32::Security:: ACL, sacl : *const super::super::super::Win32::Security:: ACL) -> super::super::super::Win32::Foundation:: NTSTATUS);
    FwpmLayerSetSecurityInfoByKey0(enginehandle.param().abi(), core::mem::transmute(key.unwrap_or(std::ptr::null())), securityinfo, core::mem::transmute(sidowner.unwrap_or(std::ptr::null())), core::mem::transmute(sidgroup.unwrap_or(std::ptr::null())), core::mem::transmute(dacl.unwrap_or(std::ptr::null())), core::mem::transmute(sacl.unwrap_or(std::ptr::null())))
}
#[cfg(all(feature = "Win32_NetworkManagement_WindowsFilteringPlatform", feature = "Win32_Security"))]
#[inline]
pub unsafe fn FwpmNetEventCreateEnumHandle0<P0>(enginehandle: P0, enumtemplate: Option<*const super::super::super::Win32::NetworkManagement::WindowsFilteringPlatform::FWPM_NET_EVENT_ENUM_TEMPLATE0>, enumhandle: *mut super::super::super::Win32::Foundation::HANDLE) -> super::super::super::Win32::Foundation::NTSTATUS
where
    P0: windows_core::Param<super::super::super::Win32::Foundation::HANDLE>,
{
    windows_targets::link!("fwpuclnt.dll" "system" fn FwpmNetEventCreateEnumHandle0(enginehandle : super::super::super::Win32::Foundation:: HANDLE, enumtemplate : *const super::super::super::Win32::NetworkManagement::WindowsFilteringPlatform:: FWPM_NET_EVENT_ENUM_TEMPLATE0, enumhandle : *mut super::super::super::Win32::Foundation:: HANDLE) -> super::super::super::Win32::Foundation:: NTSTATUS);
    FwpmNetEventCreateEnumHandle0(enginehandle.param().abi(), core::mem::transmute(enumtemplate.unwrap_or(std::ptr::null())), enumhandle)
}
#[inline]
pub unsafe fn FwpmNetEventDestroyEnumHandle0<P0>(enginehandle: P0, enumhandle: super::super::super::Win32::Foundation::HANDLE) -> super::super::super::Win32::Foundation::NTSTATUS
where
    P0: windows_core::Param<super::super::super::Win32::Foundation::HANDLE>,
{
    windows_targets::link!("fwpuclnt.dll" "system" fn FwpmNetEventDestroyEnumHandle0(enginehandle : super::super::super::Win32::Foundation:: HANDLE, enumhandle : super::super::super::Win32::Foundation:: HANDLE) -> super::super::super::Win32::Foundation:: NTSTATUS);
    FwpmNetEventDestroyEnumHandle0(enginehandle.param().abi(), enumhandle)
}
#[cfg(all(feature = "Win32_NetworkManagement_WindowsFilteringPlatform", feature = "Win32_Security"))]
#[inline]
pub unsafe fn FwpmNetEventEnum0<P0, P1>(enginehandle: P0, enumhandle: P1, numentriesrequested: u32, entries: *mut *mut *mut super::super::super::Win32::NetworkManagement::WindowsFilteringPlatform::FWPM_NET_EVENT0, numentriesreturned: *mut u32) -> super::super::super::Win32::Foundation::NTSTATUS
where
    P0: windows_core::Param<super::super::super::Win32::Foundation::HANDLE>,
    P1: windows_core::Param<super::super::super::Win32::Foundation::HANDLE>,
{
    windows_targets::link!("fwpuclnt.dll" "system" fn FwpmNetEventEnum0(enginehandle : super::super::super::Win32::Foundation:: HANDLE, enumhandle : super::super::super::Win32::Foundation:: HANDLE, numentriesrequested : u32, entries : *mut *mut *mut super::super::super::Win32::NetworkManagement::WindowsFilteringPlatform:: FWPM_NET_EVENT0, numentriesreturned : *mut u32) -> super::super::super::Win32::Foundation:: NTSTATUS);
    FwpmNetEventEnum0(enginehandle.param().abi(), enumhandle.param().abi(), numentriesrequested, entries, numentriesreturned)
}
#[cfg(all(feature = "Win32_NetworkManagement_WindowsFilteringPlatform", feature = "Win32_Security"))]
#[inline]
pub unsafe fn FwpmNetEventEnum1<P0, P1>(enginehandle: P0, enumhandle: P1, numentriesrequested: u32, entries: *mut *mut *mut super::super::super::Win32::NetworkManagement::WindowsFilteringPlatform::FWPM_NET_EVENT1, numentriesreturned: *mut u32) -> super::super::super::Win32::Foundation::NTSTATUS
where
    P0: windows_core::Param<super::super::super::Win32::Foundation::HANDLE>,
    P1: windows_core::Param<super::super::super::Win32::Foundation::HANDLE>,
{
    windows_targets::link!("fwpuclnt.dll" "system" fn FwpmNetEventEnum1(enginehandle : super::super::super::Win32::Foundation:: HANDLE, enumhandle : super::super::super::Win32::Foundation:: HANDLE, numentriesrequested : u32, entries : *mut *mut *mut super::super::super::Win32::NetworkManagement::WindowsFilteringPlatform:: FWPM_NET_EVENT1, numentriesreturned : *mut u32) -> super::super::super::Win32::Foundation:: NTSTATUS);
    FwpmNetEventEnum1(enginehandle.param().abi(), enumhandle.param().abi(), numentriesrequested, entries, numentriesreturned)
}
#[cfg(all(feature = "Win32_NetworkManagement_WindowsFilteringPlatform", feature = "Win32_Security"))]
#[inline]
pub unsafe fn FwpmNetEventEnum2<P0, P1>(enginehandle: P0, enumhandle: P1, numentriesrequested: u32, entries: *mut *mut *mut super::super::super::Win32::NetworkManagement::WindowsFilteringPlatform::FWPM_NET_EVENT2, numentriesreturned: *mut u32) -> super::super::super::Win32::Foundation::NTSTATUS
where
    P0: windows_core::Param<super::super::super::Win32::Foundation::HANDLE>,
    P1: windows_core::Param<super::super::super::Win32::Foundation::HANDLE>,
{
    windows_targets::link!("fwpuclnt.dll" "system" fn FwpmNetEventEnum2(enginehandle : super::super::super::Win32::Foundation:: HANDLE, enumhandle : super::super::super::Win32::Foundation:: HANDLE, numentriesrequested : u32, entries : *mut *mut *mut super::super::super::Win32::NetworkManagement::WindowsFilteringPlatform:: FWPM_NET_EVENT2, numentriesreturned : *mut u32) -> super::super::super::Win32::Foundation:: NTSTATUS);
    FwpmNetEventEnum2(enginehandle.param().abi(), enumhandle.param().abi(), numentriesrequested, entries, numentriesreturned)
}
#[cfg(all(feature = "Win32_NetworkManagement_WindowsFilteringPlatform", feature = "Win32_Security"))]
#[inline]
pub unsafe fn FwpmNetEventEnum3<P0, P1>(enginehandle: P0, enumhandle: P1, numentriesrequested: u32, entries: *mut *mut *mut super::super::super::Win32::NetworkManagement::WindowsFilteringPlatform::FWPM_NET_EVENT3, numentriesreturned: *mut u32) -> super::super::super::Win32::Foundation::NTSTATUS
where
    P0: windows_core::Param<super::super::super::Win32::Foundation::HANDLE>,
    P1: windows_core::Param<super::super::super::Win32::Foundation::HANDLE>,
{
    windows_targets::link!("fwpuclnt.dll" "system" fn FwpmNetEventEnum3(enginehandle : super::super::super::Win32::Foundation:: HANDLE, enumhandle : super::super::super::Win32::Foundation:: HANDLE, numentriesrequested : u32, entries : *mut *mut *mut super::super::super::Win32::NetworkManagement::WindowsFilteringPlatform:: FWPM_NET_EVENT3, numentriesreturned : *mut u32) -> super::super::super::Win32::Foundation:: NTSTATUS);
    FwpmNetEventEnum3(enginehandle.param().abi(), enumhandle.param().abi(), numentriesrequested, entries, numentriesreturned)
}
#[cfg(all(feature = "Win32_NetworkManagement_WindowsFilteringPlatform", feature = "Win32_Security"))]
#[inline]
pub unsafe fn FwpmNetEventEnum4<P0, P1>(enginehandle: P0, enumhandle: P1, numentriesrequested: u32, entries: *mut *mut *mut super::super::super::Win32::NetworkManagement::WindowsFilteringPlatform::FWPM_NET_EVENT4, numentriesreturned: *mut u32) -> super::super::super::Win32::Foundation::NTSTATUS
where
    P0: windows_core::Param<super::super::super::Win32::Foundation::HANDLE>,
    P1: windows_core::Param<super::super::super::Win32::Foundation::HANDLE>,
{
    windows_targets::link!("fwpuclnt.dll" "system" fn FwpmNetEventEnum4(enginehandle : super::super::super::Win32::Foundation:: HANDLE, enumhandle : super::super::super::Win32::Foundation:: HANDLE, numentriesrequested : u32, entries : *mut *mut *mut super::super::super::Win32::NetworkManagement::WindowsFilteringPlatform:: FWPM_NET_EVENT4, numentriesreturned : *mut u32) -> super::super::super::Win32::Foundation:: NTSTATUS);
    FwpmNetEventEnum4(enginehandle.param().abi(), enumhandle.param().abi(), numentriesrequested, entries, numentriesreturned)
}
#[cfg(all(feature = "Win32_NetworkManagement_WindowsFilteringPlatform", feature = "Win32_Security"))]
#[inline]
pub unsafe fn FwpmNetEventEnum5<P0, P1>(enginehandle: P0, enumhandle: P1, numentriesrequested: u32, entries: *mut *mut *mut super::super::super::Win32::NetworkManagement::WindowsFilteringPlatform::FWPM_NET_EVENT5, numentriesreturned: *mut u32) -> super::super::super::Win32::Foundation::NTSTATUS
where
    P0: windows_core::Param<super::super::super::Win32::Foundation::HANDLE>,
    P1: windows_core::Param<super::super::super::Win32::Foundation::HANDLE>,
{
    windows_targets::link!("fwpuclnt.dll" "system" fn FwpmNetEventEnum5(enginehandle : super::super::super::Win32::Foundation:: HANDLE, enumhandle : super::super::super::Win32::Foundation:: HANDLE, numentriesrequested : u32, entries : *mut *mut *mut super::super::super::Win32::NetworkManagement::WindowsFilteringPlatform:: FWPM_NET_EVENT5, numentriesreturned : *mut u32) -> super::super::super::Win32::Foundation:: NTSTATUS);
    FwpmNetEventEnum5(enginehandle.param().abi(), enumhandle.param().abi(), numentriesrequested, entries, numentriesreturned)
}
#[cfg(feature = "Win32_Security")]
#[inline]
pub unsafe fn FwpmNetEventsGetSecurityInfo0<P0>(enginehandle: P0, securityinfo: u32, sidowner: *mut super::super::super::Win32::Security::PSID, sidgroup: *mut super::super::super::Win32::Security::PSID, dacl: *mut *mut super::super::super::Win32::Security::ACL, sacl: *mut *mut super::super::super::Win32::Security::ACL, securitydescriptor: *mut super::super::super::Win32::Security::PSECURITY_DESCRIPTOR) -> super::super::super::Win32::Foundation::NTSTATUS
where
    P0: windows_core::Param<super::super::super::Win32::Foundation::HANDLE>,
{
    windows_targets::link!("fwpuclnt.dll" "system" fn FwpmNetEventsGetSecurityInfo0(enginehandle : super::super::super::Win32::Foundation:: HANDLE, securityinfo : u32, sidowner : *mut super::super::super::Win32::Security:: PSID, sidgroup : *mut super::super::super::Win32::Security:: PSID, dacl : *mut *mut super::super::super::Win32::Security:: ACL, sacl : *mut *mut super::super::super::Win32::Security:: ACL, securitydescriptor : *mut super::super::super::Win32::Security:: PSECURITY_DESCRIPTOR) -> super::super::super::Win32::Foundation:: NTSTATUS);
    FwpmNetEventsGetSecurityInfo0(enginehandle.param().abi(), securityinfo, sidowner, sidgroup, dacl, sacl, securitydescriptor)
}
#[cfg(feature = "Win32_Security")]
#[inline]
pub unsafe fn FwpmNetEventsSetSecurityInfo0<P0>(enginehandle: P0, securityinfo: u32, sidowner: Option<*const super::super::super::Win32::Security::SID>, sidgroup: Option<*const super::super::super::Win32::Security::SID>, dacl: Option<*const super::super::super::Win32::Security::ACL>, sacl: Option<*const super::super::super::Win32::Security::ACL>) -> super::super::super::Win32::Foundation::NTSTATUS
where
    P0: windows_core::Param<super::super::super::Win32::Foundation::HANDLE>,
{
    windows_targets::link!("fwpuclnt.dll" "system" fn FwpmNetEventsSetSecurityInfo0(enginehandle : super::super::super::Win32::Foundation:: HANDLE, securityinfo : u32, sidowner : *const super::super::super::Win32::Security:: SID, sidgroup : *const super::super::super::Win32::Security:: SID, dacl : *const super::super::super::Win32::Security:: ACL, sacl : *const super::super::super::Win32::Security:: ACL) -> super::super::super::Win32::Foundation:: NTSTATUS);
    FwpmNetEventsSetSecurityInfo0(enginehandle.param().abi(), securityinfo, core::mem::transmute(sidowner.unwrap_or(std::ptr::null())), core::mem::transmute(sidgroup.unwrap_or(std::ptr::null())), core::mem::transmute(dacl.unwrap_or(std::ptr::null())), core::mem::transmute(sacl.unwrap_or(std::ptr::null())))
}
#[cfg(all(feature = "Win32_NetworkManagement_WindowsFilteringPlatform", feature = "Win32_Security"))]
#[inline]
pub unsafe fn FwpmProviderAdd0<P0, P1>(enginehandle: P0, provider: *const super::super::super::Win32::NetworkManagement::WindowsFilteringPlatform::FWPM_PROVIDER0, sd: P1) -> super::super::super::Win32::Foundation::NTSTATUS
where
    P0: windows_core::Param<super::super::super::Win32::Foundation::HANDLE>,
    P1: windows_core::Param<super::super::super::Win32::Security::PSECURITY_DESCRIPTOR>,
{
    windows_targets::link!("fwpuclnt.dll" "system" fn FwpmProviderAdd0(enginehandle : super::super::super::Win32::Foundation:: HANDLE, provider : *const super::super::super::Win32::NetworkManagement::WindowsFilteringPlatform:: FWPM_PROVIDER0, sd : super::super::super::Win32::Security:: PSECURITY_DESCRIPTOR) -> super::super::super::Win32::Foundation:: NTSTATUS);
    FwpmProviderAdd0(enginehandle.param().abi(), provider, sd.param().abi())
}
#[cfg(all(feature = "Win32_NetworkManagement_WindowsFilteringPlatform", feature = "Win32_Security"))]
#[inline]
pub unsafe fn FwpmProviderContextAdd0<P0, P1>(enginehandle: P0, providercontext: *const super::super::super::Win32::NetworkManagement::WindowsFilteringPlatform::FWPM_PROVIDER_CONTEXT0, sd: P1, id: Option<*mut u64>) -> super::super::super::Win32::Foundation::NTSTATUS
where
    P0: windows_core::Param<super::super::super::Win32::Foundation::HANDLE>,
    P1: windows_core::Param<super::super::super::Win32::Security::PSECURITY_DESCRIPTOR>,
{
    windows_targets::link!("fwpuclnt.dll" "system" fn FwpmProviderContextAdd0(enginehandle : super::super::super::Win32::Foundation:: HANDLE, providercontext : *const super::super::super::Win32::NetworkManagement::WindowsFilteringPlatform:: FWPM_PROVIDER_CONTEXT0, sd : super::super::super::Win32::Security:: PSECURITY_DESCRIPTOR, id : *mut u64) -> super::super::super::Win32::Foundation:: NTSTATUS);
    FwpmProviderContextAdd0(enginehandle.param().abi(), providercontext, sd.param().abi(), core::mem::transmute(id.unwrap_or(std::ptr::null_mut())))
}
#[cfg(all(feature = "Win32_NetworkManagement_WindowsFilteringPlatform", feature = "Win32_Security"))]
#[inline]
pub unsafe fn FwpmProviderContextAdd1<P0, P1>(enginehandle: P0, providercontext: *const super::super::super::Win32::NetworkManagement::WindowsFilteringPlatform::FWPM_PROVIDER_CONTEXT1, sd: P1, id: Option<*mut u64>) -> super::super::super::Win32::Foundation::NTSTATUS
where
    P0: windows_core::Param<super::super::super::Win32::Foundation::HANDLE>,
    P1: windows_core::Param<super::super::super::Win32::Security::PSECURITY_DESCRIPTOR>,
{
    windows_targets::link!("fwpuclnt.dll" "system" fn FwpmProviderContextAdd1(enginehandle : super::super::super::Win32::Foundation:: HANDLE, providercontext : *const super::super::super::Win32::NetworkManagement::WindowsFilteringPlatform:: FWPM_PROVIDER_CONTEXT1, sd : super::super::super::Win32::Security:: PSECURITY_DESCRIPTOR, id : *mut u64) -> super::super::super::Win32::Foundation:: NTSTATUS);
    FwpmProviderContextAdd1(enginehandle.param().abi(), providercontext, sd.param().abi(), core::mem::transmute(id.unwrap_or(std::ptr::null_mut())))
}
#[cfg(all(feature = "Win32_NetworkManagement_WindowsFilteringPlatform", feature = "Win32_Security"))]
#[inline]
pub unsafe fn FwpmProviderContextAdd2<P0, P1>(enginehandle: P0, providercontext: *const super::super::super::Win32::NetworkManagement::WindowsFilteringPlatform::FWPM_PROVIDER_CONTEXT2, sd: P1, id: Option<*mut u64>) -> super::super::super::Win32::Foundation::NTSTATUS
where
    P0: windows_core::Param<super::super::super::Win32::Foundation::HANDLE>,
    P1: windows_core::Param<super::super::super::Win32::Security::PSECURITY_DESCRIPTOR>,
{
    windows_targets::link!("fwpuclnt.dll" "system" fn FwpmProviderContextAdd2(enginehandle : super::super::super::Win32::Foundation:: HANDLE, providercontext : *const super::super::super::Win32::NetworkManagement::WindowsFilteringPlatform:: FWPM_PROVIDER_CONTEXT2, sd : super::super::super::Win32::Security:: PSECURITY_DESCRIPTOR, id : *mut u64) -> super::super::super::Win32::Foundation:: NTSTATUS);
    FwpmProviderContextAdd2(enginehandle.param().abi(), providercontext, sd.param().abi(), core::mem::transmute(id.unwrap_or(std::ptr::null_mut())))
}
#[cfg(all(feature = "Win32_NetworkManagement_WindowsFilteringPlatform", feature = "Win32_Security"))]
#[inline]
pub unsafe fn FwpmProviderContextAdd3<P0, P1>(enginehandle: P0, providercontext: *const super::super::super::Win32::NetworkManagement::WindowsFilteringPlatform::FWPM_PROVIDER_CONTEXT3, sd: P1, id: Option<*mut u64>) -> super::super::super::Win32::Foundation::NTSTATUS
where
    P0: windows_core::Param<super::super::super::Win32::Foundation::HANDLE>,
    P1: windows_core::Param<super::super::super::Win32::Security::PSECURITY_DESCRIPTOR>,
{
    windows_targets::link!("fwpuclnt.dll" "system" fn FwpmProviderContextAdd3(enginehandle : super::super::super::Win32::Foundation:: HANDLE, providercontext : *const super::super::super::Win32::NetworkManagement::WindowsFilteringPlatform:: FWPM_PROVIDER_CONTEXT3, sd : super::super::super::Win32::Security:: PSECURITY_DESCRIPTOR, id : *mut u64) -> super::super::super::Win32::Foundation:: NTSTATUS);
    FwpmProviderContextAdd3(enginehandle.param().abi(), providercontext, sd.param().abi(), core::mem::transmute(id.unwrap_or(std::ptr::null_mut())))
}
#[cfg(feature = "Win32_NetworkManagement_WindowsFilteringPlatform")]
#[inline]
pub unsafe fn FwpmProviderContextCreateEnumHandle0<P0>(enginehandle: P0, enumtemplate: Option<*const super::super::super::Win32::NetworkManagement::WindowsFilteringPlatform::FWPM_PROVIDER_CONTEXT_ENUM_TEMPLATE0>, enumhandle: *mut super::super::super::Win32::Foundation::HANDLE) -> super::super::super::Win32::Foundation::NTSTATUS
where
    P0: windows_core::Param<super::super::super::Win32::Foundation::HANDLE>,
{
    windows_targets::link!("fwpuclnt.dll" "system" fn FwpmProviderContextCreateEnumHandle0(enginehandle : super::super::super::Win32::Foundation:: HANDLE, enumtemplate : *const super::super::super::Win32::NetworkManagement::WindowsFilteringPlatform:: FWPM_PROVIDER_CONTEXT_ENUM_TEMPLATE0, enumhandle : *mut super::super::super::Win32::Foundation:: HANDLE) -> super::super::super::Win32::Foundation:: NTSTATUS);
    FwpmProviderContextCreateEnumHandle0(enginehandle.param().abi(), core::mem::transmute(enumtemplate.unwrap_or(std::ptr::null())), enumhandle)
}
#[inline]
pub unsafe fn FwpmProviderContextDeleteById0<P0>(enginehandle: P0, id: u64) -> super::super::super::Win32::Foundation::NTSTATUS
where
    P0: windows_core::Param<super::super::super::Win32::Foundation::HANDLE>,
{
    windows_targets::link!("fwpuclnt.dll" "system" fn FwpmProviderContextDeleteById0(enginehandle : super::super::super::Win32::Foundation:: HANDLE, id : u64) -> super::super::super::Win32::Foundation:: NTSTATUS);
    FwpmProviderContextDeleteById0(enginehandle.param().abi(), id)
}
#[inline]
pub unsafe fn FwpmProviderContextDeleteByKey0<P0>(enginehandle: P0, key: *const windows_core::GUID) -> super::super::super::Win32::Foundation::NTSTATUS
where
    P0: windows_core::Param<super::super::super::Win32::Foundation::HANDLE>,
{
    windows_targets::link!("fwpuclnt.dll" "system" fn FwpmProviderContextDeleteByKey0(enginehandle : super::super::super::Win32::Foundation:: HANDLE, key : *const windows_core::GUID) -> super::super::super::Win32::Foundation:: NTSTATUS);
    FwpmProviderContextDeleteByKey0(enginehandle.param().abi(), key)
}
#[inline]
pub unsafe fn FwpmProviderContextDestroyEnumHandle0<P0>(enginehandle: P0, enumhandle: super::super::super::Win32::Foundation::HANDLE) -> super::super::super::Win32::Foundation::NTSTATUS
where
    P0: windows_core::Param<super::super::super::Win32::Foundation::HANDLE>,
{
    windows_targets::link!("fwpuclnt.dll" "system" fn FwpmProviderContextDestroyEnumHandle0(enginehandle : super::super::super::Win32::Foundation:: HANDLE, enumhandle : super::super::super::Win32::Foundation:: HANDLE) -> super::super::super::Win32::Foundation:: NTSTATUS);
    FwpmProviderContextDestroyEnumHandle0(enginehandle.param().abi(), enumhandle)
}
#[cfg(all(feature = "Win32_NetworkManagement_WindowsFilteringPlatform", feature = "Win32_Security"))]
#[inline]
pub unsafe fn FwpmProviderContextEnum0<P0, P1>(enginehandle: P0, enumhandle: P1, numentriesrequested: u32, entries: *mut *mut *mut super::super::super::Win32::NetworkManagement::WindowsFilteringPlatform::FWPM_PROVIDER_CONTEXT0, numentriesreturned: *mut u32) -> super::super::super::Win32::Foundation::NTSTATUS
where
    P0: windows_core::Param<super::super::super::Win32::Foundation::HANDLE>,
    P1: windows_core::Param<super::super::super::Win32::Foundation::HANDLE>,
{
    windows_targets::link!("fwpuclnt.dll" "system" fn FwpmProviderContextEnum0(enginehandle : super::super::super::Win32::Foundation:: HANDLE, enumhandle : super::super::super::Win32::Foundation:: HANDLE, numentriesrequested : u32, entries : *mut *mut *mut super::super::super::Win32::NetworkManagement::WindowsFilteringPlatform:: FWPM_PROVIDER_CONTEXT0, numentriesreturned : *mut u32) -> super::super::super::Win32::Foundation:: NTSTATUS);
    FwpmProviderContextEnum0(enginehandle.param().abi(), enumhandle.param().abi(), numentriesrequested, entries, numentriesreturned)
}
#[cfg(all(feature = "Win32_NetworkManagement_WindowsFilteringPlatform", feature = "Win32_Security"))]
#[inline]
pub unsafe fn FwpmProviderContextEnum1<P0, P1>(enginehandle: P0, enumhandle: P1, numentriesrequested: u32, entries: *mut *mut *mut super::super::super::Win32::NetworkManagement::WindowsFilteringPlatform::FWPM_PROVIDER_CONTEXT1, numentriesreturned: *mut u32) -> super::super::super::Win32::Foundation::NTSTATUS
where
    P0: windows_core::Param<super::super::super::Win32::Foundation::HANDLE>,
    P1: windows_core::Param<super::super::super::Win32::Foundation::HANDLE>,
{
    windows_targets::link!("fwpuclnt.dll" "system" fn FwpmProviderContextEnum1(enginehandle : super::super::super::Win32::Foundation:: HANDLE, enumhandle : super::super::super::Win32::Foundation:: HANDLE, numentriesrequested : u32, entries : *mut *mut *mut super::super::super::Win32::NetworkManagement::WindowsFilteringPlatform:: FWPM_PROVIDER_CONTEXT1, numentriesreturned : *mut u32) -> super::super::super::Win32::Foundation:: NTSTATUS);
    FwpmProviderContextEnum1(enginehandle.param().abi(), enumhandle.param().abi(), numentriesrequested, entries, numentriesreturned)
}
#[cfg(all(feature = "Win32_NetworkManagement_WindowsFilteringPlatform", feature = "Win32_Security"))]
#[inline]
pub unsafe fn FwpmProviderContextEnum2<P0, P1>(enginehandle: P0, enumhandle: P1, numentriesrequested: u32, entries: *mut *mut *mut super::super::super::Win32::NetworkManagement::WindowsFilteringPlatform::FWPM_PROVIDER_CONTEXT2, numentriesreturned: *mut u32) -> super::super::super::Win32::Foundation::NTSTATUS
where
    P0: windows_core::Param<super::super::super::Win32::Foundation::HANDLE>,
    P1: windows_core::Param<super::super::super::Win32::Foundation::HANDLE>,
{
    windows_targets::link!("fwpuclnt.dll" "system" fn FwpmProviderContextEnum2(enginehandle : super::super::super::Win32::Foundation:: HANDLE, enumhandle : super::super::super::Win32::Foundation:: HANDLE, numentriesrequested : u32, entries : *mut *mut *mut super::super::super::Win32::NetworkManagement::WindowsFilteringPlatform:: FWPM_PROVIDER_CONTEXT2, numentriesreturned : *mut u32) -> super::super::super::Win32::Foundation:: NTSTATUS);
    FwpmProviderContextEnum2(enginehandle.param().abi(), enumhandle.param().abi(), numentriesrequested, entries, numentriesreturned)
}
#[cfg(all(feature = "Win32_NetworkManagement_WindowsFilteringPlatform", feature = "Win32_Security"))]
#[inline]
pub unsafe fn FwpmProviderContextEnum3<P0, P1>(enginehandle: P0, enumhandle: P1, numentriesrequested: u32, entries: *mut *mut *mut super::super::super::Win32::NetworkManagement::WindowsFilteringPlatform::FWPM_PROVIDER_CONTEXT3, numentriesreturned: *mut u32) -> super::super::super::Win32::Foundation::NTSTATUS
where
    P0: windows_core::Param<super::super::super::Win32::Foundation::HANDLE>,
    P1: windows_core::Param<super::super::super::Win32::Foundation::HANDLE>,
{
    windows_targets::link!("fwpuclnt.dll" "system" fn FwpmProviderContextEnum3(enginehandle : super::super::super::Win32::Foundation:: HANDLE, enumhandle : super::super::super::Win32::Foundation:: HANDLE, numentriesrequested : u32, entries : *mut *mut *mut super::super::super::Win32::NetworkManagement::WindowsFilteringPlatform:: FWPM_PROVIDER_CONTEXT3, numentriesreturned : *mut u32) -> super::super::super::Win32::Foundation:: NTSTATUS);
    FwpmProviderContextEnum3(enginehandle.param().abi(), enumhandle.param().abi(), numentriesrequested, entries, numentriesreturned)
}
#[cfg(all(feature = "Win32_NetworkManagement_WindowsFilteringPlatform", feature = "Win32_Security"))]
#[inline]
pub unsafe fn FwpmProviderContextGetById0<P0>(enginehandle: P0, id: u64, providercontext: *mut *mut super::super::super::Win32::NetworkManagement::WindowsFilteringPlatform::FWPM_PROVIDER_CONTEXT0) -> super::super::super::Win32::Foundation::NTSTATUS
where
    P0: windows_core::Param<super::super::super::Win32::Foundation::HANDLE>,
{
    windows_targets::link!("fwpuclnt.dll" "system" fn FwpmProviderContextGetById0(enginehandle : super::super::super::Win32::Foundation:: HANDLE, id : u64, providercontext : *mut *mut super::super::super::Win32::NetworkManagement::WindowsFilteringPlatform:: FWPM_PROVIDER_CONTEXT0) -> super::super::super::Win32::Foundation:: NTSTATUS);
    FwpmProviderContextGetById0(enginehandle.param().abi(), id, providercontext)
}
#[cfg(all(feature = "Win32_NetworkManagement_WindowsFilteringPlatform", feature = "Win32_Security"))]
#[inline]
pub unsafe fn FwpmProviderContextGetById1<P0>(enginehandle: P0, id: u64, providercontext: *mut *mut super::super::super::Win32::NetworkManagement::WindowsFilteringPlatform::FWPM_PROVIDER_CONTEXT1) -> super::super::super::Win32::Foundation::NTSTATUS
where
    P0: windows_core::Param<super::super::super::Win32::Foundation::HANDLE>,
{
    windows_targets::link!("fwpuclnt.dll" "system" fn FwpmProviderContextGetById1(enginehandle : super::super::super::Win32::Foundation:: HANDLE, id : u64, providercontext : *mut *mut super::super::super::Win32::NetworkManagement::WindowsFilteringPlatform:: FWPM_PROVIDER_CONTEXT1) -> super::super::super::Win32::Foundation:: NTSTATUS);
    FwpmProviderContextGetById1(enginehandle.param().abi(), id, providercontext)
}
#[cfg(all(feature = "Win32_NetworkManagement_WindowsFilteringPlatform", feature = "Win32_Security"))]
#[inline]
pub unsafe fn FwpmProviderContextGetById2<P0>(enginehandle: P0, id: u64, providercontext: *mut *mut super::super::super::Win32::NetworkManagement::WindowsFilteringPlatform::FWPM_PROVIDER_CONTEXT2) -> super::super::super::Win32::Foundation::NTSTATUS
where
    P0: windows_core::Param<super::super::super::Win32::Foundation::HANDLE>,
{
    windows_targets::link!("fwpuclnt.dll" "system" fn FwpmProviderContextGetById2(enginehandle : super::super::super::Win32::Foundation:: HANDLE, id : u64, providercontext : *mut *mut super::super::super::Win32::NetworkManagement::WindowsFilteringPlatform:: FWPM_PROVIDER_CONTEXT2) -> super::super::super::Win32::Foundation:: NTSTATUS);
    FwpmProviderContextGetById2(enginehandle.param().abi(), id, providercontext)
}
#[cfg(all(feature = "Win32_NetworkManagement_WindowsFilteringPlatform", feature = "Win32_Security"))]
#[inline]
pub unsafe fn FwpmProviderContextGetById3<P0>(enginehandle: P0, id: u64, providercontext: *mut *mut super::super::super::Win32::NetworkManagement::WindowsFilteringPlatform::FWPM_PROVIDER_CONTEXT3) -> super::super::super::Win32::Foundation::NTSTATUS
where
    P0: windows_core::Param<super::super::super::Win32::Foundation::HANDLE>,
{
    windows_targets::link!("fwpuclnt.dll" "system" fn FwpmProviderContextGetById3(enginehandle : super::super::super::Win32::Foundation:: HANDLE, id : u64, providercontext : *mut *mut super::super::super::Win32::NetworkManagement::WindowsFilteringPlatform:: FWPM_PROVIDER_CONTEXT3) -> super::super::super::Win32::Foundation:: NTSTATUS);
    FwpmProviderContextGetById3(enginehandle.param().abi(), id, providercontext)
}
#[cfg(all(feature = "Win32_NetworkManagement_WindowsFilteringPlatform", feature = "Win32_Security"))]
#[inline]
pub unsafe fn FwpmProviderContextGetByKey0<P0>(enginehandle: P0, key: *const windows_core::GUID, providercontext: *mut *mut super::super::super::Win32::NetworkManagement::WindowsFilteringPlatform::FWPM_PROVIDER_CONTEXT0) -> super::super::super::Win32::Foundation::NTSTATUS
where
    P0: windows_core::Param<super::super::super::Win32::Foundation::HANDLE>,
{
    windows_targets::link!("fwpuclnt.dll" "system" fn FwpmProviderContextGetByKey0(enginehandle : super::super::super::Win32::Foundation:: HANDLE, key : *const windows_core::GUID, providercontext : *mut *mut super::super::super::Win32::NetworkManagement::WindowsFilteringPlatform:: FWPM_PROVIDER_CONTEXT0) -> super::super::super::Win32::Foundation:: NTSTATUS);
    FwpmProviderContextGetByKey0(enginehandle.param().abi(), key, providercontext)
}
#[cfg(all(feature = "Win32_NetworkManagement_WindowsFilteringPlatform", feature = "Win32_Security"))]
#[inline]
pub unsafe fn FwpmProviderContextGetByKey1<P0>(enginehandle: P0, key: *const windows_core::GUID, providercontext: *mut *mut super::super::super::Win32::NetworkManagement::WindowsFilteringPlatform::FWPM_PROVIDER_CONTEXT1) -> super::super::super::Win32::Foundation::NTSTATUS
where
    P0: windows_core::Param<super::super::super::Win32::Foundation::HANDLE>,
{
    windows_targets::link!("fwpuclnt.dll" "system" fn FwpmProviderContextGetByKey1(enginehandle : super::super::super::Win32::Foundation:: HANDLE, key : *const windows_core::GUID, providercontext : *mut *mut super::super::super::Win32::NetworkManagement::WindowsFilteringPlatform:: FWPM_PROVIDER_CONTEXT1) -> super::super::super::Win32::Foundation:: NTSTATUS);
    FwpmProviderContextGetByKey1(enginehandle.param().abi(), key, providercontext)
}
#[cfg(all(feature = "Win32_NetworkManagement_WindowsFilteringPlatform", feature = "Win32_Security"))]
#[inline]
pub unsafe fn FwpmProviderContextGetByKey2<P0>(enginehandle: P0, key: *const windows_core::GUID, providercontext: *mut *mut super::super::super::Win32::NetworkManagement::WindowsFilteringPlatform::FWPM_PROVIDER_CONTEXT2) -> super::super::super::Win32::Foundation::NTSTATUS
where
    P0: windows_core::Param<super::super::super::Win32::Foundation::HANDLE>,
{
    windows_targets::link!("fwpuclnt.dll" "system" fn FwpmProviderContextGetByKey2(enginehandle : super::super::super::Win32::Foundation:: HANDLE, key : *const windows_core::GUID, providercontext : *mut *mut super::super::super::Win32::NetworkManagement::WindowsFilteringPlatform:: FWPM_PROVIDER_CONTEXT2) -> super::super::super::Win32::Foundation:: NTSTATUS);
    FwpmProviderContextGetByKey2(enginehandle.param().abi(), key, providercontext)
}
#[cfg(all(feature = "Win32_NetworkManagement_WindowsFilteringPlatform", feature = "Win32_Security"))]
#[inline]
pub unsafe fn FwpmProviderContextGetByKey3<P0>(enginehandle: P0, key: *const windows_core::GUID, providercontext: *mut *mut super::super::super::Win32::NetworkManagement::WindowsFilteringPlatform::FWPM_PROVIDER_CONTEXT3) -> super::super::super::Win32::Foundation::NTSTATUS
where
    P0: windows_core::Param<super::super::super::Win32::Foundation::HANDLE>,
{
    windows_targets::link!("fwpuclnt.dll" "system" fn FwpmProviderContextGetByKey3(enginehandle : super::super::super::Win32::Foundation:: HANDLE, key : *const windows_core::GUID, providercontext : *mut *mut super::super::super::Win32::NetworkManagement::WindowsFilteringPlatform:: FWPM_PROVIDER_CONTEXT3) -> super::super::super::Win32::Foundation:: NTSTATUS);
    FwpmProviderContextGetByKey3(enginehandle.param().abi(), key, providercontext)
}
#[cfg(feature = "Win32_Security")]
#[inline]
pub unsafe fn FwpmProviderContextGetSecurityInfoByKey0<P0>(enginehandle: P0, key: Option<*const windows_core::GUID>, securityinfo: u32, sidowner: *mut super::super::super::Win32::Security::PSID, sidgroup: *mut super::super::super::Win32::Security::PSID, dacl: *mut *mut super::super::super::Win32::Security::ACL, sacl: *mut *mut super::super::super::Win32::Security::ACL, securitydescriptor: *mut super::super::super::Win32::Security::PSECURITY_DESCRIPTOR) -> super::super::super::Win32::Foundation::NTSTATUS
where
    P0: windows_core::Param<super::super::super::Win32::Foundation::HANDLE>,
{
    windows_targets::link!("fwpuclnt.dll" "system" fn FwpmProviderContextGetSecurityInfoByKey0(enginehandle : super::super::super::Win32::Foundation:: HANDLE, key : *const windows_core::GUID, securityinfo : u32, sidowner : *mut super::super::super::Win32::Security:: PSID, sidgroup : *mut super::super::super::Win32::Security:: PSID, dacl : *mut *mut super::super::super::Win32::Security:: ACL, sacl : *mut *mut super::super::super::Win32::Security:: ACL, securitydescriptor : *mut super::super::super::Win32::Security:: PSECURITY_DESCRIPTOR) -> super::super::super::Win32::Foundation:: NTSTATUS);
    FwpmProviderContextGetSecurityInfoByKey0(enginehandle.param().abi(), core::mem::transmute(key.unwrap_or(std::ptr::null())), securityinfo, sidowner, sidgroup, dacl, sacl, securitydescriptor)
}
#[cfg(feature = "Win32_Security")]
#[inline]
pub unsafe fn FwpmProviderContextSetSecurityInfoByKey0<P0>(enginehandle: P0, key: Option<*const windows_core::GUID>, securityinfo: u32, sidowner: Option<*const super::super::super::Win32::Security::SID>, sidgroup: Option<*const super::super::super::Win32::Security::SID>, dacl: Option<*const super::super::super::Win32::Security::ACL>, sacl: Option<*const super::super::super::Win32::Security::ACL>) -> super::super::super::Win32::Foundation::NTSTATUS
where
    P0: windows_core::Param<super::super::super::Win32::Foundation::HANDLE>,
{
    windows_targets::link!("fwpuclnt.dll" "system" fn FwpmProviderContextSetSecurityInfoByKey0(enginehandle : super::super::super::Win32::Foundation:: HANDLE, key : *const windows_core::GUID, securityinfo : u32, sidowner : *const super::super::super::Win32::Security:: SID, sidgroup : *const super::super::super::Win32::Security:: SID, dacl : *const super::super::super::Win32::Security:: ACL, sacl : *const super::super::super::Win32::Security:: ACL) -> super::super::super::Win32::Foundation:: NTSTATUS);
    FwpmProviderContextSetSecurityInfoByKey0(enginehandle.param().abi(), core::mem::transmute(key.unwrap_or(std::ptr::null())), securityinfo, core::mem::transmute(sidowner.unwrap_or(std::ptr::null())), core::mem::transmute(sidgroup.unwrap_or(std::ptr::null())), core::mem::transmute(dacl.unwrap_or(std::ptr::null())), core::mem::transmute(sacl.unwrap_or(std::ptr::null())))
}
#[cfg(feature = "Win32_NetworkManagement_WindowsFilteringPlatform")]
#[inline]
pub unsafe fn FwpmProviderCreateEnumHandle0<P0>(enginehandle: P0, enumtemplate: Option<*const super::super::super::Win32::NetworkManagement::WindowsFilteringPlatform::FWPM_PROVIDER_ENUM_TEMPLATE0>, enumhandle: *mut super::super::super::Win32::Foundation::HANDLE) -> super::super::super::Win32::Foundation::NTSTATUS
where
    P0: windows_core::Param<super::super::super::Win32::Foundation::HANDLE>,
{
    windows_targets::link!("fwpuclnt.dll" "system" fn FwpmProviderCreateEnumHandle0(enginehandle : super::super::super::Win32::Foundation:: HANDLE, enumtemplate : *const super::super::super::Win32::NetworkManagement::WindowsFilteringPlatform:: FWPM_PROVIDER_ENUM_TEMPLATE0, enumhandle : *mut super::super::super::Win32::Foundation:: HANDLE) -> super::super::super::Win32::Foundation:: NTSTATUS);
    FwpmProviderCreateEnumHandle0(enginehandle.param().abi(), core::mem::transmute(enumtemplate.unwrap_or(std::ptr::null())), enumhandle)
}
#[inline]
pub unsafe fn FwpmProviderDeleteByKey0<P0>(enginehandle: P0, key: *const windows_core::GUID) -> super::super::super::Win32::Foundation::NTSTATUS
where
    P0: windows_core::Param<super::super::super::Win32::Foundation::HANDLE>,
{
    windows_targets::link!("fwpuclnt.dll" "system" fn FwpmProviderDeleteByKey0(enginehandle : super::super::super::Win32::Foundation:: HANDLE, key : *const windows_core::GUID) -> super::super::super::Win32::Foundation:: NTSTATUS);
    FwpmProviderDeleteByKey0(enginehandle.param().abi(), key)
}
#[inline]
pub unsafe fn FwpmProviderDestroyEnumHandle0<P0>(enginehandle: P0, enumhandle: super::super::super::Win32::Foundation::HANDLE) -> super::super::super::Win32::Foundation::NTSTATUS
where
    P0: windows_core::Param<super::super::super::Win32::Foundation::HANDLE>,
{
    windows_targets::link!("fwpuclnt.dll" "system" fn FwpmProviderDestroyEnumHandle0(enginehandle : super::super::super::Win32::Foundation:: HANDLE, enumhandle : super::super::super::Win32::Foundation:: HANDLE) -> super::super::super::Win32::Foundation:: NTSTATUS);
    FwpmProviderDestroyEnumHandle0(enginehandle.param().abi(), enumhandle)
}
#[cfg(feature = "Win32_NetworkManagement_WindowsFilteringPlatform")]
#[inline]
pub unsafe fn FwpmProviderEnum0<P0, P1>(enginehandle: P0, enumhandle: P1, numentriesrequested: u32, entries: *mut *mut *mut super::super::super::Win32::NetworkManagement::WindowsFilteringPlatform::FWPM_PROVIDER0, numentriesreturned: *mut u32) -> super::super::super::Win32::Foundation::NTSTATUS
where
    P0: windows_core::Param<super::super::super::Win32::Foundation::HANDLE>,
    P1: windows_core::Param<super::super::super::Win32::Foundation::HANDLE>,
{
    windows_targets::link!("fwpuclnt.dll" "system" fn FwpmProviderEnum0(enginehandle : super::super::super::Win32::Foundation:: HANDLE, enumhandle : super::super::super::Win32::Foundation:: HANDLE, numentriesrequested : u32, entries : *mut *mut *mut super::super::super::Win32::NetworkManagement::WindowsFilteringPlatform:: FWPM_PROVIDER0, numentriesreturned : *mut u32) -> super::super::super::Win32::Foundation:: NTSTATUS);
    FwpmProviderEnum0(enginehandle.param().abi(), enumhandle.param().abi(), numentriesrequested, entries, numentriesreturned)
}
#[cfg(feature = "Win32_NetworkManagement_WindowsFilteringPlatform")]
#[inline]
pub unsafe fn FwpmProviderGetByKey0<P0>(enginehandle: P0, key: *const windows_core::GUID, provider: *mut *mut super::super::super::Win32::NetworkManagement::WindowsFilteringPlatform::FWPM_PROVIDER0) -> super::super::super::Win32::Foundation::NTSTATUS
where
    P0: windows_core::Param<super::super::super::Win32::Foundation::HANDLE>,
{
    windows_targets::link!("fwpuclnt.dll" "system" fn FwpmProviderGetByKey0(enginehandle : super::super::super::Win32::Foundation:: HANDLE, key : *const windows_core::GUID, provider : *mut *mut super::super::super::Win32::NetworkManagement::WindowsFilteringPlatform:: FWPM_PROVIDER0) -> super::super::super::Win32::Foundation:: NTSTATUS);
    FwpmProviderGetByKey0(enginehandle.param().abi(), key, provider)
}
#[cfg(feature = "Win32_Security")]
#[inline]
pub unsafe fn FwpmProviderGetSecurityInfoByKey0<P0>(enginehandle: P0, key: Option<*const windows_core::GUID>, securityinfo: u32, sidowner: *mut super::super::super::Win32::Security::PSID, sidgroup: *mut super::super::super::Win32::Security::PSID, dacl: *mut *mut super::super::super::Win32::Security::ACL, sacl: *mut *mut super::super::super::Win32::Security::ACL, securitydescriptor: *mut super::super::super::Win32::Security::PSECURITY_DESCRIPTOR) -> super::super::super::Win32::Foundation::NTSTATUS
where
    P0: windows_core::Param<super::super::super::Win32::Foundation::HANDLE>,
{
    windows_targets::link!("fwpuclnt.dll" "system" fn FwpmProviderGetSecurityInfoByKey0(enginehandle : super::super::super::Win32::Foundation:: HANDLE, key : *const windows_core::GUID, securityinfo : u32, sidowner : *mut super::super::super::Win32::Security:: PSID, sidgroup : *mut super::super::super::Win32::Security:: PSID, dacl : *mut *mut super::super::super::Win32::Security:: ACL, sacl : *mut *mut super::super::super::Win32::Security:: ACL, securitydescriptor : *mut super::super::super::Win32::Security:: PSECURITY_DESCRIPTOR) -> super::super::super::Win32::Foundation:: NTSTATUS);
    FwpmProviderGetSecurityInfoByKey0(enginehandle.param().abi(), core::mem::transmute(key.unwrap_or(std::ptr::null())), securityinfo, sidowner, sidgroup, dacl, sacl, securitydescriptor)
}
#[cfg(feature = "Win32_Security")]
#[inline]
pub unsafe fn FwpmProviderSetSecurityInfoByKey0<P0>(enginehandle: P0, key: Option<*const windows_core::GUID>, securityinfo: u32, sidowner: Option<*const super::super::super::Win32::Security::SID>, sidgroup: Option<*const super::super::super::Win32::Security::SID>, dacl: Option<*const super::super::super::Win32::Security::ACL>, sacl: Option<*const super::super::super::Win32::Security::ACL>) -> super::super::super::Win32::Foundation::NTSTATUS
where
    P0: windows_core::Param<super::super::super::Win32::Foundation::HANDLE>,
{
    windows_targets::link!("fwpuclnt.dll" "system" fn FwpmProviderSetSecurityInfoByKey0(enginehandle : super::super::super::Win32::Foundation:: HANDLE, key : *const windows_core::GUID, securityinfo : u32, sidowner : *const super::super::super::Win32::Security:: SID, sidgroup : *const super::super::super::Win32::Security:: SID, dacl : *const super::super::super::Win32::Security:: ACL, sacl : *const super::super::super::Win32::Security:: ACL) -> super::super::super::Win32::Foundation:: NTSTATUS);
    FwpmProviderSetSecurityInfoByKey0(enginehandle.param().abi(), core::mem::transmute(key.unwrap_or(std::ptr::null())), securityinfo, core::mem::transmute(sidowner.unwrap_or(std::ptr::null())), core::mem::transmute(sidgroup.unwrap_or(std::ptr::null())), core::mem::transmute(dacl.unwrap_or(std::ptr::null())), core::mem::transmute(sacl.unwrap_or(std::ptr::null())))
}
#[cfg(feature = "Win32_NetworkManagement_WindowsFilteringPlatform")]
#[inline]
pub unsafe fn FwpmSessionCreateEnumHandle0<P0>(enginehandle: P0, enumtemplate: Option<*const super::super::super::Win32::NetworkManagement::WindowsFilteringPlatform::FWPM_SESSION_ENUM_TEMPLATE0>, enumhandle: *mut super::super::super::Win32::Foundation::HANDLE) -> super::super::super::Win32::Foundation::NTSTATUS
where
    P0: windows_core::Param<super::super::super::Win32::Foundation::HANDLE>,
{
    windows_targets::link!("fwpuclnt.dll" "system" fn FwpmSessionCreateEnumHandle0(enginehandle : super::super::super::Win32::Foundation:: HANDLE, enumtemplate : *const super::super::super::Win32::NetworkManagement::WindowsFilteringPlatform:: FWPM_SESSION_ENUM_TEMPLATE0, enumhandle : *mut super::super::super::Win32::Foundation:: HANDLE) -> super::super::super::Win32::Foundation:: NTSTATUS);
    FwpmSessionCreateEnumHandle0(enginehandle.param().abi(), core::mem::transmute(enumtemplate.unwrap_or(std::ptr::null())), enumhandle)
}
#[inline]
pub unsafe fn FwpmSessionDestroyEnumHandle0<P0>(enginehandle: P0, enumhandle: super::super::super::Win32::Foundation::HANDLE) -> super::super::super::Win32::Foundation::NTSTATUS
where
    P0: windows_core::Param<super::super::super::Win32::Foundation::HANDLE>,
{
    windows_targets::link!("fwpuclnt.dll" "system" fn FwpmSessionDestroyEnumHandle0(enginehandle : super::super::super::Win32::Foundation:: HANDLE, enumhandle : super::super::super::Win32::Foundation:: HANDLE) -> super::super::super::Win32::Foundation:: NTSTATUS);
    FwpmSessionDestroyEnumHandle0(enginehandle.param().abi(), enumhandle)
}
#[cfg(all(feature = "Win32_NetworkManagement_WindowsFilteringPlatform", feature = "Win32_Security"))]
#[inline]
pub unsafe fn FwpmSessionEnum0<P0, P1>(enginehandle: P0, enumhandle: P1, numentriesrequested: u32, entries: *mut *mut *mut super::super::super::Win32::NetworkManagement::WindowsFilteringPlatform::FWPM_SESSION0, numentriesreturned: *mut u32) -> super::super::super::Win32::Foundation::NTSTATUS
where
    P0: windows_core::Param<super::super::super::Win32::Foundation::HANDLE>,
    P1: windows_core::Param<super::super::super::Win32::Foundation::HANDLE>,
{
    windows_targets::link!("fwpuclnt.dll" "system" fn FwpmSessionEnum0(enginehandle : super::super::super::Win32::Foundation:: HANDLE, enumhandle : super::super::super::Win32::Foundation:: HANDLE, numentriesrequested : u32, entries : *mut *mut *mut super::super::super::Win32::NetworkManagement::WindowsFilteringPlatform:: FWPM_SESSION0, numentriesreturned : *mut u32) -> super::super::super::Win32::Foundation:: NTSTATUS);
    FwpmSessionEnum0(enginehandle.param().abi(), enumhandle.param().abi(), numentriesrequested, entries, numentriesreturned)
}
#[cfg(all(feature = "Win32_NetworkManagement_WindowsFilteringPlatform", feature = "Win32_Security"))]
#[inline]
pub unsafe fn FwpmSubLayerAdd0<P0, P1>(enginehandle: P0, sublayer: *const super::super::super::Win32::NetworkManagement::WindowsFilteringPlatform::FWPM_SUBLAYER0, sd: P1) -> super::super::super::Win32::Foundation::NTSTATUS
where
    P0: windows_core::Param<super::super::super::Win32::Foundation::HANDLE>,
    P1: windows_core::Param<super::super::super::Win32::Security::PSECURITY_DESCRIPTOR>,
{
    windows_targets::link!("fwpuclnt.dll" "system" fn FwpmSubLayerAdd0(enginehandle : super::super::super::Win32::Foundation:: HANDLE, sublayer : *const super::super::super::Win32::NetworkManagement::WindowsFilteringPlatform:: FWPM_SUBLAYER0, sd : super::super::super::Win32::Security:: PSECURITY_DESCRIPTOR) -> super::super::super::Win32::Foundation:: NTSTATUS);
    FwpmSubLayerAdd0(enginehandle.param().abi(), sublayer, sd.param().abi())
}
#[cfg(feature = "Win32_NetworkManagement_WindowsFilteringPlatform")]
#[inline]
pub unsafe fn FwpmSubLayerCreateEnumHandle0<P0>(enginehandle: P0, enumtemplate: Option<*const super::super::super::Win32::NetworkManagement::WindowsFilteringPlatform::FWPM_SUBLAYER_ENUM_TEMPLATE0>, enumhandle: *mut super::super::super::Win32::Foundation::HANDLE) -> super::super::super::Win32::Foundation::NTSTATUS
where
    P0: windows_core::Param<super::super::super::Win32::Foundation::HANDLE>,
{
    windows_targets::link!("fwpuclnt.dll" "system" fn FwpmSubLayerCreateEnumHandle0(enginehandle : super::super::super::Win32::Foundation:: HANDLE, enumtemplate : *const super::super::super::Win32::NetworkManagement::WindowsFilteringPlatform:: FWPM_SUBLAYER_ENUM_TEMPLATE0, enumhandle : *mut super::super::super::Win32::Foundation:: HANDLE) -> super::super::super::Win32::Foundation:: NTSTATUS);
    FwpmSubLayerCreateEnumHandle0(enginehandle.param().abi(), core::mem::transmute(enumtemplate.unwrap_or(std::ptr::null())), enumhandle)
}
#[inline]
pub unsafe fn FwpmSubLayerDeleteByKey0<P0>(enginehandle: P0, key: *const windows_core::GUID) -> super::super::super::Win32::Foundation::NTSTATUS
where
    P0: windows_core::Param<super::super::super::Win32::Foundation::HANDLE>,
{
    windows_targets::link!("fwpuclnt.dll" "system" fn FwpmSubLayerDeleteByKey0(enginehandle : super::super::super::Win32::Foundation:: HANDLE, key : *const windows_core::GUID) -> super::super::super::Win32::Foundation:: NTSTATUS);
    FwpmSubLayerDeleteByKey0(enginehandle.param().abi(), key)
}
#[inline]
pub unsafe fn FwpmSubLayerDestroyEnumHandle0<P0>(enginehandle: P0, enumhandle: super::super::super::Win32::Foundation::HANDLE) -> super::super::super::Win32::Foundation::NTSTATUS
where
    P0: windows_core::Param<super::super::super::Win32::Foundation::HANDLE>,
{
    windows_targets::link!("fwpuclnt.dll" "system" fn FwpmSubLayerDestroyEnumHandle0(enginehandle : super::super::super::Win32::Foundation:: HANDLE, enumhandle : super::super::super::Win32::Foundation:: HANDLE) -> super::super::super::Win32::Foundation:: NTSTATUS);
    FwpmSubLayerDestroyEnumHandle0(enginehandle.param().abi(), enumhandle)
}
#[cfg(feature = "Win32_NetworkManagement_WindowsFilteringPlatform")]
#[inline]
pub unsafe fn FwpmSubLayerEnum0<P0, P1>(enginehandle: P0, enumhandle: P1, numentriesrequested: u32, entries: *mut *mut *mut super::super::super::Win32::NetworkManagement::WindowsFilteringPlatform::FWPM_SUBLAYER0, numentriesreturned: *mut u32) -> super::super::super::Win32::Foundation::NTSTATUS
where
    P0: windows_core::Param<super::super::super::Win32::Foundation::HANDLE>,
    P1: windows_core::Param<super::super::super::Win32::Foundation::HANDLE>,
{
    windows_targets::link!("fwpuclnt.dll" "system" fn FwpmSubLayerEnum0(enginehandle : super::super::super::Win32::Foundation:: HANDLE, enumhandle : super::super::super::Win32::Foundation:: HANDLE, numentriesrequested : u32, entries : *mut *mut *mut super::super::super::Win32::NetworkManagement::WindowsFilteringPlatform:: FWPM_SUBLAYER0, numentriesreturned : *mut u32) -> super::super::super::Win32::Foundation:: NTSTATUS);
    FwpmSubLayerEnum0(enginehandle.param().abi(), enumhandle.param().abi(), numentriesrequested, entries, numentriesreturned)
}
#[cfg(feature = "Win32_NetworkManagement_WindowsFilteringPlatform")]
#[inline]
pub unsafe fn FwpmSubLayerGetByKey0<P0>(enginehandle: P0, key: *const windows_core::GUID, sublayer: *mut *mut super::super::super::Win32::NetworkManagement::WindowsFilteringPlatform::FWPM_SUBLAYER0) -> super::super::super::Win32::Foundation::NTSTATUS
where
    P0: windows_core::Param<super::super::super::Win32::Foundation::HANDLE>,
{
    windows_targets::link!("fwpuclnt.dll" "system" fn FwpmSubLayerGetByKey0(enginehandle : super::super::super::Win32::Foundation:: HANDLE, key : *const windows_core::GUID, sublayer : *mut *mut super::super::super::Win32::NetworkManagement::WindowsFilteringPlatform:: FWPM_SUBLAYER0) -> super::super::super::Win32::Foundation:: NTSTATUS);
    FwpmSubLayerGetByKey0(enginehandle.param().abi(), key, sublayer)
}
#[cfg(feature = "Win32_Security")]
#[inline]
pub unsafe fn FwpmSubLayerGetSecurityInfoByKey0<P0>(enginehandle: P0, key: Option<*const windows_core::GUID>, securityinfo: u32, sidowner: *mut super::super::super::Win32::Security::PSID, sidgroup: *mut super::super::super::Win32::Security::PSID, dacl: *mut *mut super::super::super::Win32::Security::ACL, sacl: *mut *mut super::super::super::Win32::Security::ACL, securitydescriptor: *mut super::super::super::Win32::Security::PSECURITY_DESCRIPTOR) -> super::super::super::Win32::Foundation::NTSTATUS
where
    P0: windows_core::Param<super::super::super::Win32::Foundation::HANDLE>,
{
    windows_targets::link!("fwpuclnt.dll" "system" fn FwpmSubLayerGetSecurityInfoByKey0(enginehandle : super::super::super::Win32::Foundation:: HANDLE, key : *const windows_core::GUID, securityinfo : u32, sidowner : *mut super::super::super::Win32::Security:: PSID, sidgroup : *mut super::super::super::Win32::Security:: PSID, dacl : *mut *mut super::super::super::Win32::Security:: ACL, sacl : *mut *mut super::super::super::Win32::Security:: ACL, securitydescriptor : *mut super::super::super::Win32::Security:: PSECURITY_DESCRIPTOR) -> super::super::super::Win32::Foundation:: NTSTATUS);
    FwpmSubLayerGetSecurityInfoByKey0(enginehandle.param().abi(), core::mem::transmute(key.unwrap_or(std::ptr::null())), securityinfo, sidowner, sidgroup, dacl, sacl, securitydescriptor)
}
#[cfg(feature = "Win32_Security")]
#[inline]
pub unsafe fn FwpmSubLayerSetSecurityInfoByKey0<P0>(enginehandle: P0, key: Option<*const windows_core::GUID>, securityinfo: u32, sidowner: Option<*const super::super::super::Win32::Security::SID>, sidgroup: Option<*const super::super::super::Win32::Security::SID>, dacl: Option<*const super::super::super::Win32::Security::ACL>, sacl: Option<*const super::super::super::Win32::Security::ACL>) -> super::super::super::Win32::Foundation::NTSTATUS
where
    P0: windows_core::Param<super::super::super::Win32::Foundation::HANDLE>,
{
    windows_targets::link!("fwpuclnt.dll" "system" fn FwpmSubLayerSetSecurityInfoByKey0(enginehandle : super::super::super::Win32::Foundation:: HANDLE, key : *const windows_core::GUID, securityinfo : u32, sidowner : *const super::super::super::Win32::Security:: SID, sidgroup : *const super::super::super::Win32::Security:: SID, dacl : *const super::super::super::Win32::Security:: ACL, sacl : *const super::super::super::Win32::Security:: ACL) -> super::super::super::Win32::Foundation:: NTSTATUS);
    FwpmSubLayerSetSecurityInfoByKey0(enginehandle.param().abi(), core::mem::transmute(key.unwrap_or(std::ptr::null())), securityinfo, core::mem::transmute(sidowner.unwrap_or(std::ptr::null())), core::mem::transmute(sidgroup.unwrap_or(std::ptr::null())), core::mem::transmute(dacl.unwrap_or(std::ptr::null())), core::mem::transmute(sacl.unwrap_or(std::ptr::null())))
}
#[inline]
pub unsafe fn FwpmTransactionAbort0<P0>(enginehandle: P0) -> super::super::super::Win32::Foundation::NTSTATUS
where
    P0: windows_core::Param<super::super::super::Win32::Foundation::HANDLE>,
{
    windows_targets::link!("fwpuclnt.dll" "system" fn FwpmTransactionAbort0(enginehandle : super::super::super::Win32::Foundation:: HANDLE) -> super::super::super::Win32::Foundation:: NTSTATUS);
    FwpmTransactionAbort0(enginehandle.param().abi())
}
#[inline]
pub unsafe fn FwpmTransactionBegin0<P0>(enginehandle: P0, flags: u32) -> super::super::super::Win32::Foundation::NTSTATUS
where
    P0: windows_core::Param<super::super::super::Win32::Foundation::HANDLE>,
{
    windows_targets::link!("fwpuclnt.dll" "system" fn FwpmTransactionBegin0(enginehandle : super::super::super::Win32::Foundation:: HANDLE, flags : u32) -> super::super::super::Win32::Foundation:: NTSTATUS);
    FwpmTransactionBegin0(enginehandle.param().abi(), flags)
}
#[inline]
pub unsafe fn FwpmTransactionCommit0<P0>(enginehandle: P0) -> super::super::super::Win32::Foundation::NTSTATUS
where
    P0: windows_core::Param<super::super::super::Win32::Foundation::HANDLE>,
{
    windows_targets::link!("fwpuclnt.dll" "system" fn FwpmTransactionCommit0(enginehandle : super::super::super::Win32::Foundation:: HANDLE) -> super::super::super::Win32::Foundation:: NTSTATUS);
    FwpmTransactionCommit0(enginehandle.param().abi())
}
#[cfg(feature = "Win32_Security")]
#[inline]
pub unsafe fn FwpmvSwitchEventsGetSecurityInfo0<P0>(enginehandle: P0, securityinfo: u32, sidowner: *mut super::super::super::Win32::Security::PSID, sidgroup: *mut super::super::super::Win32::Security::PSID, dacl: *mut *mut super::super::super::Win32::Security::ACL, sacl: *mut *mut super::super::super::Win32::Security::ACL, securitydescriptor: *mut super::super::super::Win32::Security::PSECURITY_DESCRIPTOR) -> super::super::super::Win32::Foundation::NTSTATUS
where
    P0: windows_core::Param<super::super::super::Win32::Foundation::HANDLE>,
{
    windows_targets::link!("fwpuclnt.dll" "system" fn FwpmvSwitchEventsGetSecurityInfo0(enginehandle : super::super::super::Win32::Foundation:: HANDLE, securityinfo : u32, sidowner : *mut super::super::super::Win32::Security:: PSID, sidgroup : *mut super::super::super::Win32::Security:: PSID, dacl : *mut *mut super::super::super::Win32::Security:: ACL, sacl : *mut *mut super::super::super::Win32::Security:: ACL, securitydescriptor : *mut super::super::super::Win32::Security:: PSECURITY_DESCRIPTOR) -> super::super::super::Win32::Foundation:: NTSTATUS);
    FwpmvSwitchEventsGetSecurityInfo0(enginehandle.param().abi(), securityinfo, sidowner, sidgroup, dacl, sacl, securitydescriptor)
}
#[cfg(feature = "Win32_Security")]
#[inline]
pub unsafe fn FwpmvSwitchEventsSetSecurityInfo0<P0>(enginehandle: P0, securityinfo: u32, sidowner: Option<*const super::super::super::Win32::Security::SID>, sidgroup: Option<*const super::super::super::Win32::Security::SID>, dacl: Option<*const super::super::super::Win32::Security::ACL>, sacl: Option<*const super::super::super::Win32::Security::ACL>) -> super::super::super::Win32::Foundation::NTSTATUS
where
    P0: windows_core::Param<super::super::super::Win32::Foundation::HANDLE>,
{
    windows_targets::link!("fwpuclnt.dll" "system" fn FwpmvSwitchEventsSetSecurityInfo0(enginehandle : super::super::super::Win32::Foundation:: HANDLE, securityinfo : u32, sidowner : *const super::super::super::Win32::Security:: SID, sidgroup : *const super::super::super::Win32::Security:: SID, dacl : *const super::super::super::Win32::Security:: ACL, sacl : *const super::super::super::Win32::Security:: ACL) -> super::super::super::Win32::Foundation:: NTSTATUS);
    FwpmvSwitchEventsSetSecurityInfo0(enginehandle.param().abi(), securityinfo, core::mem::transmute(sidowner.unwrap_or(std::ptr::null())), core::mem::transmute(sidgroup.unwrap_or(std::ptr::null())), core::mem::transmute(dacl.unwrap_or(std::ptr::null())), core::mem::transmute(sacl.unwrap_or(std::ptr::null())))
}
#[cfg(feature = "Win32_Security")]
#[inline]
pub unsafe fn IPsecDospGetSecurityInfo0<P0>(enginehandle: P0, securityinfo: u32, sidowner: *mut super::super::super::Win32::Security::PSID, sidgroup: *mut super::super::super::Win32::Security::PSID, dacl: *mut *mut super::super::super::Win32::Security::ACL, sacl: *mut *mut super::super::super::Win32::Security::ACL, securitydescriptor: *mut super::super::super::Win32::Security::PSECURITY_DESCRIPTOR) -> super::super::super::Win32::Foundation::NTSTATUS
where
    P0: windows_core::Param<super::super::super::Win32::Foundation::HANDLE>,
{
    windows_targets::link!("fwpuclnt.dll" "system" fn IPsecDospGetSecurityInfo0(enginehandle : super::super::super::Win32::Foundation:: HANDLE, securityinfo : u32, sidowner : *mut super::super::super::Win32::Security:: PSID, sidgroup : *mut super::super::super::Win32::Security:: PSID, dacl : *mut *mut super::super::super::Win32::Security:: ACL, sacl : *mut *mut super::super::super::Win32::Security:: ACL, securitydescriptor : *mut super::super::super::Win32::Security:: PSECURITY_DESCRIPTOR) -> super::super::super::Win32::Foundation:: NTSTATUS);
    IPsecDospGetSecurityInfo0(enginehandle.param().abi(), securityinfo, sidowner, sidgroup, dacl, sacl, securitydescriptor)
}
#[cfg(feature = "Win32_NetworkManagement_WindowsFilteringPlatform")]
#[inline]
pub unsafe fn IPsecDospGetStatistics0<P0>(enginehandle: P0, idpstatistics: *mut super::super::super::Win32::NetworkManagement::WindowsFilteringPlatform::IPSEC_DOSP_STATISTICS0) -> super::super::super::Win32::Foundation::NTSTATUS
where
    P0: windows_core::Param<super::super::super::Win32::Foundation::HANDLE>,
{
    windows_targets::link!("fwpuclnt.dll" "system" fn IPsecDospGetStatistics0(enginehandle : super::super::super::Win32::Foundation:: HANDLE, idpstatistics : *mut super::super::super::Win32::NetworkManagement::WindowsFilteringPlatform:: IPSEC_DOSP_STATISTICS0) -> super::super::super::Win32::Foundation:: NTSTATUS);
    IPsecDospGetStatistics0(enginehandle.param().abi(), idpstatistics)
}
#[cfg(feature = "Win32_Security")]
#[inline]
pub unsafe fn IPsecDospSetSecurityInfo0<P0>(enginehandle: P0, securityinfo: u32, sidowner: Option<*const super::super::super::Win32::Security::SID>, sidgroup: Option<*const super::super::super::Win32::Security::SID>, dacl: Option<*const super::super::super::Win32::Security::ACL>, sacl: Option<*const super::super::super::Win32::Security::ACL>) -> super::super::super::Win32::Foundation::NTSTATUS
where
    P0: windows_core::Param<super::super::super::Win32::Foundation::HANDLE>,
{
    windows_targets::link!("fwpuclnt.dll" "system" fn IPsecDospSetSecurityInfo0(enginehandle : super::super::super::Win32::Foundation:: HANDLE, securityinfo : u32, sidowner : *const super::super::super::Win32::Security:: SID, sidgroup : *const super::super::super::Win32::Security:: SID, dacl : *const super::super::super::Win32::Security:: ACL, sacl : *const super::super::super::Win32::Security:: ACL) -> super::super::super::Win32::Foundation:: NTSTATUS);
    IPsecDospSetSecurityInfo0(enginehandle.param().abi(), securityinfo, core::mem::transmute(sidowner.unwrap_or(std::ptr::null())), core::mem::transmute(sidgroup.unwrap_or(std::ptr::null())), core::mem::transmute(dacl.unwrap_or(std::ptr::null())), core::mem::transmute(sacl.unwrap_or(std::ptr::null())))
}
#[cfg(feature = "Win32_NetworkManagement_WindowsFilteringPlatform")]
#[inline]
pub unsafe fn IPsecDospStateCreateEnumHandle0<P0>(enginehandle: P0, enumtemplate: Option<*const super::super::super::Win32::NetworkManagement::WindowsFilteringPlatform::IPSEC_DOSP_STATE_ENUM_TEMPLATE0>, enumhandle: *mut super::super::super::Win32::Foundation::HANDLE) -> super::super::super::Win32::Foundation::NTSTATUS
where
    P0: windows_core::Param<super::super::super::Win32::Foundation::HANDLE>,
{
    windows_targets::link!("fwpuclnt.dll" "system" fn IPsecDospStateCreateEnumHandle0(enginehandle : super::super::super::Win32::Foundation:: HANDLE, enumtemplate : *const super::super::super::Win32::NetworkManagement::WindowsFilteringPlatform:: IPSEC_DOSP_STATE_ENUM_TEMPLATE0, enumhandle : *mut super::super::super::Win32::Foundation:: HANDLE) -> super::super::super::Win32::Foundation:: NTSTATUS);
    IPsecDospStateCreateEnumHandle0(enginehandle.param().abi(), core::mem::transmute(enumtemplate.unwrap_or(std::ptr::null())), enumhandle)
}
#[inline]
pub unsafe fn IPsecDospStateDestroyEnumHandle0<P0>(enginehandle: P0, enumhandle: super::super::super::Win32::Foundation::HANDLE) -> super::super::super::Win32::Foundation::NTSTATUS
where
    P0: windows_core::Param<super::super::super::Win32::Foundation::HANDLE>,
{
    windows_targets::link!("fwpuclnt.dll" "system" fn IPsecDospStateDestroyEnumHandle0(enginehandle : super::super::super::Win32::Foundation:: HANDLE, enumhandle : super::super::super::Win32::Foundation:: HANDLE) -> super::super::super::Win32::Foundation:: NTSTATUS);
    IPsecDospStateDestroyEnumHandle0(enginehandle.param().abi(), enumhandle)
}
#[cfg(feature = "Win32_NetworkManagement_WindowsFilteringPlatform")]
#[inline]
pub unsafe fn IPsecDospStateEnum0<P0, P1>(enginehandle: P0, enumhandle: P1, numentriesrequested: u32, entries: *mut *mut *mut super::super::super::Win32::NetworkManagement::WindowsFilteringPlatform::IPSEC_DOSP_STATE0, numentries: *mut u32) -> super::super::super::Win32::Foundation::NTSTATUS
where
    P0: windows_core::Param<super::super::super::Win32::Foundation::HANDLE>,
    P1: windows_core::Param<super::super::super::Win32::Foundation::HANDLE>,
{
    windows_targets::link!("fwpuclnt.dll" "system" fn IPsecDospStateEnum0(enginehandle : super::super::super::Win32::Foundation:: HANDLE, enumhandle : super::super::super::Win32::Foundation:: HANDLE, numentriesrequested : u32, entries : *mut *mut *mut super::super::super::Win32::NetworkManagement::WindowsFilteringPlatform:: IPSEC_DOSP_STATE0, numentries : *mut u32) -> super::super::super::Win32::Foundation:: NTSTATUS);
    IPsecDospStateEnum0(enginehandle.param().abi(), enumhandle.param().abi(), numentriesrequested, entries, numentries)
}
#[cfg(feature = "Win32_NetworkManagement_WindowsFilteringPlatform")]
#[inline]
pub unsafe fn IPsecGetStatistics0<P0>(enginehandle: P0, ipsecstatistics: *mut super::super::super::Win32::NetworkManagement::WindowsFilteringPlatform::IPSEC_STATISTICS0) -> super::super::super::Win32::Foundation::NTSTATUS
where
    P0: windows_core::Param<super::super::super::Win32::Foundation::HANDLE>,
{
    windows_targets::link!("fwpuclnt.dll" "system" fn IPsecGetStatistics0(enginehandle : super::super::super::Win32::Foundation:: HANDLE, ipsecstatistics : *mut super::super::super::Win32::NetworkManagement::WindowsFilteringPlatform:: IPSEC_STATISTICS0) -> super::super::super::Win32::Foundation:: NTSTATUS);
    IPsecGetStatistics0(enginehandle.param().abi(), ipsecstatistics)
}
#[cfg(feature = "Win32_NetworkManagement_WindowsFilteringPlatform")]
#[inline]
pub unsafe fn IPsecGetStatistics1<P0>(enginehandle: P0, ipsecstatistics: *mut super::super::super::Win32::NetworkManagement::WindowsFilteringPlatform::IPSEC_STATISTICS1) -> super::super::super::Win32::Foundation::NTSTATUS
where
    P0: windows_core::Param<super::super::super::Win32::Foundation::HANDLE>,
{
    windows_targets::link!("fwpuclnt.dll" "system" fn IPsecGetStatistics1(enginehandle : super::super::super::Win32::Foundation:: HANDLE, ipsecstatistics : *mut super::super::super::Win32::NetworkManagement::WindowsFilteringPlatform:: IPSEC_STATISTICS1) -> super::super::super::Win32::Foundation:: NTSTATUS);
    IPsecGetStatistics1(enginehandle.param().abi(), ipsecstatistics)
}
#[cfg(feature = "Win32_NetworkManagement_WindowsFilteringPlatform")]
#[inline]
pub unsafe fn IPsecSaContextAddInbound0<P0>(enginehandle: P0, id: u64, inboundbundle: *const super::super::super::Win32::NetworkManagement::WindowsFilteringPlatform::IPSEC_SA_BUNDLE0) -> super::super::super::Win32::Foundation::NTSTATUS
where
    P0: windows_core::Param<super::super::super::Win32::Foundation::HANDLE>,
{
    windows_targets::link!("fwpuclnt.dll" "system" fn IPsecSaContextAddInbound0(enginehandle : super::super::super::Win32::Foundation:: HANDLE, id : u64, inboundbundle : *const super::super::super::Win32::NetworkManagement::WindowsFilteringPlatform:: IPSEC_SA_BUNDLE0) -> super::super::super::Win32::Foundation:: NTSTATUS);
    IPsecSaContextAddInbound0(enginehandle.param().abi(), id, inboundbundle)
}
#[cfg(feature = "Win32_NetworkManagement_WindowsFilteringPlatform")]
#[inline]
pub unsafe fn IPsecSaContextAddInbound1<P0>(enginehandle: P0, id: u64, inboundbundle: *const super::super::super::Win32::NetworkManagement::WindowsFilteringPlatform::IPSEC_SA_BUNDLE1) -> super::super::super::Win32::Foundation::NTSTATUS
where
    P0: windows_core::Param<super::super::super::Win32::Foundation::HANDLE>,
{
    windows_targets::link!("fwpuclnt.dll" "system" fn IPsecSaContextAddInbound1(enginehandle : super::super::super::Win32::Foundation:: HANDLE, id : u64, inboundbundle : *const super::super::super::Win32::NetworkManagement::WindowsFilteringPlatform:: IPSEC_SA_BUNDLE1) -> super::super::super::Win32::Foundation:: NTSTATUS);
    IPsecSaContextAddInbound1(enginehandle.param().abi(), id, inboundbundle)
}
#[cfg(feature = "Win32_NetworkManagement_WindowsFilteringPlatform")]
#[inline]
pub unsafe fn IPsecSaContextAddOutbound0<P0>(enginehandle: P0, id: u64, outboundbundle: *const super::super::super::Win32::NetworkManagement::WindowsFilteringPlatform::IPSEC_SA_BUNDLE0) -> super::super::super::Win32::Foundation::NTSTATUS
where
    P0: windows_core::Param<super::super::super::Win32::Foundation::HANDLE>,
{
    windows_targets::link!("fwpuclnt.dll" "system" fn IPsecSaContextAddOutbound0(enginehandle : super::super::super::Win32::Foundation:: HANDLE, id : u64, outboundbundle : *const super::super::super::Win32::NetworkManagement::WindowsFilteringPlatform:: IPSEC_SA_BUNDLE0) -> super::super::super::Win32::Foundation:: NTSTATUS);
    IPsecSaContextAddOutbound0(enginehandle.param().abi(), id, outboundbundle)
}
#[cfg(feature = "Win32_NetworkManagement_WindowsFilteringPlatform")]
#[inline]
pub unsafe fn IPsecSaContextAddOutbound1<P0>(enginehandle: P0, id: u64, outboundbundle: *const super::super::super::Win32::NetworkManagement::WindowsFilteringPlatform::IPSEC_SA_BUNDLE1) -> super::super::super::Win32::Foundation::NTSTATUS
where
    P0: windows_core::Param<super::super::super::Win32::Foundation::HANDLE>,
{
    windows_targets::link!("fwpuclnt.dll" "system" fn IPsecSaContextAddOutbound1(enginehandle : super::super::super::Win32::Foundation:: HANDLE, id : u64, outboundbundle : *const super::super::super::Win32::NetworkManagement::WindowsFilteringPlatform:: IPSEC_SA_BUNDLE1) -> super::super::super::Win32::Foundation:: NTSTATUS);
    IPsecSaContextAddOutbound1(enginehandle.param().abi(), id, outboundbundle)
}
#[cfg(feature = "Win32_NetworkManagement_WindowsFilteringPlatform")]
#[inline]
pub unsafe fn IPsecSaContextCreate0<P0>(enginehandle: P0, outboundtraffic: *const super::super::super::Win32::NetworkManagement::WindowsFilteringPlatform::IPSEC_TRAFFIC0, inboundfilterid: Option<*mut u64>, id: *mut u64) -> super::super::super::Win32::Foundation::NTSTATUS
where
    P0: windows_core::Param<super::super::super::Win32::Foundation::HANDLE>,
{
    windows_targets::link!("fwpuclnt.dll" "system" fn IPsecSaContextCreate0(enginehandle : super::super::super::Win32::Foundation:: HANDLE, outboundtraffic : *const super::super::super::Win32::NetworkManagement::WindowsFilteringPlatform:: IPSEC_TRAFFIC0, inboundfilterid : *mut u64, id : *mut u64) -> super::super::super::Win32::Foundation:: NTSTATUS);
    IPsecSaContextCreate0(enginehandle.param().abi(), outboundtraffic, core::mem::transmute(inboundfilterid.unwrap_or(std::ptr::null_mut())), id)
}
#[cfg(feature = "Win32_NetworkManagement_WindowsFilteringPlatform")]
#[inline]
pub unsafe fn IPsecSaContextCreate1<P0>(enginehandle: P0, outboundtraffic: *const super::super::super::Win32::NetworkManagement::WindowsFilteringPlatform::IPSEC_TRAFFIC1, virtualiftunnelinfo: Option<*const super::super::super::Win32::NetworkManagement::WindowsFilteringPlatform::IPSEC_VIRTUAL_IF_TUNNEL_INFO0>, inboundfilterid: Option<*mut u64>, id: *mut u64) -> super::super::super::Win32::Foundation::NTSTATUS
where
    P0: windows_core::Param<super::super::super::Win32::Foundation::HANDLE>,
{
    windows_targets::link!("fwpuclnt.dll" "system" fn IPsecSaContextCreate1(enginehandle : super::super::super::Win32::Foundation:: HANDLE, outboundtraffic : *const super::super::super::Win32::NetworkManagement::WindowsFilteringPlatform:: IPSEC_TRAFFIC1, virtualiftunnelinfo : *const super::super::super::Win32::NetworkManagement::WindowsFilteringPlatform:: IPSEC_VIRTUAL_IF_TUNNEL_INFO0, inboundfilterid : *mut u64, id : *mut u64) -> super::super::super::Win32::Foundation:: NTSTATUS);
    IPsecSaContextCreate1(enginehandle.param().abi(), outboundtraffic, core::mem::transmute(virtualiftunnelinfo.unwrap_or(std::ptr::null())), core::mem::transmute(inboundfilterid.unwrap_or(std::ptr::null_mut())), id)
}
#[cfg(all(feature = "Win32_NetworkManagement_WindowsFilteringPlatform", feature = "Win32_Security"))]
#[inline]
pub unsafe fn IPsecSaContextCreateEnumHandle0<P0>(enginehandle: P0, enumtemplate: Option<*const super::super::super::Win32::NetworkManagement::WindowsFilteringPlatform::IPSEC_SA_CONTEXT_ENUM_TEMPLATE0>, enumhandle: *mut super::super::super::Win32::Foundation::HANDLE) -> super::super::super::Win32::Foundation::NTSTATUS
where
    P0: windows_core::Param<super::super::super::Win32::Foundation::HANDLE>,
{
    windows_targets::link!("fwpuclnt.dll" "system" fn IPsecSaContextCreateEnumHandle0(enginehandle : super::super::super::Win32::Foundation:: HANDLE, enumtemplate : *const super::super::super::Win32::NetworkManagement::WindowsFilteringPlatform:: IPSEC_SA_CONTEXT_ENUM_TEMPLATE0, enumhandle : *mut super::super::super::Win32::Foundation:: HANDLE) -> super::super::super::Win32::Foundation:: NTSTATUS);
    IPsecSaContextCreateEnumHandle0(enginehandle.param().abi(), core::mem::transmute(enumtemplate.unwrap_or(std::ptr::null())), enumhandle)
}
#[inline]
pub unsafe fn IPsecSaContextDeleteById0<P0>(enginehandle: P0, id: u64) -> super::super::super::Win32::Foundation::NTSTATUS
where
    P0: windows_core::Param<super::super::super::Win32::Foundation::HANDLE>,
{
    windows_targets::link!("fwpuclnt.dll" "system" fn IPsecSaContextDeleteById0(enginehandle : super::super::super::Win32::Foundation:: HANDLE, id : u64) -> super::super::super::Win32::Foundation:: NTSTATUS);
    IPsecSaContextDeleteById0(enginehandle.param().abi(), id)
}
#[inline]
pub unsafe fn IPsecSaContextDestroyEnumHandle0<P0>(enginehandle: P0, enumhandle: super::super::super::Win32::Foundation::HANDLE) -> super::super::super::Win32::Foundation::NTSTATUS
where
    P0: windows_core::Param<super::super::super::Win32::Foundation::HANDLE>,
{
    windows_targets::link!("fwpuclnt.dll" "system" fn IPsecSaContextDestroyEnumHandle0(enginehandle : super::super::super::Win32::Foundation:: HANDLE, enumhandle : super::super::super::Win32::Foundation:: HANDLE) -> super::super::super::Win32::Foundation:: NTSTATUS);
    IPsecSaContextDestroyEnumHandle0(enginehandle.param().abi(), enumhandle)
}
#[cfg(all(feature = "Win32_NetworkManagement_WindowsFilteringPlatform", feature = "Win32_Security"))]
#[inline]
pub unsafe fn IPsecSaContextEnum0<P0, P1>(enginehandle: P0, enumhandle: P1, numentriesrequested: u32, entries: *mut *mut *mut super::super::super::Win32::NetworkManagement::WindowsFilteringPlatform::IPSEC_SA_CONTEXT0, numentriesreturned: *mut u32) -> super::super::super::Win32::Foundation::NTSTATUS
where
    P0: windows_core::Param<super::super::super::Win32::Foundation::HANDLE>,
    P1: windows_core::Param<super::super::super::Win32::Foundation::HANDLE>,
{
    windows_targets::link!("fwpuclnt.dll" "system" fn IPsecSaContextEnum0(enginehandle : super::super::super::Win32::Foundation:: HANDLE, enumhandle : super::super::super::Win32::Foundation:: HANDLE, numentriesrequested : u32, entries : *mut *mut *mut super::super::super::Win32::NetworkManagement::WindowsFilteringPlatform:: IPSEC_SA_CONTEXT0, numentriesreturned : *mut u32) -> super::super::super::Win32::Foundation:: NTSTATUS);
    IPsecSaContextEnum0(enginehandle.param().abi(), enumhandle.param().abi(), numentriesrequested, entries, numentriesreturned)
}
#[cfg(all(feature = "Win32_NetworkManagement_WindowsFilteringPlatform", feature = "Win32_Security"))]
#[inline]
pub unsafe fn IPsecSaContextEnum1<P0, P1>(enginehandle: P0, enumhandle: P1, numentriesrequested: u32, entries: *mut *mut *mut super::super::super::Win32::NetworkManagement::WindowsFilteringPlatform::IPSEC_SA_CONTEXT1, numentriesreturned: *mut u32) -> super::super::super::Win32::Foundation::NTSTATUS
where
    P0: windows_core::Param<super::super::super::Win32::Foundation::HANDLE>,
    P1: windows_core::Param<super::super::super::Win32::Foundation::HANDLE>,
{
    windows_targets::link!("fwpuclnt.dll" "system" fn IPsecSaContextEnum1(enginehandle : super::super::super::Win32::Foundation:: HANDLE, enumhandle : super::super::super::Win32::Foundation:: HANDLE, numentriesrequested : u32, entries : *mut *mut *mut super::super::super::Win32::NetworkManagement::WindowsFilteringPlatform:: IPSEC_SA_CONTEXT1, numentriesreturned : *mut u32) -> super::super::super::Win32::Foundation:: NTSTATUS);
    IPsecSaContextEnum1(enginehandle.param().abi(), enumhandle.param().abi(), numentriesrequested, entries, numentriesreturned)
}
#[inline]
pub unsafe fn IPsecSaContextExpire0<P0>(enginehandle: P0, id: u64) -> super::super::super::Win32::Foundation::NTSTATUS
where
    P0: windows_core::Param<super::super::super::Win32::Foundation::HANDLE>,
{
    windows_targets::link!("fwpuclnt.dll" "system" fn IPsecSaContextExpire0(enginehandle : super::super::super::Win32::Foundation:: HANDLE, id : u64) -> super::super::super::Win32::Foundation:: NTSTATUS);
    IPsecSaContextExpire0(enginehandle.param().abi(), id)
}
#[cfg(all(feature = "Win32_NetworkManagement_WindowsFilteringPlatform", feature = "Win32_Security"))]
#[inline]
pub unsafe fn IPsecSaContextGetById0<P0>(enginehandle: P0, id: u64, sacontext: *mut *mut super::super::super::Win32::NetworkManagement::WindowsFilteringPlatform::IPSEC_SA_CONTEXT0) -> super::super::super::Win32::Foundation::NTSTATUS
where
    P0: windows_core::Param<super::super::super::Win32::Foundation::HANDLE>,
{
    windows_targets::link!("fwpuclnt.dll" "system" fn IPsecSaContextGetById0(enginehandle : super::super::super::Win32::Foundation:: HANDLE, id : u64, sacontext : *mut *mut super::super::super::Win32::NetworkManagement::WindowsFilteringPlatform:: IPSEC_SA_CONTEXT0) -> super::super::super::Win32::Foundation:: NTSTATUS);
    IPsecSaContextGetById0(enginehandle.param().abi(), id, sacontext)
}
#[cfg(all(feature = "Win32_NetworkManagement_WindowsFilteringPlatform", feature = "Win32_Security"))]
#[inline]
pub unsafe fn IPsecSaContextGetById1<P0>(enginehandle: P0, id: u64, sacontext: *mut *mut super::super::super::Win32::NetworkManagement::WindowsFilteringPlatform::IPSEC_SA_CONTEXT1) -> super::super::super::Win32::Foundation::NTSTATUS
where
    P0: windows_core::Param<super::super::super::Win32::Foundation::HANDLE>,
{
    windows_targets::link!("fwpuclnt.dll" "system" fn IPsecSaContextGetById1(enginehandle : super::super::super::Win32::Foundation:: HANDLE, id : u64, sacontext : *mut *mut super::super::super::Win32::NetworkManagement::WindowsFilteringPlatform:: IPSEC_SA_CONTEXT1) -> super::super::super::Win32::Foundation:: NTSTATUS);
    IPsecSaContextGetById1(enginehandle.param().abi(), id, sacontext)
}
#[cfg(feature = "Win32_NetworkManagement_WindowsFilteringPlatform")]
#[inline]
pub unsafe fn IPsecSaContextGetSpi0<P0>(enginehandle: P0, id: u64, getspi: *const super::super::super::Win32::NetworkManagement::WindowsFilteringPlatform::IPSEC_GETSPI0, inboundspi: *mut u32) -> super::super::super::Win32::Foundation::NTSTATUS
where
    P0: windows_core::Param<super::super::super::Win32::Foundation::HANDLE>,
{
    windows_targets::link!("fwpuclnt.dll" "system" fn IPsecSaContextGetSpi0(enginehandle : super::super::super::Win32::Foundation:: HANDLE, id : u64, getspi : *const super::super::super::Win32::NetworkManagement::WindowsFilteringPlatform:: IPSEC_GETSPI0, inboundspi : *mut u32) -> super::super::super::Win32::Foundation:: NTSTATUS);
    IPsecSaContextGetSpi0(enginehandle.param().abi(), id, getspi, inboundspi)
}
#[cfg(feature = "Win32_NetworkManagement_WindowsFilteringPlatform")]
#[inline]
pub unsafe fn IPsecSaContextGetSpi1<P0>(enginehandle: P0, id: u64, getspi: *const super::super::super::Win32::NetworkManagement::WindowsFilteringPlatform::IPSEC_GETSPI1, inboundspi: *mut u32) -> super::super::super::Win32::Foundation::NTSTATUS
where
    P0: windows_core::Param<super::super::super::Win32::Foundation::HANDLE>,
{
    windows_targets::link!("fwpuclnt.dll" "system" fn IPsecSaContextGetSpi1(enginehandle : super::super::super::Win32::Foundation:: HANDLE, id : u64, getspi : *const super::super::super::Win32::NetworkManagement::WindowsFilteringPlatform:: IPSEC_GETSPI1, inboundspi : *mut u32) -> super::super::super::Win32::Foundation:: NTSTATUS);
    IPsecSaContextGetSpi1(enginehandle.param().abi(), id, getspi, inboundspi)
}
#[cfg(feature = "Win32_NetworkManagement_WindowsFilteringPlatform")]
#[inline]
pub unsafe fn IPsecSaContextSetSpi0<P0>(enginehandle: P0, id: u64, getspi: *const super::super::super::Win32::NetworkManagement::WindowsFilteringPlatform::IPSEC_GETSPI1, inboundspi: u32) -> super::super::super::Win32::Foundation::NTSTATUS
where
    P0: windows_core::Param<super::super::super::Win32::Foundation::HANDLE>,
{
    windows_targets::link!("fwpuclnt.dll" "system" fn IPsecSaContextSetSpi0(enginehandle : super::super::super::Win32::Foundation:: HANDLE, id : u64, getspi : *const super::super::super::Win32::NetworkManagement::WindowsFilteringPlatform:: IPSEC_GETSPI1, inboundspi : u32) -> super::super::super::Win32::Foundation:: NTSTATUS);
    IPsecSaContextSetSpi0(enginehandle.param().abi(), id, getspi, inboundspi)
}
#[cfg(all(feature = "Win32_NetworkManagement_WindowsFilteringPlatform", feature = "Win32_Security"))]
#[inline]
pub unsafe fn IPsecSaContextUpdate0<P0>(enginehandle: P0, flags: u64, newvalues: *const super::super::super::Win32::NetworkManagement::WindowsFilteringPlatform::IPSEC_SA_CONTEXT1) -> super::super::super::Win32::Foundation::NTSTATUS
where
    P0: windows_core::Param<super::super::super::Win32::Foundation::HANDLE>,
{
    windows_targets::link!("fwpuclnt.dll" "system" fn IPsecSaContextUpdate0(enginehandle : super::super::super::Win32::Foundation:: HANDLE, flags : u64, newvalues : *const super::super::super::Win32::NetworkManagement::WindowsFilteringPlatform:: IPSEC_SA_CONTEXT1) -> super::super::super::Win32::Foundation:: NTSTATUS);
    IPsecSaContextUpdate0(enginehandle.param().abi(), flags, newvalues)
}
#[cfg(feature = "Win32_NetworkManagement_WindowsFilteringPlatform")]
#[inline]
pub unsafe fn IPsecSaCreateEnumHandle0<P0>(enginehandle: P0, enumtemplate: Option<*const super::super::super::Win32::NetworkManagement::WindowsFilteringPlatform::IPSEC_SA_ENUM_TEMPLATE0>, enumhandle: *mut super::super::super::Win32::Foundation::HANDLE) -> super::super::super::Win32::Foundation::NTSTATUS
where
    P0: windows_core::Param<super::super::super::Win32::Foundation::HANDLE>,
{
    windows_targets::link!("fwpuclnt.dll" "system" fn IPsecSaCreateEnumHandle0(enginehandle : super::super::super::Win32::Foundation:: HANDLE, enumtemplate : *const super::super::super::Win32::NetworkManagement::WindowsFilteringPlatform:: IPSEC_SA_ENUM_TEMPLATE0, enumhandle : *mut super::super::super::Win32::Foundation:: HANDLE) -> super::super::super::Win32::Foundation:: NTSTATUS);
    IPsecSaCreateEnumHandle0(enginehandle.param().abi(), core::mem::transmute(enumtemplate.unwrap_or(std::ptr::null())), enumhandle)
}
#[cfg(feature = "Win32_Security")]
#[inline]
pub unsafe fn IPsecSaDbGetSecurityInfo0<P0>(enginehandle: P0, securityinfo: u32, sidowner: *mut super::super::super::Win32::Security::PSID, sidgroup: *mut super::super::super::Win32::Security::PSID, dacl: *mut *mut super::super::super::Win32::Security::ACL, sacl: *mut *mut super::super::super::Win32::Security::ACL, securitydescriptor: *mut super::super::super::Win32::Security::PSECURITY_DESCRIPTOR) -> super::super::super::Win32::Foundation::NTSTATUS
where
    P0: windows_core::Param<super::super::super::Win32::Foundation::HANDLE>,
{
    windows_targets::link!("fwpuclnt.dll" "system" fn IPsecSaDbGetSecurityInfo0(enginehandle : super::super::super::Win32::Foundation:: HANDLE, securityinfo : u32, sidowner : *mut super::super::super::Win32::Security:: PSID, sidgroup : *mut super::super::super::Win32::Security:: PSID, dacl : *mut *mut super::super::super::Win32::Security:: ACL, sacl : *mut *mut super::super::super::Win32::Security:: ACL, securitydescriptor : *mut super::super::super::Win32::Security:: PSECURITY_DESCRIPTOR) -> super::super::super::Win32::Foundation:: NTSTATUS);
    IPsecSaDbGetSecurityInfo0(enginehandle.param().abi(), securityinfo, sidowner, sidgroup, dacl, sacl, securitydescriptor)
}
#[cfg(feature = "Win32_Security")]
#[inline]
pub unsafe fn IPsecSaDbSetSecurityInfo0<P0>(enginehandle: P0, securityinfo: u32, sidowner: Option<*const super::super::super::Win32::Security::SID>, sidgroup: Option<*const super::super::super::Win32::Security::SID>, dacl: Option<*const super::super::super::Win32::Security::ACL>, sacl: Option<*const super::super::super::Win32::Security::ACL>) -> super::super::super::Win32::Foundation::NTSTATUS
where
    P0: windows_core::Param<super::super::super::Win32::Foundation::HANDLE>,
{
    windows_targets::link!("fwpuclnt.dll" "system" fn IPsecSaDbSetSecurityInfo0(enginehandle : super::super::super::Win32::Foundation:: HANDLE, securityinfo : u32, sidowner : *const super::super::super::Win32::Security:: SID, sidgroup : *const super::super::super::Win32::Security:: SID, dacl : *const super::super::super::Win32::Security:: ACL, sacl : *const super::super::super::Win32::Security:: ACL) -> super::super::super::Win32::Foundation:: NTSTATUS);
    IPsecSaDbSetSecurityInfo0(enginehandle.param().abi(), securityinfo, core::mem::transmute(sidowner.unwrap_or(std::ptr::null())), core::mem::transmute(sidgroup.unwrap_or(std::ptr::null())), core::mem::transmute(dacl.unwrap_or(std::ptr::null())), core::mem::transmute(sacl.unwrap_or(std::ptr::null())))
}
#[inline]
pub unsafe fn IPsecSaDestroyEnumHandle0<P0>(enginehandle: P0, enumhandle: super::super::super::Win32::Foundation::HANDLE) -> super::super::super::Win32::Foundation::NTSTATUS
where
    P0: windows_core::Param<super::super::super::Win32::Foundation::HANDLE>,
{
    windows_targets::link!("fwpuclnt.dll" "system" fn IPsecSaDestroyEnumHandle0(enginehandle : super::super::super::Win32::Foundation:: HANDLE, enumhandle : super::super::super::Win32::Foundation:: HANDLE) -> super::super::super::Win32::Foundation:: NTSTATUS);
    IPsecSaDestroyEnumHandle0(enginehandle.param().abi(), enumhandle)
}
#[cfg(all(feature = "Win32_NetworkManagement_WindowsFilteringPlatform", feature = "Win32_Security"))]
#[inline]
pub unsafe fn IPsecSaEnum0<P0, P1>(enginehandle: P0, enumhandle: P1, numentriesrequested: u32, entries: *mut *mut *mut super::super::super::Win32::NetworkManagement::WindowsFilteringPlatform::IPSEC_SA_DETAILS0, numentriesreturned: *mut u32) -> super::super::super::Win32::Foundation::NTSTATUS
where
    P0: windows_core::Param<super::super::super::Win32::Foundation::HANDLE>,
    P1: windows_core::Param<super::super::super::Win32::Foundation::HANDLE>,
{
    windows_targets::link!("fwpuclnt.dll" "system" fn IPsecSaEnum0(enginehandle : super::super::super::Win32::Foundation:: HANDLE, enumhandle : super::super::super::Win32::Foundation:: HANDLE, numentriesrequested : u32, entries : *mut *mut *mut super::super::super::Win32::NetworkManagement::WindowsFilteringPlatform:: IPSEC_SA_DETAILS0, numentriesreturned : *mut u32) -> super::super::super::Win32::Foundation:: NTSTATUS);
    IPsecSaEnum0(enginehandle.param().abi(), enumhandle.param().abi(), numentriesrequested, entries, numentriesreturned)
}
#[cfg(all(feature = "Win32_NetworkManagement_WindowsFilteringPlatform", feature = "Win32_Security"))]
#[inline]
pub unsafe fn IPsecSaEnum1<P0, P1>(enginehandle: P0, enumhandle: P1, numentriesrequested: u32, entries: *mut *mut *mut super::super::super::Win32::NetworkManagement::WindowsFilteringPlatform::IPSEC_SA_DETAILS1, numentriesreturned: *mut u32) -> super::super::super::Win32::Foundation::NTSTATUS
where
    P0: windows_core::Param<super::super::super::Win32::Foundation::HANDLE>,
    P1: windows_core::Param<super::super::super::Win32::Foundation::HANDLE>,
{
    windows_targets::link!("fwpuclnt.dll" "system" fn IPsecSaEnum1(enginehandle : super::super::super::Win32::Foundation:: HANDLE, enumhandle : super::super::super::Win32::Foundation:: HANDLE, numentriesrequested : u32, entries : *mut *mut *mut super::super::super::Win32::NetworkManagement::WindowsFilteringPlatform:: IPSEC_SA_DETAILS1, numentriesreturned : *mut u32) -> super::super::super::Win32::Foundation:: NTSTATUS);
    IPsecSaEnum1(enginehandle.param().abi(), enumhandle.param().abi(), numentriesrequested, entries, numentriesreturned)
}
#[cfg(feature = "Win32_NetworkManagement_WindowsFilteringPlatform")]
#[inline]
pub unsafe fn IkeextGetStatistics0<P0>(enginehandle: P0, ikeextstatistics: *mut super::super::super::Win32::NetworkManagement::WindowsFilteringPlatform::IKEEXT_STATISTICS0) -> super::super::super::Win32::Foundation::NTSTATUS
where
    P0: windows_core::Param<super::super::super::Win32::Foundation::HANDLE>,
{
    windows_targets::link!("fwpuclnt.dll" "system" fn IkeextGetStatistics0(enginehandle : super::super::super::Win32::Foundation:: HANDLE, ikeextstatistics : *mut super::super::super::Win32::NetworkManagement::WindowsFilteringPlatform:: IKEEXT_STATISTICS0) -> super::super::super::Win32::Foundation:: NTSTATUS);
    IkeextGetStatistics0(enginehandle.param().abi(), ikeextstatistics)
}
#[cfg(feature = "Win32_NetworkManagement_WindowsFilteringPlatform")]
#[inline]
pub unsafe fn IkeextGetStatistics1<P0>(enginehandle: P0, ikeextstatistics: *mut super::super::super::Win32::NetworkManagement::WindowsFilteringPlatform::IKEEXT_STATISTICS1) -> super::super::super::Win32::Foundation::NTSTATUS
where
    P0: windows_core::Param<super::super::super::Win32::Foundation::HANDLE>,
{
    windows_targets::link!("fwpuclnt.dll" "system" fn IkeextGetStatistics1(enginehandle : super::super::super::Win32::Foundation:: HANDLE, ikeextstatistics : *mut super::super::super::Win32::NetworkManagement::WindowsFilteringPlatform:: IKEEXT_STATISTICS1) -> super::super::super::Win32::Foundation:: NTSTATUS);
    IkeextGetStatistics1(enginehandle.param().abi(), ikeextstatistics)
}
#[cfg(all(feature = "Win32_NetworkManagement_WindowsFilteringPlatform", feature = "Win32_Security"))]
#[inline]
pub unsafe fn IkeextSaCreateEnumHandle0<P0>(enginehandle: P0, enumtemplate: Option<*const super::super::super::Win32::NetworkManagement::WindowsFilteringPlatform::IKEEXT_SA_ENUM_TEMPLATE0>, enumhandle: *mut super::super::super::Win32::Foundation::HANDLE) -> super::super::super::Win32::Foundation::NTSTATUS
where
    P0: windows_core::Param<super::super::super::Win32::Foundation::HANDLE>,
{
    windows_targets::link!("fwpuclnt.dll" "system" fn IkeextSaCreateEnumHandle0(enginehandle : super::super::super::Win32::Foundation:: HANDLE, enumtemplate : *const super::super::super::Win32::NetworkManagement::WindowsFilteringPlatform:: IKEEXT_SA_ENUM_TEMPLATE0, enumhandle : *mut super::super::super::Win32::Foundation:: HANDLE) -> super::super::super::Win32::Foundation:: NTSTATUS);
    IkeextSaCreateEnumHandle0(enginehandle.param().abi(), core::mem::transmute(enumtemplate.unwrap_or(std::ptr::null())), enumhandle)
}
#[cfg(feature = "Win32_Security")]
#[inline]
pub unsafe fn IkeextSaDbGetSecurityInfo0<P0>(enginehandle: P0, securityinfo: u32, sidowner: *mut super::super::super::Win32::Security::PSID, sidgroup: *mut super::super::super::Win32::Security::PSID, dacl: *mut *mut super::super::super::Win32::Security::ACL, sacl: *mut *mut super::super::super::Win32::Security::ACL, securitydescriptor: *mut super::super::super::Win32::Security::PSECURITY_DESCRIPTOR) -> super::super::super::Win32::Foundation::NTSTATUS
where
    P0: windows_core::Param<super::super::super::Win32::Foundation::HANDLE>,
{
    windows_targets::link!("fwpuclnt.dll" "system" fn IkeextSaDbGetSecurityInfo0(enginehandle : super::super::super::Win32::Foundation:: HANDLE, securityinfo : u32, sidowner : *mut super::super::super::Win32::Security:: PSID, sidgroup : *mut super::super::super::Win32::Security:: PSID, dacl : *mut *mut super::super::super::Win32::Security:: ACL, sacl : *mut *mut super::super::super::Win32::Security:: ACL, securitydescriptor : *mut super::super::super::Win32::Security:: PSECURITY_DESCRIPTOR) -> super::super::super::Win32::Foundation:: NTSTATUS);
    IkeextSaDbGetSecurityInfo0(enginehandle.param().abi(), securityinfo, sidowner, sidgroup, dacl, sacl, securitydescriptor)
}
#[cfg(feature = "Win32_Security")]
#[inline]
pub unsafe fn IkeextSaDbSetSecurityInfo0<P0>(enginehandle: P0, securityinfo: u32, sidowner: Option<*const super::super::super::Win32::Security::SID>, sidgroup: Option<*const super::super::super::Win32::Security::SID>, dacl: Option<*const super::super::super::Win32::Security::ACL>, sacl: Option<*const super::super::super::Win32::Security::ACL>) -> super::super::super::Win32::Foundation::NTSTATUS
where
    P0: windows_core::Param<super::super::super::Win32::Foundation::HANDLE>,
{
    windows_targets::link!("fwpuclnt.dll" "system" fn IkeextSaDbSetSecurityInfo0(enginehandle : super::super::super::Win32::Foundation:: HANDLE, securityinfo : u32, sidowner : *const super::super::super::Win32::Security:: SID, sidgroup : *const super::super::super::Win32::Security:: SID, dacl : *const super::super::super::Win32::Security:: ACL, sacl : *const super::super::super::Win32::Security:: ACL) -> super::super::super::Win32::Foundation:: NTSTATUS);
    IkeextSaDbSetSecurityInfo0(enginehandle.param().abi(), securityinfo, core::mem::transmute(sidowner.unwrap_or(std::ptr::null())), core::mem::transmute(sidgroup.unwrap_or(std::ptr::null())), core::mem::transmute(dacl.unwrap_or(std::ptr::null())), core::mem::transmute(sacl.unwrap_or(std::ptr::null())))
}
#[inline]
pub unsafe fn IkeextSaDeleteById0<P0>(enginehandle: P0, id: u64) -> super::super::super::Win32::Foundation::NTSTATUS
where
    P0: windows_core::Param<super::super::super::Win32::Foundation::HANDLE>,
{
    windows_targets::link!("fwpuclnt.dll" "system" fn IkeextSaDeleteById0(enginehandle : super::super::super::Win32::Foundation:: HANDLE, id : u64) -> super::super::super::Win32::Foundation:: NTSTATUS);
    IkeextSaDeleteById0(enginehandle.param().abi(), id)
}
#[inline]
pub unsafe fn IkeextSaDestroyEnumHandle0<P0>(enginehandle: P0, enumhandle: super::super::super::Win32::Foundation::HANDLE) -> super::super::super::Win32::Foundation::NTSTATUS
where
    P0: windows_core::Param<super::super::super::Win32::Foundation::HANDLE>,
{
    windows_targets::link!("fwpuclnt.dll" "system" fn IkeextSaDestroyEnumHandle0(enginehandle : super::super::super::Win32::Foundation:: HANDLE, enumhandle : super::super::super::Win32::Foundation:: HANDLE) -> super::super::super::Win32::Foundation:: NTSTATUS);
    IkeextSaDestroyEnumHandle0(enginehandle.param().abi(), enumhandle)
}
#[cfg(feature = "Win32_NetworkManagement_WindowsFilteringPlatform")]
#[inline]
pub unsafe fn IkeextSaEnum0<P0, P1>(enginehandle: P0, enumhandle: P1, numentriesrequested: u32, entries: *mut *mut *mut super::super::super::Win32::NetworkManagement::WindowsFilteringPlatform::IKEEXT_SA_DETAILS0, numentriesreturned: *mut u32) -> super::super::super::Win32::Foundation::NTSTATUS
where
    P0: windows_core::Param<super::super::super::Win32::Foundation::HANDLE>,
    P1: windows_core::Param<super::super::super::Win32::Foundation::HANDLE>,
{
    windows_targets::link!("fwpuclnt.dll" "system" fn IkeextSaEnum0(enginehandle : super::super::super::Win32::Foundation:: HANDLE, enumhandle : super::super::super::Win32::Foundation:: HANDLE, numentriesrequested : u32, entries : *mut *mut *mut super::super::super::Win32::NetworkManagement::WindowsFilteringPlatform:: IKEEXT_SA_DETAILS0, numentriesreturned : *mut u32) -> super::super::super::Win32::Foundation:: NTSTATUS);
    IkeextSaEnum0(enginehandle.param().abi(), enumhandle.param().abi(), numentriesrequested, entries, numentriesreturned)
}
#[cfg(feature = "Win32_NetworkManagement_WindowsFilteringPlatform")]
#[inline]
pub unsafe fn IkeextSaEnum1<P0, P1>(enginehandle: P0, enumhandle: P1, numentriesrequested: u32, entries: *mut *mut *mut super::super::super::Win32::NetworkManagement::WindowsFilteringPlatform::IKEEXT_SA_DETAILS1, numentriesreturned: *mut u32) -> super::super::super::Win32::Foundation::NTSTATUS
where
    P0: windows_core::Param<super::super::super::Win32::Foundation::HANDLE>,
    P1: windows_core::Param<super::super::super::Win32::Foundation::HANDLE>,
{
    windows_targets::link!("fwpuclnt.dll" "system" fn IkeextSaEnum1(enginehandle : super::super::super::Win32::Foundation:: HANDLE, enumhandle : super::super::super::Win32::Foundation:: HANDLE, numentriesrequested : u32, entries : *mut *mut *mut super::super::super::Win32::NetworkManagement::WindowsFilteringPlatform:: IKEEXT_SA_DETAILS1, numentriesreturned : *mut u32) -> super::super::super::Win32::Foundation:: NTSTATUS);
    IkeextSaEnum1(enginehandle.param().abi(), enumhandle.param().abi(), numentriesrequested, entries, numentriesreturned)
}
#[cfg(feature = "Win32_NetworkManagement_WindowsFilteringPlatform")]
#[inline]
pub unsafe fn IkeextSaEnum2<P0, P1>(enginehandle: P0, enumhandle: P1, numentriesrequested: u32, entries: *mut *mut *mut super::super::super::Win32::NetworkManagement::WindowsFilteringPlatform::IKEEXT_SA_DETAILS2, numentriesreturned: *mut u32) -> super::super::super::Win32::Foundation::NTSTATUS
where
    P0: windows_core::Param<super::super::super::Win32::Foundation::HANDLE>,
    P1: windows_core::Param<super::super::super::Win32::Foundation::HANDLE>,
{
    windows_targets::link!("fwpuclnt.dll" "system" fn IkeextSaEnum2(enginehandle : super::super::super::Win32::Foundation:: HANDLE, enumhandle : super::super::super::Win32::Foundation:: HANDLE, numentriesrequested : u32, entries : *mut *mut *mut super::super::super::Win32::NetworkManagement::WindowsFilteringPlatform:: IKEEXT_SA_DETAILS2, numentriesreturned : *mut u32) -> super::super::super::Win32::Foundation:: NTSTATUS);
    IkeextSaEnum2(enginehandle.param().abi(), enumhandle.param().abi(), numentriesrequested, entries, numentriesreturned)
}
#[cfg(feature = "Win32_NetworkManagement_WindowsFilteringPlatform")]
#[inline]
pub unsafe fn IkeextSaGetById0<P0>(enginehandle: P0, id: u64, sa: *mut *mut super::super::super::Win32::NetworkManagement::WindowsFilteringPlatform::IKEEXT_SA_DETAILS0) -> super::super::super::Win32::Foundation::NTSTATUS
where
    P0: windows_core::Param<super::super::super::Win32::Foundation::HANDLE>,
{
    windows_targets::link!("fwpuclnt.dll" "system" fn IkeextSaGetById0(enginehandle : super::super::super::Win32::Foundation:: HANDLE, id : u64, sa : *mut *mut super::super::super::Win32::NetworkManagement::WindowsFilteringPlatform:: IKEEXT_SA_DETAILS0) -> super::super::super::Win32::Foundation:: NTSTATUS);
    IkeextSaGetById0(enginehandle.param().abi(), id, sa)
}
#[cfg(feature = "Win32_NetworkManagement_WindowsFilteringPlatform")]
#[inline]
pub unsafe fn IkeextSaGetById1<P0>(enginehandle: P0, id: u64, salookupcontext: Option<*const windows_core::GUID>, sa: *mut *mut super::super::super::Win32::NetworkManagement::WindowsFilteringPlatform::IKEEXT_SA_DETAILS1) -> super::super::super::Win32::Foundation::NTSTATUS
where
    P0: windows_core::Param<super::super::super::Win32::Foundation::HANDLE>,
{
    windows_targets::link!("fwpuclnt.dll" "system" fn IkeextSaGetById1(enginehandle : super::super::super::Win32::Foundation:: HANDLE, id : u64, salookupcontext : *const windows_core::GUID, sa : *mut *mut super::super::super::Win32::NetworkManagement::WindowsFilteringPlatform:: IKEEXT_SA_DETAILS1) -> super::super::super::Win32::Foundation:: NTSTATUS);
    IkeextSaGetById1(enginehandle.param().abi(), id, core::mem::transmute(salookupcontext.unwrap_or(std::ptr::null())), sa)
}
#[cfg(feature = "Win32_NetworkManagement_WindowsFilteringPlatform")]
#[inline]
pub unsafe fn IkeextSaGetById2<P0>(enginehandle: P0, id: u64, salookupcontext: Option<*const windows_core::GUID>, sa: *mut *mut super::super::super::Win32::NetworkManagement::WindowsFilteringPlatform::IKEEXT_SA_DETAILS2) -> super::super::super::Win32::Foundation::NTSTATUS
where
    P0: windows_core::Param<super::super::super::Win32::Foundation::HANDLE>,
{
    windows_targets::link!("fwpuclnt.dll" "system" fn IkeextSaGetById2(enginehandle : super::super::super::Win32::Foundation:: HANDLE, id : u64, salookupcontext : *const windows_core::GUID, sa : *mut *mut super::super::super::Win32::NetworkManagement::WindowsFilteringPlatform:: IKEEXT_SA_DETAILS2) -> super::super::super::Win32::Foundation:: NTSTATUS);
    IkeextSaGetById2(enginehandle.param().abi(), id, core::mem::transmute(salookupcontext.unwrap_or(std::ptr::null())), sa)
}
#[cfg(feature = "Win32_NetworkManagement_WindowsFilteringPlatform")]
pub type FWPM_SERVICE_STATE_CHANGE_CALLBACK0 = Option<unsafe extern "system" fn(context: *mut core::ffi::c_void, newstate: super::super::super::Win32::NetworkManagement::WindowsFilteringPlatform::FWPM_SERVICE_STATE)>;
