#[cfg(feature = "Win32_Security_AppLocker")]
pub mod AppLocker;
#[cfg(feature = "Win32_Security_Authentication")]
pub mod Authentication;
#[cfg(feature = "Win32_Security_Authorization")]
pub mod Authorization;
#[cfg(feature = "Win32_Security_ConfigurationSnapin")]
pub mod ConfigurationSnapin;
#[cfg(feature = "Win32_Security_Credentials")]
pub mod Credentials;
#[cfg(feature = "Win32_Security_Cryptography")]
pub mod Cryptography;
#[cfg(feature = "Win32_Security_DiagnosticDataQuery")]
pub mod DiagnosticDataQuery;
#[cfg(feature = "Win32_Security_DirectoryServices")]
pub mod DirectoryServices;
#[cfg(feature = "Win32_Security_EnterpriseData")]
pub mod EnterpriseData;
#[cfg(feature = "Win32_Security_ExtensibleAuthenticationProtocol")]
pub mod ExtensibleAuthenticationProtocol;
#[cfg(feature = "Win32_Security_Isolation")]
pub mod Isolation;
#[cfg(feature = "Win32_Security_LicenseProtection")]
pub mod LicenseProtection;
#[cfg(feature = "Win32_Security_NetworkAccessProtection")]
pub mod NetworkAccessProtection;
#[cfg(feature = "Win32_Security_Tpm")]
pub mod Tpm;
#[cfg(feature = "Win32_Security_WinTrust")]
pub mod WinTrust;
#[cfg(feature = "Win32_Security_WinWlx")]
pub mod WinWlx;
#[inline]
pub unsafe fn AccessCheck<P0, P1>(psecuritydescriptor: P0, clienttoken: P1, desiredaccess: u32, genericmapping: *const GENERIC_MAPPING, privilegeset: Option<*mut PRIVILEGE_SET>, privilegesetlength: *mut u32, grantedaccess: *mut u32, accessstatus: *mut super::Foundation::BOOL) -> windows_core::Result<()>
where
    P0: windows_core::Param<PSECURITY_DESCRIPTOR>,
    P1: windows_core::Param<super::Foundation::HANDLE>,
{
    windows_targets::link!("advapi32.dll" "system" fn AccessCheck(psecuritydescriptor : PSECURITY_DESCRIPTOR, clienttoken : super::Foundation:: HANDLE, desiredaccess : u32, genericmapping : *const GENERIC_MAPPING, privilegeset : *mut PRIVILEGE_SET, privilegesetlength : *mut u32, grantedaccess : *mut u32, accessstatus : *mut super::Foundation:: BOOL) -> super::Foundation:: BOOL);
    AccessCheck(psecuritydescriptor.param().abi(), clienttoken.param().abi(), desiredaccess, genericmapping, core::mem::transmute(privilegeset.unwrap_or(std::ptr::null_mut())), privilegesetlength, grantedaccess, accessstatus).ok()
}
#[inline]
pub unsafe fn AccessCheckAndAuditAlarmA<P0, P1, P2, P3, P4>(subsystemname: P0, handleid: Option<*const core::ffi::c_void>, objecttypename: P1, objectname: P2, securitydescriptor: P3, desiredaccess: u32, genericmapping: *const GENERIC_MAPPING, objectcreation: P4, grantedaccess: *mut u32, accessstatus: *mut super::Foundation::BOOL, pfgenerateonclose: *mut super::Foundation::BOOL) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P1: windows_core::Param<windows_core::PCSTR>,
    P2: windows_core::Param<windows_core::PCSTR>,
    P3: windows_core::Param<PSECURITY_DESCRIPTOR>,
    P4: windows_core::Param<super::Foundation::BOOL>,
{
    windows_targets::link!("advapi32.dll" "system" fn AccessCheckAndAuditAlarmA(subsystemname : windows_core::PCSTR, handleid : *const core::ffi::c_void, objecttypename : windows_core::PCSTR, objectname : windows_core::PCSTR, securitydescriptor : PSECURITY_DESCRIPTOR, desiredaccess : u32, genericmapping : *const GENERIC_MAPPING, objectcreation : super::Foundation:: BOOL, grantedaccess : *mut u32, accessstatus : *mut super::Foundation:: BOOL, pfgenerateonclose : *mut super::Foundation:: BOOL) -> super::Foundation:: BOOL);
    AccessCheckAndAuditAlarmA(subsystemname.param().abi(), core::mem::transmute(handleid.unwrap_or(std::ptr::null())), objecttypename.param().abi(), objectname.param().abi(), securitydescriptor.param().abi(), desiredaccess, genericmapping, objectcreation.param().abi(), grantedaccess, accessstatus, pfgenerateonclose).ok()
}
#[inline]
pub unsafe fn AccessCheckAndAuditAlarmW<P0, P1, P2, P3, P4>(subsystemname: P0, handleid: Option<*const core::ffi::c_void>, objecttypename: P1, objectname: P2, securitydescriptor: P3, desiredaccess: u32, genericmapping: *const GENERIC_MAPPING, objectcreation: P4, grantedaccess: *mut u32, accessstatus: *mut super::Foundation::BOOL, pfgenerateonclose: *mut super::Foundation::BOOL) -> super::Foundation::BOOL
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
    P3: windows_core::Param<PSECURITY_DESCRIPTOR>,
    P4: windows_core::Param<super::Foundation::BOOL>,
{
    windows_targets::link!("advapi32.dll" "system" fn AccessCheckAndAuditAlarmW(subsystemname : windows_core::PCWSTR, handleid : *const core::ffi::c_void, objecttypename : windows_core::PCWSTR, objectname : windows_core::PCWSTR, securitydescriptor : PSECURITY_DESCRIPTOR, desiredaccess : u32, genericmapping : *const GENERIC_MAPPING, objectcreation : super::Foundation:: BOOL, grantedaccess : *mut u32, accessstatus : *mut super::Foundation:: BOOL, pfgenerateonclose : *mut super::Foundation:: BOOL) -> super::Foundation:: BOOL);
    AccessCheckAndAuditAlarmW(subsystemname.param().abi(), core::mem::transmute(handleid.unwrap_or(std::ptr::null())), objecttypename.param().abi(), objectname.param().abi(), securitydescriptor.param().abi(), desiredaccess, genericmapping, objectcreation.param().abi(), grantedaccess, accessstatus, pfgenerateonclose)
}
#[inline]
pub unsafe fn AccessCheckByType<P0, P1, P2>(psecuritydescriptor: P0, principalselfsid: P1, clienttoken: P2, desiredaccess: u32, objecttypelist: Option<&mut [OBJECT_TYPE_LIST]>, genericmapping: *const GENERIC_MAPPING, privilegeset: Option<*mut PRIVILEGE_SET>, privilegesetlength: *mut u32, grantedaccess: *mut u32, accessstatus: *mut super::Foundation::BOOL) -> windows_core::Result<()>
where
    P0: windows_core::Param<PSECURITY_DESCRIPTOR>,
    P1: windows_core::Param<super::Foundation::PSID>,
    P2: windows_core::Param<super::Foundation::HANDLE>,
{
    windows_targets::link!("advapi32.dll" "system" fn AccessCheckByType(psecuritydescriptor : PSECURITY_DESCRIPTOR, principalselfsid : super::Foundation:: PSID, clienttoken : super::Foundation:: HANDLE, desiredaccess : u32, objecttypelist : *mut OBJECT_TYPE_LIST, objecttypelistlength : u32, genericmapping : *const GENERIC_MAPPING, privilegeset : *mut PRIVILEGE_SET, privilegesetlength : *mut u32, grantedaccess : *mut u32, accessstatus : *mut super::Foundation:: BOOL) -> super::Foundation:: BOOL);
    AccessCheckByType(psecuritydescriptor.param().abi(), principalselfsid.param().abi(), clienttoken.param().abi(), desiredaccess, core::mem::transmute(objecttypelist.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), objecttypelist.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), genericmapping, core::mem::transmute(privilegeset.unwrap_or(std::ptr::null_mut())), privilegesetlength, grantedaccess, accessstatus).ok()
}
#[inline]
pub unsafe fn AccessCheckByTypeAndAuditAlarmA<P0, P1, P2, P3, P4, P5>(subsystemname: P0, handleid: *const core::ffi::c_void, objecttypename: P1, objectname: P2, securitydescriptor: P3, principalselfsid: P4, desiredaccess: u32, audittype: AUDIT_EVENT_TYPE, flags: u32, objecttypelist: Option<&mut [OBJECT_TYPE_LIST]>, genericmapping: *const GENERIC_MAPPING, objectcreation: P5, grantedaccess: *mut u32, accessstatus: *mut super::Foundation::BOOL, pfgenerateonclose: *mut super::Foundation::BOOL) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P1: windows_core::Param<windows_core::PCSTR>,
    P2: windows_core::Param<windows_core::PCSTR>,
    P3: windows_core::Param<PSECURITY_DESCRIPTOR>,
    P4: windows_core::Param<super::Foundation::PSID>,
    P5: windows_core::Param<super::Foundation::BOOL>,
{
    windows_targets::link!("advapi32.dll" "system" fn AccessCheckByTypeAndAuditAlarmA(subsystemname : windows_core::PCSTR, handleid : *const core::ffi::c_void, objecttypename : windows_core::PCSTR, objectname : windows_core::PCSTR, securitydescriptor : PSECURITY_DESCRIPTOR, principalselfsid : super::Foundation:: PSID, desiredaccess : u32, audittype : AUDIT_EVENT_TYPE, flags : u32, objecttypelist : *mut OBJECT_TYPE_LIST, objecttypelistlength : u32, genericmapping : *const GENERIC_MAPPING, objectcreation : super::Foundation:: BOOL, grantedaccess : *mut u32, accessstatus : *mut super::Foundation:: BOOL, pfgenerateonclose : *mut super::Foundation:: BOOL) -> super::Foundation:: BOOL);
    AccessCheckByTypeAndAuditAlarmA(subsystemname.param().abi(), handleid, objecttypename.param().abi(), objectname.param().abi(), securitydescriptor.param().abi(), principalselfsid.param().abi(), desiredaccess, audittype, flags, core::mem::transmute(objecttypelist.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), objecttypelist.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), genericmapping, objectcreation.param().abi(), grantedaccess, accessstatus, pfgenerateonclose).ok()
}
#[inline]
pub unsafe fn AccessCheckByTypeAndAuditAlarmW<P0, P1, P2, P3, P4, P5>(subsystemname: P0, handleid: *const core::ffi::c_void, objecttypename: P1, objectname: P2, securitydescriptor: P3, principalselfsid: P4, desiredaccess: u32, audittype: AUDIT_EVENT_TYPE, flags: u32, objecttypelist: Option<&mut [OBJECT_TYPE_LIST]>, genericmapping: *const GENERIC_MAPPING, objectcreation: P5, grantedaccess: *mut u32, accessstatus: *mut super::Foundation::BOOL, pfgenerateonclose: *mut super::Foundation::BOOL) -> super::Foundation::BOOL
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
    P3: windows_core::Param<PSECURITY_DESCRIPTOR>,
    P4: windows_core::Param<super::Foundation::PSID>,
    P5: windows_core::Param<super::Foundation::BOOL>,
{
    windows_targets::link!("advapi32.dll" "system" fn AccessCheckByTypeAndAuditAlarmW(subsystemname : windows_core::PCWSTR, handleid : *const core::ffi::c_void, objecttypename : windows_core::PCWSTR, objectname : windows_core::PCWSTR, securitydescriptor : PSECURITY_DESCRIPTOR, principalselfsid : super::Foundation:: PSID, desiredaccess : u32, audittype : AUDIT_EVENT_TYPE, flags : u32, objecttypelist : *mut OBJECT_TYPE_LIST, objecttypelistlength : u32, genericmapping : *const GENERIC_MAPPING, objectcreation : super::Foundation:: BOOL, grantedaccess : *mut u32, accessstatus : *mut super::Foundation:: BOOL, pfgenerateonclose : *mut super::Foundation:: BOOL) -> super::Foundation:: BOOL);
    AccessCheckByTypeAndAuditAlarmW(subsystemname.param().abi(), handleid, objecttypename.param().abi(), objectname.param().abi(), securitydescriptor.param().abi(), principalselfsid.param().abi(), desiredaccess, audittype, flags, core::mem::transmute(objecttypelist.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), objecttypelist.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), genericmapping, objectcreation.param().abi(), grantedaccess, accessstatus, pfgenerateonclose)
}
#[inline]
pub unsafe fn AccessCheckByTypeResultList<P0, P1, P2>(psecuritydescriptor: P0, principalselfsid: P1, clienttoken: P2, desiredaccess: u32, objecttypelist: Option<*mut OBJECT_TYPE_LIST>, objecttypelistlength: u32, genericmapping: *const GENERIC_MAPPING, privilegeset: Option<*mut PRIVILEGE_SET>, privilegesetlength: *mut u32, grantedaccesslist: *mut u32, accessstatuslist: *mut u32) -> windows_core::Result<()>
where
    P0: windows_core::Param<PSECURITY_DESCRIPTOR>,
    P1: windows_core::Param<super::Foundation::PSID>,
    P2: windows_core::Param<super::Foundation::HANDLE>,
{
    windows_targets::link!("advapi32.dll" "system" fn AccessCheckByTypeResultList(psecuritydescriptor : PSECURITY_DESCRIPTOR, principalselfsid : super::Foundation:: PSID, clienttoken : super::Foundation:: HANDLE, desiredaccess : u32, objecttypelist : *mut OBJECT_TYPE_LIST, objecttypelistlength : u32, genericmapping : *const GENERIC_MAPPING, privilegeset : *mut PRIVILEGE_SET, privilegesetlength : *mut u32, grantedaccesslist : *mut u32, accessstatuslist : *mut u32) -> super::Foundation:: BOOL);
    AccessCheckByTypeResultList(psecuritydescriptor.param().abi(), principalselfsid.param().abi(), clienttoken.param().abi(), desiredaccess, core::mem::transmute(objecttypelist.unwrap_or(std::ptr::null_mut())), objecttypelistlength, genericmapping, core::mem::transmute(privilegeset.unwrap_or(std::ptr::null_mut())), privilegesetlength, grantedaccesslist, accessstatuslist).ok()
}
#[inline]
pub unsafe fn AccessCheckByTypeResultListAndAuditAlarmA<P0, P1, P2, P3, P4, P5>(subsystemname: P0, handleid: *const core::ffi::c_void, objecttypename: P1, objectname: P2, securitydescriptor: P3, principalselfsid: P4, desiredaccess: u32, audittype: AUDIT_EVENT_TYPE, flags: u32, objecttypelist: Option<*mut OBJECT_TYPE_LIST>, objecttypelistlength: u32, genericmapping: *const GENERIC_MAPPING, objectcreation: P5, grantedaccess: *mut u32, accessstatuslist: *mut u32, pfgenerateonclose: *mut super::Foundation::BOOL) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P1: windows_core::Param<windows_core::PCSTR>,
    P2: windows_core::Param<windows_core::PCSTR>,
    P3: windows_core::Param<PSECURITY_DESCRIPTOR>,
    P4: windows_core::Param<super::Foundation::PSID>,
    P5: windows_core::Param<super::Foundation::BOOL>,
{
    windows_targets::link!("advapi32.dll" "system" fn AccessCheckByTypeResultListAndAuditAlarmA(subsystemname : windows_core::PCSTR, handleid : *const core::ffi::c_void, objecttypename : windows_core::PCSTR, objectname : windows_core::PCSTR, securitydescriptor : PSECURITY_DESCRIPTOR, principalselfsid : super::Foundation:: PSID, desiredaccess : u32, audittype : AUDIT_EVENT_TYPE, flags : u32, objecttypelist : *mut OBJECT_TYPE_LIST, objecttypelistlength : u32, genericmapping : *const GENERIC_MAPPING, objectcreation : super::Foundation:: BOOL, grantedaccess : *mut u32, accessstatuslist : *mut u32, pfgenerateonclose : *mut super::Foundation:: BOOL) -> super::Foundation:: BOOL);
    AccessCheckByTypeResultListAndAuditAlarmA(subsystemname.param().abi(), handleid, objecttypename.param().abi(), objectname.param().abi(), securitydescriptor.param().abi(), principalselfsid.param().abi(), desiredaccess, audittype, flags, core::mem::transmute(objecttypelist.unwrap_or(std::ptr::null_mut())), objecttypelistlength, genericmapping, objectcreation.param().abi(), grantedaccess, accessstatuslist, pfgenerateonclose).ok()
}
#[inline]
pub unsafe fn AccessCheckByTypeResultListAndAuditAlarmByHandleA<P0, P1, P2, P3, P4, P5, P6>(subsystemname: P0, handleid: *const core::ffi::c_void, clienttoken: P1, objecttypename: P2, objectname: P3, securitydescriptor: P4, principalselfsid: P5, desiredaccess: u32, audittype: AUDIT_EVENT_TYPE, flags: u32, objecttypelist: Option<*mut OBJECT_TYPE_LIST>, objecttypelistlength: u32, genericmapping: *const GENERIC_MAPPING, objectcreation: P6, grantedaccess: *mut u32, accessstatuslist: *mut u32, pfgenerateonclose: *mut super::Foundation::BOOL) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P1: windows_core::Param<super::Foundation::HANDLE>,
    P2: windows_core::Param<windows_core::PCSTR>,
    P3: windows_core::Param<windows_core::PCSTR>,
    P4: windows_core::Param<PSECURITY_DESCRIPTOR>,
    P5: windows_core::Param<super::Foundation::PSID>,
    P6: windows_core::Param<super::Foundation::BOOL>,
{
    windows_targets::link!("advapi32.dll" "system" fn AccessCheckByTypeResultListAndAuditAlarmByHandleA(subsystemname : windows_core::PCSTR, handleid : *const core::ffi::c_void, clienttoken : super::Foundation:: HANDLE, objecttypename : windows_core::PCSTR, objectname : windows_core::PCSTR, securitydescriptor : PSECURITY_DESCRIPTOR, principalselfsid : super::Foundation:: PSID, desiredaccess : u32, audittype : AUDIT_EVENT_TYPE, flags : u32, objecttypelist : *mut OBJECT_TYPE_LIST, objecttypelistlength : u32, genericmapping : *const GENERIC_MAPPING, objectcreation : super::Foundation:: BOOL, grantedaccess : *mut u32, accessstatuslist : *mut u32, pfgenerateonclose : *mut super::Foundation:: BOOL) -> super::Foundation:: BOOL);
    AccessCheckByTypeResultListAndAuditAlarmByHandleA(subsystemname.param().abi(), handleid, clienttoken.param().abi(), objecttypename.param().abi(), objectname.param().abi(), securitydescriptor.param().abi(), principalselfsid.param().abi(), desiredaccess, audittype, flags, core::mem::transmute(objecttypelist.unwrap_or(std::ptr::null_mut())), objecttypelistlength, genericmapping, objectcreation.param().abi(), grantedaccess, accessstatuslist, pfgenerateonclose).ok()
}
#[inline]
pub unsafe fn AccessCheckByTypeResultListAndAuditAlarmByHandleW<P0, P1, P2, P3, P4, P5, P6>(subsystemname: P0, handleid: *const core::ffi::c_void, clienttoken: P1, objecttypename: P2, objectname: P3, securitydescriptor: P4, principalselfsid: P5, desiredaccess: u32, audittype: AUDIT_EVENT_TYPE, flags: u32, objecttypelist: Option<*mut OBJECT_TYPE_LIST>, objecttypelistlength: u32, genericmapping: *const GENERIC_MAPPING, objectcreation: P6, grantedaccesslist: *mut u32, accessstatuslist: *mut u32, pfgenerateonclose: *mut super::Foundation::BOOL) -> super::Foundation::BOOL
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<super::Foundation::HANDLE>,
    P2: windows_core::Param<windows_core::PCWSTR>,
    P3: windows_core::Param<windows_core::PCWSTR>,
    P4: windows_core::Param<PSECURITY_DESCRIPTOR>,
    P5: windows_core::Param<super::Foundation::PSID>,
    P6: windows_core::Param<super::Foundation::BOOL>,
{
    windows_targets::link!("advapi32.dll" "system" fn AccessCheckByTypeResultListAndAuditAlarmByHandleW(subsystemname : windows_core::PCWSTR, handleid : *const core::ffi::c_void, clienttoken : super::Foundation:: HANDLE, objecttypename : windows_core::PCWSTR, objectname : windows_core::PCWSTR, securitydescriptor : PSECURITY_DESCRIPTOR, principalselfsid : super::Foundation:: PSID, desiredaccess : u32, audittype : AUDIT_EVENT_TYPE, flags : u32, objecttypelist : *mut OBJECT_TYPE_LIST, objecttypelistlength : u32, genericmapping : *const GENERIC_MAPPING, objectcreation : super::Foundation:: BOOL, grantedaccesslist : *mut u32, accessstatuslist : *mut u32, pfgenerateonclose : *mut super::Foundation:: BOOL) -> super::Foundation:: BOOL);
    AccessCheckByTypeResultListAndAuditAlarmByHandleW(subsystemname.param().abi(), handleid, clienttoken.param().abi(), objecttypename.param().abi(), objectname.param().abi(), securitydescriptor.param().abi(), principalselfsid.param().abi(), desiredaccess, audittype, flags, core::mem::transmute(objecttypelist.unwrap_or(std::ptr::null_mut())), objecttypelistlength, genericmapping, objectcreation.param().abi(), grantedaccesslist, accessstatuslist, pfgenerateonclose)
}
#[inline]
pub unsafe fn AccessCheckByTypeResultListAndAuditAlarmW<P0, P1, P2, P3, P4, P5>(subsystemname: P0, handleid: *const core::ffi::c_void, objecttypename: P1, objectname: P2, securitydescriptor: P3, principalselfsid: P4, desiredaccess: u32, audittype: AUDIT_EVENT_TYPE, flags: u32, objecttypelist: Option<*mut OBJECT_TYPE_LIST>, objecttypelistlength: u32, genericmapping: *const GENERIC_MAPPING, objectcreation: P5, grantedaccesslist: *mut u32, accessstatuslist: *mut u32, pfgenerateonclose: *mut super::Foundation::BOOL) -> super::Foundation::BOOL
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
    P3: windows_core::Param<PSECURITY_DESCRIPTOR>,
    P4: windows_core::Param<super::Foundation::PSID>,
    P5: windows_core::Param<super::Foundation::BOOL>,
{
    windows_targets::link!("advapi32.dll" "system" fn AccessCheckByTypeResultListAndAuditAlarmW(subsystemname : windows_core::PCWSTR, handleid : *const core::ffi::c_void, objecttypename : windows_core::PCWSTR, objectname : windows_core::PCWSTR, securitydescriptor : PSECURITY_DESCRIPTOR, principalselfsid : super::Foundation:: PSID, desiredaccess : u32, audittype : AUDIT_EVENT_TYPE, flags : u32, objecttypelist : *mut OBJECT_TYPE_LIST, objecttypelistlength : u32, genericmapping : *const GENERIC_MAPPING, objectcreation : super::Foundation:: BOOL, grantedaccesslist : *mut u32, accessstatuslist : *mut u32, pfgenerateonclose : *mut super::Foundation:: BOOL) -> super::Foundation:: BOOL);
    AccessCheckByTypeResultListAndAuditAlarmW(subsystemname.param().abi(), handleid, objecttypename.param().abi(), objectname.param().abi(), securitydescriptor.param().abi(), principalselfsid.param().abi(), desiredaccess, audittype, flags, core::mem::transmute(objecttypelist.unwrap_or(std::ptr::null_mut())), objecttypelistlength, genericmapping, objectcreation.param().abi(), grantedaccesslist, accessstatuslist, pfgenerateonclose)
}
#[inline]
pub unsafe fn AddAccessAllowedAce<P0>(pacl: *mut ACL, dwacerevision: ACE_REVISION, accessmask: u32, psid: P0) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::Foundation::PSID>,
{
    windows_targets::link!("advapi32.dll" "system" fn AddAccessAllowedAce(pacl : *mut ACL, dwacerevision : ACE_REVISION, accessmask : u32, psid : super::Foundation:: PSID) -> super::Foundation:: BOOL);
    AddAccessAllowedAce(pacl, dwacerevision, accessmask, psid.param().abi()).ok()
}
#[inline]
pub unsafe fn AddAccessAllowedAceEx<P0>(pacl: *mut ACL, dwacerevision: ACE_REVISION, aceflags: ACE_FLAGS, accessmask: u32, psid: P0) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::Foundation::PSID>,
{
    windows_targets::link!("advapi32.dll" "system" fn AddAccessAllowedAceEx(pacl : *mut ACL, dwacerevision : ACE_REVISION, aceflags : ACE_FLAGS, accessmask : u32, psid : super::Foundation:: PSID) -> super::Foundation:: BOOL);
    AddAccessAllowedAceEx(pacl, dwacerevision, aceflags, accessmask, psid.param().abi()).ok()
}
#[inline]
pub unsafe fn AddAccessAllowedObjectAce<P0>(pacl: *mut ACL, dwacerevision: ACE_REVISION, aceflags: ACE_FLAGS, accessmask: u32, objecttypeguid: Option<*const windows_core::GUID>, inheritedobjecttypeguid: Option<*const windows_core::GUID>, psid: P0) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::Foundation::PSID>,
{
    windows_targets::link!("advapi32.dll" "system" fn AddAccessAllowedObjectAce(pacl : *mut ACL, dwacerevision : ACE_REVISION, aceflags : ACE_FLAGS, accessmask : u32, objecttypeguid : *const windows_core::GUID, inheritedobjecttypeguid : *const windows_core::GUID, psid : super::Foundation:: PSID) -> super::Foundation:: BOOL);
    AddAccessAllowedObjectAce(pacl, dwacerevision, aceflags, accessmask, core::mem::transmute(objecttypeguid.unwrap_or(std::ptr::null())), core::mem::transmute(inheritedobjecttypeguid.unwrap_or(std::ptr::null())), psid.param().abi()).ok()
}
#[inline]
pub unsafe fn AddAccessDeniedAce<P0>(pacl: *mut ACL, dwacerevision: ACE_REVISION, accessmask: u32, psid: P0) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::Foundation::PSID>,
{
    windows_targets::link!("advapi32.dll" "system" fn AddAccessDeniedAce(pacl : *mut ACL, dwacerevision : ACE_REVISION, accessmask : u32, psid : super::Foundation:: PSID) -> super::Foundation:: BOOL);
    AddAccessDeniedAce(pacl, dwacerevision, accessmask, psid.param().abi()).ok()
}
#[inline]
pub unsafe fn AddAccessDeniedAceEx<P0>(pacl: *mut ACL, dwacerevision: ACE_REVISION, aceflags: ACE_FLAGS, accessmask: u32, psid: P0) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::Foundation::PSID>,
{
    windows_targets::link!("advapi32.dll" "system" fn AddAccessDeniedAceEx(pacl : *mut ACL, dwacerevision : ACE_REVISION, aceflags : ACE_FLAGS, accessmask : u32, psid : super::Foundation:: PSID) -> super::Foundation:: BOOL);
    AddAccessDeniedAceEx(pacl, dwacerevision, aceflags, accessmask, psid.param().abi()).ok()
}
#[inline]
pub unsafe fn AddAccessDeniedObjectAce<P0>(pacl: *mut ACL, dwacerevision: ACE_REVISION, aceflags: ACE_FLAGS, accessmask: u32, objecttypeguid: Option<*const windows_core::GUID>, inheritedobjecttypeguid: Option<*const windows_core::GUID>, psid: P0) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::Foundation::PSID>,
{
    windows_targets::link!("advapi32.dll" "system" fn AddAccessDeniedObjectAce(pacl : *mut ACL, dwacerevision : ACE_REVISION, aceflags : ACE_FLAGS, accessmask : u32, objecttypeguid : *const windows_core::GUID, inheritedobjecttypeguid : *const windows_core::GUID, psid : super::Foundation:: PSID) -> super::Foundation:: BOOL);
    AddAccessDeniedObjectAce(pacl, dwacerevision, aceflags, accessmask, core::mem::transmute(objecttypeguid.unwrap_or(std::ptr::null())), core::mem::transmute(inheritedobjecttypeguid.unwrap_or(std::ptr::null())), psid.param().abi()).ok()
}
#[inline]
pub unsafe fn AddAce(pacl: *mut ACL, dwacerevision: ACE_REVISION, dwstartingaceindex: u32, pacelist: *const core::ffi::c_void, nacelistlength: u32) -> windows_core::Result<()> {
    windows_targets::link!("advapi32.dll" "system" fn AddAce(pacl : *mut ACL, dwacerevision : ACE_REVISION, dwstartingaceindex : u32, pacelist : *const core::ffi::c_void, nacelistlength : u32) -> super::Foundation:: BOOL);
    AddAce(pacl, dwacerevision, dwstartingaceindex, pacelist, nacelistlength).ok()
}
#[inline]
pub unsafe fn AddAuditAccessAce<P0, P1, P2>(pacl: *mut ACL, dwacerevision: ACE_REVISION, dwaccessmask: u32, psid: P0, bauditsuccess: P1, bauditfailure: P2) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::Foundation::PSID>,
    P1: windows_core::Param<super::Foundation::BOOL>,
    P2: windows_core::Param<super::Foundation::BOOL>,
{
    windows_targets::link!("advapi32.dll" "system" fn AddAuditAccessAce(pacl : *mut ACL, dwacerevision : ACE_REVISION, dwaccessmask : u32, psid : super::Foundation:: PSID, bauditsuccess : super::Foundation:: BOOL, bauditfailure : super::Foundation:: BOOL) -> super::Foundation:: BOOL);
    AddAuditAccessAce(pacl, dwacerevision, dwaccessmask, psid.param().abi(), bauditsuccess.param().abi(), bauditfailure.param().abi()).ok()
}
#[inline]
pub unsafe fn AddAuditAccessAceEx<P0, P1, P2>(pacl: *mut ACL, dwacerevision: ACE_REVISION, aceflags: ACE_FLAGS, dwaccessmask: u32, psid: P0, bauditsuccess: P1, bauditfailure: P2) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::Foundation::PSID>,
    P1: windows_core::Param<super::Foundation::BOOL>,
    P2: windows_core::Param<super::Foundation::BOOL>,
{
    windows_targets::link!("advapi32.dll" "system" fn AddAuditAccessAceEx(pacl : *mut ACL, dwacerevision : ACE_REVISION, aceflags : ACE_FLAGS, dwaccessmask : u32, psid : super::Foundation:: PSID, bauditsuccess : super::Foundation:: BOOL, bauditfailure : super::Foundation:: BOOL) -> super::Foundation:: BOOL);
    AddAuditAccessAceEx(pacl, dwacerevision, aceflags, dwaccessmask, psid.param().abi(), bauditsuccess.param().abi(), bauditfailure.param().abi()).ok()
}
#[inline]
pub unsafe fn AddAuditAccessObjectAce<P0, P1, P2>(pacl: *mut ACL, dwacerevision: ACE_REVISION, aceflags: ACE_FLAGS, accessmask: u32, objecttypeguid: Option<*const windows_core::GUID>, inheritedobjecttypeguid: Option<*const windows_core::GUID>, psid: P0, bauditsuccess: P1, bauditfailure: P2) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::Foundation::PSID>,
    P1: windows_core::Param<super::Foundation::BOOL>,
    P2: windows_core::Param<super::Foundation::BOOL>,
{
    windows_targets::link!("advapi32.dll" "system" fn AddAuditAccessObjectAce(pacl : *mut ACL, dwacerevision : ACE_REVISION, aceflags : ACE_FLAGS, accessmask : u32, objecttypeguid : *const windows_core::GUID, inheritedobjecttypeguid : *const windows_core::GUID, psid : super::Foundation:: PSID, bauditsuccess : super::Foundation:: BOOL, bauditfailure : super::Foundation:: BOOL) -> super::Foundation:: BOOL);
    AddAuditAccessObjectAce(pacl, dwacerevision, aceflags, accessmask, core::mem::transmute(objecttypeguid.unwrap_or(std::ptr::null())), core::mem::transmute(inheritedobjecttypeguid.unwrap_or(std::ptr::null())), psid.param().abi(), bauditsuccess.param().abi(), bauditfailure.param().abi()).ok()
}
#[inline]
pub unsafe fn AddConditionalAce<P0, P1>(pacl: *mut ACL, dwacerevision: ACE_REVISION, aceflags: ACE_FLAGS, acetype: u8, accessmask: u32, psid: P0, conditionstr: P1, returnlength: *mut u32) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::Foundation::PSID>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("advapi32.dll" "system" fn AddConditionalAce(pacl : *mut ACL, dwacerevision : ACE_REVISION, aceflags : ACE_FLAGS, acetype : u8, accessmask : u32, psid : super::Foundation:: PSID, conditionstr : windows_core::PCWSTR, returnlength : *mut u32) -> super::Foundation:: BOOL);
    AddConditionalAce(pacl, dwacerevision, aceflags, acetype, accessmask, psid.param().abi(), conditionstr.param().abi(), returnlength).ok()
}
#[inline]
pub unsafe fn AddMandatoryAce<P0>(pacl: *mut ACL, dwacerevision: ACE_REVISION, aceflags: ACE_FLAGS, mandatorypolicy: u32, plabelsid: P0) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::Foundation::PSID>,
{
    windows_targets::link!("advapi32.dll" "system" fn AddMandatoryAce(pacl : *mut ACL, dwacerevision : ACE_REVISION, aceflags : ACE_FLAGS, mandatorypolicy : u32, plabelsid : super::Foundation:: PSID) -> super::Foundation:: BOOL);
    AddMandatoryAce(pacl, dwacerevision, aceflags, mandatorypolicy, plabelsid.param().abi()).ok()
}
#[inline]
pub unsafe fn AddResourceAttributeAce<P0>(pacl: *mut ACL, dwacerevision: ACE_REVISION, aceflags: ACE_FLAGS, accessmask: u32, psid: P0, pattributeinfo: *const CLAIM_SECURITY_ATTRIBUTES_INFORMATION, preturnlength: *mut u32) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::Foundation::PSID>,
{
    windows_targets::link!("kernel32.dll" "system" fn AddResourceAttributeAce(pacl : *mut ACL, dwacerevision : ACE_REVISION, aceflags : ACE_FLAGS, accessmask : u32, psid : super::Foundation:: PSID, pattributeinfo : *const CLAIM_SECURITY_ATTRIBUTES_INFORMATION, preturnlength : *mut u32) -> super::Foundation:: BOOL);
    AddResourceAttributeAce(pacl, dwacerevision, aceflags, accessmask, psid.param().abi(), pattributeinfo, preturnlength).ok()
}
#[inline]
pub unsafe fn AddScopedPolicyIDAce<P0>(pacl: *mut ACL, dwacerevision: ACE_REVISION, aceflags: ACE_FLAGS, accessmask: u32, psid: P0) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::Foundation::PSID>,
{
    windows_targets::link!("kernel32.dll" "system" fn AddScopedPolicyIDAce(pacl : *mut ACL, dwacerevision : ACE_REVISION, aceflags : ACE_FLAGS, accessmask : u32, psid : super::Foundation:: PSID) -> super::Foundation:: BOOL);
    AddScopedPolicyIDAce(pacl, dwacerevision, aceflags, accessmask, psid.param().abi()).ok()
}
#[inline]
pub unsafe fn AdjustTokenGroups<P0, P1>(tokenhandle: P0, resettodefault: P1, newstate: Option<*const TOKEN_GROUPS>, bufferlength: u32, previousstate: Option<*mut TOKEN_GROUPS>, returnlength: Option<*mut u32>) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::Foundation::HANDLE>,
    P1: windows_core::Param<super::Foundation::BOOL>,
{
    windows_targets::link!("advapi32.dll" "system" fn AdjustTokenGroups(tokenhandle : super::Foundation:: HANDLE, resettodefault : super::Foundation:: BOOL, newstate : *const TOKEN_GROUPS, bufferlength : u32, previousstate : *mut TOKEN_GROUPS, returnlength : *mut u32) -> super::Foundation:: BOOL);
    AdjustTokenGroups(tokenhandle.param().abi(), resettodefault.param().abi(), core::mem::transmute(newstate.unwrap_or(std::ptr::null())), bufferlength, core::mem::transmute(previousstate.unwrap_or(std::ptr::null_mut())), core::mem::transmute(returnlength.unwrap_or(std::ptr::null_mut()))).ok()
}
#[inline]
pub unsafe fn AdjustTokenPrivileges<P0, P1>(tokenhandle: P0, disableallprivileges: P1, newstate: Option<*const TOKEN_PRIVILEGES>, bufferlength: u32, previousstate: Option<*mut TOKEN_PRIVILEGES>, returnlength: Option<*mut u32>) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::Foundation::HANDLE>,
    P1: windows_core::Param<super::Foundation::BOOL>,
{
    windows_targets::link!("advapi32.dll" "system" fn AdjustTokenPrivileges(tokenhandle : super::Foundation:: HANDLE, disableallprivileges : super::Foundation:: BOOL, newstate : *const TOKEN_PRIVILEGES, bufferlength : u32, previousstate : *mut TOKEN_PRIVILEGES, returnlength : *mut u32) -> super::Foundation:: BOOL);
    AdjustTokenPrivileges(tokenhandle.param().abi(), disableallprivileges.param().abi(), core::mem::transmute(newstate.unwrap_or(std::ptr::null())), bufferlength, core::mem::transmute(previousstate.unwrap_or(std::ptr::null_mut())), core::mem::transmute(returnlength.unwrap_or(std::ptr::null_mut()))).ok()
}
#[inline]
pub unsafe fn AllocateAndInitializeSid(pidentifierauthority: *const SID_IDENTIFIER_AUTHORITY, nsubauthoritycount: u8, nsubauthority0: u32, nsubauthority1: u32, nsubauthority2: u32, nsubauthority3: u32, nsubauthority4: u32, nsubauthority5: u32, nsubauthority6: u32, nsubauthority7: u32, psid: *mut super::Foundation::PSID) -> windows_core::Result<()> {
    windows_targets::link!("advapi32.dll" "system" fn AllocateAndInitializeSid(pidentifierauthority : *const SID_IDENTIFIER_AUTHORITY, nsubauthoritycount : u8, nsubauthority0 : u32, nsubauthority1 : u32, nsubauthority2 : u32, nsubauthority3 : u32, nsubauthority4 : u32, nsubauthority5 : u32, nsubauthority6 : u32, nsubauthority7 : u32, psid : *mut super::Foundation:: PSID) -> super::Foundation:: BOOL);
    AllocateAndInitializeSid(pidentifierauthority, nsubauthoritycount, nsubauthority0, nsubauthority1, nsubauthority2, nsubauthority3, nsubauthority4, nsubauthority5, nsubauthority6, nsubauthority7, psid).ok()
}
#[inline]
pub unsafe fn AllocateLocallyUniqueId(luid: *mut super::Foundation::LUID) -> windows_core::Result<()> {
    windows_targets::link!("advapi32.dll" "system" fn AllocateLocallyUniqueId(luid : *mut super::Foundation:: LUID) -> super::Foundation:: BOOL);
    AllocateLocallyUniqueId(luid).ok()
}
#[inline]
pub unsafe fn AreAllAccessesGranted(grantedaccess: u32, desiredaccess: u32) -> super::Foundation::BOOL {
    windows_targets::link!("advapi32.dll" "system" fn AreAllAccessesGranted(grantedaccess : u32, desiredaccess : u32) -> super::Foundation:: BOOL);
    AreAllAccessesGranted(grantedaccess, desiredaccess)
}
#[inline]
pub unsafe fn AreAnyAccessesGranted(grantedaccess: u32, desiredaccess: u32) -> super::Foundation::BOOL {
    windows_targets::link!("advapi32.dll" "system" fn AreAnyAccessesGranted(grantedaccess : u32, desiredaccess : u32) -> super::Foundation:: BOOL);
    AreAnyAccessesGranted(grantedaccess, desiredaccess)
}
#[inline]
pub unsafe fn CheckTokenCapability<P0, P1>(tokenhandle: P0, capabilitysidtocheck: P1, hascapability: *mut super::Foundation::BOOL) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::Foundation::HANDLE>,
    P1: windows_core::Param<super::Foundation::PSID>,
{
    windows_targets::link!("kernel32.dll" "system" fn CheckTokenCapability(tokenhandle : super::Foundation:: HANDLE, capabilitysidtocheck : super::Foundation:: PSID, hascapability : *mut super::Foundation:: BOOL) -> super::Foundation:: BOOL);
    CheckTokenCapability(tokenhandle.param().abi(), capabilitysidtocheck.param().abi(), hascapability).ok()
}
#[inline]
pub unsafe fn CheckTokenMembership<P0, P1>(tokenhandle: P0, sidtocheck: P1, ismember: *mut super::Foundation::BOOL) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::Foundation::HANDLE>,
    P1: windows_core::Param<super::Foundation::PSID>,
{
    windows_targets::link!("advapi32.dll" "system" fn CheckTokenMembership(tokenhandle : super::Foundation:: HANDLE, sidtocheck : super::Foundation:: PSID, ismember : *mut super::Foundation:: BOOL) -> super::Foundation:: BOOL);
    CheckTokenMembership(tokenhandle.param().abi(), sidtocheck.param().abi(), ismember).ok()
}
#[inline]
pub unsafe fn CheckTokenMembershipEx<P0, P1>(tokenhandle: P0, sidtocheck: P1, flags: u32, ismember: *mut super::Foundation::BOOL) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::Foundation::HANDLE>,
    P1: windows_core::Param<super::Foundation::PSID>,
{
    windows_targets::link!("kernel32.dll" "system" fn CheckTokenMembershipEx(tokenhandle : super::Foundation:: HANDLE, sidtocheck : super::Foundation:: PSID, flags : u32, ismember : *mut super::Foundation:: BOOL) -> super::Foundation:: BOOL);
    CheckTokenMembershipEx(tokenhandle.param().abi(), sidtocheck.param().abi(), flags, ismember).ok()
}
#[inline]
pub unsafe fn ConvertToAutoInheritPrivateObjectSecurity<P0, P1, P2>(parentdescriptor: P0, currentsecuritydescriptor: P1, newsecuritydescriptor: *mut PSECURITY_DESCRIPTOR, objecttype: Option<*const windows_core::GUID>, isdirectoryobject: P2, genericmapping: *const GENERIC_MAPPING) -> windows_core::Result<()>
where
    P0: windows_core::Param<PSECURITY_DESCRIPTOR>,
    P1: windows_core::Param<PSECURITY_DESCRIPTOR>,
    P2: windows_core::Param<super::Foundation::BOOLEAN>,
{
    windows_targets::link!("advapi32.dll" "system" fn ConvertToAutoInheritPrivateObjectSecurity(parentdescriptor : PSECURITY_DESCRIPTOR, currentsecuritydescriptor : PSECURITY_DESCRIPTOR, newsecuritydescriptor : *mut PSECURITY_DESCRIPTOR, objecttype : *const windows_core::GUID, isdirectoryobject : super::Foundation:: BOOLEAN, genericmapping : *const GENERIC_MAPPING) -> super::Foundation:: BOOL);
    ConvertToAutoInheritPrivateObjectSecurity(parentdescriptor.param().abi(), currentsecuritydescriptor.param().abi(), newsecuritydescriptor, core::mem::transmute(objecttype.unwrap_or(std::ptr::null())), isdirectoryobject.param().abi(), genericmapping).ok()
}
#[inline]
pub unsafe fn CopySid<P0>(ndestinationsidlength: u32, pdestinationsid: super::Foundation::PSID, psourcesid: P0) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::Foundation::PSID>,
{
    windows_targets::link!("advapi32.dll" "system" fn CopySid(ndestinationsidlength : u32, pdestinationsid : super::Foundation:: PSID, psourcesid : super::Foundation:: PSID) -> super::Foundation:: BOOL);
    CopySid(ndestinationsidlength, pdestinationsid, psourcesid.param().abi()).ok()
}
#[inline]
pub unsafe fn CreatePrivateObjectSecurity<P0, P1, P2, P3>(parentdescriptor: P0, creatordescriptor: P1, newdescriptor: *mut PSECURITY_DESCRIPTOR, isdirectoryobject: P2, token: P3, genericmapping: *const GENERIC_MAPPING) -> windows_core::Result<()>
where
    P0: windows_core::Param<PSECURITY_DESCRIPTOR>,
    P1: windows_core::Param<PSECURITY_DESCRIPTOR>,
    P2: windows_core::Param<super::Foundation::BOOL>,
    P3: windows_core::Param<super::Foundation::HANDLE>,
{
    windows_targets::link!("advapi32.dll" "system" fn CreatePrivateObjectSecurity(parentdescriptor : PSECURITY_DESCRIPTOR, creatordescriptor : PSECURITY_DESCRIPTOR, newdescriptor : *mut PSECURITY_DESCRIPTOR, isdirectoryobject : super::Foundation:: BOOL, token : super::Foundation:: HANDLE, genericmapping : *const GENERIC_MAPPING) -> super::Foundation:: BOOL);
    CreatePrivateObjectSecurity(parentdescriptor.param().abi(), creatordescriptor.param().abi(), newdescriptor, isdirectoryobject.param().abi(), token.param().abi(), genericmapping).ok()
}
#[inline]
pub unsafe fn CreatePrivateObjectSecurityEx<P0, P1, P2, P3>(parentdescriptor: P0, creatordescriptor: P1, newdescriptor: *mut PSECURITY_DESCRIPTOR, objecttype: Option<*const windows_core::GUID>, iscontainerobject: P2, autoinheritflags: SECURITY_AUTO_INHERIT_FLAGS, token: P3, genericmapping: *const GENERIC_MAPPING) -> windows_core::Result<()>
where
    P0: windows_core::Param<PSECURITY_DESCRIPTOR>,
    P1: windows_core::Param<PSECURITY_DESCRIPTOR>,
    P2: windows_core::Param<super::Foundation::BOOL>,
    P3: windows_core::Param<super::Foundation::HANDLE>,
{
    windows_targets::link!("advapi32.dll" "system" fn CreatePrivateObjectSecurityEx(parentdescriptor : PSECURITY_DESCRIPTOR, creatordescriptor : PSECURITY_DESCRIPTOR, newdescriptor : *mut PSECURITY_DESCRIPTOR, objecttype : *const windows_core::GUID, iscontainerobject : super::Foundation:: BOOL, autoinheritflags : SECURITY_AUTO_INHERIT_FLAGS, token : super::Foundation:: HANDLE, genericmapping : *const GENERIC_MAPPING) -> super::Foundation:: BOOL);
    CreatePrivateObjectSecurityEx(parentdescriptor.param().abi(), creatordescriptor.param().abi(), newdescriptor, core::mem::transmute(objecttype.unwrap_or(std::ptr::null())), iscontainerobject.param().abi(), autoinheritflags, token.param().abi(), genericmapping).ok()
}
#[inline]
pub unsafe fn CreatePrivateObjectSecurityWithMultipleInheritance<P0, P1, P2, P3>(parentdescriptor: P0, creatordescriptor: P1, newdescriptor: *mut PSECURITY_DESCRIPTOR, objecttypes: Option<&[*const windows_core::GUID]>, iscontainerobject: P2, autoinheritflags: SECURITY_AUTO_INHERIT_FLAGS, token: P3, genericmapping: *const GENERIC_MAPPING) -> windows_core::Result<()>
where
    P0: windows_core::Param<PSECURITY_DESCRIPTOR>,
    P1: windows_core::Param<PSECURITY_DESCRIPTOR>,
    P2: windows_core::Param<super::Foundation::BOOL>,
    P3: windows_core::Param<super::Foundation::HANDLE>,
{
    windows_targets::link!("advapi32.dll" "system" fn CreatePrivateObjectSecurityWithMultipleInheritance(parentdescriptor : PSECURITY_DESCRIPTOR, creatordescriptor : PSECURITY_DESCRIPTOR, newdescriptor : *mut PSECURITY_DESCRIPTOR, objecttypes : *const *const windows_core::GUID, guidcount : u32, iscontainerobject : super::Foundation:: BOOL, autoinheritflags : SECURITY_AUTO_INHERIT_FLAGS, token : super::Foundation:: HANDLE, genericmapping : *const GENERIC_MAPPING) -> super::Foundation:: BOOL);
    CreatePrivateObjectSecurityWithMultipleInheritance(parentdescriptor.param().abi(), creatordescriptor.param().abi(), newdescriptor, core::mem::transmute(objecttypes.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), objecttypes.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), iscontainerobject.param().abi(), autoinheritflags, token.param().abi(), genericmapping).ok()
}
#[inline]
pub unsafe fn CreateRestrictedToken<P0>(existingtokenhandle: P0, flags: CREATE_RESTRICTED_TOKEN_FLAGS, sidstodisable: Option<&[SID_AND_ATTRIBUTES]>, privilegestodelete: Option<&[LUID_AND_ATTRIBUTES]>, sidstorestrict: Option<&[SID_AND_ATTRIBUTES]>, newtokenhandle: *mut super::Foundation::HANDLE) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::Foundation::HANDLE>,
{
    windows_targets::link!("advapi32.dll" "system" fn CreateRestrictedToken(existingtokenhandle : super::Foundation:: HANDLE, flags : CREATE_RESTRICTED_TOKEN_FLAGS, disablesidcount : u32, sidstodisable : *const SID_AND_ATTRIBUTES, deleteprivilegecount : u32, privilegestodelete : *const LUID_AND_ATTRIBUTES, restrictedsidcount : u32, sidstorestrict : *const SID_AND_ATTRIBUTES, newtokenhandle : *mut super::Foundation:: HANDLE) -> super::Foundation:: BOOL);
    CreateRestrictedToken(
        existingtokenhandle.param().abi(),
        flags,
        sidstodisable.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()),
        core::mem::transmute(sidstodisable.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())),
        privilegestodelete.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()),
        core::mem::transmute(privilegestodelete.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())),
        sidstorestrict.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()),
        core::mem::transmute(sidstorestrict.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())),
        newtokenhandle,
    )
    .ok()
}
#[inline]
pub unsafe fn CreateWellKnownSid<P0>(wellknownsidtype: WELL_KNOWN_SID_TYPE, domainsid: P0, psid: super::Foundation::PSID, cbsid: *mut u32) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::Foundation::PSID>,
{
    windows_targets::link!("advapi32.dll" "system" fn CreateWellKnownSid(wellknownsidtype : WELL_KNOWN_SID_TYPE, domainsid : super::Foundation:: PSID, psid : super::Foundation:: PSID, cbsid : *mut u32) -> super::Foundation:: BOOL);
    CreateWellKnownSid(wellknownsidtype, domainsid.param().abi(), psid, cbsid).ok()
}
#[inline]
pub unsafe fn DeleteAce(pacl: *mut ACL, dwaceindex: u32) -> windows_core::Result<()> {
    windows_targets::link!("advapi32.dll" "system" fn DeleteAce(pacl : *mut ACL, dwaceindex : u32) -> super::Foundation:: BOOL);
    DeleteAce(pacl, dwaceindex).ok()
}
#[inline]
pub unsafe fn DeriveCapabilitySidsFromName<P0>(capname: P0, capabilitygroupsids: *mut *mut super::Foundation::PSID, capabilitygroupsidcount: *mut u32, capabilitysids: *mut *mut super::Foundation::PSID, capabilitysidcount: *mut u32) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("api-ms-win-security-base-l1-2-2.dll" "system" fn DeriveCapabilitySidsFromName(capname : windows_core::PCWSTR, capabilitygroupsids : *mut *mut super::Foundation:: PSID, capabilitygroupsidcount : *mut u32, capabilitysids : *mut *mut super::Foundation:: PSID, capabilitysidcount : *mut u32) -> super::Foundation:: BOOL);
    DeriveCapabilitySidsFromName(capname.param().abi(), capabilitygroupsids, capabilitygroupsidcount, capabilitysids, capabilitysidcount).ok()
}
#[inline]
pub unsafe fn DestroyPrivateObjectSecurity(objectdescriptor: *const PSECURITY_DESCRIPTOR) -> windows_core::Result<()> {
    windows_targets::link!("advapi32.dll" "system" fn DestroyPrivateObjectSecurity(objectdescriptor : *const PSECURITY_DESCRIPTOR) -> super::Foundation:: BOOL);
    DestroyPrivateObjectSecurity(objectdescriptor).ok()
}
#[inline]
pub unsafe fn DuplicateToken<P0>(existingtokenhandle: P0, impersonationlevel: SECURITY_IMPERSONATION_LEVEL, duplicatetokenhandle: *mut super::Foundation::HANDLE) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::Foundation::HANDLE>,
{
    windows_targets::link!("advapi32.dll" "system" fn DuplicateToken(existingtokenhandle : super::Foundation:: HANDLE, impersonationlevel : SECURITY_IMPERSONATION_LEVEL, duplicatetokenhandle : *mut super::Foundation:: HANDLE) -> super::Foundation:: BOOL);
    DuplicateToken(existingtokenhandle.param().abi(), impersonationlevel, duplicatetokenhandle).ok()
}
#[inline]
pub unsafe fn DuplicateTokenEx<P0>(hexistingtoken: P0, dwdesiredaccess: TOKEN_ACCESS_MASK, lptokenattributes: Option<*const SECURITY_ATTRIBUTES>, impersonationlevel: SECURITY_IMPERSONATION_LEVEL, tokentype: TOKEN_TYPE, phnewtoken: *mut super::Foundation::HANDLE) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::Foundation::HANDLE>,
{
    windows_targets::link!("advapi32.dll" "system" fn DuplicateTokenEx(hexistingtoken : super::Foundation:: HANDLE, dwdesiredaccess : TOKEN_ACCESS_MASK, lptokenattributes : *const SECURITY_ATTRIBUTES, impersonationlevel : SECURITY_IMPERSONATION_LEVEL, tokentype : TOKEN_TYPE, phnewtoken : *mut super::Foundation:: HANDLE) -> super::Foundation:: BOOL);
    DuplicateTokenEx(hexistingtoken.param().abi(), dwdesiredaccess, core::mem::transmute(lptokenattributes.unwrap_or(std::ptr::null())), impersonationlevel, tokentype, phnewtoken).ok()
}
#[inline]
pub unsafe fn EqualDomainSid<P0, P1>(psid1: P0, psid2: P1, pfequal: *mut super::Foundation::BOOL) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::Foundation::PSID>,
    P1: windows_core::Param<super::Foundation::PSID>,
{
    windows_targets::link!("advapi32.dll" "system" fn EqualDomainSid(psid1 : super::Foundation:: PSID, psid2 : super::Foundation:: PSID, pfequal : *mut super::Foundation:: BOOL) -> super::Foundation:: BOOL);
    EqualDomainSid(psid1.param().abi(), psid2.param().abi(), pfequal).ok()
}
#[inline]
pub unsafe fn EqualPrefixSid<P0, P1>(psid1: P0, psid2: P1) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::Foundation::PSID>,
    P1: windows_core::Param<super::Foundation::PSID>,
{
    windows_targets::link!("advapi32.dll" "system" fn EqualPrefixSid(psid1 : super::Foundation:: PSID, psid2 : super::Foundation:: PSID) -> super::Foundation:: BOOL);
    EqualPrefixSid(psid1.param().abi(), psid2.param().abi()).ok()
}
#[inline]
pub unsafe fn EqualSid<P0, P1>(psid1: P0, psid2: P1) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::Foundation::PSID>,
    P1: windows_core::Param<super::Foundation::PSID>,
{
    windows_targets::link!("advapi32.dll" "system" fn EqualSid(psid1 : super::Foundation:: PSID, psid2 : super::Foundation:: PSID) -> super::Foundation:: BOOL);
    EqualSid(psid1.param().abi(), psid2.param().abi()).ok()
}
#[inline]
pub unsafe fn FindFirstFreeAce(pacl: *const ACL, pace: *mut *mut core::ffi::c_void) -> windows_core::Result<()> {
    windows_targets::link!("advapi32.dll" "system" fn FindFirstFreeAce(pacl : *const ACL, pace : *mut *mut core::ffi::c_void) -> super::Foundation:: BOOL);
    FindFirstFreeAce(pacl, pace).ok()
}
#[inline]
pub unsafe fn FreeSid<P0>(psid: P0) -> *mut core::ffi::c_void
where
    P0: windows_core::Param<super::Foundation::PSID>,
{
    windows_targets::link!("advapi32.dll" "system" fn FreeSid(psid : super::Foundation:: PSID) -> *mut core::ffi::c_void);
    FreeSid(psid.param().abi())
}
#[inline]
pub unsafe fn GetAce(pacl: *const ACL, dwaceindex: u32, pace: *mut *mut core::ffi::c_void) -> windows_core::Result<()> {
    windows_targets::link!("advapi32.dll" "system" fn GetAce(pacl : *const ACL, dwaceindex : u32, pace : *mut *mut core::ffi::c_void) -> super::Foundation:: BOOL);
    GetAce(pacl, dwaceindex, pace).ok()
}
#[inline]
pub unsafe fn GetAclInformation(pacl: *const ACL, paclinformation: *mut core::ffi::c_void, naclinformationlength: u32, dwaclinformationclass: ACL_INFORMATION_CLASS) -> windows_core::Result<()> {
    windows_targets::link!("advapi32.dll" "system" fn GetAclInformation(pacl : *const ACL, paclinformation : *mut core::ffi::c_void, naclinformationlength : u32, dwaclinformationclass : ACL_INFORMATION_CLASS) -> super::Foundation:: BOOL);
    GetAclInformation(pacl, paclinformation, naclinformationlength, dwaclinformationclass).ok()
}
#[inline]
pub unsafe fn GetAppContainerAce(acl: *const ACL, startingaceindex: u32, appcontainerace: *mut *mut core::ffi::c_void, appcontaineraceindex: Option<*mut u32>) -> super::Foundation::BOOL {
    windows_targets::link!("kernel32.dll" "system" fn GetAppContainerAce(acl : *const ACL, startingaceindex : u32, appcontainerace : *mut *mut core::ffi::c_void, appcontaineraceindex : *mut u32) -> super::Foundation:: BOOL);
    GetAppContainerAce(acl, startingaceindex, appcontainerace, core::mem::transmute(appcontaineraceindex.unwrap_or(std::ptr::null_mut())))
}
#[inline]
pub unsafe fn GetCachedSigningLevel<P0>(file: P0, flags: *mut u32, signinglevel: *mut u32, thumbprint: Option<*mut u8>, thumbprintsize: Option<*mut u32>, thumbprintalgorithm: Option<*mut u32>) -> super::Foundation::BOOL
where
    P0: windows_core::Param<super::Foundation::HANDLE>,
{
    windows_targets::link!("kernel32.dll" "system" fn GetCachedSigningLevel(file : super::Foundation:: HANDLE, flags : *mut u32, signinglevel : *mut u32, thumbprint : *mut u8, thumbprintsize : *mut u32, thumbprintalgorithm : *mut u32) -> super::Foundation:: BOOL);
    GetCachedSigningLevel(file.param().abi(), flags, signinglevel, core::mem::transmute(thumbprint.unwrap_or(std::ptr::null_mut())), core::mem::transmute(thumbprintsize.unwrap_or(std::ptr::null_mut())), core::mem::transmute(thumbprintalgorithm.unwrap_or(std::ptr::null_mut())))
}
#[inline]
pub unsafe fn GetFileSecurityA<P0>(lpfilename: P0, requestedinformation: u32, psecuritydescriptor: PSECURITY_DESCRIPTOR, nlength: u32, lpnlengthneeded: *mut u32) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("advapi32.dll" "system" fn GetFileSecurityA(lpfilename : windows_core::PCSTR, requestedinformation : u32, psecuritydescriptor : PSECURITY_DESCRIPTOR, nlength : u32, lpnlengthneeded : *mut u32) -> super::Foundation:: BOOL);
    GetFileSecurityA(lpfilename.param().abi(), requestedinformation, psecuritydescriptor, nlength, lpnlengthneeded).ok()
}
#[inline]
pub unsafe fn GetFileSecurityW<P0>(lpfilename: P0, requestedinformation: u32, psecuritydescriptor: PSECURITY_DESCRIPTOR, nlength: u32, lpnlengthneeded: *mut u32) -> super::Foundation::BOOL
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("advapi32.dll" "system" fn GetFileSecurityW(lpfilename : windows_core::PCWSTR, requestedinformation : u32, psecuritydescriptor : PSECURITY_DESCRIPTOR, nlength : u32, lpnlengthneeded : *mut u32) -> super::Foundation:: BOOL);
    GetFileSecurityW(lpfilename.param().abi(), requestedinformation, psecuritydescriptor, nlength, lpnlengthneeded)
}
#[inline]
pub unsafe fn GetKernelObjectSecurity<P0>(handle: P0, requestedinformation: u32, psecuritydescriptor: PSECURITY_DESCRIPTOR, nlength: u32, lpnlengthneeded: *mut u32) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::Foundation::HANDLE>,
{
    windows_targets::link!("advapi32.dll" "system" fn GetKernelObjectSecurity(handle : super::Foundation:: HANDLE, requestedinformation : u32, psecuritydescriptor : PSECURITY_DESCRIPTOR, nlength : u32, lpnlengthneeded : *mut u32) -> super::Foundation:: BOOL);
    GetKernelObjectSecurity(handle.param().abi(), requestedinformation, psecuritydescriptor, nlength, lpnlengthneeded).ok()
}
#[inline]
pub unsafe fn GetLengthSid(psid: super::Foundation::PSID) -> u32 {
    windows_targets::link!("advapi32.dll" "system" fn GetLengthSid(psid : super::Foundation:: PSID) -> u32);
    GetLengthSid(psid)
}
#[inline]
pub unsafe fn GetPrivateObjectSecurity<P0>(objectdescriptor: P0, securityinformation: OBJECT_SECURITY_INFORMATION, resultantdescriptor: PSECURITY_DESCRIPTOR, descriptorlength: u32, returnlength: *mut u32) -> windows_core::Result<()>
where
    P0: windows_core::Param<PSECURITY_DESCRIPTOR>,
{
    windows_targets::link!("advapi32.dll" "system" fn GetPrivateObjectSecurity(objectdescriptor : PSECURITY_DESCRIPTOR, securityinformation : OBJECT_SECURITY_INFORMATION, resultantdescriptor : PSECURITY_DESCRIPTOR, descriptorlength : u32, returnlength : *mut u32) -> super::Foundation:: BOOL);
    GetPrivateObjectSecurity(objectdescriptor.param().abi(), securityinformation, resultantdescriptor, descriptorlength, returnlength).ok()
}
#[inline]
pub unsafe fn GetSecurityDescriptorControl<P0>(psecuritydescriptor: P0, pcontrol: *mut u16, lpdwrevision: *mut u32) -> windows_core::Result<()>
where
    P0: windows_core::Param<PSECURITY_DESCRIPTOR>,
{
    windows_targets::link!("advapi32.dll" "system" fn GetSecurityDescriptorControl(psecuritydescriptor : PSECURITY_DESCRIPTOR, pcontrol : *mut u16, lpdwrevision : *mut u32) -> super::Foundation:: BOOL);
    GetSecurityDescriptorControl(psecuritydescriptor.param().abi(), pcontrol, lpdwrevision).ok()
}
#[inline]
pub unsafe fn GetSecurityDescriptorDacl<P0>(psecuritydescriptor: P0, lpbdaclpresent: *mut super::Foundation::BOOL, pdacl: *mut *mut ACL, lpbdacldefaulted: *mut super::Foundation::BOOL) -> windows_core::Result<()>
where
    P0: windows_core::Param<PSECURITY_DESCRIPTOR>,
{
    windows_targets::link!("advapi32.dll" "system" fn GetSecurityDescriptorDacl(psecuritydescriptor : PSECURITY_DESCRIPTOR, lpbdaclpresent : *mut super::Foundation:: BOOL, pdacl : *mut *mut ACL, lpbdacldefaulted : *mut super::Foundation:: BOOL) -> super::Foundation:: BOOL);
    GetSecurityDescriptorDacl(psecuritydescriptor.param().abi(), lpbdaclpresent, pdacl, lpbdacldefaulted).ok()
}
#[inline]
pub unsafe fn GetSecurityDescriptorGroup<P0>(psecuritydescriptor: P0, pgroup: *mut super::Foundation::PSID, lpbgroupdefaulted: *mut super::Foundation::BOOL) -> windows_core::Result<()>
where
    P0: windows_core::Param<PSECURITY_DESCRIPTOR>,
{
    windows_targets::link!("advapi32.dll" "system" fn GetSecurityDescriptorGroup(psecuritydescriptor : PSECURITY_DESCRIPTOR, pgroup : *mut super::Foundation:: PSID, lpbgroupdefaulted : *mut super::Foundation:: BOOL) -> super::Foundation:: BOOL);
    GetSecurityDescriptorGroup(psecuritydescriptor.param().abi(), pgroup, lpbgroupdefaulted).ok()
}
#[inline]
pub unsafe fn GetSecurityDescriptorLength<P0>(psecuritydescriptor: P0) -> u32
where
    P0: windows_core::Param<PSECURITY_DESCRIPTOR>,
{
    windows_targets::link!("advapi32.dll" "system" fn GetSecurityDescriptorLength(psecuritydescriptor : PSECURITY_DESCRIPTOR) -> u32);
    GetSecurityDescriptorLength(psecuritydescriptor.param().abi())
}
#[inline]
pub unsafe fn GetSecurityDescriptorOwner<P0>(psecuritydescriptor: P0, powner: *mut super::Foundation::PSID, lpbownerdefaulted: *mut super::Foundation::BOOL) -> windows_core::Result<()>
where
    P0: windows_core::Param<PSECURITY_DESCRIPTOR>,
{
    windows_targets::link!("advapi32.dll" "system" fn GetSecurityDescriptorOwner(psecuritydescriptor : PSECURITY_DESCRIPTOR, powner : *mut super::Foundation:: PSID, lpbownerdefaulted : *mut super::Foundation:: BOOL) -> super::Foundation:: BOOL);
    GetSecurityDescriptorOwner(psecuritydescriptor.param().abi(), powner, lpbownerdefaulted).ok()
}
#[inline]
pub unsafe fn GetSecurityDescriptorRMControl<P0>(securitydescriptor: P0, rmcontrol: *mut u8) -> u32
where
    P0: windows_core::Param<PSECURITY_DESCRIPTOR>,
{
    windows_targets::link!("advapi32.dll" "system" fn GetSecurityDescriptorRMControl(securitydescriptor : PSECURITY_DESCRIPTOR, rmcontrol : *mut u8) -> u32);
    GetSecurityDescriptorRMControl(securitydescriptor.param().abi(), rmcontrol)
}
#[inline]
pub unsafe fn GetSecurityDescriptorSacl<P0>(psecuritydescriptor: P0, lpbsaclpresent: *mut super::Foundation::BOOL, psacl: *mut *mut ACL, lpbsacldefaulted: *mut super::Foundation::BOOL) -> windows_core::Result<()>
where
    P0: windows_core::Param<PSECURITY_DESCRIPTOR>,
{
    windows_targets::link!("advapi32.dll" "system" fn GetSecurityDescriptorSacl(psecuritydescriptor : PSECURITY_DESCRIPTOR, lpbsaclpresent : *mut super::Foundation:: BOOL, psacl : *mut *mut ACL, lpbsacldefaulted : *mut super::Foundation:: BOOL) -> super::Foundation:: BOOL);
    GetSecurityDescriptorSacl(psecuritydescriptor.param().abi(), lpbsaclpresent, psacl, lpbsacldefaulted).ok()
}
#[inline]
pub unsafe fn GetSidIdentifierAuthority<P0>(psid: P0) -> *mut SID_IDENTIFIER_AUTHORITY
where
    P0: windows_core::Param<super::Foundation::PSID>,
{
    windows_targets::link!("advapi32.dll" "system" fn GetSidIdentifierAuthority(psid : super::Foundation:: PSID) -> *mut SID_IDENTIFIER_AUTHORITY);
    GetSidIdentifierAuthority(psid.param().abi())
}
#[inline]
pub unsafe fn GetSidLengthRequired(nsubauthoritycount: u8) -> u32 {
    windows_targets::link!("advapi32.dll" "system" fn GetSidLengthRequired(nsubauthoritycount : u8) -> u32);
    GetSidLengthRequired(nsubauthoritycount)
}
#[inline]
pub unsafe fn GetSidSubAuthority<P0>(psid: P0, nsubauthority: u32) -> *mut u32
where
    P0: windows_core::Param<super::Foundation::PSID>,
{
    windows_targets::link!("advapi32.dll" "system" fn GetSidSubAuthority(psid : super::Foundation:: PSID, nsubauthority : u32) -> *mut u32);
    GetSidSubAuthority(psid.param().abi(), nsubauthority)
}
#[inline]
pub unsafe fn GetSidSubAuthorityCount<P0>(psid: P0) -> *mut u8
where
    P0: windows_core::Param<super::Foundation::PSID>,
{
    windows_targets::link!("advapi32.dll" "system" fn GetSidSubAuthorityCount(psid : super::Foundation:: PSID) -> *mut u8);
    GetSidSubAuthorityCount(psid.param().abi())
}
#[inline]
pub unsafe fn GetTokenInformation<P0>(tokenhandle: P0, tokeninformationclass: TOKEN_INFORMATION_CLASS, tokeninformation: Option<*mut core::ffi::c_void>, tokeninformationlength: u32, returnlength: *mut u32) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::Foundation::HANDLE>,
{
    windows_targets::link!("advapi32.dll" "system" fn GetTokenInformation(tokenhandle : super::Foundation:: HANDLE, tokeninformationclass : TOKEN_INFORMATION_CLASS, tokeninformation : *mut core::ffi::c_void, tokeninformationlength : u32, returnlength : *mut u32) -> super::Foundation:: BOOL);
    GetTokenInformation(tokenhandle.param().abi(), tokeninformationclass, core::mem::transmute(tokeninformation.unwrap_or(std::ptr::null_mut())), tokeninformationlength, returnlength).ok()
}
#[inline]
pub unsafe fn GetUserObjectSecurity<P0>(hobj: P0, psirequested: *const u32, psid: PSECURITY_DESCRIPTOR, nlength: u32, lpnlengthneeded: *mut u32) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::Foundation::HANDLE>,
{
    windows_targets::link!("user32.dll" "system" fn GetUserObjectSecurity(hobj : super::Foundation:: HANDLE, psirequested : *const u32, psid : PSECURITY_DESCRIPTOR, nlength : u32, lpnlengthneeded : *mut u32) -> super::Foundation:: BOOL);
    GetUserObjectSecurity(hobj.param().abi(), psirequested, psid, nlength, lpnlengthneeded).ok()
}
#[inline]
pub unsafe fn GetWindowsAccountDomainSid<P0>(psid: P0, pdomainsid: super::Foundation::PSID, cbdomainsid: *mut u32) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::Foundation::PSID>,
{
    windows_targets::link!("advapi32.dll" "system" fn GetWindowsAccountDomainSid(psid : super::Foundation:: PSID, pdomainsid : super::Foundation:: PSID, cbdomainsid : *mut u32) -> super::Foundation:: BOOL);
    GetWindowsAccountDomainSid(psid.param().abi(), pdomainsid, cbdomainsid).ok()
}
#[inline]
pub unsafe fn ImpersonateAnonymousToken<P0>(threadhandle: P0) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::Foundation::HANDLE>,
{
    windows_targets::link!("advapi32.dll" "system" fn ImpersonateAnonymousToken(threadhandle : super::Foundation:: HANDLE) -> super::Foundation:: BOOL);
    ImpersonateAnonymousToken(threadhandle.param().abi()).ok()
}
#[inline]
pub unsafe fn ImpersonateLoggedOnUser<P0>(htoken: P0) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::Foundation::HANDLE>,
{
    windows_targets::link!("advapi32.dll" "system" fn ImpersonateLoggedOnUser(htoken : super::Foundation:: HANDLE) -> super::Foundation:: BOOL);
    ImpersonateLoggedOnUser(htoken.param().abi()).ok()
}
#[inline]
pub unsafe fn ImpersonateSelf(impersonationlevel: SECURITY_IMPERSONATION_LEVEL) -> windows_core::Result<()> {
    windows_targets::link!("advapi32.dll" "system" fn ImpersonateSelf(impersonationlevel : SECURITY_IMPERSONATION_LEVEL) -> super::Foundation:: BOOL);
    ImpersonateSelf(impersonationlevel).ok()
}
#[inline]
pub unsafe fn InitializeAcl(pacl: *mut ACL, nacllength: u32, dwaclrevision: ACE_REVISION) -> windows_core::Result<()> {
    windows_targets::link!("advapi32.dll" "system" fn InitializeAcl(pacl : *mut ACL, nacllength : u32, dwaclrevision : ACE_REVISION) -> super::Foundation:: BOOL);
    InitializeAcl(pacl, nacllength, dwaclrevision).ok()
}
#[inline]
pub unsafe fn InitializeSecurityDescriptor(psecuritydescriptor: PSECURITY_DESCRIPTOR, dwrevision: u32) -> windows_core::Result<()> {
    windows_targets::link!("advapi32.dll" "system" fn InitializeSecurityDescriptor(psecuritydescriptor : PSECURITY_DESCRIPTOR, dwrevision : u32) -> super::Foundation:: BOOL);
    InitializeSecurityDescriptor(psecuritydescriptor, dwrevision).ok()
}
#[inline]
pub unsafe fn InitializeSid(sid: super::Foundation::PSID, pidentifierauthority: *const SID_IDENTIFIER_AUTHORITY, nsubauthoritycount: u8) -> windows_core::Result<()> {
    windows_targets::link!("advapi32.dll" "system" fn InitializeSid(sid : super::Foundation:: PSID, pidentifierauthority : *const SID_IDENTIFIER_AUTHORITY, nsubauthoritycount : u8) -> super::Foundation:: BOOL);
    InitializeSid(sid, pidentifierauthority, nsubauthoritycount).ok()
}
#[inline]
pub unsafe fn IsTokenRestricted<P0>(tokenhandle: P0) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::Foundation::HANDLE>,
{
    windows_targets::link!("advapi32.dll" "system" fn IsTokenRestricted(tokenhandle : super::Foundation:: HANDLE) -> super::Foundation:: BOOL);
    IsTokenRestricted(tokenhandle.param().abi()).ok()
}
#[inline]
pub unsafe fn IsValidAcl(pacl: *const ACL) -> super::Foundation::BOOL {
    windows_targets::link!("advapi32.dll" "system" fn IsValidAcl(pacl : *const ACL) -> super::Foundation:: BOOL);
    IsValidAcl(pacl)
}
#[inline]
pub unsafe fn IsValidSecurityDescriptor<P0>(psecuritydescriptor: P0) -> super::Foundation::BOOL
where
    P0: windows_core::Param<PSECURITY_DESCRIPTOR>,
{
    windows_targets::link!("advapi32.dll" "system" fn IsValidSecurityDescriptor(psecuritydescriptor : PSECURITY_DESCRIPTOR) -> super::Foundation:: BOOL);
    IsValidSecurityDescriptor(psecuritydescriptor.param().abi())
}
#[inline]
pub unsafe fn IsValidSid<P0>(psid: P0) -> super::Foundation::BOOL
where
    P0: windows_core::Param<super::Foundation::PSID>,
{
    windows_targets::link!("advapi32.dll" "system" fn IsValidSid(psid : super::Foundation:: PSID) -> super::Foundation:: BOOL);
    IsValidSid(psid.param().abi())
}
#[inline]
pub unsafe fn IsWellKnownSid<P0>(psid: P0, wellknownsidtype: WELL_KNOWN_SID_TYPE) -> super::Foundation::BOOL
where
    P0: windows_core::Param<super::Foundation::PSID>,
{
    windows_targets::link!("advapi32.dll" "system" fn IsWellKnownSid(psid : super::Foundation:: PSID, wellknownsidtype : WELL_KNOWN_SID_TYPE) -> super::Foundation:: BOOL);
    IsWellKnownSid(psid.param().abi(), wellknownsidtype)
}
#[inline]
pub unsafe fn LogonUserA<P0, P1, P2>(lpszusername: P0, lpszdomain: P1, lpszpassword: P2, dwlogontype: LOGON32_LOGON, dwlogonprovider: LOGON32_PROVIDER, phtoken: *mut super::Foundation::HANDLE) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P1: windows_core::Param<windows_core::PCSTR>,
    P2: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("advapi32.dll" "system" fn LogonUserA(lpszusername : windows_core::PCSTR, lpszdomain : windows_core::PCSTR, lpszpassword : windows_core::PCSTR, dwlogontype : LOGON32_LOGON, dwlogonprovider : LOGON32_PROVIDER, phtoken : *mut super::Foundation:: HANDLE) -> super::Foundation:: BOOL);
    LogonUserA(lpszusername.param().abi(), lpszdomain.param().abi(), lpszpassword.param().abi(), dwlogontype, dwlogonprovider, phtoken).ok()
}
#[inline]
pub unsafe fn LogonUserExA<P0, P1, P2>(lpszusername: P0, lpszdomain: P1, lpszpassword: P2, dwlogontype: LOGON32_LOGON, dwlogonprovider: LOGON32_PROVIDER, phtoken: Option<*mut super::Foundation::HANDLE>, pplogonsid: Option<*mut super::Foundation::PSID>, ppprofilebuffer: Option<*mut *mut core::ffi::c_void>, pdwprofilelength: Option<*mut u32>, pquotalimits: Option<*mut QUOTA_LIMITS>) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P1: windows_core::Param<windows_core::PCSTR>,
    P2: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("advapi32.dll" "system" fn LogonUserExA(lpszusername : windows_core::PCSTR, lpszdomain : windows_core::PCSTR, lpszpassword : windows_core::PCSTR, dwlogontype : LOGON32_LOGON, dwlogonprovider : LOGON32_PROVIDER, phtoken : *mut super::Foundation:: HANDLE, pplogonsid : *mut super::Foundation:: PSID, ppprofilebuffer : *mut *mut core::ffi::c_void, pdwprofilelength : *mut u32, pquotalimits : *mut QUOTA_LIMITS) -> super::Foundation:: BOOL);
    LogonUserExA(lpszusername.param().abi(), lpszdomain.param().abi(), lpszpassword.param().abi(), dwlogontype, dwlogonprovider, core::mem::transmute(phtoken.unwrap_or(std::ptr::null_mut())), core::mem::transmute(pplogonsid.unwrap_or(std::ptr::null_mut())), core::mem::transmute(ppprofilebuffer.unwrap_or(std::ptr::null_mut())), core::mem::transmute(pdwprofilelength.unwrap_or(std::ptr::null_mut())), core::mem::transmute(pquotalimits.unwrap_or(std::ptr::null_mut()))).ok()
}
#[inline]
pub unsafe fn LogonUserExW<P0, P1, P2>(lpszusername: P0, lpszdomain: P1, lpszpassword: P2, dwlogontype: LOGON32_LOGON, dwlogonprovider: LOGON32_PROVIDER, phtoken: Option<*mut super::Foundation::HANDLE>, pplogonsid: Option<*mut super::Foundation::PSID>, ppprofilebuffer: Option<*mut *mut core::ffi::c_void>, pdwprofilelength: Option<*mut u32>, pquotalimits: Option<*mut QUOTA_LIMITS>) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("advapi32.dll" "system" fn LogonUserExW(lpszusername : windows_core::PCWSTR, lpszdomain : windows_core::PCWSTR, lpszpassword : windows_core::PCWSTR, dwlogontype : LOGON32_LOGON, dwlogonprovider : LOGON32_PROVIDER, phtoken : *mut super::Foundation:: HANDLE, pplogonsid : *mut super::Foundation:: PSID, ppprofilebuffer : *mut *mut core::ffi::c_void, pdwprofilelength : *mut u32, pquotalimits : *mut QUOTA_LIMITS) -> super::Foundation:: BOOL);
    LogonUserExW(lpszusername.param().abi(), lpszdomain.param().abi(), lpszpassword.param().abi(), dwlogontype, dwlogonprovider, core::mem::transmute(phtoken.unwrap_or(std::ptr::null_mut())), core::mem::transmute(pplogonsid.unwrap_or(std::ptr::null_mut())), core::mem::transmute(ppprofilebuffer.unwrap_or(std::ptr::null_mut())), core::mem::transmute(pdwprofilelength.unwrap_or(std::ptr::null_mut())), core::mem::transmute(pquotalimits.unwrap_or(std::ptr::null_mut()))).ok()
}
#[inline]
pub unsafe fn LogonUserW<P0, P1, P2>(lpszusername: P0, lpszdomain: P1, lpszpassword: P2, dwlogontype: LOGON32_LOGON, dwlogonprovider: LOGON32_PROVIDER, phtoken: *mut super::Foundation::HANDLE) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("advapi32.dll" "system" fn LogonUserW(lpszusername : windows_core::PCWSTR, lpszdomain : windows_core::PCWSTR, lpszpassword : windows_core::PCWSTR, dwlogontype : LOGON32_LOGON, dwlogonprovider : LOGON32_PROVIDER, phtoken : *mut super::Foundation:: HANDLE) -> super::Foundation:: BOOL);
    LogonUserW(lpszusername.param().abi(), lpszdomain.param().abi(), lpszpassword.param().abi(), dwlogontype, dwlogonprovider, phtoken).ok()
}
#[inline]
pub unsafe fn LookupAccountNameA<P0, P1>(lpsystemname: P0, lpaccountname: P1, sid: super::Foundation::PSID, cbsid: *mut u32, referenceddomainname: windows_core::PSTR, cchreferenceddomainname: *mut u32, peuse: *mut SID_NAME_USE) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("advapi32.dll" "system" fn LookupAccountNameA(lpsystemname : windows_core::PCSTR, lpaccountname : windows_core::PCSTR, sid : super::Foundation:: PSID, cbsid : *mut u32, referenceddomainname : windows_core::PSTR, cchreferenceddomainname : *mut u32, peuse : *mut SID_NAME_USE) -> super::Foundation:: BOOL);
    LookupAccountNameA(lpsystemname.param().abi(), lpaccountname.param().abi(), sid, cbsid, core::mem::transmute(referenceddomainname), cchreferenceddomainname, peuse).ok()
}
#[inline]
pub unsafe fn LookupAccountNameW<P0, P1>(lpsystemname: P0, lpaccountname: P1, sid: super::Foundation::PSID, cbsid: *mut u32, referenceddomainname: windows_core::PWSTR, cchreferenceddomainname: *mut u32, peuse: *mut SID_NAME_USE) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("advapi32.dll" "system" fn LookupAccountNameW(lpsystemname : windows_core::PCWSTR, lpaccountname : windows_core::PCWSTR, sid : super::Foundation:: PSID, cbsid : *mut u32, referenceddomainname : windows_core::PWSTR, cchreferenceddomainname : *mut u32, peuse : *mut SID_NAME_USE) -> super::Foundation:: BOOL);
    LookupAccountNameW(lpsystemname.param().abi(), lpaccountname.param().abi(), sid, cbsid, core::mem::transmute(referenceddomainname), cchreferenceddomainname, peuse).ok()
}
#[inline]
pub unsafe fn LookupAccountSidA<P0, P1>(lpsystemname: P0, sid: P1, name: windows_core::PSTR, cchname: *mut u32, referenceddomainname: windows_core::PSTR, cchreferenceddomainname: *mut u32, peuse: *mut SID_NAME_USE) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P1: windows_core::Param<super::Foundation::PSID>,
{
    windows_targets::link!("advapi32.dll" "system" fn LookupAccountSidA(lpsystemname : windows_core::PCSTR, sid : super::Foundation:: PSID, name : windows_core::PSTR, cchname : *mut u32, referenceddomainname : windows_core::PSTR, cchreferenceddomainname : *mut u32, peuse : *mut SID_NAME_USE) -> super::Foundation:: BOOL);
    LookupAccountSidA(lpsystemname.param().abi(), sid.param().abi(), core::mem::transmute(name), cchname, core::mem::transmute(referenceddomainname), cchreferenceddomainname, peuse).ok()
}
#[inline]
pub unsafe fn LookupAccountSidW<P0, P1>(lpsystemname: P0, sid: P1, name: windows_core::PWSTR, cchname: *mut u32, referenceddomainname: windows_core::PWSTR, cchreferenceddomainname: *mut u32, peuse: *mut SID_NAME_USE) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<super::Foundation::PSID>,
{
    windows_targets::link!("advapi32.dll" "system" fn LookupAccountSidW(lpsystemname : windows_core::PCWSTR, sid : super::Foundation:: PSID, name : windows_core::PWSTR, cchname : *mut u32, referenceddomainname : windows_core::PWSTR, cchreferenceddomainname : *mut u32, peuse : *mut SID_NAME_USE) -> super::Foundation:: BOOL);
    LookupAccountSidW(lpsystemname.param().abi(), sid.param().abi(), core::mem::transmute(name), cchname, core::mem::transmute(referenceddomainname), cchreferenceddomainname, peuse).ok()
}
#[inline]
pub unsafe fn LookupPrivilegeDisplayNameA<P0, P1>(lpsystemname: P0, lpname: P1, lpdisplayname: windows_core::PSTR, cchdisplayname: *mut u32, lplanguageid: *mut u32) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("advapi32.dll" "system" fn LookupPrivilegeDisplayNameA(lpsystemname : windows_core::PCSTR, lpname : windows_core::PCSTR, lpdisplayname : windows_core::PSTR, cchdisplayname : *mut u32, lplanguageid : *mut u32) -> super::Foundation:: BOOL);
    LookupPrivilegeDisplayNameA(lpsystemname.param().abi(), lpname.param().abi(), core::mem::transmute(lpdisplayname), cchdisplayname, lplanguageid).ok()
}
#[inline]
pub unsafe fn LookupPrivilegeDisplayNameW<P0, P1>(lpsystemname: P0, lpname: P1, lpdisplayname: windows_core::PWSTR, cchdisplayname: *mut u32, lplanguageid: *mut u32) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("advapi32.dll" "system" fn LookupPrivilegeDisplayNameW(lpsystemname : windows_core::PCWSTR, lpname : windows_core::PCWSTR, lpdisplayname : windows_core::PWSTR, cchdisplayname : *mut u32, lplanguageid : *mut u32) -> super::Foundation:: BOOL);
    LookupPrivilegeDisplayNameW(lpsystemname.param().abi(), lpname.param().abi(), core::mem::transmute(lpdisplayname), cchdisplayname, lplanguageid).ok()
}
#[inline]
pub unsafe fn LookupPrivilegeNameA<P0>(lpsystemname: P0, lpluid: *const super::Foundation::LUID, lpname: windows_core::PSTR, cchname: *mut u32) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("advapi32.dll" "system" fn LookupPrivilegeNameA(lpsystemname : windows_core::PCSTR, lpluid : *const super::Foundation:: LUID, lpname : windows_core::PSTR, cchname : *mut u32) -> super::Foundation:: BOOL);
    LookupPrivilegeNameA(lpsystemname.param().abi(), lpluid, core::mem::transmute(lpname), cchname).ok()
}
#[inline]
pub unsafe fn LookupPrivilegeNameW<P0>(lpsystemname: P0, lpluid: *const super::Foundation::LUID, lpname: windows_core::PWSTR, cchname: *mut u32) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("advapi32.dll" "system" fn LookupPrivilegeNameW(lpsystemname : windows_core::PCWSTR, lpluid : *const super::Foundation:: LUID, lpname : windows_core::PWSTR, cchname : *mut u32) -> super::Foundation:: BOOL);
    LookupPrivilegeNameW(lpsystemname.param().abi(), lpluid, core::mem::transmute(lpname), cchname).ok()
}
#[inline]
pub unsafe fn LookupPrivilegeValueA<P0, P1>(lpsystemname: P0, lpname: P1, lpluid: *mut super::Foundation::LUID) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("advapi32.dll" "system" fn LookupPrivilegeValueA(lpsystemname : windows_core::PCSTR, lpname : windows_core::PCSTR, lpluid : *mut super::Foundation:: LUID) -> super::Foundation:: BOOL);
    LookupPrivilegeValueA(lpsystemname.param().abi(), lpname.param().abi(), lpluid).ok()
}
#[inline]
pub unsafe fn LookupPrivilegeValueW<P0, P1>(lpsystemname: P0, lpname: P1, lpluid: *mut super::Foundation::LUID) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("advapi32.dll" "system" fn LookupPrivilegeValueW(lpsystemname : windows_core::PCWSTR, lpname : windows_core::PCWSTR, lpluid : *mut super::Foundation:: LUID) -> super::Foundation:: BOOL);
    LookupPrivilegeValueW(lpsystemname.param().abi(), lpname.param().abi(), lpluid).ok()
}
#[inline]
pub unsafe fn MakeAbsoluteSD<P0>(pselfrelativesecuritydescriptor: P0, pabsolutesecuritydescriptor: PSECURITY_DESCRIPTOR, lpdwabsolutesecuritydescriptorsize: *mut u32, pdacl: Option<*mut ACL>, lpdwdaclsize: *mut u32, psacl: Option<*mut ACL>, lpdwsaclsize: *mut u32, powner: super::Foundation::PSID, lpdwownersize: *mut u32, pprimarygroup: super::Foundation::PSID, lpdwprimarygroupsize: *mut u32) -> windows_core::Result<()>
where
    P0: windows_core::Param<PSECURITY_DESCRIPTOR>,
{
    windows_targets::link!("advapi32.dll" "system" fn MakeAbsoluteSD(pselfrelativesecuritydescriptor : PSECURITY_DESCRIPTOR, pabsolutesecuritydescriptor : PSECURITY_DESCRIPTOR, lpdwabsolutesecuritydescriptorsize : *mut u32, pdacl : *mut ACL, lpdwdaclsize : *mut u32, psacl : *mut ACL, lpdwsaclsize : *mut u32, powner : super::Foundation:: PSID, lpdwownersize : *mut u32, pprimarygroup : super::Foundation:: PSID, lpdwprimarygroupsize : *mut u32) -> super::Foundation:: BOOL);
    MakeAbsoluteSD(pselfrelativesecuritydescriptor.param().abi(), pabsolutesecuritydescriptor, lpdwabsolutesecuritydescriptorsize, core::mem::transmute(pdacl.unwrap_or(std::ptr::null_mut())), lpdwdaclsize, core::mem::transmute(psacl.unwrap_or(std::ptr::null_mut())), lpdwsaclsize, powner, lpdwownersize, pprimarygroup, lpdwprimarygroupsize).ok()
}
#[inline]
pub unsafe fn MakeSelfRelativeSD<P0>(pabsolutesecuritydescriptor: P0, pselfrelativesecuritydescriptor: PSECURITY_DESCRIPTOR, lpdwbufferlength: *mut u32) -> windows_core::Result<()>
where
    P0: windows_core::Param<PSECURITY_DESCRIPTOR>,
{
    windows_targets::link!("advapi32.dll" "system" fn MakeSelfRelativeSD(pabsolutesecuritydescriptor : PSECURITY_DESCRIPTOR, pselfrelativesecuritydescriptor : PSECURITY_DESCRIPTOR, lpdwbufferlength : *mut u32) -> super::Foundation:: BOOL);
    MakeSelfRelativeSD(pabsolutesecuritydescriptor.param().abi(), pselfrelativesecuritydescriptor, lpdwbufferlength).ok()
}
#[inline]
pub unsafe fn MapGenericMask(accessmask: *mut u32, genericmapping: *const GENERIC_MAPPING) {
    windows_targets::link!("advapi32.dll" "system" fn MapGenericMask(accessmask : *mut u32, genericmapping : *const GENERIC_MAPPING));
    MapGenericMask(accessmask, genericmapping)
}
#[inline]
pub unsafe fn ObjectCloseAuditAlarmA<P0, P1>(subsystemname: P0, handleid: *const core::ffi::c_void, generateonclose: P1) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P1: windows_core::Param<super::Foundation::BOOL>,
{
    windows_targets::link!("advapi32.dll" "system" fn ObjectCloseAuditAlarmA(subsystemname : windows_core::PCSTR, handleid : *const core::ffi::c_void, generateonclose : super::Foundation:: BOOL) -> super::Foundation:: BOOL);
    ObjectCloseAuditAlarmA(subsystemname.param().abi(), handleid, generateonclose.param().abi()).ok()
}
#[inline]
pub unsafe fn ObjectCloseAuditAlarmW<P0, P1>(subsystemname: P0, handleid: *const core::ffi::c_void, generateonclose: P1) -> super::Foundation::BOOL
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<super::Foundation::BOOL>,
{
    windows_targets::link!("advapi32.dll" "system" fn ObjectCloseAuditAlarmW(subsystemname : windows_core::PCWSTR, handleid : *const core::ffi::c_void, generateonclose : super::Foundation:: BOOL) -> super::Foundation:: BOOL);
    ObjectCloseAuditAlarmW(subsystemname.param().abi(), handleid, generateonclose.param().abi())
}
#[inline]
pub unsafe fn ObjectDeleteAuditAlarmA<P0, P1>(subsystemname: P0, handleid: *const core::ffi::c_void, generateonclose: P1) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P1: windows_core::Param<super::Foundation::BOOL>,
{
    windows_targets::link!("advapi32.dll" "system" fn ObjectDeleteAuditAlarmA(subsystemname : windows_core::PCSTR, handleid : *const core::ffi::c_void, generateonclose : super::Foundation:: BOOL) -> super::Foundation:: BOOL);
    ObjectDeleteAuditAlarmA(subsystemname.param().abi(), handleid, generateonclose.param().abi()).ok()
}
#[inline]
pub unsafe fn ObjectDeleteAuditAlarmW<P0, P1>(subsystemname: P0, handleid: *const core::ffi::c_void, generateonclose: P1) -> super::Foundation::BOOL
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<super::Foundation::BOOL>,
{
    windows_targets::link!("advapi32.dll" "system" fn ObjectDeleteAuditAlarmW(subsystemname : windows_core::PCWSTR, handleid : *const core::ffi::c_void, generateonclose : super::Foundation:: BOOL) -> super::Foundation:: BOOL);
    ObjectDeleteAuditAlarmW(subsystemname.param().abi(), handleid, generateonclose.param().abi())
}
#[inline]
pub unsafe fn ObjectOpenAuditAlarmA<P0, P1, P2, P3, P4, P5, P6>(subsystemname: P0, handleid: *const core::ffi::c_void, objecttypename: P1, objectname: P2, psecuritydescriptor: P3, clienttoken: P4, desiredaccess: u32, grantedaccess: u32, privileges: Option<*const PRIVILEGE_SET>, objectcreation: P5, accessgranted: P6, generateonclose: *mut super::Foundation::BOOL) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P1: windows_core::Param<windows_core::PCSTR>,
    P2: windows_core::Param<windows_core::PCSTR>,
    P3: windows_core::Param<PSECURITY_DESCRIPTOR>,
    P4: windows_core::Param<super::Foundation::HANDLE>,
    P5: windows_core::Param<super::Foundation::BOOL>,
    P6: windows_core::Param<super::Foundation::BOOL>,
{
    windows_targets::link!("advapi32.dll" "system" fn ObjectOpenAuditAlarmA(subsystemname : windows_core::PCSTR, handleid : *const core::ffi::c_void, objecttypename : windows_core::PCSTR, objectname : windows_core::PCSTR, psecuritydescriptor : PSECURITY_DESCRIPTOR, clienttoken : super::Foundation:: HANDLE, desiredaccess : u32, grantedaccess : u32, privileges : *const PRIVILEGE_SET, objectcreation : super::Foundation:: BOOL, accessgranted : super::Foundation:: BOOL, generateonclose : *mut super::Foundation:: BOOL) -> super::Foundation:: BOOL);
    ObjectOpenAuditAlarmA(subsystemname.param().abi(), handleid, objecttypename.param().abi(), objectname.param().abi(), psecuritydescriptor.param().abi(), clienttoken.param().abi(), desiredaccess, grantedaccess, core::mem::transmute(privileges.unwrap_or(std::ptr::null())), objectcreation.param().abi(), accessgranted.param().abi(), generateonclose).ok()
}
#[inline]
pub unsafe fn ObjectOpenAuditAlarmW<P0, P1, P2, P3, P4, P5, P6>(subsystemname: P0, handleid: *const core::ffi::c_void, objecttypename: P1, objectname: P2, psecuritydescriptor: P3, clienttoken: P4, desiredaccess: u32, grantedaccess: u32, privileges: Option<*const PRIVILEGE_SET>, objectcreation: P5, accessgranted: P6, generateonclose: *mut super::Foundation::BOOL) -> super::Foundation::BOOL
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
    P3: windows_core::Param<PSECURITY_DESCRIPTOR>,
    P4: windows_core::Param<super::Foundation::HANDLE>,
    P5: windows_core::Param<super::Foundation::BOOL>,
    P6: windows_core::Param<super::Foundation::BOOL>,
{
    windows_targets::link!("advapi32.dll" "system" fn ObjectOpenAuditAlarmW(subsystemname : windows_core::PCWSTR, handleid : *const core::ffi::c_void, objecttypename : windows_core::PCWSTR, objectname : windows_core::PCWSTR, psecuritydescriptor : PSECURITY_DESCRIPTOR, clienttoken : super::Foundation:: HANDLE, desiredaccess : u32, grantedaccess : u32, privileges : *const PRIVILEGE_SET, objectcreation : super::Foundation:: BOOL, accessgranted : super::Foundation:: BOOL, generateonclose : *mut super::Foundation:: BOOL) -> super::Foundation:: BOOL);
    ObjectOpenAuditAlarmW(subsystemname.param().abi(), handleid, objecttypename.param().abi(), objectname.param().abi(), psecuritydescriptor.param().abi(), clienttoken.param().abi(), desiredaccess, grantedaccess, core::mem::transmute(privileges.unwrap_or(std::ptr::null())), objectcreation.param().abi(), accessgranted.param().abi(), generateonclose)
}
#[inline]
pub unsafe fn ObjectPrivilegeAuditAlarmA<P0, P1, P2>(subsystemname: P0, handleid: *const core::ffi::c_void, clienttoken: P1, desiredaccess: u32, privileges: *const PRIVILEGE_SET, accessgranted: P2) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P1: windows_core::Param<super::Foundation::HANDLE>,
    P2: windows_core::Param<super::Foundation::BOOL>,
{
    windows_targets::link!("advapi32.dll" "system" fn ObjectPrivilegeAuditAlarmA(subsystemname : windows_core::PCSTR, handleid : *const core::ffi::c_void, clienttoken : super::Foundation:: HANDLE, desiredaccess : u32, privileges : *const PRIVILEGE_SET, accessgranted : super::Foundation:: BOOL) -> super::Foundation:: BOOL);
    ObjectPrivilegeAuditAlarmA(subsystemname.param().abi(), handleid, clienttoken.param().abi(), desiredaccess, privileges, accessgranted.param().abi()).ok()
}
#[inline]
pub unsafe fn ObjectPrivilegeAuditAlarmW<P0, P1, P2>(subsystemname: P0, handleid: *const core::ffi::c_void, clienttoken: P1, desiredaccess: u32, privileges: *const PRIVILEGE_SET, accessgranted: P2) -> super::Foundation::BOOL
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<super::Foundation::HANDLE>,
    P2: windows_core::Param<super::Foundation::BOOL>,
{
    windows_targets::link!("advapi32.dll" "system" fn ObjectPrivilegeAuditAlarmW(subsystemname : windows_core::PCWSTR, handleid : *const core::ffi::c_void, clienttoken : super::Foundation:: HANDLE, desiredaccess : u32, privileges : *const PRIVILEGE_SET, accessgranted : super::Foundation:: BOOL) -> super::Foundation:: BOOL);
    ObjectPrivilegeAuditAlarmW(subsystemname.param().abi(), handleid, clienttoken.param().abi(), desiredaccess, privileges, accessgranted.param().abi())
}
#[inline]
pub unsafe fn PrivilegeCheck<P0>(clienttoken: P0, requiredprivileges: *mut PRIVILEGE_SET, pfresult: *mut super::Foundation::BOOL) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::Foundation::HANDLE>,
{
    windows_targets::link!("advapi32.dll" "system" fn PrivilegeCheck(clienttoken : super::Foundation:: HANDLE, requiredprivileges : *mut PRIVILEGE_SET, pfresult : *mut super::Foundation:: BOOL) -> super::Foundation:: BOOL);
    PrivilegeCheck(clienttoken.param().abi(), requiredprivileges, pfresult).ok()
}
#[inline]
pub unsafe fn PrivilegedServiceAuditAlarmA<P0, P1, P2, P3>(subsystemname: P0, servicename: P1, clienttoken: P2, privileges: *const PRIVILEGE_SET, accessgranted: P3) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P1: windows_core::Param<windows_core::PCSTR>,
    P2: windows_core::Param<super::Foundation::HANDLE>,
    P3: windows_core::Param<super::Foundation::BOOL>,
{
    windows_targets::link!("advapi32.dll" "system" fn PrivilegedServiceAuditAlarmA(subsystemname : windows_core::PCSTR, servicename : windows_core::PCSTR, clienttoken : super::Foundation:: HANDLE, privileges : *const PRIVILEGE_SET, accessgranted : super::Foundation:: BOOL) -> super::Foundation:: BOOL);
    PrivilegedServiceAuditAlarmA(subsystemname.param().abi(), servicename.param().abi(), clienttoken.param().abi(), privileges, accessgranted.param().abi()).ok()
}
#[inline]
pub unsafe fn PrivilegedServiceAuditAlarmW<P0, P1, P2, P3>(subsystemname: P0, servicename: P1, clienttoken: P2, privileges: *const PRIVILEGE_SET, accessgranted: P3) -> super::Foundation::BOOL
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<super::Foundation::HANDLE>,
    P3: windows_core::Param<super::Foundation::BOOL>,
{
    windows_targets::link!("advapi32.dll" "system" fn PrivilegedServiceAuditAlarmW(subsystemname : windows_core::PCWSTR, servicename : windows_core::PCWSTR, clienttoken : super::Foundation:: HANDLE, privileges : *const PRIVILEGE_SET, accessgranted : super::Foundation:: BOOL) -> super::Foundation:: BOOL);
    PrivilegedServiceAuditAlarmW(subsystemname.param().abi(), servicename.param().abi(), clienttoken.param().abi(), privileges, accessgranted.param().abi())
}
#[inline]
pub unsafe fn QuerySecurityAccessMask(securityinformation: OBJECT_SECURITY_INFORMATION) -> u32 {
    windows_targets::link!("advapi32.dll" "system" fn QuerySecurityAccessMask(securityinformation : OBJECT_SECURITY_INFORMATION, desiredaccess : *mut u32));
    let mut result__ = core::mem::zeroed();
    QuerySecurityAccessMask(securityinformation, &mut result__);
    result__
}
#[inline]
pub unsafe fn RevertToSelf() -> windows_core::Result<()> {
    windows_targets::link!("advapi32.dll" "system" fn RevertToSelf() -> super::Foundation:: BOOL);
    RevertToSelf().ok()
}
#[inline]
pub unsafe fn RtlConvertSidToUnicodeString<P0, P1>(unicodestring: *mut super::Foundation::UNICODE_STRING, sid: P0, allocatedestinationstring: P1) -> super::Foundation::NTSTATUS
where
    P0: windows_core::Param<super::Foundation::PSID>,
    P1: windows_core::Param<super::Foundation::BOOLEAN>,
{
    windows_targets::link!("ntdll.dll" "system" fn RtlConvertSidToUnicodeString(unicodestring : *mut super::Foundation:: UNICODE_STRING, sid : super::Foundation:: PSID, allocatedestinationstring : super::Foundation:: BOOLEAN) -> super::Foundation:: NTSTATUS);
    RtlConvertSidToUnicodeString(unicodestring, sid.param().abi(), allocatedestinationstring.param().abi())
}
#[inline]
pub unsafe fn RtlNormalizeSecurityDescriptor<P0>(securitydescriptor: *mut PSECURITY_DESCRIPTOR, securitydescriptorlength: u32, newsecuritydescriptor: Option<*mut PSECURITY_DESCRIPTOR>, newsecuritydescriptorlength: Option<*mut u32>, checkonly: P0) -> super::Foundation::BOOLEAN
where
    P0: windows_core::Param<super::Foundation::BOOLEAN>,
{
    windows_targets::link!("ntdll.dll" "system" fn RtlNormalizeSecurityDescriptor(securitydescriptor : *mut PSECURITY_DESCRIPTOR, securitydescriptorlength : u32, newsecuritydescriptor : *mut PSECURITY_DESCRIPTOR, newsecuritydescriptorlength : *mut u32, checkonly : super::Foundation:: BOOLEAN) -> super::Foundation:: BOOLEAN);
    RtlNormalizeSecurityDescriptor(securitydescriptor, securitydescriptorlength, core::mem::transmute(newsecuritydescriptor.unwrap_or(std::ptr::null_mut())), core::mem::transmute(newsecuritydescriptorlength.unwrap_or(std::ptr::null_mut())), checkonly.param().abi())
}
#[inline]
pub unsafe fn SetAclInformation(pacl: *mut ACL, paclinformation: *const core::ffi::c_void, naclinformationlength: u32, dwaclinformationclass: ACL_INFORMATION_CLASS) -> windows_core::Result<()> {
    windows_targets::link!("advapi32.dll" "system" fn SetAclInformation(pacl : *mut ACL, paclinformation : *const core::ffi::c_void, naclinformationlength : u32, dwaclinformationclass : ACL_INFORMATION_CLASS) -> super::Foundation:: BOOL);
    SetAclInformation(pacl, paclinformation, naclinformationlength, dwaclinformationclass).ok()
}
#[inline]
pub unsafe fn SetCachedSigningLevel<P0>(sourcefiles: &[super::Foundation::HANDLE], flags: u32, targetfile: P0) -> super::Foundation::BOOL
where
    P0: windows_core::Param<super::Foundation::HANDLE>,
{
    windows_targets::link!("kernel32.dll" "system" fn SetCachedSigningLevel(sourcefiles : *const super::Foundation:: HANDLE, sourcefilecount : u32, flags : u32, targetfile : super::Foundation:: HANDLE) -> super::Foundation:: BOOL);
    SetCachedSigningLevel(core::mem::transmute(sourcefiles.as_ptr()), sourcefiles.len().try_into().unwrap(), flags, targetfile.param().abi())
}
#[inline]
pub unsafe fn SetFileSecurityA<P0, P1>(lpfilename: P0, securityinformation: OBJECT_SECURITY_INFORMATION, psecuritydescriptor: P1) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P1: windows_core::Param<PSECURITY_DESCRIPTOR>,
{
    windows_targets::link!("advapi32.dll" "system" fn SetFileSecurityA(lpfilename : windows_core::PCSTR, securityinformation : OBJECT_SECURITY_INFORMATION, psecuritydescriptor : PSECURITY_DESCRIPTOR) -> super::Foundation:: BOOL);
    SetFileSecurityA(lpfilename.param().abi(), securityinformation, psecuritydescriptor.param().abi()).ok()
}
#[inline]
pub unsafe fn SetFileSecurityW<P0, P1>(lpfilename: P0, securityinformation: OBJECT_SECURITY_INFORMATION, psecuritydescriptor: P1) -> super::Foundation::BOOL
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<PSECURITY_DESCRIPTOR>,
{
    windows_targets::link!("advapi32.dll" "system" fn SetFileSecurityW(lpfilename : windows_core::PCWSTR, securityinformation : OBJECT_SECURITY_INFORMATION, psecuritydescriptor : PSECURITY_DESCRIPTOR) -> super::Foundation:: BOOL);
    SetFileSecurityW(lpfilename.param().abi(), securityinformation, psecuritydescriptor.param().abi())
}
#[inline]
pub unsafe fn SetKernelObjectSecurity<P0, P1>(handle: P0, securityinformation: OBJECT_SECURITY_INFORMATION, securitydescriptor: P1) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::Foundation::HANDLE>,
    P1: windows_core::Param<PSECURITY_DESCRIPTOR>,
{
    windows_targets::link!("advapi32.dll" "system" fn SetKernelObjectSecurity(handle : super::Foundation:: HANDLE, securityinformation : OBJECT_SECURITY_INFORMATION, securitydescriptor : PSECURITY_DESCRIPTOR) -> super::Foundation:: BOOL);
    SetKernelObjectSecurity(handle.param().abi(), securityinformation, securitydescriptor.param().abi()).ok()
}
#[inline]
pub unsafe fn SetPrivateObjectSecurity<P0, P1>(securityinformation: OBJECT_SECURITY_INFORMATION, modificationdescriptor: P0, objectssecuritydescriptor: *mut PSECURITY_DESCRIPTOR, genericmapping: *const GENERIC_MAPPING, token: P1) -> windows_core::Result<()>
where
    P0: windows_core::Param<PSECURITY_DESCRIPTOR>,
    P1: windows_core::Param<super::Foundation::HANDLE>,
{
    windows_targets::link!("advapi32.dll" "system" fn SetPrivateObjectSecurity(securityinformation : OBJECT_SECURITY_INFORMATION, modificationdescriptor : PSECURITY_DESCRIPTOR, objectssecuritydescriptor : *mut PSECURITY_DESCRIPTOR, genericmapping : *const GENERIC_MAPPING, token : super::Foundation:: HANDLE) -> super::Foundation:: BOOL);
    SetPrivateObjectSecurity(securityinformation, modificationdescriptor.param().abi(), objectssecuritydescriptor, genericmapping, token.param().abi()).ok()
}
#[inline]
pub unsafe fn SetPrivateObjectSecurityEx<P0, P1>(securityinformation: OBJECT_SECURITY_INFORMATION, modificationdescriptor: P0, objectssecuritydescriptor: *mut PSECURITY_DESCRIPTOR, autoinheritflags: SECURITY_AUTO_INHERIT_FLAGS, genericmapping: *const GENERIC_MAPPING, token: P1) -> windows_core::Result<()>
where
    P0: windows_core::Param<PSECURITY_DESCRIPTOR>,
    P1: windows_core::Param<super::Foundation::HANDLE>,
{
    windows_targets::link!("advapi32.dll" "system" fn SetPrivateObjectSecurityEx(securityinformation : OBJECT_SECURITY_INFORMATION, modificationdescriptor : PSECURITY_DESCRIPTOR, objectssecuritydescriptor : *mut PSECURITY_DESCRIPTOR, autoinheritflags : SECURITY_AUTO_INHERIT_FLAGS, genericmapping : *const GENERIC_MAPPING, token : super::Foundation:: HANDLE) -> super::Foundation:: BOOL);
    SetPrivateObjectSecurityEx(securityinformation, modificationdescriptor.param().abi(), objectssecuritydescriptor, autoinheritflags, genericmapping, token.param().abi()).ok()
}
#[inline]
pub unsafe fn SetSecurityAccessMask(securityinformation: OBJECT_SECURITY_INFORMATION) -> u32 {
    windows_targets::link!("advapi32.dll" "system" fn SetSecurityAccessMask(securityinformation : OBJECT_SECURITY_INFORMATION, desiredaccess : *mut u32));
    let mut result__ = core::mem::zeroed();
    SetSecurityAccessMask(securityinformation, &mut result__);
    result__
}
#[inline]
pub unsafe fn SetSecurityDescriptorControl<P0>(psecuritydescriptor: P0, controlbitsofinterest: SECURITY_DESCRIPTOR_CONTROL, controlbitstoset: SECURITY_DESCRIPTOR_CONTROL) -> windows_core::Result<()>
where
    P0: windows_core::Param<PSECURITY_DESCRIPTOR>,
{
    windows_targets::link!("advapi32.dll" "system" fn SetSecurityDescriptorControl(psecuritydescriptor : PSECURITY_DESCRIPTOR, controlbitsofinterest : SECURITY_DESCRIPTOR_CONTROL, controlbitstoset : SECURITY_DESCRIPTOR_CONTROL) -> super::Foundation:: BOOL);
    SetSecurityDescriptorControl(psecuritydescriptor.param().abi(), controlbitsofinterest, controlbitstoset).ok()
}
#[inline]
pub unsafe fn SetSecurityDescriptorDacl<P0, P1>(psecuritydescriptor: PSECURITY_DESCRIPTOR, bdaclpresent: P0, pdacl: Option<*const ACL>, bdacldefaulted: P1) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::Foundation::BOOL>,
    P1: windows_core::Param<super::Foundation::BOOL>,
{
    windows_targets::link!("advapi32.dll" "system" fn SetSecurityDescriptorDacl(psecuritydescriptor : PSECURITY_DESCRIPTOR, bdaclpresent : super::Foundation:: BOOL, pdacl : *const ACL, bdacldefaulted : super::Foundation:: BOOL) -> super::Foundation:: BOOL);
    SetSecurityDescriptorDacl(psecuritydescriptor, bdaclpresent.param().abi(), core::mem::transmute(pdacl.unwrap_or(std::ptr::null())), bdacldefaulted.param().abi()).ok()
}
#[inline]
pub unsafe fn SetSecurityDescriptorGroup<P0, P1>(psecuritydescriptor: PSECURITY_DESCRIPTOR, pgroup: P0, bgroupdefaulted: P1) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::Foundation::PSID>,
    P1: windows_core::Param<super::Foundation::BOOL>,
{
    windows_targets::link!("advapi32.dll" "system" fn SetSecurityDescriptorGroup(psecuritydescriptor : PSECURITY_DESCRIPTOR, pgroup : super::Foundation:: PSID, bgroupdefaulted : super::Foundation:: BOOL) -> super::Foundation:: BOOL);
    SetSecurityDescriptorGroup(psecuritydescriptor, pgroup.param().abi(), bgroupdefaulted.param().abi()).ok()
}
#[inline]
pub unsafe fn SetSecurityDescriptorOwner<P0, P1>(psecuritydescriptor: PSECURITY_DESCRIPTOR, powner: P0, bownerdefaulted: P1) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::Foundation::PSID>,
    P1: windows_core::Param<super::Foundation::BOOL>,
{
    windows_targets::link!("advapi32.dll" "system" fn SetSecurityDescriptorOwner(psecuritydescriptor : PSECURITY_DESCRIPTOR, powner : super::Foundation:: PSID, bownerdefaulted : super::Foundation:: BOOL) -> super::Foundation:: BOOL);
    SetSecurityDescriptorOwner(psecuritydescriptor, powner.param().abi(), bownerdefaulted.param().abi()).ok()
}
#[inline]
pub unsafe fn SetSecurityDescriptorRMControl(securitydescriptor: PSECURITY_DESCRIPTOR, rmcontrol: Option<*const u8>) -> u32 {
    windows_targets::link!("advapi32.dll" "system" fn SetSecurityDescriptorRMControl(securitydescriptor : PSECURITY_DESCRIPTOR, rmcontrol : *const u8) -> u32);
    SetSecurityDescriptorRMControl(securitydescriptor, core::mem::transmute(rmcontrol.unwrap_or(std::ptr::null())))
}
#[inline]
pub unsafe fn SetSecurityDescriptorSacl<P0, P1>(psecuritydescriptor: PSECURITY_DESCRIPTOR, bsaclpresent: P0, psacl: Option<*const ACL>, bsacldefaulted: P1) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::Foundation::BOOL>,
    P1: windows_core::Param<super::Foundation::BOOL>,
{
    windows_targets::link!("advapi32.dll" "system" fn SetSecurityDescriptorSacl(psecuritydescriptor : PSECURITY_DESCRIPTOR, bsaclpresent : super::Foundation:: BOOL, psacl : *const ACL, bsacldefaulted : super::Foundation:: BOOL) -> super::Foundation:: BOOL);
    SetSecurityDescriptorSacl(psecuritydescriptor, bsaclpresent.param().abi(), core::mem::transmute(psacl.unwrap_or(std::ptr::null())), bsacldefaulted.param().abi()).ok()
}
#[inline]
pub unsafe fn SetTokenInformation<P0>(tokenhandle: P0, tokeninformationclass: TOKEN_INFORMATION_CLASS, tokeninformation: *const core::ffi::c_void, tokeninformationlength: u32) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::Foundation::HANDLE>,
{
    windows_targets::link!("advapi32.dll" "system" fn SetTokenInformation(tokenhandle : super::Foundation:: HANDLE, tokeninformationclass : TOKEN_INFORMATION_CLASS, tokeninformation : *const core::ffi::c_void, tokeninformationlength : u32) -> super::Foundation:: BOOL);
    SetTokenInformation(tokenhandle.param().abi(), tokeninformationclass, tokeninformation, tokeninformationlength).ok()
}
#[inline]
pub unsafe fn SetUserObjectSecurity<P0, P1>(hobj: P0, psirequested: *const OBJECT_SECURITY_INFORMATION, psid: P1) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::Foundation::HANDLE>,
    P1: windows_core::Param<PSECURITY_DESCRIPTOR>,
{
    windows_targets::link!("user32.dll" "system" fn SetUserObjectSecurity(hobj : super::Foundation:: HANDLE, psirequested : *const OBJECT_SECURITY_INFORMATION, psid : PSECURITY_DESCRIPTOR) -> super::Foundation:: BOOL);
    SetUserObjectSecurity(hobj.param().abi(), psirequested, psid.param().abi()).ok()
}
pub const ACE_INHERITED_OBJECT_TYPE_PRESENT: SYSTEM_AUDIT_OBJECT_ACE_FLAGS = SYSTEM_AUDIT_OBJECT_ACE_FLAGS(2u32);
pub const ACE_OBJECT_TYPE_PRESENT: SYSTEM_AUDIT_OBJECT_ACE_FLAGS = SYSTEM_AUDIT_OBJECT_ACE_FLAGS(1u32);
pub const ACL_REVISION: ACE_REVISION = ACE_REVISION(2u32);
pub const ACL_REVISION_DS: ACE_REVISION = ACE_REVISION(4u32);
pub const ATTRIBUTE_SECURITY_INFORMATION: OBJECT_SECURITY_INFORMATION = OBJECT_SECURITY_INFORMATION(32u32);
pub const AclRevisionInformation: ACL_INFORMATION_CLASS = ACL_INFORMATION_CLASS(1i32);
pub const AclSizeInformation: ACL_INFORMATION_CLASS = ACL_INFORMATION_CLASS(2i32);
pub const AuditEventDirectoryServiceAccess: AUDIT_EVENT_TYPE = AUDIT_EVENT_TYPE(1i32);
pub const AuditEventObjectAccess: AUDIT_EVENT_TYPE = AUDIT_EVENT_TYPE(0i32);
pub const BACKUP_SECURITY_INFORMATION: OBJECT_SECURITY_INFORMATION = OBJECT_SECURITY_INFORMATION(65536u32);
pub const CLAIM_SECURITY_ATTRIBUTE_DISABLED: CLAIM_SECURITY_ATTRIBUTE_FLAGS = CLAIM_SECURITY_ATTRIBUTE_FLAGS(16u32);
pub const CLAIM_SECURITY_ATTRIBUTE_DISABLED_BY_DEFAULT: CLAIM_SECURITY_ATTRIBUTE_FLAGS = CLAIM_SECURITY_ATTRIBUTE_FLAGS(8u32);
pub const CLAIM_SECURITY_ATTRIBUTE_MANDATORY: CLAIM_SECURITY_ATTRIBUTE_FLAGS = CLAIM_SECURITY_ATTRIBUTE_FLAGS(32u32);
pub const CLAIM_SECURITY_ATTRIBUTE_NON_INHERITABLE: CLAIM_SECURITY_ATTRIBUTE_FLAGS = CLAIM_SECURITY_ATTRIBUTE_FLAGS(1u32);
pub const CLAIM_SECURITY_ATTRIBUTE_TYPE_BOOLEAN: CLAIM_SECURITY_ATTRIBUTE_VALUE_TYPE = CLAIM_SECURITY_ATTRIBUTE_VALUE_TYPE(6u16);
pub const CLAIM_SECURITY_ATTRIBUTE_TYPE_FQBN: CLAIM_SECURITY_ATTRIBUTE_VALUE_TYPE = CLAIM_SECURITY_ATTRIBUTE_VALUE_TYPE(4u16);
pub const CLAIM_SECURITY_ATTRIBUTE_TYPE_INT64: CLAIM_SECURITY_ATTRIBUTE_VALUE_TYPE = CLAIM_SECURITY_ATTRIBUTE_VALUE_TYPE(1u16);
pub const CLAIM_SECURITY_ATTRIBUTE_TYPE_OCTET_STRING: CLAIM_SECURITY_ATTRIBUTE_VALUE_TYPE = CLAIM_SECURITY_ATTRIBUTE_VALUE_TYPE(16u16);
pub const CLAIM_SECURITY_ATTRIBUTE_TYPE_SID: CLAIM_SECURITY_ATTRIBUTE_VALUE_TYPE = CLAIM_SECURITY_ATTRIBUTE_VALUE_TYPE(5u16);
pub const CLAIM_SECURITY_ATTRIBUTE_TYPE_STRING: CLAIM_SECURITY_ATTRIBUTE_VALUE_TYPE = CLAIM_SECURITY_ATTRIBUTE_VALUE_TYPE(3u16);
pub const CLAIM_SECURITY_ATTRIBUTE_TYPE_UINT64: CLAIM_SECURITY_ATTRIBUTE_VALUE_TYPE = CLAIM_SECURITY_ATTRIBUTE_VALUE_TYPE(2u16);
pub const CLAIM_SECURITY_ATTRIBUTE_USE_FOR_DENY_ONLY: CLAIM_SECURITY_ATTRIBUTE_FLAGS = CLAIM_SECURITY_ATTRIBUTE_FLAGS(4u32);
pub const CLAIM_SECURITY_ATTRIBUTE_VALUE_CASE_SENSITIVE: CLAIM_SECURITY_ATTRIBUTE_FLAGS = CLAIM_SECURITY_ATTRIBUTE_FLAGS(2u32);
pub const CONTAINER_INHERIT_ACE: ACE_FLAGS = ACE_FLAGS(2u32);
pub const CVT_SECONDS: u32 = 1u32;
pub const DACL_SECURITY_INFORMATION: OBJECT_SECURITY_INFORMATION = OBJECT_SECURITY_INFORMATION(4u32);
pub const DISABLE_MAX_PRIVILEGE: CREATE_RESTRICTED_TOKEN_FLAGS = CREATE_RESTRICTED_TOKEN_FLAGS(1u32);
pub const ENUM_PERIOD_DAYS: ENUM_PERIOD = ENUM_PERIOD(3i32);
pub const ENUM_PERIOD_HOURS: ENUM_PERIOD = ENUM_PERIOD(2i32);
pub const ENUM_PERIOD_INVALID: ENUM_PERIOD = ENUM_PERIOD(-1i32);
pub const ENUM_PERIOD_MINUTES: ENUM_PERIOD = ENUM_PERIOD(1i32);
pub const ENUM_PERIOD_MONTHS: ENUM_PERIOD = ENUM_PERIOD(5i32);
pub const ENUM_PERIOD_SECONDS: ENUM_PERIOD = ENUM_PERIOD(0i32);
pub const ENUM_PERIOD_WEEKS: ENUM_PERIOD = ENUM_PERIOD(4i32);
pub const ENUM_PERIOD_YEARS: ENUM_PERIOD = ENUM_PERIOD(6i32);
pub const FAILED_ACCESS_ACE_FLAG: ACE_FLAGS = ACE_FLAGS(128u32);
pub const GROUP_SECURITY_INFORMATION: OBJECT_SECURITY_INFORMATION = OBJECT_SECURITY_INFORMATION(2u32);
pub const INHERITED_ACE: ACE_FLAGS = ACE_FLAGS(16u32);
pub const INHERIT_NO_PROPAGATE: ACE_FLAGS = ACE_FLAGS(4u32);
pub const INHERIT_ONLY: ACE_FLAGS = ACE_FLAGS(8u32);
pub const INHERIT_ONLY_ACE: ACE_FLAGS = ACE_FLAGS(8u32);
pub const LABEL_SECURITY_INFORMATION: OBJECT_SECURITY_INFORMATION = OBJECT_SECURITY_INFORMATION(16u32);
pub const LOGON32_LOGON_BATCH: LOGON32_LOGON = LOGON32_LOGON(4u32);
pub const LOGON32_LOGON_INTERACTIVE: LOGON32_LOGON = LOGON32_LOGON(2u32);
pub const LOGON32_LOGON_NETWORK: LOGON32_LOGON = LOGON32_LOGON(3u32);
pub const LOGON32_LOGON_NETWORK_CLEARTEXT: LOGON32_LOGON = LOGON32_LOGON(8u32);
pub const LOGON32_LOGON_NEW_CREDENTIALS: LOGON32_LOGON = LOGON32_LOGON(9u32);
pub const LOGON32_LOGON_SERVICE: LOGON32_LOGON = LOGON32_LOGON(5u32);
pub const LOGON32_LOGON_UNLOCK: LOGON32_LOGON = LOGON32_LOGON(7u32);
pub const LOGON32_PROVIDER_DEFAULT: LOGON32_PROVIDER = LOGON32_PROVIDER(0u32);
pub const LOGON32_PROVIDER_WINNT40: LOGON32_PROVIDER = LOGON32_PROVIDER(2u32);
pub const LOGON32_PROVIDER_WINNT50: LOGON32_PROVIDER = LOGON32_PROVIDER(3u32);
pub const LUA_TOKEN: CREATE_RESTRICTED_TOKEN_FLAGS = CREATE_RESTRICTED_TOKEN_FLAGS(4u32);
pub const MandatoryLevelCount: MANDATORY_LEVEL = MANDATORY_LEVEL(6i32);
pub const MandatoryLevelHigh: MANDATORY_LEVEL = MANDATORY_LEVEL(3i32);
pub const MandatoryLevelLow: MANDATORY_LEVEL = MANDATORY_LEVEL(1i32);
pub const MandatoryLevelMedium: MANDATORY_LEVEL = MANDATORY_LEVEL(2i32);
pub const MandatoryLevelSecureProcess: MANDATORY_LEVEL = MANDATORY_LEVEL(5i32);
pub const MandatoryLevelSystem: MANDATORY_LEVEL = MANDATORY_LEVEL(4i32);
pub const MandatoryLevelUntrusted: MANDATORY_LEVEL = MANDATORY_LEVEL(0i32);
pub const MaxTokenInfoClass: TOKEN_INFORMATION_CLASS = TOKEN_INFORMATION_CLASS(49i32);
pub const NO_INHERITANCE: ACE_FLAGS = ACE_FLAGS(0u32);
pub const NO_PROPAGATE_INHERIT_ACE: ACE_FLAGS = ACE_FLAGS(4u32);
pub const OBJECT_INHERIT_ACE: ACE_FLAGS = ACE_FLAGS(1u32);
pub const OWNER_SECURITY_INFORMATION: OBJECT_SECURITY_INFORMATION = OBJECT_SECURITY_INFORMATION(1u32);
pub const PROTECTED_DACL_SECURITY_INFORMATION: OBJECT_SECURITY_INFORMATION = OBJECT_SECURITY_INFORMATION(2147483648u32);
pub const PROTECTED_SACL_SECURITY_INFORMATION: OBJECT_SECURITY_INFORMATION = OBJECT_SECURITY_INFORMATION(1073741824u32);
pub const SACL_SECURITY_INFORMATION: OBJECT_SECURITY_INFORMATION = OBJECT_SECURITY_INFORMATION(8u32);
pub const SANDBOX_INERT: CREATE_RESTRICTED_TOKEN_FLAGS = CREATE_RESTRICTED_TOKEN_FLAGS(2u32);
pub const SCOPE_SECURITY_INFORMATION: OBJECT_SECURITY_INFORMATION = OBJECT_SECURITY_INFORMATION(64u32);
pub const SECURITY_APP_PACKAGE_AUTHORITY: SID_IDENTIFIER_AUTHORITY = SID_IDENTIFIER_AUTHORITY { Value: [0, 0, 0, 0, 0, 15] };
pub const SECURITY_AUTHENTICATION_AUTHORITY: SID_IDENTIFIER_AUTHORITY = SID_IDENTIFIER_AUTHORITY { Value: [0, 0, 0, 0, 0, 18] };
pub const SECURITY_CREATOR_SID_AUTHORITY: SID_IDENTIFIER_AUTHORITY = SID_IDENTIFIER_AUTHORITY { Value: [0, 0, 0, 0, 0, 3] };
pub const SECURITY_DYNAMIC_TRACKING: super::Foundation::BOOLEAN = super::Foundation::BOOLEAN(1u8);
pub const SECURITY_LOCAL_SID_AUTHORITY: SID_IDENTIFIER_AUTHORITY = SID_IDENTIFIER_AUTHORITY { Value: [0, 0, 0, 0, 0, 2] };
pub const SECURITY_MANDATORY_LABEL_AUTHORITY: SID_IDENTIFIER_AUTHORITY = SID_IDENTIFIER_AUTHORITY { Value: [0, 0, 0, 0, 0, 16] };
pub const SECURITY_NON_UNIQUE_AUTHORITY: SID_IDENTIFIER_AUTHORITY = SID_IDENTIFIER_AUTHORITY { Value: [0, 0, 0, 0, 0, 4] };
pub const SECURITY_NT_AUTHORITY: SID_IDENTIFIER_AUTHORITY = SID_IDENTIFIER_AUTHORITY { Value: [0, 0, 0, 0, 0, 5] };
pub const SECURITY_NULL_SID_AUTHORITY: SID_IDENTIFIER_AUTHORITY = SID_IDENTIFIER_AUTHORITY { Value: [0, 0, 0, 0, 0, 0] };
pub const SECURITY_PROCESS_TRUST_AUTHORITY: SID_IDENTIFIER_AUTHORITY = SID_IDENTIFIER_AUTHORITY { Value: [0, 0, 0, 0, 0, 19] };
pub const SECURITY_RESOURCE_MANAGER_AUTHORITY: SID_IDENTIFIER_AUTHORITY = SID_IDENTIFIER_AUTHORITY { Value: [0, 0, 0, 0, 0, 9] };
pub const SECURITY_SCOPED_POLICY_ID_AUTHORITY: SID_IDENTIFIER_AUTHORITY = SID_IDENTIFIER_AUTHORITY { Value: [0, 0, 0, 0, 0, 17] };
pub const SECURITY_STATIC_TRACKING: super::Foundation::BOOLEAN = super::Foundation::BOOLEAN(0u8);
pub const SECURITY_WORLD_SID_AUTHORITY: SID_IDENTIFIER_AUTHORITY = SID_IDENTIFIER_AUTHORITY { Value: [0, 0, 0, 0, 0, 1] };
pub const SEF_AVOID_OWNER_CHECK: SECURITY_AUTO_INHERIT_FLAGS = SECURITY_AUTO_INHERIT_FLAGS(16u32);
pub const SEF_AVOID_OWNER_RESTRICTION: SECURITY_AUTO_INHERIT_FLAGS = SECURITY_AUTO_INHERIT_FLAGS(4096u32);
pub const SEF_AVOID_PRIVILEGE_CHECK: SECURITY_AUTO_INHERIT_FLAGS = SECURITY_AUTO_INHERIT_FLAGS(8u32);
pub const SEF_DACL_AUTO_INHERIT: SECURITY_AUTO_INHERIT_FLAGS = SECURITY_AUTO_INHERIT_FLAGS(1u32);
pub const SEF_DEFAULT_DESCRIPTOR_FOR_OBJECT: SECURITY_AUTO_INHERIT_FLAGS = SECURITY_AUTO_INHERIT_FLAGS(4u32);
pub const SEF_DEFAULT_GROUP_FROM_PARENT: SECURITY_AUTO_INHERIT_FLAGS = SECURITY_AUTO_INHERIT_FLAGS(64u32);
pub const SEF_DEFAULT_OWNER_FROM_PARENT: SECURITY_AUTO_INHERIT_FLAGS = SECURITY_AUTO_INHERIT_FLAGS(32u32);
pub const SEF_MACL_NO_EXECUTE_UP: SECURITY_AUTO_INHERIT_FLAGS = SECURITY_AUTO_INHERIT_FLAGS(1024u32);
pub const SEF_MACL_NO_READ_UP: SECURITY_AUTO_INHERIT_FLAGS = SECURITY_AUTO_INHERIT_FLAGS(512u32);
pub const SEF_MACL_NO_WRITE_UP: SECURITY_AUTO_INHERIT_FLAGS = SECURITY_AUTO_INHERIT_FLAGS(256u32);
pub const SEF_SACL_AUTO_INHERIT: SECURITY_AUTO_INHERIT_FLAGS = SECURITY_AUTO_INHERIT_FLAGS(2u32);
pub const SE_ASSIGNPRIMARYTOKEN_NAME: windows_core::PCWSTR = windows_core::w!("SeAssignPrimaryTokenPrivilege");
pub const SE_AUDIT_NAME: windows_core::PCWSTR = windows_core::w!("SeAuditPrivilege");
pub const SE_BACKUP_NAME: windows_core::PCWSTR = windows_core::w!("SeBackupPrivilege");
pub const SE_CHANGE_NOTIFY_NAME: windows_core::PCWSTR = windows_core::w!("SeChangeNotifyPrivilege");
pub const SE_CREATE_GLOBAL_NAME: windows_core::PCWSTR = windows_core::w!("SeCreateGlobalPrivilege");
pub const SE_CREATE_PAGEFILE_NAME: windows_core::PCWSTR = windows_core::w!("SeCreatePagefilePrivilege");
pub const SE_CREATE_PERMANENT_NAME: windows_core::PCWSTR = windows_core::w!("SeCreatePermanentPrivilege");
pub const SE_CREATE_SYMBOLIC_LINK_NAME: windows_core::PCWSTR = windows_core::w!("SeCreateSymbolicLinkPrivilege");
pub const SE_CREATE_TOKEN_NAME: windows_core::PCWSTR = windows_core::w!("SeCreateTokenPrivilege");
pub const SE_DACL_AUTO_INHERITED: SECURITY_DESCRIPTOR_CONTROL = SECURITY_DESCRIPTOR_CONTROL(1024u16);
pub const SE_DACL_AUTO_INHERIT_REQ: SECURITY_DESCRIPTOR_CONTROL = SECURITY_DESCRIPTOR_CONTROL(256u16);
pub const SE_DACL_DEFAULTED: SECURITY_DESCRIPTOR_CONTROL = SECURITY_DESCRIPTOR_CONTROL(8u16);
pub const SE_DACL_PRESENT: SECURITY_DESCRIPTOR_CONTROL = SECURITY_DESCRIPTOR_CONTROL(4u16);
pub const SE_DACL_PROTECTED: SECURITY_DESCRIPTOR_CONTROL = SECURITY_DESCRIPTOR_CONTROL(4096u16);
pub const SE_DEBUG_NAME: windows_core::PCWSTR = windows_core::w!("SeDebugPrivilege");
pub const SE_DELEGATE_SESSION_USER_IMPERSONATE_NAME: windows_core::PCWSTR = windows_core::w!("SeDelegateSessionUserImpersonatePrivilege");
pub const SE_ENABLE_DELEGATION_NAME: windows_core::PCWSTR = windows_core::w!("SeEnableDelegationPrivilege");
pub const SE_GROUP_DEFAULTED: SECURITY_DESCRIPTOR_CONTROL = SECURITY_DESCRIPTOR_CONTROL(2u16);
pub const SE_IMPERSONATE_NAME: windows_core::PCWSTR = windows_core::w!("SeImpersonatePrivilege");
pub const SE_INCREASE_QUOTA_NAME: windows_core::PCWSTR = windows_core::w!("SeIncreaseQuotaPrivilege");
pub const SE_INC_BASE_PRIORITY_NAME: windows_core::PCWSTR = windows_core::w!("SeIncreaseBasePriorityPrivilege");
pub const SE_INC_WORKING_SET_NAME: windows_core::PCWSTR = windows_core::w!("SeIncreaseWorkingSetPrivilege");
pub const SE_LOAD_DRIVER_NAME: windows_core::PCWSTR = windows_core::w!("SeLoadDriverPrivilege");
pub const SE_LOCK_MEMORY_NAME: windows_core::PCWSTR = windows_core::w!("SeLockMemoryPrivilege");
pub const SE_MACHINE_ACCOUNT_NAME: windows_core::PCWSTR = windows_core::w!("SeMachineAccountPrivilege");
pub const SE_MANAGE_VOLUME_NAME: windows_core::PCWSTR = windows_core::w!("SeManageVolumePrivilege");
pub const SE_OWNER_DEFAULTED: SECURITY_DESCRIPTOR_CONTROL = SECURITY_DESCRIPTOR_CONTROL(1u16);
pub const SE_PRIVILEGE_ENABLED: TOKEN_PRIVILEGES_ATTRIBUTES = TOKEN_PRIVILEGES_ATTRIBUTES(2u32);
pub const SE_PRIVILEGE_ENABLED_BY_DEFAULT: TOKEN_PRIVILEGES_ATTRIBUTES = TOKEN_PRIVILEGES_ATTRIBUTES(1u32);
pub const SE_PRIVILEGE_REMOVED: TOKEN_PRIVILEGES_ATTRIBUTES = TOKEN_PRIVILEGES_ATTRIBUTES(4u32);
pub const SE_PRIVILEGE_USED_FOR_ACCESS: TOKEN_PRIVILEGES_ATTRIBUTES = TOKEN_PRIVILEGES_ATTRIBUTES(2147483648u32);
pub const SE_PROF_SINGLE_PROCESS_NAME: windows_core::PCWSTR = windows_core::w!("SeProfileSingleProcessPrivilege");
pub const SE_RELABEL_NAME: windows_core::PCWSTR = windows_core::w!("SeRelabelPrivilege");
pub const SE_REMOTE_SHUTDOWN_NAME: windows_core::PCWSTR = windows_core::w!("SeRemoteShutdownPrivilege");
pub const SE_RESTORE_NAME: windows_core::PCWSTR = windows_core::w!("SeRestorePrivilege");
pub const SE_RM_CONTROL_VALID: SECURITY_DESCRIPTOR_CONTROL = SECURITY_DESCRIPTOR_CONTROL(16384u16);
pub const SE_SACL_AUTO_INHERITED: SECURITY_DESCRIPTOR_CONTROL = SECURITY_DESCRIPTOR_CONTROL(2048u16);
pub const SE_SACL_AUTO_INHERIT_REQ: SECURITY_DESCRIPTOR_CONTROL = SECURITY_DESCRIPTOR_CONTROL(512u16);
pub const SE_SACL_DEFAULTED: SECURITY_DESCRIPTOR_CONTROL = SECURITY_DESCRIPTOR_CONTROL(32u16);
pub const SE_SACL_PRESENT: SECURITY_DESCRIPTOR_CONTROL = SECURITY_DESCRIPTOR_CONTROL(16u16);
pub const SE_SACL_PROTECTED: SECURITY_DESCRIPTOR_CONTROL = SECURITY_DESCRIPTOR_CONTROL(8192u16);
pub const SE_SECURITY_NAME: windows_core::PCWSTR = windows_core::w!("SeSecurityPrivilege");
pub const SE_SELF_RELATIVE: SECURITY_DESCRIPTOR_CONTROL = SECURITY_DESCRIPTOR_CONTROL(32768u16);
pub const SE_SHUTDOWN_NAME: windows_core::PCWSTR = windows_core::w!("SeShutdownPrivilege");
pub const SE_SYNC_AGENT_NAME: windows_core::PCWSTR = windows_core::w!("SeSyncAgentPrivilege");
pub const SE_SYSTEMTIME_NAME: windows_core::PCWSTR = windows_core::w!("SeSystemtimePrivilege");
pub const SE_SYSTEM_ENVIRONMENT_NAME: windows_core::PCWSTR = windows_core::w!("SeSystemEnvironmentPrivilege");
pub const SE_SYSTEM_PROFILE_NAME: windows_core::PCWSTR = windows_core::w!("SeSystemProfilePrivilege");
pub const SE_TAKE_OWNERSHIP_NAME: windows_core::PCWSTR = windows_core::w!("SeTakeOwnershipPrivilege");
pub const SE_TCB_NAME: windows_core::PCWSTR = windows_core::w!("SeTcbPrivilege");
pub const SE_TIME_ZONE_NAME: windows_core::PCWSTR = windows_core::w!("SeTimeZonePrivilege");
pub const SE_TRUSTED_CREDMAN_ACCESS_NAME: windows_core::PCWSTR = windows_core::w!("SeTrustedCredManAccessPrivilege");
pub const SE_UNDOCK_NAME: windows_core::PCWSTR = windows_core::w!("SeUndockPrivilege");
pub const SE_UNSOLICITED_INPUT_NAME: windows_core::PCWSTR = windows_core::w!("SeUnsolicitedInputPrivilege");
pub const SIGNING_LEVEL_FILE_CACHE_FLAG_NOT_VALIDATED: u32 = 1u32;
pub const SIGNING_LEVEL_FILE_CACHE_FLAG_VALIDATE_ONLY: u32 = 4u32;
pub const SIGNING_LEVEL_MICROSOFT: u32 = 8u32;
pub const SUB_CONTAINERS_AND_OBJECTS_INHERIT: ACE_FLAGS = ACE_FLAGS(3u32);
pub const SUB_CONTAINERS_ONLY_INHERIT: ACE_FLAGS = ACE_FLAGS(2u32);
pub const SUB_OBJECTS_ONLY_INHERIT: ACE_FLAGS = ACE_FLAGS(1u32);
pub const SUCCESSFUL_ACCESS_ACE_FLAG: ACE_FLAGS = ACE_FLAGS(64u32);
pub const SecurityAnonymous: SECURITY_IMPERSONATION_LEVEL = SECURITY_IMPERSONATION_LEVEL(0i32);
pub const SecurityDelegation: SECURITY_IMPERSONATION_LEVEL = SECURITY_IMPERSONATION_LEVEL(3i32);
pub const SecurityIdentification: SECURITY_IMPERSONATION_LEVEL = SECURITY_IMPERSONATION_LEVEL(1i32);
pub const SecurityImpersonation: SECURITY_IMPERSONATION_LEVEL = SECURITY_IMPERSONATION_LEVEL(2i32);
pub const SidTypeAlias: SID_NAME_USE = SID_NAME_USE(4i32);
pub const SidTypeComputer: SID_NAME_USE = SID_NAME_USE(9i32);
pub const SidTypeDeletedAccount: SID_NAME_USE = SID_NAME_USE(6i32);
pub const SidTypeDomain: SID_NAME_USE = SID_NAME_USE(3i32);
pub const SidTypeGroup: SID_NAME_USE = SID_NAME_USE(2i32);
pub const SidTypeInvalid: SID_NAME_USE = SID_NAME_USE(7i32);
pub const SidTypeLabel: SID_NAME_USE = SID_NAME_USE(10i32);
pub const SidTypeLogonSession: SID_NAME_USE = SID_NAME_USE(11i32);
pub const SidTypeUnknown: SID_NAME_USE = SID_NAME_USE(8i32);
pub const SidTypeUser: SID_NAME_USE = SID_NAME_USE(1i32);
pub const SidTypeWellKnownGroup: SID_NAME_USE = SID_NAME_USE(5i32);
pub const TOKEN_ACCESS_PSEUDO_HANDLE: TOKEN_ACCESS_MASK = TOKEN_ACCESS_MASK(24u32);
pub const TOKEN_ACCESS_PSEUDO_HANDLE_WIN8: TOKEN_ACCESS_MASK = TOKEN_ACCESS_MASK(24u32);
pub const TOKEN_ACCESS_SYSTEM_SECURITY: TOKEN_ACCESS_MASK = TOKEN_ACCESS_MASK(16777216u32);
pub const TOKEN_ADJUST_DEFAULT: TOKEN_ACCESS_MASK = TOKEN_ACCESS_MASK(128u32);
pub const TOKEN_ADJUST_GROUPS: TOKEN_ACCESS_MASK = TOKEN_ACCESS_MASK(64u32);
pub const TOKEN_ADJUST_PRIVILEGES: TOKEN_ACCESS_MASK = TOKEN_ACCESS_MASK(32u32);
pub const TOKEN_ADJUST_SESSIONID: TOKEN_ACCESS_MASK = TOKEN_ACCESS_MASK(256u32);
pub const TOKEN_ALL_ACCESS: TOKEN_ACCESS_MASK = TOKEN_ACCESS_MASK(983551u32);
pub const TOKEN_ASSIGN_PRIMARY: TOKEN_ACCESS_MASK = TOKEN_ACCESS_MASK(1u32);
pub const TOKEN_DELETE: TOKEN_ACCESS_MASK = TOKEN_ACCESS_MASK(65536u32);
pub const TOKEN_DUPLICATE: TOKEN_ACCESS_MASK = TOKEN_ACCESS_MASK(2u32);
pub const TOKEN_EXECUTE: TOKEN_ACCESS_MASK = TOKEN_ACCESS_MASK(131072u32);
pub const TOKEN_IMPERSONATE: TOKEN_ACCESS_MASK = TOKEN_ACCESS_MASK(4u32);
pub const TOKEN_MANDATORY_POLICY_NEW_PROCESS_MIN: TOKEN_MANDATORY_POLICY_ID = TOKEN_MANDATORY_POLICY_ID(2u32);
pub const TOKEN_MANDATORY_POLICY_NO_WRITE_UP: TOKEN_MANDATORY_POLICY_ID = TOKEN_MANDATORY_POLICY_ID(1u32);
pub const TOKEN_MANDATORY_POLICY_OFF: TOKEN_MANDATORY_POLICY_ID = TOKEN_MANDATORY_POLICY_ID(0u32);
pub const TOKEN_MANDATORY_POLICY_VALID_MASK: TOKEN_MANDATORY_POLICY_ID = TOKEN_MANDATORY_POLICY_ID(3u32);
pub const TOKEN_QUERY: TOKEN_ACCESS_MASK = TOKEN_ACCESS_MASK(8u32);
pub const TOKEN_QUERY_SOURCE: TOKEN_ACCESS_MASK = TOKEN_ACCESS_MASK(16u32);
pub const TOKEN_READ: TOKEN_ACCESS_MASK = TOKEN_ACCESS_MASK(131080u32);
pub const TOKEN_READ_CONTROL: TOKEN_ACCESS_MASK = TOKEN_ACCESS_MASK(131072u32);
pub const TOKEN_TRUST_CONSTRAINT_MASK: TOKEN_ACCESS_MASK = TOKEN_ACCESS_MASK(131096u32);
pub const TOKEN_WRITE: TOKEN_ACCESS_MASK = TOKEN_ACCESS_MASK(131296u32);
pub const TOKEN_WRITE_DAC: TOKEN_ACCESS_MASK = TOKEN_ACCESS_MASK(262144u32);
pub const TOKEN_WRITE_OWNER: TOKEN_ACCESS_MASK = TOKEN_ACCESS_MASK(524288u32);
pub const TokenAccessInformation: TOKEN_INFORMATION_CLASS = TOKEN_INFORMATION_CLASS(22i32);
pub const TokenAppContainerNumber: TOKEN_INFORMATION_CLASS = TOKEN_INFORMATION_CLASS(32i32);
pub const TokenAppContainerSid: TOKEN_INFORMATION_CLASS = TOKEN_INFORMATION_CLASS(31i32);
pub const TokenAuditPolicy: TOKEN_INFORMATION_CLASS = TOKEN_INFORMATION_CLASS(16i32);
pub const TokenBnoIsolation: TOKEN_INFORMATION_CLASS = TOKEN_INFORMATION_CLASS(44i32);
pub const TokenCapabilities: TOKEN_INFORMATION_CLASS = TOKEN_INFORMATION_CLASS(30i32);
pub const TokenChildProcessFlags: TOKEN_INFORMATION_CLASS = TOKEN_INFORMATION_CLASS(45i32);
pub const TokenDefaultDacl: TOKEN_INFORMATION_CLASS = TOKEN_INFORMATION_CLASS(6i32);
pub const TokenDeviceClaimAttributes: TOKEN_INFORMATION_CLASS = TOKEN_INFORMATION_CLASS(34i32);
pub const TokenDeviceGroups: TOKEN_INFORMATION_CLASS = TOKEN_INFORMATION_CLASS(37i32);
pub const TokenElevation: TOKEN_INFORMATION_CLASS = TOKEN_INFORMATION_CLASS(20i32);
pub const TokenElevationType: TOKEN_INFORMATION_CLASS = TOKEN_INFORMATION_CLASS(18i32);
pub const TokenElevationTypeDefault: TOKEN_ELEVATION_TYPE = TOKEN_ELEVATION_TYPE(1i32);
pub const TokenElevationTypeFull: TOKEN_ELEVATION_TYPE = TOKEN_ELEVATION_TYPE(2i32);
pub const TokenElevationTypeLimited: TOKEN_ELEVATION_TYPE = TOKEN_ELEVATION_TYPE(3i32);
pub const TokenGroups: TOKEN_INFORMATION_CLASS = TOKEN_INFORMATION_CLASS(2i32);
pub const TokenGroupsAndPrivileges: TOKEN_INFORMATION_CLASS = TOKEN_INFORMATION_CLASS(13i32);
pub const TokenHasRestrictions: TOKEN_INFORMATION_CLASS = TOKEN_INFORMATION_CLASS(21i32);
pub const TokenImpersonation: TOKEN_TYPE = TOKEN_TYPE(2i32);
pub const TokenImpersonationLevel: TOKEN_INFORMATION_CLASS = TOKEN_INFORMATION_CLASS(9i32);
pub const TokenIntegrityLevel: TOKEN_INFORMATION_CLASS = TOKEN_INFORMATION_CLASS(25i32);
pub const TokenIsAppContainer: TOKEN_INFORMATION_CLASS = TOKEN_INFORMATION_CLASS(29i32);
pub const TokenIsAppSilo: TOKEN_INFORMATION_CLASS = TOKEN_INFORMATION_CLASS(48i32);
pub const TokenIsLessPrivilegedAppContainer: TOKEN_INFORMATION_CLASS = TOKEN_INFORMATION_CLASS(46i32);
pub const TokenIsRestricted: TOKEN_INFORMATION_CLASS = TOKEN_INFORMATION_CLASS(40i32);
pub const TokenIsSandboxed: TOKEN_INFORMATION_CLASS = TOKEN_INFORMATION_CLASS(47i32);
pub const TokenLinkedToken: TOKEN_INFORMATION_CLASS = TOKEN_INFORMATION_CLASS(19i32);
pub const TokenLogonSid: TOKEN_INFORMATION_CLASS = TOKEN_INFORMATION_CLASS(28i32);
pub const TokenMandatoryPolicy: TOKEN_INFORMATION_CLASS = TOKEN_INFORMATION_CLASS(27i32);
pub const TokenOrigin: TOKEN_INFORMATION_CLASS = TOKEN_INFORMATION_CLASS(17i32);
pub const TokenOwner: TOKEN_INFORMATION_CLASS = TOKEN_INFORMATION_CLASS(4i32);
pub const TokenPrimary: TOKEN_TYPE = TOKEN_TYPE(1i32);
pub const TokenPrimaryGroup: TOKEN_INFORMATION_CLASS = TOKEN_INFORMATION_CLASS(5i32);
pub const TokenPrivateNameSpace: TOKEN_INFORMATION_CLASS = TOKEN_INFORMATION_CLASS(42i32);
pub const TokenPrivileges: TOKEN_INFORMATION_CLASS = TOKEN_INFORMATION_CLASS(3i32);
pub const TokenProcessTrustLevel: TOKEN_INFORMATION_CLASS = TOKEN_INFORMATION_CLASS(41i32);
pub const TokenRestrictedDeviceClaimAttributes: TOKEN_INFORMATION_CLASS = TOKEN_INFORMATION_CLASS(36i32);
pub const TokenRestrictedDeviceGroups: TOKEN_INFORMATION_CLASS = TOKEN_INFORMATION_CLASS(38i32);
pub const TokenRestrictedSids: TOKEN_INFORMATION_CLASS = TOKEN_INFORMATION_CLASS(11i32);
pub const TokenRestrictedUserClaimAttributes: TOKEN_INFORMATION_CLASS = TOKEN_INFORMATION_CLASS(35i32);
pub const TokenSandBoxInert: TOKEN_INFORMATION_CLASS = TOKEN_INFORMATION_CLASS(15i32);
pub const TokenSecurityAttributes: TOKEN_INFORMATION_CLASS = TOKEN_INFORMATION_CLASS(39i32);
pub const TokenSessionId: TOKEN_INFORMATION_CLASS = TOKEN_INFORMATION_CLASS(12i32);
pub const TokenSessionReference: TOKEN_INFORMATION_CLASS = TOKEN_INFORMATION_CLASS(14i32);
pub const TokenSingletonAttributes: TOKEN_INFORMATION_CLASS = TOKEN_INFORMATION_CLASS(43i32);
pub const TokenSource: TOKEN_INFORMATION_CLASS = TOKEN_INFORMATION_CLASS(7i32);
pub const TokenStatistics: TOKEN_INFORMATION_CLASS = TOKEN_INFORMATION_CLASS(10i32);
pub const TokenType: TOKEN_INFORMATION_CLASS = TOKEN_INFORMATION_CLASS(8i32);
pub const TokenUIAccess: TOKEN_INFORMATION_CLASS = TOKEN_INFORMATION_CLASS(26i32);
pub const TokenUser: TOKEN_INFORMATION_CLASS = TOKEN_INFORMATION_CLASS(1i32);
pub const TokenUserClaimAttributes: TOKEN_INFORMATION_CLASS = TOKEN_INFORMATION_CLASS(33i32);
pub const TokenVirtualizationAllowed: TOKEN_INFORMATION_CLASS = TOKEN_INFORMATION_CLASS(23i32);
pub const TokenVirtualizationEnabled: TOKEN_INFORMATION_CLASS = TOKEN_INFORMATION_CLASS(24i32);
pub const UNPROTECTED_DACL_SECURITY_INFORMATION: OBJECT_SECURITY_INFORMATION = OBJECT_SECURITY_INFORMATION(536870912u32);
pub const UNPROTECTED_SACL_SECURITY_INFORMATION: OBJECT_SECURITY_INFORMATION = OBJECT_SECURITY_INFORMATION(268435456u32);
pub const WRITE_RESTRICTED: CREATE_RESTRICTED_TOKEN_FLAGS = CREATE_RESTRICTED_TOKEN_FLAGS(8u32);
pub const WinAccountAdministratorSid: WELL_KNOWN_SID_TYPE = WELL_KNOWN_SID_TYPE(38i32);
pub const WinAccountCertAdminsSid: WELL_KNOWN_SID_TYPE = WELL_KNOWN_SID_TYPE(46i32);
pub const WinAccountCloneableControllersSid: WELL_KNOWN_SID_TYPE = WELL_KNOWN_SID_TYPE(100i32);
pub const WinAccountComputersSid: WELL_KNOWN_SID_TYPE = WELL_KNOWN_SID_TYPE(44i32);
pub const WinAccountControllersSid: WELL_KNOWN_SID_TYPE = WELL_KNOWN_SID_TYPE(45i32);
pub const WinAccountDefaultSystemManagedSid: WELL_KNOWN_SID_TYPE = WELL_KNOWN_SID_TYPE(110i32);
pub const WinAccountDomainAdminsSid: WELL_KNOWN_SID_TYPE = WELL_KNOWN_SID_TYPE(41i32);
pub const WinAccountDomainGuestsSid: WELL_KNOWN_SID_TYPE = WELL_KNOWN_SID_TYPE(43i32);
pub const WinAccountDomainUsersSid: WELL_KNOWN_SID_TYPE = WELL_KNOWN_SID_TYPE(42i32);
pub const WinAccountEnterpriseAdminsSid: WELL_KNOWN_SID_TYPE = WELL_KNOWN_SID_TYPE(48i32);
pub const WinAccountEnterpriseKeyAdminsSid: WELL_KNOWN_SID_TYPE = WELL_KNOWN_SID_TYPE(114i32);
pub const WinAccountGuestSid: WELL_KNOWN_SID_TYPE = WELL_KNOWN_SID_TYPE(39i32);
pub const WinAccountKeyAdminsSid: WELL_KNOWN_SID_TYPE = WELL_KNOWN_SID_TYPE(113i32);
pub const WinAccountKrbtgtSid: WELL_KNOWN_SID_TYPE = WELL_KNOWN_SID_TYPE(40i32);
pub const WinAccountPolicyAdminsSid: WELL_KNOWN_SID_TYPE = WELL_KNOWN_SID_TYPE(49i32);
pub const WinAccountProtectedUsersSid: WELL_KNOWN_SID_TYPE = WELL_KNOWN_SID_TYPE(107i32);
pub const WinAccountRasAndIasServersSid: WELL_KNOWN_SID_TYPE = WELL_KNOWN_SID_TYPE(50i32);
pub const WinAccountReadonlyControllersSid: WELL_KNOWN_SID_TYPE = WELL_KNOWN_SID_TYPE(75i32);
pub const WinAccountSchemaAdminsSid: WELL_KNOWN_SID_TYPE = WELL_KNOWN_SID_TYPE(47i32);
pub const WinAnonymousSid: WELL_KNOWN_SID_TYPE = WELL_KNOWN_SID_TYPE(13i32);
pub const WinApplicationPackageAuthoritySid: WELL_KNOWN_SID_TYPE = WELL_KNOWN_SID_TYPE(83i32);
pub const WinAuthenticatedUserSid: WELL_KNOWN_SID_TYPE = WELL_KNOWN_SID_TYPE(17i32);
pub const WinAuthenticationAuthorityAssertedSid: WELL_KNOWN_SID_TYPE = WELL_KNOWN_SID_TYPE(103i32);
pub const WinAuthenticationFreshKeyAuthSid: WELL_KNOWN_SID_TYPE = WELL_KNOWN_SID_TYPE(118i32);
pub const WinAuthenticationKeyPropertyAttestationSid: WELL_KNOWN_SID_TYPE = WELL_KNOWN_SID_TYPE(117i32);
pub const WinAuthenticationKeyPropertyMFASid: WELL_KNOWN_SID_TYPE = WELL_KNOWN_SID_TYPE(116i32);
pub const WinAuthenticationKeyTrustSid: WELL_KNOWN_SID_TYPE = WELL_KNOWN_SID_TYPE(115i32);
pub const WinAuthenticationServiceAssertedSid: WELL_KNOWN_SID_TYPE = WELL_KNOWN_SID_TYPE(104i32);
pub const WinBatchSid: WELL_KNOWN_SID_TYPE = WELL_KNOWN_SID_TYPE(10i32);
pub const WinBuiltinAccessControlAssistanceOperatorsSid: WELL_KNOWN_SID_TYPE = WELL_KNOWN_SID_TYPE(101i32);
pub const WinBuiltinAccountOperatorsSid: WELL_KNOWN_SID_TYPE = WELL_KNOWN_SID_TYPE(30i32);
pub const WinBuiltinAdministratorsSid: WELL_KNOWN_SID_TYPE = WELL_KNOWN_SID_TYPE(26i32);
pub const WinBuiltinAnyPackageSid: WELL_KNOWN_SID_TYPE = WELL_KNOWN_SID_TYPE(84i32);
pub const WinBuiltinAuthorizationAccessSid: WELL_KNOWN_SID_TYPE = WELL_KNOWN_SID_TYPE(59i32);
pub const WinBuiltinBackupOperatorsSid: WELL_KNOWN_SID_TYPE = WELL_KNOWN_SID_TYPE(33i32);
pub const WinBuiltinCertSvcDComAccessGroup: WELL_KNOWN_SID_TYPE = WELL_KNOWN_SID_TYPE(78i32);
pub const WinBuiltinCryptoOperatorsSid: WELL_KNOWN_SID_TYPE = WELL_KNOWN_SID_TYPE(64i32);
pub const WinBuiltinDCOMUsersSid: WELL_KNOWN_SID_TYPE = WELL_KNOWN_SID_TYPE(61i32);
pub const WinBuiltinDefaultSystemManagedGroupSid: WELL_KNOWN_SID_TYPE = WELL_KNOWN_SID_TYPE(111i32);
pub const WinBuiltinDeviceOwnersSid: WELL_KNOWN_SID_TYPE = WELL_KNOWN_SID_TYPE(119i32);
pub const WinBuiltinDomainSid: WELL_KNOWN_SID_TYPE = WELL_KNOWN_SID_TYPE(25i32);
pub const WinBuiltinEventLogReadersGroup: WELL_KNOWN_SID_TYPE = WELL_KNOWN_SID_TYPE(76i32);
pub const WinBuiltinGuestsSid: WELL_KNOWN_SID_TYPE = WELL_KNOWN_SID_TYPE(28i32);
pub const WinBuiltinHyperVAdminsSid: WELL_KNOWN_SID_TYPE = WELL_KNOWN_SID_TYPE(99i32);
pub const WinBuiltinIUsersSid: WELL_KNOWN_SID_TYPE = WELL_KNOWN_SID_TYPE(62i32);
pub const WinBuiltinIncomingForestTrustBuildersSid: WELL_KNOWN_SID_TYPE = WELL_KNOWN_SID_TYPE(56i32);
pub const WinBuiltinNetworkConfigurationOperatorsSid: WELL_KNOWN_SID_TYPE = WELL_KNOWN_SID_TYPE(37i32);
pub const WinBuiltinPerfLoggingUsersSid: WELL_KNOWN_SID_TYPE = WELL_KNOWN_SID_TYPE(58i32);
pub const WinBuiltinPerfMonitoringUsersSid: WELL_KNOWN_SID_TYPE = WELL_KNOWN_SID_TYPE(57i32);
pub const WinBuiltinPowerUsersSid: WELL_KNOWN_SID_TYPE = WELL_KNOWN_SID_TYPE(29i32);
pub const WinBuiltinPreWindows2000CompatibleAccessSid: WELL_KNOWN_SID_TYPE = WELL_KNOWN_SID_TYPE(35i32);
pub const WinBuiltinPrintOperatorsSid: WELL_KNOWN_SID_TYPE = WELL_KNOWN_SID_TYPE(32i32);
pub const WinBuiltinRDSEndpointServersSid: WELL_KNOWN_SID_TYPE = WELL_KNOWN_SID_TYPE(96i32);
pub const WinBuiltinRDSManagementServersSid: WELL_KNOWN_SID_TYPE = WELL_KNOWN_SID_TYPE(97i32);
pub const WinBuiltinRDSRemoteAccessServersSid: WELL_KNOWN_SID_TYPE = WELL_KNOWN_SID_TYPE(95i32);
pub const WinBuiltinRemoteDesktopUsersSid: WELL_KNOWN_SID_TYPE = WELL_KNOWN_SID_TYPE(36i32);
pub const WinBuiltinRemoteManagementUsersSid: WELL_KNOWN_SID_TYPE = WELL_KNOWN_SID_TYPE(102i32);
pub const WinBuiltinReplicatorSid: WELL_KNOWN_SID_TYPE = WELL_KNOWN_SID_TYPE(34i32);
pub const WinBuiltinStorageReplicaAdminsSid: WELL_KNOWN_SID_TYPE = WELL_KNOWN_SID_TYPE(112i32);
pub const WinBuiltinSystemOperatorsSid: WELL_KNOWN_SID_TYPE = WELL_KNOWN_SID_TYPE(31i32);
pub const WinBuiltinTerminalServerLicenseServersSid: WELL_KNOWN_SID_TYPE = WELL_KNOWN_SID_TYPE(60i32);
pub const WinBuiltinUsersSid: WELL_KNOWN_SID_TYPE = WELL_KNOWN_SID_TYPE(27i32);
pub const WinCacheablePrincipalsGroupSid: WELL_KNOWN_SID_TYPE = WELL_KNOWN_SID_TYPE(72i32);
pub const WinCapabilityAppointmentsSid: WELL_KNOWN_SID_TYPE = WELL_KNOWN_SID_TYPE(108i32);
pub const WinCapabilityContactsSid: WELL_KNOWN_SID_TYPE = WELL_KNOWN_SID_TYPE(109i32);
pub const WinCapabilityDocumentsLibrarySid: WELL_KNOWN_SID_TYPE = WELL_KNOWN_SID_TYPE(91i32);
pub const WinCapabilityEnterpriseAuthenticationSid: WELL_KNOWN_SID_TYPE = WELL_KNOWN_SID_TYPE(93i32);
pub const WinCapabilityInternetClientServerSid: WELL_KNOWN_SID_TYPE = WELL_KNOWN_SID_TYPE(86i32);
pub const WinCapabilityInternetClientSid: WELL_KNOWN_SID_TYPE = WELL_KNOWN_SID_TYPE(85i32);
pub const WinCapabilityMusicLibrarySid: WELL_KNOWN_SID_TYPE = WELL_KNOWN_SID_TYPE(90i32);
pub const WinCapabilityPicturesLibrarySid: WELL_KNOWN_SID_TYPE = WELL_KNOWN_SID_TYPE(88i32);
pub const WinCapabilityPrivateNetworkClientServerSid: WELL_KNOWN_SID_TYPE = WELL_KNOWN_SID_TYPE(87i32);
pub const WinCapabilityRemovableStorageSid: WELL_KNOWN_SID_TYPE = WELL_KNOWN_SID_TYPE(94i32);
pub const WinCapabilitySharedUserCertificatesSid: WELL_KNOWN_SID_TYPE = WELL_KNOWN_SID_TYPE(92i32);
pub const WinCapabilityVideosLibrarySid: WELL_KNOWN_SID_TYPE = WELL_KNOWN_SID_TYPE(89i32);
pub const WinConsoleLogonSid: WELL_KNOWN_SID_TYPE = WELL_KNOWN_SID_TYPE(81i32);
pub const WinCreatorGroupServerSid: WELL_KNOWN_SID_TYPE = WELL_KNOWN_SID_TYPE(6i32);
pub const WinCreatorGroupSid: WELL_KNOWN_SID_TYPE = WELL_KNOWN_SID_TYPE(4i32);
pub const WinCreatorOwnerRightsSid: WELL_KNOWN_SID_TYPE = WELL_KNOWN_SID_TYPE(71i32);
pub const WinCreatorOwnerServerSid: WELL_KNOWN_SID_TYPE = WELL_KNOWN_SID_TYPE(5i32);
pub const WinCreatorOwnerSid: WELL_KNOWN_SID_TYPE = WELL_KNOWN_SID_TYPE(3i32);
pub const WinDialupSid: WELL_KNOWN_SID_TYPE = WELL_KNOWN_SID_TYPE(8i32);
pub const WinDigestAuthenticationSid: WELL_KNOWN_SID_TYPE = WELL_KNOWN_SID_TYPE(52i32);
pub const WinEnterpriseControllersSid: WELL_KNOWN_SID_TYPE = WELL_KNOWN_SID_TYPE(15i32);
pub const WinEnterpriseReadonlyControllersSid: WELL_KNOWN_SID_TYPE = WELL_KNOWN_SID_TYPE(74i32);
pub const WinHighLabelSid: WELL_KNOWN_SID_TYPE = WELL_KNOWN_SID_TYPE(68i32);
pub const WinIUserSid: WELL_KNOWN_SID_TYPE = WELL_KNOWN_SID_TYPE(63i32);
pub const WinInteractiveSid: WELL_KNOWN_SID_TYPE = WELL_KNOWN_SID_TYPE(11i32);
pub const WinLocalAccountAndAdministratorSid: WELL_KNOWN_SID_TYPE = WELL_KNOWN_SID_TYPE(106i32);
pub const WinLocalAccountSid: WELL_KNOWN_SID_TYPE = WELL_KNOWN_SID_TYPE(105i32);
pub const WinLocalLogonSid: WELL_KNOWN_SID_TYPE = WELL_KNOWN_SID_TYPE(80i32);
pub const WinLocalServiceSid: WELL_KNOWN_SID_TYPE = WELL_KNOWN_SID_TYPE(23i32);
pub const WinLocalSid: WELL_KNOWN_SID_TYPE = WELL_KNOWN_SID_TYPE(2i32);
pub const WinLocalSystemSid: WELL_KNOWN_SID_TYPE = WELL_KNOWN_SID_TYPE(22i32);
pub const WinLogonIdsSid: WELL_KNOWN_SID_TYPE = WELL_KNOWN_SID_TYPE(21i32);
pub const WinLowLabelSid: WELL_KNOWN_SID_TYPE = WELL_KNOWN_SID_TYPE(66i32);
pub const WinMediumLabelSid: WELL_KNOWN_SID_TYPE = WELL_KNOWN_SID_TYPE(67i32);
pub const WinMediumPlusLabelSid: WELL_KNOWN_SID_TYPE = WELL_KNOWN_SID_TYPE(79i32);
pub const WinNTLMAuthenticationSid: WELL_KNOWN_SID_TYPE = WELL_KNOWN_SID_TYPE(51i32);
pub const WinNetworkServiceSid: WELL_KNOWN_SID_TYPE = WELL_KNOWN_SID_TYPE(24i32);
pub const WinNetworkSid: WELL_KNOWN_SID_TYPE = WELL_KNOWN_SID_TYPE(9i32);
pub const WinNewEnterpriseReadonlyControllersSid: WELL_KNOWN_SID_TYPE = WELL_KNOWN_SID_TYPE(77i32);
pub const WinNonCacheablePrincipalsGroupSid: WELL_KNOWN_SID_TYPE = WELL_KNOWN_SID_TYPE(73i32);
pub const WinNtAuthoritySid: WELL_KNOWN_SID_TYPE = WELL_KNOWN_SID_TYPE(7i32);
pub const WinNullSid: WELL_KNOWN_SID_TYPE = WELL_KNOWN_SID_TYPE(0i32);
pub const WinOtherOrganizationSid: WELL_KNOWN_SID_TYPE = WELL_KNOWN_SID_TYPE(55i32);
pub const WinProxySid: WELL_KNOWN_SID_TYPE = WELL_KNOWN_SID_TYPE(14i32);
pub const WinRemoteLogonIdSid: WELL_KNOWN_SID_TYPE = WELL_KNOWN_SID_TYPE(20i32);
pub const WinRestrictedCodeSid: WELL_KNOWN_SID_TYPE = WELL_KNOWN_SID_TYPE(18i32);
pub const WinSChannelAuthenticationSid: WELL_KNOWN_SID_TYPE = WELL_KNOWN_SID_TYPE(53i32);
pub const WinSelfSid: WELL_KNOWN_SID_TYPE = WELL_KNOWN_SID_TYPE(16i32);
pub const WinServiceSid: WELL_KNOWN_SID_TYPE = WELL_KNOWN_SID_TYPE(12i32);
pub const WinSystemLabelSid: WELL_KNOWN_SID_TYPE = WELL_KNOWN_SID_TYPE(69i32);
pub const WinTerminalServerSid: WELL_KNOWN_SID_TYPE = WELL_KNOWN_SID_TYPE(19i32);
pub const WinThisOrganizationCertificateSid: WELL_KNOWN_SID_TYPE = WELL_KNOWN_SID_TYPE(82i32);
pub const WinThisOrganizationSid: WELL_KNOWN_SID_TYPE = WELL_KNOWN_SID_TYPE(54i32);
pub const WinUntrustedLabelSid: WELL_KNOWN_SID_TYPE = WELL_KNOWN_SID_TYPE(65i32);
pub const WinUserModeDriversSid: WELL_KNOWN_SID_TYPE = WELL_KNOWN_SID_TYPE(98i32);
pub const WinWorldSid: WELL_KNOWN_SID_TYPE = WELL_KNOWN_SID_TYPE(1i32);
pub const WinWriteRestrictedCodeSid: WELL_KNOWN_SID_TYPE = WELL_KNOWN_SID_TYPE(70i32);
pub const cwcFILENAMESUFFIXMAX: u32 = 20u32;
pub const cwcHRESULTSTRING: u32 = 40u32;
pub const szLBRACE: windows_core::PCSTR = windows_core::s!("{");
pub const szLPAREN: windows_core::PCSTR = windows_core::s!("(");
pub const szRBRACE: windows_core::PCSTR = windows_core::s!("}");
pub const szRPAREN: windows_core::PCSTR = windows_core::s!(")");
pub const wszCERTENROLLSHAREPATH: windows_core::PCWSTR = windows_core::w!("CertSrv\\CertEnroll");
pub const wszFCSAPARM_CERTFILENAMESUFFIX: windows_core::PCWSTR = windows_core::w!("%4");
pub const wszFCSAPARM_CONFIGDN: windows_core::PCWSTR = windows_core::w!("%6");
pub const wszFCSAPARM_CRLDELTAFILENAMESUFFIX: windows_core::PCWSTR = windows_core::w!("%9");
pub const wszFCSAPARM_CRLFILENAMESUFFIX: windows_core::PCWSTR = windows_core::w!("%8");
pub const wszFCSAPARM_DOMAINDN: windows_core::PCWSTR = windows_core::w!("%5");
pub const wszFCSAPARM_DSCACERTATTRIBUTE: windows_core::PCWSTR = windows_core::w!("%11");
pub const wszFCSAPARM_DSCRLATTRIBUTE: windows_core::PCWSTR = windows_core::w!("%10");
pub const wszFCSAPARM_DSCROSSCERTPAIRATTRIBUTE: windows_core::PCWSTR = windows_core::w!("%14");
pub const wszFCSAPARM_DSKRACERTATTRIBUTE: windows_core::PCWSTR = windows_core::w!("%13");
pub const wszFCSAPARM_DSUSERCERTATTRIBUTE: windows_core::PCWSTR = windows_core::w!("%12");
pub const wszFCSAPARM_SANITIZEDCANAME: windows_core::PCWSTR = windows_core::w!("%3");
pub const wszFCSAPARM_SANITIZEDCANAMEHASH: windows_core::PCWSTR = windows_core::w!("%7");
pub const wszFCSAPARM_SERVERDNSNAME: windows_core::PCWSTR = windows_core::w!("%1");
pub const wszFCSAPARM_SERVERSHORTNAME: windows_core::PCWSTR = windows_core::w!("%2");
pub const wszLBRACE: windows_core::PCWSTR = windows_core::w!("{");
pub const wszLPAREN: windows_core::PCWSTR = windows_core::w!("(");
pub const wszRBRACE: windows_core::PCWSTR = windows_core::w!("}");
pub const wszRPAREN: windows_core::PCWSTR = windows_core::w!(")");
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct ACE_FLAGS(pub u32);
impl windows_core::TypeKind for ACE_FLAGS {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for ACE_FLAGS {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("ACE_FLAGS").field(&self.0).finish()
    }
}
impl ACE_FLAGS {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for ACE_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for ACE_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for ACE_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for ACE_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for ACE_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct ACE_REVISION(pub u32);
impl windows_core::TypeKind for ACE_REVISION {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for ACE_REVISION {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("ACE_REVISION").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct ACL_INFORMATION_CLASS(pub i32);
impl windows_core::TypeKind for ACL_INFORMATION_CLASS {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for ACL_INFORMATION_CLASS {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("ACL_INFORMATION_CLASS").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct AUDIT_EVENT_TYPE(pub i32);
impl windows_core::TypeKind for AUDIT_EVENT_TYPE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for AUDIT_EVENT_TYPE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("AUDIT_EVENT_TYPE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct CLAIM_SECURITY_ATTRIBUTE_FLAGS(pub u32);
impl windows_core::TypeKind for CLAIM_SECURITY_ATTRIBUTE_FLAGS {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for CLAIM_SECURITY_ATTRIBUTE_FLAGS {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("CLAIM_SECURITY_ATTRIBUTE_FLAGS").field(&self.0).finish()
    }
}
impl CLAIM_SECURITY_ATTRIBUTE_FLAGS {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for CLAIM_SECURITY_ATTRIBUTE_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for CLAIM_SECURITY_ATTRIBUTE_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for CLAIM_SECURITY_ATTRIBUTE_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for CLAIM_SECURITY_ATTRIBUTE_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for CLAIM_SECURITY_ATTRIBUTE_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct CLAIM_SECURITY_ATTRIBUTE_VALUE_TYPE(pub u16);
impl windows_core::TypeKind for CLAIM_SECURITY_ATTRIBUTE_VALUE_TYPE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for CLAIM_SECURITY_ATTRIBUTE_VALUE_TYPE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("CLAIM_SECURITY_ATTRIBUTE_VALUE_TYPE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct CREATE_RESTRICTED_TOKEN_FLAGS(pub u32);
impl windows_core::TypeKind for CREATE_RESTRICTED_TOKEN_FLAGS {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for CREATE_RESTRICTED_TOKEN_FLAGS {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("CREATE_RESTRICTED_TOKEN_FLAGS").field(&self.0).finish()
    }
}
impl CREATE_RESTRICTED_TOKEN_FLAGS {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for CREATE_RESTRICTED_TOKEN_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for CREATE_RESTRICTED_TOKEN_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for CREATE_RESTRICTED_TOKEN_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for CREATE_RESTRICTED_TOKEN_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for CREATE_RESTRICTED_TOKEN_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct ENUM_PERIOD(pub i32);
impl windows_core::TypeKind for ENUM_PERIOD {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for ENUM_PERIOD {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("ENUM_PERIOD").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct LOGON32_LOGON(pub u32);
impl windows_core::TypeKind for LOGON32_LOGON {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for LOGON32_LOGON {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("LOGON32_LOGON").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct LOGON32_PROVIDER(pub u32);
impl windows_core::TypeKind for LOGON32_PROVIDER {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for LOGON32_PROVIDER {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("LOGON32_PROVIDER").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct MANDATORY_LEVEL(pub i32);
impl windows_core::TypeKind for MANDATORY_LEVEL {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for MANDATORY_LEVEL {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("MANDATORY_LEVEL").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct OBJECT_SECURITY_INFORMATION(pub u32);
impl windows_core::TypeKind for OBJECT_SECURITY_INFORMATION {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for OBJECT_SECURITY_INFORMATION {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("OBJECT_SECURITY_INFORMATION").field(&self.0).finish()
    }
}
impl OBJECT_SECURITY_INFORMATION {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for OBJECT_SECURITY_INFORMATION {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for OBJECT_SECURITY_INFORMATION {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for OBJECT_SECURITY_INFORMATION {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for OBJECT_SECURITY_INFORMATION {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for OBJECT_SECURITY_INFORMATION {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct SECURITY_AUTO_INHERIT_FLAGS(pub u32);
impl windows_core::TypeKind for SECURITY_AUTO_INHERIT_FLAGS {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for SECURITY_AUTO_INHERIT_FLAGS {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("SECURITY_AUTO_INHERIT_FLAGS").field(&self.0).finish()
    }
}
impl SECURITY_AUTO_INHERIT_FLAGS {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for SECURITY_AUTO_INHERIT_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for SECURITY_AUTO_INHERIT_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for SECURITY_AUTO_INHERIT_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for SECURITY_AUTO_INHERIT_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for SECURITY_AUTO_INHERIT_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct SECURITY_DESCRIPTOR_CONTROL(pub u16);
impl windows_core::TypeKind for SECURITY_DESCRIPTOR_CONTROL {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for SECURITY_DESCRIPTOR_CONTROL {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("SECURITY_DESCRIPTOR_CONTROL").field(&self.0).finish()
    }
}
impl SECURITY_DESCRIPTOR_CONTROL {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for SECURITY_DESCRIPTOR_CONTROL {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for SECURITY_DESCRIPTOR_CONTROL {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for SECURITY_DESCRIPTOR_CONTROL {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for SECURITY_DESCRIPTOR_CONTROL {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for SECURITY_DESCRIPTOR_CONTROL {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct SECURITY_IMPERSONATION_LEVEL(pub i32);
impl windows_core::TypeKind for SECURITY_IMPERSONATION_LEVEL {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for SECURITY_IMPERSONATION_LEVEL {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("SECURITY_IMPERSONATION_LEVEL").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct SID_NAME_USE(pub i32);
impl windows_core::TypeKind for SID_NAME_USE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for SID_NAME_USE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("SID_NAME_USE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct SYSTEM_AUDIT_OBJECT_ACE_FLAGS(pub u32);
impl windows_core::TypeKind for SYSTEM_AUDIT_OBJECT_ACE_FLAGS {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for SYSTEM_AUDIT_OBJECT_ACE_FLAGS {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("SYSTEM_AUDIT_OBJECT_ACE_FLAGS").field(&self.0).finish()
    }
}
impl SYSTEM_AUDIT_OBJECT_ACE_FLAGS {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for SYSTEM_AUDIT_OBJECT_ACE_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for SYSTEM_AUDIT_OBJECT_ACE_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for SYSTEM_AUDIT_OBJECT_ACE_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for SYSTEM_AUDIT_OBJECT_ACE_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for SYSTEM_AUDIT_OBJECT_ACE_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct TOKEN_ACCESS_MASK(pub u32);
impl windows_core::TypeKind for TOKEN_ACCESS_MASK {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for TOKEN_ACCESS_MASK {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("TOKEN_ACCESS_MASK").field(&self.0).finish()
    }
}
impl TOKEN_ACCESS_MASK {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for TOKEN_ACCESS_MASK {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for TOKEN_ACCESS_MASK {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for TOKEN_ACCESS_MASK {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for TOKEN_ACCESS_MASK {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for TOKEN_ACCESS_MASK {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct TOKEN_ELEVATION_TYPE(pub i32);
impl windows_core::TypeKind for TOKEN_ELEVATION_TYPE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for TOKEN_ELEVATION_TYPE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("TOKEN_ELEVATION_TYPE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct TOKEN_INFORMATION_CLASS(pub i32);
impl windows_core::TypeKind for TOKEN_INFORMATION_CLASS {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for TOKEN_INFORMATION_CLASS {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("TOKEN_INFORMATION_CLASS").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct TOKEN_MANDATORY_POLICY_ID(pub u32);
impl windows_core::TypeKind for TOKEN_MANDATORY_POLICY_ID {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for TOKEN_MANDATORY_POLICY_ID {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("TOKEN_MANDATORY_POLICY_ID").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct TOKEN_PRIVILEGES_ATTRIBUTES(pub u32);
impl windows_core::TypeKind for TOKEN_PRIVILEGES_ATTRIBUTES {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for TOKEN_PRIVILEGES_ATTRIBUTES {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("TOKEN_PRIVILEGES_ATTRIBUTES").field(&self.0).finish()
    }
}
impl TOKEN_PRIVILEGES_ATTRIBUTES {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for TOKEN_PRIVILEGES_ATTRIBUTES {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for TOKEN_PRIVILEGES_ATTRIBUTES {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for TOKEN_PRIVILEGES_ATTRIBUTES {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for TOKEN_PRIVILEGES_ATTRIBUTES {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for TOKEN_PRIVILEGES_ATTRIBUTES {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct TOKEN_TYPE(pub i32);
impl windows_core::TypeKind for TOKEN_TYPE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for TOKEN_TYPE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("TOKEN_TYPE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct WELL_KNOWN_SID_TYPE(pub i32);
impl windows_core::TypeKind for WELL_KNOWN_SID_TYPE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for WELL_KNOWN_SID_TYPE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("WELL_KNOWN_SID_TYPE").field(&self.0).finish()
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ACCESS_ALLOWED_ACE {
    pub Header: ACE_HEADER,
    pub Mask: u32,
    pub SidStart: u32,
}
impl windows_core::TypeKind for ACCESS_ALLOWED_ACE {
    type TypeKind = windows_core::CopyType;
}
impl Default for ACCESS_ALLOWED_ACE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ACCESS_ALLOWED_CALLBACK_ACE {
    pub Header: ACE_HEADER,
    pub Mask: u32,
    pub SidStart: u32,
}
impl windows_core::TypeKind for ACCESS_ALLOWED_CALLBACK_ACE {
    type TypeKind = windows_core::CopyType;
}
impl Default for ACCESS_ALLOWED_CALLBACK_ACE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ACCESS_ALLOWED_CALLBACK_OBJECT_ACE {
    pub Header: ACE_HEADER,
    pub Mask: u32,
    pub Flags: SYSTEM_AUDIT_OBJECT_ACE_FLAGS,
    pub ObjectType: windows_core::GUID,
    pub InheritedObjectType: windows_core::GUID,
    pub SidStart: u32,
}
impl windows_core::TypeKind for ACCESS_ALLOWED_CALLBACK_OBJECT_ACE {
    type TypeKind = windows_core::CopyType;
}
impl Default for ACCESS_ALLOWED_CALLBACK_OBJECT_ACE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ACCESS_ALLOWED_OBJECT_ACE {
    pub Header: ACE_HEADER,
    pub Mask: u32,
    pub Flags: SYSTEM_AUDIT_OBJECT_ACE_FLAGS,
    pub ObjectType: windows_core::GUID,
    pub InheritedObjectType: windows_core::GUID,
    pub SidStart: u32,
}
impl windows_core::TypeKind for ACCESS_ALLOWED_OBJECT_ACE {
    type TypeKind = windows_core::CopyType;
}
impl Default for ACCESS_ALLOWED_OBJECT_ACE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ACCESS_DENIED_ACE {
    pub Header: ACE_HEADER,
    pub Mask: u32,
    pub SidStart: u32,
}
impl windows_core::TypeKind for ACCESS_DENIED_ACE {
    type TypeKind = windows_core::CopyType;
}
impl Default for ACCESS_DENIED_ACE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ACCESS_DENIED_CALLBACK_ACE {
    pub Header: ACE_HEADER,
    pub Mask: u32,
    pub SidStart: u32,
}
impl windows_core::TypeKind for ACCESS_DENIED_CALLBACK_ACE {
    type TypeKind = windows_core::CopyType;
}
impl Default for ACCESS_DENIED_CALLBACK_ACE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ACCESS_DENIED_CALLBACK_OBJECT_ACE {
    pub Header: ACE_HEADER,
    pub Mask: u32,
    pub Flags: SYSTEM_AUDIT_OBJECT_ACE_FLAGS,
    pub ObjectType: windows_core::GUID,
    pub InheritedObjectType: windows_core::GUID,
    pub SidStart: u32,
}
impl windows_core::TypeKind for ACCESS_DENIED_CALLBACK_OBJECT_ACE {
    type TypeKind = windows_core::CopyType;
}
impl Default for ACCESS_DENIED_CALLBACK_OBJECT_ACE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ACCESS_DENIED_OBJECT_ACE {
    pub Header: ACE_HEADER,
    pub Mask: u32,
    pub Flags: SYSTEM_AUDIT_OBJECT_ACE_FLAGS,
    pub ObjectType: windows_core::GUID,
    pub InheritedObjectType: windows_core::GUID,
    pub SidStart: u32,
}
impl windows_core::TypeKind for ACCESS_DENIED_OBJECT_ACE {
    type TypeKind = windows_core::CopyType;
}
impl Default for ACCESS_DENIED_OBJECT_ACE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ACCESS_REASONS {
    pub Data: [u32; 32],
}
impl windows_core::TypeKind for ACCESS_REASONS {
    type TypeKind = windows_core::CopyType;
}
impl Default for ACCESS_REASONS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ACE_HEADER {
    pub AceType: u8,
    pub AceFlags: u8,
    pub AceSize: u16,
}
impl windows_core::TypeKind for ACE_HEADER {
    type TypeKind = windows_core::CopyType;
}
impl Default for ACE_HEADER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ACL {
    pub AclRevision: u8,
    pub Sbz1: u8,
    pub AclSize: u16,
    pub AceCount: u16,
    pub Sbz2: u16,
}
impl windows_core::TypeKind for ACL {
    type TypeKind = windows_core::CopyType;
}
impl Default for ACL {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ACL_REVISION_INFORMATION {
    pub AclRevision: u32,
}
impl windows_core::TypeKind for ACL_REVISION_INFORMATION {
    type TypeKind = windows_core::CopyType;
}
impl Default for ACL_REVISION_INFORMATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ACL_SIZE_INFORMATION {
    pub AceCount: u32,
    pub AclBytesInUse: u32,
    pub AclBytesFree: u32,
}
impl windows_core::TypeKind for ACL_SIZE_INFORMATION {
    type TypeKind = windows_core::CopyType;
}
impl Default for ACL_SIZE_INFORMATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct CLAIM_SECURITY_ATTRIBUTES_INFORMATION {
    pub Version: u16,
    pub Reserved: u16,
    pub AttributeCount: u32,
    pub Attribute: CLAIM_SECURITY_ATTRIBUTES_INFORMATION_0,
}
impl windows_core::TypeKind for CLAIM_SECURITY_ATTRIBUTES_INFORMATION {
    type TypeKind = windows_core::CopyType;
}
impl Default for CLAIM_SECURITY_ATTRIBUTES_INFORMATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union CLAIM_SECURITY_ATTRIBUTES_INFORMATION_0 {
    pub pAttributeV1: *mut CLAIM_SECURITY_ATTRIBUTE_V1,
}
impl windows_core::TypeKind for CLAIM_SECURITY_ATTRIBUTES_INFORMATION_0 {
    type TypeKind = windows_core::CopyType;
}
impl Default for CLAIM_SECURITY_ATTRIBUTES_INFORMATION_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct CLAIM_SECURITY_ATTRIBUTE_FQBN_VALUE {
    pub Version: u64,
    pub Name: windows_core::PWSTR,
}
impl windows_core::TypeKind for CLAIM_SECURITY_ATTRIBUTE_FQBN_VALUE {
    type TypeKind = windows_core::CopyType;
}
impl Default for CLAIM_SECURITY_ATTRIBUTE_FQBN_VALUE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct CLAIM_SECURITY_ATTRIBUTE_OCTET_STRING_VALUE {
    pub pValue: *mut core::ffi::c_void,
    pub ValueLength: u32,
}
impl windows_core::TypeKind for CLAIM_SECURITY_ATTRIBUTE_OCTET_STRING_VALUE {
    type TypeKind = windows_core::CopyType;
}
impl Default for CLAIM_SECURITY_ATTRIBUTE_OCTET_STRING_VALUE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct CLAIM_SECURITY_ATTRIBUTE_RELATIVE_V1 {
    pub Name: u32,
    pub ValueType: CLAIM_SECURITY_ATTRIBUTE_VALUE_TYPE,
    pub Reserved: u16,
    pub Flags: CLAIM_SECURITY_ATTRIBUTE_FLAGS,
    pub ValueCount: u32,
    pub Values: CLAIM_SECURITY_ATTRIBUTE_RELATIVE_V1_0,
}
impl windows_core::TypeKind for CLAIM_SECURITY_ATTRIBUTE_RELATIVE_V1 {
    type TypeKind = windows_core::CopyType;
}
impl Default for CLAIM_SECURITY_ATTRIBUTE_RELATIVE_V1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union CLAIM_SECURITY_ATTRIBUTE_RELATIVE_V1_0 {
    pub pInt64: [u32; 1],
    pub pUint64: [u32; 1],
    pub ppString: [u32; 1],
    pub pFqbn: [u32; 1],
    pub pOctetString: [u32; 1],
}
impl windows_core::TypeKind for CLAIM_SECURITY_ATTRIBUTE_RELATIVE_V1_0 {
    type TypeKind = windows_core::CopyType;
}
impl Default for CLAIM_SECURITY_ATTRIBUTE_RELATIVE_V1_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct CLAIM_SECURITY_ATTRIBUTE_V1 {
    pub Name: windows_core::PWSTR,
    pub ValueType: CLAIM_SECURITY_ATTRIBUTE_VALUE_TYPE,
    pub Reserved: u16,
    pub Flags: u32,
    pub ValueCount: u32,
    pub Values: CLAIM_SECURITY_ATTRIBUTE_V1_0,
}
impl windows_core::TypeKind for CLAIM_SECURITY_ATTRIBUTE_V1 {
    type TypeKind = windows_core::CopyType;
}
impl Default for CLAIM_SECURITY_ATTRIBUTE_V1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union CLAIM_SECURITY_ATTRIBUTE_V1_0 {
    pub pInt64: *mut i64,
    pub pUint64: *mut u64,
    pub ppString: *mut windows_core::PWSTR,
    pub pFqbn: *mut CLAIM_SECURITY_ATTRIBUTE_FQBN_VALUE,
    pub pOctetString: *mut CLAIM_SECURITY_ATTRIBUTE_OCTET_STRING_VALUE,
}
impl windows_core::TypeKind for CLAIM_SECURITY_ATTRIBUTE_V1_0 {
    type TypeKind = windows_core::CopyType;
}
impl Default for CLAIM_SECURITY_ATTRIBUTE_V1_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct GENERIC_MAPPING {
    pub GenericRead: u32,
    pub GenericWrite: u32,
    pub GenericExecute: u32,
    pub GenericAll: u32,
}
impl windows_core::TypeKind for GENERIC_MAPPING {
    type TypeKind = windows_core::CopyType;
}
impl Default for GENERIC_MAPPING {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct HDIAGNOSTIC_DATA_QUERY_SESSION(pub isize);
impl HDIAGNOSTIC_DATA_QUERY_SESSION {
    pub fn is_invalid(&self) -> bool {
        self.0 == -1 || self.0 == 0
    }
}
impl Default for HDIAGNOSTIC_DATA_QUERY_SESSION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for HDIAGNOSTIC_DATA_QUERY_SESSION {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct HDIAGNOSTIC_EVENT_CATEGORY_DESCRIPTION(pub isize);
impl HDIAGNOSTIC_EVENT_CATEGORY_DESCRIPTION {
    pub fn is_invalid(&self) -> bool {
        self.0 == -1 || self.0 == 0
    }
}
impl Default for HDIAGNOSTIC_EVENT_CATEGORY_DESCRIPTION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for HDIAGNOSTIC_EVENT_CATEGORY_DESCRIPTION {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct HDIAGNOSTIC_EVENT_PRODUCER_DESCRIPTION(pub isize);
impl HDIAGNOSTIC_EVENT_PRODUCER_DESCRIPTION {
    pub fn is_invalid(&self) -> bool {
        self.0 == -1 || self.0 == 0
    }
}
impl Default for HDIAGNOSTIC_EVENT_PRODUCER_DESCRIPTION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for HDIAGNOSTIC_EVENT_PRODUCER_DESCRIPTION {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct HDIAGNOSTIC_EVENT_TAG_DESCRIPTION(pub isize);
impl HDIAGNOSTIC_EVENT_TAG_DESCRIPTION {
    pub fn is_invalid(&self) -> bool {
        self.0 == -1 || self.0 == 0
    }
}
impl Default for HDIAGNOSTIC_EVENT_TAG_DESCRIPTION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for HDIAGNOSTIC_EVENT_TAG_DESCRIPTION {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct HDIAGNOSTIC_RECORD(pub isize);
impl HDIAGNOSTIC_RECORD {
    pub fn is_invalid(&self) -> bool {
        self.0 == -1 || self.0 == 0
    }
}
impl Default for HDIAGNOSTIC_RECORD {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for HDIAGNOSTIC_RECORD {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct HDIAGNOSTIC_REPORT(pub isize);
impl HDIAGNOSTIC_REPORT {
    pub fn is_invalid(&self) -> bool {
        self.0 == -1 || self.0 == 0
    }
}
impl Default for HDIAGNOSTIC_REPORT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for HDIAGNOSTIC_REPORT {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct LLFILETIME {
    pub Anonymous: LLFILETIME_0,
}
impl windows_core::TypeKind for LLFILETIME {
    type TypeKind = windows_core::CopyType;
}
impl Default for LLFILETIME {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union LLFILETIME_0 {
    pub ll: i64,
    pub ft: super::Foundation::FILETIME,
}
impl windows_core::TypeKind for LLFILETIME_0 {
    type TypeKind = windows_core::CopyType;
}
impl Default for LLFILETIME_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct LUID_AND_ATTRIBUTES {
    pub Luid: super::Foundation::LUID,
    pub Attributes: TOKEN_PRIVILEGES_ATTRIBUTES,
}
impl windows_core::TypeKind for LUID_AND_ATTRIBUTES {
    type TypeKind = windows_core::CopyType;
}
impl Default for LUID_AND_ATTRIBUTES {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct NCRYPT_DESCRIPTOR_HANDLE(pub isize);
impl NCRYPT_DESCRIPTOR_HANDLE {
    pub fn is_invalid(&self) -> bool {
        self.0 == -1 || self.0 == 0
    }
}
impl Default for NCRYPT_DESCRIPTOR_HANDLE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for NCRYPT_DESCRIPTOR_HANDLE {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct NCRYPT_STREAM_HANDLE(pub isize);
impl NCRYPT_STREAM_HANDLE {
    pub fn is_invalid(&self) -> bool {
        self.0 == -1 || self.0 == 0
    }
}
impl Default for NCRYPT_STREAM_HANDLE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for NCRYPT_STREAM_HANDLE {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct OBJECT_TYPE_LIST {
    pub Level: u16,
    pub Sbz: u16,
    pub ObjectType: *mut windows_core::GUID,
}
impl windows_core::TypeKind for OBJECT_TYPE_LIST {
    type TypeKind = windows_core::CopyType;
}
impl Default for OBJECT_TYPE_LIST {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct PRIVILEGE_SET {
    pub PrivilegeCount: u32,
    pub Control: u32,
    pub Privilege: [LUID_AND_ATTRIBUTES; 1],
}
impl windows_core::TypeKind for PRIVILEGE_SET {
    type TypeKind = windows_core::CopyType;
}
impl Default for PRIVILEGE_SET {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PSECURITY_DESCRIPTOR(pub *mut core::ffi::c_void);
impl PSECURITY_DESCRIPTOR {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PSECURITY_DESCRIPTOR {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for PSECURITY_DESCRIPTOR {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct QUOTA_LIMITS {
    pub PagedPoolLimit: usize,
    pub NonPagedPoolLimit: usize,
    pub MinimumWorkingSetSize: usize,
    pub MaximumWorkingSetSize: usize,
    pub PagefileLimit: usize,
    pub TimeLimit: i64,
}
impl windows_core::TypeKind for QUOTA_LIMITS {
    type TypeKind = windows_core::CopyType;
}
impl Default for QUOTA_LIMITS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SAFER_LEVEL_HANDLE(pub isize);
impl SAFER_LEVEL_HANDLE {
    pub fn is_invalid(&self) -> bool {
        self.0 == -1 || self.0 == 0
    }
}
impl Default for SAFER_LEVEL_HANDLE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for SAFER_LEVEL_HANDLE {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SC_HANDLE(pub isize);
impl SC_HANDLE {
    pub fn is_invalid(&self) -> bool {
        self.0 == -1 || self.0 == 0
    }
}
impl Default for SC_HANDLE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for SC_HANDLE {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SECURITY_ATTRIBUTES {
    pub nLength: u32,
    pub lpSecurityDescriptor: *mut core::ffi::c_void,
    pub bInheritHandle: super::Foundation::BOOL,
}
impl windows_core::TypeKind for SECURITY_ATTRIBUTES {
    type TypeKind = windows_core::CopyType;
}
impl Default for SECURITY_ATTRIBUTES {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SECURITY_CAPABILITIES {
    pub AppContainerSid: super::Foundation::PSID,
    pub Capabilities: *mut SID_AND_ATTRIBUTES,
    pub CapabilityCount: u32,
    pub Reserved: u32,
}
impl windows_core::TypeKind for SECURITY_CAPABILITIES {
    type TypeKind = windows_core::CopyType;
}
impl Default for SECURITY_CAPABILITIES {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SECURITY_DESCRIPTOR {
    pub Revision: u8,
    pub Sbz1: u8,
    pub Control: SECURITY_DESCRIPTOR_CONTROL,
    pub Owner: super::Foundation::PSID,
    pub Group: super::Foundation::PSID,
    pub Sacl: *mut ACL,
    pub Dacl: *mut ACL,
}
impl windows_core::TypeKind for SECURITY_DESCRIPTOR {
    type TypeKind = windows_core::CopyType;
}
impl Default for SECURITY_DESCRIPTOR {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SECURITY_DESCRIPTOR_RELATIVE {
    pub Revision: u8,
    pub Sbz1: u8,
    pub Control: SECURITY_DESCRIPTOR_CONTROL,
    pub Owner: u32,
    pub Group: u32,
    pub Sacl: u32,
    pub Dacl: u32,
}
impl windows_core::TypeKind for SECURITY_DESCRIPTOR_RELATIVE {
    type TypeKind = windows_core::CopyType;
}
impl Default for SECURITY_DESCRIPTOR_RELATIVE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SECURITY_QUALITY_OF_SERVICE {
    pub Length: u32,
    pub ImpersonationLevel: SECURITY_IMPERSONATION_LEVEL,
    pub ContextTrackingMode: u8,
    pub EffectiveOnly: super::Foundation::BOOLEAN,
}
impl windows_core::TypeKind for SECURITY_QUALITY_OF_SERVICE {
    type TypeKind = windows_core::CopyType;
}
impl Default for SECURITY_QUALITY_OF_SERVICE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SE_ACCESS_REPLY {
    pub Size: u32,
    pub ResultListCount: u32,
    pub GrantedAccess: *mut u32,
    pub AccessStatus: *mut u32,
    pub AccessReason: *mut ACCESS_REASONS,
    pub Privileges: *mut *mut PRIVILEGE_SET,
}
impl windows_core::TypeKind for SE_ACCESS_REPLY {
    type TypeKind = windows_core::CopyType;
}
impl Default for SE_ACCESS_REPLY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SE_ACCESS_REQUEST {
    pub Size: u32,
    pub SeSecurityDescriptor: *mut SE_SECURITY_DESCRIPTOR,
    pub DesiredAccess: u32,
    pub PreviouslyGrantedAccess: u32,
    pub PrincipalSelfSid: super::Foundation::PSID,
    pub GenericMapping: *mut GENERIC_MAPPING,
    pub ObjectTypeListCount: u32,
    pub ObjectTypeList: *mut OBJECT_TYPE_LIST,
}
impl windows_core::TypeKind for SE_ACCESS_REQUEST {
    type TypeKind = windows_core::CopyType;
}
impl Default for SE_ACCESS_REQUEST {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SE_IMPERSONATION_STATE {
    pub Token: *mut core::ffi::c_void,
    pub CopyOnOpen: super::Foundation::BOOLEAN,
    pub EffectiveOnly: super::Foundation::BOOLEAN,
    pub Level: SECURITY_IMPERSONATION_LEVEL,
}
impl windows_core::TypeKind for SE_IMPERSONATION_STATE {
    type TypeKind = windows_core::CopyType;
}
impl Default for SE_IMPERSONATION_STATE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SE_SECURITY_DESCRIPTOR {
    pub Size: u32,
    pub Flags: u32,
    pub SecurityDescriptor: PSECURITY_DESCRIPTOR,
}
impl windows_core::TypeKind for SE_SECURITY_DESCRIPTOR {
    type TypeKind = windows_core::CopyType;
}
impl Default for SE_SECURITY_DESCRIPTOR {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union SE_SID {
    pub Sid: SID,
    pub Buffer: [u8; 68],
}
impl windows_core::TypeKind for SE_SID {
    type TypeKind = windows_core::CopyType;
}
impl Default for SE_SID {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SID {
    pub Revision: u8,
    pub SubAuthorityCount: u8,
    pub IdentifierAuthority: SID_IDENTIFIER_AUTHORITY,
    pub SubAuthority: [u32; 1],
}
impl windows_core::TypeKind for SID {
    type TypeKind = windows_core::CopyType;
}
impl Default for SID {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SID_AND_ATTRIBUTES {
    pub Sid: super::Foundation::PSID,
    pub Attributes: u32,
}
impl windows_core::TypeKind for SID_AND_ATTRIBUTES {
    type TypeKind = windows_core::CopyType;
}
impl Default for SID_AND_ATTRIBUTES {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SID_AND_ATTRIBUTES_HASH {
    pub SidCount: u32,
    pub SidAttr: *mut SID_AND_ATTRIBUTES,
    pub Hash: [usize; 32],
}
impl windows_core::TypeKind for SID_AND_ATTRIBUTES_HASH {
    type TypeKind = windows_core::CopyType;
}
impl Default for SID_AND_ATTRIBUTES_HASH {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SID_IDENTIFIER_AUTHORITY {
    pub Value: [u8; 6],
}
impl windows_core::TypeKind for SID_IDENTIFIER_AUTHORITY {
    type TypeKind = windows_core::CopyType;
}
impl Default for SID_IDENTIFIER_AUTHORITY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SYSTEM_ACCESS_FILTER_ACE {
    pub Header: ACE_HEADER,
    pub Mask: u32,
    pub SidStart: u32,
}
impl windows_core::TypeKind for SYSTEM_ACCESS_FILTER_ACE {
    type TypeKind = windows_core::CopyType;
}
impl Default for SYSTEM_ACCESS_FILTER_ACE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SYSTEM_ALARM_ACE {
    pub Header: ACE_HEADER,
    pub Mask: u32,
    pub SidStart: u32,
}
impl windows_core::TypeKind for SYSTEM_ALARM_ACE {
    type TypeKind = windows_core::CopyType;
}
impl Default for SYSTEM_ALARM_ACE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SYSTEM_ALARM_CALLBACK_ACE {
    pub Header: ACE_HEADER,
    pub Mask: u32,
    pub SidStart: u32,
}
impl windows_core::TypeKind for SYSTEM_ALARM_CALLBACK_ACE {
    type TypeKind = windows_core::CopyType;
}
impl Default for SYSTEM_ALARM_CALLBACK_ACE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SYSTEM_ALARM_CALLBACK_OBJECT_ACE {
    pub Header: ACE_HEADER,
    pub Mask: u32,
    pub Flags: SYSTEM_AUDIT_OBJECT_ACE_FLAGS,
    pub ObjectType: windows_core::GUID,
    pub InheritedObjectType: windows_core::GUID,
    pub SidStart: u32,
}
impl windows_core::TypeKind for SYSTEM_ALARM_CALLBACK_OBJECT_ACE {
    type TypeKind = windows_core::CopyType;
}
impl Default for SYSTEM_ALARM_CALLBACK_OBJECT_ACE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SYSTEM_ALARM_OBJECT_ACE {
    pub Header: ACE_HEADER,
    pub Mask: u32,
    pub Flags: u32,
    pub ObjectType: windows_core::GUID,
    pub InheritedObjectType: windows_core::GUID,
    pub SidStart: u32,
}
impl windows_core::TypeKind for SYSTEM_ALARM_OBJECT_ACE {
    type TypeKind = windows_core::CopyType;
}
impl Default for SYSTEM_ALARM_OBJECT_ACE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SYSTEM_AUDIT_ACE {
    pub Header: ACE_HEADER,
    pub Mask: u32,
    pub SidStart: u32,
}
impl windows_core::TypeKind for SYSTEM_AUDIT_ACE {
    type TypeKind = windows_core::CopyType;
}
impl Default for SYSTEM_AUDIT_ACE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SYSTEM_AUDIT_CALLBACK_ACE {
    pub Header: ACE_HEADER,
    pub Mask: u32,
    pub SidStart: u32,
}
impl windows_core::TypeKind for SYSTEM_AUDIT_CALLBACK_ACE {
    type TypeKind = windows_core::CopyType;
}
impl Default for SYSTEM_AUDIT_CALLBACK_ACE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SYSTEM_AUDIT_CALLBACK_OBJECT_ACE {
    pub Header: ACE_HEADER,
    pub Mask: u32,
    pub Flags: SYSTEM_AUDIT_OBJECT_ACE_FLAGS,
    pub ObjectType: windows_core::GUID,
    pub InheritedObjectType: windows_core::GUID,
    pub SidStart: u32,
}
impl windows_core::TypeKind for SYSTEM_AUDIT_CALLBACK_OBJECT_ACE {
    type TypeKind = windows_core::CopyType;
}
impl Default for SYSTEM_AUDIT_CALLBACK_OBJECT_ACE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SYSTEM_AUDIT_OBJECT_ACE {
    pub Header: ACE_HEADER,
    pub Mask: u32,
    pub Flags: SYSTEM_AUDIT_OBJECT_ACE_FLAGS,
    pub ObjectType: windows_core::GUID,
    pub InheritedObjectType: windows_core::GUID,
    pub SidStart: u32,
}
impl windows_core::TypeKind for SYSTEM_AUDIT_OBJECT_ACE {
    type TypeKind = windows_core::CopyType;
}
impl Default for SYSTEM_AUDIT_OBJECT_ACE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SYSTEM_MANDATORY_LABEL_ACE {
    pub Header: ACE_HEADER,
    pub Mask: u32,
    pub SidStart: u32,
}
impl windows_core::TypeKind for SYSTEM_MANDATORY_LABEL_ACE {
    type TypeKind = windows_core::CopyType;
}
impl Default for SYSTEM_MANDATORY_LABEL_ACE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SYSTEM_PROCESS_TRUST_LABEL_ACE {
    pub Header: ACE_HEADER,
    pub Mask: u32,
    pub SidStart: u32,
}
impl windows_core::TypeKind for SYSTEM_PROCESS_TRUST_LABEL_ACE {
    type TypeKind = windows_core::CopyType;
}
impl Default for SYSTEM_PROCESS_TRUST_LABEL_ACE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SYSTEM_RESOURCE_ATTRIBUTE_ACE {
    pub Header: ACE_HEADER,
    pub Mask: u32,
    pub SidStart: u32,
}
impl windows_core::TypeKind for SYSTEM_RESOURCE_ATTRIBUTE_ACE {
    type TypeKind = windows_core::CopyType;
}
impl Default for SYSTEM_RESOURCE_ATTRIBUTE_ACE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SYSTEM_SCOPED_POLICY_ID_ACE {
    pub Header: ACE_HEADER,
    pub Mask: u32,
    pub SidStart: u32,
}
impl windows_core::TypeKind for SYSTEM_SCOPED_POLICY_ID_ACE {
    type TypeKind = windows_core::CopyType;
}
impl Default for SYSTEM_SCOPED_POLICY_ID_ACE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct TOKEN_ACCESS_INFORMATION {
    pub SidHash: *mut SID_AND_ATTRIBUTES_HASH,
    pub RestrictedSidHash: *mut SID_AND_ATTRIBUTES_HASH,
    pub Privileges: *mut TOKEN_PRIVILEGES,
    pub AuthenticationId: super::Foundation::LUID,
    pub TokenType: TOKEN_TYPE,
    pub ImpersonationLevel: SECURITY_IMPERSONATION_LEVEL,
    pub MandatoryPolicy: TOKEN_MANDATORY_POLICY,
    pub Flags: u32,
    pub AppContainerNumber: u32,
    pub PackageSid: super::Foundation::PSID,
    pub CapabilitiesHash: *mut SID_AND_ATTRIBUTES_HASH,
    pub TrustLevelSid: super::Foundation::PSID,
    pub SecurityAttributes: *mut core::ffi::c_void,
}
impl windows_core::TypeKind for TOKEN_ACCESS_INFORMATION {
    type TypeKind = windows_core::CopyType;
}
impl Default for TOKEN_ACCESS_INFORMATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct TOKEN_APPCONTAINER_INFORMATION {
    pub TokenAppContainer: super::Foundation::PSID,
}
impl windows_core::TypeKind for TOKEN_APPCONTAINER_INFORMATION {
    type TypeKind = windows_core::CopyType;
}
impl Default for TOKEN_APPCONTAINER_INFORMATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct TOKEN_AUDIT_POLICY {
    pub PerUserPolicy: [u8; 30],
}
impl windows_core::TypeKind for TOKEN_AUDIT_POLICY {
    type TypeKind = windows_core::CopyType;
}
impl Default for TOKEN_AUDIT_POLICY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct TOKEN_CONTROL {
    pub TokenId: super::Foundation::LUID,
    pub AuthenticationId: super::Foundation::LUID,
    pub ModifiedId: super::Foundation::LUID,
    pub TokenSource: TOKEN_SOURCE,
}
impl windows_core::TypeKind for TOKEN_CONTROL {
    type TypeKind = windows_core::CopyType;
}
impl Default for TOKEN_CONTROL {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct TOKEN_DEFAULT_DACL {
    pub DefaultDacl: *mut ACL,
}
impl windows_core::TypeKind for TOKEN_DEFAULT_DACL {
    type TypeKind = windows_core::CopyType;
}
impl Default for TOKEN_DEFAULT_DACL {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct TOKEN_DEVICE_CLAIMS {
    pub DeviceClaims: *mut core::ffi::c_void,
}
impl windows_core::TypeKind for TOKEN_DEVICE_CLAIMS {
    type TypeKind = windows_core::CopyType;
}
impl Default for TOKEN_DEVICE_CLAIMS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct TOKEN_ELEVATION {
    pub TokenIsElevated: u32,
}
impl windows_core::TypeKind for TOKEN_ELEVATION {
    type TypeKind = windows_core::CopyType;
}
impl Default for TOKEN_ELEVATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct TOKEN_GROUPS {
    pub GroupCount: u32,
    pub Groups: [SID_AND_ATTRIBUTES; 1],
}
impl windows_core::TypeKind for TOKEN_GROUPS {
    type TypeKind = windows_core::CopyType;
}
impl Default for TOKEN_GROUPS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct TOKEN_GROUPS_AND_PRIVILEGES {
    pub SidCount: u32,
    pub SidLength: u32,
    pub Sids: *mut SID_AND_ATTRIBUTES,
    pub RestrictedSidCount: u32,
    pub RestrictedSidLength: u32,
    pub RestrictedSids: *mut SID_AND_ATTRIBUTES,
    pub PrivilegeCount: u32,
    pub PrivilegeLength: u32,
    pub Privileges: *mut LUID_AND_ATTRIBUTES,
    pub AuthenticationId: super::Foundation::LUID,
}
impl windows_core::TypeKind for TOKEN_GROUPS_AND_PRIVILEGES {
    type TypeKind = windows_core::CopyType;
}
impl Default for TOKEN_GROUPS_AND_PRIVILEGES {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct TOKEN_LINKED_TOKEN {
    pub LinkedToken: super::Foundation::HANDLE,
}
impl windows_core::TypeKind for TOKEN_LINKED_TOKEN {
    type TypeKind = windows_core::CopyType;
}
impl Default for TOKEN_LINKED_TOKEN {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct TOKEN_MANDATORY_LABEL {
    pub Label: SID_AND_ATTRIBUTES,
}
impl windows_core::TypeKind for TOKEN_MANDATORY_LABEL {
    type TypeKind = windows_core::CopyType;
}
impl Default for TOKEN_MANDATORY_LABEL {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct TOKEN_MANDATORY_POLICY {
    pub Policy: TOKEN_MANDATORY_POLICY_ID,
}
impl windows_core::TypeKind for TOKEN_MANDATORY_POLICY {
    type TypeKind = windows_core::CopyType;
}
impl Default for TOKEN_MANDATORY_POLICY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct TOKEN_ORIGIN {
    pub OriginatingLogonSession: super::Foundation::LUID,
}
impl windows_core::TypeKind for TOKEN_ORIGIN {
    type TypeKind = windows_core::CopyType;
}
impl Default for TOKEN_ORIGIN {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct TOKEN_OWNER {
    pub Owner: super::Foundation::PSID,
}
impl windows_core::TypeKind for TOKEN_OWNER {
    type TypeKind = windows_core::CopyType;
}
impl Default for TOKEN_OWNER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct TOKEN_PRIMARY_GROUP {
    pub PrimaryGroup: super::Foundation::PSID,
}
impl windows_core::TypeKind for TOKEN_PRIMARY_GROUP {
    type TypeKind = windows_core::CopyType;
}
impl Default for TOKEN_PRIMARY_GROUP {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct TOKEN_PRIVILEGES {
    pub PrivilegeCount: u32,
    pub Privileges: [LUID_AND_ATTRIBUTES; 1],
}
impl windows_core::TypeKind for TOKEN_PRIVILEGES {
    type TypeKind = windows_core::CopyType;
}
impl Default for TOKEN_PRIVILEGES {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct TOKEN_SOURCE {
    pub SourceName: [i8; 8],
    pub SourceIdentifier: super::Foundation::LUID,
}
impl windows_core::TypeKind for TOKEN_SOURCE {
    type TypeKind = windows_core::CopyType;
}
impl Default for TOKEN_SOURCE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct TOKEN_STATISTICS {
    pub TokenId: super::Foundation::LUID,
    pub AuthenticationId: super::Foundation::LUID,
    pub ExpirationTime: i64,
    pub TokenType: TOKEN_TYPE,
    pub ImpersonationLevel: SECURITY_IMPERSONATION_LEVEL,
    pub DynamicCharged: u32,
    pub DynamicAvailable: u32,
    pub GroupCount: u32,
    pub PrivilegeCount: u32,
    pub ModifiedId: super::Foundation::LUID,
}
impl windows_core::TypeKind for TOKEN_STATISTICS {
    type TypeKind = windows_core::CopyType;
}
impl Default for TOKEN_STATISTICS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct TOKEN_USER {
    pub User: SID_AND_ATTRIBUTES,
}
impl windows_core::TypeKind for TOKEN_USER {
    type TypeKind = windows_core::CopyType;
}
impl Default for TOKEN_USER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct TOKEN_USER_CLAIMS {
    pub UserClaims: *mut core::ffi::c_void,
}
impl windows_core::TypeKind for TOKEN_USER_CLAIMS {
    type TypeKind = windows_core::CopyType;
}
impl Default for TOKEN_USER_CLAIMS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type PLSA_AP_CALL_PACKAGE_UNTRUSTED = Option<unsafe extern "system" fn(clientrequest: *const *const core::ffi::c_void, protocolsubmitbuffer: *const core::ffi::c_void, clientbufferbase: *const core::ffi::c_void, submitbufferlength: u32, protocolreturnbuffer: *mut *mut core::ffi::c_void, returnbufferlength: *mut u32, protocolstatus: *mut i32) -> super::Foundation::NTSTATUS>;
pub type SEC_THREAD_START = Option<unsafe extern "system" fn(lpthreadparameter: *mut core::ffi::c_void) -> u32>;
