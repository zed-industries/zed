#[cfg(feature = "Win32_NetworkManagement_WindowsFilteringPlatform")]
#[inline]
pub unsafe fn FwpmBfeStateGet0() -> super::super::super::Win32::NetworkManagement::WindowsFilteringPlatform::FWPM_SERVICE_STATE {
    windows_core::link!("fwpkclnt.sys" "system" fn FwpmBfeStateGet0() -> super::super::super::Win32::NetworkManagement::WindowsFilteringPlatform:: FWPM_SERVICE_STATE);
    unsafe { FwpmBfeStateGet0() }
}
#[cfg(feature = "Win32_NetworkManagement_WindowsFilteringPlatform")]
#[inline]
pub unsafe fn FwpmBfeStateSubscribeChanges0(deviceobject: *mut core::ffi::c_void, callback: FWPM_SERVICE_STATE_CHANGE_CALLBACK0, context: Option<*const core::ffi::c_void>, changehandle: *mut super::super::super::Win32::Foundation::HANDLE) -> super::super::super::Win32::Foundation::NTSTATUS {
    windows_core::link!("fwpkclnt.sys" "system" fn FwpmBfeStateSubscribeChanges0(deviceobject : *mut core::ffi::c_void, callback : FWPM_SERVICE_STATE_CHANGE_CALLBACK0, context : *const core::ffi::c_void, changehandle : *mut super::super::super::Win32::Foundation:: HANDLE) -> super::super::super::Win32::Foundation:: NTSTATUS);
    unsafe { FwpmBfeStateSubscribeChanges0(deviceobject as _, callback, context.unwrap_or(core::mem::zeroed()) as _, changehandle as _) }
}
#[inline]
pub unsafe fn FwpmBfeStateUnsubscribeChanges0(changehandle: super::super::super::Win32::Foundation::HANDLE) -> super::super::super::Win32::Foundation::NTSTATUS {
    windows_core::link!("fwpkclnt.sys" "system" fn FwpmBfeStateUnsubscribeChanges0(changehandle : super::super::super::Win32::Foundation:: HANDLE) -> super::super::super::Win32::Foundation:: NTSTATUS);
    unsafe { FwpmBfeStateUnsubscribeChanges0(changehandle as _) }
}
#[cfg(all(feature = "Win32_NetworkManagement_WindowsFilteringPlatform", feature = "Win32_Security"))]
#[inline]
pub unsafe fn FwpmCalloutAdd0(enginehandle: super::super::super::Win32::Foundation::HANDLE, callout: *const super::super::super::Win32::NetworkManagement::WindowsFilteringPlatform::FWPM_CALLOUT0, sd: Option<super::super::super::Win32::Security::PSECURITY_DESCRIPTOR>, id: Option<*mut u32>) -> super::super::super::Win32::Foundation::NTSTATUS {
    windows_core::link!("fwpuclnt.dll" "system" fn FwpmCalloutAdd0(enginehandle : super::super::super::Win32::Foundation:: HANDLE, callout : *const super::super::super::Win32::NetworkManagement::WindowsFilteringPlatform:: FWPM_CALLOUT0, sd : super::super::super::Win32::Security:: PSECURITY_DESCRIPTOR, id : *mut u32) -> super::super::super::Win32::Foundation:: NTSTATUS);
    unsafe { FwpmCalloutAdd0(enginehandle, callout, sd.unwrap_or(core::mem::zeroed()) as _, id.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "Win32_NetworkManagement_WindowsFilteringPlatform")]
#[inline]
pub unsafe fn FwpmCalloutCreateEnumHandle0(enginehandle: super::super::super::Win32::Foundation::HANDLE, enumtemplate: Option<*const super::super::super::Win32::NetworkManagement::WindowsFilteringPlatform::FWPM_CALLOUT_ENUM_TEMPLATE0>, enumhandle: *mut super::super::super::Win32::Foundation::HANDLE) -> super::super::super::Win32::Foundation::NTSTATUS {
    windows_core::link!("fwpuclnt.dll" "system" fn FwpmCalloutCreateEnumHandle0(enginehandle : super::super::super::Win32::Foundation:: HANDLE, enumtemplate : *const super::super::super::Win32::NetworkManagement::WindowsFilteringPlatform:: FWPM_CALLOUT_ENUM_TEMPLATE0, enumhandle : *mut super::super::super::Win32::Foundation:: HANDLE) -> super::super::super::Win32::Foundation:: NTSTATUS);
    unsafe { FwpmCalloutCreateEnumHandle0(enginehandle, enumtemplate.unwrap_or(core::mem::zeroed()) as _, enumhandle as _) }
}
#[inline]
pub unsafe fn FwpmCalloutDeleteById0(enginehandle: super::super::super::Win32::Foundation::HANDLE, id: u32) -> super::super::super::Win32::Foundation::NTSTATUS {
    windows_core::link!("fwpuclnt.dll" "system" fn FwpmCalloutDeleteById0(enginehandle : super::super::super::Win32::Foundation:: HANDLE, id : u32) -> super::super::super::Win32::Foundation:: NTSTATUS);
    unsafe { FwpmCalloutDeleteById0(enginehandle, id) }
}
#[inline]
pub unsafe fn FwpmCalloutDeleteByKey0(enginehandle: super::super::super::Win32::Foundation::HANDLE, key: *const windows_core::GUID) -> super::super::super::Win32::Foundation::NTSTATUS {
    windows_core::link!("fwpuclnt.dll" "system" fn FwpmCalloutDeleteByKey0(enginehandle : super::super::super::Win32::Foundation:: HANDLE, key : *const windows_core::GUID) -> super::super::super::Win32::Foundation:: NTSTATUS);
    unsafe { FwpmCalloutDeleteByKey0(enginehandle, key) }
}
#[inline]
pub unsafe fn FwpmCalloutDestroyEnumHandle0(enginehandle: super::super::super::Win32::Foundation::HANDLE, enumhandle: super::super::super::Win32::Foundation::HANDLE) -> super::super::super::Win32::Foundation::NTSTATUS {
    windows_core::link!("fwpuclnt.dll" "system" fn FwpmCalloutDestroyEnumHandle0(enginehandle : super::super::super::Win32::Foundation:: HANDLE, enumhandle : super::super::super::Win32::Foundation:: HANDLE) -> super::super::super::Win32::Foundation:: NTSTATUS);
    unsafe { FwpmCalloutDestroyEnumHandle0(enginehandle, enumhandle as _) }
}
#[cfg(feature = "Win32_NetworkManagement_WindowsFilteringPlatform")]
#[inline]
pub unsafe fn FwpmCalloutEnum0(enginehandle: super::super::super::Win32::Foundation::HANDLE, enumhandle: super::super::super::Win32::Foundation::HANDLE, numentriesrequested: u32, entries: *mut *mut *mut super::super::super::Win32::NetworkManagement::WindowsFilteringPlatform::FWPM_CALLOUT0, numentriesreturned: *mut u32) -> super::super::super::Win32::Foundation::NTSTATUS {
    windows_core::link!("fwpuclnt.dll" "system" fn FwpmCalloutEnum0(enginehandle : super::super::super::Win32::Foundation:: HANDLE, enumhandle : super::super::super::Win32::Foundation:: HANDLE, numentriesrequested : u32, entries : *mut *mut *mut super::super::super::Win32::NetworkManagement::WindowsFilteringPlatform:: FWPM_CALLOUT0, numentriesreturned : *mut u32) -> super::super::super::Win32::Foundation:: NTSTATUS);
    unsafe { FwpmCalloutEnum0(enginehandle, enumhandle, numentriesrequested, entries as _, numentriesreturned as _) }
}
#[cfg(feature = "Win32_NetworkManagement_WindowsFilteringPlatform")]
#[inline]
pub unsafe fn FwpmCalloutGetById0(enginehandle: super::super::super::Win32::Foundation::HANDLE, id: u32, callout: *mut *mut super::super::super::Win32::NetworkManagement::WindowsFilteringPlatform::FWPM_CALLOUT0) -> super::super::super::Win32::Foundation::NTSTATUS {
    windows_core::link!("fwpuclnt.dll" "system" fn FwpmCalloutGetById0(enginehandle : super::super::super::Win32::Foundation:: HANDLE, id : u32, callout : *mut *mut super::super::super::Win32::NetworkManagement::WindowsFilteringPlatform:: FWPM_CALLOUT0) -> super::super::super::Win32::Foundation:: NTSTATUS);
    unsafe { FwpmCalloutGetById0(enginehandle, id, callout as _) }
}
#[cfg(feature = "Win32_NetworkManagement_WindowsFilteringPlatform")]
#[inline]
pub unsafe fn FwpmCalloutGetByKey0(enginehandle: super::super::super::Win32::Foundation::HANDLE, key: *const windows_core::GUID, callout: *mut *mut super::super::super::Win32::NetworkManagement::WindowsFilteringPlatform::FWPM_CALLOUT0) -> super::super::super::Win32::Foundation::NTSTATUS {
    windows_core::link!("fwpuclnt.dll" "system" fn FwpmCalloutGetByKey0(enginehandle : super::super::super::Win32::Foundation:: HANDLE, key : *const windows_core::GUID, callout : *mut *mut super::super::super::Win32::NetworkManagement::WindowsFilteringPlatform:: FWPM_CALLOUT0) -> super::super::super::Win32::Foundation:: NTSTATUS);
    unsafe { FwpmCalloutGetByKey0(enginehandle, key, callout as _) }
}
#[cfg(feature = "Win32_Security")]
#[inline]
pub unsafe fn FwpmCalloutGetSecurityInfoByKey0(enginehandle: super::super::super::Win32::Foundation::HANDLE, key: Option<*const windows_core::GUID>, securityinfo: u32, sidowner: *mut super::super::super::Win32::Security::PSID, sidgroup: *mut super::super::super::Win32::Security::PSID, dacl: *mut *mut super::super::super::Win32::Security::ACL, sacl: *mut *mut super::super::super::Win32::Security::ACL, securitydescriptor: *mut super::super::super::Win32::Security::PSECURITY_DESCRIPTOR) -> super::super::super::Win32::Foundation::NTSTATUS {
    windows_core::link!("fwpuclnt.dll" "system" fn FwpmCalloutGetSecurityInfoByKey0(enginehandle : super::super::super::Win32::Foundation:: HANDLE, key : *const windows_core::GUID, securityinfo : u32, sidowner : *mut super::super::super::Win32::Security:: PSID, sidgroup : *mut super::super::super::Win32::Security:: PSID, dacl : *mut *mut super::super::super::Win32::Security:: ACL, sacl : *mut *mut super::super::super::Win32::Security:: ACL, securitydescriptor : *mut super::super::super::Win32::Security:: PSECURITY_DESCRIPTOR) -> super::super::super::Win32::Foundation:: NTSTATUS);
    unsafe { FwpmCalloutGetSecurityInfoByKey0(enginehandle, key.unwrap_or(core::mem::zeroed()) as _, securityinfo, sidowner as _, sidgroup as _, dacl as _, sacl as _, securitydescriptor as _) }
}
#[cfg(feature = "Win32_Security")]
#[inline]
pub unsafe fn FwpmCalloutSetSecurityInfoByKey0(enginehandle: super::super::super::Win32::Foundation::HANDLE, key: Option<*const windows_core::GUID>, securityinfo: u32, sidowner: Option<*const super::super::super::Win32::Security::SID>, sidgroup: Option<*const super::super::super::Win32::Security::SID>, dacl: Option<*const super::super::super::Win32::Security::ACL>, sacl: Option<*const super::super::super::Win32::Security::ACL>) -> super::super::super::Win32::Foundation::NTSTATUS {
    windows_core::link!("fwpuclnt.dll" "system" fn FwpmCalloutSetSecurityInfoByKey0(enginehandle : super::super::super::Win32::Foundation:: HANDLE, key : *const windows_core::GUID, securityinfo : u32, sidowner : *const super::super::super::Win32::Security:: SID, sidgroup : *const super::super::super::Win32::Security:: SID, dacl : *const super::super::super::Win32::Security:: ACL, sacl : *const super::super::super::Win32::Security:: ACL) -> super::super::super::Win32::Foundation:: NTSTATUS);
    unsafe { FwpmCalloutSetSecurityInfoByKey0(enginehandle, key.unwrap_or(core::mem::zeroed()) as _, securityinfo, sidowner.unwrap_or(core::mem::zeroed()) as _, sidgroup.unwrap_or(core::mem::zeroed()) as _, dacl.unwrap_or(core::mem::zeroed()) as _, sacl.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "Win32_NetworkManagement_WindowsFilteringPlatform")]
#[inline]
pub unsafe fn FwpmConnectionCreateEnumHandle0(enginehandle: super::super::super::Win32::Foundation::HANDLE, enumtemplate: Option<*const super::super::super::Win32::NetworkManagement::WindowsFilteringPlatform::FWPM_CONNECTION_ENUM_TEMPLATE0>, enumhandle: *mut super::super::super::Win32::Foundation::HANDLE) -> super::super::super::Win32::Foundation::NTSTATUS {
    windows_core::link!("fwpuclnt.dll" "system" fn FwpmConnectionCreateEnumHandle0(enginehandle : super::super::super::Win32::Foundation:: HANDLE, enumtemplate : *const super::super::super::Win32::NetworkManagement::WindowsFilteringPlatform:: FWPM_CONNECTION_ENUM_TEMPLATE0, enumhandle : *mut super::super::super::Win32::Foundation:: HANDLE) -> super::super::super::Win32::Foundation:: NTSTATUS);
    unsafe { FwpmConnectionCreateEnumHandle0(enginehandle, enumtemplate.unwrap_or(core::mem::zeroed()) as _, enumhandle as _) }
}
#[inline]
pub unsafe fn FwpmConnectionDestroyEnumHandle0(enginehandle: super::super::super::Win32::Foundation::HANDLE, enumhandle: super::super::super::Win32::Foundation::HANDLE) -> super::super::super::Win32::Foundation::NTSTATUS {
    windows_core::link!("fwpuclnt.dll" "system" fn FwpmConnectionDestroyEnumHandle0(enginehandle : super::super::super::Win32::Foundation:: HANDLE, enumhandle : super::super::super::Win32::Foundation:: HANDLE) -> super::super::super::Win32::Foundation:: NTSTATUS);
    unsafe { FwpmConnectionDestroyEnumHandle0(enginehandle, enumhandle) }
}
#[cfg(feature = "Win32_NetworkManagement_WindowsFilteringPlatform")]
#[inline]
pub unsafe fn FwpmConnectionEnum0(enginehandle: super::super::super::Win32::Foundation::HANDLE, enumhandle: super::super::super::Win32::Foundation::HANDLE, numentriesrequested: u32, entries: *mut *mut *mut super::super::super::Win32::NetworkManagement::WindowsFilteringPlatform::FWPM_CONNECTION0, numentriesreturned: *mut u32) -> super::super::super::Win32::Foundation::NTSTATUS {
    windows_core::link!("fwpuclnt.dll" "system" fn FwpmConnectionEnum0(enginehandle : super::super::super::Win32::Foundation:: HANDLE, enumhandle : super::super::super::Win32::Foundation:: HANDLE, numentriesrequested : u32, entries : *mut *mut *mut super::super::super::Win32::NetworkManagement::WindowsFilteringPlatform:: FWPM_CONNECTION0, numentriesreturned : *mut u32) -> super::super::super::Win32::Foundation:: NTSTATUS);
    unsafe { FwpmConnectionEnum0(enginehandle, enumhandle, numentriesrequested, entries as _, numentriesreturned as _) }
}
#[cfg(feature = "Win32_NetworkManagement_WindowsFilteringPlatform")]
#[inline]
pub unsafe fn FwpmConnectionGetById0(enginehandle: super::super::super::Win32::Foundation::HANDLE, id: u64, connection: *mut *mut super::super::super::Win32::NetworkManagement::WindowsFilteringPlatform::FWPM_CONNECTION0) -> super::super::super::Win32::Foundation::NTSTATUS {
    windows_core::link!("fwpuclnt.dll" "system" fn FwpmConnectionGetById0(enginehandle : super::super::super::Win32::Foundation:: HANDLE, id : u64, connection : *mut *mut super::super::super::Win32::NetworkManagement::WindowsFilteringPlatform:: FWPM_CONNECTION0) -> super::super::super::Win32::Foundation:: NTSTATUS);
    unsafe { FwpmConnectionGetById0(enginehandle, id, connection as _) }
}
#[cfg(feature = "Win32_Security")]
#[inline]
pub unsafe fn FwpmConnectionGetSecurityInfo0(enginehandle: super::super::super::Win32::Foundation::HANDLE, securityinfo: u32, sidowner: *mut super::super::super::Win32::Security::PSID, sidgroup: *mut super::super::super::Win32::Security::PSID, dacl: *mut *mut super::super::super::Win32::Security::ACL, sacl: *mut *mut super::super::super::Win32::Security::ACL, securitydescriptor: *mut super::super::super::Win32::Security::PSECURITY_DESCRIPTOR) -> super::super::super::Win32::Foundation::NTSTATUS {
    windows_core::link!("fwpuclnt.dll" "system" fn FwpmConnectionGetSecurityInfo0(enginehandle : super::super::super::Win32::Foundation:: HANDLE, securityinfo : u32, sidowner : *mut super::super::super::Win32::Security:: PSID, sidgroup : *mut super::super::super::Win32::Security:: PSID, dacl : *mut *mut super::super::super::Win32::Security:: ACL, sacl : *mut *mut super::super::super::Win32::Security:: ACL, securitydescriptor : *mut super::super::super::Win32::Security:: PSECURITY_DESCRIPTOR) -> super::super::super::Win32::Foundation:: NTSTATUS);
    unsafe { FwpmConnectionGetSecurityInfo0(enginehandle, securityinfo, sidowner as _, sidgroup as _, dacl as _, sacl as _, securitydescriptor as _) }
}
#[cfg(feature = "Win32_Security")]
#[inline]
pub unsafe fn FwpmConnectionSetSecurityInfo0(enginehandle: super::super::super::Win32::Foundation::HANDLE, securityinfo: u32, sidowner: Option<*const super::super::super::Win32::Security::SID>, sidgroup: Option<*const super::super::super::Win32::Security::SID>, dacl: Option<*const super::super::super::Win32::Security::ACL>, sacl: Option<*const super::super::super::Win32::Security::ACL>) -> super::super::super::Win32::Foundation::NTSTATUS {
    windows_core::link!("fwpuclnt.dll" "system" fn FwpmConnectionSetSecurityInfo0(enginehandle : super::super::super::Win32::Foundation:: HANDLE, securityinfo : u32, sidowner : *const super::super::super::Win32::Security:: SID, sidgroup : *const super::super::super::Win32::Security:: SID, dacl : *const super::super::super::Win32::Security:: ACL, sacl : *const super::super::super::Win32::Security:: ACL) -> super::super::super::Win32::Foundation:: NTSTATUS);
    unsafe { FwpmConnectionSetSecurityInfo0(enginehandle, securityinfo, sidowner.unwrap_or(core::mem::zeroed()) as _, sidgroup.unwrap_or(core::mem::zeroed()) as _, dacl.unwrap_or(core::mem::zeroed()) as _, sacl.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn FwpmEngineClose0(enginehandle: super::super::super::Win32::Foundation::HANDLE) -> super::super::super::Win32::Foundation::NTSTATUS {
    windows_core::link!("fwpuclnt.dll" "system" fn FwpmEngineClose0(enginehandle : super::super::super::Win32::Foundation:: HANDLE) -> super::super::super::Win32::Foundation:: NTSTATUS);
    unsafe { FwpmEngineClose0(enginehandle as _) }
}
#[cfg(all(feature = "Win32_NetworkManagement_WindowsFilteringPlatform", feature = "Win32_Security"))]
#[inline]
pub unsafe fn FwpmEngineGetOption0(enginehandle: super::super::super::Win32::Foundation::HANDLE, option: super::super::super::Win32::NetworkManagement::WindowsFilteringPlatform::FWPM_ENGINE_OPTION, value: *mut *mut super::super::super::Win32::NetworkManagement::WindowsFilteringPlatform::FWP_VALUE0) -> super::super::super::Win32::Foundation::NTSTATUS {
    windows_core::link!("fwpuclnt.dll" "system" fn FwpmEngineGetOption0(enginehandle : super::super::super::Win32::Foundation:: HANDLE, option : super::super::super::Win32::NetworkManagement::WindowsFilteringPlatform:: FWPM_ENGINE_OPTION, value : *mut *mut super::super::super::Win32::NetworkManagement::WindowsFilteringPlatform:: FWP_VALUE0) -> super::super::super::Win32::Foundation:: NTSTATUS);
    unsafe { FwpmEngineGetOption0(enginehandle, option, value as _) }
}
#[cfg(feature = "Win32_Security")]
#[inline]
pub unsafe fn FwpmEngineGetSecurityInfo0(enginehandle: super::super::super::Win32::Foundation::HANDLE, securityinfo: u32, sidowner: *mut super::super::super::Win32::Security::PSID, sidgroup: *mut super::super::super::Win32::Security::PSID, dacl: *mut *mut super::super::super::Win32::Security::ACL, sacl: *mut *mut super::super::super::Win32::Security::ACL, securitydescriptor: *mut super::super::super::Win32::Security::PSECURITY_DESCRIPTOR) -> super::super::super::Win32::Foundation::NTSTATUS {
    windows_core::link!("fwpuclnt.dll" "system" fn FwpmEngineGetSecurityInfo0(enginehandle : super::super::super::Win32::Foundation:: HANDLE, securityinfo : u32, sidowner : *mut super::super::super::Win32::Security:: PSID, sidgroup : *mut super::super::super::Win32::Security:: PSID, dacl : *mut *mut super::super::super::Win32::Security:: ACL, sacl : *mut *mut super::super::super::Win32::Security:: ACL, securitydescriptor : *mut super::super::super::Win32::Security:: PSECURITY_DESCRIPTOR) -> super::super::super::Win32::Foundation:: NTSTATUS);
    unsafe { FwpmEngineGetSecurityInfo0(enginehandle, securityinfo, sidowner as _, sidgroup as _, dacl as _, sacl as _, securitydescriptor as _) }
}
#[cfg(all(feature = "Win32_NetworkManagement_WindowsFilteringPlatform", feature = "Win32_Security", feature = "Win32_System_Rpc"))]
#[inline]
pub unsafe fn FwpmEngineOpen0<P0>(servername: P0, authnservice: u32, authidentity: Option<*const super::super::super::Win32::System::Rpc::SEC_WINNT_AUTH_IDENTITY_W>, session: Option<*const super::super::super::Win32::NetworkManagement::WindowsFilteringPlatform::FWPM_SESSION0>, enginehandle: *mut super::super::super::Win32::Foundation::HANDLE) -> super::super::super::Win32::Foundation::NTSTATUS
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("fwpuclnt.dll" "system" fn FwpmEngineOpen0(servername : windows_core::PCWSTR, authnservice : u32, authidentity : *const super::super::super::Win32::System::Rpc:: SEC_WINNT_AUTH_IDENTITY_W, session : *const super::super::super::Win32::NetworkManagement::WindowsFilteringPlatform:: FWPM_SESSION0, enginehandle : *mut super::super::super::Win32::Foundation:: HANDLE) -> super::super::super::Win32::Foundation:: NTSTATUS);
    unsafe { FwpmEngineOpen0(servername.param().abi(), authnservice, authidentity.unwrap_or(core::mem::zeroed()) as _, session.unwrap_or(core::mem::zeroed()) as _, enginehandle as _) }
}
#[cfg(all(feature = "Win32_NetworkManagement_WindowsFilteringPlatform", feature = "Win32_Security"))]
#[inline]
pub unsafe fn FwpmEngineSetOption0(enginehandle: super::super::super::Win32::Foundation::HANDLE, option: super::super::super::Win32::NetworkManagement::WindowsFilteringPlatform::FWPM_ENGINE_OPTION, newvalue: *const super::super::super::Win32::NetworkManagement::WindowsFilteringPlatform::FWP_VALUE0) -> super::super::super::Win32::Foundation::NTSTATUS {
    windows_core::link!("fwpuclnt.dll" "system" fn FwpmEngineSetOption0(enginehandle : super::super::super::Win32::Foundation:: HANDLE, option : super::super::super::Win32::NetworkManagement::WindowsFilteringPlatform:: FWPM_ENGINE_OPTION, newvalue : *const super::super::super::Win32::NetworkManagement::WindowsFilteringPlatform:: FWP_VALUE0) -> super::super::super::Win32::Foundation:: NTSTATUS);
    unsafe { FwpmEngineSetOption0(enginehandle, option, newvalue) }
}
#[cfg(feature = "Win32_Security")]
#[inline]
pub unsafe fn FwpmEngineSetSecurityInfo0(enginehandle: super::super::super::Win32::Foundation::HANDLE, securityinfo: u32, sidowner: Option<*const super::super::super::Win32::Security::SID>, sidgroup: Option<*const super::super::super::Win32::Security::SID>, dacl: Option<*const super::super::super::Win32::Security::ACL>, sacl: Option<*const super::super::super::Win32::Security::ACL>) -> super::super::super::Win32::Foundation::NTSTATUS {
    windows_core::link!("fwpuclnt.dll" "system" fn FwpmEngineSetSecurityInfo0(enginehandle : super::super::super::Win32::Foundation:: HANDLE, securityinfo : u32, sidowner : *const super::super::super::Win32::Security:: SID, sidgroup : *const super::super::super::Win32::Security:: SID, dacl : *const super::super::super::Win32::Security:: ACL, sacl : *const super::super::super::Win32::Security:: ACL) -> super::super::super::Win32::Foundation:: NTSTATUS);
    unsafe { FwpmEngineSetSecurityInfo0(enginehandle, securityinfo, sidowner.unwrap_or(core::mem::zeroed()) as _, sidgroup.unwrap_or(core::mem::zeroed()) as _, dacl.unwrap_or(core::mem::zeroed()) as _, sacl.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "Win32_NetworkManagement_WindowsFilteringPlatform", feature = "Win32_Security"))]
#[inline]
pub unsafe fn FwpmFilterAdd0(enginehandle: super::super::super::Win32::Foundation::HANDLE, filter: *const super::super::super::Win32::NetworkManagement::WindowsFilteringPlatform::FWPM_FILTER0, sd: Option<super::super::super::Win32::Security::PSECURITY_DESCRIPTOR>, id: Option<*mut u64>) -> super::super::super::Win32::Foundation::NTSTATUS {
    windows_core::link!("fwpuclnt.dll" "system" fn FwpmFilterAdd0(enginehandle : super::super::super::Win32::Foundation:: HANDLE, filter : *const super::super::super::Win32::NetworkManagement::WindowsFilteringPlatform:: FWPM_FILTER0, sd : super::super::super::Win32::Security:: PSECURITY_DESCRIPTOR, id : *mut u64) -> super::super::super::Win32::Foundation:: NTSTATUS);
    unsafe { FwpmFilterAdd0(enginehandle, filter, sd.unwrap_or(core::mem::zeroed()) as _, id.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "Win32_NetworkManagement_WindowsFilteringPlatform", feature = "Win32_Security"))]
#[inline]
pub unsafe fn FwpmFilterCreateEnumHandle0(enginehandle: super::super::super::Win32::Foundation::HANDLE, enumtemplate: Option<*const super::super::super::Win32::NetworkManagement::WindowsFilteringPlatform::FWPM_FILTER_ENUM_TEMPLATE0>, enumhandle: *mut super::super::super::Win32::Foundation::HANDLE) -> super::super::super::Win32::Foundation::NTSTATUS {
    windows_core::link!("fwpuclnt.dll" "system" fn FwpmFilterCreateEnumHandle0(enginehandle : super::super::super::Win32::Foundation:: HANDLE, enumtemplate : *const super::super::super::Win32::NetworkManagement::WindowsFilteringPlatform:: FWPM_FILTER_ENUM_TEMPLATE0, enumhandle : *mut super::super::super::Win32::Foundation:: HANDLE) -> super::super::super::Win32::Foundation:: NTSTATUS);
    unsafe { FwpmFilterCreateEnumHandle0(enginehandle, enumtemplate.unwrap_or(core::mem::zeroed()) as _, enumhandle as _) }
}
#[inline]
pub unsafe fn FwpmFilterDeleteById0(enginehandle: super::super::super::Win32::Foundation::HANDLE, id: u64) -> super::super::super::Win32::Foundation::NTSTATUS {
    windows_core::link!("fwpuclnt.dll" "system" fn FwpmFilterDeleteById0(enginehandle : super::super::super::Win32::Foundation:: HANDLE, id : u64) -> super::super::super::Win32::Foundation:: NTSTATUS);
    unsafe { FwpmFilterDeleteById0(enginehandle, id) }
}
#[inline]
pub unsafe fn FwpmFilterDeleteByKey0(enginehandle: super::super::super::Win32::Foundation::HANDLE, key: *const windows_core::GUID) -> super::super::super::Win32::Foundation::NTSTATUS {
    windows_core::link!("fwpuclnt.dll" "system" fn FwpmFilterDeleteByKey0(enginehandle : super::super::super::Win32::Foundation:: HANDLE, key : *const windows_core::GUID) -> super::super::super::Win32::Foundation:: NTSTATUS);
    unsafe { FwpmFilterDeleteByKey0(enginehandle, key) }
}
#[inline]
pub unsafe fn FwpmFilterDestroyEnumHandle0(enginehandle: super::super::super::Win32::Foundation::HANDLE, enumhandle: super::super::super::Win32::Foundation::HANDLE) -> super::super::super::Win32::Foundation::NTSTATUS {
    windows_core::link!("fwpuclnt.dll" "system" fn FwpmFilterDestroyEnumHandle0(enginehandle : super::super::super::Win32::Foundation:: HANDLE, enumhandle : super::super::super::Win32::Foundation:: HANDLE) -> super::super::super::Win32::Foundation:: NTSTATUS);
    unsafe { FwpmFilterDestroyEnumHandle0(enginehandle, enumhandle as _) }
}
#[cfg(all(feature = "Win32_NetworkManagement_WindowsFilteringPlatform", feature = "Win32_Security"))]
#[inline]
pub unsafe fn FwpmFilterEnum0(enginehandle: super::super::super::Win32::Foundation::HANDLE, enumhandle: super::super::super::Win32::Foundation::HANDLE, numentriesrequested: u32, entries: *mut *mut *mut super::super::super::Win32::NetworkManagement::WindowsFilteringPlatform::FWPM_FILTER0, numentriesreturned: *mut u32) -> super::super::super::Win32::Foundation::NTSTATUS {
    windows_core::link!("fwpuclnt.dll" "system" fn FwpmFilterEnum0(enginehandle : super::super::super::Win32::Foundation:: HANDLE, enumhandle : super::super::super::Win32::Foundation:: HANDLE, numentriesrequested : u32, entries : *mut *mut *mut super::super::super::Win32::NetworkManagement::WindowsFilteringPlatform:: FWPM_FILTER0, numentriesreturned : *mut u32) -> super::super::super::Win32::Foundation:: NTSTATUS);
    unsafe { FwpmFilterEnum0(enginehandle, enumhandle, numentriesrequested, entries as _, numentriesreturned as _) }
}
#[cfg(all(feature = "Win32_NetworkManagement_WindowsFilteringPlatform", feature = "Win32_Security"))]
#[inline]
pub unsafe fn FwpmFilterGetById0(enginehandle: super::super::super::Win32::Foundation::HANDLE, id: u64, filter: *mut *mut super::super::super::Win32::NetworkManagement::WindowsFilteringPlatform::FWPM_FILTER0) -> super::super::super::Win32::Foundation::NTSTATUS {
    windows_core::link!("fwpuclnt.dll" "system" fn FwpmFilterGetById0(enginehandle : super::super::super::Win32::Foundation:: HANDLE, id : u64, filter : *mut *mut super::super::super::Win32::NetworkManagement::WindowsFilteringPlatform:: FWPM_FILTER0) -> super::super::super::Win32::Foundation:: NTSTATUS);
    unsafe { FwpmFilterGetById0(enginehandle, id, filter as _) }
}
#[cfg(all(feature = "Win32_NetworkManagement_WindowsFilteringPlatform", feature = "Win32_Security"))]
#[inline]
pub unsafe fn FwpmFilterGetByKey0(enginehandle: super::super::super::Win32::Foundation::HANDLE, key: *const windows_core::GUID, filter: *mut *mut super::super::super::Win32::NetworkManagement::WindowsFilteringPlatform::FWPM_FILTER0) -> super::super::super::Win32::Foundation::NTSTATUS {
    windows_core::link!("fwpuclnt.dll" "system" fn FwpmFilterGetByKey0(enginehandle : super::super::super::Win32::Foundation:: HANDLE, key : *const windows_core::GUID, filter : *mut *mut super::super::super::Win32::NetworkManagement::WindowsFilteringPlatform:: FWPM_FILTER0) -> super::super::super::Win32::Foundation:: NTSTATUS);
    unsafe { FwpmFilterGetByKey0(enginehandle, key, filter as _) }
}
#[cfg(feature = "Win32_Security")]
#[inline]
pub unsafe fn FwpmFilterGetSecurityInfoByKey0(enginehandle: super::super::super::Win32::Foundation::HANDLE, key: Option<*const windows_core::GUID>, securityinfo: u32, sidowner: *mut super::super::super::Win32::Security::PSID, sidgroup: *mut super::super::super::Win32::Security::PSID, dacl: *mut *mut super::super::super::Win32::Security::ACL, sacl: *mut *mut super::super::super::Win32::Security::ACL, securitydescriptor: *mut super::super::super::Win32::Security::PSECURITY_DESCRIPTOR) -> super::super::super::Win32::Foundation::NTSTATUS {
    windows_core::link!("fwpuclnt.dll" "system" fn FwpmFilterGetSecurityInfoByKey0(enginehandle : super::super::super::Win32::Foundation:: HANDLE, key : *const windows_core::GUID, securityinfo : u32, sidowner : *mut super::super::super::Win32::Security:: PSID, sidgroup : *mut super::super::super::Win32::Security:: PSID, dacl : *mut *mut super::super::super::Win32::Security:: ACL, sacl : *mut *mut super::super::super::Win32::Security:: ACL, securitydescriptor : *mut super::super::super::Win32::Security:: PSECURITY_DESCRIPTOR) -> super::super::super::Win32::Foundation:: NTSTATUS);
    unsafe { FwpmFilterGetSecurityInfoByKey0(enginehandle, key.unwrap_or(core::mem::zeroed()) as _, securityinfo, sidowner as _, sidgroup as _, dacl as _, sacl as _, securitydescriptor as _) }
}
#[cfg(feature = "Win32_Security")]
#[inline]
pub unsafe fn FwpmFilterSetSecurityInfoByKey0(enginehandle: super::super::super::Win32::Foundation::HANDLE, key: Option<*const windows_core::GUID>, securityinfo: u32, sidowner: Option<*const super::super::super::Win32::Security::SID>, sidgroup: Option<*const super::super::super::Win32::Security::SID>, dacl: Option<*const super::super::super::Win32::Security::ACL>, sacl: Option<*const super::super::super::Win32::Security::ACL>) -> super::super::super::Win32::Foundation::NTSTATUS {
    windows_core::link!("fwpuclnt.dll" "system" fn FwpmFilterSetSecurityInfoByKey0(enginehandle : super::super::super::Win32::Foundation:: HANDLE, key : *const windows_core::GUID, securityinfo : u32, sidowner : *const super::super::super::Win32::Security:: SID, sidgroup : *const super::super::super::Win32::Security:: SID, dacl : *const super::super::super::Win32::Security:: ACL, sacl : *const super::super::super::Win32::Security:: ACL) -> super::super::super::Win32::Foundation:: NTSTATUS);
    unsafe { FwpmFilterSetSecurityInfoByKey0(enginehandle, key.unwrap_or(core::mem::zeroed()) as _, securityinfo, sidowner.unwrap_or(core::mem::zeroed()) as _, sidgroup.unwrap_or(core::mem::zeroed()) as _, dacl.unwrap_or(core::mem::zeroed()) as _, sacl.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn FwpmFreeMemory0(p: *mut *mut core::ffi::c_void) {
    windows_core::link!("fwpuclnt.dll" "system" fn FwpmFreeMemory0(p : *mut *mut core::ffi::c_void));
    unsafe { FwpmFreeMemory0(p as _) }
}
#[cfg(all(feature = "Win32_NetworkManagement_WindowsFilteringPlatform", feature = "Win32_Security"))]
#[inline]
pub unsafe fn FwpmIPsecTunnelAdd0(enginehandle: super::super::super::Win32::Foundation::HANDLE, flags: u32, mainmodepolicy: Option<*const super::super::super::Win32::NetworkManagement::WindowsFilteringPlatform::FWPM_PROVIDER_CONTEXT0>, tunnelpolicy: *const super::super::super::Win32::NetworkManagement::WindowsFilteringPlatform::FWPM_PROVIDER_CONTEXT0, filterconditions: &[super::super::super::Win32::NetworkManagement::WindowsFilteringPlatform::FWPM_FILTER_CONDITION0], sd: Option<super::super::super::Win32::Security::PSECURITY_DESCRIPTOR>) -> super::super::super::Win32::Foundation::NTSTATUS {
    windows_core::link!("fwpuclnt.dll" "system" fn FwpmIPsecTunnelAdd0(enginehandle : super::super::super::Win32::Foundation:: HANDLE, flags : u32, mainmodepolicy : *const super::super::super::Win32::NetworkManagement::WindowsFilteringPlatform:: FWPM_PROVIDER_CONTEXT0, tunnelpolicy : *const super::super::super::Win32::NetworkManagement::WindowsFilteringPlatform:: FWPM_PROVIDER_CONTEXT0, numfilterconditions : u32, filterconditions : *const super::super::super::Win32::NetworkManagement::WindowsFilteringPlatform:: FWPM_FILTER_CONDITION0, sd : super::super::super::Win32::Security:: PSECURITY_DESCRIPTOR) -> super::super::super::Win32::Foundation:: NTSTATUS);
    unsafe { FwpmIPsecTunnelAdd0(enginehandle, flags, mainmodepolicy.unwrap_or(core::mem::zeroed()) as _, tunnelpolicy, filterconditions.len().try_into().unwrap(), core::mem::transmute(filterconditions.as_ptr()), sd.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "Win32_NetworkManagement_WindowsFilteringPlatform", feature = "Win32_Security"))]
#[inline]
pub unsafe fn FwpmIPsecTunnelAdd1(enginehandle: super::super::super::Win32::Foundation::HANDLE, flags: u32, mainmodepolicy: Option<*const super::super::super::Win32::NetworkManagement::WindowsFilteringPlatform::FWPM_PROVIDER_CONTEXT1>, tunnelpolicy: *const super::super::super::Win32::NetworkManagement::WindowsFilteringPlatform::FWPM_PROVIDER_CONTEXT1, filterconditions: &[super::super::super::Win32::NetworkManagement::WindowsFilteringPlatform::FWPM_FILTER_CONDITION0], keymodkey: Option<*const windows_core::GUID>, sd: Option<super::super::super::Win32::Security::PSECURITY_DESCRIPTOR>) -> super::super::super::Win32::Foundation::NTSTATUS {
    windows_core::link!("fwpuclnt.dll" "system" fn FwpmIPsecTunnelAdd1(enginehandle : super::super::super::Win32::Foundation:: HANDLE, flags : u32, mainmodepolicy : *const super::super::super::Win32::NetworkManagement::WindowsFilteringPlatform:: FWPM_PROVIDER_CONTEXT1, tunnelpolicy : *const super::super::super::Win32::NetworkManagement::WindowsFilteringPlatform:: FWPM_PROVIDER_CONTEXT1, numfilterconditions : u32, filterconditions : *const super::super::super::Win32::NetworkManagement::WindowsFilteringPlatform:: FWPM_FILTER_CONDITION0, keymodkey : *const windows_core::GUID, sd : super::super::super::Win32::Security:: PSECURITY_DESCRIPTOR) -> super::super::super::Win32::Foundation:: NTSTATUS);
    unsafe { FwpmIPsecTunnelAdd1(enginehandle, flags, mainmodepolicy.unwrap_or(core::mem::zeroed()) as _, tunnelpolicy, filterconditions.len().try_into().unwrap(), core::mem::transmute(filterconditions.as_ptr()), keymodkey.unwrap_or(core::mem::zeroed()) as _, sd.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "Win32_NetworkManagement_WindowsFilteringPlatform", feature = "Win32_Security"))]
#[inline]
pub unsafe fn FwpmIPsecTunnelAdd2(enginehandle: super::super::super::Win32::Foundation::HANDLE, flags: u32, mainmodepolicy: Option<*const super::super::super::Win32::NetworkManagement::WindowsFilteringPlatform::FWPM_PROVIDER_CONTEXT2>, tunnelpolicy: *const super::super::super::Win32::NetworkManagement::WindowsFilteringPlatform::FWPM_PROVIDER_CONTEXT2, filterconditions: &[super::super::super::Win32::NetworkManagement::WindowsFilteringPlatform::FWPM_FILTER_CONDITION0], keymodkey: Option<*const windows_core::GUID>, sd: Option<super::super::super::Win32::Security::PSECURITY_DESCRIPTOR>) -> super::super::super::Win32::Foundation::NTSTATUS {
    windows_core::link!("fwpuclnt.dll" "system" fn FwpmIPsecTunnelAdd2(enginehandle : super::super::super::Win32::Foundation:: HANDLE, flags : u32, mainmodepolicy : *const super::super::super::Win32::NetworkManagement::WindowsFilteringPlatform:: FWPM_PROVIDER_CONTEXT2, tunnelpolicy : *const super::super::super::Win32::NetworkManagement::WindowsFilteringPlatform:: FWPM_PROVIDER_CONTEXT2, numfilterconditions : u32, filterconditions : *const super::super::super::Win32::NetworkManagement::WindowsFilteringPlatform:: FWPM_FILTER_CONDITION0, keymodkey : *const windows_core::GUID, sd : super::super::super::Win32::Security:: PSECURITY_DESCRIPTOR) -> super::super::super::Win32::Foundation:: NTSTATUS);
    unsafe { FwpmIPsecTunnelAdd2(enginehandle, flags, mainmodepolicy.unwrap_or(core::mem::zeroed()) as _, tunnelpolicy, filterconditions.len().try_into().unwrap(), core::mem::transmute(filterconditions.as_ptr()), keymodkey.unwrap_or(core::mem::zeroed()) as _, sd.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "Win32_NetworkManagement_WindowsFilteringPlatform", feature = "Win32_Security"))]
#[inline]
pub unsafe fn FwpmIPsecTunnelAdd3(enginehandle: super::super::super::Win32::Foundation::HANDLE, flags: u32, mainmodepolicy: Option<*const super::super::super::Win32::NetworkManagement::WindowsFilteringPlatform::FWPM_PROVIDER_CONTEXT3>, tunnelpolicy: *const super::super::super::Win32::NetworkManagement::WindowsFilteringPlatform::FWPM_PROVIDER_CONTEXT3, filterconditions: &[super::super::super::Win32::NetworkManagement::WindowsFilteringPlatform::FWPM_FILTER_CONDITION0], keymodkey: Option<*const windows_core::GUID>, sd: Option<super::super::super::Win32::Security::PSECURITY_DESCRIPTOR>) -> super::super::super::Win32::Foundation::NTSTATUS {
    windows_core::link!("fwpuclnt.dll" "system" fn FwpmIPsecTunnelAdd3(enginehandle : super::super::super::Win32::Foundation:: HANDLE, flags : u32, mainmodepolicy : *const super::super::super::Win32::NetworkManagement::WindowsFilteringPlatform:: FWPM_PROVIDER_CONTEXT3, tunnelpolicy : *const super::super::super::Win32::NetworkManagement::WindowsFilteringPlatform:: FWPM_PROVIDER_CONTEXT3, numfilterconditions : u32, filterconditions : *const super::super::super::Win32::NetworkManagement::WindowsFilteringPlatform:: FWPM_FILTER_CONDITION0, keymodkey : *const windows_core::GUID, sd : super::super::super::Win32::Security:: PSECURITY_DESCRIPTOR) -> super::super::super::Win32::Foundation:: NTSTATUS);
    unsafe { FwpmIPsecTunnelAdd3(enginehandle, flags, mainmodepolicy.unwrap_or(core::mem::zeroed()) as _, tunnelpolicy, filterconditions.len().try_into().unwrap(), core::mem::transmute(filterconditions.as_ptr()), keymodkey.unwrap_or(core::mem::zeroed()) as _, sd.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn FwpmIPsecTunnelDeleteByKey0(enginehandle: super::super::super::Win32::Foundation::HANDLE, key: *const windows_core::GUID) -> super::super::super::Win32::Foundation::NTSTATUS {
    windows_core::link!("fwpuclnt.dll" "system" fn FwpmIPsecTunnelDeleteByKey0(enginehandle : super::super::super::Win32::Foundation:: HANDLE, key : *const windows_core::GUID) -> super::super::super::Win32::Foundation:: NTSTATUS);
    unsafe { FwpmIPsecTunnelDeleteByKey0(enginehandle, key) }
}
#[cfg(feature = "Win32_NetworkManagement_WindowsFilteringPlatform")]
#[inline]
pub unsafe fn FwpmLayerCreateEnumHandle0(enginehandle: super::super::super::Win32::Foundation::HANDLE, enumtemplate: Option<*const super::super::super::Win32::NetworkManagement::WindowsFilteringPlatform::FWPM_LAYER_ENUM_TEMPLATE0>, enumhandle: *mut super::super::super::Win32::Foundation::HANDLE) -> super::super::super::Win32::Foundation::NTSTATUS {
    windows_core::link!("fwpuclnt.dll" "system" fn FwpmLayerCreateEnumHandle0(enginehandle : super::super::super::Win32::Foundation:: HANDLE, enumtemplate : *const super::super::super::Win32::NetworkManagement::WindowsFilteringPlatform:: FWPM_LAYER_ENUM_TEMPLATE0, enumhandle : *mut super::super::super::Win32::Foundation:: HANDLE) -> super::super::super::Win32::Foundation:: NTSTATUS);
    unsafe { FwpmLayerCreateEnumHandle0(enginehandle, enumtemplate.unwrap_or(core::mem::zeroed()) as _, enumhandle as _) }
}
#[inline]
pub unsafe fn FwpmLayerDestroyEnumHandle0(enginehandle: super::super::super::Win32::Foundation::HANDLE, enumhandle: super::super::super::Win32::Foundation::HANDLE) -> super::super::super::Win32::Foundation::NTSTATUS {
    windows_core::link!("fwpuclnt.dll" "system" fn FwpmLayerDestroyEnumHandle0(enginehandle : super::super::super::Win32::Foundation:: HANDLE, enumhandle : super::super::super::Win32::Foundation:: HANDLE) -> super::super::super::Win32::Foundation:: NTSTATUS);
    unsafe { FwpmLayerDestroyEnumHandle0(enginehandle, enumhandle as _) }
}
#[cfg(feature = "Win32_NetworkManagement_WindowsFilteringPlatform")]
#[inline]
pub unsafe fn FwpmLayerEnum0(enginehandle: super::super::super::Win32::Foundation::HANDLE, enumhandle: super::super::super::Win32::Foundation::HANDLE, numentriesrequested: u32, entries: *mut *mut *mut super::super::super::Win32::NetworkManagement::WindowsFilteringPlatform::FWPM_LAYER0, numentriesreturned: *mut u32) -> super::super::super::Win32::Foundation::NTSTATUS {
    windows_core::link!("fwpuclnt.dll" "system" fn FwpmLayerEnum0(enginehandle : super::super::super::Win32::Foundation:: HANDLE, enumhandle : super::super::super::Win32::Foundation:: HANDLE, numentriesrequested : u32, entries : *mut *mut *mut super::super::super::Win32::NetworkManagement::WindowsFilteringPlatform:: FWPM_LAYER0, numentriesreturned : *mut u32) -> super::super::super::Win32::Foundation:: NTSTATUS);
    unsafe { FwpmLayerEnum0(enginehandle, enumhandle, numentriesrequested, entries as _, numentriesreturned as _) }
}
#[cfg(feature = "Win32_NetworkManagement_WindowsFilteringPlatform")]
#[inline]
pub unsafe fn FwpmLayerGetById0(enginehandle: super::super::super::Win32::Foundation::HANDLE, id: u16, layer: *mut *mut super::super::super::Win32::NetworkManagement::WindowsFilteringPlatform::FWPM_LAYER0) -> super::super::super::Win32::Foundation::NTSTATUS {
    windows_core::link!("fwpuclnt.dll" "system" fn FwpmLayerGetById0(enginehandle : super::super::super::Win32::Foundation:: HANDLE, id : u16, layer : *mut *mut super::super::super::Win32::NetworkManagement::WindowsFilteringPlatform:: FWPM_LAYER0) -> super::super::super::Win32::Foundation:: NTSTATUS);
    unsafe { FwpmLayerGetById0(enginehandle, id, layer as _) }
}
#[cfg(feature = "Win32_NetworkManagement_WindowsFilteringPlatform")]
#[inline]
pub unsafe fn FwpmLayerGetByKey0(enginehandle: super::super::super::Win32::Foundation::HANDLE, key: *const windows_core::GUID, layer: *mut *mut super::super::super::Win32::NetworkManagement::WindowsFilteringPlatform::FWPM_LAYER0) -> super::super::super::Win32::Foundation::NTSTATUS {
    windows_core::link!("fwpuclnt.dll" "system" fn FwpmLayerGetByKey0(enginehandle : super::super::super::Win32::Foundation:: HANDLE, key : *const windows_core::GUID, layer : *mut *mut super::super::super::Win32::NetworkManagement::WindowsFilteringPlatform:: FWPM_LAYER0) -> super::super::super::Win32::Foundation:: NTSTATUS);
    unsafe { FwpmLayerGetByKey0(enginehandle, key, layer as _) }
}
#[cfg(feature = "Win32_Security")]
#[inline]
pub unsafe fn FwpmLayerGetSecurityInfoByKey0(enginehandle: super::super::super::Win32::Foundation::HANDLE, key: Option<*const windows_core::GUID>, securityinfo: u32, sidowner: *mut super::super::super::Win32::Security::PSID, sidgroup: *mut super::super::super::Win32::Security::PSID, dacl: *mut *mut super::super::super::Win32::Security::ACL, sacl: *mut *mut super::super::super::Win32::Security::ACL, securitydescriptor: *mut super::super::super::Win32::Security::PSECURITY_DESCRIPTOR) -> super::super::super::Win32::Foundation::NTSTATUS {
    windows_core::link!("fwpuclnt.dll" "system" fn FwpmLayerGetSecurityInfoByKey0(enginehandle : super::super::super::Win32::Foundation:: HANDLE, key : *const windows_core::GUID, securityinfo : u32, sidowner : *mut super::super::super::Win32::Security:: PSID, sidgroup : *mut super::super::super::Win32::Security:: PSID, dacl : *mut *mut super::super::super::Win32::Security:: ACL, sacl : *mut *mut super::super::super::Win32::Security:: ACL, securitydescriptor : *mut super::super::super::Win32::Security:: PSECURITY_DESCRIPTOR) -> super::super::super::Win32::Foundation:: NTSTATUS);
    unsafe { FwpmLayerGetSecurityInfoByKey0(enginehandle, key.unwrap_or(core::mem::zeroed()) as _, securityinfo, sidowner as _, sidgroup as _, dacl as _, sacl as _, securitydescriptor as _) }
}
#[cfg(feature = "Win32_Security")]
#[inline]
pub unsafe fn FwpmLayerSetSecurityInfoByKey0(enginehandle: super::super::super::Win32::Foundation::HANDLE, key: Option<*const windows_core::GUID>, securityinfo: u32, sidowner: Option<*const super::super::super::Win32::Security::SID>, sidgroup: Option<*const super::super::super::Win32::Security::SID>, dacl: Option<*const super::super::super::Win32::Security::ACL>, sacl: Option<*const super::super::super::Win32::Security::ACL>) -> super::super::super::Win32::Foundation::NTSTATUS {
    windows_core::link!("fwpuclnt.dll" "system" fn FwpmLayerSetSecurityInfoByKey0(enginehandle : super::super::super::Win32::Foundation:: HANDLE, key : *const windows_core::GUID, securityinfo : u32, sidowner : *const super::super::super::Win32::Security:: SID, sidgroup : *const super::super::super::Win32::Security:: SID, dacl : *const super::super::super::Win32::Security:: ACL, sacl : *const super::super::super::Win32::Security:: ACL) -> super::super::super::Win32::Foundation:: NTSTATUS);
    unsafe { FwpmLayerSetSecurityInfoByKey0(enginehandle, key.unwrap_or(core::mem::zeroed()) as _, securityinfo, sidowner.unwrap_or(core::mem::zeroed()) as _, sidgroup.unwrap_or(core::mem::zeroed()) as _, dacl.unwrap_or(core::mem::zeroed()) as _, sacl.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "Win32_NetworkManagement_WindowsFilteringPlatform", feature = "Win32_Security"))]
#[inline]
pub unsafe fn FwpmNetEventCreateEnumHandle0(enginehandle: super::super::super::Win32::Foundation::HANDLE, enumtemplate: Option<*const super::super::super::Win32::NetworkManagement::WindowsFilteringPlatform::FWPM_NET_EVENT_ENUM_TEMPLATE0>, enumhandle: *mut super::super::super::Win32::Foundation::HANDLE) -> super::super::super::Win32::Foundation::NTSTATUS {
    windows_core::link!("fwpuclnt.dll" "system" fn FwpmNetEventCreateEnumHandle0(enginehandle : super::super::super::Win32::Foundation:: HANDLE, enumtemplate : *const super::super::super::Win32::NetworkManagement::WindowsFilteringPlatform:: FWPM_NET_EVENT_ENUM_TEMPLATE0, enumhandle : *mut super::super::super::Win32::Foundation:: HANDLE) -> super::super::super::Win32::Foundation:: NTSTATUS);
    unsafe { FwpmNetEventCreateEnumHandle0(enginehandle, enumtemplate.unwrap_or(core::mem::zeroed()) as _, enumhandle as _) }
}
#[inline]
pub unsafe fn FwpmNetEventDestroyEnumHandle0(enginehandle: super::super::super::Win32::Foundation::HANDLE, enumhandle: super::super::super::Win32::Foundation::HANDLE) -> super::super::super::Win32::Foundation::NTSTATUS {
    windows_core::link!("fwpuclnt.dll" "system" fn FwpmNetEventDestroyEnumHandle0(enginehandle : super::super::super::Win32::Foundation:: HANDLE, enumhandle : super::super::super::Win32::Foundation:: HANDLE) -> super::super::super::Win32::Foundation:: NTSTATUS);
    unsafe { FwpmNetEventDestroyEnumHandle0(enginehandle, enumhandle as _) }
}
#[cfg(all(feature = "Win32_NetworkManagement_WindowsFilteringPlatform", feature = "Win32_Security"))]
#[inline]
pub unsafe fn FwpmNetEventEnum0(enginehandle: super::super::super::Win32::Foundation::HANDLE, enumhandle: super::super::super::Win32::Foundation::HANDLE, numentriesrequested: u32, entries: *mut *mut *mut super::super::super::Win32::NetworkManagement::WindowsFilteringPlatform::FWPM_NET_EVENT0, numentriesreturned: *mut u32) -> super::super::super::Win32::Foundation::NTSTATUS {
    windows_core::link!("fwpuclnt.dll" "system" fn FwpmNetEventEnum0(enginehandle : super::super::super::Win32::Foundation:: HANDLE, enumhandle : super::super::super::Win32::Foundation:: HANDLE, numentriesrequested : u32, entries : *mut *mut *mut super::super::super::Win32::NetworkManagement::WindowsFilteringPlatform:: FWPM_NET_EVENT0, numentriesreturned : *mut u32) -> super::super::super::Win32::Foundation:: NTSTATUS);
    unsafe { FwpmNetEventEnum0(enginehandle, enumhandle, numentriesrequested, entries as _, numentriesreturned as _) }
}
#[cfg(all(feature = "Win32_NetworkManagement_WindowsFilteringPlatform", feature = "Win32_Security"))]
#[inline]
pub unsafe fn FwpmNetEventEnum1(enginehandle: super::super::super::Win32::Foundation::HANDLE, enumhandle: super::super::super::Win32::Foundation::HANDLE, numentriesrequested: u32, entries: *mut *mut *mut super::super::super::Win32::NetworkManagement::WindowsFilteringPlatform::FWPM_NET_EVENT1, numentriesreturned: *mut u32) -> super::super::super::Win32::Foundation::NTSTATUS {
    windows_core::link!("fwpuclnt.dll" "system" fn FwpmNetEventEnum1(enginehandle : super::super::super::Win32::Foundation:: HANDLE, enumhandle : super::super::super::Win32::Foundation:: HANDLE, numentriesrequested : u32, entries : *mut *mut *mut super::super::super::Win32::NetworkManagement::WindowsFilteringPlatform:: FWPM_NET_EVENT1, numentriesreturned : *mut u32) -> super::super::super::Win32::Foundation:: NTSTATUS);
    unsafe { FwpmNetEventEnum1(enginehandle, enumhandle, numentriesrequested, entries as _, numentriesreturned as _) }
}
#[cfg(all(feature = "Win32_NetworkManagement_WindowsFilteringPlatform", feature = "Win32_Security"))]
#[inline]
pub unsafe fn FwpmNetEventEnum2(enginehandle: super::super::super::Win32::Foundation::HANDLE, enumhandle: super::super::super::Win32::Foundation::HANDLE, numentriesrequested: u32, entries: *mut *mut *mut super::super::super::Win32::NetworkManagement::WindowsFilteringPlatform::FWPM_NET_EVENT2, numentriesreturned: *mut u32) -> super::super::super::Win32::Foundation::NTSTATUS {
    windows_core::link!("fwpuclnt.dll" "system" fn FwpmNetEventEnum2(enginehandle : super::super::super::Win32::Foundation:: HANDLE, enumhandle : super::super::super::Win32::Foundation:: HANDLE, numentriesrequested : u32, entries : *mut *mut *mut super::super::super::Win32::NetworkManagement::WindowsFilteringPlatform:: FWPM_NET_EVENT2, numentriesreturned : *mut u32) -> super::super::super::Win32::Foundation:: NTSTATUS);
    unsafe { FwpmNetEventEnum2(enginehandle, enumhandle, numentriesrequested, entries as _, numentriesreturned as _) }
}
#[cfg(all(feature = "Win32_NetworkManagement_WindowsFilteringPlatform", feature = "Win32_Security"))]
#[inline]
pub unsafe fn FwpmNetEventEnum3(enginehandle: super::super::super::Win32::Foundation::HANDLE, enumhandle: super::super::super::Win32::Foundation::HANDLE, numentriesrequested: u32, entries: *mut *mut *mut super::super::super::Win32::NetworkManagement::WindowsFilteringPlatform::FWPM_NET_EVENT3, numentriesreturned: *mut u32) -> super::super::super::Win32::Foundation::NTSTATUS {
    windows_core::link!("fwpuclnt.dll" "system" fn FwpmNetEventEnum3(enginehandle : super::super::super::Win32::Foundation:: HANDLE, enumhandle : super::super::super::Win32::Foundation:: HANDLE, numentriesrequested : u32, entries : *mut *mut *mut super::super::super::Win32::NetworkManagement::WindowsFilteringPlatform:: FWPM_NET_EVENT3, numentriesreturned : *mut u32) -> super::super::super::Win32::Foundation:: NTSTATUS);
    unsafe { FwpmNetEventEnum3(enginehandle, enumhandle, numentriesrequested, entries as _, numentriesreturned as _) }
}
#[cfg(all(feature = "Win32_NetworkManagement_WindowsFilteringPlatform", feature = "Win32_Security"))]
#[inline]
pub unsafe fn FwpmNetEventEnum4(enginehandle: super::super::super::Win32::Foundation::HANDLE, enumhandle: super::super::super::Win32::Foundation::HANDLE, numentriesrequested: u32, entries: *mut *mut *mut super::super::super::Win32::NetworkManagement::WindowsFilteringPlatform::FWPM_NET_EVENT4, numentriesreturned: *mut u32) -> super::super::super::Win32::Foundation::NTSTATUS {
    windows_core::link!("fwpuclnt.dll" "system" fn FwpmNetEventEnum4(enginehandle : super::super::super::Win32::Foundation:: HANDLE, enumhandle : super::super::super::Win32::Foundation:: HANDLE, numentriesrequested : u32, entries : *mut *mut *mut super::super::super::Win32::NetworkManagement::WindowsFilteringPlatform:: FWPM_NET_EVENT4, numentriesreturned : *mut u32) -> super::super::super::Win32::Foundation:: NTSTATUS);
    unsafe { FwpmNetEventEnum4(enginehandle, enumhandle, numentriesrequested, entries as _, numentriesreturned as _) }
}
#[cfg(all(feature = "Win32_NetworkManagement_WindowsFilteringPlatform", feature = "Win32_Security"))]
#[inline]
pub unsafe fn FwpmNetEventEnum5(enginehandle: super::super::super::Win32::Foundation::HANDLE, enumhandle: super::super::super::Win32::Foundation::HANDLE, numentriesrequested: u32, entries: *mut *mut *mut super::super::super::Win32::NetworkManagement::WindowsFilteringPlatform::FWPM_NET_EVENT5, numentriesreturned: *mut u32) -> super::super::super::Win32::Foundation::NTSTATUS {
    windows_core::link!("fwpuclnt.dll" "system" fn FwpmNetEventEnum5(enginehandle : super::super::super::Win32::Foundation:: HANDLE, enumhandle : super::super::super::Win32::Foundation:: HANDLE, numentriesrequested : u32, entries : *mut *mut *mut super::super::super::Win32::NetworkManagement::WindowsFilteringPlatform:: FWPM_NET_EVENT5, numentriesreturned : *mut u32) -> super::super::super::Win32::Foundation:: NTSTATUS);
    unsafe { FwpmNetEventEnum5(enginehandle, enumhandle, numentriesrequested, entries as _, numentriesreturned as _) }
}
#[cfg(feature = "Win32_Security")]
#[inline]
pub unsafe fn FwpmNetEventsGetSecurityInfo0(enginehandle: super::super::super::Win32::Foundation::HANDLE, securityinfo: u32, sidowner: *mut super::super::super::Win32::Security::PSID, sidgroup: *mut super::super::super::Win32::Security::PSID, dacl: *mut *mut super::super::super::Win32::Security::ACL, sacl: *mut *mut super::super::super::Win32::Security::ACL, securitydescriptor: *mut super::super::super::Win32::Security::PSECURITY_DESCRIPTOR) -> super::super::super::Win32::Foundation::NTSTATUS {
    windows_core::link!("fwpuclnt.dll" "system" fn FwpmNetEventsGetSecurityInfo0(enginehandle : super::super::super::Win32::Foundation:: HANDLE, securityinfo : u32, sidowner : *mut super::super::super::Win32::Security:: PSID, sidgroup : *mut super::super::super::Win32::Security:: PSID, dacl : *mut *mut super::super::super::Win32::Security:: ACL, sacl : *mut *mut super::super::super::Win32::Security:: ACL, securitydescriptor : *mut super::super::super::Win32::Security:: PSECURITY_DESCRIPTOR) -> super::super::super::Win32::Foundation:: NTSTATUS);
    unsafe { FwpmNetEventsGetSecurityInfo0(enginehandle, securityinfo, sidowner as _, sidgroup as _, dacl as _, sacl as _, securitydescriptor as _) }
}
#[cfg(feature = "Win32_Security")]
#[inline]
pub unsafe fn FwpmNetEventsSetSecurityInfo0(enginehandle: super::super::super::Win32::Foundation::HANDLE, securityinfo: u32, sidowner: Option<*const super::super::super::Win32::Security::SID>, sidgroup: Option<*const super::super::super::Win32::Security::SID>, dacl: Option<*const super::super::super::Win32::Security::ACL>, sacl: Option<*const super::super::super::Win32::Security::ACL>) -> super::super::super::Win32::Foundation::NTSTATUS {
    windows_core::link!("fwpuclnt.dll" "system" fn FwpmNetEventsSetSecurityInfo0(enginehandle : super::super::super::Win32::Foundation:: HANDLE, securityinfo : u32, sidowner : *const super::super::super::Win32::Security:: SID, sidgroup : *const super::super::super::Win32::Security:: SID, dacl : *const super::super::super::Win32::Security:: ACL, sacl : *const super::super::super::Win32::Security:: ACL) -> super::super::super::Win32::Foundation:: NTSTATUS);
    unsafe { FwpmNetEventsSetSecurityInfo0(enginehandle, securityinfo, sidowner.unwrap_or(core::mem::zeroed()) as _, sidgroup.unwrap_or(core::mem::zeroed()) as _, dacl.unwrap_or(core::mem::zeroed()) as _, sacl.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "Win32_NetworkManagement_WindowsFilteringPlatform", feature = "Win32_Security"))]
#[inline]
pub unsafe fn FwpmProviderAdd0(enginehandle: super::super::super::Win32::Foundation::HANDLE, provider: *const super::super::super::Win32::NetworkManagement::WindowsFilteringPlatform::FWPM_PROVIDER0, sd: Option<super::super::super::Win32::Security::PSECURITY_DESCRIPTOR>) -> super::super::super::Win32::Foundation::NTSTATUS {
    windows_core::link!("fwpuclnt.dll" "system" fn FwpmProviderAdd0(enginehandle : super::super::super::Win32::Foundation:: HANDLE, provider : *const super::super::super::Win32::NetworkManagement::WindowsFilteringPlatform:: FWPM_PROVIDER0, sd : super::super::super::Win32::Security:: PSECURITY_DESCRIPTOR) -> super::super::super::Win32::Foundation:: NTSTATUS);
    unsafe { FwpmProviderAdd0(enginehandle, provider, sd.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "Win32_NetworkManagement_WindowsFilteringPlatform", feature = "Win32_Security"))]
#[inline]
pub unsafe fn FwpmProviderContextAdd0(enginehandle: super::super::super::Win32::Foundation::HANDLE, providercontext: *const super::super::super::Win32::NetworkManagement::WindowsFilteringPlatform::FWPM_PROVIDER_CONTEXT0, sd: Option<super::super::super::Win32::Security::PSECURITY_DESCRIPTOR>, id: Option<*mut u64>) -> super::super::super::Win32::Foundation::NTSTATUS {
    windows_core::link!("fwpuclnt.dll" "system" fn FwpmProviderContextAdd0(enginehandle : super::super::super::Win32::Foundation:: HANDLE, providercontext : *const super::super::super::Win32::NetworkManagement::WindowsFilteringPlatform:: FWPM_PROVIDER_CONTEXT0, sd : super::super::super::Win32::Security:: PSECURITY_DESCRIPTOR, id : *mut u64) -> super::super::super::Win32::Foundation:: NTSTATUS);
    unsafe { FwpmProviderContextAdd0(enginehandle, providercontext, sd.unwrap_or(core::mem::zeroed()) as _, id.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "Win32_NetworkManagement_WindowsFilteringPlatform", feature = "Win32_Security"))]
#[inline]
pub unsafe fn FwpmProviderContextAdd1(enginehandle: super::super::super::Win32::Foundation::HANDLE, providercontext: *const super::super::super::Win32::NetworkManagement::WindowsFilteringPlatform::FWPM_PROVIDER_CONTEXT1, sd: Option<super::super::super::Win32::Security::PSECURITY_DESCRIPTOR>, id: Option<*mut u64>) -> super::super::super::Win32::Foundation::NTSTATUS {
    windows_core::link!("fwpuclnt.dll" "system" fn FwpmProviderContextAdd1(enginehandle : super::super::super::Win32::Foundation:: HANDLE, providercontext : *const super::super::super::Win32::NetworkManagement::WindowsFilteringPlatform:: FWPM_PROVIDER_CONTEXT1, sd : super::super::super::Win32::Security:: PSECURITY_DESCRIPTOR, id : *mut u64) -> super::super::super::Win32::Foundation:: NTSTATUS);
    unsafe { FwpmProviderContextAdd1(enginehandle, providercontext, sd.unwrap_or(core::mem::zeroed()) as _, id.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "Win32_NetworkManagement_WindowsFilteringPlatform", feature = "Win32_Security"))]
#[inline]
pub unsafe fn FwpmProviderContextAdd2(enginehandle: super::super::super::Win32::Foundation::HANDLE, providercontext: *const super::super::super::Win32::NetworkManagement::WindowsFilteringPlatform::FWPM_PROVIDER_CONTEXT2, sd: Option<super::super::super::Win32::Security::PSECURITY_DESCRIPTOR>, id: Option<*mut u64>) -> super::super::super::Win32::Foundation::NTSTATUS {
    windows_core::link!("fwpuclnt.dll" "system" fn FwpmProviderContextAdd2(enginehandle : super::super::super::Win32::Foundation:: HANDLE, providercontext : *const super::super::super::Win32::NetworkManagement::WindowsFilteringPlatform:: FWPM_PROVIDER_CONTEXT2, sd : super::super::super::Win32::Security:: PSECURITY_DESCRIPTOR, id : *mut u64) -> super::super::super::Win32::Foundation:: NTSTATUS);
    unsafe { FwpmProviderContextAdd2(enginehandle, providercontext, sd.unwrap_or(core::mem::zeroed()) as _, id.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "Win32_NetworkManagement_WindowsFilteringPlatform", feature = "Win32_Security"))]
#[inline]
pub unsafe fn FwpmProviderContextAdd3(enginehandle: super::super::super::Win32::Foundation::HANDLE, providercontext: *const super::super::super::Win32::NetworkManagement::WindowsFilteringPlatform::FWPM_PROVIDER_CONTEXT3, sd: Option<super::super::super::Win32::Security::PSECURITY_DESCRIPTOR>, id: Option<*mut u64>) -> super::super::super::Win32::Foundation::NTSTATUS {
    windows_core::link!("fwpuclnt.dll" "system" fn FwpmProviderContextAdd3(enginehandle : super::super::super::Win32::Foundation:: HANDLE, providercontext : *const super::super::super::Win32::NetworkManagement::WindowsFilteringPlatform:: FWPM_PROVIDER_CONTEXT3, sd : super::super::super::Win32::Security:: PSECURITY_DESCRIPTOR, id : *mut u64) -> super::super::super::Win32::Foundation:: NTSTATUS);
    unsafe { FwpmProviderContextAdd3(enginehandle, providercontext, sd.unwrap_or(core::mem::zeroed()) as _, id.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "Win32_NetworkManagement_WindowsFilteringPlatform")]
#[inline]
pub unsafe fn FwpmProviderContextCreateEnumHandle0(enginehandle: super::super::super::Win32::Foundation::HANDLE, enumtemplate: Option<*const super::super::super::Win32::NetworkManagement::WindowsFilteringPlatform::FWPM_PROVIDER_CONTEXT_ENUM_TEMPLATE0>, enumhandle: *mut super::super::super::Win32::Foundation::HANDLE) -> super::super::super::Win32::Foundation::NTSTATUS {
    windows_core::link!("fwpuclnt.dll" "system" fn FwpmProviderContextCreateEnumHandle0(enginehandle : super::super::super::Win32::Foundation:: HANDLE, enumtemplate : *const super::super::super::Win32::NetworkManagement::WindowsFilteringPlatform:: FWPM_PROVIDER_CONTEXT_ENUM_TEMPLATE0, enumhandle : *mut super::super::super::Win32::Foundation:: HANDLE) -> super::super::super::Win32::Foundation:: NTSTATUS);
    unsafe { FwpmProviderContextCreateEnumHandle0(enginehandle, enumtemplate.unwrap_or(core::mem::zeroed()) as _, enumhandle as _) }
}
#[inline]
pub unsafe fn FwpmProviderContextDeleteById0(enginehandle: super::super::super::Win32::Foundation::HANDLE, id: u64) -> super::super::super::Win32::Foundation::NTSTATUS {
    windows_core::link!("fwpuclnt.dll" "system" fn FwpmProviderContextDeleteById0(enginehandle : super::super::super::Win32::Foundation:: HANDLE, id : u64) -> super::super::super::Win32::Foundation:: NTSTATUS);
    unsafe { FwpmProviderContextDeleteById0(enginehandle, id) }
}
#[inline]
pub unsafe fn FwpmProviderContextDeleteByKey0(enginehandle: super::super::super::Win32::Foundation::HANDLE, key: *const windows_core::GUID) -> super::super::super::Win32::Foundation::NTSTATUS {
    windows_core::link!("fwpuclnt.dll" "system" fn FwpmProviderContextDeleteByKey0(enginehandle : super::super::super::Win32::Foundation:: HANDLE, key : *const windows_core::GUID) -> super::super::super::Win32::Foundation:: NTSTATUS);
    unsafe { FwpmProviderContextDeleteByKey0(enginehandle, key) }
}
#[inline]
pub unsafe fn FwpmProviderContextDestroyEnumHandle0(enginehandle: super::super::super::Win32::Foundation::HANDLE, enumhandle: super::super::super::Win32::Foundation::HANDLE) -> super::super::super::Win32::Foundation::NTSTATUS {
    windows_core::link!("fwpuclnt.dll" "system" fn FwpmProviderContextDestroyEnumHandle0(enginehandle : super::super::super::Win32::Foundation:: HANDLE, enumhandle : super::super::super::Win32::Foundation:: HANDLE) -> super::super::super::Win32::Foundation:: NTSTATUS);
    unsafe { FwpmProviderContextDestroyEnumHandle0(enginehandle, enumhandle as _) }
}
#[cfg(all(feature = "Win32_NetworkManagement_WindowsFilteringPlatform", feature = "Win32_Security"))]
#[inline]
pub unsafe fn FwpmProviderContextEnum0(enginehandle: super::super::super::Win32::Foundation::HANDLE, enumhandle: super::super::super::Win32::Foundation::HANDLE, numentriesrequested: u32, entries: *mut *mut *mut super::super::super::Win32::NetworkManagement::WindowsFilteringPlatform::FWPM_PROVIDER_CONTEXT0, numentriesreturned: *mut u32) -> super::super::super::Win32::Foundation::NTSTATUS {
    windows_core::link!("fwpuclnt.dll" "system" fn FwpmProviderContextEnum0(enginehandle : super::super::super::Win32::Foundation:: HANDLE, enumhandle : super::super::super::Win32::Foundation:: HANDLE, numentriesrequested : u32, entries : *mut *mut *mut super::super::super::Win32::NetworkManagement::WindowsFilteringPlatform:: FWPM_PROVIDER_CONTEXT0, numentriesreturned : *mut u32) -> super::super::super::Win32::Foundation:: NTSTATUS);
    unsafe { FwpmProviderContextEnum0(enginehandle, enumhandle, numentriesrequested, entries as _, numentriesreturned as _) }
}
#[cfg(all(feature = "Win32_NetworkManagement_WindowsFilteringPlatform", feature = "Win32_Security"))]
#[inline]
pub unsafe fn FwpmProviderContextEnum1(enginehandle: super::super::super::Win32::Foundation::HANDLE, enumhandle: super::super::super::Win32::Foundation::HANDLE, numentriesrequested: u32, entries: *mut *mut *mut super::super::super::Win32::NetworkManagement::WindowsFilteringPlatform::FWPM_PROVIDER_CONTEXT1, numentriesreturned: *mut u32) -> super::super::super::Win32::Foundation::NTSTATUS {
    windows_core::link!("fwpuclnt.dll" "system" fn FwpmProviderContextEnum1(enginehandle : super::super::super::Win32::Foundation:: HANDLE, enumhandle : super::super::super::Win32::Foundation:: HANDLE, numentriesrequested : u32, entries : *mut *mut *mut super::super::super::Win32::NetworkManagement::WindowsFilteringPlatform:: FWPM_PROVIDER_CONTEXT1, numentriesreturned : *mut u32) -> super::super::super::Win32::Foundation:: NTSTATUS);
    unsafe { FwpmProviderContextEnum1(enginehandle, enumhandle, numentriesrequested, entries as _, numentriesreturned as _) }
}
#[cfg(all(feature = "Win32_NetworkManagement_WindowsFilteringPlatform", feature = "Win32_Security"))]
#[inline]
pub unsafe fn FwpmProviderContextEnum2(enginehandle: super::super::super::Win32::Foundation::HANDLE, enumhandle: super::super::super::Win32::Foundation::HANDLE, numentriesrequested: u32, entries: *mut *mut *mut super::super::super::Win32::NetworkManagement::WindowsFilteringPlatform::FWPM_PROVIDER_CONTEXT2, numentriesreturned: *mut u32) -> super::super::super::Win32::Foundation::NTSTATUS {
    windows_core::link!("fwpuclnt.dll" "system" fn FwpmProviderContextEnum2(enginehandle : super::super::super::Win32::Foundation:: HANDLE, enumhandle : super::super::super::Win32::Foundation:: HANDLE, numentriesrequested : u32, entries : *mut *mut *mut super::super::super::Win32::NetworkManagement::WindowsFilteringPlatform:: FWPM_PROVIDER_CONTEXT2, numentriesreturned : *mut u32) -> super::super::super::Win32::Foundation:: NTSTATUS);
    unsafe { FwpmProviderContextEnum2(enginehandle, enumhandle, numentriesrequested, entries as _, numentriesreturned as _) }
}
#[cfg(all(feature = "Win32_NetworkManagement_WindowsFilteringPlatform", feature = "Win32_Security"))]
#[inline]
pub unsafe fn FwpmProviderContextEnum3(enginehandle: super::super::super::Win32::Foundation::HANDLE, enumhandle: super::super::super::Win32::Foundation::HANDLE, numentriesrequested: u32, entries: *mut *mut *mut super::super::super::Win32::NetworkManagement::WindowsFilteringPlatform::FWPM_PROVIDER_CONTEXT3, numentriesreturned: *mut u32) -> super::super::super::Win32::Foundation::NTSTATUS {
    windows_core::link!("fwpuclnt.dll" "system" fn FwpmProviderContextEnum3(enginehandle : super::super::super::Win32::Foundation:: HANDLE, enumhandle : super::super::super::Win32::Foundation:: HANDLE, numentriesrequested : u32, entries : *mut *mut *mut super::super::super::Win32::NetworkManagement::WindowsFilteringPlatform:: FWPM_PROVIDER_CONTEXT3, numentriesreturned : *mut u32) -> super::super::super::Win32::Foundation:: NTSTATUS);
    unsafe { FwpmProviderContextEnum3(enginehandle, enumhandle, numentriesrequested, entries as _, numentriesreturned as _) }
}
#[cfg(all(feature = "Win32_NetworkManagement_WindowsFilteringPlatform", feature = "Win32_Security"))]
#[inline]
pub unsafe fn FwpmProviderContextGetById0(enginehandle: super::super::super::Win32::Foundation::HANDLE, id: u64, providercontext: *mut *mut super::super::super::Win32::NetworkManagement::WindowsFilteringPlatform::FWPM_PROVIDER_CONTEXT0) -> super::super::super::Win32::Foundation::NTSTATUS {
    windows_core::link!("fwpuclnt.dll" "system" fn FwpmProviderContextGetById0(enginehandle : super::super::super::Win32::Foundation:: HANDLE, id : u64, providercontext : *mut *mut super::super::super::Win32::NetworkManagement::WindowsFilteringPlatform:: FWPM_PROVIDER_CONTEXT0) -> super::super::super::Win32::Foundation:: NTSTATUS);
    unsafe { FwpmProviderContextGetById0(enginehandle, id, providercontext as _) }
}
#[cfg(all(feature = "Win32_NetworkManagement_WindowsFilteringPlatform", feature = "Win32_Security"))]
#[inline]
pub unsafe fn FwpmProviderContextGetById1(enginehandle: super::super::super::Win32::Foundation::HANDLE, id: u64, providercontext: *mut *mut super::super::super::Win32::NetworkManagement::WindowsFilteringPlatform::FWPM_PROVIDER_CONTEXT1) -> super::super::super::Win32::Foundation::NTSTATUS {
    windows_core::link!("fwpuclnt.dll" "system" fn FwpmProviderContextGetById1(enginehandle : super::super::super::Win32::Foundation:: HANDLE, id : u64, providercontext : *mut *mut super::super::super::Win32::NetworkManagement::WindowsFilteringPlatform:: FWPM_PROVIDER_CONTEXT1) -> super::super::super::Win32::Foundation:: NTSTATUS);
    unsafe { FwpmProviderContextGetById1(enginehandle, id, providercontext as _) }
}
#[cfg(all(feature = "Win32_NetworkManagement_WindowsFilteringPlatform", feature = "Win32_Security"))]
#[inline]
pub unsafe fn FwpmProviderContextGetById2(enginehandle: super::super::super::Win32::Foundation::HANDLE, id: u64, providercontext: *mut *mut super::super::super::Win32::NetworkManagement::WindowsFilteringPlatform::FWPM_PROVIDER_CONTEXT2) -> super::super::super::Win32::Foundation::NTSTATUS {
    windows_core::link!("fwpuclnt.dll" "system" fn FwpmProviderContextGetById2(enginehandle : super::super::super::Win32::Foundation:: HANDLE, id : u64, providercontext : *mut *mut super::super::super::Win32::NetworkManagement::WindowsFilteringPlatform:: FWPM_PROVIDER_CONTEXT2) -> super::super::super::Win32::Foundation:: NTSTATUS);
    unsafe { FwpmProviderContextGetById2(enginehandle, id, providercontext as _) }
}
#[cfg(all(feature = "Win32_NetworkManagement_WindowsFilteringPlatform", feature = "Win32_Security"))]
#[inline]
pub unsafe fn FwpmProviderContextGetById3(enginehandle: super::super::super::Win32::Foundation::HANDLE, id: u64, providercontext: *mut *mut super::super::super::Win32::NetworkManagement::WindowsFilteringPlatform::FWPM_PROVIDER_CONTEXT3) -> super::super::super::Win32::Foundation::NTSTATUS {
    windows_core::link!("fwpuclnt.dll" "system" fn FwpmProviderContextGetById3(enginehandle : super::super::super::Win32::Foundation:: HANDLE, id : u64, providercontext : *mut *mut super::super::super::Win32::NetworkManagement::WindowsFilteringPlatform:: FWPM_PROVIDER_CONTEXT3) -> super::super::super::Win32::Foundation:: NTSTATUS);
    unsafe { FwpmProviderContextGetById3(enginehandle, id, providercontext as _) }
}
#[cfg(all(feature = "Win32_NetworkManagement_WindowsFilteringPlatform", feature = "Win32_Security"))]
#[inline]
pub unsafe fn FwpmProviderContextGetByKey0(enginehandle: super::super::super::Win32::Foundation::HANDLE, key: *const windows_core::GUID, providercontext: *mut *mut super::super::super::Win32::NetworkManagement::WindowsFilteringPlatform::FWPM_PROVIDER_CONTEXT0) -> super::super::super::Win32::Foundation::NTSTATUS {
    windows_core::link!("fwpuclnt.dll" "system" fn FwpmProviderContextGetByKey0(enginehandle : super::super::super::Win32::Foundation:: HANDLE, key : *const windows_core::GUID, providercontext : *mut *mut super::super::super::Win32::NetworkManagement::WindowsFilteringPlatform:: FWPM_PROVIDER_CONTEXT0) -> super::super::super::Win32::Foundation:: NTSTATUS);
    unsafe { FwpmProviderContextGetByKey0(enginehandle, key, providercontext as _) }
}
#[cfg(all(feature = "Win32_NetworkManagement_WindowsFilteringPlatform", feature = "Win32_Security"))]
#[inline]
pub unsafe fn FwpmProviderContextGetByKey1(enginehandle: super::super::super::Win32::Foundation::HANDLE, key: *const windows_core::GUID, providercontext: *mut *mut super::super::super::Win32::NetworkManagement::WindowsFilteringPlatform::FWPM_PROVIDER_CONTEXT1) -> super::super::super::Win32::Foundation::NTSTATUS {
    windows_core::link!("fwpuclnt.dll" "system" fn FwpmProviderContextGetByKey1(enginehandle : super::super::super::Win32::Foundation:: HANDLE, key : *const windows_core::GUID, providercontext : *mut *mut super::super::super::Win32::NetworkManagement::WindowsFilteringPlatform:: FWPM_PROVIDER_CONTEXT1) -> super::super::super::Win32::Foundation:: NTSTATUS);
    unsafe { FwpmProviderContextGetByKey1(enginehandle, key, providercontext as _) }
}
#[cfg(all(feature = "Win32_NetworkManagement_WindowsFilteringPlatform", feature = "Win32_Security"))]
#[inline]
pub unsafe fn FwpmProviderContextGetByKey2(enginehandle: super::super::super::Win32::Foundation::HANDLE, key: *const windows_core::GUID, providercontext: *mut *mut super::super::super::Win32::NetworkManagement::WindowsFilteringPlatform::FWPM_PROVIDER_CONTEXT2) -> super::super::super::Win32::Foundation::NTSTATUS {
    windows_core::link!("fwpuclnt.dll" "system" fn FwpmProviderContextGetByKey2(enginehandle : super::super::super::Win32::Foundation:: HANDLE, key : *const windows_core::GUID, providercontext : *mut *mut super::super::super::Win32::NetworkManagement::WindowsFilteringPlatform:: FWPM_PROVIDER_CONTEXT2) -> super::super::super::Win32::Foundation:: NTSTATUS);
    unsafe { FwpmProviderContextGetByKey2(enginehandle, key, providercontext as _) }
}
#[cfg(all(feature = "Win32_NetworkManagement_WindowsFilteringPlatform", feature = "Win32_Security"))]
#[inline]
pub unsafe fn FwpmProviderContextGetByKey3(enginehandle: super::super::super::Win32::Foundation::HANDLE, key: *const windows_core::GUID, providercontext: *mut *mut super::super::super::Win32::NetworkManagement::WindowsFilteringPlatform::FWPM_PROVIDER_CONTEXT3) -> super::super::super::Win32::Foundation::NTSTATUS {
    windows_core::link!("fwpuclnt.dll" "system" fn FwpmProviderContextGetByKey3(enginehandle : super::super::super::Win32::Foundation:: HANDLE, key : *const windows_core::GUID, providercontext : *mut *mut super::super::super::Win32::NetworkManagement::WindowsFilteringPlatform:: FWPM_PROVIDER_CONTEXT3) -> super::super::super::Win32::Foundation:: NTSTATUS);
    unsafe { FwpmProviderContextGetByKey3(enginehandle, key, providercontext as _) }
}
#[cfg(feature = "Win32_Security")]
#[inline]
pub unsafe fn FwpmProviderContextGetSecurityInfoByKey0(enginehandle: super::super::super::Win32::Foundation::HANDLE, key: Option<*const windows_core::GUID>, securityinfo: u32, sidowner: *mut super::super::super::Win32::Security::PSID, sidgroup: *mut super::super::super::Win32::Security::PSID, dacl: *mut *mut super::super::super::Win32::Security::ACL, sacl: *mut *mut super::super::super::Win32::Security::ACL, securitydescriptor: *mut super::super::super::Win32::Security::PSECURITY_DESCRIPTOR) -> super::super::super::Win32::Foundation::NTSTATUS {
    windows_core::link!("fwpuclnt.dll" "system" fn FwpmProviderContextGetSecurityInfoByKey0(enginehandle : super::super::super::Win32::Foundation:: HANDLE, key : *const windows_core::GUID, securityinfo : u32, sidowner : *mut super::super::super::Win32::Security:: PSID, sidgroup : *mut super::super::super::Win32::Security:: PSID, dacl : *mut *mut super::super::super::Win32::Security:: ACL, sacl : *mut *mut super::super::super::Win32::Security:: ACL, securitydescriptor : *mut super::super::super::Win32::Security:: PSECURITY_DESCRIPTOR) -> super::super::super::Win32::Foundation:: NTSTATUS);
    unsafe { FwpmProviderContextGetSecurityInfoByKey0(enginehandle, key.unwrap_or(core::mem::zeroed()) as _, securityinfo, sidowner as _, sidgroup as _, dacl as _, sacl as _, securitydescriptor as _) }
}
#[cfg(feature = "Win32_Security")]
#[inline]
pub unsafe fn FwpmProviderContextSetSecurityInfoByKey0(enginehandle: super::super::super::Win32::Foundation::HANDLE, key: Option<*const windows_core::GUID>, securityinfo: u32, sidowner: Option<*const super::super::super::Win32::Security::SID>, sidgroup: Option<*const super::super::super::Win32::Security::SID>, dacl: Option<*const super::super::super::Win32::Security::ACL>, sacl: Option<*const super::super::super::Win32::Security::ACL>) -> super::super::super::Win32::Foundation::NTSTATUS {
    windows_core::link!("fwpuclnt.dll" "system" fn FwpmProviderContextSetSecurityInfoByKey0(enginehandle : super::super::super::Win32::Foundation:: HANDLE, key : *const windows_core::GUID, securityinfo : u32, sidowner : *const super::super::super::Win32::Security:: SID, sidgroup : *const super::super::super::Win32::Security:: SID, dacl : *const super::super::super::Win32::Security:: ACL, sacl : *const super::super::super::Win32::Security:: ACL) -> super::super::super::Win32::Foundation:: NTSTATUS);
    unsafe { FwpmProviderContextSetSecurityInfoByKey0(enginehandle, key.unwrap_or(core::mem::zeroed()) as _, securityinfo, sidowner.unwrap_or(core::mem::zeroed()) as _, sidgroup.unwrap_or(core::mem::zeroed()) as _, dacl.unwrap_or(core::mem::zeroed()) as _, sacl.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "Win32_NetworkManagement_WindowsFilteringPlatform")]
#[inline]
pub unsafe fn FwpmProviderCreateEnumHandle0(enginehandle: super::super::super::Win32::Foundation::HANDLE, enumtemplate: Option<*const super::super::super::Win32::NetworkManagement::WindowsFilteringPlatform::FWPM_PROVIDER_ENUM_TEMPLATE0>, enumhandle: *mut super::super::super::Win32::Foundation::HANDLE) -> super::super::super::Win32::Foundation::NTSTATUS {
    windows_core::link!("fwpuclnt.dll" "system" fn FwpmProviderCreateEnumHandle0(enginehandle : super::super::super::Win32::Foundation:: HANDLE, enumtemplate : *const super::super::super::Win32::NetworkManagement::WindowsFilteringPlatform:: FWPM_PROVIDER_ENUM_TEMPLATE0, enumhandle : *mut super::super::super::Win32::Foundation:: HANDLE) -> super::super::super::Win32::Foundation:: NTSTATUS);
    unsafe { FwpmProviderCreateEnumHandle0(enginehandle, enumtemplate.unwrap_or(core::mem::zeroed()) as _, enumhandle as _) }
}
#[inline]
pub unsafe fn FwpmProviderDeleteByKey0(enginehandle: super::super::super::Win32::Foundation::HANDLE, key: *const windows_core::GUID) -> super::super::super::Win32::Foundation::NTSTATUS {
    windows_core::link!("fwpuclnt.dll" "system" fn FwpmProviderDeleteByKey0(enginehandle : super::super::super::Win32::Foundation:: HANDLE, key : *const windows_core::GUID) -> super::super::super::Win32::Foundation:: NTSTATUS);
    unsafe { FwpmProviderDeleteByKey0(enginehandle, key) }
}
#[inline]
pub unsafe fn FwpmProviderDestroyEnumHandle0(enginehandle: super::super::super::Win32::Foundation::HANDLE, enumhandle: super::super::super::Win32::Foundation::HANDLE) -> super::super::super::Win32::Foundation::NTSTATUS {
    windows_core::link!("fwpuclnt.dll" "system" fn FwpmProviderDestroyEnumHandle0(enginehandle : super::super::super::Win32::Foundation:: HANDLE, enumhandle : super::super::super::Win32::Foundation:: HANDLE) -> super::super::super::Win32::Foundation:: NTSTATUS);
    unsafe { FwpmProviderDestroyEnumHandle0(enginehandle, enumhandle as _) }
}
#[cfg(feature = "Win32_NetworkManagement_WindowsFilteringPlatform")]
#[inline]
pub unsafe fn FwpmProviderEnum0(enginehandle: super::super::super::Win32::Foundation::HANDLE, enumhandle: super::super::super::Win32::Foundation::HANDLE, numentriesrequested: u32, entries: *mut *mut *mut super::super::super::Win32::NetworkManagement::WindowsFilteringPlatform::FWPM_PROVIDER0, numentriesreturned: *mut u32) -> super::super::super::Win32::Foundation::NTSTATUS {
    windows_core::link!("fwpuclnt.dll" "system" fn FwpmProviderEnum0(enginehandle : super::super::super::Win32::Foundation:: HANDLE, enumhandle : super::super::super::Win32::Foundation:: HANDLE, numentriesrequested : u32, entries : *mut *mut *mut super::super::super::Win32::NetworkManagement::WindowsFilteringPlatform:: FWPM_PROVIDER0, numentriesreturned : *mut u32) -> super::super::super::Win32::Foundation:: NTSTATUS);
    unsafe { FwpmProviderEnum0(enginehandle, enumhandle, numentriesrequested, entries as _, numentriesreturned as _) }
}
#[cfg(feature = "Win32_NetworkManagement_WindowsFilteringPlatform")]
#[inline]
pub unsafe fn FwpmProviderGetByKey0(enginehandle: super::super::super::Win32::Foundation::HANDLE, key: *const windows_core::GUID, provider: *mut *mut super::super::super::Win32::NetworkManagement::WindowsFilteringPlatform::FWPM_PROVIDER0) -> super::super::super::Win32::Foundation::NTSTATUS {
    windows_core::link!("fwpuclnt.dll" "system" fn FwpmProviderGetByKey0(enginehandle : super::super::super::Win32::Foundation:: HANDLE, key : *const windows_core::GUID, provider : *mut *mut super::super::super::Win32::NetworkManagement::WindowsFilteringPlatform:: FWPM_PROVIDER0) -> super::super::super::Win32::Foundation:: NTSTATUS);
    unsafe { FwpmProviderGetByKey0(enginehandle, key, provider as _) }
}
#[cfg(feature = "Win32_Security")]
#[inline]
pub unsafe fn FwpmProviderGetSecurityInfoByKey0(enginehandle: super::super::super::Win32::Foundation::HANDLE, key: Option<*const windows_core::GUID>, securityinfo: u32, sidowner: *mut super::super::super::Win32::Security::PSID, sidgroup: *mut super::super::super::Win32::Security::PSID, dacl: *mut *mut super::super::super::Win32::Security::ACL, sacl: *mut *mut super::super::super::Win32::Security::ACL, securitydescriptor: *mut super::super::super::Win32::Security::PSECURITY_DESCRIPTOR) -> super::super::super::Win32::Foundation::NTSTATUS {
    windows_core::link!("fwpuclnt.dll" "system" fn FwpmProviderGetSecurityInfoByKey0(enginehandle : super::super::super::Win32::Foundation:: HANDLE, key : *const windows_core::GUID, securityinfo : u32, sidowner : *mut super::super::super::Win32::Security:: PSID, sidgroup : *mut super::super::super::Win32::Security:: PSID, dacl : *mut *mut super::super::super::Win32::Security:: ACL, sacl : *mut *mut super::super::super::Win32::Security:: ACL, securitydescriptor : *mut super::super::super::Win32::Security:: PSECURITY_DESCRIPTOR) -> super::super::super::Win32::Foundation:: NTSTATUS);
    unsafe { FwpmProviderGetSecurityInfoByKey0(enginehandle, key.unwrap_or(core::mem::zeroed()) as _, securityinfo, sidowner as _, sidgroup as _, dacl as _, sacl as _, securitydescriptor as _) }
}
#[cfg(feature = "Win32_Security")]
#[inline]
pub unsafe fn FwpmProviderSetSecurityInfoByKey0(enginehandle: super::super::super::Win32::Foundation::HANDLE, key: Option<*const windows_core::GUID>, securityinfo: u32, sidowner: Option<*const super::super::super::Win32::Security::SID>, sidgroup: Option<*const super::super::super::Win32::Security::SID>, dacl: Option<*const super::super::super::Win32::Security::ACL>, sacl: Option<*const super::super::super::Win32::Security::ACL>) -> super::super::super::Win32::Foundation::NTSTATUS {
    windows_core::link!("fwpuclnt.dll" "system" fn FwpmProviderSetSecurityInfoByKey0(enginehandle : super::super::super::Win32::Foundation:: HANDLE, key : *const windows_core::GUID, securityinfo : u32, sidowner : *const super::super::super::Win32::Security:: SID, sidgroup : *const super::super::super::Win32::Security:: SID, dacl : *const super::super::super::Win32::Security:: ACL, sacl : *const super::super::super::Win32::Security:: ACL) -> super::super::super::Win32::Foundation:: NTSTATUS);
    unsafe { FwpmProviderSetSecurityInfoByKey0(enginehandle, key.unwrap_or(core::mem::zeroed()) as _, securityinfo, sidowner.unwrap_or(core::mem::zeroed()) as _, sidgroup.unwrap_or(core::mem::zeroed()) as _, dacl.unwrap_or(core::mem::zeroed()) as _, sacl.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "Win32_NetworkManagement_WindowsFilteringPlatform")]
#[inline]
pub unsafe fn FwpmSessionCreateEnumHandle0(enginehandle: super::super::super::Win32::Foundation::HANDLE, enumtemplate: Option<*const super::super::super::Win32::NetworkManagement::WindowsFilteringPlatform::FWPM_SESSION_ENUM_TEMPLATE0>, enumhandle: *mut super::super::super::Win32::Foundation::HANDLE) -> super::super::super::Win32::Foundation::NTSTATUS {
    windows_core::link!("fwpuclnt.dll" "system" fn FwpmSessionCreateEnumHandle0(enginehandle : super::super::super::Win32::Foundation:: HANDLE, enumtemplate : *const super::super::super::Win32::NetworkManagement::WindowsFilteringPlatform:: FWPM_SESSION_ENUM_TEMPLATE0, enumhandle : *mut super::super::super::Win32::Foundation:: HANDLE) -> super::super::super::Win32::Foundation:: NTSTATUS);
    unsafe { FwpmSessionCreateEnumHandle0(enginehandle, enumtemplate.unwrap_or(core::mem::zeroed()) as _, enumhandle as _) }
}
#[inline]
pub unsafe fn FwpmSessionDestroyEnumHandle0(enginehandle: super::super::super::Win32::Foundation::HANDLE, enumhandle: super::super::super::Win32::Foundation::HANDLE) -> super::super::super::Win32::Foundation::NTSTATUS {
    windows_core::link!("fwpuclnt.dll" "system" fn FwpmSessionDestroyEnumHandle0(enginehandle : super::super::super::Win32::Foundation:: HANDLE, enumhandle : super::super::super::Win32::Foundation:: HANDLE) -> super::super::super::Win32::Foundation:: NTSTATUS);
    unsafe { FwpmSessionDestroyEnumHandle0(enginehandle, enumhandle as _) }
}
#[cfg(all(feature = "Win32_NetworkManagement_WindowsFilteringPlatform", feature = "Win32_Security"))]
#[inline]
pub unsafe fn FwpmSessionEnum0(enginehandle: super::super::super::Win32::Foundation::HANDLE, enumhandle: super::super::super::Win32::Foundation::HANDLE, numentriesrequested: u32, entries: *mut *mut *mut super::super::super::Win32::NetworkManagement::WindowsFilteringPlatform::FWPM_SESSION0, numentriesreturned: *mut u32) -> super::super::super::Win32::Foundation::NTSTATUS {
    windows_core::link!("fwpuclnt.dll" "system" fn FwpmSessionEnum0(enginehandle : super::super::super::Win32::Foundation:: HANDLE, enumhandle : super::super::super::Win32::Foundation:: HANDLE, numentriesrequested : u32, entries : *mut *mut *mut super::super::super::Win32::NetworkManagement::WindowsFilteringPlatform:: FWPM_SESSION0, numentriesreturned : *mut u32) -> super::super::super::Win32::Foundation:: NTSTATUS);
    unsafe { FwpmSessionEnum0(enginehandle, enumhandle, numentriesrequested, entries as _, numentriesreturned as _) }
}
#[cfg(all(feature = "Win32_NetworkManagement_WindowsFilteringPlatform", feature = "Win32_Security"))]
#[inline]
pub unsafe fn FwpmSubLayerAdd0(enginehandle: super::super::super::Win32::Foundation::HANDLE, sublayer: *const super::super::super::Win32::NetworkManagement::WindowsFilteringPlatform::FWPM_SUBLAYER0, sd: Option<super::super::super::Win32::Security::PSECURITY_DESCRIPTOR>) -> super::super::super::Win32::Foundation::NTSTATUS {
    windows_core::link!("fwpuclnt.dll" "system" fn FwpmSubLayerAdd0(enginehandle : super::super::super::Win32::Foundation:: HANDLE, sublayer : *const super::super::super::Win32::NetworkManagement::WindowsFilteringPlatform:: FWPM_SUBLAYER0, sd : super::super::super::Win32::Security:: PSECURITY_DESCRIPTOR) -> super::super::super::Win32::Foundation:: NTSTATUS);
    unsafe { FwpmSubLayerAdd0(enginehandle, sublayer, sd.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "Win32_NetworkManagement_WindowsFilteringPlatform")]
#[inline]
pub unsafe fn FwpmSubLayerCreateEnumHandle0(enginehandle: super::super::super::Win32::Foundation::HANDLE, enumtemplate: Option<*const super::super::super::Win32::NetworkManagement::WindowsFilteringPlatform::FWPM_SUBLAYER_ENUM_TEMPLATE0>, enumhandle: *mut super::super::super::Win32::Foundation::HANDLE) -> super::super::super::Win32::Foundation::NTSTATUS {
    windows_core::link!("fwpuclnt.dll" "system" fn FwpmSubLayerCreateEnumHandle0(enginehandle : super::super::super::Win32::Foundation:: HANDLE, enumtemplate : *const super::super::super::Win32::NetworkManagement::WindowsFilteringPlatform:: FWPM_SUBLAYER_ENUM_TEMPLATE0, enumhandle : *mut super::super::super::Win32::Foundation:: HANDLE) -> super::super::super::Win32::Foundation:: NTSTATUS);
    unsafe { FwpmSubLayerCreateEnumHandle0(enginehandle, enumtemplate.unwrap_or(core::mem::zeroed()) as _, enumhandle as _) }
}
#[inline]
pub unsafe fn FwpmSubLayerDeleteByKey0(enginehandle: super::super::super::Win32::Foundation::HANDLE, key: *const windows_core::GUID) -> super::super::super::Win32::Foundation::NTSTATUS {
    windows_core::link!("fwpuclnt.dll" "system" fn FwpmSubLayerDeleteByKey0(enginehandle : super::super::super::Win32::Foundation:: HANDLE, key : *const windows_core::GUID) -> super::super::super::Win32::Foundation:: NTSTATUS);
    unsafe { FwpmSubLayerDeleteByKey0(enginehandle, key) }
}
#[inline]
pub unsafe fn FwpmSubLayerDestroyEnumHandle0(enginehandle: super::super::super::Win32::Foundation::HANDLE, enumhandle: super::super::super::Win32::Foundation::HANDLE) -> super::super::super::Win32::Foundation::NTSTATUS {
    windows_core::link!("fwpuclnt.dll" "system" fn FwpmSubLayerDestroyEnumHandle0(enginehandle : super::super::super::Win32::Foundation:: HANDLE, enumhandle : super::super::super::Win32::Foundation:: HANDLE) -> super::super::super::Win32::Foundation:: NTSTATUS);
    unsafe { FwpmSubLayerDestroyEnumHandle0(enginehandle, enumhandle as _) }
}
#[cfg(feature = "Win32_NetworkManagement_WindowsFilteringPlatform")]
#[inline]
pub unsafe fn FwpmSubLayerEnum0(enginehandle: super::super::super::Win32::Foundation::HANDLE, enumhandle: super::super::super::Win32::Foundation::HANDLE, numentriesrequested: u32, entries: *mut *mut *mut super::super::super::Win32::NetworkManagement::WindowsFilteringPlatform::FWPM_SUBLAYER0, numentriesreturned: *mut u32) -> super::super::super::Win32::Foundation::NTSTATUS {
    windows_core::link!("fwpuclnt.dll" "system" fn FwpmSubLayerEnum0(enginehandle : super::super::super::Win32::Foundation:: HANDLE, enumhandle : super::super::super::Win32::Foundation:: HANDLE, numentriesrequested : u32, entries : *mut *mut *mut super::super::super::Win32::NetworkManagement::WindowsFilteringPlatform:: FWPM_SUBLAYER0, numentriesreturned : *mut u32) -> super::super::super::Win32::Foundation:: NTSTATUS);
    unsafe { FwpmSubLayerEnum0(enginehandle, enumhandle, numentriesrequested, entries as _, numentriesreturned as _) }
}
#[cfg(feature = "Win32_NetworkManagement_WindowsFilteringPlatform")]
#[inline]
pub unsafe fn FwpmSubLayerGetByKey0(enginehandle: super::super::super::Win32::Foundation::HANDLE, key: *const windows_core::GUID, sublayer: *mut *mut super::super::super::Win32::NetworkManagement::WindowsFilteringPlatform::FWPM_SUBLAYER0) -> super::super::super::Win32::Foundation::NTSTATUS {
    windows_core::link!("fwpuclnt.dll" "system" fn FwpmSubLayerGetByKey0(enginehandle : super::super::super::Win32::Foundation:: HANDLE, key : *const windows_core::GUID, sublayer : *mut *mut super::super::super::Win32::NetworkManagement::WindowsFilteringPlatform:: FWPM_SUBLAYER0) -> super::super::super::Win32::Foundation:: NTSTATUS);
    unsafe { FwpmSubLayerGetByKey0(enginehandle, key, sublayer as _) }
}
#[cfg(feature = "Win32_Security")]
#[inline]
pub unsafe fn FwpmSubLayerGetSecurityInfoByKey0(enginehandle: super::super::super::Win32::Foundation::HANDLE, key: Option<*const windows_core::GUID>, securityinfo: u32, sidowner: *mut super::super::super::Win32::Security::PSID, sidgroup: *mut super::super::super::Win32::Security::PSID, dacl: *mut *mut super::super::super::Win32::Security::ACL, sacl: *mut *mut super::super::super::Win32::Security::ACL, securitydescriptor: *mut super::super::super::Win32::Security::PSECURITY_DESCRIPTOR) -> super::super::super::Win32::Foundation::NTSTATUS {
    windows_core::link!("fwpuclnt.dll" "system" fn FwpmSubLayerGetSecurityInfoByKey0(enginehandle : super::super::super::Win32::Foundation:: HANDLE, key : *const windows_core::GUID, securityinfo : u32, sidowner : *mut super::super::super::Win32::Security:: PSID, sidgroup : *mut super::super::super::Win32::Security:: PSID, dacl : *mut *mut super::super::super::Win32::Security:: ACL, sacl : *mut *mut super::super::super::Win32::Security:: ACL, securitydescriptor : *mut super::super::super::Win32::Security:: PSECURITY_DESCRIPTOR) -> super::super::super::Win32::Foundation:: NTSTATUS);
    unsafe { FwpmSubLayerGetSecurityInfoByKey0(enginehandle, key.unwrap_or(core::mem::zeroed()) as _, securityinfo, sidowner as _, sidgroup as _, dacl as _, sacl as _, securitydescriptor as _) }
}
#[cfg(feature = "Win32_Security")]
#[inline]
pub unsafe fn FwpmSubLayerSetSecurityInfoByKey0(enginehandle: super::super::super::Win32::Foundation::HANDLE, key: Option<*const windows_core::GUID>, securityinfo: u32, sidowner: Option<*const super::super::super::Win32::Security::SID>, sidgroup: Option<*const super::super::super::Win32::Security::SID>, dacl: Option<*const super::super::super::Win32::Security::ACL>, sacl: Option<*const super::super::super::Win32::Security::ACL>) -> super::super::super::Win32::Foundation::NTSTATUS {
    windows_core::link!("fwpuclnt.dll" "system" fn FwpmSubLayerSetSecurityInfoByKey0(enginehandle : super::super::super::Win32::Foundation:: HANDLE, key : *const windows_core::GUID, securityinfo : u32, sidowner : *const super::super::super::Win32::Security:: SID, sidgroup : *const super::super::super::Win32::Security:: SID, dacl : *const super::super::super::Win32::Security:: ACL, sacl : *const super::super::super::Win32::Security:: ACL) -> super::super::super::Win32::Foundation:: NTSTATUS);
    unsafe { FwpmSubLayerSetSecurityInfoByKey0(enginehandle, key.unwrap_or(core::mem::zeroed()) as _, securityinfo, sidowner.unwrap_or(core::mem::zeroed()) as _, sidgroup.unwrap_or(core::mem::zeroed()) as _, dacl.unwrap_or(core::mem::zeroed()) as _, sacl.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn FwpmTransactionAbort0(enginehandle: super::super::super::Win32::Foundation::HANDLE) -> super::super::super::Win32::Foundation::NTSTATUS {
    windows_core::link!("fwpuclnt.dll" "system" fn FwpmTransactionAbort0(enginehandle : super::super::super::Win32::Foundation:: HANDLE) -> super::super::super::Win32::Foundation:: NTSTATUS);
    unsafe { FwpmTransactionAbort0(enginehandle) }
}
#[inline]
pub unsafe fn FwpmTransactionBegin0(enginehandle: super::super::super::Win32::Foundation::HANDLE, flags: u32) -> super::super::super::Win32::Foundation::NTSTATUS {
    windows_core::link!("fwpuclnt.dll" "system" fn FwpmTransactionBegin0(enginehandle : super::super::super::Win32::Foundation:: HANDLE, flags : u32) -> super::super::super::Win32::Foundation:: NTSTATUS);
    unsafe { FwpmTransactionBegin0(enginehandle, flags) }
}
#[inline]
pub unsafe fn FwpmTransactionCommit0(enginehandle: super::super::super::Win32::Foundation::HANDLE) -> super::super::super::Win32::Foundation::NTSTATUS {
    windows_core::link!("fwpuclnt.dll" "system" fn FwpmTransactionCommit0(enginehandle : super::super::super::Win32::Foundation:: HANDLE) -> super::super::super::Win32::Foundation:: NTSTATUS);
    unsafe { FwpmTransactionCommit0(enginehandle) }
}
#[cfg(feature = "Win32_Security")]
#[inline]
pub unsafe fn FwpmvSwitchEventsGetSecurityInfo0(enginehandle: super::super::super::Win32::Foundation::HANDLE, securityinfo: u32, sidowner: *mut super::super::super::Win32::Security::PSID, sidgroup: *mut super::super::super::Win32::Security::PSID, dacl: *mut *mut super::super::super::Win32::Security::ACL, sacl: *mut *mut super::super::super::Win32::Security::ACL, securitydescriptor: *mut super::super::super::Win32::Security::PSECURITY_DESCRIPTOR) -> super::super::super::Win32::Foundation::NTSTATUS {
    windows_core::link!("fwpuclnt.dll" "system" fn FwpmvSwitchEventsGetSecurityInfo0(enginehandle : super::super::super::Win32::Foundation:: HANDLE, securityinfo : u32, sidowner : *mut super::super::super::Win32::Security:: PSID, sidgroup : *mut super::super::super::Win32::Security:: PSID, dacl : *mut *mut super::super::super::Win32::Security:: ACL, sacl : *mut *mut super::super::super::Win32::Security:: ACL, securitydescriptor : *mut super::super::super::Win32::Security:: PSECURITY_DESCRIPTOR) -> super::super::super::Win32::Foundation:: NTSTATUS);
    unsafe { FwpmvSwitchEventsGetSecurityInfo0(enginehandle, securityinfo, sidowner as _, sidgroup as _, dacl as _, sacl as _, securitydescriptor as _) }
}
#[cfg(feature = "Win32_Security")]
#[inline]
pub unsafe fn FwpmvSwitchEventsSetSecurityInfo0(enginehandle: super::super::super::Win32::Foundation::HANDLE, securityinfo: u32, sidowner: Option<*const super::super::super::Win32::Security::SID>, sidgroup: Option<*const super::super::super::Win32::Security::SID>, dacl: Option<*const super::super::super::Win32::Security::ACL>, sacl: Option<*const super::super::super::Win32::Security::ACL>) -> super::super::super::Win32::Foundation::NTSTATUS {
    windows_core::link!("fwpuclnt.dll" "system" fn FwpmvSwitchEventsSetSecurityInfo0(enginehandle : super::super::super::Win32::Foundation:: HANDLE, securityinfo : u32, sidowner : *const super::super::super::Win32::Security:: SID, sidgroup : *const super::super::super::Win32::Security:: SID, dacl : *const super::super::super::Win32::Security:: ACL, sacl : *const super::super::super::Win32::Security:: ACL) -> super::super::super::Win32::Foundation:: NTSTATUS);
    unsafe { FwpmvSwitchEventsSetSecurityInfo0(enginehandle, securityinfo, sidowner.unwrap_or(core::mem::zeroed()) as _, sidgroup.unwrap_or(core::mem::zeroed()) as _, dacl.unwrap_or(core::mem::zeroed()) as _, sacl.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "Win32_Security")]
#[inline]
pub unsafe fn IPsecDospGetSecurityInfo0(enginehandle: super::super::super::Win32::Foundation::HANDLE, securityinfo: u32, sidowner: *mut super::super::super::Win32::Security::PSID, sidgroup: *mut super::super::super::Win32::Security::PSID, dacl: *mut *mut super::super::super::Win32::Security::ACL, sacl: *mut *mut super::super::super::Win32::Security::ACL, securitydescriptor: *mut super::super::super::Win32::Security::PSECURITY_DESCRIPTOR) -> super::super::super::Win32::Foundation::NTSTATUS {
    windows_core::link!("fwpuclnt.dll" "system" fn IPsecDospGetSecurityInfo0(enginehandle : super::super::super::Win32::Foundation:: HANDLE, securityinfo : u32, sidowner : *mut super::super::super::Win32::Security:: PSID, sidgroup : *mut super::super::super::Win32::Security:: PSID, dacl : *mut *mut super::super::super::Win32::Security:: ACL, sacl : *mut *mut super::super::super::Win32::Security:: ACL, securitydescriptor : *mut super::super::super::Win32::Security:: PSECURITY_DESCRIPTOR) -> super::super::super::Win32::Foundation:: NTSTATUS);
    unsafe { IPsecDospGetSecurityInfo0(enginehandle, securityinfo, sidowner as _, sidgroup as _, dacl as _, sacl as _, securitydescriptor as _) }
}
#[cfg(feature = "Win32_NetworkManagement_WindowsFilteringPlatform")]
#[inline]
pub unsafe fn IPsecDospGetStatistics0(enginehandle: super::super::super::Win32::Foundation::HANDLE, idpstatistics: *mut super::super::super::Win32::NetworkManagement::WindowsFilteringPlatform::IPSEC_DOSP_STATISTICS0) -> super::super::super::Win32::Foundation::NTSTATUS {
    windows_core::link!("fwpuclnt.dll" "system" fn IPsecDospGetStatistics0(enginehandle : super::super::super::Win32::Foundation:: HANDLE, idpstatistics : *mut super::super::super::Win32::NetworkManagement::WindowsFilteringPlatform:: IPSEC_DOSP_STATISTICS0) -> super::super::super::Win32::Foundation:: NTSTATUS);
    unsafe { IPsecDospGetStatistics0(enginehandle, idpstatistics as _) }
}
#[cfg(feature = "Win32_Security")]
#[inline]
pub unsafe fn IPsecDospSetSecurityInfo0(enginehandle: super::super::super::Win32::Foundation::HANDLE, securityinfo: u32, sidowner: Option<*const super::super::super::Win32::Security::SID>, sidgroup: Option<*const super::super::super::Win32::Security::SID>, dacl: Option<*const super::super::super::Win32::Security::ACL>, sacl: Option<*const super::super::super::Win32::Security::ACL>) -> super::super::super::Win32::Foundation::NTSTATUS {
    windows_core::link!("fwpuclnt.dll" "system" fn IPsecDospSetSecurityInfo0(enginehandle : super::super::super::Win32::Foundation:: HANDLE, securityinfo : u32, sidowner : *const super::super::super::Win32::Security:: SID, sidgroup : *const super::super::super::Win32::Security:: SID, dacl : *const super::super::super::Win32::Security:: ACL, sacl : *const super::super::super::Win32::Security:: ACL) -> super::super::super::Win32::Foundation:: NTSTATUS);
    unsafe { IPsecDospSetSecurityInfo0(enginehandle, securityinfo, sidowner.unwrap_or(core::mem::zeroed()) as _, sidgroup.unwrap_or(core::mem::zeroed()) as _, dacl.unwrap_or(core::mem::zeroed()) as _, sacl.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "Win32_NetworkManagement_WindowsFilteringPlatform")]
#[inline]
pub unsafe fn IPsecDospStateCreateEnumHandle0(enginehandle: super::super::super::Win32::Foundation::HANDLE, enumtemplate: Option<*const super::super::super::Win32::NetworkManagement::WindowsFilteringPlatform::IPSEC_DOSP_STATE_ENUM_TEMPLATE0>, enumhandle: *mut super::super::super::Win32::Foundation::HANDLE) -> super::super::super::Win32::Foundation::NTSTATUS {
    windows_core::link!("fwpuclnt.dll" "system" fn IPsecDospStateCreateEnumHandle0(enginehandle : super::super::super::Win32::Foundation:: HANDLE, enumtemplate : *const super::super::super::Win32::NetworkManagement::WindowsFilteringPlatform:: IPSEC_DOSP_STATE_ENUM_TEMPLATE0, enumhandle : *mut super::super::super::Win32::Foundation:: HANDLE) -> super::super::super::Win32::Foundation:: NTSTATUS);
    unsafe { IPsecDospStateCreateEnumHandle0(enginehandle, enumtemplate.unwrap_or(core::mem::zeroed()) as _, enumhandle as _) }
}
#[inline]
pub unsafe fn IPsecDospStateDestroyEnumHandle0(enginehandle: super::super::super::Win32::Foundation::HANDLE, enumhandle: super::super::super::Win32::Foundation::HANDLE) -> super::super::super::Win32::Foundation::NTSTATUS {
    windows_core::link!("fwpuclnt.dll" "system" fn IPsecDospStateDestroyEnumHandle0(enginehandle : super::super::super::Win32::Foundation:: HANDLE, enumhandle : super::super::super::Win32::Foundation:: HANDLE) -> super::super::super::Win32::Foundation:: NTSTATUS);
    unsafe { IPsecDospStateDestroyEnumHandle0(enginehandle, enumhandle as _) }
}
#[cfg(feature = "Win32_NetworkManagement_WindowsFilteringPlatform")]
#[inline]
pub unsafe fn IPsecDospStateEnum0(enginehandle: super::super::super::Win32::Foundation::HANDLE, enumhandle: super::super::super::Win32::Foundation::HANDLE, numentriesrequested: u32, entries: *mut *mut *mut super::super::super::Win32::NetworkManagement::WindowsFilteringPlatform::IPSEC_DOSP_STATE0, numentries: *mut u32) -> super::super::super::Win32::Foundation::NTSTATUS {
    windows_core::link!("fwpuclnt.dll" "system" fn IPsecDospStateEnum0(enginehandle : super::super::super::Win32::Foundation:: HANDLE, enumhandle : super::super::super::Win32::Foundation:: HANDLE, numentriesrequested : u32, entries : *mut *mut *mut super::super::super::Win32::NetworkManagement::WindowsFilteringPlatform:: IPSEC_DOSP_STATE0, numentries : *mut u32) -> super::super::super::Win32::Foundation:: NTSTATUS);
    unsafe { IPsecDospStateEnum0(enginehandle, enumhandle, numentriesrequested, entries as _, numentries as _) }
}
#[cfg(feature = "Win32_NetworkManagement_WindowsFilteringPlatform")]
#[inline]
pub unsafe fn IPsecGetStatistics0(enginehandle: super::super::super::Win32::Foundation::HANDLE, ipsecstatistics: *mut super::super::super::Win32::NetworkManagement::WindowsFilteringPlatform::IPSEC_STATISTICS0) -> super::super::super::Win32::Foundation::NTSTATUS {
    windows_core::link!("fwpuclnt.dll" "system" fn IPsecGetStatistics0(enginehandle : super::super::super::Win32::Foundation:: HANDLE, ipsecstatistics : *mut super::super::super::Win32::NetworkManagement::WindowsFilteringPlatform:: IPSEC_STATISTICS0) -> super::super::super::Win32::Foundation:: NTSTATUS);
    unsafe { IPsecGetStatistics0(enginehandle, ipsecstatistics as _) }
}
#[cfg(feature = "Win32_NetworkManagement_WindowsFilteringPlatform")]
#[inline]
pub unsafe fn IPsecGetStatistics1(enginehandle: super::super::super::Win32::Foundation::HANDLE, ipsecstatistics: *mut super::super::super::Win32::NetworkManagement::WindowsFilteringPlatform::IPSEC_STATISTICS1) -> super::super::super::Win32::Foundation::NTSTATUS {
    windows_core::link!("fwpuclnt.dll" "system" fn IPsecGetStatistics1(enginehandle : super::super::super::Win32::Foundation:: HANDLE, ipsecstatistics : *mut super::super::super::Win32::NetworkManagement::WindowsFilteringPlatform:: IPSEC_STATISTICS1) -> super::super::super::Win32::Foundation:: NTSTATUS);
    unsafe { IPsecGetStatistics1(enginehandle, ipsecstatistics as _) }
}
#[cfg(feature = "Win32_NetworkManagement_WindowsFilteringPlatform")]
#[inline]
pub unsafe fn IPsecSaContextAddInbound0(enginehandle: super::super::super::Win32::Foundation::HANDLE, id: u64, inboundbundle: *const super::super::super::Win32::NetworkManagement::WindowsFilteringPlatform::IPSEC_SA_BUNDLE0) -> super::super::super::Win32::Foundation::NTSTATUS {
    windows_core::link!("fwpuclnt.dll" "system" fn IPsecSaContextAddInbound0(enginehandle : super::super::super::Win32::Foundation:: HANDLE, id : u64, inboundbundle : *const super::super::super::Win32::NetworkManagement::WindowsFilteringPlatform:: IPSEC_SA_BUNDLE0) -> super::super::super::Win32::Foundation:: NTSTATUS);
    unsafe { IPsecSaContextAddInbound0(enginehandle, id, inboundbundle) }
}
#[cfg(feature = "Win32_NetworkManagement_WindowsFilteringPlatform")]
#[inline]
pub unsafe fn IPsecSaContextAddInbound1(enginehandle: super::super::super::Win32::Foundation::HANDLE, id: u64, inboundbundle: *const super::super::super::Win32::NetworkManagement::WindowsFilteringPlatform::IPSEC_SA_BUNDLE1) -> super::super::super::Win32::Foundation::NTSTATUS {
    windows_core::link!("fwpuclnt.dll" "system" fn IPsecSaContextAddInbound1(enginehandle : super::super::super::Win32::Foundation:: HANDLE, id : u64, inboundbundle : *const super::super::super::Win32::NetworkManagement::WindowsFilteringPlatform:: IPSEC_SA_BUNDLE1) -> super::super::super::Win32::Foundation:: NTSTATUS);
    unsafe { IPsecSaContextAddInbound1(enginehandle, id, inboundbundle) }
}
#[cfg(feature = "Win32_NetworkManagement_WindowsFilteringPlatform")]
#[inline]
pub unsafe fn IPsecSaContextAddOutbound0(enginehandle: super::super::super::Win32::Foundation::HANDLE, id: u64, outboundbundle: *const super::super::super::Win32::NetworkManagement::WindowsFilteringPlatform::IPSEC_SA_BUNDLE0) -> super::super::super::Win32::Foundation::NTSTATUS {
    windows_core::link!("fwpuclnt.dll" "system" fn IPsecSaContextAddOutbound0(enginehandle : super::super::super::Win32::Foundation:: HANDLE, id : u64, outboundbundle : *const super::super::super::Win32::NetworkManagement::WindowsFilteringPlatform:: IPSEC_SA_BUNDLE0) -> super::super::super::Win32::Foundation:: NTSTATUS);
    unsafe { IPsecSaContextAddOutbound0(enginehandle, id, outboundbundle) }
}
#[cfg(feature = "Win32_NetworkManagement_WindowsFilteringPlatform")]
#[inline]
pub unsafe fn IPsecSaContextAddOutbound1(enginehandle: super::super::super::Win32::Foundation::HANDLE, id: u64, outboundbundle: *const super::super::super::Win32::NetworkManagement::WindowsFilteringPlatform::IPSEC_SA_BUNDLE1) -> super::super::super::Win32::Foundation::NTSTATUS {
    windows_core::link!("fwpuclnt.dll" "system" fn IPsecSaContextAddOutbound1(enginehandle : super::super::super::Win32::Foundation:: HANDLE, id : u64, outboundbundle : *const super::super::super::Win32::NetworkManagement::WindowsFilteringPlatform:: IPSEC_SA_BUNDLE1) -> super::super::super::Win32::Foundation:: NTSTATUS);
    unsafe { IPsecSaContextAddOutbound1(enginehandle, id, outboundbundle) }
}
#[cfg(feature = "Win32_NetworkManagement_WindowsFilteringPlatform")]
#[inline]
pub unsafe fn IPsecSaContextCreate0(enginehandle: super::super::super::Win32::Foundation::HANDLE, outboundtraffic: *const super::super::super::Win32::NetworkManagement::WindowsFilteringPlatform::IPSEC_TRAFFIC0, inboundfilterid: Option<*mut u64>, id: *mut u64) -> super::super::super::Win32::Foundation::NTSTATUS {
    windows_core::link!("fwpuclnt.dll" "system" fn IPsecSaContextCreate0(enginehandle : super::super::super::Win32::Foundation:: HANDLE, outboundtraffic : *const super::super::super::Win32::NetworkManagement::WindowsFilteringPlatform:: IPSEC_TRAFFIC0, inboundfilterid : *mut u64, id : *mut u64) -> super::super::super::Win32::Foundation:: NTSTATUS);
    unsafe { IPsecSaContextCreate0(enginehandle, outboundtraffic, inboundfilterid.unwrap_or(core::mem::zeroed()) as _, id as _) }
}
#[cfg(feature = "Win32_NetworkManagement_WindowsFilteringPlatform")]
#[inline]
pub unsafe fn IPsecSaContextCreate1(enginehandle: super::super::super::Win32::Foundation::HANDLE, outboundtraffic: *const super::super::super::Win32::NetworkManagement::WindowsFilteringPlatform::IPSEC_TRAFFIC1, virtualiftunnelinfo: Option<*const super::super::super::Win32::NetworkManagement::WindowsFilteringPlatform::IPSEC_VIRTUAL_IF_TUNNEL_INFO0>, inboundfilterid: Option<*mut u64>, id: *mut u64) -> super::super::super::Win32::Foundation::NTSTATUS {
    windows_core::link!("fwpuclnt.dll" "system" fn IPsecSaContextCreate1(enginehandle : super::super::super::Win32::Foundation:: HANDLE, outboundtraffic : *const super::super::super::Win32::NetworkManagement::WindowsFilteringPlatform:: IPSEC_TRAFFIC1, virtualiftunnelinfo : *const super::super::super::Win32::NetworkManagement::WindowsFilteringPlatform:: IPSEC_VIRTUAL_IF_TUNNEL_INFO0, inboundfilterid : *mut u64, id : *mut u64) -> super::super::super::Win32::Foundation:: NTSTATUS);
    unsafe { IPsecSaContextCreate1(enginehandle, outboundtraffic, virtualiftunnelinfo.unwrap_or(core::mem::zeroed()) as _, inboundfilterid.unwrap_or(core::mem::zeroed()) as _, id as _) }
}
#[cfg(all(feature = "Win32_NetworkManagement_WindowsFilteringPlatform", feature = "Win32_Security"))]
#[inline]
pub unsafe fn IPsecSaContextCreateEnumHandle0(enginehandle: super::super::super::Win32::Foundation::HANDLE, enumtemplate: Option<*const super::super::super::Win32::NetworkManagement::WindowsFilteringPlatform::IPSEC_SA_CONTEXT_ENUM_TEMPLATE0>, enumhandle: *mut super::super::super::Win32::Foundation::HANDLE) -> super::super::super::Win32::Foundation::NTSTATUS {
    windows_core::link!("fwpuclnt.dll" "system" fn IPsecSaContextCreateEnumHandle0(enginehandle : super::super::super::Win32::Foundation:: HANDLE, enumtemplate : *const super::super::super::Win32::NetworkManagement::WindowsFilteringPlatform:: IPSEC_SA_CONTEXT_ENUM_TEMPLATE0, enumhandle : *mut super::super::super::Win32::Foundation:: HANDLE) -> super::super::super::Win32::Foundation:: NTSTATUS);
    unsafe { IPsecSaContextCreateEnumHandle0(enginehandle, enumtemplate.unwrap_or(core::mem::zeroed()) as _, enumhandle as _) }
}
#[inline]
pub unsafe fn IPsecSaContextDeleteById0(enginehandle: super::super::super::Win32::Foundation::HANDLE, id: u64) -> super::super::super::Win32::Foundation::NTSTATUS {
    windows_core::link!("fwpuclnt.dll" "system" fn IPsecSaContextDeleteById0(enginehandle : super::super::super::Win32::Foundation:: HANDLE, id : u64) -> super::super::super::Win32::Foundation:: NTSTATUS);
    unsafe { IPsecSaContextDeleteById0(enginehandle, id) }
}
#[inline]
pub unsafe fn IPsecSaContextDestroyEnumHandle0(enginehandle: super::super::super::Win32::Foundation::HANDLE, enumhandle: super::super::super::Win32::Foundation::HANDLE) -> super::super::super::Win32::Foundation::NTSTATUS {
    windows_core::link!("fwpuclnt.dll" "system" fn IPsecSaContextDestroyEnumHandle0(enginehandle : super::super::super::Win32::Foundation:: HANDLE, enumhandle : super::super::super::Win32::Foundation:: HANDLE) -> super::super::super::Win32::Foundation:: NTSTATUS);
    unsafe { IPsecSaContextDestroyEnumHandle0(enginehandle, enumhandle as _) }
}
#[cfg(all(feature = "Win32_NetworkManagement_WindowsFilteringPlatform", feature = "Win32_Security"))]
#[inline]
pub unsafe fn IPsecSaContextEnum0(enginehandle: super::super::super::Win32::Foundation::HANDLE, enumhandle: super::super::super::Win32::Foundation::HANDLE, numentriesrequested: u32, entries: *mut *mut *mut super::super::super::Win32::NetworkManagement::WindowsFilteringPlatform::IPSEC_SA_CONTEXT0, numentriesreturned: *mut u32) -> super::super::super::Win32::Foundation::NTSTATUS {
    windows_core::link!("fwpuclnt.dll" "system" fn IPsecSaContextEnum0(enginehandle : super::super::super::Win32::Foundation:: HANDLE, enumhandle : super::super::super::Win32::Foundation:: HANDLE, numentriesrequested : u32, entries : *mut *mut *mut super::super::super::Win32::NetworkManagement::WindowsFilteringPlatform:: IPSEC_SA_CONTEXT0, numentriesreturned : *mut u32) -> super::super::super::Win32::Foundation:: NTSTATUS);
    unsafe { IPsecSaContextEnum0(enginehandle, enumhandle, numentriesrequested, entries as _, numentriesreturned as _) }
}
#[cfg(all(feature = "Win32_NetworkManagement_WindowsFilteringPlatform", feature = "Win32_Security"))]
#[inline]
pub unsafe fn IPsecSaContextEnum1(enginehandle: super::super::super::Win32::Foundation::HANDLE, enumhandle: super::super::super::Win32::Foundation::HANDLE, numentriesrequested: u32, entries: *mut *mut *mut super::super::super::Win32::NetworkManagement::WindowsFilteringPlatform::IPSEC_SA_CONTEXT1, numentriesreturned: *mut u32) -> super::super::super::Win32::Foundation::NTSTATUS {
    windows_core::link!("fwpuclnt.dll" "system" fn IPsecSaContextEnum1(enginehandle : super::super::super::Win32::Foundation:: HANDLE, enumhandle : super::super::super::Win32::Foundation:: HANDLE, numentriesrequested : u32, entries : *mut *mut *mut super::super::super::Win32::NetworkManagement::WindowsFilteringPlatform:: IPSEC_SA_CONTEXT1, numentriesreturned : *mut u32) -> super::super::super::Win32::Foundation:: NTSTATUS);
    unsafe { IPsecSaContextEnum1(enginehandle, enumhandle, numentriesrequested, entries as _, numentriesreturned as _) }
}
#[inline]
pub unsafe fn IPsecSaContextExpire0(enginehandle: super::super::super::Win32::Foundation::HANDLE, id: u64) -> super::super::super::Win32::Foundation::NTSTATUS {
    windows_core::link!("fwpuclnt.dll" "system" fn IPsecSaContextExpire0(enginehandle : super::super::super::Win32::Foundation:: HANDLE, id : u64) -> super::super::super::Win32::Foundation:: NTSTATUS);
    unsafe { IPsecSaContextExpire0(enginehandle, id) }
}
#[cfg(all(feature = "Win32_NetworkManagement_WindowsFilteringPlatform", feature = "Win32_Security"))]
#[inline]
pub unsafe fn IPsecSaContextGetById0(enginehandle: super::super::super::Win32::Foundation::HANDLE, id: u64, sacontext: *mut *mut super::super::super::Win32::NetworkManagement::WindowsFilteringPlatform::IPSEC_SA_CONTEXT0) -> super::super::super::Win32::Foundation::NTSTATUS {
    windows_core::link!("fwpuclnt.dll" "system" fn IPsecSaContextGetById0(enginehandle : super::super::super::Win32::Foundation:: HANDLE, id : u64, sacontext : *mut *mut super::super::super::Win32::NetworkManagement::WindowsFilteringPlatform:: IPSEC_SA_CONTEXT0) -> super::super::super::Win32::Foundation:: NTSTATUS);
    unsafe { IPsecSaContextGetById0(enginehandle, id, sacontext as _) }
}
#[cfg(all(feature = "Win32_NetworkManagement_WindowsFilteringPlatform", feature = "Win32_Security"))]
#[inline]
pub unsafe fn IPsecSaContextGetById1(enginehandle: super::super::super::Win32::Foundation::HANDLE, id: u64, sacontext: *mut *mut super::super::super::Win32::NetworkManagement::WindowsFilteringPlatform::IPSEC_SA_CONTEXT1) -> super::super::super::Win32::Foundation::NTSTATUS {
    windows_core::link!("fwpuclnt.dll" "system" fn IPsecSaContextGetById1(enginehandle : super::super::super::Win32::Foundation:: HANDLE, id : u64, sacontext : *mut *mut super::super::super::Win32::NetworkManagement::WindowsFilteringPlatform:: IPSEC_SA_CONTEXT1) -> super::super::super::Win32::Foundation:: NTSTATUS);
    unsafe { IPsecSaContextGetById1(enginehandle, id, sacontext as _) }
}
#[cfg(feature = "Win32_NetworkManagement_WindowsFilteringPlatform")]
#[inline]
pub unsafe fn IPsecSaContextGetSpi0(enginehandle: super::super::super::Win32::Foundation::HANDLE, id: u64, getspi: *const super::super::super::Win32::NetworkManagement::WindowsFilteringPlatform::IPSEC_GETSPI0, inboundspi: *mut u32) -> super::super::super::Win32::Foundation::NTSTATUS {
    windows_core::link!("fwpuclnt.dll" "system" fn IPsecSaContextGetSpi0(enginehandle : super::super::super::Win32::Foundation:: HANDLE, id : u64, getspi : *const super::super::super::Win32::NetworkManagement::WindowsFilteringPlatform:: IPSEC_GETSPI0, inboundspi : *mut u32) -> super::super::super::Win32::Foundation:: NTSTATUS);
    unsafe { IPsecSaContextGetSpi0(enginehandle, id, getspi, inboundspi as _) }
}
#[cfg(feature = "Win32_NetworkManagement_WindowsFilteringPlatform")]
#[inline]
pub unsafe fn IPsecSaContextGetSpi1(enginehandle: super::super::super::Win32::Foundation::HANDLE, id: u64, getspi: *const super::super::super::Win32::NetworkManagement::WindowsFilteringPlatform::IPSEC_GETSPI1, inboundspi: *mut u32) -> super::super::super::Win32::Foundation::NTSTATUS {
    windows_core::link!("fwpuclnt.dll" "system" fn IPsecSaContextGetSpi1(enginehandle : super::super::super::Win32::Foundation:: HANDLE, id : u64, getspi : *const super::super::super::Win32::NetworkManagement::WindowsFilteringPlatform:: IPSEC_GETSPI1, inboundspi : *mut u32) -> super::super::super::Win32::Foundation:: NTSTATUS);
    unsafe { IPsecSaContextGetSpi1(enginehandle, id, getspi, inboundspi as _) }
}
#[cfg(feature = "Win32_NetworkManagement_WindowsFilteringPlatform")]
#[inline]
pub unsafe fn IPsecSaContextSetSpi0(enginehandle: super::super::super::Win32::Foundation::HANDLE, id: u64, getspi: *const super::super::super::Win32::NetworkManagement::WindowsFilteringPlatform::IPSEC_GETSPI1, inboundspi: u32) -> super::super::super::Win32::Foundation::NTSTATUS {
    windows_core::link!("fwpuclnt.dll" "system" fn IPsecSaContextSetSpi0(enginehandle : super::super::super::Win32::Foundation:: HANDLE, id : u64, getspi : *const super::super::super::Win32::NetworkManagement::WindowsFilteringPlatform:: IPSEC_GETSPI1, inboundspi : u32) -> super::super::super::Win32::Foundation:: NTSTATUS);
    unsafe { IPsecSaContextSetSpi0(enginehandle, id, getspi, inboundspi) }
}
#[cfg(all(feature = "Win32_NetworkManagement_WindowsFilteringPlatform", feature = "Win32_Security"))]
#[inline]
pub unsafe fn IPsecSaContextUpdate0(enginehandle: super::super::super::Win32::Foundation::HANDLE, flags: u64, newvalues: *const super::super::super::Win32::NetworkManagement::WindowsFilteringPlatform::IPSEC_SA_CONTEXT1) -> super::super::super::Win32::Foundation::NTSTATUS {
    windows_core::link!("fwpuclnt.dll" "system" fn IPsecSaContextUpdate0(enginehandle : super::super::super::Win32::Foundation:: HANDLE, flags : u64, newvalues : *const super::super::super::Win32::NetworkManagement::WindowsFilteringPlatform:: IPSEC_SA_CONTEXT1) -> super::super::super::Win32::Foundation:: NTSTATUS);
    unsafe { IPsecSaContextUpdate0(enginehandle, flags, newvalues) }
}
#[cfg(feature = "Win32_NetworkManagement_WindowsFilteringPlatform")]
#[inline]
pub unsafe fn IPsecSaCreateEnumHandle0(enginehandle: super::super::super::Win32::Foundation::HANDLE, enumtemplate: Option<*const super::super::super::Win32::NetworkManagement::WindowsFilteringPlatform::IPSEC_SA_ENUM_TEMPLATE0>, enumhandle: *mut super::super::super::Win32::Foundation::HANDLE) -> super::super::super::Win32::Foundation::NTSTATUS {
    windows_core::link!("fwpuclnt.dll" "system" fn IPsecSaCreateEnumHandle0(enginehandle : super::super::super::Win32::Foundation:: HANDLE, enumtemplate : *const super::super::super::Win32::NetworkManagement::WindowsFilteringPlatform:: IPSEC_SA_ENUM_TEMPLATE0, enumhandle : *mut super::super::super::Win32::Foundation:: HANDLE) -> super::super::super::Win32::Foundation:: NTSTATUS);
    unsafe { IPsecSaCreateEnumHandle0(enginehandle, enumtemplate.unwrap_or(core::mem::zeroed()) as _, enumhandle as _) }
}
#[cfg(feature = "Win32_Security")]
#[inline]
pub unsafe fn IPsecSaDbGetSecurityInfo0(enginehandle: super::super::super::Win32::Foundation::HANDLE, securityinfo: u32, sidowner: *mut super::super::super::Win32::Security::PSID, sidgroup: *mut super::super::super::Win32::Security::PSID, dacl: *mut *mut super::super::super::Win32::Security::ACL, sacl: *mut *mut super::super::super::Win32::Security::ACL, securitydescriptor: *mut super::super::super::Win32::Security::PSECURITY_DESCRIPTOR) -> super::super::super::Win32::Foundation::NTSTATUS {
    windows_core::link!("fwpuclnt.dll" "system" fn IPsecSaDbGetSecurityInfo0(enginehandle : super::super::super::Win32::Foundation:: HANDLE, securityinfo : u32, sidowner : *mut super::super::super::Win32::Security:: PSID, sidgroup : *mut super::super::super::Win32::Security:: PSID, dacl : *mut *mut super::super::super::Win32::Security:: ACL, sacl : *mut *mut super::super::super::Win32::Security:: ACL, securitydescriptor : *mut super::super::super::Win32::Security:: PSECURITY_DESCRIPTOR) -> super::super::super::Win32::Foundation:: NTSTATUS);
    unsafe { IPsecSaDbGetSecurityInfo0(enginehandle, securityinfo, sidowner as _, sidgroup as _, dacl as _, sacl as _, securitydescriptor as _) }
}
#[cfg(feature = "Win32_Security")]
#[inline]
pub unsafe fn IPsecSaDbSetSecurityInfo0(enginehandle: super::super::super::Win32::Foundation::HANDLE, securityinfo: u32, sidowner: Option<*const super::super::super::Win32::Security::SID>, sidgroup: Option<*const super::super::super::Win32::Security::SID>, dacl: Option<*const super::super::super::Win32::Security::ACL>, sacl: Option<*const super::super::super::Win32::Security::ACL>) -> super::super::super::Win32::Foundation::NTSTATUS {
    windows_core::link!("fwpuclnt.dll" "system" fn IPsecSaDbSetSecurityInfo0(enginehandle : super::super::super::Win32::Foundation:: HANDLE, securityinfo : u32, sidowner : *const super::super::super::Win32::Security:: SID, sidgroup : *const super::super::super::Win32::Security:: SID, dacl : *const super::super::super::Win32::Security:: ACL, sacl : *const super::super::super::Win32::Security:: ACL) -> super::super::super::Win32::Foundation:: NTSTATUS);
    unsafe { IPsecSaDbSetSecurityInfo0(enginehandle, securityinfo, sidowner.unwrap_or(core::mem::zeroed()) as _, sidgroup.unwrap_or(core::mem::zeroed()) as _, dacl.unwrap_or(core::mem::zeroed()) as _, sacl.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn IPsecSaDestroyEnumHandle0(enginehandle: super::super::super::Win32::Foundation::HANDLE, enumhandle: super::super::super::Win32::Foundation::HANDLE) -> super::super::super::Win32::Foundation::NTSTATUS {
    windows_core::link!("fwpuclnt.dll" "system" fn IPsecSaDestroyEnumHandle0(enginehandle : super::super::super::Win32::Foundation:: HANDLE, enumhandle : super::super::super::Win32::Foundation:: HANDLE) -> super::super::super::Win32::Foundation:: NTSTATUS);
    unsafe { IPsecSaDestroyEnumHandle0(enginehandle, enumhandle as _) }
}
#[cfg(all(feature = "Win32_NetworkManagement_WindowsFilteringPlatform", feature = "Win32_Security"))]
#[inline]
pub unsafe fn IPsecSaEnum0(enginehandle: super::super::super::Win32::Foundation::HANDLE, enumhandle: super::super::super::Win32::Foundation::HANDLE, numentriesrequested: u32, entries: *mut *mut *mut super::super::super::Win32::NetworkManagement::WindowsFilteringPlatform::IPSEC_SA_DETAILS0, numentriesreturned: *mut u32) -> super::super::super::Win32::Foundation::NTSTATUS {
    windows_core::link!("fwpuclnt.dll" "system" fn IPsecSaEnum0(enginehandle : super::super::super::Win32::Foundation:: HANDLE, enumhandle : super::super::super::Win32::Foundation:: HANDLE, numentriesrequested : u32, entries : *mut *mut *mut super::super::super::Win32::NetworkManagement::WindowsFilteringPlatform:: IPSEC_SA_DETAILS0, numentriesreturned : *mut u32) -> super::super::super::Win32::Foundation:: NTSTATUS);
    unsafe { IPsecSaEnum0(enginehandle, enumhandle, numentriesrequested, entries as _, numentriesreturned as _) }
}
#[cfg(all(feature = "Win32_NetworkManagement_WindowsFilteringPlatform", feature = "Win32_Security"))]
#[inline]
pub unsafe fn IPsecSaEnum1(enginehandle: super::super::super::Win32::Foundation::HANDLE, enumhandle: super::super::super::Win32::Foundation::HANDLE, numentriesrequested: u32, entries: *mut *mut *mut super::super::super::Win32::NetworkManagement::WindowsFilteringPlatform::IPSEC_SA_DETAILS1, numentriesreturned: *mut u32) -> super::super::super::Win32::Foundation::NTSTATUS {
    windows_core::link!("fwpuclnt.dll" "system" fn IPsecSaEnum1(enginehandle : super::super::super::Win32::Foundation:: HANDLE, enumhandle : super::super::super::Win32::Foundation:: HANDLE, numentriesrequested : u32, entries : *mut *mut *mut super::super::super::Win32::NetworkManagement::WindowsFilteringPlatform:: IPSEC_SA_DETAILS1, numentriesreturned : *mut u32) -> super::super::super::Win32::Foundation:: NTSTATUS);
    unsafe { IPsecSaEnum1(enginehandle, enumhandle, numentriesrequested, entries as _, numentriesreturned as _) }
}
#[cfg(feature = "Win32_NetworkManagement_WindowsFilteringPlatform")]
#[inline]
pub unsafe fn IkeextGetStatistics0(enginehandle: super::super::super::Win32::Foundation::HANDLE, ikeextstatistics: *mut super::super::super::Win32::NetworkManagement::WindowsFilteringPlatform::IKEEXT_STATISTICS0) -> super::super::super::Win32::Foundation::NTSTATUS {
    windows_core::link!("fwpuclnt.dll" "system" fn IkeextGetStatistics0(enginehandle : super::super::super::Win32::Foundation:: HANDLE, ikeextstatistics : *mut super::super::super::Win32::NetworkManagement::WindowsFilteringPlatform:: IKEEXT_STATISTICS0) -> super::super::super::Win32::Foundation:: NTSTATUS);
    unsafe { IkeextGetStatistics0(enginehandle, ikeextstatistics as _) }
}
#[cfg(feature = "Win32_NetworkManagement_WindowsFilteringPlatform")]
#[inline]
pub unsafe fn IkeextGetStatistics1(enginehandle: super::super::super::Win32::Foundation::HANDLE, ikeextstatistics: *mut super::super::super::Win32::NetworkManagement::WindowsFilteringPlatform::IKEEXT_STATISTICS1) -> super::super::super::Win32::Foundation::NTSTATUS {
    windows_core::link!("fwpuclnt.dll" "system" fn IkeextGetStatistics1(enginehandle : super::super::super::Win32::Foundation:: HANDLE, ikeextstatistics : *mut super::super::super::Win32::NetworkManagement::WindowsFilteringPlatform:: IKEEXT_STATISTICS1) -> super::super::super::Win32::Foundation:: NTSTATUS);
    unsafe { IkeextGetStatistics1(enginehandle, ikeextstatistics as _) }
}
#[cfg(all(feature = "Win32_NetworkManagement_WindowsFilteringPlatform", feature = "Win32_Security"))]
#[inline]
pub unsafe fn IkeextSaCreateEnumHandle0(enginehandle: super::super::super::Win32::Foundation::HANDLE, enumtemplate: Option<*const super::super::super::Win32::NetworkManagement::WindowsFilteringPlatform::IKEEXT_SA_ENUM_TEMPLATE0>, enumhandle: *mut super::super::super::Win32::Foundation::HANDLE) -> super::super::super::Win32::Foundation::NTSTATUS {
    windows_core::link!("fwpuclnt.dll" "system" fn IkeextSaCreateEnumHandle0(enginehandle : super::super::super::Win32::Foundation:: HANDLE, enumtemplate : *const super::super::super::Win32::NetworkManagement::WindowsFilteringPlatform:: IKEEXT_SA_ENUM_TEMPLATE0, enumhandle : *mut super::super::super::Win32::Foundation:: HANDLE) -> super::super::super::Win32::Foundation:: NTSTATUS);
    unsafe { IkeextSaCreateEnumHandle0(enginehandle, enumtemplate.unwrap_or(core::mem::zeroed()) as _, enumhandle as _) }
}
#[cfg(feature = "Win32_Security")]
#[inline]
pub unsafe fn IkeextSaDbGetSecurityInfo0(enginehandle: super::super::super::Win32::Foundation::HANDLE, securityinfo: u32, sidowner: *mut super::super::super::Win32::Security::PSID, sidgroup: *mut super::super::super::Win32::Security::PSID, dacl: *mut *mut super::super::super::Win32::Security::ACL, sacl: *mut *mut super::super::super::Win32::Security::ACL, securitydescriptor: *mut super::super::super::Win32::Security::PSECURITY_DESCRIPTOR) -> super::super::super::Win32::Foundation::NTSTATUS {
    windows_core::link!("fwpuclnt.dll" "system" fn IkeextSaDbGetSecurityInfo0(enginehandle : super::super::super::Win32::Foundation:: HANDLE, securityinfo : u32, sidowner : *mut super::super::super::Win32::Security:: PSID, sidgroup : *mut super::super::super::Win32::Security:: PSID, dacl : *mut *mut super::super::super::Win32::Security:: ACL, sacl : *mut *mut super::super::super::Win32::Security:: ACL, securitydescriptor : *mut super::super::super::Win32::Security:: PSECURITY_DESCRIPTOR) -> super::super::super::Win32::Foundation:: NTSTATUS);
    unsafe { IkeextSaDbGetSecurityInfo0(enginehandle, securityinfo, sidowner as _, sidgroup as _, dacl as _, sacl as _, securitydescriptor as _) }
}
#[cfg(feature = "Win32_Security")]
#[inline]
pub unsafe fn IkeextSaDbSetSecurityInfo0(enginehandle: super::super::super::Win32::Foundation::HANDLE, securityinfo: u32, sidowner: Option<*const super::super::super::Win32::Security::SID>, sidgroup: Option<*const super::super::super::Win32::Security::SID>, dacl: Option<*const super::super::super::Win32::Security::ACL>, sacl: Option<*const super::super::super::Win32::Security::ACL>) -> super::super::super::Win32::Foundation::NTSTATUS {
    windows_core::link!("fwpuclnt.dll" "system" fn IkeextSaDbSetSecurityInfo0(enginehandle : super::super::super::Win32::Foundation:: HANDLE, securityinfo : u32, sidowner : *const super::super::super::Win32::Security:: SID, sidgroup : *const super::super::super::Win32::Security:: SID, dacl : *const super::super::super::Win32::Security:: ACL, sacl : *const super::super::super::Win32::Security:: ACL) -> super::super::super::Win32::Foundation:: NTSTATUS);
    unsafe { IkeextSaDbSetSecurityInfo0(enginehandle, securityinfo, sidowner.unwrap_or(core::mem::zeroed()) as _, sidgroup.unwrap_or(core::mem::zeroed()) as _, dacl.unwrap_or(core::mem::zeroed()) as _, sacl.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn IkeextSaDeleteById0(enginehandle: super::super::super::Win32::Foundation::HANDLE, id: u64) -> super::super::super::Win32::Foundation::NTSTATUS {
    windows_core::link!("fwpuclnt.dll" "system" fn IkeextSaDeleteById0(enginehandle : super::super::super::Win32::Foundation:: HANDLE, id : u64) -> super::super::super::Win32::Foundation:: NTSTATUS);
    unsafe { IkeextSaDeleteById0(enginehandle, id) }
}
#[inline]
pub unsafe fn IkeextSaDestroyEnumHandle0(enginehandle: super::super::super::Win32::Foundation::HANDLE, enumhandle: super::super::super::Win32::Foundation::HANDLE) -> super::super::super::Win32::Foundation::NTSTATUS {
    windows_core::link!("fwpuclnt.dll" "system" fn IkeextSaDestroyEnumHandle0(enginehandle : super::super::super::Win32::Foundation:: HANDLE, enumhandle : super::super::super::Win32::Foundation:: HANDLE) -> super::super::super::Win32::Foundation:: NTSTATUS);
    unsafe { IkeextSaDestroyEnumHandle0(enginehandle, enumhandle as _) }
}
#[cfg(feature = "Win32_NetworkManagement_WindowsFilteringPlatform")]
#[inline]
pub unsafe fn IkeextSaEnum0(enginehandle: super::super::super::Win32::Foundation::HANDLE, enumhandle: super::super::super::Win32::Foundation::HANDLE, numentriesrequested: u32, entries: *mut *mut *mut super::super::super::Win32::NetworkManagement::WindowsFilteringPlatform::IKEEXT_SA_DETAILS0, numentriesreturned: *mut u32) -> super::super::super::Win32::Foundation::NTSTATUS {
    windows_core::link!("fwpuclnt.dll" "system" fn IkeextSaEnum0(enginehandle : super::super::super::Win32::Foundation:: HANDLE, enumhandle : super::super::super::Win32::Foundation:: HANDLE, numentriesrequested : u32, entries : *mut *mut *mut super::super::super::Win32::NetworkManagement::WindowsFilteringPlatform:: IKEEXT_SA_DETAILS0, numentriesreturned : *mut u32) -> super::super::super::Win32::Foundation:: NTSTATUS);
    unsafe { IkeextSaEnum0(enginehandle, enumhandle, numentriesrequested, entries as _, numentriesreturned as _) }
}
#[cfg(feature = "Win32_NetworkManagement_WindowsFilteringPlatform")]
#[inline]
pub unsafe fn IkeextSaEnum1(enginehandle: super::super::super::Win32::Foundation::HANDLE, enumhandle: super::super::super::Win32::Foundation::HANDLE, numentriesrequested: u32, entries: *mut *mut *mut super::super::super::Win32::NetworkManagement::WindowsFilteringPlatform::IKEEXT_SA_DETAILS1, numentriesreturned: *mut u32) -> super::super::super::Win32::Foundation::NTSTATUS {
    windows_core::link!("fwpuclnt.dll" "system" fn IkeextSaEnum1(enginehandle : super::super::super::Win32::Foundation:: HANDLE, enumhandle : super::super::super::Win32::Foundation:: HANDLE, numentriesrequested : u32, entries : *mut *mut *mut super::super::super::Win32::NetworkManagement::WindowsFilteringPlatform:: IKEEXT_SA_DETAILS1, numentriesreturned : *mut u32) -> super::super::super::Win32::Foundation:: NTSTATUS);
    unsafe { IkeextSaEnum1(enginehandle, enumhandle, numentriesrequested, entries as _, numentriesreturned as _) }
}
#[cfg(feature = "Win32_NetworkManagement_WindowsFilteringPlatform")]
#[inline]
pub unsafe fn IkeextSaEnum2(enginehandle: super::super::super::Win32::Foundation::HANDLE, enumhandle: super::super::super::Win32::Foundation::HANDLE, numentriesrequested: u32, entries: *mut *mut *mut super::super::super::Win32::NetworkManagement::WindowsFilteringPlatform::IKEEXT_SA_DETAILS2, numentriesreturned: *mut u32) -> super::super::super::Win32::Foundation::NTSTATUS {
    windows_core::link!("fwpuclnt.dll" "system" fn IkeextSaEnum2(enginehandle : super::super::super::Win32::Foundation:: HANDLE, enumhandle : super::super::super::Win32::Foundation:: HANDLE, numentriesrequested : u32, entries : *mut *mut *mut super::super::super::Win32::NetworkManagement::WindowsFilteringPlatform:: IKEEXT_SA_DETAILS2, numentriesreturned : *mut u32) -> super::super::super::Win32::Foundation:: NTSTATUS);
    unsafe { IkeextSaEnum2(enginehandle, enumhandle, numentriesrequested, entries as _, numentriesreturned as _) }
}
#[cfg(feature = "Win32_NetworkManagement_WindowsFilteringPlatform")]
#[inline]
pub unsafe fn IkeextSaGetById0(enginehandle: super::super::super::Win32::Foundation::HANDLE, id: u64, sa: *mut *mut super::super::super::Win32::NetworkManagement::WindowsFilteringPlatform::IKEEXT_SA_DETAILS0) -> super::super::super::Win32::Foundation::NTSTATUS {
    windows_core::link!("fwpuclnt.dll" "system" fn IkeextSaGetById0(enginehandle : super::super::super::Win32::Foundation:: HANDLE, id : u64, sa : *mut *mut super::super::super::Win32::NetworkManagement::WindowsFilteringPlatform:: IKEEXT_SA_DETAILS0) -> super::super::super::Win32::Foundation:: NTSTATUS);
    unsafe { IkeextSaGetById0(enginehandle, id, sa as _) }
}
#[cfg(feature = "Win32_NetworkManagement_WindowsFilteringPlatform")]
#[inline]
pub unsafe fn IkeextSaGetById1(enginehandle: super::super::super::Win32::Foundation::HANDLE, id: u64, salookupcontext: Option<*const windows_core::GUID>, sa: *mut *mut super::super::super::Win32::NetworkManagement::WindowsFilteringPlatform::IKEEXT_SA_DETAILS1) -> super::super::super::Win32::Foundation::NTSTATUS {
    windows_core::link!("fwpuclnt.dll" "system" fn IkeextSaGetById1(enginehandle : super::super::super::Win32::Foundation:: HANDLE, id : u64, salookupcontext : *const windows_core::GUID, sa : *mut *mut super::super::super::Win32::NetworkManagement::WindowsFilteringPlatform:: IKEEXT_SA_DETAILS1) -> super::super::super::Win32::Foundation:: NTSTATUS);
    unsafe { IkeextSaGetById1(enginehandle, id, salookupcontext.unwrap_or(core::mem::zeroed()) as _, sa as _) }
}
#[cfg(feature = "Win32_NetworkManagement_WindowsFilteringPlatform")]
#[inline]
pub unsafe fn IkeextSaGetById2(enginehandle: super::super::super::Win32::Foundation::HANDLE, id: u64, salookupcontext: Option<*const windows_core::GUID>, sa: *mut *mut super::super::super::Win32::NetworkManagement::WindowsFilteringPlatform::IKEEXT_SA_DETAILS2) -> super::super::super::Win32::Foundation::NTSTATUS {
    windows_core::link!("fwpuclnt.dll" "system" fn IkeextSaGetById2(enginehandle : super::super::super::Win32::Foundation:: HANDLE, id : u64, salookupcontext : *const windows_core::GUID, sa : *mut *mut super::super::super::Win32::NetworkManagement::WindowsFilteringPlatform:: IKEEXT_SA_DETAILS2) -> super::super::super::Win32::Foundation:: NTSTATUS);
    unsafe { IkeextSaGetById2(enginehandle, id, salookupcontext.unwrap_or(core::mem::zeroed()) as _, sa as _) }
}
#[cfg(feature = "Win32_NetworkManagement_WindowsFilteringPlatform")]
pub type FWPM_SERVICE_STATE_CHANGE_CALLBACK0 = Option<unsafe extern "system" fn(context: *mut core::ffi::c_void, newstate: super::super::super::Win32::NetworkManagement::WindowsFilteringPlatform::FWPM_SERVICE_STATE)>;
