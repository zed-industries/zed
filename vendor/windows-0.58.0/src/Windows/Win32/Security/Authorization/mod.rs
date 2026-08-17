#[cfg(feature = "Win32_Security_Authorization_UI")]
pub mod UI;
#[inline]
pub unsafe fn AuthzAccessCheck<P0, P1, P2>(flags: AUTHZ_ACCESS_CHECK_FLAGS, hauthzclientcontext: P0, prequest: *const AUTHZ_ACCESS_REQUEST, hauditevent: P1, psecuritydescriptor: P2, optionalsecuritydescriptorarray: Option<&[super::PSECURITY_DESCRIPTOR]>, preply: *mut AUTHZ_ACCESS_REPLY, phaccesscheckresults: Option<*mut AUTHZ_ACCESS_CHECK_RESULTS_HANDLE>) -> windows_core::Result<()>
where
    P0: windows_core::Param<AUTHZ_CLIENT_CONTEXT_HANDLE>,
    P1: windows_core::Param<AUTHZ_AUDIT_EVENT_HANDLE>,
    P2: windows_core::Param<super::PSECURITY_DESCRIPTOR>,
{
    windows_targets::link!("authz.dll" "system" fn AuthzAccessCheck(flags : AUTHZ_ACCESS_CHECK_FLAGS, hauthzclientcontext : AUTHZ_CLIENT_CONTEXT_HANDLE, prequest : *const AUTHZ_ACCESS_REQUEST, hauditevent : AUTHZ_AUDIT_EVENT_HANDLE, psecuritydescriptor : super:: PSECURITY_DESCRIPTOR, optionalsecuritydescriptorarray : *const super:: PSECURITY_DESCRIPTOR, optionalsecuritydescriptorcount : u32, preply : *mut AUTHZ_ACCESS_REPLY, phaccesscheckresults : *mut AUTHZ_ACCESS_CHECK_RESULTS_HANDLE) -> super::super::Foundation:: BOOL);
    AuthzAccessCheck(flags, hauthzclientcontext.param().abi(), prequest, hauditevent.param().abi(), psecuritydescriptor.param().abi(), core::mem::transmute(optionalsecuritydescriptorarray.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), optionalsecuritydescriptorarray.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), preply, core::mem::transmute(phaccesscheckresults.unwrap_or(std::ptr::null_mut()))).ok()
}
#[inline]
pub unsafe fn AuthzAddSidsToContext<P0>(hauthzclientcontext: P0, sids: Option<*const super::SID_AND_ATTRIBUTES>, sidcount: u32, restrictedsids: Option<*const super::SID_AND_ATTRIBUTES>, restrictedsidcount: u32, phnewauthzclientcontext: *mut AUTHZ_CLIENT_CONTEXT_HANDLE) -> windows_core::Result<()>
where
    P0: windows_core::Param<AUTHZ_CLIENT_CONTEXT_HANDLE>,
{
    windows_targets::link!("authz.dll" "system" fn AuthzAddSidsToContext(hauthzclientcontext : AUTHZ_CLIENT_CONTEXT_HANDLE, sids : *const super:: SID_AND_ATTRIBUTES, sidcount : u32, restrictedsids : *const super:: SID_AND_ATTRIBUTES, restrictedsidcount : u32, phnewauthzclientcontext : *mut AUTHZ_CLIENT_CONTEXT_HANDLE) -> super::super::Foundation:: BOOL);
    AuthzAddSidsToContext(hauthzclientcontext.param().abi(), core::mem::transmute(sids.unwrap_or(std::ptr::null())), sidcount, core::mem::transmute(restrictedsids.unwrap_or(std::ptr::null())), restrictedsidcount, phnewauthzclientcontext).ok()
}
#[inline]
pub unsafe fn AuthzCachedAccessCheck<P0, P1>(flags: u32, haccesscheckresults: P0, prequest: *const AUTHZ_ACCESS_REQUEST, hauditevent: P1, preply: *mut AUTHZ_ACCESS_REPLY) -> windows_core::Result<()>
where
    P0: windows_core::Param<AUTHZ_ACCESS_CHECK_RESULTS_HANDLE>,
    P1: windows_core::Param<AUTHZ_AUDIT_EVENT_HANDLE>,
{
    windows_targets::link!("authz.dll" "system" fn AuthzCachedAccessCheck(flags : u32, haccesscheckresults : AUTHZ_ACCESS_CHECK_RESULTS_HANDLE, prequest : *const AUTHZ_ACCESS_REQUEST, hauditevent : AUTHZ_AUDIT_EVENT_HANDLE, preply : *mut AUTHZ_ACCESS_REPLY) -> super::super::Foundation:: BOOL);
    AuthzCachedAccessCheck(flags, haccesscheckresults.param().abi(), prequest, hauditevent.param().abi(), preply).ok()
}
#[inline]
pub unsafe fn AuthzEnumerateSecurityEventSources(dwflags: u32, buffer: *mut AUTHZ_SOURCE_SCHEMA_REGISTRATION, pdwcount: *mut u32, pdwlength: *mut u32) -> windows_core::Result<()> {
    windows_targets::link!("authz.dll" "system" fn AuthzEnumerateSecurityEventSources(dwflags : u32, buffer : *mut AUTHZ_SOURCE_SCHEMA_REGISTRATION, pdwcount : *mut u32, pdwlength : *mut u32) -> super::super::Foundation:: BOOL);
    AuthzEnumerateSecurityEventSources(dwflags, buffer, pdwcount, pdwlength).ok()
}
#[inline]
pub unsafe fn AuthzEvaluateSacl<P0, P1>(authzclientcontext: P0, prequest: *const AUTHZ_ACCESS_REQUEST, sacl: *const super::ACL, grantedaccess: u32, accessgranted: P1, pbgenerateaudit: *mut super::super::Foundation::BOOL) -> super::super::Foundation::BOOL
where
    P0: windows_core::Param<AUTHZ_CLIENT_CONTEXT_HANDLE>,
    P1: windows_core::Param<super::super::Foundation::BOOL>,
{
    windows_targets::link!("authz.dll" "system" fn AuthzEvaluateSacl(authzclientcontext : AUTHZ_CLIENT_CONTEXT_HANDLE, prequest : *const AUTHZ_ACCESS_REQUEST, sacl : *const super:: ACL, grantedaccess : u32, accessgranted : super::super::Foundation:: BOOL, pbgenerateaudit : *mut super::super::Foundation:: BOOL) -> super::super::Foundation:: BOOL);
    AuthzEvaluateSacl(authzclientcontext.param().abi(), prequest, sacl, grantedaccess, accessgranted.param().abi(), pbgenerateaudit)
}
#[inline]
pub unsafe fn AuthzFreeAuditEvent<P0>(hauditevent: P0) -> windows_core::Result<()>
where
    P0: windows_core::Param<AUTHZ_AUDIT_EVENT_HANDLE>,
{
    windows_targets::link!("authz.dll" "system" fn AuthzFreeAuditEvent(hauditevent : AUTHZ_AUDIT_EVENT_HANDLE) -> super::super::Foundation:: BOOL);
    AuthzFreeAuditEvent(hauditevent.param().abi()).ok()
}
#[inline]
pub unsafe fn AuthzFreeCentralAccessPolicyCache() -> windows_core::Result<()> {
    windows_targets::link!("authz.dll" "system" fn AuthzFreeCentralAccessPolicyCache() -> super::super::Foundation:: BOOL);
    AuthzFreeCentralAccessPolicyCache().ok()
}
#[inline]
pub unsafe fn AuthzFreeContext<P0>(hauthzclientcontext: P0) -> windows_core::Result<()>
where
    P0: windows_core::Param<AUTHZ_CLIENT_CONTEXT_HANDLE>,
{
    windows_targets::link!("authz.dll" "system" fn AuthzFreeContext(hauthzclientcontext : AUTHZ_CLIENT_CONTEXT_HANDLE) -> super::super::Foundation:: BOOL);
    AuthzFreeContext(hauthzclientcontext.param().abi()).ok()
}
#[inline]
pub unsafe fn AuthzFreeHandle(haccesscheckresults: AUTHZ_ACCESS_CHECK_RESULTS_HANDLE) -> windows_core::Result<()> {
    windows_targets::link!("authz.dll" "system" fn AuthzFreeHandle(haccesscheckresults : AUTHZ_ACCESS_CHECK_RESULTS_HANDLE) -> super::super::Foundation:: BOOL);
    AuthzFreeHandle(haccesscheckresults).ok()
}
#[inline]
pub unsafe fn AuthzFreeResourceManager<P0>(hauthzresourcemanager: P0) -> windows_core::Result<()>
where
    P0: windows_core::Param<AUTHZ_RESOURCE_MANAGER_HANDLE>,
{
    windows_targets::link!("authz.dll" "system" fn AuthzFreeResourceManager(hauthzresourcemanager : AUTHZ_RESOURCE_MANAGER_HANDLE) -> super::super::Foundation:: BOOL);
    AuthzFreeResourceManager(hauthzresourcemanager.param().abi()).ok()
}
#[inline]
pub unsafe fn AuthzGetInformationFromContext<P0>(hauthzclientcontext: P0, infoclass: AUTHZ_CONTEXT_INFORMATION_CLASS, buffersize: u32, psizerequired: *mut u32, buffer: *mut core::ffi::c_void) -> windows_core::Result<()>
where
    P0: windows_core::Param<AUTHZ_CLIENT_CONTEXT_HANDLE>,
{
    windows_targets::link!("authz.dll" "system" fn AuthzGetInformationFromContext(hauthzclientcontext : AUTHZ_CLIENT_CONTEXT_HANDLE, infoclass : AUTHZ_CONTEXT_INFORMATION_CLASS, buffersize : u32, psizerequired : *mut u32, buffer : *mut core::ffi::c_void) -> super::super::Foundation:: BOOL);
    AuthzGetInformationFromContext(hauthzclientcontext.param().abi(), infoclass, buffersize, psizerequired, buffer).ok()
}
#[inline]
pub unsafe fn AuthzInitializeCompoundContext<P0, P1>(usercontext: P0, devicecontext: P1, phcompoundcontext: *mut AUTHZ_CLIENT_CONTEXT_HANDLE) -> windows_core::Result<()>
where
    P0: windows_core::Param<AUTHZ_CLIENT_CONTEXT_HANDLE>,
    P1: windows_core::Param<AUTHZ_CLIENT_CONTEXT_HANDLE>,
{
    windows_targets::link!("authz.dll" "system" fn AuthzInitializeCompoundContext(usercontext : AUTHZ_CLIENT_CONTEXT_HANDLE, devicecontext : AUTHZ_CLIENT_CONTEXT_HANDLE, phcompoundcontext : *mut AUTHZ_CLIENT_CONTEXT_HANDLE) -> super::super::Foundation:: BOOL);
    AuthzInitializeCompoundContext(usercontext.param().abi(), devicecontext.param().abi(), phcompoundcontext).ok()
}
#[inline]
pub unsafe fn AuthzInitializeContextFromAuthzContext<P0>(flags: u32, hauthzclientcontext: P0, pexpirationtime: Option<*const i64>, identifier: super::super::Foundation::LUID, dynamicgroupargs: *const core::ffi::c_void, phnewauthzclientcontext: *mut AUTHZ_CLIENT_CONTEXT_HANDLE) -> windows_core::Result<()>
where
    P0: windows_core::Param<AUTHZ_CLIENT_CONTEXT_HANDLE>,
{
    windows_targets::link!("authz.dll" "system" fn AuthzInitializeContextFromAuthzContext(flags : u32, hauthzclientcontext : AUTHZ_CLIENT_CONTEXT_HANDLE, pexpirationtime : *const i64, identifier : super::super::Foundation:: LUID, dynamicgroupargs : *const core::ffi::c_void, phnewauthzclientcontext : *mut AUTHZ_CLIENT_CONTEXT_HANDLE) -> super::super::Foundation:: BOOL);
    AuthzInitializeContextFromAuthzContext(flags, hauthzclientcontext.param().abi(), core::mem::transmute(pexpirationtime.unwrap_or(std::ptr::null())), core::mem::transmute(identifier), dynamicgroupargs, phnewauthzclientcontext).ok()
}
#[inline]
pub unsafe fn AuthzInitializeContextFromSid<P0, P1>(flags: u32, usersid: P0, hauthzresourcemanager: P1, pexpirationtime: Option<*const i64>, identifier: super::super::Foundation::LUID, dynamicgroupargs: Option<*const core::ffi::c_void>, phauthzclientcontext: *mut AUTHZ_CLIENT_CONTEXT_HANDLE) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::PSID>,
    P1: windows_core::Param<AUTHZ_RESOURCE_MANAGER_HANDLE>,
{
    windows_targets::link!("authz.dll" "system" fn AuthzInitializeContextFromSid(flags : u32, usersid : super:: PSID, hauthzresourcemanager : AUTHZ_RESOURCE_MANAGER_HANDLE, pexpirationtime : *const i64, identifier : super::super::Foundation:: LUID, dynamicgroupargs : *const core::ffi::c_void, phauthzclientcontext : *mut AUTHZ_CLIENT_CONTEXT_HANDLE) -> super::super::Foundation:: BOOL);
    AuthzInitializeContextFromSid(flags, usersid.param().abi(), hauthzresourcemanager.param().abi(), core::mem::transmute(pexpirationtime.unwrap_or(std::ptr::null())), core::mem::transmute(identifier), core::mem::transmute(dynamicgroupargs.unwrap_or(std::ptr::null())), phauthzclientcontext).ok()
}
#[inline]
pub unsafe fn AuthzInitializeContextFromToken<P0, P1>(flags: u32, tokenhandle: P0, hauthzresourcemanager: P1, pexpirationtime: Option<*const i64>, identifier: super::super::Foundation::LUID, dynamicgroupargs: Option<*const core::ffi::c_void>, phauthzclientcontext: *mut AUTHZ_CLIENT_CONTEXT_HANDLE) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::super::Foundation::HANDLE>,
    P1: windows_core::Param<AUTHZ_RESOURCE_MANAGER_HANDLE>,
{
    windows_targets::link!("authz.dll" "system" fn AuthzInitializeContextFromToken(flags : u32, tokenhandle : super::super::Foundation:: HANDLE, hauthzresourcemanager : AUTHZ_RESOURCE_MANAGER_HANDLE, pexpirationtime : *const i64, identifier : super::super::Foundation:: LUID, dynamicgroupargs : *const core::ffi::c_void, phauthzclientcontext : *mut AUTHZ_CLIENT_CONTEXT_HANDLE) -> super::super::Foundation:: BOOL);
    AuthzInitializeContextFromToken(flags, tokenhandle.param().abi(), hauthzresourcemanager.param().abi(), core::mem::transmute(pexpirationtime.unwrap_or(std::ptr::null())), core::mem::transmute(identifier), core::mem::transmute(dynamicgroupargs.unwrap_or(std::ptr::null())), phauthzclientcontext).ok()
}
#[inline]
pub unsafe fn AuthzInitializeObjectAccessAuditEvent<P0, P1, P2, P3, P4>(flags: AUTHZ_INITIALIZE_OBJECT_ACCESS_AUDIT_EVENT_FLAGS, hauditeventtype: P0, szoperationtype: P1, szobjecttype: P2, szobjectname: P3, szadditionalinfo: P4, phauditevent: *mut AUTHZ_AUDIT_EVENT_HANDLE, dwadditionalparametercount: u32) -> windows_core::Result<()>
where
    P0: windows_core::Param<AUTHZ_AUDIT_EVENT_TYPE_HANDLE>,
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
    P3: windows_core::Param<windows_core::PCWSTR>,
    P4: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("authz.dll" "cdecl" fn AuthzInitializeObjectAccessAuditEvent(flags : AUTHZ_INITIALIZE_OBJECT_ACCESS_AUDIT_EVENT_FLAGS, hauditeventtype : AUTHZ_AUDIT_EVENT_TYPE_HANDLE, szoperationtype : windows_core::PCWSTR, szobjecttype : windows_core::PCWSTR, szobjectname : windows_core::PCWSTR, szadditionalinfo : windows_core::PCWSTR, phauditevent : *mut AUTHZ_AUDIT_EVENT_HANDLE, dwadditionalparametercount : u32) -> super::super::Foundation:: BOOL);
    AuthzInitializeObjectAccessAuditEvent(flags, hauditeventtype.param().abi(), szoperationtype.param().abi(), szobjecttype.param().abi(), szobjectname.param().abi(), szadditionalinfo.param().abi(), phauditevent, dwadditionalparametercount).ok()
}
#[inline]
pub unsafe fn AuthzInitializeObjectAccessAuditEvent2<P0, P1, P2, P3, P4, P5>(flags: u32, hauditeventtype: P0, szoperationtype: P1, szobjecttype: P2, szobjectname: P3, szadditionalinfo: P4, szadditionalinfo2: P5, phauditevent: *mut AUTHZ_AUDIT_EVENT_HANDLE, dwadditionalparametercount: u32) -> windows_core::Result<()>
where
    P0: windows_core::Param<AUTHZ_AUDIT_EVENT_TYPE_HANDLE>,
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
    P3: windows_core::Param<windows_core::PCWSTR>,
    P4: windows_core::Param<windows_core::PCWSTR>,
    P5: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("authz.dll" "cdecl" fn AuthzInitializeObjectAccessAuditEvent2(flags : u32, hauditeventtype : AUTHZ_AUDIT_EVENT_TYPE_HANDLE, szoperationtype : windows_core::PCWSTR, szobjecttype : windows_core::PCWSTR, szobjectname : windows_core::PCWSTR, szadditionalinfo : windows_core::PCWSTR, szadditionalinfo2 : windows_core::PCWSTR, phauditevent : *mut AUTHZ_AUDIT_EVENT_HANDLE, dwadditionalparametercount : u32) -> super::super::Foundation:: BOOL);
    AuthzInitializeObjectAccessAuditEvent2(flags, hauditeventtype.param().abi(), szoperationtype.param().abi(), szobjecttype.param().abi(), szobjectname.param().abi(), szadditionalinfo.param().abi(), szadditionalinfo2.param().abi(), phauditevent, dwadditionalparametercount).ok()
}
#[inline]
pub unsafe fn AuthzInitializeRemoteResourceManager(prpcinitinfo: *const AUTHZ_RPC_INIT_INFO_CLIENT, phauthzresourcemanager: *mut AUTHZ_RESOURCE_MANAGER_HANDLE) -> windows_core::Result<()> {
    windows_targets::link!("authz.dll" "system" fn AuthzInitializeRemoteResourceManager(prpcinitinfo : *const AUTHZ_RPC_INIT_INFO_CLIENT, phauthzresourcemanager : *mut AUTHZ_RESOURCE_MANAGER_HANDLE) -> super::super::Foundation:: BOOL);
    AuthzInitializeRemoteResourceManager(prpcinitinfo, phauthzresourcemanager).ok()
}
#[inline]
pub unsafe fn AuthzInitializeResourceManager<P0>(flags: u32, pfndynamicaccesscheck: PFN_AUTHZ_DYNAMIC_ACCESS_CHECK, pfncomputedynamicgroups: PFN_AUTHZ_COMPUTE_DYNAMIC_GROUPS, pfnfreedynamicgroups: PFN_AUTHZ_FREE_DYNAMIC_GROUPS, szresourcemanagername: P0, phauthzresourcemanager: *mut AUTHZ_RESOURCE_MANAGER_HANDLE) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("authz.dll" "system" fn AuthzInitializeResourceManager(flags : u32, pfndynamicaccesscheck : PFN_AUTHZ_DYNAMIC_ACCESS_CHECK, pfncomputedynamicgroups : PFN_AUTHZ_COMPUTE_DYNAMIC_GROUPS, pfnfreedynamicgroups : PFN_AUTHZ_FREE_DYNAMIC_GROUPS, szresourcemanagername : windows_core::PCWSTR, phauthzresourcemanager : *mut AUTHZ_RESOURCE_MANAGER_HANDLE) -> super::super::Foundation:: BOOL);
    AuthzInitializeResourceManager(flags, pfndynamicaccesscheck, pfncomputedynamicgroups, pfnfreedynamicgroups, szresourcemanagername.param().abi(), phauthzresourcemanager).ok()
}
#[inline]
pub unsafe fn AuthzInitializeResourceManagerEx(flags: AUTHZ_RESOURCE_MANAGER_FLAGS, pauthzinitinfo: Option<*const AUTHZ_INIT_INFO>, phauthzresourcemanager: *mut AUTHZ_RESOURCE_MANAGER_HANDLE) -> windows_core::Result<()> {
    windows_targets::link!("authz.dll" "system" fn AuthzInitializeResourceManagerEx(flags : AUTHZ_RESOURCE_MANAGER_FLAGS, pauthzinitinfo : *const AUTHZ_INIT_INFO, phauthzresourcemanager : *mut AUTHZ_RESOURCE_MANAGER_HANDLE) -> super::super::Foundation:: BOOL);
    AuthzInitializeResourceManagerEx(flags, core::mem::transmute(pauthzinitinfo.unwrap_or(std::ptr::null())), phauthzresourcemanager).ok()
}
#[inline]
pub unsafe fn AuthzInstallSecurityEventSource(dwflags: u32, pregistration: *const AUTHZ_SOURCE_SCHEMA_REGISTRATION) -> windows_core::Result<()> {
    windows_targets::link!("authz.dll" "system" fn AuthzInstallSecurityEventSource(dwflags : u32, pregistration : *const AUTHZ_SOURCE_SCHEMA_REGISTRATION) -> super::super::Foundation:: BOOL);
    AuthzInstallSecurityEventSource(dwflags, pregistration).ok()
}
#[inline]
pub unsafe fn AuthzModifyClaims<P0>(hauthzclientcontext: P0, claimclass: AUTHZ_CONTEXT_INFORMATION_CLASS, pclaimoperations: *const AUTHZ_SECURITY_ATTRIBUTE_OPERATION, pclaims: Option<*const AUTHZ_SECURITY_ATTRIBUTES_INFORMATION>) -> windows_core::Result<()>
where
    P0: windows_core::Param<AUTHZ_CLIENT_CONTEXT_HANDLE>,
{
    windows_targets::link!("authz.dll" "system" fn AuthzModifyClaims(hauthzclientcontext : AUTHZ_CLIENT_CONTEXT_HANDLE, claimclass : AUTHZ_CONTEXT_INFORMATION_CLASS, pclaimoperations : *const AUTHZ_SECURITY_ATTRIBUTE_OPERATION, pclaims : *const AUTHZ_SECURITY_ATTRIBUTES_INFORMATION) -> super::super::Foundation:: BOOL);
    AuthzModifyClaims(hauthzclientcontext.param().abi(), claimclass, pclaimoperations, core::mem::transmute(pclaims.unwrap_or(std::ptr::null()))).ok()
}
#[inline]
pub unsafe fn AuthzModifySecurityAttributes<P0>(hauthzclientcontext: P0, poperations: *const AUTHZ_SECURITY_ATTRIBUTE_OPERATION, pattributes: Option<*const AUTHZ_SECURITY_ATTRIBUTES_INFORMATION>) -> windows_core::Result<()>
where
    P0: windows_core::Param<AUTHZ_CLIENT_CONTEXT_HANDLE>,
{
    windows_targets::link!("authz.dll" "system" fn AuthzModifySecurityAttributes(hauthzclientcontext : AUTHZ_CLIENT_CONTEXT_HANDLE, poperations : *const AUTHZ_SECURITY_ATTRIBUTE_OPERATION, pattributes : *const AUTHZ_SECURITY_ATTRIBUTES_INFORMATION) -> super::super::Foundation:: BOOL);
    AuthzModifySecurityAttributes(hauthzclientcontext.param().abi(), poperations, core::mem::transmute(pattributes.unwrap_or(std::ptr::null()))).ok()
}
#[inline]
pub unsafe fn AuthzModifySids<P0>(hauthzclientcontext: P0, sidclass: AUTHZ_CONTEXT_INFORMATION_CLASS, psidoperations: *const AUTHZ_SID_OPERATION, psids: Option<*const super::TOKEN_GROUPS>) -> windows_core::Result<()>
where
    P0: windows_core::Param<AUTHZ_CLIENT_CONTEXT_HANDLE>,
{
    windows_targets::link!("authz.dll" "system" fn AuthzModifySids(hauthzclientcontext : AUTHZ_CLIENT_CONTEXT_HANDLE, sidclass : AUTHZ_CONTEXT_INFORMATION_CLASS, psidoperations : *const AUTHZ_SID_OPERATION, psids : *const super:: TOKEN_GROUPS) -> super::super::Foundation:: BOOL);
    AuthzModifySids(hauthzclientcontext.param().abi(), sidclass, psidoperations, core::mem::transmute(psids.unwrap_or(std::ptr::null()))).ok()
}
#[inline]
pub unsafe fn AuthzOpenObjectAudit<P0, P1, P2>(flags: u32, hauthzclientcontext: P0, prequest: *const AUTHZ_ACCESS_REQUEST, hauditevent: P1, psecuritydescriptor: P2, optionalsecuritydescriptorarray: Option<&[super::PSECURITY_DESCRIPTOR]>, preply: *const AUTHZ_ACCESS_REPLY) -> windows_core::Result<()>
where
    P0: windows_core::Param<AUTHZ_CLIENT_CONTEXT_HANDLE>,
    P1: windows_core::Param<AUTHZ_AUDIT_EVENT_HANDLE>,
    P2: windows_core::Param<super::PSECURITY_DESCRIPTOR>,
{
    windows_targets::link!("authz.dll" "system" fn AuthzOpenObjectAudit(flags : u32, hauthzclientcontext : AUTHZ_CLIENT_CONTEXT_HANDLE, prequest : *const AUTHZ_ACCESS_REQUEST, hauditevent : AUTHZ_AUDIT_EVENT_HANDLE, psecuritydescriptor : super:: PSECURITY_DESCRIPTOR, optionalsecuritydescriptorarray : *const super:: PSECURITY_DESCRIPTOR, optionalsecuritydescriptorcount : u32, preply : *const AUTHZ_ACCESS_REPLY) -> super::super::Foundation:: BOOL);
    AuthzOpenObjectAudit(flags, hauthzclientcontext.param().abi(), prequest, hauditevent.param().abi(), psecuritydescriptor.param().abi(), core::mem::transmute(optionalsecuritydescriptorarray.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), optionalsecuritydescriptorarray.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), preply).ok()
}
#[cfg(feature = "Win32_System_Threading")]
#[inline]
pub unsafe fn AuthzRegisterCapChangeNotification(phcapchangesubscription: *mut AUTHZ_CAP_CHANGE_SUBSCRIPTION_HANDLE, pfncapchangecallback: super::super::System::Threading::LPTHREAD_START_ROUTINE, pcallbackcontext: Option<*const core::ffi::c_void>) -> windows_core::Result<()> {
    windows_targets::link!("authz.dll" "system" fn AuthzRegisterCapChangeNotification(phcapchangesubscription : *mut AUTHZ_CAP_CHANGE_SUBSCRIPTION_HANDLE, pfncapchangecallback : super::super::System::Threading:: LPTHREAD_START_ROUTINE, pcallbackcontext : *const core::ffi::c_void) -> super::super::Foundation:: BOOL);
    AuthzRegisterCapChangeNotification(phcapchangesubscription, pfncapchangecallback, core::mem::transmute(pcallbackcontext.unwrap_or(std::ptr::null()))).ok()
}
#[inline]
pub unsafe fn AuthzRegisterSecurityEventSource<P0>(dwflags: u32, szeventsourcename: P0, pheventprovider: *mut AUTHZ_SECURITY_EVENT_PROVIDER_HANDLE) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("authz.dll" "system" fn AuthzRegisterSecurityEventSource(dwflags : u32, szeventsourcename : windows_core::PCWSTR, pheventprovider : *mut AUTHZ_SECURITY_EVENT_PROVIDER_HANDLE) -> super::super::Foundation:: BOOL);
    AuthzRegisterSecurityEventSource(dwflags, szeventsourcename.param().abi(), pheventprovider).ok()
}
#[inline]
pub unsafe fn AuthzReportSecurityEvent<P0>(dwflags: u32, heventprovider: AUTHZ_SECURITY_EVENT_PROVIDER_HANDLE, dwauditid: u32, pusersid: P0, dwcount: u32) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::PSID>,
{
    windows_targets::link!("authz.dll" "cdecl" fn AuthzReportSecurityEvent(dwflags : u32, heventprovider : AUTHZ_SECURITY_EVENT_PROVIDER_HANDLE, dwauditid : u32, pusersid : super:: PSID, dwcount : u32) -> super::super::Foundation:: BOOL);
    AuthzReportSecurityEvent(dwflags, heventprovider, dwauditid, pusersid.param().abi(), dwcount).ok()
}
#[inline]
pub unsafe fn AuthzReportSecurityEventFromParams<P0>(dwflags: u32, heventprovider: AUTHZ_SECURITY_EVENT_PROVIDER_HANDLE, dwauditid: u32, pusersid: P0, pparams: *const AUDIT_PARAMS) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::PSID>,
{
    windows_targets::link!("authz.dll" "system" fn AuthzReportSecurityEventFromParams(dwflags : u32, heventprovider : AUTHZ_SECURITY_EVENT_PROVIDER_HANDLE, dwauditid : u32, pusersid : super:: PSID, pparams : *const AUDIT_PARAMS) -> super::super::Foundation:: BOOL);
    AuthzReportSecurityEventFromParams(dwflags, heventprovider, dwauditid, pusersid.param().abi(), pparams).ok()
}
#[inline]
pub unsafe fn AuthzSetAppContainerInformation<P0, P1>(hauthzclientcontext: P0, pappcontainersid: P1, pcapabilitysids: Option<&[super::SID_AND_ATTRIBUTES]>) -> windows_core::Result<()>
where
    P0: windows_core::Param<AUTHZ_CLIENT_CONTEXT_HANDLE>,
    P1: windows_core::Param<super::PSID>,
{
    windows_targets::link!("authz.dll" "system" fn AuthzSetAppContainerInformation(hauthzclientcontext : AUTHZ_CLIENT_CONTEXT_HANDLE, pappcontainersid : super:: PSID, capabilitycount : u32, pcapabilitysids : *const super:: SID_AND_ATTRIBUTES) -> super::super::Foundation:: BOOL);
    AuthzSetAppContainerInformation(hauthzclientcontext.param().abi(), pappcontainersid.param().abi(), pcapabilitysids.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), core::mem::transmute(pcapabilitysids.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr()))).ok()
}
#[inline]
pub unsafe fn AuthzUninstallSecurityEventSource<P0>(dwflags: u32, szeventsourcename: P0) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("authz.dll" "system" fn AuthzUninstallSecurityEventSource(dwflags : u32, szeventsourcename : windows_core::PCWSTR) -> super::super::Foundation:: BOOL);
    AuthzUninstallSecurityEventSource(dwflags, szeventsourcename.param().abi()).ok()
}
#[inline]
pub unsafe fn AuthzUnregisterCapChangeNotification<P0>(hcapchangesubscription: P0) -> windows_core::Result<()>
where
    P0: windows_core::Param<AUTHZ_CAP_CHANGE_SUBSCRIPTION_HANDLE>,
{
    windows_targets::link!("authz.dll" "system" fn AuthzUnregisterCapChangeNotification(hcapchangesubscription : AUTHZ_CAP_CHANGE_SUBSCRIPTION_HANDLE) -> super::super::Foundation:: BOOL);
    AuthzUnregisterCapChangeNotification(hcapchangesubscription.param().abi()).ok()
}
#[inline]
pub unsafe fn AuthzUnregisterSecurityEventSource(dwflags: u32, pheventprovider: *mut AUTHZ_SECURITY_EVENT_PROVIDER_HANDLE) -> windows_core::Result<()> {
    windows_targets::link!("authz.dll" "system" fn AuthzUnregisterSecurityEventSource(dwflags : u32, pheventprovider : *mut AUTHZ_SECURITY_EVENT_PROVIDER_HANDLE) -> super::super::Foundation:: BOOL);
    AuthzUnregisterSecurityEventSource(dwflags, pheventprovider).ok()
}
#[inline]
pub unsafe fn BuildExplicitAccessWithNameA<P0>(pexplicitaccess: *mut EXPLICIT_ACCESS_A, ptrusteename: P0, accesspermissions: u32, accessmode: ACCESS_MODE, inheritance: super::ACE_FLAGS)
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("advapi32.dll" "system" fn BuildExplicitAccessWithNameA(pexplicitaccess : *mut EXPLICIT_ACCESS_A, ptrusteename : windows_core::PCSTR, accesspermissions : u32, accessmode : ACCESS_MODE, inheritance : super:: ACE_FLAGS));
    BuildExplicitAccessWithNameA(pexplicitaccess, ptrusteename.param().abi(), accesspermissions, accessmode, inheritance)
}
#[inline]
pub unsafe fn BuildExplicitAccessWithNameW<P0>(pexplicitaccess: *mut EXPLICIT_ACCESS_W, ptrusteename: P0, accesspermissions: u32, accessmode: ACCESS_MODE, inheritance: super::ACE_FLAGS)
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("advapi32.dll" "system" fn BuildExplicitAccessWithNameW(pexplicitaccess : *mut EXPLICIT_ACCESS_W, ptrusteename : windows_core::PCWSTR, accesspermissions : u32, accessmode : ACCESS_MODE, inheritance : super:: ACE_FLAGS));
    BuildExplicitAccessWithNameW(pexplicitaccess, ptrusteename.param().abi(), accesspermissions, accessmode, inheritance)
}
#[inline]
pub unsafe fn BuildImpersonateExplicitAccessWithNameA<P0>(pexplicitaccess: *mut EXPLICIT_ACCESS_A, ptrusteename: P0, ptrustee: Option<*const TRUSTEE_A>, accesspermissions: u32, accessmode: ACCESS_MODE, inheritance: u32)
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("advapi32.dll" "system" fn BuildImpersonateExplicitAccessWithNameA(pexplicitaccess : *mut EXPLICIT_ACCESS_A, ptrusteename : windows_core::PCSTR, ptrustee : *const TRUSTEE_A, accesspermissions : u32, accessmode : ACCESS_MODE, inheritance : u32));
    BuildImpersonateExplicitAccessWithNameA(pexplicitaccess, ptrusteename.param().abi(), core::mem::transmute(ptrustee.unwrap_or(std::ptr::null())), accesspermissions, accessmode, inheritance)
}
#[inline]
pub unsafe fn BuildImpersonateExplicitAccessWithNameW<P0>(pexplicitaccess: *mut EXPLICIT_ACCESS_W, ptrusteename: P0, ptrustee: Option<*const TRUSTEE_W>, accesspermissions: u32, accessmode: ACCESS_MODE, inheritance: u32)
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("advapi32.dll" "system" fn BuildImpersonateExplicitAccessWithNameW(pexplicitaccess : *mut EXPLICIT_ACCESS_W, ptrusteename : windows_core::PCWSTR, ptrustee : *const TRUSTEE_W, accesspermissions : u32, accessmode : ACCESS_MODE, inheritance : u32));
    BuildImpersonateExplicitAccessWithNameW(pexplicitaccess, ptrusteename.param().abi(), core::mem::transmute(ptrustee.unwrap_or(std::ptr::null())), accesspermissions, accessmode, inheritance)
}
#[inline]
pub unsafe fn BuildImpersonateTrusteeA(ptrustee: *mut TRUSTEE_A, pimpersonatetrustee: Option<*const TRUSTEE_A>) {
    windows_targets::link!("advapi32.dll" "system" fn BuildImpersonateTrusteeA(ptrustee : *mut TRUSTEE_A, pimpersonatetrustee : *const TRUSTEE_A));
    BuildImpersonateTrusteeA(ptrustee, core::mem::transmute(pimpersonatetrustee.unwrap_or(std::ptr::null())))
}
#[inline]
pub unsafe fn BuildImpersonateTrusteeW(ptrustee: *mut TRUSTEE_W, pimpersonatetrustee: Option<*const TRUSTEE_W>) {
    windows_targets::link!("advapi32.dll" "system" fn BuildImpersonateTrusteeW(ptrustee : *mut TRUSTEE_W, pimpersonatetrustee : *const TRUSTEE_W));
    BuildImpersonateTrusteeW(ptrustee, core::mem::transmute(pimpersonatetrustee.unwrap_or(std::ptr::null())))
}
#[inline]
pub unsafe fn BuildSecurityDescriptorA<P0>(powner: Option<*const TRUSTEE_A>, pgroup: Option<*const TRUSTEE_A>, plistofaccessentries: Option<&[EXPLICIT_ACCESS_A]>, plistofauditentries: Option<&[EXPLICIT_ACCESS_A]>, poldsd: P0, psizenewsd: *mut u32, pnewsd: *mut super::PSECURITY_DESCRIPTOR) -> super::super::Foundation::WIN32_ERROR
where
    P0: windows_core::Param<super::PSECURITY_DESCRIPTOR>,
{
    windows_targets::link!("advapi32.dll" "system" fn BuildSecurityDescriptorA(powner : *const TRUSTEE_A, pgroup : *const TRUSTEE_A, ccountofaccessentries : u32, plistofaccessentries : *const EXPLICIT_ACCESS_A, ccountofauditentries : u32, plistofauditentries : *const EXPLICIT_ACCESS_A, poldsd : super:: PSECURITY_DESCRIPTOR, psizenewsd : *mut u32, pnewsd : *mut super:: PSECURITY_DESCRIPTOR) -> super::super::Foundation:: WIN32_ERROR);
    BuildSecurityDescriptorA(
        core::mem::transmute(powner.unwrap_or(std::ptr::null())),
        core::mem::transmute(pgroup.unwrap_or(std::ptr::null())),
        plistofaccessentries.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()),
        core::mem::transmute(plistofaccessentries.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())),
        plistofauditentries.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()),
        core::mem::transmute(plistofauditentries.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())),
        poldsd.param().abi(),
        psizenewsd,
        pnewsd,
    )
}
#[inline]
pub unsafe fn BuildSecurityDescriptorW<P0>(powner: Option<*const TRUSTEE_W>, pgroup: Option<*const TRUSTEE_W>, plistofaccessentries: Option<&[EXPLICIT_ACCESS_W]>, plistofauditentries: Option<&[EXPLICIT_ACCESS_W]>, poldsd: P0, psizenewsd: *mut u32, pnewsd: *mut super::PSECURITY_DESCRIPTOR) -> super::super::Foundation::WIN32_ERROR
where
    P0: windows_core::Param<super::PSECURITY_DESCRIPTOR>,
{
    windows_targets::link!("advapi32.dll" "system" fn BuildSecurityDescriptorW(powner : *const TRUSTEE_W, pgroup : *const TRUSTEE_W, ccountofaccessentries : u32, plistofaccessentries : *const EXPLICIT_ACCESS_W, ccountofauditentries : u32, plistofauditentries : *const EXPLICIT_ACCESS_W, poldsd : super:: PSECURITY_DESCRIPTOR, psizenewsd : *mut u32, pnewsd : *mut super:: PSECURITY_DESCRIPTOR) -> super::super::Foundation:: WIN32_ERROR);
    BuildSecurityDescriptorW(
        core::mem::transmute(powner.unwrap_or(std::ptr::null())),
        core::mem::transmute(pgroup.unwrap_or(std::ptr::null())),
        plistofaccessentries.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()),
        core::mem::transmute(plistofaccessentries.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())),
        plistofauditentries.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()),
        core::mem::transmute(plistofauditentries.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())),
        poldsd.param().abi(),
        psizenewsd,
        pnewsd,
    )
}
#[inline]
pub unsafe fn BuildTrusteeWithNameA<P0>(ptrustee: *mut TRUSTEE_A, pname: P0)
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("advapi32.dll" "system" fn BuildTrusteeWithNameA(ptrustee : *mut TRUSTEE_A, pname : windows_core::PCSTR));
    BuildTrusteeWithNameA(ptrustee, pname.param().abi())
}
#[inline]
pub unsafe fn BuildTrusteeWithNameW<P0>(ptrustee: *mut TRUSTEE_W, pname: P0)
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("advapi32.dll" "system" fn BuildTrusteeWithNameW(ptrustee : *mut TRUSTEE_W, pname : windows_core::PCWSTR));
    BuildTrusteeWithNameW(ptrustee, pname.param().abi())
}
#[inline]
pub unsafe fn BuildTrusteeWithObjectsAndNameA<P0, P1, P2>(ptrustee: *mut TRUSTEE_A, pobjname: Option<*const OBJECTS_AND_NAME_A>, objecttype: SE_OBJECT_TYPE, objecttypename: P0, inheritedobjecttypename: P1, name: P2)
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P1: windows_core::Param<windows_core::PCSTR>,
    P2: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("advapi32.dll" "system" fn BuildTrusteeWithObjectsAndNameA(ptrustee : *mut TRUSTEE_A, pobjname : *const OBJECTS_AND_NAME_A, objecttype : SE_OBJECT_TYPE, objecttypename : windows_core::PCSTR, inheritedobjecttypename : windows_core::PCSTR, name : windows_core::PCSTR));
    BuildTrusteeWithObjectsAndNameA(ptrustee, core::mem::transmute(pobjname.unwrap_or(std::ptr::null())), objecttype, objecttypename.param().abi(), inheritedobjecttypename.param().abi(), name.param().abi())
}
#[inline]
pub unsafe fn BuildTrusteeWithObjectsAndNameW<P0, P1, P2>(ptrustee: *mut TRUSTEE_W, pobjname: Option<*const OBJECTS_AND_NAME_W>, objecttype: SE_OBJECT_TYPE, objecttypename: P0, inheritedobjecttypename: P1, name: P2)
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("advapi32.dll" "system" fn BuildTrusteeWithObjectsAndNameW(ptrustee : *mut TRUSTEE_W, pobjname : *const OBJECTS_AND_NAME_W, objecttype : SE_OBJECT_TYPE, objecttypename : windows_core::PCWSTR, inheritedobjecttypename : windows_core::PCWSTR, name : windows_core::PCWSTR));
    BuildTrusteeWithObjectsAndNameW(ptrustee, core::mem::transmute(pobjname.unwrap_or(std::ptr::null())), objecttype, objecttypename.param().abi(), inheritedobjecttypename.param().abi(), name.param().abi())
}
#[inline]
pub unsafe fn BuildTrusteeWithObjectsAndSidA<P0>(ptrustee: *mut TRUSTEE_A, pobjsid: Option<*const OBJECTS_AND_SID>, pobjectguid: Option<*const windows_core::GUID>, pinheritedobjectguid: Option<*const windows_core::GUID>, psid: P0)
where
    P0: windows_core::Param<super::PSID>,
{
    windows_targets::link!("advapi32.dll" "system" fn BuildTrusteeWithObjectsAndSidA(ptrustee : *mut TRUSTEE_A, pobjsid : *const OBJECTS_AND_SID, pobjectguid : *const windows_core::GUID, pinheritedobjectguid : *const windows_core::GUID, psid : super:: PSID));
    BuildTrusteeWithObjectsAndSidA(ptrustee, core::mem::transmute(pobjsid.unwrap_or(std::ptr::null())), core::mem::transmute(pobjectguid.unwrap_or(std::ptr::null())), core::mem::transmute(pinheritedobjectguid.unwrap_or(std::ptr::null())), psid.param().abi())
}
#[inline]
pub unsafe fn BuildTrusteeWithObjectsAndSidW<P0>(ptrustee: *mut TRUSTEE_W, pobjsid: Option<*const OBJECTS_AND_SID>, pobjectguid: Option<*const windows_core::GUID>, pinheritedobjectguid: Option<*const windows_core::GUID>, psid: P0)
where
    P0: windows_core::Param<super::PSID>,
{
    windows_targets::link!("advapi32.dll" "system" fn BuildTrusteeWithObjectsAndSidW(ptrustee : *mut TRUSTEE_W, pobjsid : *const OBJECTS_AND_SID, pobjectguid : *const windows_core::GUID, pinheritedobjectguid : *const windows_core::GUID, psid : super:: PSID));
    BuildTrusteeWithObjectsAndSidW(ptrustee, core::mem::transmute(pobjsid.unwrap_or(std::ptr::null())), core::mem::transmute(pobjectguid.unwrap_or(std::ptr::null())), core::mem::transmute(pinheritedobjectguid.unwrap_or(std::ptr::null())), psid.param().abi())
}
#[inline]
pub unsafe fn BuildTrusteeWithSidA<P0>(ptrustee: *mut TRUSTEE_A, psid: P0)
where
    P0: windows_core::Param<super::PSID>,
{
    windows_targets::link!("advapi32.dll" "system" fn BuildTrusteeWithSidA(ptrustee : *mut TRUSTEE_A, psid : super:: PSID));
    BuildTrusteeWithSidA(ptrustee, psid.param().abi())
}
#[inline]
pub unsafe fn BuildTrusteeWithSidW<P0>(ptrustee: *mut TRUSTEE_W, psid: P0)
where
    P0: windows_core::Param<super::PSID>,
{
    windows_targets::link!("advapi32.dll" "system" fn BuildTrusteeWithSidW(ptrustee : *mut TRUSTEE_W, psid : super:: PSID));
    BuildTrusteeWithSidW(ptrustee, psid.param().abi())
}
#[inline]
pub unsafe fn ConvertSecurityDescriptorToStringSecurityDescriptorA<P0>(securitydescriptor: P0, requestedstringsdrevision: u32, securityinformation: super::OBJECT_SECURITY_INFORMATION, stringsecuritydescriptor: *mut windows_core::PSTR, stringsecuritydescriptorlen: Option<*mut u32>) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::PSECURITY_DESCRIPTOR>,
{
    windows_targets::link!("advapi32.dll" "system" fn ConvertSecurityDescriptorToStringSecurityDescriptorA(securitydescriptor : super:: PSECURITY_DESCRIPTOR, requestedstringsdrevision : u32, securityinformation : super:: OBJECT_SECURITY_INFORMATION, stringsecuritydescriptor : *mut windows_core::PSTR, stringsecuritydescriptorlen : *mut u32) -> super::super::Foundation:: BOOL);
    ConvertSecurityDescriptorToStringSecurityDescriptorA(securitydescriptor.param().abi(), requestedstringsdrevision, securityinformation, stringsecuritydescriptor, core::mem::transmute(stringsecuritydescriptorlen.unwrap_or(std::ptr::null_mut()))).ok()
}
#[inline]
pub unsafe fn ConvertSecurityDescriptorToStringSecurityDescriptorW<P0>(securitydescriptor: P0, requestedstringsdrevision: u32, securityinformation: super::OBJECT_SECURITY_INFORMATION, stringsecuritydescriptor: *mut windows_core::PWSTR, stringsecuritydescriptorlen: Option<*mut u32>) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::PSECURITY_DESCRIPTOR>,
{
    windows_targets::link!("advapi32.dll" "system" fn ConvertSecurityDescriptorToStringSecurityDescriptorW(securitydescriptor : super:: PSECURITY_DESCRIPTOR, requestedstringsdrevision : u32, securityinformation : super:: OBJECT_SECURITY_INFORMATION, stringsecuritydescriptor : *mut windows_core::PWSTR, stringsecuritydescriptorlen : *mut u32) -> super::super::Foundation:: BOOL);
    ConvertSecurityDescriptorToStringSecurityDescriptorW(securitydescriptor.param().abi(), requestedstringsdrevision, securityinformation, stringsecuritydescriptor, core::mem::transmute(stringsecuritydescriptorlen.unwrap_or(std::ptr::null_mut()))).ok()
}
#[inline]
pub unsafe fn ConvertSidToStringSidA<P0>(sid: P0, stringsid: *mut windows_core::PSTR) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::PSID>,
{
    windows_targets::link!("advapi32.dll" "system" fn ConvertSidToStringSidA(sid : super:: PSID, stringsid : *mut windows_core::PSTR) -> super::super::Foundation:: BOOL);
    ConvertSidToStringSidA(sid.param().abi(), stringsid).ok()
}
#[inline]
pub unsafe fn ConvertSidToStringSidW<P0>(sid: P0, stringsid: *mut windows_core::PWSTR) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::PSID>,
{
    windows_targets::link!("advapi32.dll" "system" fn ConvertSidToStringSidW(sid : super:: PSID, stringsid : *mut windows_core::PWSTR) -> super::super::Foundation:: BOOL);
    ConvertSidToStringSidW(sid.param().abi(), stringsid).ok()
}
#[inline]
pub unsafe fn ConvertStringSecurityDescriptorToSecurityDescriptorA<P0>(stringsecuritydescriptor: P0, stringsdrevision: u32, securitydescriptor: *mut super::PSECURITY_DESCRIPTOR, securitydescriptorsize: Option<*mut u32>) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("advapi32.dll" "system" fn ConvertStringSecurityDescriptorToSecurityDescriptorA(stringsecuritydescriptor : windows_core::PCSTR, stringsdrevision : u32, securitydescriptor : *mut super:: PSECURITY_DESCRIPTOR, securitydescriptorsize : *mut u32) -> super::super::Foundation:: BOOL);
    ConvertStringSecurityDescriptorToSecurityDescriptorA(stringsecuritydescriptor.param().abi(), stringsdrevision, securitydescriptor, core::mem::transmute(securitydescriptorsize.unwrap_or(std::ptr::null_mut()))).ok()
}
#[inline]
pub unsafe fn ConvertStringSecurityDescriptorToSecurityDescriptorW<P0>(stringsecuritydescriptor: P0, stringsdrevision: u32, securitydescriptor: *mut super::PSECURITY_DESCRIPTOR, securitydescriptorsize: Option<*mut u32>) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("advapi32.dll" "system" fn ConvertStringSecurityDescriptorToSecurityDescriptorW(stringsecuritydescriptor : windows_core::PCWSTR, stringsdrevision : u32, securitydescriptor : *mut super:: PSECURITY_DESCRIPTOR, securitydescriptorsize : *mut u32) -> super::super::Foundation:: BOOL);
    ConvertStringSecurityDescriptorToSecurityDescriptorW(stringsecuritydescriptor.param().abi(), stringsdrevision, securitydescriptor, core::mem::transmute(securitydescriptorsize.unwrap_or(std::ptr::null_mut()))).ok()
}
#[inline]
pub unsafe fn ConvertStringSidToSidA<P0>(stringsid: P0, sid: *mut super::PSID) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("advapi32.dll" "system" fn ConvertStringSidToSidA(stringsid : windows_core::PCSTR, sid : *mut super:: PSID) -> super::super::Foundation:: BOOL);
    ConvertStringSidToSidA(stringsid.param().abi(), sid).ok()
}
#[inline]
pub unsafe fn ConvertStringSidToSidW<P0>(stringsid: P0, sid: *mut super::PSID) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("advapi32.dll" "system" fn ConvertStringSidToSidW(stringsid : windows_core::PCWSTR, sid : *mut super:: PSID) -> super::super::Foundation:: BOOL);
    ConvertStringSidToSidW(stringsid.param().abi(), sid).ok()
}
#[inline]
pub unsafe fn FreeInheritedFromArray(pinheritarray: &[INHERITED_FROMW], pfnarray: Option<*const FN_OBJECT_MGR_FUNCTS>) -> super::super::Foundation::WIN32_ERROR {
    windows_targets::link!("advapi32.dll" "system" fn FreeInheritedFromArray(pinheritarray : *const INHERITED_FROMW, acecnt : u16, pfnarray : *const FN_OBJECT_MGR_FUNCTS) -> super::super::Foundation:: WIN32_ERROR);
    FreeInheritedFromArray(core::mem::transmute(pinheritarray.as_ptr()), pinheritarray.len().try_into().unwrap(), core::mem::transmute(pfnarray.unwrap_or(std::ptr::null())))
}
#[inline]
pub unsafe fn GetAuditedPermissionsFromAclA(pacl: *const super::ACL, ptrustee: *const TRUSTEE_A, psuccessfulauditedrights: *mut u32, pfailedauditrights: *mut u32) -> super::super::Foundation::WIN32_ERROR {
    windows_targets::link!("advapi32.dll" "system" fn GetAuditedPermissionsFromAclA(pacl : *const super:: ACL, ptrustee : *const TRUSTEE_A, psuccessfulauditedrights : *mut u32, pfailedauditrights : *mut u32) -> super::super::Foundation:: WIN32_ERROR);
    GetAuditedPermissionsFromAclA(pacl, ptrustee, psuccessfulauditedrights, pfailedauditrights)
}
#[inline]
pub unsafe fn GetAuditedPermissionsFromAclW(pacl: *const super::ACL, ptrustee: *const TRUSTEE_W, psuccessfulauditedrights: *mut u32, pfailedauditrights: *mut u32) -> super::super::Foundation::WIN32_ERROR {
    windows_targets::link!("advapi32.dll" "system" fn GetAuditedPermissionsFromAclW(pacl : *const super:: ACL, ptrustee : *const TRUSTEE_W, psuccessfulauditedrights : *mut u32, pfailedauditrights : *mut u32) -> super::super::Foundation:: WIN32_ERROR);
    GetAuditedPermissionsFromAclW(pacl, ptrustee, psuccessfulauditedrights, pfailedauditrights)
}
#[inline]
pub unsafe fn GetEffectiveRightsFromAclA(pacl: *const super::ACL, ptrustee: *const TRUSTEE_A, paccessrights: *mut u32) -> super::super::Foundation::WIN32_ERROR {
    windows_targets::link!("advapi32.dll" "system" fn GetEffectiveRightsFromAclA(pacl : *const super:: ACL, ptrustee : *const TRUSTEE_A, paccessrights : *mut u32) -> super::super::Foundation:: WIN32_ERROR);
    GetEffectiveRightsFromAclA(pacl, ptrustee, paccessrights)
}
#[inline]
pub unsafe fn GetEffectiveRightsFromAclW(pacl: *const super::ACL, ptrustee: *const TRUSTEE_W, paccessrights: *mut u32) -> super::super::Foundation::WIN32_ERROR {
    windows_targets::link!("advapi32.dll" "system" fn GetEffectiveRightsFromAclW(pacl : *const super:: ACL, ptrustee : *const TRUSTEE_W, paccessrights : *mut u32) -> super::super::Foundation:: WIN32_ERROR);
    GetEffectiveRightsFromAclW(pacl, ptrustee, paccessrights)
}
#[inline]
pub unsafe fn GetExplicitEntriesFromAclA(pacl: *const super::ACL, pccountofexplicitentries: *mut u32, plistofexplicitentries: *mut *mut EXPLICIT_ACCESS_A) -> super::super::Foundation::WIN32_ERROR {
    windows_targets::link!("advapi32.dll" "system" fn GetExplicitEntriesFromAclA(pacl : *const super:: ACL, pccountofexplicitentries : *mut u32, plistofexplicitentries : *mut *mut EXPLICIT_ACCESS_A) -> super::super::Foundation:: WIN32_ERROR);
    GetExplicitEntriesFromAclA(pacl, pccountofexplicitentries, plistofexplicitentries)
}
#[inline]
pub unsafe fn GetExplicitEntriesFromAclW(pacl: *const super::ACL, pccountofexplicitentries: *mut u32, plistofexplicitentries: *mut *mut EXPLICIT_ACCESS_W) -> super::super::Foundation::WIN32_ERROR {
    windows_targets::link!("advapi32.dll" "system" fn GetExplicitEntriesFromAclW(pacl : *const super:: ACL, pccountofexplicitentries : *mut u32, plistofexplicitentries : *mut *mut EXPLICIT_ACCESS_W) -> super::super::Foundation:: WIN32_ERROR);
    GetExplicitEntriesFromAclW(pacl, pccountofexplicitentries, plistofexplicitentries)
}
#[inline]
pub unsafe fn GetInheritanceSourceA<P0, P1>(pobjectname: P0, objecttype: SE_OBJECT_TYPE, securityinfo: super::OBJECT_SECURITY_INFORMATION, container: P1, pobjectclassguids: Option<&[*const windows_core::GUID]>, pacl: *const super::ACL, pfnarray: Option<*const FN_OBJECT_MGR_FUNCTS>, pgenericmapping: *const super::GENERIC_MAPPING, pinheritarray: *mut INHERITED_FROMA) -> super::super::Foundation::WIN32_ERROR
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P1: windows_core::Param<super::super::Foundation::BOOL>,
{
    windows_targets::link!("advapi32.dll" "system" fn GetInheritanceSourceA(pobjectname : windows_core::PCSTR, objecttype : SE_OBJECT_TYPE, securityinfo : super:: OBJECT_SECURITY_INFORMATION, container : super::super::Foundation:: BOOL, pobjectclassguids : *const *const windows_core::GUID, guidcount : u32, pacl : *const super:: ACL, pfnarray : *const FN_OBJECT_MGR_FUNCTS, pgenericmapping : *const super:: GENERIC_MAPPING, pinheritarray : *mut INHERITED_FROMA) -> super::super::Foundation:: WIN32_ERROR);
    GetInheritanceSourceA(pobjectname.param().abi(), objecttype, securityinfo, container.param().abi(), core::mem::transmute(pobjectclassguids.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), pobjectclassguids.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), pacl, core::mem::transmute(pfnarray.unwrap_or(std::ptr::null())), pgenericmapping, pinheritarray)
}
#[inline]
pub unsafe fn GetInheritanceSourceW<P0, P1>(pobjectname: P0, objecttype: SE_OBJECT_TYPE, securityinfo: super::OBJECT_SECURITY_INFORMATION, container: P1, pobjectclassguids: Option<&[*const windows_core::GUID]>, pacl: *const super::ACL, pfnarray: Option<*const FN_OBJECT_MGR_FUNCTS>, pgenericmapping: *const super::GENERIC_MAPPING, pinheritarray: *mut INHERITED_FROMW) -> super::super::Foundation::WIN32_ERROR
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<super::super::Foundation::BOOL>,
{
    windows_targets::link!("advapi32.dll" "system" fn GetInheritanceSourceW(pobjectname : windows_core::PCWSTR, objecttype : SE_OBJECT_TYPE, securityinfo : super:: OBJECT_SECURITY_INFORMATION, container : super::super::Foundation:: BOOL, pobjectclassguids : *const *const windows_core::GUID, guidcount : u32, pacl : *const super:: ACL, pfnarray : *const FN_OBJECT_MGR_FUNCTS, pgenericmapping : *const super:: GENERIC_MAPPING, pinheritarray : *mut INHERITED_FROMW) -> super::super::Foundation:: WIN32_ERROR);
    GetInheritanceSourceW(pobjectname.param().abi(), objecttype, securityinfo, container.param().abi(), core::mem::transmute(pobjectclassguids.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), pobjectclassguids.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), pacl, core::mem::transmute(pfnarray.unwrap_or(std::ptr::null())), pgenericmapping, pinheritarray)
}
#[inline]
pub unsafe fn GetMultipleTrusteeA(ptrustee: Option<*const TRUSTEE_A>) -> *mut TRUSTEE_A {
    windows_targets::link!("advapi32.dll" "system" fn GetMultipleTrusteeA(ptrustee : *const TRUSTEE_A) -> *mut TRUSTEE_A);
    GetMultipleTrusteeA(core::mem::transmute(ptrustee.unwrap_or(std::ptr::null())))
}
#[inline]
pub unsafe fn GetMultipleTrusteeOperationA(ptrustee: Option<*const TRUSTEE_A>) -> MULTIPLE_TRUSTEE_OPERATION {
    windows_targets::link!("advapi32.dll" "system" fn GetMultipleTrusteeOperationA(ptrustee : *const TRUSTEE_A) -> MULTIPLE_TRUSTEE_OPERATION);
    GetMultipleTrusteeOperationA(core::mem::transmute(ptrustee.unwrap_or(std::ptr::null())))
}
#[inline]
pub unsafe fn GetMultipleTrusteeOperationW(ptrustee: Option<*const TRUSTEE_W>) -> MULTIPLE_TRUSTEE_OPERATION {
    windows_targets::link!("advapi32.dll" "system" fn GetMultipleTrusteeOperationW(ptrustee : *const TRUSTEE_W) -> MULTIPLE_TRUSTEE_OPERATION);
    GetMultipleTrusteeOperationW(core::mem::transmute(ptrustee.unwrap_or(std::ptr::null())))
}
#[inline]
pub unsafe fn GetMultipleTrusteeW(ptrustee: Option<*const TRUSTEE_W>) -> *mut TRUSTEE_W {
    windows_targets::link!("advapi32.dll" "system" fn GetMultipleTrusteeW(ptrustee : *const TRUSTEE_W) -> *mut TRUSTEE_W);
    GetMultipleTrusteeW(core::mem::transmute(ptrustee.unwrap_or(std::ptr::null())))
}
#[inline]
pub unsafe fn GetNamedSecurityInfoA<P0>(pobjectname: P0, objecttype: SE_OBJECT_TYPE, securityinfo: super::OBJECT_SECURITY_INFORMATION, ppsidowner: Option<*mut super::PSID>, ppsidgroup: Option<*mut super::PSID>, ppdacl: Option<*mut *mut super::ACL>, ppsacl: Option<*mut *mut super::ACL>, ppsecuritydescriptor: *mut super::PSECURITY_DESCRIPTOR) -> super::super::Foundation::WIN32_ERROR
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("advapi32.dll" "system" fn GetNamedSecurityInfoA(pobjectname : windows_core::PCSTR, objecttype : SE_OBJECT_TYPE, securityinfo : super:: OBJECT_SECURITY_INFORMATION, ppsidowner : *mut super:: PSID, ppsidgroup : *mut super:: PSID, ppdacl : *mut *mut super:: ACL, ppsacl : *mut *mut super:: ACL, ppsecuritydescriptor : *mut super:: PSECURITY_DESCRIPTOR) -> super::super::Foundation:: WIN32_ERROR);
    GetNamedSecurityInfoA(pobjectname.param().abi(), objecttype, securityinfo, core::mem::transmute(ppsidowner.unwrap_or(std::ptr::null_mut())), core::mem::transmute(ppsidgroup.unwrap_or(std::ptr::null_mut())), core::mem::transmute(ppdacl.unwrap_or(std::ptr::null_mut())), core::mem::transmute(ppsacl.unwrap_or(std::ptr::null_mut())), ppsecuritydescriptor)
}
#[inline]
pub unsafe fn GetNamedSecurityInfoW<P0>(pobjectname: P0, objecttype: SE_OBJECT_TYPE, securityinfo: super::OBJECT_SECURITY_INFORMATION, ppsidowner: Option<*mut super::PSID>, ppsidgroup: Option<*mut super::PSID>, ppdacl: Option<*mut *mut super::ACL>, ppsacl: Option<*mut *mut super::ACL>, ppsecuritydescriptor: *mut super::PSECURITY_DESCRIPTOR) -> super::super::Foundation::WIN32_ERROR
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("advapi32.dll" "system" fn GetNamedSecurityInfoW(pobjectname : windows_core::PCWSTR, objecttype : SE_OBJECT_TYPE, securityinfo : super:: OBJECT_SECURITY_INFORMATION, ppsidowner : *mut super:: PSID, ppsidgroup : *mut super:: PSID, ppdacl : *mut *mut super:: ACL, ppsacl : *mut *mut super:: ACL, ppsecuritydescriptor : *mut super:: PSECURITY_DESCRIPTOR) -> super::super::Foundation:: WIN32_ERROR);
    GetNamedSecurityInfoW(pobjectname.param().abi(), objecttype, securityinfo, core::mem::transmute(ppsidowner.unwrap_or(std::ptr::null_mut())), core::mem::transmute(ppsidgroup.unwrap_or(std::ptr::null_mut())), core::mem::transmute(ppdacl.unwrap_or(std::ptr::null_mut())), core::mem::transmute(ppsacl.unwrap_or(std::ptr::null_mut())), ppsecuritydescriptor)
}
#[inline]
pub unsafe fn GetSecurityInfo<P0>(handle: P0, objecttype: SE_OBJECT_TYPE, securityinfo: super::OBJECT_SECURITY_INFORMATION, ppsidowner: Option<*mut super::PSID>, ppsidgroup: Option<*mut super::PSID>, ppdacl: Option<*mut *mut super::ACL>, ppsacl: Option<*mut *mut super::ACL>, ppsecuritydescriptor: Option<*mut super::PSECURITY_DESCRIPTOR>) -> super::super::Foundation::WIN32_ERROR
where
    P0: windows_core::Param<super::super::Foundation::HANDLE>,
{
    windows_targets::link!("advapi32.dll" "system" fn GetSecurityInfo(handle : super::super::Foundation:: HANDLE, objecttype : SE_OBJECT_TYPE, securityinfo : super:: OBJECT_SECURITY_INFORMATION, ppsidowner : *mut super:: PSID, ppsidgroup : *mut super:: PSID, ppdacl : *mut *mut super:: ACL, ppsacl : *mut *mut super:: ACL, ppsecuritydescriptor : *mut super:: PSECURITY_DESCRIPTOR) -> super::super::Foundation:: WIN32_ERROR);
    GetSecurityInfo(handle.param().abi(), objecttype, securityinfo, core::mem::transmute(ppsidowner.unwrap_or(std::ptr::null_mut())), core::mem::transmute(ppsidgroup.unwrap_or(std::ptr::null_mut())), core::mem::transmute(ppdacl.unwrap_or(std::ptr::null_mut())), core::mem::transmute(ppsacl.unwrap_or(std::ptr::null_mut())), core::mem::transmute(ppsecuritydescriptor.unwrap_or(std::ptr::null_mut())))
}
#[inline]
pub unsafe fn GetTrusteeFormA(ptrustee: *const TRUSTEE_A) -> TRUSTEE_FORM {
    windows_targets::link!("advapi32.dll" "system" fn GetTrusteeFormA(ptrustee : *const TRUSTEE_A) -> TRUSTEE_FORM);
    GetTrusteeFormA(ptrustee)
}
#[inline]
pub unsafe fn GetTrusteeFormW(ptrustee: *const TRUSTEE_W) -> TRUSTEE_FORM {
    windows_targets::link!("advapi32.dll" "system" fn GetTrusteeFormW(ptrustee : *const TRUSTEE_W) -> TRUSTEE_FORM);
    GetTrusteeFormW(ptrustee)
}
#[inline]
pub unsafe fn GetTrusteeNameA(ptrustee: *const TRUSTEE_A) -> windows_core::PSTR {
    windows_targets::link!("advapi32.dll" "system" fn GetTrusteeNameA(ptrustee : *const TRUSTEE_A) -> windows_core::PSTR);
    GetTrusteeNameA(ptrustee)
}
#[inline]
pub unsafe fn GetTrusteeNameW(ptrustee: *const TRUSTEE_W) -> windows_core::PWSTR {
    windows_targets::link!("advapi32.dll" "system" fn GetTrusteeNameW(ptrustee : *const TRUSTEE_W) -> windows_core::PWSTR);
    GetTrusteeNameW(ptrustee)
}
#[inline]
pub unsafe fn GetTrusteeTypeA(ptrustee: Option<*const TRUSTEE_A>) -> TRUSTEE_TYPE {
    windows_targets::link!("advapi32.dll" "system" fn GetTrusteeTypeA(ptrustee : *const TRUSTEE_A) -> TRUSTEE_TYPE);
    GetTrusteeTypeA(core::mem::transmute(ptrustee.unwrap_or(std::ptr::null())))
}
#[inline]
pub unsafe fn GetTrusteeTypeW(ptrustee: Option<*const TRUSTEE_W>) -> TRUSTEE_TYPE {
    windows_targets::link!("advapi32.dll" "system" fn GetTrusteeTypeW(ptrustee : *const TRUSTEE_W) -> TRUSTEE_TYPE);
    GetTrusteeTypeW(core::mem::transmute(ptrustee.unwrap_or(std::ptr::null())))
}
#[inline]
pub unsafe fn LookupSecurityDescriptorPartsA<P0>(ppowner: Option<*mut *mut TRUSTEE_A>, ppgroup: Option<*mut *mut TRUSTEE_A>, pccountofaccessentries: Option<*mut u32>, pplistofaccessentries: *mut *mut EXPLICIT_ACCESS_A, pccountofauditentries: Option<*mut u32>, pplistofauditentries: *mut *mut EXPLICIT_ACCESS_A, psd: P0) -> super::super::Foundation::WIN32_ERROR
where
    P0: windows_core::Param<super::PSECURITY_DESCRIPTOR>,
{
    windows_targets::link!("advapi32.dll" "system" fn LookupSecurityDescriptorPartsA(ppowner : *mut *mut TRUSTEE_A, ppgroup : *mut *mut TRUSTEE_A, pccountofaccessentries : *mut u32, pplistofaccessentries : *mut *mut EXPLICIT_ACCESS_A, pccountofauditentries : *mut u32, pplistofauditentries : *mut *mut EXPLICIT_ACCESS_A, psd : super:: PSECURITY_DESCRIPTOR) -> super::super::Foundation:: WIN32_ERROR);
    LookupSecurityDescriptorPartsA(core::mem::transmute(ppowner.unwrap_or(std::ptr::null_mut())), core::mem::transmute(ppgroup.unwrap_or(std::ptr::null_mut())), core::mem::transmute(pccountofaccessentries.unwrap_or(std::ptr::null_mut())), pplistofaccessentries, core::mem::transmute(pccountofauditentries.unwrap_or(std::ptr::null_mut())), pplistofauditentries, psd.param().abi())
}
#[inline]
pub unsafe fn LookupSecurityDescriptorPartsW<P0>(ppowner: Option<*mut *mut TRUSTEE_W>, ppgroup: Option<*mut *mut TRUSTEE_W>, pccountofaccessentries: Option<*mut u32>, pplistofaccessentries: *mut *mut EXPLICIT_ACCESS_W, pccountofauditentries: Option<*mut u32>, pplistofauditentries: *mut *mut EXPLICIT_ACCESS_W, psd: P0) -> super::super::Foundation::WIN32_ERROR
where
    P0: windows_core::Param<super::PSECURITY_DESCRIPTOR>,
{
    windows_targets::link!("advapi32.dll" "system" fn LookupSecurityDescriptorPartsW(ppowner : *mut *mut TRUSTEE_W, ppgroup : *mut *mut TRUSTEE_W, pccountofaccessentries : *mut u32, pplistofaccessentries : *mut *mut EXPLICIT_ACCESS_W, pccountofauditentries : *mut u32, pplistofauditentries : *mut *mut EXPLICIT_ACCESS_W, psd : super:: PSECURITY_DESCRIPTOR) -> super::super::Foundation:: WIN32_ERROR);
    LookupSecurityDescriptorPartsW(core::mem::transmute(ppowner.unwrap_or(std::ptr::null_mut())), core::mem::transmute(ppgroup.unwrap_or(std::ptr::null_mut())), core::mem::transmute(pccountofaccessentries.unwrap_or(std::ptr::null_mut())), pplistofaccessentries, core::mem::transmute(pccountofauditentries.unwrap_or(std::ptr::null_mut())), pplistofauditentries, psd.param().abi())
}
#[inline]
pub unsafe fn SetEntriesInAclA(plistofexplicitentries: Option<&[EXPLICIT_ACCESS_A]>, oldacl: Option<*const super::ACL>, newacl: *mut *mut super::ACL) -> super::super::Foundation::WIN32_ERROR {
    windows_targets::link!("advapi32.dll" "system" fn SetEntriesInAclA(ccountofexplicitentries : u32, plistofexplicitentries : *const EXPLICIT_ACCESS_A, oldacl : *const super:: ACL, newacl : *mut *mut super:: ACL) -> super::super::Foundation:: WIN32_ERROR);
    SetEntriesInAclA(plistofexplicitentries.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), core::mem::transmute(plistofexplicitentries.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), core::mem::transmute(oldacl.unwrap_or(std::ptr::null())), newacl)
}
#[inline]
pub unsafe fn SetEntriesInAclW(plistofexplicitentries: Option<&[EXPLICIT_ACCESS_W]>, oldacl: Option<*const super::ACL>, newacl: *mut *mut super::ACL) -> super::super::Foundation::WIN32_ERROR {
    windows_targets::link!("advapi32.dll" "system" fn SetEntriesInAclW(ccountofexplicitentries : u32, plistofexplicitentries : *const EXPLICIT_ACCESS_W, oldacl : *const super:: ACL, newacl : *mut *mut super:: ACL) -> super::super::Foundation:: WIN32_ERROR);
    SetEntriesInAclW(plistofexplicitentries.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), core::mem::transmute(plistofexplicitentries.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), core::mem::transmute(oldacl.unwrap_or(std::ptr::null())), newacl)
}
#[inline]
pub unsafe fn SetNamedSecurityInfoA<P0, P1, P2>(pobjectname: P0, objecttype: SE_OBJECT_TYPE, securityinfo: super::OBJECT_SECURITY_INFORMATION, psidowner: P1, psidgroup: P2, pdacl: Option<*const super::ACL>, psacl: Option<*const super::ACL>) -> super::super::Foundation::WIN32_ERROR
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P1: windows_core::Param<super::PSID>,
    P2: windows_core::Param<super::PSID>,
{
    windows_targets::link!("advapi32.dll" "system" fn SetNamedSecurityInfoA(pobjectname : windows_core::PCSTR, objecttype : SE_OBJECT_TYPE, securityinfo : super:: OBJECT_SECURITY_INFORMATION, psidowner : super:: PSID, psidgroup : super:: PSID, pdacl : *const super:: ACL, psacl : *const super:: ACL) -> super::super::Foundation:: WIN32_ERROR);
    SetNamedSecurityInfoA(pobjectname.param().abi(), objecttype, securityinfo, psidowner.param().abi(), psidgroup.param().abi(), core::mem::transmute(pdacl.unwrap_or(std::ptr::null())), core::mem::transmute(psacl.unwrap_or(std::ptr::null())))
}
#[inline]
pub unsafe fn SetNamedSecurityInfoW<P0, P1, P2>(pobjectname: P0, objecttype: SE_OBJECT_TYPE, securityinfo: super::OBJECT_SECURITY_INFORMATION, psidowner: P1, psidgroup: P2, pdacl: Option<*const super::ACL>, psacl: Option<*const super::ACL>) -> super::super::Foundation::WIN32_ERROR
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<super::PSID>,
    P2: windows_core::Param<super::PSID>,
{
    windows_targets::link!("advapi32.dll" "system" fn SetNamedSecurityInfoW(pobjectname : windows_core::PCWSTR, objecttype : SE_OBJECT_TYPE, securityinfo : super:: OBJECT_SECURITY_INFORMATION, psidowner : super:: PSID, psidgroup : super:: PSID, pdacl : *const super:: ACL, psacl : *const super:: ACL) -> super::super::Foundation:: WIN32_ERROR);
    SetNamedSecurityInfoW(pobjectname.param().abi(), objecttype, securityinfo, psidowner.param().abi(), psidgroup.param().abi(), core::mem::transmute(pdacl.unwrap_or(std::ptr::null())), core::mem::transmute(psacl.unwrap_or(std::ptr::null())))
}
#[inline]
pub unsafe fn SetSecurityInfo<P0, P1, P2>(handle: P0, objecttype: SE_OBJECT_TYPE, securityinfo: super::OBJECT_SECURITY_INFORMATION, psidowner: P1, psidgroup: P2, pdacl: Option<*const super::ACL>, psacl: Option<*const super::ACL>) -> super::super::Foundation::WIN32_ERROR
where
    P0: windows_core::Param<super::super::Foundation::HANDLE>,
    P1: windows_core::Param<super::PSID>,
    P2: windows_core::Param<super::PSID>,
{
    windows_targets::link!("advapi32.dll" "system" fn SetSecurityInfo(handle : super::super::Foundation:: HANDLE, objecttype : SE_OBJECT_TYPE, securityinfo : super:: OBJECT_SECURITY_INFORMATION, psidowner : super:: PSID, psidgroup : super:: PSID, pdacl : *const super:: ACL, psacl : *const super:: ACL) -> super::super::Foundation:: WIN32_ERROR);
    SetSecurityInfo(handle.param().abi(), objecttype, securityinfo, psidowner.param().abi(), psidgroup.param().abi(), core::mem::transmute(pdacl.unwrap_or(std::ptr::null())), core::mem::transmute(psacl.unwrap_or(std::ptr::null())))
}
#[inline]
pub unsafe fn TreeResetNamedSecurityInfoA<P0, P1, P2, P3>(pobjectname: P0, objecttype: SE_OBJECT_TYPE, securityinfo: super::OBJECT_SECURITY_INFORMATION, powner: P1, pgroup: P2, pdacl: Option<*const super::ACL>, psacl: Option<*const super::ACL>, keepexplicit: P3, fnprogress: FN_PROGRESS, progressinvokesetting: PROG_INVOKE_SETTING, args: Option<*const core::ffi::c_void>) -> super::super::Foundation::WIN32_ERROR
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P1: windows_core::Param<super::PSID>,
    P2: windows_core::Param<super::PSID>,
    P3: windows_core::Param<super::super::Foundation::BOOL>,
{
    windows_targets::link!("advapi32.dll" "system" fn TreeResetNamedSecurityInfoA(pobjectname : windows_core::PCSTR, objecttype : SE_OBJECT_TYPE, securityinfo : super:: OBJECT_SECURITY_INFORMATION, powner : super:: PSID, pgroup : super:: PSID, pdacl : *const super:: ACL, psacl : *const super:: ACL, keepexplicit : super::super::Foundation:: BOOL, fnprogress : FN_PROGRESS, progressinvokesetting : PROG_INVOKE_SETTING, args : *const core::ffi::c_void) -> super::super::Foundation:: WIN32_ERROR);
    TreeResetNamedSecurityInfoA(pobjectname.param().abi(), objecttype, securityinfo, powner.param().abi(), pgroup.param().abi(), core::mem::transmute(pdacl.unwrap_or(std::ptr::null())), core::mem::transmute(psacl.unwrap_or(std::ptr::null())), keepexplicit.param().abi(), fnprogress, progressinvokesetting, core::mem::transmute(args.unwrap_or(std::ptr::null())))
}
#[inline]
pub unsafe fn TreeResetNamedSecurityInfoW<P0, P1, P2, P3>(pobjectname: P0, objecttype: SE_OBJECT_TYPE, securityinfo: super::OBJECT_SECURITY_INFORMATION, powner: P1, pgroup: P2, pdacl: Option<*const super::ACL>, psacl: Option<*const super::ACL>, keepexplicit: P3, fnprogress: FN_PROGRESS, progressinvokesetting: PROG_INVOKE_SETTING, args: Option<*const core::ffi::c_void>) -> super::super::Foundation::WIN32_ERROR
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<super::PSID>,
    P2: windows_core::Param<super::PSID>,
    P3: windows_core::Param<super::super::Foundation::BOOL>,
{
    windows_targets::link!("advapi32.dll" "system" fn TreeResetNamedSecurityInfoW(pobjectname : windows_core::PCWSTR, objecttype : SE_OBJECT_TYPE, securityinfo : super:: OBJECT_SECURITY_INFORMATION, powner : super:: PSID, pgroup : super:: PSID, pdacl : *const super:: ACL, psacl : *const super:: ACL, keepexplicit : super::super::Foundation:: BOOL, fnprogress : FN_PROGRESS, progressinvokesetting : PROG_INVOKE_SETTING, args : *const core::ffi::c_void) -> super::super::Foundation:: WIN32_ERROR);
    TreeResetNamedSecurityInfoW(pobjectname.param().abi(), objecttype, securityinfo, powner.param().abi(), pgroup.param().abi(), core::mem::transmute(pdacl.unwrap_or(std::ptr::null())), core::mem::transmute(psacl.unwrap_or(std::ptr::null())), keepexplicit.param().abi(), fnprogress, progressinvokesetting, core::mem::transmute(args.unwrap_or(std::ptr::null())))
}
#[inline]
pub unsafe fn TreeSetNamedSecurityInfoA<P0, P1, P2>(pobjectname: P0, objecttype: SE_OBJECT_TYPE, securityinfo: super::OBJECT_SECURITY_INFORMATION, powner: P1, pgroup: P2, pdacl: Option<*const super::ACL>, psacl: Option<*const super::ACL>, dwaction: TREE_SEC_INFO, fnprogress: FN_PROGRESS, progressinvokesetting: PROG_INVOKE_SETTING, args: Option<*const core::ffi::c_void>) -> super::super::Foundation::WIN32_ERROR
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P1: windows_core::Param<super::PSID>,
    P2: windows_core::Param<super::PSID>,
{
    windows_targets::link!("advapi32.dll" "system" fn TreeSetNamedSecurityInfoA(pobjectname : windows_core::PCSTR, objecttype : SE_OBJECT_TYPE, securityinfo : super:: OBJECT_SECURITY_INFORMATION, powner : super:: PSID, pgroup : super:: PSID, pdacl : *const super:: ACL, psacl : *const super:: ACL, dwaction : TREE_SEC_INFO, fnprogress : FN_PROGRESS, progressinvokesetting : PROG_INVOKE_SETTING, args : *const core::ffi::c_void) -> super::super::Foundation:: WIN32_ERROR);
    TreeSetNamedSecurityInfoA(pobjectname.param().abi(), objecttype, securityinfo, powner.param().abi(), pgroup.param().abi(), core::mem::transmute(pdacl.unwrap_or(std::ptr::null())), core::mem::transmute(psacl.unwrap_or(std::ptr::null())), dwaction, fnprogress, progressinvokesetting, core::mem::transmute(args.unwrap_or(std::ptr::null())))
}
#[inline]
pub unsafe fn TreeSetNamedSecurityInfoW<P0, P1, P2>(pobjectname: P0, objecttype: SE_OBJECT_TYPE, securityinfo: super::OBJECT_SECURITY_INFORMATION, powner: P1, pgroup: P2, pdacl: Option<*const super::ACL>, psacl: Option<*const super::ACL>, dwaction: TREE_SEC_INFO, fnprogress: FN_PROGRESS, progressinvokesetting: PROG_INVOKE_SETTING, args: Option<*const core::ffi::c_void>) -> super::super::Foundation::WIN32_ERROR
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<super::PSID>,
    P2: windows_core::Param<super::PSID>,
{
    windows_targets::link!("advapi32.dll" "system" fn TreeSetNamedSecurityInfoW(pobjectname : windows_core::PCWSTR, objecttype : SE_OBJECT_TYPE, securityinfo : super:: OBJECT_SECURITY_INFORMATION, powner : super:: PSID, pgroup : super:: PSID, pdacl : *const super:: ACL, psacl : *const super:: ACL, dwaction : TREE_SEC_INFO, fnprogress : FN_PROGRESS, progressinvokesetting : PROG_INVOKE_SETTING, args : *const core::ffi::c_void) -> super::super::Foundation:: WIN32_ERROR);
    TreeSetNamedSecurityInfoW(pobjectname.param().abi(), objecttype, securityinfo, powner.param().abi(), pgroup.param().abi(), core::mem::transmute(pdacl.unwrap_or(std::ptr::null())), core::mem::transmute(psacl.unwrap_or(std::ptr::null())), dwaction, fnprogress, progressinvokesetting, core::mem::transmute(args.unwrap_or(std::ptr::null())))
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IAzApplication, IAzApplication_Vtbl, 0x987bc7c7_b813_4d27_bede_6ba5ae867e95);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IAzApplication {
    type Target = super::super::System::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IAzApplication, windows_core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl IAzApplication {
    pub unsafe fn Name(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Name)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetName<P0>(&self, bstrname: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).SetName)(windows_core::Interface::as_raw(self), bstrname.param().abi()).ok()
    }
    pub unsafe fn Description(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Description)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetDescription<P0>(&self, bstrdescription: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).SetDescription)(windows_core::Interface::as_raw(self), bstrdescription.param().abi()).ok()
    }
    pub unsafe fn ApplicationData(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).ApplicationData)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetApplicationData<P0>(&self, bstrapplicationdata: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).SetApplicationData)(windows_core::Interface::as_raw(self), bstrapplicationdata.param().abi()).ok()
    }
    pub unsafe fn AuthzInterfaceClsid(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).AuthzInterfaceClsid)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetAuthzInterfaceClsid<P0>(&self, bstrprop: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).SetAuthzInterfaceClsid)(windows_core::Interface::as_raw(self), bstrprop.param().abi()).ok()
    }
    pub unsafe fn Version(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Version)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetVersion<P0>(&self, bstrprop: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).SetVersion)(windows_core::Interface::as_raw(self), bstrprop.param().abi()).ok()
    }
    pub unsafe fn GenerateAudits(&self) -> windows_core::Result<super::super::Foundation::BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GenerateAudits)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetGenerateAudits<P0>(&self, bprop: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::BOOL>,
    {
        (windows_core::Interface::vtable(self).SetGenerateAudits)(windows_core::Interface::as_raw(self), bprop.param().abi()).ok()
    }
    pub unsafe fn ApplyStoreSacl(&self) -> windows_core::Result<super::super::Foundation::BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).ApplyStoreSacl)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetApplyStoreSacl<P0>(&self, bprop: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::BOOL>,
    {
        (windows_core::Interface::vtable(self).SetApplyStoreSacl)(windows_core::Interface::as_raw(self), bprop.param().abi()).ok()
    }
    pub unsafe fn Writable(&self) -> windows_core::Result<super::super::Foundation::BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Writable)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn GetProperty<P0>(&self, lpropid: i32, varreserved: P0) -> windows_core::Result<windows_core::VARIANT>
    where
        P0: windows_core::Param<windows_core::VARIANT>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetProperty)(windows_core::Interface::as_raw(self), lpropid, varreserved.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetProperty<P0, P1>(&self, lpropid: i32, varprop: P0, varreserved: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::VARIANT>,
        P1: windows_core::Param<windows_core::VARIANT>,
    {
        (windows_core::Interface::vtable(self).SetProperty)(windows_core::Interface::as_raw(self), lpropid, varprop.param().abi(), varreserved.param().abi()).ok()
    }
    pub unsafe fn PolicyAdministrators(&self) -> windows_core::Result<windows_core::VARIANT> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).PolicyAdministrators)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn PolicyReaders(&self) -> windows_core::Result<windows_core::VARIANT> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).PolicyReaders)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn AddPolicyAdministrator<P0, P1>(&self, bstradmin: P0, varreserved: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
        P1: windows_core::Param<windows_core::VARIANT>,
    {
        (windows_core::Interface::vtable(self).AddPolicyAdministrator)(windows_core::Interface::as_raw(self), bstradmin.param().abi(), varreserved.param().abi()).ok()
    }
    pub unsafe fn DeletePolicyAdministrator<P0, P1>(&self, bstradmin: P0, varreserved: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
        P1: windows_core::Param<windows_core::VARIANT>,
    {
        (windows_core::Interface::vtable(self).DeletePolicyAdministrator)(windows_core::Interface::as_raw(self), bstradmin.param().abi(), varreserved.param().abi()).ok()
    }
    pub unsafe fn AddPolicyReader<P0, P1>(&self, bstrreader: P0, varreserved: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
        P1: windows_core::Param<windows_core::VARIANT>,
    {
        (windows_core::Interface::vtable(self).AddPolicyReader)(windows_core::Interface::as_raw(self), bstrreader.param().abi(), varreserved.param().abi()).ok()
    }
    pub unsafe fn DeletePolicyReader<P0, P1>(&self, bstrreader: P0, varreserved: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
        P1: windows_core::Param<windows_core::VARIANT>,
    {
        (windows_core::Interface::vtable(self).DeletePolicyReader)(windows_core::Interface::as_raw(self), bstrreader.param().abi(), varreserved.param().abi()).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Scopes(&self) -> windows_core::Result<IAzScopes> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Scopes)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn OpenScope<P0, P1>(&self, bstrscopename: P0, varreserved: P1) -> windows_core::Result<IAzScope>
    where
        P0: windows_core::Param<windows_core::BSTR>,
        P1: windows_core::Param<windows_core::VARIANT>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).OpenScope)(windows_core::Interface::as_raw(self), bstrscopename.param().abi(), varreserved.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CreateScope<P0, P1>(&self, bstrscopename: P0, varreserved: P1) -> windows_core::Result<IAzScope>
    where
        P0: windows_core::Param<windows_core::BSTR>,
        P1: windows_core::Param<windows_core::VARIANT>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CreateScope)(windows_core::Interface::as_raw(self), bstrscopename.param().abi(), varreserved.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn DeleteScope<P0, P1>(&self, bstrscopename: P0, varreserved: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
        P1: windows_core::Param<windows_core::VARIANT>,
    {
        (windows_core::Interface::vtable(self).DeleteScope)(windows_core::Interface::as_raw(self), bstrscopename.param().abi(), varreserved.param().abi()).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Operations(&self) -> windows_core::Result<IAzOperations> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Operations)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn OpenOperation<P0, P1>(&self, bstroperationname: P0, varreserved: P1) -> windows_core::Result<IAzOperation>
    where
        P0: windows_core::Param<windows_core::BSTR>,
        P1: windows_core::Param<windows_core::VARIANT>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).OpenOperation)(windows_core::Interface::as_raw(self), bstroperationname.param().abi(), varreserved.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CreateOperation<P0, P1>(&self, bstroperationname: P0, varreserved: P1) -> windows_core::Result<IAzOperation>
    where
        P0: windows_core::Param<windows_core::BSTR>,
        P1: windows_core::Param<windows_core::VARIANT>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CreateOperation)(windows_core::Interface::as_raw(self), bstroperationname.param().abi(), varreserved.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn DeleteOperation<P0, P1>(&self, bstroperationname: P0, varreserved: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
        P1: windows_core::Param<windows_core::VARIANT>,
    {
        (windows_core::Interface::vtable(self).DeleteOperation)(windows_core::Interface::as_raw(self), bstroperationname.param().abi(), varreserved.param().abi()).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Tasks(&self) -> windows_core::Result<IAzTasks> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Tasks)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn OpenTask<P0, P1>(&self, bstrtaskname: P0, varreserved: P1) -> windows_core::Result<IAzTask>
    where
        P0: windows_core::Param<windows_core::BSTR>,
        P1: windows_core::Param<windows_core::VARIANT>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).OpenTask)(windows_core::Interface::as_raw(self), bstrtaskname.param().abi(), varreserved.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CreateTask<P0, P1>(&self, bstrtaskname: P0, varreserved: P1) -> windows_core::Result<IAzTask>
    where
        P0: windows_core::Param<windows_core::BSTR>,
        P1: windows_core::Param<windows_core::VARIANT>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CreateTask)(windows_core::Interface::as_raw(self), bstrtaskname.param().abi(), varreserved.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn DeleteTask<P0, P1>(&self, bstrtaskname: P0, varreserved: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
        P1: windows_core::Param<windows_core::VARIANT>,
    {
        (windows_core::Interface::vtable(self).DeleteTask)(windows_core::Interface::as_raw(self), bstrtaskname.param().abi(), varreserved.param().abi()).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn ApplicationGroups(&self) -> windows_core::Result<IAzApplicationGroups> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).ApplicationGroups)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn OpenApplicationGroup<P0, P1>(&self, bstrgroupname: P0, varreserved: P1) -> windows_core::Result<IAzApplicationGroup>
    where
        P0: windows_core::Param<windows_core::BSTR>,
        P1: windows_core::Param<windows_core::VARIANT>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).OpenApplicationGroup)(windows_core::Interface::as_raw(self), bstrgroupname.param().abi(), varreserved.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CreateApplicationGroup<P0, P1>(&self, bstrgroupname: P0, varreserved: P1) -> windows_core::Result<IAzApplicationGroup>
    where
        P0: windows_core::Param<windows_core::BSTR>,
        P1: windows_core::Param<windows_core::VARIANT>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CreateApplicationGroup)(windows_core::Interface::as_raw(self), bstrgroupname.param().abi(), varreserved.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn DeleteApplicationGroup<P0, P1>(&self, bstrgroupname: P0, varreserved: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
        P1: windows_core::Param<windows_core::VARIANT>,
    {
        (windows_core::Interface::vtable(self).DeleteApplicationGroup)(windows_core::Interface::as_raw(self), bstrgroupname.param().abi(), varreserved.param().abi()).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Roles(&self) -> windows_core::Result<IAzRoles> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Roles)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn OpenRole<P0, P1>(&self, bstrrolename: P0, varreserved: P1) -> windows_core::Result<IAzRole>
    where
        P0: windows_core::Param<windows_core::BSTR>,
        P1: windows_core::Param<windows_core::VARIANT>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).OpenRole)(windows_core::Interface::as_raw(self), bstrrolename.param().abi(), varreserved.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CreateRole<P0, P1>(&self, bstrrolename: P0, varreserved: P1) -> windows_core::Result<IAzRole>
    where
        P0: windows_core::Param<windows_core::BSTR>,
        P1: windows_core::Param<windows_core::VARIANT>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CreateRole)(windows_core::Interface::as_raw(self), bstrrolename.param().abi(), varreserved.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn DeleteRole<P0, P1>(&self, bstrrolename: P0, varreserved: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
        P1: windows_core::Param<windows_core::VARIANT>,
    {
        (windows_core::Interface::vtable(self).DeleteRole)(windows_core::Interface::as_raw(self), bstrrolename.param().abi(), varreserved.param().abi()).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn InitializeClientContextFromToken<P0>(&self, ulltokenhandle: u64, varreserved: P0) -> windows_core::Result<IAzClientContext>
    where
        P0: windows_core::Param<windows_core::VARIANT>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).InitializeClientContextFromToken)(windows_core::Interface::as_raw(self), ulltokenhandle, varreserved.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn AddPropertyItem<P0, P1>(&self, lpropid: i32, varprop: P0, varreserved: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::VARIANT>,
        P1: windows_core::Param<windows_core::VARIANT>,
    {
        (windows_core::Interface::vtable(self).AddPropertyItem)(windows_core::Interface::as_raw(self), lpropid, varprop.param().abi(), varreserved.param().abi()).ok()
    }
    pub unsafe fn DeletePropertyItem<P0, P1>(&self, lpropid: i32, varprop: P0, varreserved: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::VARIANT>,
        P1: windows_core::Param<windows_core::VARIANT>,
    {
        (windows_core::Interface::vtable(self).DeletePropertyItem)(windows_core::Interface::as_raw(self), lpropid, varprop.param().abi(), varreserved.param().abi()).ok()
    }
    pub unsafe fn Submit<P0>(&self, lflags: i32, varreserved: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::VARIANT>,
    {
        (windows_core::Interface::vtable(self).Submit)(windows_core::Interface::as_raw(self), lflags, varreserved.param().abi()).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn InitializeClientContextFromName<P0, P1, P2>(&self, clientname: P0, domainname: P1, varreserved: P2) -> windows_core::Result<IAzClientContext>
    where
        P0: windows_core::Param<windows_core::BSTR>,
        P1: windows_core::Param<windows_core::BSTR>,
        P2: windows_core::Param<windows_core::VARIANT>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).InitializeClientContextFromName)(windows_core::Interface::as_raw(self), clientname.param().abi(), domainname.param().abi(), varreserved.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn DelegatedPolicyUsers(&self) -> windows_core::Result<windows_core::VARIANT> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).DelegatedPolicyUsers)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn AddDelegatedPolicyUser<P0, P1>(&self, bstrdelegatedpolicyuser: P0, varreserved: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
        P1: windows_core::Param<windows_core::VARIANT>,
    {
        (windows_core::Interface::vtable(self).AddDelegatedPolicyUser)(windows_core::Interface::as_raw(self), bstrdelegatedpolicyuser.param().abi(), varreserved.param().abi()).ok()
    }
    pub unsafe fn DeleteDelegatedPolicyUser<P0, P1>(&self, bstrdelegatedpolicyuser: P0, varreserved: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
        P1: windows_core::Param<windows_core::VARIANT>,
    {
        (windows_core::Interface::vtable(self).DeleteDelegatedPolicyUser)(windows_core::Interface::as_raw(self), bstrdelegatedpolicyuser.param().abi(), varreserved.param().abi()).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn InitializeClientContextFromStringSid<P0, P1>(&self, sidstring: P0, loptions: i32, varreserved: P1) -> windows_core::Result<IAzClientContext>
    where
        P0: windows_core::Param<windows_core::BSTR>,
        P1: windows_core::Param<windows_core::VARIANT>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).InitializeClientContextFromStringSid)(windows_core::Interface::as_raw(self), sidstring.param().abi(), loptions, varreserved.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn PolicyAdministratorsName(&self) -> windows_core::Result<windows_core::VARIANT> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).PolicyAdministratorsName)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn PolicyReadersName(&self) -> windows_core::Result<windows_core::VARIANT> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).PolicyReadersName)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn AddPolicyAdministratorName<P0, P1>(&self, bstradmin: P0, varreserved: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
        P1: windows_core::Param<windows_core::VARIANT>,
    {
        (windows_core::Interface::vtable(self).AddPolicyAdministratorName)(windows_core::Interface::as_raw(self), bstradmin.param().abi(), varreserved.param().abi()).ok()
    }
    pub unsafe fn DeletePolicyAdministratorName<P0, P1>(&self, bstradmin: P0, varreserved: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
        P1: windows_core::Param<windows_core::VARIANT>,
    {
        (windows_core::Interface::vtable(self).DeletePolicyAdministratorName)(windows_core::Interface::as_raw(self), bstradmin.param().abi(), varreserved.param().abi()).ok()
    }
    pub unsafe fn AddPolicyReaderName<P0, P1>(&self, bstrreader: P0, varreserved: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
        P1: windows_core::Param<windows_core::VARIANT>,
    {
        (windows_core::Interface::vtable(self).AddPolicyReaderName)(windows_core::Interface::as_raw(self), bstrreader.param().abi(), varreserved.param().abi()).ok()
    }
    pub unsafe fn DeletePolicyReaderName<P0, P1>(&self, bstrreader: P0, varreserved: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
        P1: windows_core::Param<windows_core::VARIANT>,
    {
        (windows_core::Interface::vtable(self).DeletePolicyReaderName)(windows_core::Interface::as_raw(self), bstrreader.param().abi(), varreserved.param().abi()).ok()
    }
    pub unsafe fn DelegatedPolicyUsersName(&self) -> windows_core::Result<windows_core::VARIANT> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).DelegatedPolicyUsersName)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn AddDelegatedPolicyUserName<P0, P1>(&self, bstrdelegatedpolicyuser: P0, varreserved: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
        P1: windows_core::Param<windows_core::VARIANT>,
    {
        (windows_core::Interface::vtable(self).AddDelegatedPolicyUserName)(windows_core::Interface::as_raw(self), bstrdelegatedpolicyuser.param().abi(), varreserved.param().abi()).ok()
    }
    pub unsafe fn DeleteDelegatedPolicyUserName<P0, P1>(&self, bstrdelegatedpolicyuser: P0, varreserved: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
        P1: windows_core::Param<windows_core::VARIANT>,
    {
        (windows_core::Interface::vtable(self).DeleteDelegatedPolicyUserName)(windows_core::Interface::as_raw(self), bstrdelegatedpolicyuser.param().abi(), varreserved.param().abi()).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IAzApplication_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub Name: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub SetName: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub Description: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub SetDescription: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub ApplicationData: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub SetApplicationData: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub AuthzInterfaceClsid: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub SetAuthzInterfaceClsid: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub Version: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub SetVersion: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub GenerateAudits: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::BOOL) -> windows_core::HRESULT,
    pub SetGenerateAudits: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::BOOL) -> windows_core::HRESULT,
    pub ApplyStoreSacl: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::BOOL) -> windows_core::HRESULT,
    pub SetApplyStoreSacl: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::BOOL) -> windows_core::HRESULT,
    pub Writable: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::BOOL) -> windows_core::HRESULT,
    pub GetProperty: unsafe extern "system" fn(*mut core::ffi::c_void, i32, core::mem::MaybeUninit<windows_core::VARIANT>, *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
    pub SetProperty: unsafe extern "system" fn(*mut core::ffi::c_void, i32, core::mem::MaybeUninit<windows_core::VARIANT>, core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
    pub PolicyAdministrators: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
    pub PolicyReaders: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
    pub AddPolicyAdministrator: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
    pub DeletePolicyAdministrator: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
    pub AddPolicyReader: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
    pub DeletePolicyReader: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub Scopes: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Scopes: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub OpenScope: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, core::mem::MaybeUninit<windows_core::VARIANT>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    OpenScope: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub CreateScope: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, core::mem::MaybeUninit<windows_core::VARIANT>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    CreateScope: usize,
    pub DeleteScope: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub Operations: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Operations: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub OpenOperation: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, core::mem::MaybeUninit<windows_core::VARIANT>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    OpenOperation: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub CreateOperation: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, core::mem::MaybeUninit<windows_core::VARIANT>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    CreateOperation: usize,
    pub DeleteOperation: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub Tasks: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Tasks: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub OpenTask: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, core::mem::MaybeUninit<windows_core::VARIANT>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    OpenTask: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub CreateTask: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, core::mem::MaybeUninit<windows_core::VARIANT>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    CreateTask: usize,
    pub DeleteTask: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub ApplicationGroups: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    ApplicationGroups: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub OpenApplicationGroup: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, core::mem::MaybeUninit<windows_core::VARIANT>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    OpenApplicationGroup: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub CreateApplicationGroup: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, core::mem::MaybeUninit<windows_core::VARIANT>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    CreateApplicationGroup: usize,
    pub DeleteApplicationGroup: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub Roles: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Roles: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub OpenRole: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, core::mem::MaybeUninit<windows_core::VARIANT>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    OpenRole: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub CreateRole: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, core::mem::MaybeUninit<windows_core::VARIANT>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    CreateRole: usize,
    pub DeleteRole: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub InitializeClientContextFromToken: unsafe extern "system" fn(*mut core::ffi::c_void, u64, core::mem::MaybeUninit<windows_core::VARIANT>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    InitializeClientContextFromToken: usize,
    pub AddPropertyItem: unsafe extern "system" fn(*mut core::ffi::c_void, i32, core::mem::MaybeUninit<windows_core::VARIANT>, core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
    pub DeletePropertyItem: unsafe extern "system" fn(*mut core::ffi::c_void, i32, core::mem::MaybeUninit<windows_core::VARIANT>, core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
    pub Submit: unsafe extern "system" fn(*mut core::ffi::c_void, i32, core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub InitializeClientContextFromName: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, core::mem::MaybeUninit<windows_core::BSTR>, core::mem::MaybeUninit<windows_core::VARIANT>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    InitializeClientContextFromName: usize,
    pub DelegatedPolicyUsers: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
    pub AddDelegatedPolicyUser: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
    pub DeleteDelegatedPolicyUser: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub InitializeClientContextFromStringSid: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, i32, core::mem::MaybeUninit<windows_core::VARIANT>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    InitializeClientContextFromStringSid: usize,
    pub PolicyAdministratorsName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
    pub PolicyReadersName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
    pub AddPolicyAdministratorName: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
    pub DeletePolicyAdministratorName: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
    pub AddPolicyReaderName: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
    pub DeletePolicyReaderName: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
    pub DelegatedPolicyUsersName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
    pub AddDelegatedPolicyUserName: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
    pub DeleteDelegatedPolicyUserName: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IAzApplication2, IAzApplication2_Vtbl, 0x086a68af_a249_437c_b18d_d4d86d6a9660);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IAzApplication2 {
    type Target = IAzApplication;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IAzApplication2, windows_core::IUnknown, super::super::System::Com::IDispatch, IAzApplication);
#[cfg(feature = "Win32_System_Com")]
impl IAzApplication2 {
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn InitializeClientContextFromToken2<P0>(&self, ultokenhandlelowpart: u32, ultokenhandlehighpart: u32, varreserved: P0) -> windows_core::Result<IAzClientContext2>
    where
        P0: windows_core::Param<windows_core::VARIANT>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).InitializeClientContextFromToken2)(windows_core::Interface::as_raw(self), ultokenhandlelowpart, ultokenhandlehighpart, varreserved.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn InitializeClientContext2<P0, P1>(&self, identifyingstring: P0, varreserved: P1) -> windows_core::Result<IAzClientContext2>
    where
        P0: windows_core::Param<windows_core::BSTR>,
        P1: windows_core::Param<windows_core::VARIANT>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).InitializeClientContext2)(windows_core::Interface::as_raw(self), identifyingstring.param().abi(), varreserved.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IAzApplication2_Vtbl {
    pub base__: IAzApplication_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub InitializeClientContextFromToken2: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, core::mem::MaybeUninit<windows_core::VARIANT>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    InitializeClientContextFromToken2: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub InitializeClientContext2: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, core::mem::MaybeUninit<windows_core::VARIANT>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    InitializeClientContext2: usize,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IAzApplication3, IAzApplication3_Vtbl, 0x181c845e_7196_4a7d_ac2e_020c0bb7a303);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IAzApplication3 {
    type Target = IAzApplication2;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IAzApplication3, windows_core::IUnknown, super::super::System::Com::IDispatch, IAzApplication, IAzApplication2);
#[cfg(feature = "Win32_System_Com")]
impl IAzApplication3 {
    pub unsafe fn ScopeExists<P0>(&self, bstrscopename: P0) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).ScopeExists)(windows_core::Interface::as_raw(self), bstrscopename.param().abi(), &mut result__).map(|| result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn OpenScope2<P0>(&self, bstrscopename: P0) -> windows_core::Result<IAzScope2>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).OpenScope2)(windows_core::Interface::as_raw(self), bstrscopename.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CreateScope2<P0>(&self, bstrscopename: P0) -> windows_core::Result<IAzScope2>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CreateScope2)(windows_core::Interface::as_raw(self), bstrscopename.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn DeleteScope2<P0>(&self, bstrscopename: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).DeleteScope2)(windows_core::Interface::as_raw(self), bstrscopename.param().abi()).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn RoleDefinitions(&self) -> windows_core::Result<IAzRoleDefinitions> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).RoleDefinitions)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CreateRoleDefinition<P0>(&self, bstrroledefinitionname: P0) -> windows_core::Result<IAzRoleDefinition>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CreateRoleDefinition)(windows_core::Interface::as_raw(self), bstrroledefinitionname.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn OpenRoleDefinition<P0>(&self, bstrroledefinitionname: P0) -> windows_core::Result<IAzRoleDefinition>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).OpenRoleDefinition)(windows_core::Interface::as_raw(self), bstrroledefinitionname.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn DeleteRoleDefinition<P0>(&self, bstrroledefinitionname: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).DeleteRoleDefinition)(windows_core::Interface::as_raw(self), bstrroledefinitionname.param().abi()).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn RoleAssignments(&self) -> windows_core::Result<IAzRoleAssignments> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).RoleAssignments)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CreateRoleAssignment<P0>(&self, bstrroleassignmentname: P0) -> windows_core::Result<IAzRoleAssignment>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CreateRoleAssignment)(windows_core::Interface::as_raw(self), bstrroleassignmentname.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn OpenRoleAssignment<P0>(&self, bstrroleassignmentname: P0) -> windows_core::Result<IAzRoleAssignment>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).OpenRoleAssignment)(windows_core::Interface::as_raw(self), bstrroleassignmentname.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn DeleteRoleAssignment<P0>(&self, bstrroleassignmentname: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).DeleteRoleAssignment)(windows_core::Interface::as_raw(self), bstrroleassignmentname.param().abi()).ok()
    }
    pub unsafe fn BizRulesEnabled(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).BizRulesEnabled)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetBizRulesEnabled<P0>(&self, benabled: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::VARIANT_BOOL>,
    {
        (windows_core::Interface::vtable(self).SetBizRulesEnabled)(windows_core::Interface::as_raw(self), benabled.param().abi()).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IAzApplication3_Vtbl {
    pub base__: IAzApplication2_Vtbl,
    pub ScopeExists: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub OpenScope2: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    OpenScope2: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub CreateScope2: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    CreateScope2: usize,
    pub DeleteScope2: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub RoleDefinitions: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    RoleDefinitions: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub CreateRoleDefinition: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    CreateRoleDefinition: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub OpenRoleDefinition: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    OpenRoleDefinition: usize,
    pub DeleteRoleDefinition: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub RoleAssignments: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    RoleAssignments: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub CreateRoleAssignment: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    CreateRoleAssignment: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub OpenRoleAssignment: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    OpenRoleAssignment: usize,
    pub DeleteRoleAssignment: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub BizRulesEnabled: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub SetBizRulesEnabled: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IAzApplicationGroup, IAzApplicationGroup_Vtbl, 0xf1b744cd_58a6_4e06_9fbf_36f6d779e21e);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IAzApplicationGroup {
    type Target = super::super::System::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IAzApplicationGroup, windows_core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl IAzApplicationGroup {
    pub unsafe fn Name(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Name)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetName<P0>(&self, bstrname: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).SetName)(windows_core::Interface::as_raw(self), bstrname.param().abi()).ok()
    }
    pub unsafe fn Type(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Type)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetType(&self, lprop: i32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetType)(windows_core::Interface::as_raw(self), lprop).ok()
    }
    pub unsafe fn LdapQuery(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).LdapQuery)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetLdapQuery<P0>(&self, bstrprop: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).SetLdapQuery)(windows_core::Interface::as_raw(self), bstrprop.param().abi()).ok()
    }
    pub unsafe fn AppMembers(&self) -> windows_core::Result<windows_core::VARIANT> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).AppMembers)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn AppNonMembers(&self) -> windows_core::Result<windows_core::VARIANT> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).AppNonMembers)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn Members(&self) -> windows_core::Result<windows_core::VARIANT> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Members)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn NonMembers(&self) -> windows_core::Result<windows_core::VARIANT> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).NonMembers)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn Description(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Description)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetDescription<P0>(&self, bstrdescription: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).SetDescription)(windows_core::Interface::as_raw(self), bstrdescription.param().abi()).ok()
    }
    pub unsafe fn AddAppMember<P0, P1>(&self, bstrprop: P0, varreserved: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
        P1: windows_core::Param<windows_core::VARIANT>,
    {
        (windows_core::Interface::vtable(self).AddAppMember)(windows_core::Interface::as_raw(self), bstrprop.param().abi(), varreserved.param().abi()).ok()
    }
    pub unsafe fn DeleteAppMember<P0, P1>(&self, bstrprop: P0, varreserved: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
        P1: windows_core::Param<windows_core::VARIANT>,
    {
        (windows_core::Interface::vtable(self).DeleteAppMember)(windows_core::Interface::as_raw(self), bstrprop.param().abi(), varreserved.param().abi()).ok()
    }
    pub unsafe fn AddAppNonMember<P0, P1>(&self, bstrprop: P0, varreserved: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
        P1: windows_core::Param<windows_core::VARIANT>,
    {
        (windows_core::Interface::vtable(self).AddAppNonMember)(windows_core::Interface::as_raw(self), bstrprop.param().abi(), varreserved.param().abi()).ok()
    }
    pub unsafe fn DeleteAppNonMember<P0, P1>(&self, bstrprop: P0, varreserved: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
        P1: windows_core::Param<windows_core::VARIANT>,
    {
        (windows_core::Interface::vtable(self).DeleteAppNonMember)(windows_core::Interface::as_raw(self), bstrprop.param().abi(), varreserved.param().abi()).ok()
    }
    pub unsafe fn AddMember<P0, P1>(&self, bstrprop: P0, varreserved: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
        P1: windows_core::Param<windows_core::VARIANT>,
    {
        (windows_core::Interface::vtable(self).AddMember)(windows_core::Interface::as_raw(self), bstrprop.param().abi(), varreserved.param().abi()).ok()
    }
    pub unsafe fn DeleteMember<P0, P1>(&self, bstrprop: P0, varreserved: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
        P1: windows_core::Param<windows_core::VARIANT>,
    {
        (windows_core::Interface::vtable(self).DeleteMember)(windows_core::Interface::as_raw(self), bstrprop.param().abi(), varreserved.param().abi()).ok()
    }
    pub unsafe fn AddNonMember<P0, P1>(&self, bstrprop: P0, varreserved: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
        P1: windows_core::Param<windows_core::VARIANT>,
    {
        (windows_core::Interface::vtable(self).AddNonMember)(windows_core::Interface::as_raw(self), bstrprop.param().abi(), varreserved.param().abi()).ok()
    }
    pub unsafe fn DeleteNonMember<P0, P1>(&self, bstrprop: P0, varreserved: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
        P1: windows_core::Param<windows_core::VARIANT>,
    {
        (windows_core::Interface::vtable(self).DeleteNonMember)(windows_core::Interface::as_raw(self), bstrprop.param().abi(), varreserved.param().abi()).ok()
    }
    pub unsafe fn Writable(&self) -> windows_core::Result<super::super::Foundation::BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Writable)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn GetProperty<P0>(&self, lpropid: i32, varreserved: P0) -> windows_core::Result<windows_core::VARIANT>
    where
        P0: windows_core::Param<windows_core::VARIANT>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetProperty)(windows_core::Interface::as_raw(self), lpropid, varreserved.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetProperty<P0, P1>(&self, lpropid: i32, varprop: P0, varreserved: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::VARIANT>,
        P1: windows_core::Param<windows_core::VARIANT>,
    {
        (windows_core::Interface::vtable(self).SetProperty)(windows_core::Interface::as_raw(self), lpropid, varprop.param().abi(), varreserved.param().abi()).ok()
    }
    pub unsafe fn AddPropertyItem<P0, P1>(&self, lpropid: i32, varprop: P0, varreserved: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::VARIANT>,
        P1: windows_core::Param<windows_core::VARIANT>,
    {
        (windows_core::Interface::vtable(self).AddPropertyItem)(windows_core::Interface::as_raw(self), lpropid, varprop.param().abi(), varreserved.param().abi()).ok()
    }
    pub unsafe fn DeletePropertyItem<P0, P1>(&self, lpropid: i32, varprop: P0, varreserved: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::VARIANT>,
        P1: windows_core::Param<windows_core::VARIANT>,
    {
        (windows_core::Interface::vtable(self).DeletePropertyItem)(windows_core::Interface::as_raw(self), lpropid, varprop.param().abi(), varreserved.param().abi()).ok()
    }
    pub unsafe fn Submit<P0>(&self, lflags: i32, varreserved: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::VARIANT>,
    {
        (windows_core::Interface::vtable(self).Submit)(windows_core::Interface::as_raw(self), lflags, varreserved.param().abi()).ok()
    }
    pub unsafe fn AddMemberName<P0, P1>(&self, bstrprop: P0, varreserved: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
        P1: windows_core::Param<windows_core::VARIANT>,
    {
        (windows_core::Interface::vtable(self).AddMemberName)(windows_core::Interface::as_raw(self), bstrprop.param().abi(), varreserved.param().abi()).ok()
    }
    pub unsafe fn DeleteMemberName<P0, P1>(&self, bstrprop: P0, varreserved: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
        P1: windows_core::Param<windows_core::VARIANT>,
    {
        (windows_core::Interface::vtable(self).DeleteMemberName)(windows_core::Interface::as_raw(self), bstrprop.param().abi(), varreserved.param().abi()).ok()
    }
    pub unsafe fn AddNonMemberName<P0, P1>(&self, bstrprop: P0, varreserved: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
        P1: windows_core::Param<windows_core::VARIANT>,
    {
        (windows_core::Interface::vtable(self).AddNonMemberName)(windows_core::Interface::as_raw(self), bstrprop.param().abi(), varreserved.param().abi()).ok()
    }
    pub unsafe fn DeleteNonMemberName<P0, P1>(&self, bstrprop: P0, varreserved: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
        P1: windows_core::Param<windows_core::VARIANT>,
    {
        (windows_core::Interface::vtable(self).DeleteNonMemberName)(windows_core::Interface::as_raw(self), bstrprop.param().abi(), varreserved.param().abi()).ok()
    }
    pub unsafe fn MembersName(&self) -> windows_core::Result<windows_core::VARIANT> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).MembersName)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn NonMembersName(&self) -> windows_core::Result<windows_core::VARIANT> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).NonMembersName)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IAzApplicationGroup_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub Name: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub SetName: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub Type: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetType: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub LdapQuery: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub SetLdapQuery: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub AppMembers: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
    pub AppNonMembers: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
    pub Members: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
    pub NonMembers: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
    pub Description: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub SetDescription: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub AddAppMember: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
    pub DeleteAppMember: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
    pub AddAppNonMember: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
    pub DeleteAppNonMember: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
    pub AddMember: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
    pub DeleteMember: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
    pub AddNonMember: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
    pub DeleteNonMember: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
    pub Writable: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::BOOL) -> windows_core::HRESULT,
    pub GetProperty: unsafe extern "system" fn(*mut core::ffi::c_void, i32, core::mem::MaybeUninit<windows_core::VARIANT>, *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
    pub SetProperty: unsafe extern "system" fn(*mut core::ffi::c_void, i32, core::mem::MaybeUninit<windows_core::VARIANT>, core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
    pub AddPropertyItem: unsafe extern "system" fn(*mut core::ffi::c_void, i32, core::mem::MaybeUninit<windows_core::VARIANT>, core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
    pub DeletePropertyItem: unsafe extern "system" fn(*mut core::ffi::c_void, i32, core::mem::MaybeUninit<windows_core::VARIANT>, core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
    pub Submit: unsafe extern "system" fn(*mut core::ffi::c_void, i32, core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
    pub AddMemberName: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
    pub DeleteMemberName: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
    pub AddNonMemberName: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
    pub DeleteNonMemberName: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
    pub MembersName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
    pub NonMembersName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IAzApplicationGroup2, IAzApplicationGroup2_Vtbl, 0x3f0613fc_b71a_464e_a11d_5b881a56cefa);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IAzApplicationGroup2 {
    type Target = IAzApplicationGroup;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IAzApplicationGroup2, windows_core::IUnknown, super::super::System::Com::IDispatch, IAzApplicationGroup);
#[cfg(feature = "Win32_System_Com")]
impl IAzApplicationGroup2 {
    pub unsafe fn BizRule(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).BizRule)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetBizRule<P0>(&self, bstrprop: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).SetBizRule)(windows_core::Interface::as_raw(self), bstrprop.param().abi()).ok()
    }
    pub unsafe fn BizRuleLanguage(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).BizRuleLanguage)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetBizRuleLanguage<P0>(&self, bstrprop: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).SetBizRuleLanguage)(windows_core::Interface::as_raw(self), bstrprop.param().abi()).ok()
    }
    pub unsafe fn BizRuleImportedPath(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).BizRuleImportedPath)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetBizRuleImportedPath<P0>(&self, bstrprop: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).SetBizRuleImportedPath)(windows_core::Interface::as_raw(self), bstrprop.param().abi()).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn RoleAssignments<P0, P1>(&self, bstrscopename: P0, brecursive: P1) -> windows_core::Result<IAzRoleAssignments>
    where
        P0: windows_core::Param<windows_core::BSTR>,
        P1: windows_core::Param<super::super::Foundation::VARIANT_BOOL>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).RoleAssignments)(windows_core::Interface::as_raw(self), bstrscopename.param().abi(), brecursive.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IAzApplicationGroup2_Vtbl {
    pub base__: IAzApplicationGroup_Vtbl,
    pub BizRule: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub SetBizRule: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub BizRuleLanguage: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub SetBizRuleLanguage: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub BizRuleImportedPath: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub SetBizRuleImportedPath: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub RoleAssignments: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, super::super::Foundation::VARIANT_BOOL, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    RoleAssignments: usize,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IAzApplicationGroups, IAzApplicationGroups_Vtbl, 0x4ce66ad5_9f3c_469d_a911_b99887a7e685);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IAzApplicationGroups {
    type Target = super::super::System::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IAzApplicationGroups, windows_core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl IAzApplicationGroups {
    pub unsafe fn get_Item(&self, index: i32) -> windows_core::Result<windows_core::VARIANT> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).get_Item)(windows_core::Interface::as_raw(self), index, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn Count(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Count)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn _NewEnum(&self) -> windows_core::Result<windows_core::IUnknown> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self)._NewEnum)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IAzApplicationGroups_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub get_Item: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
    pub Count: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub _NewEnum: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IAzApplications, IAzApplications_Vtbl, 0x929b11a9_95c5_4a84_a29a_20ad42c2f16c);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IAzApplications {
    type Target = super::super::System::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IAzApplications, windows_core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl IAzApplications {
    pub unsafe fn get_Item(&self, index: i32) -> windows_core::Result<windows_core::VARIANT> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).get_Item)(windows_core::Interface::as_raw(self), index, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn Count(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Count)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn _NewEnum(&self) -> windows_core::Result<windows_core::IUnknown> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self)._NewEnum)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IAzApplications_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub get_Item: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
    pub Count: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub _NewEnum: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IAzAuthorizationStore, IAzAuthorizationStore_Vtbl, 0xedbd9ca9_9b82_4f6a_9e8b_98301e450f14);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IAzAuthorizationStore {
    type Target = super::super::System::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IAzAuthorizationStore, windows_core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl IAzAuthorizationStore {
    pub unsafe fn Description(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Description)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetDescription<P0>(&self, bstrdescription: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).SetDescription)(windows_core::Interface::as_raw(self), bstrdescription.param().abi()).ok()
    }
    pub unsafe fn ApplicationData(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).ApplicationData)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetApplicationData<P0>(&self, bstrapplicationdata: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).SetApplicationData)(windows_core::Interface::as_raw(self), bstrapplicationdata.param().abi()).ok()
    }
    pub unsafe fn DomainTimeout(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).DomainTimeout)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetDomainTimeout(&self, lprop: i32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetDomainTimeout)(windows_core::Interface::as_raw(self), lprop).ok()
    }
    pub unsafe fn ScriptEngineTimeout(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).ScriptEngineTimeout)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetScriptEngineTimeout(&self, lprop: i32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetScriptEngineTimeout)(windows_core::Interface::as_raw(self), lprop).ok()
    }
    pub unsafe fn MaxScriptEngines(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).MaxScriptEngines)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetMaxScriptEngines(&self, lprop: i32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetMaxScriptEngines)(windows_core::Interface::as_raw(self), lprop).ok()
    }
    pub unsafe fn GenerateAudits(&self) -> windows_core::Result<super::super::Foundation::BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GenerateAudits)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetGenerateAudits<P0>(&self, bprop: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::BOOL>,
    {
        (windows_core::Interface::vtable(self).SetGenerateAudits)(windows_core::Interface::as_raw(self), bprop.param().abi()).ok()
    }
    pub unsafe fn Writable(&self) -> windows_core::Result<super::super::Foundation::BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Writable)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn GetProperty<P0>(&self, lpropid: i32, varreserved: P0) -> windows_core::Result<windows_core::VARIANT>
    where
        P0: windows_core::Param<windows_core::VARIANT>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetProperty)(windows_core::Interface::as_raw(self), lpropid, varreserved.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetProperty<P0, P1>(&self, lpropid: i32, varprop: P0, varreserved: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::VARIANT>,
        P1: windows_core::Param<windows_core::VARIANT>,
    {
        (windows_core::Interface::vtable(self).SetProperty)(windows_core::Interface::as_raw(self), lpropid, varprop.param().abi(), varreserved.param().abi()).ok()
    }
    pub unsafe fn AddPropertyItem<P0, P1>(&self, lpropid: AZ_PROP_CONSTANTS, varprop: P0, varreserved: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::VARIANT>,
        P1: windows_core::Param<windows_core::VARIANT>,
    {
        (windows_core::Interface::vtable(self).AddPropertyItem)(windows_core::Interface::as_raw(self), lpropid, varprop.param().abi(), varreserved.param().abi()).ok()
    }
    pub unsafe fn DeletePropertyItem<P0, P1>(&self, lpropid: i32, varprop: P0, varreserved: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::VARIANT>,
        P1: windows_core::Param<windows_core::VARIANT>,
    {
        (windows_core::Interface::vtable(self).DeletePropertyItem)(windows_core::Interface::as_raw(self), lpropid, varprop.param().abi(), varreserved.param().abi()).ok()
    }
    pub unsafe fn PolicyAdministrators(&self) -> windows_core::Result<windows_core::VARIANT> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).PolicyAdministrators)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn PolicyReaders(&self) -> windows_core::Result<windows_core::VARIANT> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).PolicyReaders)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn AddPolicyAdministrator<P0, P1>(&self, bstradmin: P0, varreserved: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
        P1: windows_core::Param<windows_core::VARIANT>,
    {
        (windows_core::Interface::vtable(self).AddPolicyAdministrator)(windows_core::Interface::as_raw(self), bstradmin.param().abi(), varreserved.param().abi()).ok()
    }
    pub unsafe fn DeletePolicyAdministrator<P0, P1>(&self, bstradmin: P0, varreserved: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
        P1: windows_core::Param<windows_core::VARIANT>,
    {
        (windows_core::Interface::vtable(self).DeletePolicyAdministrator)(windows_core::Interface::as_raw(self), bstradmin.param().abi(), varreserved.param().abi()).ok()
    }
    pub unsafe fn AddPolicyReader<P0, P1>(&self, bstrreader: P0, varreserved: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
        P1: windows_core::Param<windows_core::VARIANT>,
    {
        (windows_core::Interface::vtable(self).AddPolicyReader)(windows_core::Interface::as_raw(self), bstrreader.param().abi(), varreserved.param().abi()).ok()
    }
    pub unsafe fn DeletePolicyReader<P0, P1>(&self, bstrreader: P0, varreserved: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
        P1: windows_core::Param<windows_core::VARIANT>,
    {
        (windows_core::Interface::vtable(self).DeletePolicyReader)(windows_core::Interface::as_raw(self), bstrreader.param().abi(), varreserved.param().abi()).ok()
    }
    pub unsafe fn Initialize<P0, P1>(&self, lflags: AZ_PROP_CONSTANTS, bstrpolicyurl: P0, varreserved: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
        P1: windows_core::Param<windows_core::VARIANT>,
    {
        (windows_core::Interface::vtable(self).Initialize)(windows_core::Interface::as_raw(self), lflags, bstrpolicyurl.param().abi(), varreserved.param().abi()).ok()
    }
    pub unsafe fn UpdateCache<P0>(&self, varreserved: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::VARIANT>,
    {
        (windows_core::Interface::vtable(self).UpdateCache)(windows_core::Interface::as_raw(self), varreserved.param().abi()).ok()
    }
    pub unsafe fn Delete<P0>(&self, varreserved: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::VARIANT>,
    {
        (windows_core::Interface::vtable(self).Delete)(windows_core::Interface::as_raw(self), varreserved.param().abi()).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Applications(&self) -> windows_core::Result<IAzApplications> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Applications)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn OpenApplication<P0, P1>(&self, bstrapplicationname: P0, varreserved: P1) -> windows_core::Result<IAzApplication>
    where
        P0: windows_core::Param<windows_core::BSTR>,
        P1: windows_core::Param<windows_core::VARIANT>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).OpenApplication)(windows_core::Interface::as_raw(self), bstrapplicationname.param().abi(), varreserved.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CreateApplication<P0, P1>(&self, bstrapplicationname: P0, varreserved: P1) -> windows_core::Result<IAzApplication>
    where
        P0: windows_core::Param<windows_core::BSTR>,
        P1: windows_core::Param<windows_core::VARIANT>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CreateApplication)(windows_core::Interface::as_raw(self), bstrapplicationname.param().abi(), varreserved.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn DeleteApplication<P0, P1>(&self, bstrapplicationname: P0, varreserved: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
        P1: windows_core::Param<windows_core::VARIANT>,
    {
        (windows_core::Interface::vtable(self).DeleteApplication)(windows_core::Interface::as_raw(self), bstrapplicationname.param().abi(), varreserved.param().abi()).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn ApplicationGroups(&self) -> windows_core::Result<IAzApplicationGroups> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).ApplicationGroups)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CreateApplicationGroup<P0, P1>(&self, bstrgroupname: P0, varreserved: P1) -> windows_core::Result<IAzApplicationGroup>
    where
        P0: windows_core::Param<windows_core::BSTR>,
        P1: windows_core::Param<windows_core::VARIANT>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CreateApplicationGroup)(windows_core::Interface::as_raw(self), bstrgroupname.param().abi(), varreserved.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn OpenApplicationGroup<P0, P1>(&self, bstrgroupname: P0, varreserved: P1) -> windows_core::Result<IAzApplicationGroup>
    where
        P0: windows_core::Param<windows_core::BSTR>,
        P1: windows_core::Param<windows_core::VARIANT>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).OpenApplicationGroup)(windows_core::Interface::as_raw(self), bstrgroupname.param().abi(), varreserved.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn DeleteApplicationGroup<P0, P1>(&self, bstrgroupname: P0, varreserved: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
        P1: windows_core::Param<windows_core::VARIANT>,
    {
        (windows_core::Interface::vtable(self).DeleteApplicationGroup)(windows_core::Interface::as_raw(self), bstrgroupname.param().abi(), varreserved.param().abi()).ok()
    }
    pub unsafe fn Submit<P0>(&self, lflags: i32, varreserved: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::VARIANT>,
    {
        (windows_core::Interface::vtable(self).Submit)(windows_core::Interface::as_raw(self), lflags, varreserved.param().abi()).ok()
    }
    pub unsafe fn DelegatedPolicyUsers(&self) -> windows_core::Result<windows_core::VARIANT> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).DelegatedPolicyUsers)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn AddDelegatedPolicyUser<P0, P1>(&self, bstrdelegatedpolicyuser: P0, varreserved: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
        P1: windows_core::Param<windows_core::VARIANT>,
    {
        (windows_core::Interface::vtable(self).AddDelegatedPolicyUser)(windows_core::Interface::as_raw(self), bstrdelegatedpolicyuser.param().abi(), varreserved.param().abi()).ok()
    }
    pub unsafe fn DeleteDelegatedPolicyUser<P0, P1>(&self, bstrdelegatedpolicyuser: P0, varreserved: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
        P1: windows_core::Param<windows_core::VARIANT>,
    {
        (windows_core::Interface::vtable(self).DeleteDelegatedPolicyUser)(windows_core::Interface::as_raw(self), bstrdelegatedpolicyuser.param().abi(), varreserved.param().abi()).ok()
    }
    pub unsafe fn TargetMachine(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).TargetMachine)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn ApplyStoreSacl(&self) -> windows_core::Result<super::super::Foundation::BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).ApplyStoreSacl)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetApplyStoreSacl<P0>(&self, bapplystoresacl: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::BOOL>,
    {
        (windows_core::Interface::vtable(self).SetApplyStoreSacl)(windows_core::Interface::as_raw(self), bapplystoresacl.param().abi()).ok()
    }
    pub unsafe fn PolicyAdministratorsName(&self) -> windows_core::Result<windows_core::VARIANT> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).PolicyAdministratorsName)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn PolicyReadersName(&self) -> windows_core::Result<windows_core::VARIANT> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).PolicyReadersName)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn AddPolicyAdministratorName<P0, P1>(&self, bstradmin: P0, varreserved: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
        P1: windows_core::Param<windows_core::VARIANT>,
    {
        (windows_core::Interface::vtable(self).AddPolicyAdministratorName)(windows_core::Interface::as_raw(self), bstradmin.param().abi(), varreserved.param().abi()).ok()
    }
    pub unsafe fn DeletePolicyAdministratorName<P0, P1>(&self, bstradmin: P0, varreserved: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
        P1: windows_core::Param<windows_core::VARIANT>,
    {
        (windows_core::Interface::vtable(self).DeletePolicyAdministratorName)(windows_core::Interface::as_raw(self), bstradmin.param().abi(), varreserved.param().abi()).ok()
    }
    pub unsafe fn AddPolicyReaderName<P0, P1>(&self, bstrreader: P0, varreserved: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
        P1: windows_core::Param<windows_core::VARIANT>,
    {
        (windows_core::Interface::vtable(self).AddPolicyReaderName)(windows_core::Interface::as_raw(self), bstrreader.param().abi(), varreserved.param().abi()).ok()
    }
    pub unsafe fn DeletePolicyReaderName<P0, P1>(&self, bstrreader: P0, varreserved: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
        P1: windows_core::Param<windows_core::VARIANT>,
    {
        (windows_core::Interface::vtable(self).DeletePolicyReaderName)(windows_core::Interface::as_raw(self), bstrreader.param().abi(), varreserved.param().abi()).ok()
    }
    pub unsafe fn DelegatedPolicyUsersName(&self) -> windows_core::Result<windows_core::VARIANT> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).DelegatedPolicyUsersName)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn AddDelegatedPolicyUserName<P0, P1>(&self, bstrdelegatedpolicyuser: P0, varreserved: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
        P1: windows_core::Param<windows_core::VARIANT>,
    {
        (windows_core::Interface::vtable(self).AddDelegatedPolicyUserName)(windows_core::Interface::as_raw(self), bstrdelegatedpolicyuser.param().abi(), varreserved.param().abi()).ok()
    }
    pub unsafe fn DeleteDelegatedPolicyUserName<P0, P1>(&self, bstrdelegatedpolicyuser: P0, varreserved: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
        P1: windows_core::Param<windows_core::VARIANT>,
    {
        (windows_core::Interface::vtable(self).DeleteDelegatedPolicyUserName)(windows_core::Interface::as_raw(self), bstrdelegatedpolicyuser.param().abi(), varreserved.param().abi()).ok()
    }
    pub unsafe fn CloseApplication<P0>(&self, bstrapplicationname: P0, lflag: i32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).CloseApplication)(windows_core::Interface::as_raw(self), bstrapplicationname.param().abi(), lflag).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IAzAuthorizationStore_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub Description: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub SetDescription: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub ApplicationData: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub SetApplicationData: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub DomainTimeout: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetDomainTimeout: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub ScriptEngineTimeout: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetScriptEngineTimeout: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub MaxScriptEngines: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetMaxScriptEngines: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub GenerateAudits: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::BOOL) -> windows_core::HRESULT,
    pub SetGenerateAudits: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::BOOL) -> windows_core::HRESULT,
    pub Writable: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::BOOL) -> windows_core::HRESULT,
    pub GetProperty: unsafe extern "system" fn(*mut core::ffi::c_void, i32, core::mem::MaybeUninit<windows_core::VARIANT>, *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
    pub SetProperty: unsafe extern "system" fn(*mut core::ffi::c_void, i32, core::mem::MaybeUninit<windows_core::VARIANT>, core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
    pub AddPropertyItem: unsafe extern "system" fn(*mut core::ffi::c_void, AZ_PROP_CONSTANTS, core::mem::MaybeUninit<windows_core::VARIANT>, core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
    pub DeletePropertyItem: unsafe extern "system" fn(*mut core::ffi::c_void, i32, core::mem::MaybeUninit<windows_core::VARIANT>, core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
    pub PolicyAdministrators: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
    pub PolicyReaders: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
    pub AddPolicyAdministrator: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
    pub DeletePolicyAdministrator: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
    pub AddPolicyReader: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
    pub DeletePolicyReader: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
    pub Initialize: unsafe extern "system" fn(*mut core::ffi::c_void, AZ_PROP_CONSTANTS, core::mem::MaybeUninit<windows_core::BSTR>, core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
    pub UpdateCache: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
    pub Delete: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub Applications: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Applications: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub OpenApplication: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, core::mem::MaybeUninit<windows_core::VARIANT>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    OpenApplication: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub CreateApplication: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, core::mem::MaybeUninit<windows_core::VARIANT>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    CreateApplication: usize,
    pub DeleteApplication: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub ApplicationGroups: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    ApplicationGroups: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub CreateApplicationGroup: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, core::mem::MaybeUninit<windows_core::VARIANT>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    CreateApplicationGroup: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub OpenApplicationGroup: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, core::mem::MaybeUninit<windows_core::VARIANT>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    OpenApplicationGroup: usize,
    pub DeleteApplicationGroup: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
    pub Submit: unsafe extern "system" fn(*mut core::ffi::c_void, i32, core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
    pub DelegatedPolicyUsers: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
    pub AddDelegatedPolicyUser: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
    pub DeleteDelegatedPolicyUser: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
    pub TargetMachine: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub ApplyStoreSacl: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::BOOL) -> windows_core::HRESULT,
    pub SetApplyStoreSacl: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::BOOL) -> windows_core::HRESULT,
    pub PolicyAdministratorsName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
    pub PolicyReadersName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
    pub AddPolicyAdministratorName: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
    pub DeletePolicyAdministratorName: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
    pub AddPolicyReaderName: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
    pub DeletePolicyReaderName: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
    pub DelegatedPolicyUsersName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
    pub AddDelegatedPolicyUserName: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
    pub DeleteDelegatedPolicyUserName: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
    pub CloseApplication: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, i32) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IAzAuthorizationStore2, IAzAuthorizationStore2_Vtbl, 0xb11e5584_d577_4273_b6c5_0973e0f8e80d);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IAzAuthorizationStore2 {
    type Target = IAzAuthorizationStore;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IAzAuthorizationStore2, windows_core::IUnknown, super::super::System::Com::IDispatch, IAzAuthorizationStore);
#[cfg(feature = "Win32_System_Com")]
impl IAzAuthorizationStore2 {
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn OpenApplication2<P0, P1>(&self, bstrapplicationname: P0, varreserved: P1) -> windows_core::Result<IAzApplication2>
    where
        P0: windows_core::Param<windows_core::BSTR>,
        P1: windows_core::Param<windows_core::VARIANT>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).OpenApplication2)(windows_core::Interface::as_raw(self), bstrapplicationname.param().abi(), varreserved.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CreateApplication2<P0, P1>(&self, bstrapplicationname: P0, varreserved: P1) -> windows_core::Result<IAzApplication2>
    where
        P0: windows_core::Param<windows_core::BSTR>,
        P1: windows_core::Param<windows_core::VARIANT>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CreateApplication2)(windows_core::Interface::as_raw(self), bstrapplicationname.param().abi(), varreserved.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IAzAuthorizationStore2_Vtbl {
    pub base__: IAzAuthorizationStore_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub OpenApplication2: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, core::mem::MaybeUninit<windows_core::VARIANT>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    OpenApplication2: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub CreateApplication2: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, core::mem::MaybeUninit<windows_core::VARIANT>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    CreateApplication2: usize,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IAzAuthorizationStore3, IAzAuthorizationStore3_Vtbl, 0xabc08425_0c86_4fa0_9be3_7189956c926e);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IAzAuthorizationStore3 {
    type Target = IAzAuthorizationStore2;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IAzAuthorizationStore3, windows_core::IUnknown, super::super::System::Com::IDispatch, IAzAuthorizationStore, IAzAuthorizationStore2);
#[cfg(feature = "Win32_System_Com")]
impl IAzAuthorizationStore3 {
    pub unsafe fn IsUpdateNeeded(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).IsUpdateNeeded)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn BizruleGroupSupported(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).BizruleGroupSupported)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn UpgradeStoresFunctionalLevel(&self, lfunctionallevel: i32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).UpgradeStoresFunctionalLevel)(windows_core::Interface::as_raw(self), lfunctionallevel).ok()
    }
    pub unsafe fn IsFunctionalLevelUpgradeSupported(&self, lfunctionallevel: i32) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).IsFunctionalLevelUpgradeSupported)(windows_core::Interface::as_raw(self), lfunctionallevel, &mut result__).map(|| result__)
    }
    pub unsafe fn GetSchemaVersion(&self, plmajorversion: *mut i32, plminorversion: *mut i32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetSchemaVersion)(windows_core::Interface::as_raw(self), plmajorversion, plminorversion).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IAzAuthorizationStore3_Vtbl {
    pub base__: IAzAuthorizationStore2_Vtbl,
    pub IsUpdateNeeded: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub BizruleGroupSupported: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub UpgradeStoresFunctionalLevel: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub IsFunctionalLevelUpgradeSupported: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub GetSchemaVersion: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32, *mut i32) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IAzBizRuleContext, IAzBizRuleContext_Vtbl, 0xe192f17d_d59f_455e_a152_940316cd77b2);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IAzBizRuleContext {
    type Target = super::super::System::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IAzBizRuleContext, windows_core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl IAzBizRuleContext {
    pub unsafe fn SetBusinessRuleResult<P0>(&self, bresult: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::BOOL>,
    {
        (windows_core::Interface::vtable(self).SetBusinessRuleResult)(windows_core::Interface::as_raw(self), bresult.param().abi()).ok()
    }
    pub unsafe fn SetBusinessRuleString<P0>(&self, bstrbusinessrulestring: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).SetBusinessRuleString)(windows_core::Interface::as_raw(self), bstrbusinessrulestring.param().abi()).ok()
    }
    pub unsafe fn BusinessRuleString(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).BusinessRuleString)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn GetParameter<P0>(&self, bstrparametername: P0) -> windows_core::Result<windows_core::VARIANT>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetParameter)(windows_core::Interface::as_raw(self), bstrparametername.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IAzBizRuleContext_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub SetBusinessRuleResult: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::BOOL) -> windows_core::HRESULT,
    pub SetBusinessRuleString: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub BusinessRuleString: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub GetParameter: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IAzBizRuleInterfaces, IAzBizRuleInterfaces_Vtbl, 0xe94128c7_e9da_44cc_b0bd_53036f3aab3d);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IAzBizRuleInterfaces {
    type Target = super::super::System::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IAzBizRuleInterfaces, windows_core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl IAzBizRuleInterfaces {
    pub unsafe fn AddInterface<P0, P1>(&self, bstrinterfacename: P0, linterfaceflag: i32, varinterface: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
        P1: windows_core::Param<windows_core::VARIANT>,
    {
        (windows_core::Interface::vtable(self).AddInterface)(windows_core::Interface::as_raw(self), bstrinterfacename.param().abi(), linterfaceflag, varinterface.param().abi()).ok()
    }
    pub unsafe fn AddInterfaces<P0, P1, P2>(&self, varinterfacenames: P0, varinterfaceflags: P1, varinterfaces: P2) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::VARIANT>,
        P1: windows_core::Param<windows_core::VARIANT>,
        P2: windows_core::Param<windows_core::VARIANT>,
    {
        (windows_core::Interface::vtable(self).AddInterfaces)(windows_core::Interface::as_raw(self), varinterfacenames.param().abi(), varinterfaceflags.param().abi(), varinterfaces.param().abi()).ok()
    }
    pub unsafe fn GetInterfaceValue<P0>(&self, bstrinterfacename: P0, linterfaceflag: *mut i32, varinterface: *mut windows_core::VARIANT) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).GetInterfaceValue)(windows_core::Interface::as_raw(self), bstrinterfacename.param().abi(), linterfaceflag, core::mem::transmute(varinterface)).ok()
    }
    pub unsafe fn Remove<P0>(&self, bstrinterfacename: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).Remove)(windows_core::Interface::as_raw(self), bstrinterfacename.param().abi()).ok()
    }
    pub unsafe fn RemoveAll(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).RemoveAll)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Count(&self) -> windows_core::Result<u32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Count)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IAzBizRuleInterfaces_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub AddInterface: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, i32, core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
    pub AddInterfaces: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::VARIANT>, core::mem::MaybeUninit<windows_core::VARIANT>, core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
    pub GetInterfaceValue: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, *mut i32, *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
    pub Remove: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub RemoveAll: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Count: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IAzBizRuleParameters, IAzBizRuleParameters_Vtbl, 0xfc17685f_e25d_4dcd_bae1_276ec9533cb5);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IAzBizRuleParameters {
    type Target = super::super::System::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IAzBizRuleParameters, windows_core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl IAzBizRuleParameters {
    pub unsafe fn AddParameter<P0, P1>(&self, bstrparametername: P0, varparametervalue: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
        P1: windows_core::Param<windows_core::VARIANT>,
    {
        (windows_core::Interface::vtable(self).AddParameter)(windows_core::Interface::as_raw(self), bstrparametername.param().abi(), varparametervalue.param().abi()).ok()
    }
    pub unsafe fn AddParameters<P0, P1>(&self, varparameternames: P0, varparametervalues: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::VARIANT>,
        P1: windows_core::Param<windows_core::VARIANT>,
    {
        (windows_core::Interface::vtable(self).AddParameters)(windows_core::Interface::as_raw(self), varparameternames.param().abi(), varparametervalues.param().abi()).ok()
    }
    pub unsafe fn GetParameterValue<P0>(&self, bstrparametername: P0) -> windows_core::Result<windows_core::VARIANT>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetParameterValue)(windows_core::Interface::as_raw(self), bstrparametername.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn Remove<P0>(&self, varparametername: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).Remove)(windows_core::Interface::as_raw(self), varparametername.param().abi()).ok()
    }
    pub unsafe fn RemoveAll(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).RemoveAll)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Count(&self) -> windows_core::Result<u32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Count)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IAzBizRuleParameters_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub AddParameter: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
    pub AddParameters: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::VARIANT>, core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
    pub GetParameterValue: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
    pub Remove: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub RemoveAll: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Count: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IAzClientContext, IAzClientContext_Vtbl, 0xeff1f00b_488a_466d_afd9_a401c5f9eef5);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IAzClientContext {
    type Target = super::super::System::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IAzClientContext, windows_core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl IAzClientContext {
    pub unsafe fn AccessCheck<P0, P1, P2, P3, P4, P5, P6, P7>(&self, bstrobjectname: P0, varscopenames: P1, varoperations: P2, varparameternames: P3, varparametervalues: P4, varinterfacenames: P5, varinterfaceflags: P6, varinterfaces: P7) -> windows_core::Result<windows_core::VARIANT>
    where
        P0: windows_core::Param<windows_core::BSTR>,
        P1: windows_core::Param<windows_core::VARIANT>,
        P2: windows_core::Param<windows_core::VARIANT>,
        P3: windows_core::Param<windows_core::VARIANT>,
        P4: windows_core::Param<windows_core::VARIANT>,
        P5: windows_core::Param<windows_core::VARIANT>,
        P6: windows_core::Param<windows_core::VARIANT>,
        P7: windows_core::Param<windows_core::VARIANT>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).AccessCheck)(windows_core::Interface::as_raw(self), bstrobjectname.param().abi(), varscopenames.param().abi(), varoperations.param().abi(), varparameternames.param().abi(), varparametervalues.param().abi(), varinterfacenames.param().abi(), varinterfaceflags.param().abi(), varinterfaces.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn GetBusinessRuleString(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetBusinessRuleString)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn UserDn(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).UserDn)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn UserSamCompat(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).UserSamCompat)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn UserDisplay(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).UserDisplay)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn UserGuid(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).UserGuid)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn UserCanonical(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).UserCanonical)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn UserUpn(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).UserUpn)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn UserDnsSamCompat(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).UserDnsSamCompat)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn GetProperty<P0>(&self, lpropid: i32, varreserved: P0) -> windows_core::Result<windows_core::VARIANT>
    where
        P0: windows_core::Param<windows_core::VARIANT>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetProperty)(windows_core::Interface::as_raw(self), lpropid, varreserved.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn GetRoles<P0>(&self, bstrscopename: P0) -> windows_core::Result<windows_core::VARIANT>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetRoles)(windows_core::Interface::as_raw(self), bstrscopename.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn RoleForAccessCheck(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).RoleForAccessCheck)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetRoleForAccessCheck<P0>(&self, bstrprop: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).SetRoleForAccessCheck)(windows_core::Interface::as_raw(self), bstrprop.param().abi()).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IAzClientContext_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub AccessCheck: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, core::mem::MaybeUninit<windows_core::VARIANT>, core::mem::MaybeUninit<windows_core::VARIANT>, core::mem::MaybeUninit<windows_core::VARIANT>, core::mem::MaybeUninit<windows_core::VARIANT>, core::mem::MaybeUninit<windows_core::VARIANT>, core::mem::MaybeUninit<windows_core::VARIANT>, core::mem::MaybeUninit<windows_core::VARIANT>, *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
    pub GetBusinessRuleString: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub UserDn: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub UserSamCompat: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub UserDisplay: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub UserGuid: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub UserCanonical: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub UserUpn: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub UserDnsSamCompat: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub GetProperty: unsafe extern "system" fn(*mut core::ffi::c_void, i32, core::mem::MaybeUninit<windows_core::VARIANT>, *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
    pub GetRoles: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
    pub RoleForAccessCheck: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub SetRoleForAccessCheck: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IAzClientContext2, IAzClientContext2_Vtbl, 0x2b0c92b8_208a_488a_8f81_e4edb22111cd);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IAzClientContext2 {
    type Target = IAzClientContext;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IAzClientContext2, windows_core::IUnknown, super::super::System::Com::IDispatch, IAzClientContext);
#[cfg(feature = "Win32_System_Com")]
impl IAzClientContext2 {
    pub unsafe fn GetAssignedScopesPage(&self, loptions: i32, pagesize: i32, pvarcursor: *mut windows_core::VARIANT, pvarscopenames: *mut windows_core::VARIANT) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetAssignedScopesPage)(windows_core::Interface::as_raw(self), loptions, pagesize, core::mem::transmute(pvarcursor), core::mem::transmute(pvarscopenames)).ok()
    }
    pub unsafe fn AddRoles<P0, P1>(&self, varroles: P0, bstrscopename: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::VARIANT>,
        P1: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).AddRoles)(windows_core::Interface::as_raw(self), varroles.param().abi(), bstrscopename.param().abi()).ok()
    }
    pub unsafe fn AddApplicationGroups<P0>(&self, varapplicationgroups: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::VARIANT>,
    {
        (windows_core::Interface::vtable(self).AddApplicationGroups)(windows_core::Interface::as_raw(self), varapplicationgroups.param().abi()).ok()
    }
    pub unsafe fn AddStringSids<P0>(&self, varstringsids: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::VARIANT>,
    {
        (windows_core::Interface::vtable(self).AddStringSids)(windows_core::Interface::as_raw(self), varstringsids.param().abi()).ok()
    }
    pub unsafe fn SetLDAPQueryDN<P0>(&self, bstrldapquerydn: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).SetLDAPQueryDN)(windows_core::Interface::as_raw(self), bstrldapquerydn.param().abi()).ok()
    }
    pub unsafe fn LDAPQueryDN(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).LDAPQueryDN)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IAzClientContext2_Vtbl {
    pub base__: IAzClientContext_Vtbl,
    pub GetAssignedScopesPage: unsafe extern "system" fn(*mut core::ffi::c_void, i32, i32, *mut core::mem::MaybeUninit<windows_core::VARIANT>, *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
    pub AddRoles: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::VARIANT>, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub AddApplicationGroups: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
    pub AddStringSids: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
    pub SetLDAPQueryDN: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub LDAPQueryDN: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IAzClientContext3, IAzClientContext3_Vtbl, 0x11894fde_1deb_4b4b_8907_6d1cda1f5d4f);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IAzClientContext3 {
    type Target = IAzClientContext2;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IAzClientContext3, windows_core::IUnknown, super::super::System::Com::IDispatch, IAzClientContext, IAzClientContext2);
#[cfg(feature = "Win32_System_Com")]
impl IAzClientContext3 {
    pub unsafe fn AccessCheck2<P0, P1>(&self, bstrobjectname: P0, bstrscopename: P1, loperation: i32) -> windows_core::Result<u32>
    where
        P0: windows_core::Param<windows_core::BSTR>,
        P1: windows_core::Param<windows_core::BSTR>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).AccessCheck2)(windows_core::Interface::as_raw(self), bstrobjectname.param().abi(), bstrscopename.param().abi(), loperation, &mut result__).map(|| result__)
    }
    pub unsafe fn IsInRoleAssignment<P0, P1>(&self, bstrscopename: P0, bstrrolename: P1) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>
    where
        P0: windows_core::Param<windows_core::BSTR>,
        P1: windows_core::Param<windows_core::BSTR>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).IsInRoleAssignment)(windows_core::Interface::as_raw(self), bstrscopename.param().abi(), bstrrolename.param().abi(), &mut result__).map(|| result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetOperations<P0>(&self, bstrscopename: P0) -> windows_core::Result<IAzOperations>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetOperations)(windows_core::Interface::as_raw(self), bstrscopename.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetTasks<P0>(&self, bstrscopename: P0) -> windows_core::Result<IAzTasks>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetTasks)(windows_core::Interface::as_raw(self), bstrscopename.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn BizRuleParameters(&self) -> windows_core::Result<IAzBizRuleParameters> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).BizRuleParameters)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn BizRuleInterfaces(&self) -> windows_core::Result<IAzBizRuleInterfaces> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).BizRuleInterfaces)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn GetGroups<P0>(&self, bstrscopename: P0, uloptions: AZ_PROP_CONSTANTS) -> windows_core::Result<windows_core::VARIANT>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetGroups)(windows_core::Interface::as_raw(self), bstrscopename.param().abi(), uloptions.0 as _, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn Sids(&self) -> windows_core::Result<windows_core::VARIANT> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Sids)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IAzClientContext3_Vtbl {
    pub base__: IAzClientContext2_Vtbl,
    pub AccessCheck2: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, core::mem::MaybeUninit<windows_core::BSTR>, i32, *mut u32) -> windows_core::HRESULT,
    pub IsInRoleAssignment: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, core::mem::MaybeUninit<windows_core::BSTR>, *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub GetOperations: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetOperations: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub GetTasks: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetTasks: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub BizRuleParameters: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    BizRuleParameters: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub BizRuleInterfaces: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    BizRuleInterfaces: usize,
    pub GetGroups: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, u32, *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
    pub Sids: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IAzNameResolver, IAzNameResolver_Vtbl, 0x504d0f15_73e2_43df_a870_a64f40714f53);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IAzNameResolver {
    type Target = super::super::System::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IAzNameResolver, windows_core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl IAzNameResolver {
    pub unsafe fn NameFromSid<P0>(&self, bstrsid: P0, psidtype: *mut i32, pbstrname: *mut windows_core::BSTR) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).NameFromSid)(windows_core::Interface::as_raw(self), bstrsid.param().abi(), psidtype, core::mem::transmute(pbstrname)).ok()
    }
    pub unsafe fn NamesFromSids<P0>(&self, vsids: P0, pvsidtypes: *mut windows_core::VARIANT, pvnames: *mut windows_core::VARIANT) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::VARIANT>,
    {
        (windows_core::Interface::vtable(self).NamesFromSids)(windows_core::Interface::as_raw(self), vsids.param().abi(), core::mem::transmute(pvsidtypes), core::mem::transmute(pvnames)).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IAzNameResolver_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub NameFromSid: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, *mut i32, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub NamesFromSids: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::VARIANT>, *mut core::mem::MaybeUninit<windows_core::VARIANT>, *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IAzObjectPicker, IAzObjectPicker_Vtbl, 0x63130a48_699a_42d8_bf01_c62ac3fb79f9);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IAzObjectPicker {
    type Target = super::super::System::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IAzObjectPicker, windows_core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl IAzObjectPicker {
    pub unsafe fn GetPrincipals<P0, P1>(&self, hparentwnd: P0, bstrtitle: P1, pvsidtypes: *mut windows_core::VARIANT, pvnames: *mut windows_core::VARIANT, pvsids: *mut windows_core::VARIANT) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::HWND>,
        P1: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).GetPrincipals)(windows_core::Interface::as_raw(self), hparentwnd.param().abi(), bstrtitle.param().abi(), core::mem::transmute(pvsidtypes), core::mem::transmute(pvnames), core::mem::transmute(pvsids)).ok()
    }
    pub unsafe fn Name(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Name)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IAzObjectPicker_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub GetPrincipals: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::HWND, core::mem::MaybeUninit<windows_core::BSTR>, *mut core::mem::MaybeUninit<windows_core::VARIANT>, *mut core::mem::MaybeUninit<windows_core::VARIANT>, *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
    pub Name: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IAzOperation, IAzOperation_Vtbl, 0x5e56b24f_ea01_4d61_be44_c49b5e4eaf74);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IAzOperation {
    type Target = super::super::System::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IAzOperation, windows_core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl IAzOperation {
    pub unsafe fn Name(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Name)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetName<P0>(&self, bstrname: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).SetName)(windows_core::Interface::as_raw(self), bstrname.param().abi()).ok()
    }
    pub unsafe fn Description(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Description)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetDescription<P0>(&self, bstrdescription: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).SetDescription)(windows_core::Interface::as_raw(self), bstrdescription.param().abi()).ok()
    }
    pub unsafe fn ApplicationData(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).ApplicationData)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetApplicationData<P0>(&self, bstrapplicationdata: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).SetApplicationData)(windows_core::Interface::as_raw(self), bstrapplicationdata.param().abi()).ok()
    }
    pub unsafe fn OperationID(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).OperationID)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetOperationID(&self, lprop: i32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetOperationID)(windows_core::Interface::as_raw(self), lprop).ok()
    }
    pub unsafe fn Writable(&self) -> windows_core::Result<super::super::Foundation::BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Writable)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn GetProperty<P0>(&self, lpropid: i32, varreserved: P0) -> windows_core::Result<windows_core::VARIANT>
    where
        P0: windows_core::Param<windows_core::VARIANT>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetProperty)(windows_core::Interface::as_raw(self), lpropid, varreserved.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetProperty<P0, P1>(&self, lpropid: i32, varprop: P0, varreserved: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::VARIANT>,
        P1: windows_core::Param<windows_core::VARIANT>,
    {
        (windows_core::Interface::vtable(self).SetProperty)(windows_core::Interface::as_raw(self), lpropid, varprop.param().abi(), varreserved.param().abi()).ok()
    }
    pub unsafe fn Submit<P0>(&self, lflags: i32, varreserved: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::VARIANT>,
    {
        (windows_core::Interface::vtable(self).Submit)(windows_core::Interface::as_raw(self), lflags, varreserved.param().abi()).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IAzOperation_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub Name: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub SetName: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub Description: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub SetDescription: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub ApplicationData: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub SetApplicationData: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub OperationID: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetOperationID: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub Writable: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::BOOL) -> windows_core::HRESULT,
    pub GetProperty: unsafe extern "system" fn(*mut core::ffi::c_void, i32, core::mem::MaybeUninit<windows_core::VARIANT>, *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
    pub SetProperty: unsafe extern "system" fn(*mut core::ffi::c_void, i32, core::mem::MaybeUninit<windows_core::VARIANT>, core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
    pub Submit: unsafe extern "system" fn(*mut core::ffi::c_void, i32, core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IAzOperation2, IAzOperation2_Vtbl, 0x1f5ea01f_44a2_4184_9c48_a75b4dcc8ccc);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IAzOperation2 {
    type Target = IAzOperation;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IAzOperation2, windows_core::IUnknown, super::super::System::Com::IDispatch, IAzOperation);
#[cfg(feature = "Win32_System_Com")]
impl IAzOperation2 {
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn RoleAssignments<P0, P1>(&self, bstrscopename: P0, brecursive: P1) -> windows_core::Result<IAzRoleAssignments>
    where
        P0: windows_core::Param<windows_core::BSTR>,
        P1: windows_core::Param<super::super::Foundation::VARIANT_BOOL>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).RoleAssignments)(windows_core::Interface::as_raw(self), bstrscopename.param().abi(), brecursive.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IAzOperation2_Vtbl {
    pub base__: IAzOperation_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub RoleAssignments: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, super::super::Foundation::VARIANT_BOOL, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    RoleAssignments: usize,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IAzOperations, IAzOperations_Vtbl, 0x90ef9c07_9706_49d9_af80_0438a5f3ec35);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IAzOperations {
    type Target = super::super::System::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IAzOperations, windows_core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl IAzOperations {
    pub unsafe fn get_Item(&self, index: i32) -> windows_core::Result<windows_core::VARIANT> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).get_Item)(windows_core::Interface::as_raw(self), index, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn Count(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Count)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn _NewEnum(&self) -> windows_core::Result<windows_core::IUnknown> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self)._NewEnum)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IAzOperations_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub get_Item: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
    pub Count: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub _NewEnum: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IAzPrincipalLocator, IAzPrincipalLocator_Vtbl, 0xe5c3507d_ad6a_4992_9c7f_74ab480b44cc);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IAzPrincipalLocator {
    type Target = super::super::System::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IAzPrincipalLocator, windows_core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl IAzPrincipalLocator {
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn NameResolver(&self) -> windows_core::Result<IAzNameResolver> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).NameResolver)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn ObjectPicker(&self) -> windows_core::Result<IAzObjectPicker> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).ObjectPicker)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IAzPrincipalLocator_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub NameResolver: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    NameResolver: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub ObjectPicker: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    ObjectPicker: usize,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IAzRole, IAzRole_Vtbl, 0x859e0d8d_62d7_41d8_a034_c0cd5d43fdfa);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IAzRole {
    type Target = super::super::System::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IAzRole, windows_core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl IAzRole {
    pub unsafe fn Name(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Name)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetName<P0>(&self, bstrname: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).SetName)(windows_core::Interface::as_raw(self), bstrname.param().abi()).ok()
    }
    pub unsafe fn Description(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Description)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetDescription<P0>(&self, bstrdescription: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).SetDescription)(windows_core::Interface::as_raw(self), bstrdescription.param().abi()).ok()
    }
    pub unsafe fn ApplicationData(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).ApplicationData)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetApplicationData<P0>(&self, bstrapplicationdata: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).SetApplicationData)(windows_core::Interface::as_raw(self), bstrapplicationdata.param().abi()).ok()
    }
    pub unsafe fn AddAppMember<P0, P1>(&self, bstrprop: P0, varreserved: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
        P1: windows_core::Param<windows_core::VARIANT>,
    {
        (windows_core::Interface::vtable(self).AddAppMember)(windows_core::Interface::as_raw(self), bstrprop.param().abi(), varreserved.param().abi()).ok()
    }
    pub unsafe fn DeleteAppMember<P0, P1>(&self, bstrprop: P0, varreserved: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
        P1: windows_core::Param<windows_core::VARIANT>,
    {
        (windows_core::Interface::vtable(self).DeleteAppMember)(windows_core::Interface::as_raw(self), bstrprop.param().abi(), varreserved.param().abi()).ok()
    }
    pub unsafe fn AddTask<P0, P1>(&self, bstrprop: P0, varreserved: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
        P1: windows_core::Param<windows_core::VARIANT>,
    {
        (windows_core::Interface::vtable(self).AddTask)(windows_core::Interface::as_raw(self), bstrprop.param().abi(), varreserved.param().abi()).ok()
    }
    pub unsafe fn DeleteTask<P0, P1>(&self, bstrprop: P0, varreserved: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
        P1: windows_core::Param<windows_core::VARIANT>,
    {
        (windows_core::Interface::vtable(self).DeleteTask)(windows_core::Interface::as_raw(self), bstrprop.param().abi(), varreserved.param().abi()).ok()
    }
    pub unsafe fn AddOperation<P0, P1>(&self, bstrprop: P0, varreserved: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
        P1: windows_core::Param<windows_core::VARIANT>,
    {
        (windows_core::Interface::vtable(self).AddOperation)(windows_core::Interface::as_raw(self), bstrprop.param().abi(), varreserved.param().abi()).ok()
    }
    pub unsafe fn DeleteOperation<P0, P1>(&self, bstrprop: P0, varreserved: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
        P1: windows_core::Param<windows_core::VARIANT>,
    {
        (windows_core::Interface::vtable(self).DeleteOperation)(windows_core::Interface::as_raw(self), bstrprop.param().abi(), varreserved.param().abi()).ok()
    }
    pub unsafe fn AddMember<P0, P1>(&self, bstrprop: P0, varreserved: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
        P1: windows_core::Param<windows_core::VARIANT>,
    {
        (windows_core::Interface::vtable(self).AddMember)(windows_core::Interface::as_raw(self), bstrprop.param().abi(), varreserved.param().abi()).ok()
    }
    pub unsafe fn DeleteMember<P0, P1>(&self, bstrprop: P0, varreserved: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
        P1: windows_core::Param<windows_core::VARIANT>,
    {
        (windows_core::Interface::vtable(self).DeleteMember)(windows_core::Interface::as_raw(self), bstrprop.param().abi(), varreserved.param().abi()).ok()
    }
    pub unsafe fn Writable(&self) -> windows_core::Result<super::super::Foundation::BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Writable)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn GetProperty<P0>(&self, lpropid: i32, varreserved: P0) -> windows_core::Result<windows_core::VARIANT>
    where
        P0: windows_core::Param<windows_core::VARIANT>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetProperty)(windows_core::Interface::as_raw(self), lpropid, varreserved.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetProperty<P0, P1>(&self, lpropid: i32, varprop: P0, varreserved: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::VARIANT>,
        P1: windows_core::Param<windows_core::VARIANT>,
    {
        (windows_core::Interface::vtable(self).SetProperty)(windows_core::Interface::as_raw(self), lpropid, varprop.param().abi(), varreserved.param().abi()).ok()
    }
    pub unsafe fn AppMembers(&self) -> windows_core::Result<windows_core::VARIANT> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).AppMembers)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn Members(&self) -> windows_core::Result<windows_core::VARIANT> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Members)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn Operations(&self) -> windows_core::Result<windows_core::VARIANT> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Operations)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn Tasks(&self) -> windows_core::Result<windows_core::VARIANT> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Tasks)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn AddPropertyItem<P0, P1>(&self, lpropid: i32, varprop: P0, varreserved: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::VARIANT>,
        P1: windows_core::Param<windows_core::VARIANT>,
    {
        (windows_core::Interface::vtable(self).AddPropertyItem)(windows_core::Interface::as_raw(self), lpropid, varprop.param().abi(), varreserved.param().abi()).ok()
    }
    pub unsafe fn DeletePropertyItem<P0, P1>(&self, lpropid: i32, varprop: P0, varreserved: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::VARIANT>,
        P1: windows_core::Param<windows_core::VARIANT>,
    {
        (windows_core::Interface::vtable(self).DeletePropertyItem)(windows_core::Interface::as_raw(self), lpropid, varprop.param().abi(), varreserved.param().abi()).ok()
    }
    pub unsafe fn Submit<P0>(&self, lflags: i32, varreserved: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::VARIANT>,
    {
        (windows_core::Interface::vtable(self).Submit)(windows_core::Interface::as_raw(self), lflags, varreserved.param().abi()).ok()
    }
    pub unsafe fn AddMemberName<P0, P1>(&self, bstrprop: P0, varreserved: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
        P1: windows_core::Param<windows_core::VARIANT>,
    {
        (windows_core::Interface::vtable(self).AddMemberName)(windows_core::Interface::as_raw(self), bstrprop.param().abi(), varreserved.param().abi()).ok()
    }
    pub unsafe fn DeleteMemberName<P0, P1>(&self, bstrprop: P0, varreserved: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
        P1: windows_core::Param<windows_core::VARIANT>,
    {
        (windows_core::Interface::vtable(self).DeleteMemberName)(windows_core::Interface::as_raw(self), bstrprop.param().abi(), varreserved.param().abi()).ok()
    }
    pub unsafe fn MembersName(&self) -> windows_core::Result<windows_core::VARIANT> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).MembersName)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IAzRole_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub Name: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub SetName: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub Description: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub SetDescription: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub ApplicationData: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub SetApplicationData: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub AddAppMember: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
    pub DeleteAppMember: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
    pub AddTask: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
    pub DeleteTask: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
    pub AddOperation: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
    pub DeleteOperation: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
    pub AddMember: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
    pub DeleteMember: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
    pub Writable: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::BOOL) -> windows_core::HRESULT,
    pub GetProperty: unsafe extern "system" fn(*mut core::ffi::c_void, i32, core::mem::MaybeUninit<windows_core::VARIANT>, *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
    pub SetProperty: unsafe extern "system" fn(*mut core::ffi::c_void, i32, core::mem::MaybeUninit<windows_core::VARIANT>, core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
    pub AppMembers: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
    pub Members: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
    pub Operations: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
    pub Tasks: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
    pub AddPropertyItem: unsafe extern "system" fn(*mut core::ffi::c_void, i32, core::mem::MaybeUninit<windows_core::VARIANT>, core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
    pub DeletePropertyItem: unsafe extern "system" fn(*mut core::ffi::c_void, i32, core::mem::MaybeUninit<windows_core::VARIANT>, core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
    pub Submit: unsafe extern "system" fn(*mut core::ffi::c_void, i32, core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
    pub AddMemberName: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
    pub DeleteMemberName: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
    pub MembersName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IAzRoleAssignment, IAzRoleAssignment_Vtbl, 0x55647d31_0d5a_4fa3_b4ac_2b5f9ad5ab76);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IAzRoleAssignment {
    type Target = IAzRole;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IAzRoleAssignment, windows_core::IUnknown, super::super::System::Com::IDispatch, IAzRole);
#[cfg(feature = "Win32_System_Com")]
impl IAzRoleAssignment {
    pub unsafe fn AddRoleDefinition<P0>(&self, bstrroledefinition: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).AddRoleDefinition)(windows_core::Interface::as_raw(self), bstrroledefinition.param().abi()).ok()
    }
    pub unsafe fn DeleteRoleDefinition<P0>(&self, bstrroledefinition: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).DeleteRoleDefinition)(windows_core::Interface::as_raw(self), bstrroledefinition.param().abi()).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn RoleDefinitions(&self) -> windows_core::Result<IAzRoleDefinitions> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).RoleDefinitions)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Scope(&self) -> windows_core::Result<IAzScope> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Scope)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IAzRoleAssignment_Vtbl {
    pub base__: IAzRole_Vtbl,
    pub AddRoleDefinition: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub DeleteRoleDefinition: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub RoleDefinitions: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    RoleDefinitions: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub Scope: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Scope: usize,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IAzRoleAssignments, IAzRoleAssignments_Vtbl, 0x9c80b900_fceb_4d73_a0f4_c83b0bbf2481);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IAzRoleAssignments {
    type Target = super::super::System::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IAzRoleAssignments, windows_core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl IAzRoleAssignments {
    pub unsafe fn get_Item(&self, index: i32) -> windows_core::Result<windows_core::VARIANT> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).get_Item)(windows_core::Interface::as_raw(self), index, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn Count(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Count)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn _NewEnum(&self) -> windows_core::Result<windows_core::IUnknown> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self)._NewEnum)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IAzRoleAssignments_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub get_Item: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
    pub Count: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub _NewEnum: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IAzRoleDefinition, IAzRoleDefinition_Vtbl, 0xd97fcea1_2599_44f1_9fc3_58e9fbe09466);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IAzRoleDefinition {
    type Target = IAzTask;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IAzRoleDefinition, windows_core::IUnknown, super::super::System::Com::IDispatch, IAzTask);
#[cfg(feature = "Win32_System_Com")]
impl IAzRoleDefinition {
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn RoleAssignments<P0, P1>(&self, bstrscopename: P0, brecursive: P1) -> windows_core::Result<IAzRoleAssignments>
    where
        P0: windows_core::Param<windows_core::BSTR>,
        P1: windows_core::Param<super::super::Foundation::VARIANT_BOOL>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).RoleAssignments)(windows_core::Interface::as_raw(self), bstrscopename.param().abi(), brecursive.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn AddRoleDefinition<P0>(&self, bstrroledefinition: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).AddRoleDefinition)(windows_core::Interface::as_raw(self), bstrroledefinition.param().abi()).ok()
    }
    pub unsafe fn DeleteRoleDefinition<P0>(&self, bstrroledefinition: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).DeleteRoleDefinition)(windows_core::Interface::as_raw(self), bstrroledefinition.param().abi()).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn RoleDefinitions(&self) -> windows_core::Result<IAzRoleDefinitions> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).RoleDefinitions)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IAzRoleDefinition_Vtbl {
    pub base__: IAzTask_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub RoleAssignments: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, super::super::Foundation::VARIANT_BOOL, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    RoleAssignments: usize,
    pub AddRoleDefinition: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub DeleteRoleDefinition: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub RoleDefinitions: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    RoleDefinitions: usize,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IAzRoleDefinitions, IAzRoleDefinitions_Vtbl, 0x881f25a5_d755_4550_957a_d503a3b34001);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IAzRoleDefinitions {
    type Target = super::super::System::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IAzRoleDefinitions, windows_core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl IAzRoleDefinitions {
    pub unsafe fn get_Item(&self, index: i32) -> windows_core::Result<windows_core::VARIANT> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).get_Item)(windows_core::Interface::as_raw(self), index, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn Count(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Count)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn _NewEnum(&self) -> windows_core::Result<windows_core::IUnknown> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self)._NewEnum)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IAzRoleDefinitions_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub get_Item: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
    pub Count: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub _NewEnum: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IAzRoles, IAzRoles_Vtbl, 0x95e0f119_13b4_4dae_b65f_2f7d60d822e4);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IAzRoles {
    type Target = super::super::System::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IAzRoles, windows_core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl IAzRoles {
    pub unsafe fn get_Item(&self, index: i32) -> windows_core::Result<windows_core::VARIANT> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).get_Item)(windows_core::Interface::as_raw(self), index, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn Count(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Count)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn _NewEnum(&self) -> windows_core::Result<windows_core::IUnknown> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self)._NewEnum)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IAzRoles_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub get_Item: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
    pub Count: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub _NewEnum: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IAzScope, IAzScope_Vtbl, 0x00e52487_e08d_4514_b62e_877d5645f5ab);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IAzScope {
    type Target = super::super::System::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IAzScope, windows_core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl IAzScope {
    pub unsafe fn Name(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Name)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetName<P0>(&self, bstrname: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).SetName)(windows_core::Interface::as_raw(self), bstrname.param().abi()).ok()
    }
    pub unsafe fn Description(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Description)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetDescription<P0>(&self, bstrdescription: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).SetDescription)(windows_core::Interface::as_raw(self), bstrdescription.param().abi()).ok()
    }
    pub unsafe fn ApplicationData(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).ApplicationData)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetApplicationData<P0>(&self, bstrapplicationdata: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).SetApplicationData)(windows_core::Interface::as_raw(self), bstrapplicationdata.param().abi()).ok()
    }
    pub unsafe fn Writable(&self) -> windows_core::Result<super::super::Foundation::BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Writable)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn GetProperty<P0>(&self, lpropid: i32, varreserved: P0) -> windows_core::Result<windows_core::VARIANT>
    where
        P0: windows_core::Param<windows_core::VARIANT>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetProperty)(windows_core::Interface::as_raw(self), lpropid, varreserved.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetProperty<P0, P1>(&self, lpropid: i32, varprop: P0, varreserved: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::VARIANT>,
        P1: windows_core::Param<windows_core::VARIANT>,
    {
        (windows_core::Interface::vtable(self).SetProperty)(windows_core::Interface::as_raw(self), lpropid, varprop.param().abi(), varreserved.param().abi()).ok()
    }
    pub unsafe fn AddPropertyItem<P0, P1>(&self, lpropid: i32, varprop: P0, varreserved: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::VARIANT>,
        P1: windows_core::Param<windows_core::VARIANT>,
    {
        (windows_core::Interface::vtable(self).AddPropertyItem)(windows_core::Interface::as_raw(self), lpropid, varprop.param().abi(), varreserved.param().abi()).ok()
    }
    pub unsafe fn DeletePropertyItem<P0, P1>(&self, lpropid: i32, varprop: P0, varreserved: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::VARIANT>,
        P1: windows_core::Param<windows_core::VARIANT>,
    {
        (windows_core::Interface::vtable(self).DeletePropertyItem)(windows_core::Interface::as_raw(self), lpropid, varprop.param().abi(), varreserved.param().abi()).ok()
    }
    pub unsafe fn PolicyAdministrators(&self) -> windows_core::Result<windows_core::VARIANT> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).PolicyAdministrators)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn PolicyReaders(&self) -> windows_core::Result<windows_core::VARIANT> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).PolicyReaders)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn AddPolicyAdministrator<P0, P1>(&self, bstradmin: P0, varreserved: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
        P1: windows_core::Param<windows_core::VARIANT>,
    {
        (windows_core::Interface::vtable(self).AddPolicyAdministrator)(windows_core::Interface::as_raw(self), bstradmin.param().abi(), varreserved.param().abi()).ok()
    }
    pub unsafe fn DeletePolicyAdministrator<P0, P1>(&self, bstradmin: P0, varreserved: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
        P1: windows_core::Param<windows_core::VARIANT>,
    {
        (windows_core::Interface::vtable(self).DeletePolicyAdministrator)(windows_core::Interface::as_raw(self), bstradmin.param().abi(), varreserved.param().abi()).ok()
    }
    pub unsafe fn AddPolicyReader<P0, P1>(&self, bstrreader: P0, varreserved: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
        P1: windows_core::Param<windows_core::VARIANT>,
    {
        (windows_core::Interface::vtable(self).AddPolicyReader)(windows_core::Interface::as_raw(self), bstrreader.param().abi(), varreserved.param().abi()).ok()
    }
    pub unsafe fn DeletePolicyReader<P0, P1>(&self, bstrreader: P0, varreserved: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
        P1: windows_core::Param<windows_core::VARIANT>,
    {
        (windows_core::Interface::vtable(self).DeletePolicyReader)(windows_core::Interface::as_raw(self), bstrreader.param().abi(), varreserved.param().abi()).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn ApplicationGroups(&self) -> windows_core::Result<IAzApplicationGroups> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).ApplicationGroups)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn OpenApplicationGroup<P0, P1>(&self, bstrgroupname: P0, varreserved: P1) -> windows_core::Result<IAzApplicationGroup>
    where
        P0: windows_core::Param<windows_core::BSTR>,
        P1: windows_core::Param<windows_core::VARIANT>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).OpenApplicationGroup)(windows_core::Interface::as_raw(self), bstrgroupname.param().abi(), varreserved.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CreateApplicationGroup<P0, P1>(&self, bstrgroupname: P0, varreserved: P1) -> windows_core::Result<IAzApplicationGroup>
    where
        P0: windows_core::Param<windows_core::BSTR>,
        P1: windows_core::Param<windows_core::VARIANT>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CreateApplicationGroup)(windows_core::Interface::as_raw(self), bstrgroupname.param().abi(), varreserved.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn DeleteApplicationGroup<P0, P1>(&self, bstrgroupname: P0, varreserved: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
        P1: windows_core::Param<windows_core::VARIANT>,
    {
        (windows_core::Interface::vtable(self).DeleteApplicationGroup)(windows_core::Interface::as_raw(self), bstrgroupname.param().abi(), varreserved.param().abi()).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Roles(&self) -> windows_core::Result<IAzRoles> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Roles)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn OpenRole<P0, P1>(&self, bstrrolename: P0, varreserved: P1) -> windows_core::Result<IAzRole>
    where
        P0: windows_core::Param<windows_core::BSTR>,
        P1: windows_core::Param<windows_core::VARIANT>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).OpenRole)(windows_core::Interface::as_raw(self), bstrrolename.param().abi(), varreserved.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CreateRole<P0, P1>(&self, bstrrolename: P0, varreserved: P1) -> windows_core::Result<IAzRole>
    where
        P0: windows_core::Param<windows_core::BSTR>,
        P1: windows_core::Param<windows_core::VARIANT>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CreateRole)(windows_core::Interface::as_raw(self), bstrrolename.param().abi(), varreserved.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn DeleteRole<P0, P1>(&self, bstrrolename: P0, varreserved: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
        P1: windows_core::Param<windows_core::VARIANT>,
    {
        (windows_core::Interface::vtable(self).DeleteRole)(windows_core::Interface::as_raw(self), bstrrolename.param().abi(), varreserved.param().abi()).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Tasks(&self) -> windows_core::Result<IAzTasks> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Tasks)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn OpenTask<P0, P1>(&self, bstrtaskname: P0, varreserved: P1) -> windows_core::Result<IAzTask>
    where
        P0: windows_core::Param<windows_core::BSTR>,
        P1: windows_core::Param<windows_core::VARIANT>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).OpenTask)(windows_core::Interface::as_raw(self), bstrtaskname.param().abi(), varreserved.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CreateTask<P0, P1>(&self, bstrtaskname: P0, varreserved: P1) -> windows_core::Result<IAzTask>
    where
        P0: windows_core::Param<windows_core::BSTR>,
        P1: windows_core::Param<windows_core::VARIANT>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CreateTask)(windows_core::Interface::as_raw(self), bstrtaskname.param().abi(), varreserved.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn DeleteTask<P0, P1>(&self, bstrtaskname: P0, varreserved: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
        P1: windows_core::Param<windows_core::VARIANT>,
    {
        (windows_core::Interface::vtable(self).DeleteTask)(windows_core::Interface::as_raw(self), bstrtaskname.param().abi(), varreserved.param().abi()).ok()
    }
    pub unsafe fn Submit<P0>(&self, lflags: i32, varreserved: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::VARIANT>,
    {
        (windows_core::Interface::vtable(self).Submit)(windows_core::Interface::as_raw(self), lflags, varreserved.param().abi()).ok()
    }
    pub unsafe fn CanBeDelegated(&self) -> windows_core::Result<super::super::Foundation::BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CanBeDelegated)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn BizrulesWritable(&self) -> windows_core::Result<super::super::Foundation::BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).BizrulesWritable)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn PolicyAdministratorsName(&self) -> windows_core::Result<windows_core::VARIANT> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).PolicyAdministratorsName)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn PolicyReadersName(&self) -> windows_core::Result<windows_core::VARIANT> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).PolicyReadersName)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn AddPolicyAdministratorName<P0, P1>(&self, bstradmin: P0, varreserved: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
        P1: windows_core::Param<windows_core::VARIANT>,
    {
        (windows_core::Interface::vtable(self).AddPolicyAdministratorName)(windows_core::Interface::as_raw(self), bstradmin.param().abi(), varreserved.param().abi()).ok()
    }
    pub unsafe fn DeletePolicyAdministratorName<P0, P1>(&self, bstradmin: P0, varreserved: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
        P1: windows_core::Param<windows_core::VARIANT>,
    {
        (windows_core::Interface::vtable(self).DeletePolicyAdministratorName)(windows_core::Interface::as_raw(self), bstradmin.param().abi(), varreserved.param().abi()).ok()
    }
    pub unsafe fn AddPolicyReaderName<P0, P1>(&self, bstrreader: P0, varreserved: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
        P1: windows_core::Param<windows_core::VARIANT>,
    {
        (windows_core::Interface::vtable(self).AddPolicyReaderName)(windows_core::Interface::as_raw(self), bstrreader.param().abi(), varreserved.param().abi()).ok()
    }
    pub unsafe fn DeletePolicyReaderName<P0, P1>(&self, bstrreader: P0, varreserved: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
        P1: windows_core::Param<windows_core::VARIANT>,
    {
        (windows_core::Interface::vtable(self).DeletePolicyReaderName)(windows_core::Interface::as_raw(self), bstrreader.param().abi(), varreserved.param().abi()).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IAzScope_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub Name: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub SetName: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub Description: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub SetDescription: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub ApplicationData: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub SetApplicationData: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub Writable: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::BOOL) -> windows_core::HRESULT,
    pub GetProperty: unsafe extern "system" fn(*mut core::ffi::c_void, i32, core::mem::MaybeUninit<windows_core::VARIANT>, *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
    pub SetProperty: unsafe extern "system" fn(*mut core::ffi::c_void, i32, core::mem::MaybeUninit<windows_core::VARIANT>, core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
    pub AddPropertyItem: unsafe extern "system" fn(*mut core::ffi::c_void, i32, core::mem::MaybeUninit<windows_core::VARIANT>, core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
    pub DeletePropertyItem: unsafe extern "system" fn(*mut core::ffi::c_void, i32, core::mem::MaybeUninit<windows_core::VARIANT>, core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
    pub PolicyAdministrators: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
    pub PolicyReaders: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
    pub AddPolicyAdministrator: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
    pub DeletePolicyAdministrator: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
    pub AddPolicyReader: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
    pub DeletePolicyReader: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub ApplicationGroups: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    ApplicationGroups: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub OpenApplicationGroup: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, core::mem::MaybeUninit<windows_core::VARIANT>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    OpenApplicationGroup: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub CreateApplicationGroup: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, core::mem::MaybeUninit<windows_core::VARIANT>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    CreateApplicationGroup: usize,
    pub DeleteApplicationGroup: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub Roles: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Roles: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub OpenRole: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, core::mem::MaybeUninit<windows_core::VARIANT>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    OpenRole: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub CreateRole: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, core::mem::MaybeUninit<windows_core::VARIANT>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    CreateRole: usize,
    pub DeleteRole: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub Tasks: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Tasks: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub OpenTask: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, core::mem::MaybeUninit<windows_core::VARIANT>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    OpenTask: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub CreateTask: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, core::mem::MaybeUninit<windows_core::VARIANT>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    CreateTask: usize,
    pub DeleteTask: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
    pub Submit: unsafe extern "system" fn(*mut core::ffi::c_void, i32, core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
    pub CanBeDelegated: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::BOOL) -> windows_core::HRESULT,
    pub BizrulesWritable: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::BOOL) -> windows_core::HRESULT,
    pub PolicyAdministratorsName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
    pub PolicyReadersName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
    pub AddPolicyAdministratorName: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
    pub DeletePolicyAdministratorName: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
    pub AddPolicyReaderName: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
    pub DeletePolicyReaderName: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IAzScope2, IAzScope2_Vtbl, 0xee9fe8c9_c9f3_40e2_aa12_d1d8599727fd);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IAzScope2 {
    type Target = IAzScope;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IAzScope2, windows_core::IUnknown, super::super::System::Com::IDispatch, IAzScope);
#[cfg(feature = "Win32_System_Com")]
impl IAzScope2 {
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn RoleDefinitions(&self) -> windows_core::Result<IAzRoleDefinitions> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).RoleDefinitions)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CreateRoleDefinition<P0>(&self, bstrroledefinitionname: P0) -> windows_core::Result<IAzRoleDefinition>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CreateRoleDefinition)(windows_core::Interface::as_raw(self), bstrroledefinitionname.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn OpenRoleDefinition<P0>(&self, bstrroledefinitionname: P0) -> windows_core::Result<IAzRoleDefinition>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).OpenRoleDefinition)(windows_core::Interface::as_raw(self), bstrroledefinitionname.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn DeleteRoleDefinition<P0>(&self, bstrroledefinitionname: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).DeleteRoleDefinition)(windows_core::Interface::as_raw(self), bstrroledefinitionname.param().abi()).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn RoleAssignments(&self) -> windows_core::Result<IAzRoleAssignments> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).RoleAssignments)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CreateRoleAssignment<P0>(&self, bstrroleassignmentname: P0) -> windows_core::Result<IAzRoleAssignment>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CreateRoleAssignment)(windows_core::Interface::as_raw(self), bstrroleassignmentname.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn OpenRoleAssignment<P0>(&self, bstrroleassignmentname: P0) -> windows_core::Result<IAzRoleAssignment>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).OpenRoleAssignment)(windows_core::Interface::as_raw(self), bstrroleassignmentname.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn DeleteRoleAssignment<P0>(&self, bstrroleassignmentname: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).DeleteRoleAssignment)(windows_core::Interface::as_raw(self), bstrroleassignmentname.param().abi()).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IAzScope2_Vtbl {
    pub base__: IAzScope_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub RoleDefinitions: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    RoleDefinitions: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub CreateRoleDefinition: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    CreateRoleDefinition: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub OpenRoleDefinition: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    OpenRoleDefinition: usize,
    pub DeleteRoleDefinition: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub RoleAssignments: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    RoleAssignments: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub CreateRoleAssignment: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    CreateRoleAssignment: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub OpenRoleAssignment: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    OpenRoleAssignment: usize,
    pub DeleteRoleAssignment: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IAzScopes, IAzScopes_Vtbl, 0x78e14853_9f5e_406d_9b91_6bdba6973510);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IAzScopes {
    type Target = super::super::System::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IAzScopes, windows_core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl IAzScopes {
    pub unsafe fn get_Item(&self, index: i32) -> windows_core::Result<windows_core::VARIANT> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).get_Item)(windows_core::Interface::as_raw(self), index, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn Count(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Count)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn _NewEnum(&self) -> windows_core::Result<windows_core::IUnknown> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self)._NewEnum)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IAzScopes_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub get_Item: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
    pub Count: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub _NewEnum: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IAzTask, IAzTask_Vtbl, 0xcb94e592_2e0e_4a6c_a336_b89a6dc1e388);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IAzTask {
    type Target = super::super::System::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IAzTask, windows_core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl IAzTask {
    pub unsafe fn Name(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Name)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetName<P0>(&self, bstrname: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).SetName)(windows_core::Interface::as_raw(self), bstrname.param().abi()).ok()
    }
    pub unsafe fn Description(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Description)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetDescription<P0>(&self, bstrdescription: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).SetDescription)(windows_core::Interface::as_raw(self), bstrdescription.param().abi()).ok()
    }
    pub unsafe fn ApplicationData(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).ApplicationData)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetApplicationData<P0>(&self, bstrapplicationdata: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).SetApplicationData)(windows_core::Interface::as_raw(self), bstrapplicationdata.param().abi()).ok()
    }
    pub unsafe fn BizRule(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).BizRule)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetBizRule<P0>(&self, bstrprop: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).SetBizRule)(windows_core::Interface::as_raw(self), bstrprop.param().abi()).ok()
    }
    pub unsafe fn BizRuleLanguage(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).BizRuleLanguage)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetBizRuleLanguage<P0>(&self, bstrprop: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).SetBizRuleLanguage)(windows_core::Interface::as_raw(self), bstrprop.param().abi()).ok()
    }
    pub unsafe fn BizRuleImportedPath(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).BizRuleImportedPath)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetBizRuleImportedPath<P0>(&self, bstrprop: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).SetBizRuleImportedPath)(windows_core::Interface::as_raw(self), bstrprop.param().abi()).ok()
    }
    pub unsafe fn IsRoleDefinition(&self) -> windows_core::Result<super::super::Foundation::BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).IsRoleDefinition)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetIsRoleDefinition<P0>(&self, fprop: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::BOOL>,
    {
        (windows_core::Interface::vtable(self).SetIsRoleDefinition)(windows_core::Interface::as_raw(self), fprop.param().abi()).ok()
    }
    pub unsafe fn Operations(&self) -> windows_core::Result<windows_core::VARIANT> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Operations)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn Tasks(&self) -> windows_core::Result<windows_core::VARIANT> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Tasks)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn AddOperation<P0, P1>(&self, bstrop: P0, varreserved: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
        P1: windows_core::Param<windows_core::VARIANT>,
    {
        (windows_core::Interface::vtable(self).AddOperation)(windows_core::Interface::as_raw(self), bstrop.param().abi(), varreserved.param().abi()).ok()
    }
    pub unsafe fn DeleteOperation<P0, P1>(&self, bstrop: P0, varreserved: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
        P1: windows_core::Param<windows_core::VARIANT>,
    {
        (windows_core::Interface::vtable(self).DeleteOperation)(windows_core::Interface::as_raw(self), bstrop.param().abi(), varreserved.param().abi()).ok()
    }
    pub unsafe fn AddTask<P0, P1>(&self, bstrtask: P0, varreserved: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
        P1: windows_core::Param<windows_core::VARIANT>,
    {
        (windows_core::Interface::vtable(self).AddTask)(windows_core::Interface::as_raw(self), bstrtask.param().abi(), varreserved.param().abi()).ok()
    }
    pub unsafe fn DeleteTask<P0, P1>(&self, bstrtask: P0, varreserved: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
        P1: windows_core::Param<windows_core::VARIANT>,
    {
        (windows_core::Interface::vtable(self).DeleteTask)(windows_core::Interface::as_raw(self), bstrtask.param().abi(), varreserved.param().abi()).ok()
    }
    pub unsafe fn Writable(&self) -> windows_core::Result<super::super::Foundation::BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Writable)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn GetProperty<P0>(&self, lpropid: i32, varreserved: P0) -> windows_core::Result<windows_core::VARIANT>
    where
        P0: windows_core::Param<windows_core::VARIANT>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetProperty)(windows_core::Interface::as_raw(self), lpropid, varreserved.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetProperty<P0, P1>(&self, lpropid: i32, varprop: P0, varreserved: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::VARIANT>,
        P1: windows_core::Param<windows_core::VARIANT>,
    {
        (windows_core::Interface::vtable(self).SetProperty)(windows_core::Interface::as_raw(self), lpropid, varprop.param().abi(), varreserved.param().abi()).ok()
    }
    pub unsafe fn AddPropertyItem<P0, P1>(&self, lpropid: i32, varprop: P0, varreserved: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::VARIANT>,
        P1: windows_core::Param<windows_core::VARIANT>,
    {
        (windows_core::Interface::vtable(self).AddPropertyItem)(windows_core::Interface::as_raw(self), lpropid, varprop.param().abi(), varreserved.param().abi()).ok()
    }
    pub unsafe fn DeletePropertyItem<P0, P1>(&self, lpropid: i32, varprop: P0, varreserved: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::VARIANT>,
        P1: windows_core::Param<windows_core::VARIANT>,
    {
        (windows_core::Interface::vtable(self).DeletePropertyItem)(windows_core::Interface::as_raw(self), lpropid, varprop.param().abi(), varreserved.param().abi()).ok()
    }
    pub unsafe fn Submit<P0>(&self, lflags: i32, varreserved: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::VARIANT>,
    {
        (windows_core::Interface::vtable(self).Submit)(windows_core::Interface::as_raw(self), lflags, varreserved.param().abi()).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IAzTask_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub Name: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub SetName: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub Description: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub SetDescription: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub ApplicationData: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub SetApplicationData: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub BizRule: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub SetBizRule: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub BizRuleLanguage: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub SetBizRuleLanguage: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub BizRuleImportedPath: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub SetBizRuleImportedPath: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub IsRoleDefinition: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::BOOL) -> windows_core::HRESULT,
    pub SetIsRoleDefinition: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::BOOL) -> windows_core::HRESULT,
    pub Operations: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
    pub Tasks: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
    pub AddOperation: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
    pub DeleteOperation: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
    pub AddTask: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
    pub DeleteTask: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
    pub Writable: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::BOOL) -> windows_core::HRESULT,
    pub GetProperty: unsafe extern "system" fn(*mut core::ffi::c_void, i32, core::mem::MaybeUninit<windows_core::VARIANT>, *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
    pub SetProperty: unsafe extern "system" fn(*mut core::ffi::c_void, i32, core::mem::MaybeUninit<windows_core::VARIANT>, core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
    pub AddPropertyItem: unsafe extern "system" fn(*mut core::ffi::c_void, i32, core::mem::MaybeUninit<windows_core::VARIANT>, core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
    pub DeletePropertyItem: unsafe extern "system" fn(*mut core::ffi::c_void, i32, core::mem::MaybeUninit<windows_core::VARIANT>, core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
    pub Submit: unsafe extern "system" fn(*mut core::ffi::c_void, i32, core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IAzTask2, IAzTask2_Vtbl, 0x03a9a5ee_48c8_4832_9025_aad503c46526);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IAzTask2 {
    type Target = IAzTask;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IAzTask2, windows_core::IUnknown, super::super::System::Com::IDispatch, IAzTask);
#[cfg(feature = "Win32_System_Com")]
impl IAzTask2 {
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn RoleAssignments<P0, P1>(&self, bstrscopename: P0, brecursive: P1) -> windows_core::Result<IAzRoleAssignments>
    where
        P0: windows_core::Param<windows_core::BSTR>,
        P1: windows_core::Param<super::super::Foundation::VARIANT_BOOL>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).RoleAssignments)(windows_core::Interface::as_raw(self), bstrscopename.param().abi(), brecursive.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IAzTask2_Vtbl {
    pub base__: IAzTask_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub RoleAssignments: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, super::super::Foundation::VARIANT_BOOL, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    RoleAssignments: usize,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IAzTasks, IAzTasks_Vtbl, 0xb338ccab_4c85_4388_8c0a_c58592bad398);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IAzTasks {
    type Target = super::super::System::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IAzTasks, windows_core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl IAzTasks {
    pub unsafe fn get_Item(&self, index: i32) -> windows_core::Result<windows_core::VARIANT> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).get_Item)(windows_core::Interface::as_raw(self), index, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn Count(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Count)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn _NewEnum(&self) -> windows_core::Result<windows_core::IUnknown> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self)._NewEnum)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IAzTasks_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub get_Item: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
    pub Count: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub _NewEnum: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub const ACCCTRL_DEFAULT_PROVIDER: windows_core::PCWSTR = windows_core::w!("Windows NT Access Provider");
pub const ACCCTRL_DEFAULT_PROVIDERA: windows_core::PCSTR = windows_core::s!("Windows NT Access Provider");
pub const ACCCTRL_DEFAULT_PROVIDERW: windows_core::PCWSTR = windows_core::w!("Windows NT Access Provider");
pub const ACTRL_ACCESS_ALLOWED: ACTRL_ACCESS_ENTRY_ACCESS_FLAGS = ACTRL_ACCESS_ENTRY_ACCESS_FLAGS(1u32);
pub const ACTRL_ACCESS_DENIED: ACTRL_ACCESS_ENTRY_ACCESS_FLAGS = ACTRL_ACCESS_ENTRY_ACCESS_FLAGS(2u32);
pub const ACTRL_ACCESS_NO_OPTIONS: u32 = 0u32;
pub const ACTRL_ACCESS_PROTECTED: u32 = 1u32;
pub const ACTRL_ACCESS_SUPPORTS_OBJECT_ENTRIES: u32 = 1u32;
pub const ACTRL_AUDIT_FAILURE: ACTRL_ACCESS_ENTRY_ACCESS_FLAGS = ACTRL_ACCESS_ENTRY_ACCESS_FLAGS(8u32);
pub const ACTRL_AUDIT_SUCCESS: ACTRL_ACCESS_ENTRY_ACCESS_FLAGS = ACTRL_ACCESS_ENTRY_ACCESS_FLAGS(4u32);
pub const ACTRL_CHANGE_ACCESS: u32 = 536870912u32;
pub const ACTRL_CHANGE_OWNER: u32 = 1073741824u32;
pub const ACTRL_DELETE: u32 = 134217728u32;
pub const ACTRL_DIR_CREATE_CHILD: u32 = 4u32;
pub const ACTRL_DIR_CREATE_OBJECT: u32 = 2u32;
pub const ACTRL_DIR_DELETE_CHILD: u32 = 64u32;
pub const ACTRL_DIR_LIST: u32 = 1u32;
pub const ACTRL_DIR_TRAVERSE: u32 = 32u32;
pub const ACTRL_FILE_APPEND: u32 = 4u32;
pub const ACTRL_FILE_CREATE_PIPE: u32 = 512u32;
pub const ACTRL_FILE_EXECUTE: u32 = 32u32;
pub const ACTRL_FILE_READ: u32 = 1u32;
pub const ACTRL_FILE_READ_ATTRIB: u32 = 128u32;
pub const ACTRL_FILE_READ_PROP: u32 = 8u32;
pub const ACTRL_FILE_WRITE: u32 = 2u32;
pub const ACTRL_FILE_WRITE_ATTRIB: u32 = 256u32;
pub const ACTRL_FILE_WRITE_PROP: u32 = 16u32;
pub const ACTRL_KERNEL_ALERT: u32 = 1024u32;
pub const ACTRL_KERNEL_CONTROL: u32 = 512u32;
pub const ACTRL_KERNEL_DIMPERSONATE: u32 = 32768u32;
pub const ACTRL_KERNEL_DUP_HANDLE: u32 = 32u32;
pub const ACTRL_KERNEL_GET_CONTEXT: u32 = 2048u32;
pub const ACTRL_KERNEL_GET_INFO: u32 = 256u32;
pub const ACTRL_KERNEL_IMPERSONATE: u32 = 16384u32;
pub const ACTRL_KERNEL_PROCESS: u32 = 64u32;
pub const ACTRL_KERNEL_SET_CONTEXT: u32 = 4096u32;
pub const ACTRL_KERNEL_SET_INFO: u32 = 128u32;
pub const ACTRL_KERNEL_TERMINATE: u32 = 1u32;
pub const ACTRL_KERNEL_THREAD: u32 = 2u32;
pub const ACTRL_KERNEL_TOKEN: u32 = 8192u32;
pub const ACTRL_KERNEL_VM: u32 = 4u32;
pub const ACTRL_KERNEL_VM_READ: u32 = 8u32;
pub const ACTRL_KERNEL_VM_WRITE: u32 = 16u32;
pub const ACTRL_PERM_1: u32 = 1u32;
pub const ACTRL_PERM_10: u32 = 512u32;
pub const ACTRL_PERM_11: u32 = 1024u32;
pub const ACTRL_PERM_12: u32 = 2048u32;
pub const ACTRL_PERM_13: u32 = 4096u32;
pub const ACTRL_PERM_14: u32 = 8192u32;
pub const ACTRL_PERM_15: u32 = 16384u32;
pub const ACTRL_PERM_16: u32 = 32768u32;
pub const ACTRL_PERM_17: u32 = 65536u32;
pub const ACTRL_PERM_18: u32 = 131072u32;
pub const ACTRL_PERM_19: u32 = 262144u32;
pub const ACTRL_PERM_2: u32 = 2u32;
pub const ACTRL_PERM_20: u32 = 524288u32;
pub const ACTRL_PERM_3: u32 = 4u32;
pub const ACTRL_PERM_4: u32 = 8u32;
pub const ACTRL_PERM_5: u32 = 16u32;
pub const ACTRL_PERM_6: u32 = 32u32;
pub const ACTRL_PERM_7: u32 = 64u32;
pub const ACTRL_PERM_8: u32 = 128u32;
pub const ACTRL_PERM_9: u32 = 256u32;
pub const ACTRL_PRINT_JADMIN: u32 = 16u32;
pub const ACTRL_PRINT_PADMIN: u32 = 4u32;
pub const ACTRL_PRINT_PUSE: u32 = 8u32;
pub const ACTRL_PRINT_SADMIN: u32 = 1u32;
pub const ACTRL_PRINT_SLIST: u32 = 2u32;
pub const ACTRL_READ_CONTROL: u32 = 268435456u32;
pub const ACTRL_REG_CREATE_CHILD: u32 = 4u32;
pub const ACTRL_REG_LINK: u32 = 32u32;
pub const ACTRL_REG_LIST: u32 = 8u32;
pub const ACTRL_REG_NOTIFY: u32 = 16u32;
pub const ACTRL_REG_QUERY: u32 = 1u32;
pub const ACTRL_REG_SET: u32 = 2u32;
pub const ACTRL_RESERVED: u32 = 0u32;
pub const ACTRL_STD_RIGHTS_ALL: u32 = 4160749568u32;
pub const ACTRL_SVC_GET_INFO: u32 = 1u32;
pub const ACTRL_SVC_INTERROGATE: u32 = 128u32;
pub const ACTRL_SVC_LIST: u32 = 8u32;
pub const ACTRL_SVC_PAUSE: u32 = 64u32;
pub const ACTRL_SVC_SET_INFO: u32 = 2u32;
pub const ACTRL_SVC_START: u32 = 16u32;
pub const ACTRL_SVC_STATUS: u32 = 4u32;
pub const ACTRL_SVC_STOP: u32 = 32u32;
pub const ACTRL_SVC_UCONTROL: u32 = 256u32;
pub const ACTRL_SYNCHRONIZE: u32 = 2147483648u32;
pub const ACTRL_SYSTEM_ACCESS: u32 = 67108864u32;
pub const ACTRL_WIN_CLIPBRD: u32 = 1u32;
pub const ACTRL_WIN_CREATE: u32 = 4u32;
pub const ACTRL_WIN_EXIT: u32 = 256u32;
pub const ACTRL_WIN_GLOBAL_ATOMS: u32 = 2u32;
pub const ACTRL_WIN_LIST: u32 = 16u32;
pub const ACTRL_WIN_LIST_DESK: u32 = 8u32;
pub const ACTRL_WIN_READ_ATTRIBS: u32 = 32u32;
pub const ACTRL_WIN_SCREEN: u32 = 128u32;
pub const ACTRL_WIN_WRITE_ATTRIBS: u32 = 64u32;
pub const APF_AuditFailure: u32 = 0u32;
pub const APF_AuditSuccess: u32 = 1u32;
pub const APF_ValidFlags: u32 = 1u32;
pub const APT_Guid: AUDIT_PARAM_TYPE = AUDIT_PARAM_TYPE(9i32);
pub const APT_Int64: AUDIT_PARAM_TYPE = AUDIT_PARAM_TYPE(11i32);
pub const APT_IpAddress: AUDIT_PARAM_TYPE = AUDIT_PARAM_TYPE(12i32);
pub const APT_LogonId: AUDIT_PARAM_TYPE = AUDIT_PARAM_TYPE(6i32);
pub const APT_LogonIdWithSid: AUDIT_PARAM_TYPE = AUDIT_PARAM_TYPE(13i32);
pub const APT_Luid: AUDIT_PARAM_TYPE = AUDIT_PARAM_TYPE(8i32);
pub const APT_None: AUDIT_PARAM_TYPE = AUDIT_PARAM_TYPE(1i32);
pub const APT_ObjectTypeList: AUDIT_PARAM_TYPE = AUDIT_PARAM_TYPE(7i32);
pub const APT_Pointer: AUDIT_PARAM_TYPE = AUDIT_PARAM_TYPE(4i32);
pub const APT_Sid: AUDIT_PARAM_TYPE = AUDIT_PARAM_TYPE(5i32);
pub const APT_String: AUDIT_PARAM_TYPE = AUDIT_PARAM_TYPE(2i32);
pub const APT_Time: AUDIT_PARAM_TYPE = AUDIT_PARAM_TYPE(10i32);
pub const APT_Ulong: AUDIT_PARAM_TYPE = AUDIT_PARAM_TYPE(3i32);
pub const AP_ParamTypeBits: u32 = 8u32;
pub const AP_ParamTypeMask: i32 = 255i32;
pub const AUDIT_TYPE_LEGACY: u32 = 1u32;
pub const AUDIT_TYPE_WMI: u32 = 2u32;
pub const AUTHZP_WPD_EVENT: u32 = 16u32;
pub const AUTHZ_ACCESS_CHECK_NO_DEEP_COPY_SD: AUTHZ_ACCESS_CHECK_FLAGS = AUTHZ_ACCESS_CHECK_FLAGS(1u32);
pub const AUTHZ_ALLOW_MULTIPLE_SOURCE_INSTANCES: u32 = 1u32;
pub const AUTHZ_AUDIT_INSTANCE_INFORMATION: u32 = 2u32;
pub const AUTHZ_COMPUTE_PRIVILEGES: u32 = 8u32;
pub const AUTHZ_FLAG_ALLOW_MULTIPLE_SOURCE_INSTANCES: u32 = 1u32;
pub const AUTHZ_GENERATE_FAILURE_AUDIT: AUTHZ_GENERATE_RESULTS = AUTHZ_GENERATE_RESULTS(2u32);
pub const AUTHZ_GENERATE_SUCCESS_AUDIT: AUTHZ_GENERATE_RESULTS = AUTHZ_GENERATE_RESULTS(1u32);
pub const AUTHZ_INIT_INFO_VERSION_V1: u32 = 1u32;
pub const AUTHZ_MIGRATED_LEGACY_PUBLISHER: u32 = 2u32;
pub const AUTHZ_NO_ALLOC_STRINGS: AUTHZ_INITIALIZE_OBJECT_ACCESS_AUDIT_EVENT_FLAGS = AUTHZ_INITIALIZE_OBJECT_ACCESS_AUDIT_EVENT_FLAGS(4u32);
pub const AUTHZ_NO_FAILURE_AUDIT: AUTHZ_INITIALIZE_OBJECT_ACCESS_AUDIT_EVENT_FLAGS = AUTHZ_INITIALIZE_OBJECT_ACCESS_AUDIT_EVENT_FLAGS(2u32);
pub const AUTHZ_NO_SUCCESS_AUDIT: AUTHZ_INITIALIZE_OBJECT_ACCESS_AUDIT_EVENT_FLAGS = AUTHZ_INITIALIZE_OBJECT_ACCESS_AUDIT_EVENT_FLAGS(1u32);
pub const AUTHZ_REQUIRE_S4U_LOGON: u32 = 4u32;
pub const AUTHZ_RM_FLAG_INITIALIZE_UNDER_IMPERSONATION: AUTHZ_RESOURCE_MANAGER_FLAGS = AUTHZ_RESOURCE_MANAGER_FLAGS(2u32);
pub const AUTHZ_RM_FLAG_NO_AUDIT: AUTHZ_RESOURCE_MANAGER_FLAGS = AUTHZ_RESOURCE_MANAGER_FLAGS(1u32);
pub const AUTHZ_RM_FLAG_NO_CENTRAL_ACCESS_POLICIES: AUTHZ_RESOURCE_MANAGER_FLAGS = AUTHZ_RESOURCE_MANAGER_FLAGS(4u32);
pub const AUTHZ_RPC_INIT_INFO_CLIENT_VERSION_V1: u32 = 1u32;
pub const AUTHZ_SECURITY_ATTRIBUTES_INFORMATION_VERSION: u32 = 1u32;
pub const AUTHZ_SECURITY_ATTRIBUTES_INFORMATION_VERSION_V1: u32 = 1u32;
pub const AUTHZ_SECURITY_ATTRIBUTE_NON_INHERITABLE: AUTHZ_SECURITY_ATTRIBUTE_FLAGS = AUTHZ_SECURITY_ATTRIBUTE_FLAGS(1u32);
pub const AUTHZ_SECURITY_ATTRIBUTE_OPERATION_ADD: AUTHZ_SECURITY_ATTRIBUTE_OPERATION = AUTHZ_SECURITY_ATTRIBUTE_OPERATION(2i32);
pub const AUTHZ_SECURITY_ATTRIBUTE_OPERATION_DELETE: AUTHZ_SECURITY_ATTRIBUTE_OPERATION = AUTHZ_SECURITY_ATTRIBUTE_OPERATION(3i32);
pub const AUTHZ_SECURITY_ATTRIBUTE_OPERATION_NONE: AUTHZ_SECURITY_ATTRIBUTE_OPERATION = AUTHZ_SECURITY_ATTRIBUTE_OPERATION(0i32);
pub const AUTHZ_SECURITY_ATTRIBUTE_OPERATION_REPLACE: AUTHZ_SECURITY_ATTRIBUTE_OPERATION = AUTHZ_SECURITY_ATTRIBUTE_OPERATION(4i32);
pub const AUTHZ_SECURITY_ATTRIBUTE_OPERATION_REPLACE_ALL: AUTHZ_SECURITY_ATTRIBUTE_OPERATION = AUTHZ_SECURITY_ATTRIBUTE_OPERATION(1i32);
pub const AUTHZ_SECURITY_ATTRIBUTE_TYPE_BOOLEAN: u32 = 6u32;
pub const AUTHZ_SECURITY_ATTRIBUTE_TYPE_FQBN: u32 = 4u32;
pub const AUTHZ_SECURITY_ATTRIBUTE_TYPE_INT64: u32 = 1u32;
pub const AUTHZ_SECURITY_ATTRIBUTE_TYPE_INVALID: u32 = 0u32;
pub const AUTHZ_SECURITY_ATTRIBUTE_TYPE_OCTET_STRING: u32 = 16u32;
pub const AUTHZ_SECURITY_ATTRIBUTE_TYPE_SID: u32 = 5u32;
pub const AUTHZ_SECURITY_ATTRIBUTE_TYPE_STRING: u32 = 3u32;
pub const AUTHZ_SECURITY_ATTRIBUTE_TYPE_UINT64: u32 = 2u32;
pub const AUTHZ_SECURITY_ATTRIBUTE_VALUE_CASE_SENSITIVE: AUTHZ_SECURITY_ATTRIBUTE_FLAGS = AUTHZ_SECURITY_ATTRIBUTE_FLAGS(2u32);
pub const AUTHZ_SID_OPERATION_ADD: AUTHZ_SID_OPERATION = AUTHZ_SID_OPERATION(2i32);
pub const AUTHZ_SID_OPERATION_DELETE: AUTHZ_SID_OPERATION = AUTHZ_SID_OPERATION(3i32);
pub const AUTHZ_SID_OPERATION_NONE: AUTHZ_SID_OPERATION = AUTHZ_SID_OPERATION(0i32);
pub const AUTHZ_SID_OPERATION_REPLACE: AUTHZ_SID_OPERATION = AUTHZ_SID_OPERATION(4i32);
pub const AUTHZ_SID_OPERATION_REPLACE_ALL: AUTHZ_SID_OPERATION = AUTHZ_SID_OPERATION(1i32);
pub const AUTHZ_SKIP_TOKEN_GROUPS: u32 = 2u32;
pub const AUTHZ_WPD_CATEGORY_FLAG: u32 = 16u32;
pub const AZ_AZSTORE_DEFAULT_DOMAIN_TIMEOUT: AZ_PROP_CONSTANTS = AZ_PROP_CONSTANTS(15000i32);
pub const AZ_AZSTORE_DEFAULT_MAX_SCRIPT_ENGINES: AZ_PROP_CONSTANTS = AZ_PROP_CONSTANTS(120i32);
pub const AZ_AZSTORE_DEFAULT_SCRIPT_ENGINE_TIMEOUT: AZ_PROP_CONSTANTS = AZ_PROP_CONSTANTS(45000i32);
pub const AZ_AZSTORE_FLAG_AUDIT_IS_CRITICAL: AZ_PROP_CONSTANTS = AZ_PROP_CONSTANTS(8i32);
pub const AZ_AZSTORE_FLAG_BATCH_UPDATE: AZ_PROP_CONSTANTS = AZ_PROP_CONSTANTS(4i32);
pub const AZ_AZSTORE_FLAG_CREATE: AZ_PROP_CONSTANTS = AZ_PROP_CONSTANTS(1i32);
pub const AZ_AZSTORE_FLAG_MANAGE_ONLY_PASSIVE_SUBMIT: AZ_PROP_CONSTANTS = AZ_PROP_CONSTANTS(32768i32);
pub const AZ_AZSTORE_FLAG_MANAGE_STORE_ONLY: AZ_PROP_CONSTANTS = AZ_PROP_CONSTANTS(2i32);
pub const AZ_AZSTORE_FORCE_APPLICATION_CLOSE: AZ_PROP_CONSTANTS = AZ_PROP_CONSTANTS(16i32);
pub const AZ_AZSTORE_MIN_DOMAIN_TIMEOUT: AZ_PROP_CONSTANTS = AZ_PROP_CONSTANTS(500i32);
pub const AZ_AZSTORE_MIN_SCRIPT_ENGINE_TIMEOUT: AZ_PROP_CONSTANTS = AZ_PROP_CONSTANTS(5000i32);
pub const AZ_AZSTORE_NT6_FUNCTION_LEVEL: AZ_PROP_CONSTANTS = AZ_PROP_CONSTANTS(32i32);
pub const AZ_CLIENT_CONTEXT_GET_GROUPS_STORE_LEVEL_ONLY: AZ_PROP_CONSTANTS = AZ_PROP_CONSTANTS(2i32);
pub const AZ_CLIENT_CONTEXT_GET_GROUP_RECURSIVE: AZ_PROP_CONSTANTS = AZ_PROP_CONSTANTS(2i32);
pub const AZ_CLIENT_CONTEXT_SKIP_GROUP: AZ_PROP_CONSTANTS = AZ_PROP_CONSTANTS(1i32);
pub const AZ_CLIENT_CONTEXT_SKIP_LDAP_QUERY: AZ_PROP_CONSTANTS = AZ_PROP_CONSTANTS(1i32);
pub const AZ_GROUPTYPE_BASIC: AZ_PROP_CONSTANTS = AZ_PROP_CONSTANTS(2i32);
pub const AZ_GROUPTYPE_BIZRULE: AZ_PROP_CONSTANTS = AZ_PROP_CONSTANTS(3i32);
pub const AZ_GROUPTYPE_LDAP_QUERY: AZ_PROP_CONSTANTS = AZ_PROP_CONSTANTS(1i32);
pub const AZ_MAX_APPLICATION_DATA_LENGTH: AZ_PROP_CONSTANTS = AZ_PROP_CONSTANTS(4096i32);
pub const AZ_MAX_APPLICATION_NAME_LENGTH: AZ_PROP_CONSTANTS = AZ_PROP_CONSTANTS(512i32);
pub const AZ_MAX_APPLICATION_VERSION_LENGTH: AZ_PROP_CONSTANTS = AZ_PROP_CONSTANTS(512i32);
pub const AZ_MAX_BIZRULE_STRING: AZ_PROP_CONSTANTS = AZ_PROP_CONSTANTS(65536i32);
pub const AZ_MAX_DESCRIPTION_LENGTH: AZ_PROP_CONSTANTS = AZ_PROP_CONSTANTS(1024i32);
pub const AZ_MAX_GROUP_BIZRULE_IMPORTED_PATH_LENGTH: AZ_PROP_CONSTANTS = AZ_PROP_CONSTANTS(512i32);
pub const AZ_MAX_GROUP_BIZRULE_LANGUAGE_LENGTH: AZ_PROP_CONSTANTS = AZ_PROP_CONSTANTS(64i32);
pub const AZ_MAX_GROUP_BIZRULE_LENGTH: AZ_PROP_CONSTANTS = AZ_PROP_CONSTANTS(65536i32);
pub const AZ_MAX_GROUP_LDAP_QUERY_LENGTH: AZ_PROP_CONSTANTS = AZ_PROP_CONSTANTS(4096i32);
pub const AZ_MAX_GROUP_NAME_LENGTH: AZ_PROP_CONSTANTS = AZ_PROP_CONSTANTS(64i32);
pub const AZ_MAX_NAME_LENGTH: AZ_PROP_CONSTANTS = AZ_PROP_CONSTANTS(65536i32);
pub const AZ_MAX_OPERATION_NAME_LENGTH: AZ_PROP_CONSTANTS = AZ_PROP_CONSTANTS(64i32);
pub const AZ_MAX_POLICY_URL_LENGTH: AZ_PROP_CONSTANTS = AZ_PROP_CONSTANTS(65536i32);
pub const AZ_MAX_ROLE_NAME_LENGTH: AZ_PROP_CONSTANTS = AZ_PROP_CONSTANTS(64i32);
pub const AZ_MAX_SCOPE_NAME_LENGTH: AZ_PROP_CONSTANTS = AZ_PROP_CONSTANTS(65536i32);
pub const AZ_MAX_TASK_BIZRULE_IMPORTED_PATH_LENGTH: AZ_PROP_CONSTANTS = AZ_PROP_CONSTANTS(512i32);
pub const AZ_MAX_TASK_BIZRULE_LANGUAGE_LENGTH: AZ_PROP_CONSTANTS = AZ_PROP_CONSTANTS(64i32);
pub const AZ_MAX_TASK_BIZRULE_LENGTH: AZ_PROP_CONSTANTS = AZ_PROP_CONSTANTS(65536i32);
pub const AZ_MAX_TASK_NAME_LENGTH: AZ_PROP_CONSTANTS = AZ_PROP_CONSTANTS(64i32);
pub const AZ_PROP_APPLICATION_AUTHZ_INTERFACE_CLSID: AZ_PROP_CONSTANTS = AZ_PROP_CONSTANTS(800i32);
pub const AZ_PROP_APPLICATION_BIZRULE_ENABLED: AZ_PROP_CONSTANTS = AZ_PROP_CONSTANTS(803i32);
pub const AZ_PROP_APPLICATION_DATA: AZ_PROP_CONSTANTS = AZ_PROP_CONSTANTS(4i32);
pub const AZ_PROP_APPLICATION_NAME: AZ_PROP_CONSTANTS = AZ_PROP_CONSTANTS(802i32);
pub const AZ_PROP_APPLICATION_VERSION: AZ_PROP_CONSTANTS = AZ_PROP_CONSTANTS(801i32);
pub const AZ_PROP_APPLY_STORE_SACL: AZ_PROP_CONSTANTS = AZ_PROP_CONSTANTS(900i32);
pub const AZ_PROP_AZSTORE_DOMAIN_TIMEOUT: AZ_PROP_CONSTANTS = AZ_PROP_CONSTANTS(100i32);
pub const AZ_PROP_AZSTORE_MAJOR_VERSION: AZ_PROP_CONSTANTS = AZ_PROP_CONSTANTS(103i32);
pub const AZ_PROP_AZSTORE_MAX_SCRIPT_ENGINES: AZ_PROP_CONSTANTS = AZ_PROP_CONSTANTS(102i32);
pub const AZ_PROP_AZSTORE_MINOR_VERSION: AZ_PROP_CONSTANTS = AZ_PROP_CONSTANTS(104i32);
pub const AZ_PROP_AZSTORE_SCRIPT_ENGINE_TIMEOUT: AZ_PROP_CONSTANTS = AZ_PROP_CONSTANTS(101i32);
pub const AZ_PROP_AZSTORE_TARGET_MACHINE: AZ_PROP_CONSTANTS = AZ_PROP_CONSTANTS(105i32);
pub const AZ_PROP_AZTORE_IS_ADAM_INSTANCE: AZ_PROP_CONSTANTS = AZ_PROP_CONSTANTS(106i32);
pub const AZ_PROP_CHILD_CREATE: AZ_PROP_CONSTANTS = AZ_PROP_CONSTANTS(5i32);
pub const AZ_PROP_CLIENT_CONTEXT_LDAP_QUERY_DN: AZ_PROP_CONSTANTS = AZ_PROP_CONSTANTS(709i32);
pub const AZ_PROP_CLIENT_CONTEXT_ROLE_FOR_ACCESS_CHECK: AZ_PROP_CONSTANTS = AZ_PROP_CONSTANTS(708i32);
pub const AZ_PROP_CLIENT_CONTEXT_USER_CANONICAL: AZ_PROP_CONSTANTS = AZ_PROP_CONSTANTS(704i32);
pub const AZ_PROP_CLIENT_CONTEXT_USER_DISPLAY: AZ_PROP_CONSTANTS = AZ_PROP_CONSTANTS(702i32);
pub const AZ_PROP_CLIENT_CONTEXT_USER_DN: AZ_PROP_CONSTANTS = AZ_PROP_CONSTANTS(700i32);
pub const AZ_PROP_CLIENT_CONTEXT_USER_DNS_SAM_COMPAT: AZ_PROP_CONSTANTS = AZ_PROP_CONSTANTS(707i32);
pub const AZ_PROP_CLIENT_CONTEXT_USER_GUID: AZ_PROP_CONSTANTS = AZ_PROP_CONSTANTS(703i32);
pub const AZ_PROP_CLIENT_CONTEXT_USER_SAM_COMPAT: AZ_PROP_CONSTANTS = AZ_PROP_CONSTANTS(701i32);
pub const AZ_PROP_CLIENT_CONTEXT_USER_UPN: AZ_PROP_CONSTANTS = AZ_PROP_CONSTANTS(705i32);
pub const AZ_PROP_DELEGATED_POLICY_USERS: AZ_PROP_CONSTANTS = AZ_PROP_CONSTANTS(904i32);
pub const AZ_PROP_DELEGATED_POLICY_USERS_NAME: AZ_PROP_CONSTANTS = AZ_PROP_CONSTANTS(907i32);
pub const AZ_PROP_DESCRIPTION: AZ_PROP_CONSTANTS = AZ_PROP_CONSTANTS(2i32);
pub const AZ_PROP_GENERATE_AUDITS: AZ_PROP_CONSTANTS = AZ_PROP_CONSTANTS(901i32);
pub const AZ_PROP_GROUP_APP_MEMBERS: AZ_PROP_CONSTANTS = AZ_PROP_CONSTANTS(401i32);
pub const AZ_PROP_GROUP_APP_NON_MEMBERS: AZ_PROP_CONSTANTS = AZ_PROP_CONSTANTS(402i32);
pub const AZ_PROP_GROUP_BIZRULE: AZ_PROP_CONSTANTS = AZ_PROP_CONSTANTS(408i32);
pub const AZ_PROP_GROUP_BIZRULE_IMPORTED_PATH: AZ_PROP_CONSTANTS = AZ_PROP_CONSTANTS(410i32);
pub const AZ_PROP_GROUP_BIZRULE_LANGUAGE: AZ_PROP_CONSTANTS = AZ_PROP_CONSTANTS(409i32);
pub const AZ_PROP_GROUP_LDAP_QUERY: AZ_PROP_CONSTANTS = AZ_PROP_CONSTANTS(403i32);
pub const AZ_PROP_GROUP_MEMBERS: AZ_PROP_CONSTANTS = AZ_PROP_CONSTANTS(404i32);
pub const AZ_PROP_GROUP_MEMBERS_NAME: AZ_PROP_CONSTANTS = AZ_PROP_CONSTANTS(406i32);
pub const AZ_PROP_GROUP_NON_MEMBERS: AZ_PROP_CONSTANTS = AZ_PROP_CONSTANTS(405i32);
pub const AZ_PROP_GROUP_NON_MEMBERS_NAME: AZ_PROP_CONSTANTS = AZ_PROP_CONSTANTS(407i32);
pub const AZ_PROP_GROUP_TYPE: AZ_PROP_CONSTANTS = AZ_PROP_CONSTANTS(400i32);
pub const AZ_PROP_NAME: AZ_PROP_CONSTANTS = AZ_PROP_CONSTANTS(1i32);
pub const AZ_PROP_OPERATION_ID: AZ_PROP_CONSTANTS = AZ_PROP_CONSTANTS(200i32);
pub const AZ_PROP_POLICY_ADMINS: AZ_PROP_CONSTANTS = AZ_PROP_CONSTANTS(902i32);
pub const AZ_PROP_POLICY_ADMINS_NAME: AZ_PROP_CONSTANTS = AZ_PROP_CONSTANTS(905i32);
pub const AZ_PROP_POLICY_READERS: AZ_PROP_CONSTANTS = AZ_PROP_CONSTANTS(903i32);
pub const AZ_PROP_POLICY_READERS_NAME: AZ_PROP_CONSTANTS = AZ_PROP_CONSTANTS(906i32);
pub const AZ_PROP_ROLE_APP_MEMBERS: AZ_PROP_CONSTANTS = AZ_PROP_CONSTANTS(500i32);
pub const AZ_PROP_ROLE_MEMBERS: AZ_PROP_CONSTANTS = AZ_PROP_CONSTANTS(501i32);
pub const AZ_PROP_ROLE_MEMBERS_NAME: AZ_PROP_CONSTANTS = AZ_PROP_CONSTANTS(505i32);
pub const AZ_PROP_ROLE_OPERATIONS: AZ_PROP_CONSTANTS = AZ_PROP_CONSTANTS(502i32);
pub const AZ_PROP_ROLE_TASKS: AZ_PROP_CONSTANTS = AZ_PROP_CONSTANTS(504i32);
pub const AZ_PROP_SCOPE_BIZRULES_WRITABLE: AZ_PROP_CONSTANTS = AZ_PROP_CONSTANTS(600i32);
pub const AZ_PROP_SCOPE_CAN_BE_DELEGATED: AZ_PROP_CONSTANTS = AZ_PROP_CONSTANTS(601i32);
pub const AZ_PROP_TASK_BIZRULE: AZ_PROP_CONSTANTS = AZ_PROP_CONSTANTS(301i32);
pub const AZ_PROP_TASK_BIZRULE_IMPORTED_PATH: AZ_PROP_CONSTANTS = AZ_PROP_CONSTANTS(304i32);
pub const AZ_PROP_TASK_BIZRULE_LANGUAGE: AZ_PROP_CONSTANTS = AZ_PROP_CONSTANTS(302i32);
pub const AZ_PROP_TASK_IS_ROLE_DEFINITION: AZ_PROP_CONSTANTS = AZ_PROP_CONSTANTS(305i32);
pub const AZ_PROP_TASK_OPERATIONS: AZ_PROP_CONSTANTS = AZ_PROP_CONSTANTS(300i32);
pub const AZ_PROP_TASK_TASKS: AZ_PROP_CONSTANTS = AZ_PROP_CONSTANTS(303i32);
pub const AZ_PROP_WRITABLE: AZ_PROP_CONSTANTS = AZ_PROP_CONSTANTS(3i32);
pub const AZ_SUBMIT_FLAG_ABORT: AZ_PROP_CONSTANTS = AZ_PROP_CONSTANTS(1i32);
pub const AZ_SUBMIT_FLAG_FLUSH: AZ_PROP_CONSTANTS = AZ_PROP_CONSTANTS(2i32);
pub const AuthzAuditEventInfoAdditionalInfo: AUTHZ_AUDIT_EVENT_INFORMATION_CLASS = AUTHZ_AUDIT_EVENT_INFORMATION_CLASS(5i32);
pub const AuthzAuditEventInfoFlags: AUTHZ_AUDIT_EVENT_INFORMATION_CLASS = AUTHZ_AUDIT_EVENT_INFORMATION_CLASS(1i32);
pub const AuthzAuditEventInfoObjectName: AUTHZ_AUDIT_EVENT_INFORMATION_CLASS = AUTHZ_AUDIT_EVENT_INFORMATION_CLASS(4i32);
pub const AuthzAuditEventInfoObjectType: AUTHZ_AUDIT_EVENT_INFORMATION_CLASS = AUTHZ_AUDIT_EVENT_INFORMATION_CLASS(3i32);
pub const AuthzAuditEventInfoOperationType: AUTHZ_AUDIT_EVENT_INFORMATION_CLASS = AUTHZ_AUDIT_EVENT_INFORMATION_CLASS(2i32);
pub const AuthzContextInfoAll: AUTHZ_CONTEXT_INFORMATION_CLASS = AUTHZ_CONTEXT_INFORMATION_CLASS(9i32);
pub const AuthzContextInfoAppContainerSid: AUTHZ_CONTEXT_INFORMATION_CLASS = AUTHZ_CONTEXT_INFORMATION_CLASS(15i32);
pub const AuthzContextInfoAuthenticationId: AUTHZ_CONTEXT_INFORMATION_CLASS = AUTHZ_CONTEXT_INFORMATION_CLASS(10i32);
pub const AuthzContextInfoCapabilitySids: AUTHZ_CONTEXT_INFORMATION_CLASS = AUTHZ_CONTEXT_INFORMATION_CLASS(16i32);
pub const AuthzContextInfoDeviceClaims: AUTHZ_CONTEXT_INFORMATION_CLASS = AUTHZ_CONTEXT_INFORMATION_CLASS(14i32);
pub const AuthzContextInfoDeviceSids: AUTHZ_CONTEXT_INFORMATION_CLASS = AUTHZ_CONTEXT_INFORMATION_CLASS(12i32);
pub const AuthzContextInfoExpirationTime: AUTHZ_CONTEXT_INFORMATION_CLASS = AUTHZ_CONTEXT_INFORMATION_CLASS(5i32);
pub const AuthzContextInfoGroupsSids: AUTHZ_CONTEXT_INFORMATION_CLASS = AUTHZ_CONTEXT_INFORMATION_CLASS(2i32);
pub const AuthzContextInfoIdentifier: AUTHZ_CONTEXT_INFORMATION_CLASS = AUTHZ_CONTEXT_INFORMATION_CLASS(7i32);
pub const AuthzContextInfoPrivileges: AUTHZ_CONTEXT_INFORMATION_CLASS = AUTHZ_CONTEXT_INFORMATION_CLASS(4i32);
pub const AuthzContextInfoRestrictedSids: AUTHZ_CONTEXT_INFORMATION_CLASS = AUTHZ_CONTEXT_INFORMATION_CLASS(3i32);
pub const AuthzContextInfoSecurityAttributes: AUTHZ_CONTEXT_INFORMATION_CLASS = AUTHZ_CONTEXT_INFORMATION_CLASS(11i32);
pub const AuthzContextInfoServerContext: AUTHZ_CONTEXT_INFORMATION_CLASS = AUTHZ_CONTEXT_INFORMATION_CLASS(6i32);
pub const AuthzContextInfoSource: AUTHZ_CONTEXT_INFORMATION_CLASS = AUTHZ_CONTEXT_INFORMATION_CLASS(8i32);
pub const AuthzContextInfoUserClaims: AUTHZ_CONTEXT_INFORMATION_CLASS = AUTHZ_CONTEXT_INFORMATION_CLASS(13i32);
pub const AuthzContextInfoUserSid: AUTHZ_CONTEXT_INFORMATION_CLASS = AUTHZ_CONTEXT_INFORMATION_CLASS(1i32);
pub const DENY_ACCESS: ACCESS_MODE = ACCESS_MODE(3i32);
pub const GRANT_ACCESS: ACCESS_MODE = ACCESS_MODE(1i32);
pub const INHERITED_ACCESS_ENTRY: u32 = 16u32;
pub const INHERITED_GRANDPARENT: u32 = 536870912u32;
pub const INHERITED_PARENT: u32 = 268435456u32;
pub const NOT_USED_ACCESS: ACCESS_MODE = ACCESS_MODE(0i32);
pub const NO_MULTIPLE_TRUSTEE: MULTIPLE_TRUSTEE_OPERATION = MULTIPLE_TRUSTEE_OPERATION(0i32);
pub const OLESCRIPT_E_SYNTAX: windows_core::HRESULT = windows_core::HRESULT(0x80020101_u32 as _);
pub const ProgressCancelOperation: PROG_INVOKE_SETTING = PROG_INVOKE_SETTING(4i32);
pub const ProgressInvokeEveryObject: PROG_INVOKE_SETTING = PROG_INVOKE_SETTING(2i32);
pub const ProgressInvokeNever: PROG_INVOKE_SETTING = PROG_INVOKE_SETTING(1i32);
pub const ProgressInvokeOnError: PROG_INVOKE_SETTING = PROG_INVOKE_SETTING(3i32);
pub const ProgressInvokePrePostError: PROG_INVOKE_SETTING = PROG_INVOKE_SETTING(6i32);
pub const ProgressRetryOperation: PROG_INVOKE_SETTING = PROG_INVOKE_SETTING(5i32);
pub const REVOKE_ACCESS: ACCESS_MODE = ACCESS_MODE(4i32);
pub const SDDL_ACCESS_ALLOWED: windows_core::PCWSTR = windows_core::w!("A");
pub const SDDL_ACCESS_CONTROL_ASSISTANCE_OPS: windows_core::PCWSTR = windows_core::w!("AA");
pub const SDDL_ACCESS_DENIED: windows_core::PCWSTR = windows_core::w!("D");
pub const SDDL_ACCESS_FILTER: windows_core::PCWSTR = windows_core::w!("FL");
pub const SDDL_ACCOUNT_OPERATORS: windows_core::PCWSTR = windows_core::w!("AO");
pub const SDDL_ACE_BEGIN: windows_core::PCWSTR = windows_core::w!("(");
pub const SDDL_ACE_COND_ATTRIBUTE_PREFIX: windows_core::PCWSTR = windows_core::w!("@");
pub const SDDL_ACE_COND_BEGIN: windows_core::PCWSTR = windows_core::w!("(");
pub const SDDL_ACE_COND_BLOB_PREFIX: windows_core::PCWSTR = windows_core::w!("#");
pub const SDDL_ACE_COND_DEVICE_ATTRIBUTE_PREFIX: windows_core::PCWSTR = windows_core::w!("@DEVICE.");
pub const SDDL_ACE_COND_END: windows_core::PCWSTR = windows_core::w!(")");
pub const SDDL_ACE_COND_RESOURCE_ATTRIBUTE_PREFIX: windows_core::PCWSTR = windows_core::w!("@RESOURCE.");
pub const SDDL_ACE_COND_SID_PREFIX: windows_core::PCWSTR = windows_core::w!("SID");
pub const SDDL_ACE_COND_TOKEN_ATTRIBUTE_PREFIX: windows_core::PCWSTR = windows_core::w!("@TOKEN.");
pub const SDDL_ACE_COND_USER_ATTRIBUTE_PREFIX: windows_core::PCWSTR = windows_core::w!("@USER.");
pub const SDDL_ACE_END: windows_core::PCWSTR = windows_core::w!(")");
pub const SDDL_ALARM: windows_core::PCWSTR = windows_core::w!("AL");
pub const SDDL_ALIAS_PREW2KCOMPACC: windows_core::PCWSTR = windows_core::w!("RU");
pub const SDDL_ALIAS_SIZE: u32 = 2u32;
pub const SDDL_ALL_APP_PACKAGES: windows_core::PCWSTR = windows_core::w!("AC");
pub const SDDL_ANONYMOUS: windows_core::PCWSTR = windows_core::w!("AN");
pub const SDDL_AUDIT: windows_core::PCWSTR = windows_core::w!("AU");
pub const SDDL_AUDIT_FAILURE: windows_core::PCWSTR = windows_core::w!("FA");
pub const SDDL_AUDIT_SUCCESS: windows_core::PCWSTR = windows_core::w!("SA");
pub const SDDL_AUTHENTICATED_USERS: windows_core::PCWSTR = windows_core::w!("AU");
pub const SDDL_AUTHORITY_ASSERTED: windows_core::PCWSTR = windows_core::w!("AS");
pub const SDDL_AUTO_INHERITED: windows_core::PCWSTR = windows_core::w!("AI");
pub const SDDL_AUTO_INHERIT_REQ: windows_core::PCWSTR = windows_core::w!("AR");
pub const SDDL_BACKUP_OPERATORS: windows_core::PCWSTR = windows_core::w!("BO");
pub const SDDL_BLOB: windows_core::PCWSTR = windows_core::w!("TX");
pub const SDDL_BOOLEAN: windows_core::PCWSTR = windows_core::w!("TB");
pub const SDDL_BUILTIN_ADMINISTRATORS: windows_core::PCWSTR = windows_core::w!("BA");
pub const SDDL_BUILTIN_GUESTS: windows_core::PCWSTR = windows_core::w!("BG");
pub const SDDL_BUILTIN_USERS: windows_core::PCWSTR = windows_core::w!("BU");
pub const SDDL_CALLBACK_ACCESS_ALLOWED: windows_core::PCWSTR = windows_core::w!("XA");
pub const SDDL_CALLBACK_ACCESS_DENIED: windows_core::PCWSTR = windows_core::w!("XD");
pub const SDDL_CALLBACK_AUDIT: windows_core::PCWSTR = windows_core::w!("XU");
pub const SDDL_CALLBACK_OBJECT_ACCESS_ALLOWED: windows_core::PCWSTR = windows_core::w!("ZA");
pub const SDDL_CERTSVC_DCOM_ACCESS: windows_core::PCWSTR = windows_core::w!("CD");
pub const SDDL_CERT_SERV_ADMINISTRATORS: windows_core::PCWSTR = windows_core::w!("CA");
pub const SDDL_CLONEABLE_CONTROLLERS: windows_core::PCWSTR = windows_core::w!("CN");
pub const SDDL_CONTAINER_INHERIT: windows_core::PCWSTR = windows_core::w!("CI");
pub const SDDL_CONTROL_ACCESS: windows_core::PCWSTR = windows_core::w!("CR");
pub const SDDL_CREATE_CHILD: windows_core::PCWSTR = windows_core::w!("CC");
pub const SDDL_CREATOR_GROUP: windows_core::PCWSTR = windows_core::w!("CG");
pub const SDDL_CREATOR_OWNER: windows_core::PCWSTR = windows_core::w!("CO");
pub const SDDL_CRITICAL: windows_core::PCWSTR = windows_core::w!("CR");
pub const SDDL_CRYPTO_OPERATORS: windows_core::PCWSTR = windows_core::w!("CY");
pub const SDDL_DACL: windows_core::PCWSTR = windows_core::w!("D");
pub const SDDL_DELETE_CHILD: windows_core::PCWSTR = windows_core::w!("DC");
pub const SDDL_DELETE_TREE: windows_core::PCWSTR = windows_core::w!("DT");
pub const SDDL_DELIMINATOR: windows_core::PCWSTR = windows_core::w!(":");
pub const SDDL_DOMAIN_ADMINISTRATORS: windows_core::PCWSTR = windows_core::w!("DA");
pub const SDDL_DOMAIN_COMPUTERS: windows_core::PCWSTR = windows_core::w!("DC");
pub const SDDL_DOMAIN_DOMAIN_CONTROLLERS: windows_core::PCWSTR = windows_core::w!("DD");
pub const SDDL_DOMAIN_GUESTS: windows_core::PCWSTR = windows_core::w!("DG");
pub const SDDL_DOMAIN_USERS: windows_core::PCWSTR = windows_core::w!("DU");
pub const SDDL_ENTERPRISE_ADMINS: windows_core::PCWSTR = windows_core::w!("EA");
pub const SDDL_ENTERPRISE_DOMAIN_CONTROLLERS: windows_core::PCWSTR = windows_core::w!("ED");
pub const SDDL_ENTERPRISE_KEY_ADMINS: windows_core::PCWSTR = windows_core::w!("EK");
pub const SDDL_ENTERPRISE_RO_DCs: windows_core::PCWSTR = windows_core::w!("RO");
pub const SDDL_EVENT_LOG_READERS: windows_core::PCWSTR = windows_core::w!("ER");
pub const SDDL_EVERYONE: windows_core::PCWSTR = windows_core::w!("WD");
pub const SDDL_FILE_ALL: windows_core::PCWSTR = windows_core::w!("FA");
pub const SDDL_FILE_EXECUTE: windows_core::PCWSTR = windows_core::w!("FX");
pub const SDDL_FILE_READ: windows_core::PCWSTR = windows_core::w!("FR");
pub const SDDL_FILE_WRITE: windows_core::PCWSTR = windows_core::w!("FW");
pub const SDDL_GENERIC_ALL: windows_core::PCWSTR = windows_core::w!("GA");
pub const SDDL_GENERIC_EXECUTE: windows_core::PCWSTR = windows_core::w!("GX");
pub const SDDL_GENERIC_READ: windows_core::PCWSTR = windows_core::w!("GR");
pub const SDDL_GENERIC_WRITE: windows_core::PCWSTR = windows_core::w!("GW");
pub const SDDL_GROUP: windows_core::PCWSTR = windows_core::w!("G");
pub const SDDL_GROUP_POLICY_ADMINS: windows_core::PCWSTR = windows_core::w!("PA");
pub const SDDL_HYPER_V_ADMINS: windows_core::PCWSTR = windows_core::w!("HA");
pub const SDDL_IIS_USERS: windows_core::PCWSTR = windows_core::w!("IS");
pub const SDDL_INHERITED: windows_core::PCWSTR = windows_core::w!("ID");
pub const SDDL_INHERIT_ONLY: windows_core::PCWSTR = windows_core::w!("IO");
pub const SDDL_INT: windows_core::PCWSTR = windows_core::w!("TI");
pub const SDDL_INTERACTIVE: windows_core::PCWSTR = windows_core::w!("IU");
pub const SDDL_KEY_ADMINS: windows_core::PCWSTR = windows_core::w!("KA");
pub const SDDL_KEY_ALL: windows_core::PCWSTR = windows_core::w!("KA");
pub const SDDL_KEY_EXECUTE: windows_core::PCWSTR = windows_core::w!("KX");
pub const SDDL_KEY_READ: windows_core::PCWSTR = windows_core::w!("KR");
pub const SDDL_KEY_WRITE: windows_core::PCWSTR = windows_core::w!("KW");
pub const SDDL_LIST_CHILDREN: windows_core::PCWSTR = windows_core::w!("LC");
pub const SDDL_LIST_OBJECT: windows_core::PCWSTR = windows_core::w!("LO");
pub const SDDL_LOCAL_ADMIN: windows_core::PCWSTR = windows_core::w!("LA");
pub const SDDL_LOCAL_GUEST: windows_core::PCWSTR = windows_core::w!("LG");
pub const SDDL_LOCAL_SERVICE: windows_core::PCWSTR = windows_core::w!("LS");
pub const SDDL_LOCAL_SYSTEM: windows_core::PCWSTR = windows_core::w!("SY");
pub const SDDL_MANDATORY_LABEL: windows_core::PCWSTR = windows_core::w!("ML");
pub const SDDL_ML_HIGH: windows_core::PCWSTR = windows_core::w!("HI");
pub const SDDL_ML_LOW: windows_core::PCWSTR = windows_core::w!("LW");
pub const SDDL_ML_MEDIUM: windows_core::PCWSTR = windows_core::w!("ME");
pub const SDDL_ML_MEDIUM_PLUS: windows_core::PCWSTR = windows_core::w!("MP");
pub const SDDL_ML_SYSTEM: windows_core::PCWSTR = windows_core::w!("SI");
pub const SDDL_NETWORK: windows_core::PCWSTR = windows_core::w!("NU");
pub const SDDL_NETWORK_CONFIGURATION_OPS: windows_core::PCWSTR = windows_core::w!("NO");
pub const SDDL_NETWORK_SERVICE: windows_core::PCWSTR = windows_core::w!("NS");
pub const SDDL_NO_EXECUTE_UP: windows_core::PCWSTR = windows_core::w!("NX");
pub const SDDL_NO_PROPAGATE: windows_core::PCWSTR = windows_core::w!("NP");
pub const SDDL_NO_READ_UP: windows_core::PCWSTR = windows_core::w!("NR");
pub const SDDL_NO_WRITE_UP: windows_core::PCWSTR = windows_core::w!("NW");
pub const SDDL_NULL_ACL: windows_core::PCWSTR = windows_core::w!("NO_ACCESS_CONTROL");
pub const SDDL_OBJECT_ACCESS_ALLOWED: windows_core::PCWSTR = windows_core::w!("OA");
pub const SDDL_OBJECT_ACCESS_DENIED: windows_core::PCWSTR = windows_core::w!("OD");
pub const SDDL_OBJECT_ALARM: windows_core::PCWSTR = windows_core::w!("OL");
pub const SDDL_OBJECT_AUDIT: windows_core::PCWSTR = windows_core::w!("OU");
pub const SDDL_OBJECT_INHERIT: windows_core::PCWSTR = windows_core::w!("OI");
pub const SDDL_OWNER: windows_core::PCWSTR = windows_core::w!("O");
pub const SDDL_OWNER_RIGHTS: windows_core::PCWSTR = windows_core::w!("OW");
pub const SDDL_PERFLOG_USERS: windows_core::PCWSTR = windows_core::w!("LU");
pub const SDDL_PERFMON_USERS: windows_core::PCWSTR = windows_core::w!("MU");
pub const SDDL_PERSONAL_SELF: windows_core::PCWSTR = windows_core::w!("PS");
pub const SDDL_POWER_USERS: windows_core::PCWSTR = windows_core::w!("PU");
pub const SDDL_PRINTER_OPERATORS: windows_core::PCWSTR = windows_core::w!("PO");
pub const SDDL_PROCESS_TRUST_LABEL: windows_core::PCWSTR = windows_core::w!("TL");
pub const SDDL_PROTECTED: windows_core::PCWSTR = windows_core::w!("P");
pub const SDDL_PROTECTED_USERS: windows_core::PCWSTR = windows_core::w!("AP");
pub const SDDL_RAS_SERVERS: windows_core::PCWSTR = windows_core::w!("RS");
pub const SDDL_RDS_ENDPOINT_SERVERS: windows_core::PCWSTR = windows_core::w!("ES");
pub const SDDL_RDS_MANAGEMENT_SERVERS: windows_core::PCWSTR = windows_core::w!("MS");
pub const SDDL_RDS_REMOTE_ACCESS_SERVERS: windows_core::PCWSTR = windows_core::w!("RA");
pub const SDDL_READ_CONTROL: windows_core::PCWSTR = windows_core::w!("RC");
pub const SDDL_READ_PROPERTY: windows_core::PCWSTR = windows_core::w!("RP");
pub const SDDL_REMOTE_DESKTOP: windows_core::PCWSTR = windows_core::w!("RD");
pub const SDDL_REMOTE_MANAGEMENT_USERS: windows_core::PCWSTR = windows_core::w!("RM");
pub const SDDL_REPLICATOR: windows_core::PCWSTR = windows_core::w!("RE");
pub const SDDL_RESOURCE_ATTRIBUTE: windows_core::PCWSTR = windows_core::w!("RA");
pub const SDDL_RESTRICTED_CODE: windows_core::PCWSTR = windows_core::w!("RC");
pub const SDDL_REVISION: u32 = 1u32;
pub const SDDL_REVISION_1: u32 = 1u32;
pub const SDDL_SACL: windows_core::PCWSTR = windows_core::w!("S");
pub const SDDL_SCHEMA_ADMINISTRATORS: windows_core::PCWSTR = windows_core::w!("SA");
pub const SDDL_SCOPED_POLICY_ID: windows_core::PCWSTR = windows_core::w!("SP");
pub const SDDL_SELF_WRITE: windows_core::PCWSTR = windows_core::w!("SW");
pub const SDDL_SEPERATOR: windows_core::PCWSTR = windows_core::w!(";");
pub const SDDL_SERVER_OPERATORS: windows_core::PCWSTR = windows_core::w!("SO");
pub const SDDL_SERVICE: windows_core::PCWSTR = windows_core::w!("SU");
pub const SDDL_SERVICE_ASSERTED: windows_core::PCWSTR = windows_core::w!("SS");
pub const SDDL_SID: windows_core::PCWSTR = windows_core::w!("TD");
pub const SDDL_SPACE: windows_core::PCWSTR = windows_core::w!(" ");
pub const SDDL_STANDARD_DELETE: windows_core::PCWSTR = windows_core::w!("SD");
pub const SDDL_TRUST_PROTECTED_FILTER: windows_core::PCWSTR = windows_core::w!("TP");
pub const SDDL_UINT: windows_core::PCWSTR = windows_core::w!("TU");
pub const SDDL_USER_MODE_DRIVERS: windows_core::PCWSTR = windows_core::w!("UD");
pub const SDDL_WRITE_DAC: windows_core::PCWSTR = windows_core::w!("WD");
pub const SDDL_WRITE_OWNER: windows_core::PCWSTR = windows_core::w!("WO");
pub const SDDL_WRITE_PROPERTY: windows_core::PCWSTR = windows_core::w!("WP");
pub const SDDL_WRITE_RESTRICTED_CODE: windows_core::PCWSTR = windows_core::w!("WR");
pub const SDDL_WSTRING: windows_core::PCWSTR = windows_core::w!("TS");
pub const SET_ACCESS: ACCESS_MODE = ACCESS_MODE(2i32);
pub const SET_AUDIT_FAILURE: ACCESS_MODE = ACCESS_MODE(6i32);
pub const SET_AUDIT_SUCCESS: ACCESS_MODE = ACCESS_MODE(5i32);
pub const SE_DS_OBJECT: SE_OBJECT_TYPE = SE_OBJECT_TYPE(8i32);
pub const SE_DS_OBJECT_ALL: SE_OBJECT_TYPE = SE_OBJECT_TYPE(9i32);
pub const SE_FILE_OBJECT: SE_OBJECT_TYPE = SE_OBJECT_TYPE(1i32);
pub const SE_KERNEL_OBJECT: SE_OBJECT_TYPE = SE_OBJECT_TYPE(6i32);
pub const SE_LMSHARE: SE_OBJECT_TYPE = SE_OBJECT_TYPE(5i32);
pub const SE_PRINTER: SE_OBJECT_TYPE = SE_OBJECT_TYPE(3i32);
pub const SE_PROVIDER_DEFINED_OBJECT: SE_OBJECT_TYPE = SE_OBJECT_TYPE(10i32);
pub const SE_REGISTRY_KEY: SE_OBJECT_TYPE = SE_OBJECT_TYPE(4i32);
pub const SE_REGISTRY_WOW64_32KEY: SE_OBJECT_TYPE = SE_OBJECT_TYPE(12i32);
pub const SE_REGISTRY_WOW64_64KEY: SE_OBJECT_TYPE = SE_OBJECT_TYPE(13i32);
pub const SE_SERVICE: SE_OBJECT_TYPE = SE_OBJECT_TYPE(2i32);
pub const SE_UNKNOWN_OBJECT_TYPE: SE_OBJECT_TYPE = SE_OBJECT_TYPE(0i32);
pub const SE_WINDOW_OBJECT: SE_OBJECT_TYPE = SE_OBJECT_TYPE(7i32);
pub const SE_WMIGUID_OBJECT: SE_OBJECT_TYPE = SE_OBJECT_TYPE(11i32);
pub const TREE_SEC_INFO_RESET: TREE_SEC_INFO = TREE_SEC_INFO(2u32);
pub const TREE_SEC_INFO_RESET_KEEP_EXPLICIT: TREE_SEC_INFO = TREE_SEC_INFO(3u32);
pub const TREE_SEC_INFO_SET: TREE_SEC_INFO = TREE_SEC_INFO(1u32);
pub const TRUSTEE_ACCESS_ALL: i32 = -1i32;
pub const TRUSTEE_ACCESS_ALLOWED: i32 = 1i32;
pub const TRUSTEE_ACCESS_EXPLICIT: i32 = 1i32;
pub const TRUSTEE_ACCESS_READ: i32 = 2i32;
pub const TRUSTEE_ACCESS_WRITE: i32 = 4i32;
pub const TRUSTEE_BAD_FORM: TRUSTEE_FORM = TRUSTEE_FORM(2i32);
pub const TRUSTEE_IS_ALIAS: TRUSTEE_TYPE = TRUSTEE_TYPE(4i32);
pub const TRUSTEE_IS_COMPUTER: TRUSTEE_TYPE = TRUSTEE_TYPE(8i32);
pub const TRUSTEE_IS_DELETED: TRUSTEE_TYPE = TRUSTEE_TYPE(6i32);
pub const TRUSTEE_IS_DOMAIN: TRUSTEE_TYPE = TRUSTEE_TYPE(3i32);
pub const TRUSTEE_IS_GROUP: TRUSTEE_TYPE = TRUSTEE_TYPE(2i32);
pub const TRUSTEE_IS_IMPERSONATE: MULTIPLE_TRUSTEE_OPERATION = MULTIPLE_TRUSTEE_OPERATION(1i32);
pub const TRUSTEE_IS_INVALID: TRUSTEE_TYPE = TRUSTEE_TYPE(7i32);
pub const TRUSTEE_IS_NAME: TRUSTEE_FORM = TRUSTEE_FORM(1i32);
pub const TRUSTEE_IS_OBJECTS_AND_NAME: TRUSTEE_FORM = TRUSTEE_FORM(4i32);
pub const TRUSTEE_IS_OBJECTS_AND_SID: TRUSTEE_FORM = TRUSTEE_FORM(3i32);
pub const TRUSTEE_IS_SID: TRUSTEE_FORM = TRUSTEE_FORM(0i32);
pub const TRUSTEE_IS_UNKNOWN: TRUSTEE_TYPE = TRUSTEE_TYPE(0i32);
pub const TRUSTEE_IS_USER: TRUSTEE_TYPE = TRUSTEE_TYPE(1i32);
pub const TRUSTEE_IS_WELL_KNOWN_GROUP: TRUSTEE_TYPE = TRUSTEE_TYPE(5i32);
pub const _AUTHZ_SS_MAXSIZE: u32 = 128u32;
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct ACCESS_MODE(pub i32);
impl windows_core::TypeKind for ACCESS_MODE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for ACCESS_MODE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("ACCESS_MODE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct ACTRL_ACCESS_ENTRY_ACCESS_FLAGS(pub u32);
impl windows_core::TypeKind for ACTRL_ACCESS_ENTRY_ACCESS_FLAGS {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for ACTRL_ACCESS_ENTRY_ACCESS_FLAGS {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("ACTRL_ACCESS_ENTRY_ACCESS_FLAGS").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct AUDIT_PARAM_TYPE(pub i32);
impl windows_core::TypeKind for AUDIT_PARAM_TYPE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for AUDIT_PARAM_TYPE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("AUDIT_PARAM_TYPE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct AUTHZ_ACCESS_CHECK_FLAGS(pub u32);
impl windows_core::TypeKind for AUTHZ_ACCESS_CHECK_FLAGS {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for AUTHZ_ACCESS_CHECK_FLAGS {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("AUTHZ_ACCESS_CHECK_FLAGS").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct AUTHZ_AUDIT_EVENT_INFORMATION_CLASS(pub i32);
impl windows_core::TypeKind for AUTHZ_AUDIT_EVENT_INFORMATION_CLASS {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for AUTHZ_AUDIT_EVENT_INFORMATION_CLASS {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("AUTHZ_AUDIT_EVENT_INFORMATION_CLASS").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct AUTHZ_CONTEXT_INFORMATION_CLASS(pub i32);
impl windows_core::TypeKind for AUTHZ_CONTEXT_INFORMATION_CLASS {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for AUTHZ_CONTEXT_INFORMATION_CLASS {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("AUTHZ_CONTEXT_INFORMATION_CLASS").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct AUTHZ_GENERATE_RESULTS(pub u32);
impl windows_core::TypeKind for AUTHZ_GENERATE_RESULTS {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for AUTHZ_GENERATE_RESULTS {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("AUTHZ_GENERATE_RESULTS").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct AUTHZ_INITIALIZE_OBJECT_ACCESS_AUDIT_EVENT_FLAGS(pub u32);
impl windows_core::TypeKind for AUTHZ_INITIALIZE_OBJECT_ACCESS_AUDIT_EVENT_FLAGS {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for AUTHZ_INITIALIZE_OBJECT_ACCESS_AUDIT_EVENT_FLAGS {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("AUTHZ_INITIALIZE_OBJECT_ACCESS_AUDIT_EVENT_FLAGS").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct AUTHZ_RESOURCE_MANAGER_FLAGS(pub u32);
impl windows_core::TypeKind for AUTHZ_RESOURCE_MANAGER_FLAGS {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for AUTHZ_RESOURCE_MANAGER_FLAGS {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("AUTHZ_RESOURCE_MANAGER_FLAGS").field(&self.0).finish()
    }
}
impl AUTHZ_RESOURCE_MANAGER_FLAGS {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for AUTHZ_RESOURCE_MANAGER_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for AUTHZ_RESOURCE_MANAGER_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for AUTHZ_RESOURCE_MANAGER_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for AUTHZ_RESOURCE_MANAGER_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for AUTHZ_RESOURCE_MANAGER_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct AUTHZ_SECURITY_ATTRIBUTE_FLAGS(pub u32);
impl windows_core::TypeKind for AUTHZ_SECURITY_ATTRIBUTE_FLAGS {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for AUTHZ_SECURITY_ATTRIBUTE_FLAGS {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("AUTHZ_SECURITY_ATTRIBUTE_FLAGS").field(&self.0).finish()
    }
}
impl AUTHZ_SECURITY_ATTRIBUTE_FLAGS {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for AUTHZ_SECURITY_ATTRIBUTE_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for AUTHZ_SECURITY_ATTRIBUTE_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for AUTHZ_SECURITY_ATTRIBUTE_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for AUTHZ_SECURITY_ATTRIBUTE_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for AUTHZ_SECURITY_ATTRIBUTE_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct AUTHZ_SECURITY_ATTRIBUTE_OPERATION(pub i32);
impl windows_core::TypeKind for AUTHZ_SECURITY_ATTRIBUTE_OPERATION {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for AUTHZ_SECURITY_ATTRIBUTE_OPERATION {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("AUTHZ_SECURITY_ATTRIBUTE_OPERATION").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct AUTHZ_SID_OPERATION(pub i32);
impl windows_core::TypeKind for AUTHZ_SID_OPERATION {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for AUTHZ_SID_OPERATION {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("AUTHZ_SID_OPERATION").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct AZ_PROP_CONSTANTS(pub i32);
impl windows_core::TypeKind for AZ_PROP_CONSTANTS {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for AZ_PROP_CONSTANTS {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("AZ_PROP_CONSTANTS").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct MULTIPLE_TRUSTEE_OPERATION(pub i32);
impl windows_core::TypeKind for MULTIPLE_TRUSTEE_OPERATION {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for MULTIPLE_TRUSTEE_OPERATION {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("MULTIPLE_TRUSTEE_OPERATION").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct PROG_INVOKE_SETTING(pub i32);
impl windows_core::TypeKind for PROG_INVOKE_SETTING {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for PROG_INVOKE_SETTING {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("PROG_INVOKE_SETTING").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct SE_OBJECT_TYPE(pub i32);
impl windows_core::TypeKind for SE_OBJECT_TYPE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for SE_OBJECT_TYPE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("SE_OBJECT_TYPE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct TREE_SEC_INFO(pub u32);
impl windows_core::TypeKind for TREE_SEC_INFO {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for TREE_SEC_INFO {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("TREE_SEC_INFO").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct TRUSTEE_FORM(pub i32);
impl windows_core::TypeKind for TRUSTEE_FORM {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for TRUSTEE_FORM {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("TRUSTEE_FORM").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct TRUSTEE_TYPE(pub i32);
impl windows_core::TypeKind for TRUSTEE_TYPE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for TRUSTEE_TYPE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("TRUSTEE_TYPE").field(&self.0).finish()
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ACTRL_ACCESSA {
    pub cEntries: u32,
    pub pPropertyAccessList: *mut ACTRL_PROPERTY_ENTRYA,
}
impl windows_core::TypeKind for ACTRL_ACCESSA {
    type TypeKind = windows_core::CopyType;
}
impl Default for ACTRL_ACCESSA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ACTRL_ACCESSW {
    pub cEntries: u32,
    pub pPropertyAccessList: *mut ACTRL_PROPERTY_ENTRYW,
}
impl windows_core::TypeKind for ACTRL_ACCESSW {
    type TypeKind = windows_core::CopyType;
}
impl Default for ACTRL_ACCESSW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ACTRL_ACCESS_ENTRYA {
    pub Trustee: TRUSTEE_A,
    pub fAccessFlags: ACTRL_ACCESS_ENTRY_ACCESS_FLAGS,
    pub Access: u32,
    pub ProvSpecificAccess: u32,
    pub Inheritance: super::ACE_FLAGS,
    pub lpInheritProperty: windows_core::PSTR,
}
impl windows_core::TypeKind for ACTRL_ACCESS_ENTRYA {
    type TypeKind = windows_core::CopyType;
}
impl Default for ACTRL_ACCESS_ENTRYA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ACTRL_ACCESS_ENTRYW {
    pub Trustee: TRUSTEE_W,
    pub fAccessFlags: ACTRL_ACCESS_ENTRY_ACCESS_FLAGS,
    pub Access: u32,
    pub ProvSpecificAccess: u32,
    pub Inheritance: super::ACE_FLAGS,
    pub lpInheritProperty: windows_core::PWSTR,
}
impl windows_core::TypeKind for ACTRL_ACCESS_ENTRYW {
    type TypeKind = windows_core::CopyType;
}
impl Default for ACTRL_ACCESS_ENTRYW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ACTRL_ACCESS_ENTRY_LISTA {
    pub cEntries: u32,
    pub pAccessList: *mut ACTRL_ACCESS_ENTRYA,
}
impl windows_core::TypeKind for ACTRL_ACCESS_ENTRY_LISTA {
    type TypeKind = windows_core::CopyType;
}
impl Default for ACTRL_ACCESS_ENTRY_LISTA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ACTRL_ACCESS_ENTRY_LISTW {
    pub cEntries: u32,
    pub pAccessList: *mut ACTRL_ACCESS_ENTRYW,
}
impl windows_core::TypeKind for ACTRL_ACCESS_ENTRY_LISTW {
    type TypeKind = windows_core::CopyType;
}
impl Default for ACTRL_ACCESS_ENTRY_LISTW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ACTRL_ACCESS_INFOA {
    pub fAccessPermission: u32,
    pub lpAccessPermissionName: windows_core::PSTR,
}
impl windows_core::TypeKind for ACTRL_ACCESS_INFOA {
    type TypeKind = windows_core::CopyType;
}
impl Default for ACTRL_ACCESS_INFOA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ACTRL_ACCESS_INFOW {
    pub fAccessPermission: u32,
    pub lpAccessPermissionName: windows_core::PWSTR,
}
impl windows_core::TypeKind for ACTRL_ACCESS_INFOW {
    type TypeKind = windows_core::CopyType;
}
impl Default for ACTRL_ACCESS_INFOW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ACTRL_CONTROL_INFOA {
    pub lpControlId: windows_core::PSTR,
    pub lpControlName: windows_core::PSTR,
}
impl windows_core::TypeKind for ACTRL_CONTROL_INFOA {
    type TypeKind = windows_core::CopyType;
}
impl Default for ACTRL_CONTROL_INFOA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ACTRL_CONTROL_INFOW {
    pub lpControlId: windows_core::PWSTR,
    pub lpControlName: windows_core::PWSTR,
}
impl windows_core::TypeKind for ACTRL_CONTROL_INFOW {
    type TypeKind = windows_core::CopyType;
}
impl Default for ACTRL_CONTROL_INFOW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct ACTRL_OVERLAPPED {
    pub Anonymous: ACTRL_OVERLAPPED_0,
    pub Reserved2: u32,
    pub hEvent: super::super::Foundation::HANDLE,
}
impl windows_core::TypeKind for ACTRL_OVERLAPPED {
    type TypeKind = windows_core::CopyType;
}
impl Default for ACTRL_OVERLAPPED {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union ACTRL_OVERLAPPED_0 {
    pub Provider: *mut core::ffi::c_void,
    pub Reserved1: u32,
}
impl windows_core::TypeKind for ACTRL_OVERLAPPED_0 {
    type TypeKind = windows_core::CopyType;
}
impl Default for ACTRL_OVERLAPPED_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ACTRL_PROPERTY_ENTRYA {
    pub lpProperty: windows_core::PSTR,
    pub pAccessEntryList: *mut ACTRL_ACCESS_ENTRY_LISTA,
    pub fListFlags: u32,
}
impl windows_core::TypeKind for ACTRL_PROPERTY_ENTRYA {
    type TypeKind = windows_core::CopyType;
}
impl Default for ACTRL_PROPERTY_ENTRYA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ACTRL_PROPERTY_ENTRYW {
    pub lpProperty: windows_core::PWSTR,
    pub pAccessEntryList: *mut ACTRL_ACCESS_ENTRY_LISTW,
    pub fListFlags: u32,
}
impl windows_core::TypeKind for ACTRL_PROPERTY_ENTRYW {
    type TypeKind = windows_core::CopyType;
}
impl Default for ACTRL_PROPERTY_ENTRYW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct AUDIT_IP_ADDRESS {
    pub pIpAddress: [u8; 128],
}
impl windows_core::TypeKind for AUDIT_IP_ADDRESS {
    type TypeKind = windows_core::CopyType;
}
impl Default for AUDIT_IP_ADDRESS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct AUDIT_OBJECT_TYPE {
    pub ObjectType: windows_core::GUID,
    pub Flags: u16,
    pub Level: u16,
    pub AccessMask: u32,
}
impl windows_core::TypeKind for AUDIT_OBJECT_TYPE {
    type TypeKind = windows_core::CopyType;
}
impl Default for AUDIT_OBJECT_TYPE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct AUDIT_OBJECT_TYPES {
    pub Count: u16,
    pub Flags: u16,
    pub pObjectTypes: *mut AUDIT_OBJECT_TYPE,
}
impl windows_core::TypeKind for AUDIT_OBJECT_TYPES {
    type TypeKind = windows_core::CopyType;
}
impl Default for AUDIT_OBJECT_TYPES {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct AUDIT_PARAM {
    pub Type: AUDIT_PARAM_TYPE,
    pub Length: u32,
    pub Flags: u32,
    pub Anonymous1: AUDIT_PARAM_0,
    pub Anonymous2: AUDIT_PARAM_1,
}
impl windows_core::TypeKind for AUDIT_PARAM {
    type TypeKind = windows_core::CopyType;
}
impl Default for AUDIT_PARAM {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union AUDIT_PARAM_0 {
    pub Data0: usize,
    pub String: windows_core::PWSTR,
    pub u: usize,
    pub psid: *mut super::SID,
    pub pguid: *mut windows_core::GUID,
    pub LogonId_LowPart: u32,
    pub pObjectTypes: *mut AUDIT_OBJECT_TYPES,
    pub pIpAddress: *mut AUDIT_IP_ADDRESS,
}
impl windows_core::TypeKind for AUDIT_PARAM_0 {
    type TypeKind = windows_core::CopyType;
}
impl Default for AUDIT_PARAM_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union AUDIT_PARAM_1 {
    pub Data1: usize,
    pub LogonId_HighPart: i32,
}
impl windows_core::TypeKind for AUDIT_PARAM_1 {
    type TypeKind = windows_core::CopyType;
}
impl Default for AUDIT_PARAM_1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct AUDIT_PARAMS {
    pub Length: u32,
    pub Flags: u32,
    pub Count: u16,
    pub Parameters: *mut AUDIT_PARAM,
}
impl windows_core::TypeKind for AUDIT_PARAMS {
    type TypeKind = windows_core::CopyType;
}
impl Default for AUDIT_PARAMS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct AUTHZ_ACCESS_CHECK_RESULTS_HANDLE(pub *mut core::ffi::c_void);
impl AUTHZ_ACCESS_CHECK_RESULTS_HANDLE {
    pub fn is_invalid(&self) -> bool {
        self.0 == -1 as _ || self.0 == 0 as _
    }
}
impl windows_core::Free for AUTHZ_ACCESS_CHECK_RESULTS_HANDLE {
    #[inline]
    unsafe fn free(&mut self) {
        if !self.is_invalid() {
            _ = AuthzFreeHandle(*self);
        }
    }
}
impl Default for AUTHZ_ACCESS_CHECK_RESULTS_HANDLE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for AUTHZ_ACCESS_CHECK_RESULTS_HANDLE {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct AUTHZ_ACCESS_REPLY {
    pub ResultListLength: u32,
    pub GrantedAccessMask: *mut u32,
    pub SaclEvaluationResults: *mut AUTHZ_GENERATE_RESULTS,
    pub Error: *mut u32,
}
impl windows_core::TypeKind for AUTHZ_ACCESS_REPLY {
    type TypeKind = windows_core::CopyType;
}
impl Default for AUTHZ_ACCESS_REPLY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct AUTHZ_ACCESS_REQUEST {
    pub DesiredAccess: u32,
    pub PrincipalSelfSid: super::PSID,
    pub ObjectTypeList: *mut super::OBJECT_TYPE_LIST,
    pub ObjectTypeListLength: u32,
    pub OptionalArguments: *mut core::ffi::c_void,
}
impl windows_core::TypeKind for AUTHZ_ACCESS_REQUEST {
    type TypeKind = windows_core::CopyType;
}
impl Default for AUTHZ_ACCESS_REQUEST {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct AUTHZ_AUDIT_EVENT_HANDLE(pub *mut core::ffi::c_void);
impl AUTHZ_AUDIT_EVENT_HANDLE {
    pub fn is_invalid(&self) -> bool {
        self.0 == -1 as _ || self.0 == 0 as _
    }
}
impl windows_core::Free for AUTHZ_AUDIT_EVENT_HANDLE {
    #[inline]
    unsafe fn free(&mut self) {
        if !self.is_invalid() {
            _ = AuthzFreeAuditEvent(*self);
        }
    }
}
impl Default for AUTHZ_AUDIT_EVENT_HANDLE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for AUTHZ_AUDIT_EVENT_HANDLE {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct AUTHZ_AUDIT_EVENT_TYPE_HANDLE(pub *mut core::ffi::c_void);
impl AUTHZ_AUDIT_EVENT_TYPE_HANDLE {
    pub fn is_invalid(&self) -> bool {
        self.0 == -1 as _ || self.0 == 0 as _
    }
}
impl Default for AUTHZ_AUDIT_EVENT_TYPE_HANDLE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for AUTHZ_AUDIT_EVENT_TYPE_HANDLE {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct AUTHZ_AUDIT_EVENT_TYPE_LEGACY {
    pub CategoryId: u16,
    pub AuditId: u16,
    pub ParameterCount: u16,
}
impl windows_core::TypeKind for AUTHZ_AUDIT_EVENT_TYPE_LEGACY {
    type TypeKind = windows_core::CopyType;
}
impl Default for AUTHZ_AUDIT_EVENT_TYPE_LEGACY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct AUTHZ_AUDIT_EVENT_TYPE_OLD {
    pub Version: u32,
    pub dwFlags: u32,
    pub RefCount: i32,
    pub hAudit: usize,
    pub LinkId: super::super::Foundation::LUID,
    pub u: AUTHZ_AUDIT_EVENT_TYPE_UNION,
}
impl windows_core::TypeKind for AUTHZ_AUDIT_EVENT_TYPE_OLD {
    type TypeKind = windows_core::CopyType;
}
impl Default for AUTHZ_AUDIT_EVENT_TYPE_OLD {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union AUTHZ_AUDIT_EVENT_TYPE_UNION {
    pub Legacy: AUTHZ_AUDIT_EVENT_TYPE_LEGACY,
}
impl windows_core::TypeKind for AUTHZ_AUDIT_EVENT_TYPE_UNION {
    type TypeKind = windows_core::CopyType;
}
impl Default for AUTHZ_AUDIT_EVENT_TYPE_UNION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct AUTHZ_CAP_CHANGE_SUBSCRIPTION_HANDLE(pub *mut core::ffi::c_void);
impl AUTHZ_CAP_CHANGE_SUBSCRIPTION_HANDLE {
    pub fn is_invalid(&self) -> bool {
        self.0 == -1 as _ || self.0 == 0 as _
    }
}
impl windows_core::Free for AUTHZ_CAP_CHANGE_SUBSCRIPTION_HANDLE {
    #[inline]
    unsafe fn free(&mut self) {
        if !self.is_invalid() {
            _ = AuthzUnregisterCapChangeNotification(*self);
        }
    }
}
impl Default for AUTHZ_CAP_CHANGE_SUBSCRIPTION_HANDLE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for AUTHZ_CAP_CHANGE_SUBSCRIPTION_HANDLE {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct AUTHZ_CLIENT_CONTEXT_HANDLE(pub *mut core::ffi::c_void);
impl AUTHZ_CLIENT_CONTEXT_HANDLE {
    pub fn is_invalid(&self) -> bool {
        self.0 == -1 as _ || self.0 == 0 as _
    }
}
impl windows_core::Free for AUTHZ_CLIENT_CONTEXT_HANDLE {
    #[inline]
    unsafe fn free(&mut self) {
        if !self.is_invalid() {
            _ = AuthzFreeContext(*self);
        }
    }
}
impl Default for AUTHZ_CLIENT_CONTEXT_HANDLE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for AUTHZ_CLIENT_CONTEXT_HANDLE {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct AUTHZ_INIT_INFO {
    pub version: u16,
    pub szResourceManagerName: windows_core::PCWSTR,
    pub pfnDynamicAccessCheck: PFN_AUTHZ_DYNAMIC_ACCESS_CHECK,
    pub pfnComputeDynamicGroups: PFN_AUTHZ_COMPUTE_DYNAMIC_GROUPS,
    pub pfnFreeDynamicGroups: PFN_AUTHZ_FREE_DYNAMIC_GROUPS,
    pub pfnGetCentralAccessPolicy: PFN_AUTHZ_GET_CENTRAL_ACCESS_POLICY,
    pub pfnFreeCentralAccessPolicy: PFN_AUTHZ_FREE_CENTRAL_ACCESS_POLICY,
}
impl windows_core::TypeKind for AUTHZ_INIT_INFO {
    type TypeKind = windows_core::CopyType;
}
impl Default for AUTHZ_INIT_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct AUTHZ_REGISTRATION_OBJECT_TYPE_NAME_OFFSET {
    pub szObjectTypeName: windows_core::PWSTR,
    pub dwOffset: u32,
}
impl windows_core::TypeKind for AUTHZ_REGISTRATION_OBJECT_TYPE_NAME_OFFSET {
    type TypeKind = windows_core::CopyType;
}
impl Default for AUTHZ_REGISTRATION_OBJECT_TYPE_NAME_OFFSET {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct AUTHZ_RESOURCE_MANAGER_HANDLE(pub *mut core::ffi::c_void);
impl AUTHZ_RESOURCE_MANAGER_HANDLE {
    pub fn is_invalid(&self) -> bool {
        self.0 == -1 as _ || self.0 == 0 as _
    }
}
impl windows_core::Free for AUTHZ_RESOURCE_MANAGER_HANDLE {
    #[inline]
    unsafe fn free(&mut self) {
        if !self.is_invalid() {
            _ = AuthzFreeResourceManager(*self);
        }
    }
}
impl Default for AUTHZ_RESOURCE_MANAGER_HANDLE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for AUTHZ_RESOURCE_MANAGER_HANDLE {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct AUTHZ_RPC_INIT_INFO_CLIENT {
    pub version: u16,
    pub ObjectUuid: windows_core::PWSTR,
    pub ProtSeq: windows_core::PWSTR,
    pub NetworkAddr: windows_core::PWSTR,
    pub Endpoint: windows_core::PWSTR,
    pub Options: windows_core::PWSTR,
    pub ServerSpn: windows_core::PWSTR,
}
impl windows_core::TypeKind for AUTHZ_RPC_INIT_INFO_CLIENT {
    type TypeKind = windows_core::CopyType;
}
impl Default for AUTHZ_RPC_INIT_INFO_CLIENT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct AUTHZ_SECURITY_ATTRIBUTES_INFORMATION {
    pub Version: u16,
    pub Reserved: u16,
    pub AttributeCount: u32,
    pub Attribute: AUTHZ_SECURITY_ATTRIBUTES_INFORMATION_0,
}
impl windows_core::TypeKind for AUTHZ_SECURITY_ATTRIBUTES_INFORMATION {
    type TypeKind = windows_core::CopyType;
}
impl Default for AUTHZ_SECURITY_ATTRIBUTES_INFORMATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union AUTHZ_SECURITY_ATTRIBUTES_INFORMATION_0 {
    pub pAttributeV1: *mut AUTHZ_SECURITY_ATTRIBUTE_V1,
}
impl windows_core::TypeKind for AUTHZ_SECURITY_ATTRIBUTES_INFORMATION_0 {
    type TypeKind = windows_core::CopyType;
}
impl Default for AUTHZ_SECURITY_ATTRIBUTES_INFORMATION_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct AUTHZ_SECURITY_ATTRIBUTE_FQBN_VALUE {
    pub Version: u64,
    pub pName: windows_core::PWSTR,
}
impl windows_core::TypeKind for AUTHZ_SECURITY_ATTRIBUTE_FQBN_VALUE {
    type TypeKind = windows_core::CopyType;
}
impl Default for AUTHZ_SECURITY_ATTRIBUTE_FQBN_VALUE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct AUTHZ_SECURITY_ATTRIBUTE_OCTET_STRING_VALUE {
    pub pValue: *mut core::ffi::c_void,
    pub ValueLength: u32,
}
impl windows_core::TypeKind for AUTHZ_SECURITY_ATTRIBUTE_OCTET_STRING_VALUE {
    type TypeKind = windows_core::CopyType;
}
impl Default for AUTHZ_SECURITY_ATTRIBUTE_OCTET_STRING_VALUE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct AUTHZ_SECURITY_ATTRIBUTE_V1 {
    pub pName: windows_core::PWSTR,
    pub ValueType: u16,
    pub Reserved: u16,
    pub Flags: AUTHZ_SECURITY_ATTRIBUTE_FLAGS,
    pub ValueCount: u32,
    pub Values: AUTHZ_SECURITY_ATTRIBUTE_V1_0,
}
impl windows_core::TypeKind for AUTHZ_SECURITY_ATTRIBUTE_V1 {
    type TypeKind = windows_core::CopyType;
}
impl Default for AUTHZ_SECURITY_ATTRIBUTE_V1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union AUTHZ_SECURITY_ATTRIBUTE_V1_0 {
    pub pInt64: *mut i64,
    pub pUint64: *mut u64,
    pub ppString: *mut windows_core::PWSTR,
    pub pFqbn: *mut AUTHZ_SECURITY_ATTRIBUTE_FQBN_VALUE,
    pub pOctetString: *mut AUTHZ_SECURITY_ATTRIBUTE_OCTET_STRING_VALUE,
}
impl windows_core::TypeKind for AUTHZ_SECURITY_ATTRIBUTE_V1_0 {
    type TypeKind = windows_core::CopyType;
}
impl Default for AUTHZ_SECURITY_ATTRIBUTE_V1_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct AUTHZ_SECURITY_EVENT_PROVIDER_HANDLE(pub *mut core::ffi::c_void);
impl AUTHZ_SECURITY_EVENT_PROVIDER_HANDLE {
    pub fn is_invalid(&self) -> bool {
        self.0 == -1 as _ || self.0 == 0 as _
    }
}
impl Default for AUTHZ_SECURITY_EVENT_PROVIDER_HANDLE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for AUTHZ_SECURITY_EVENT_PROVIDER_HANDLE {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct AUTHZ_SOURCE_SCHEMA_REGISTRATION {
    pub dwFlags: u32,
    pub szEventSourceName: windows_core::PWSTR,
    pub szEventMessageFile: windows_core::PWSTR,
    pub szEventSourceXmlSchemaFile: windows_core::PWSTR,
    pub szEventAccessStringsFile: windows_core::PWSTR,
    pub szExecutableImagePath: windows_core::PWSTR,
    pub Anonymous: AUTHZ_SOURCE_SCHEMA_REGISTRATION_0,
    pub dwObjectTypeNameCount: u32,
    pub ObjectTypeNames: [AUTHZ_REGISTRATION_OBJECT_TYPE_NAME_OFFSET; 1],
}
impl windows_core::TypeKind for AUTHZ_SOURCE_SCHEMA_REGISTRATION {
    type TypeKind = windows_core::CopyType;
}
impl Default for AUTHZ_SOURCE_SCHEMA_REGISTRATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union AUTHZ_SOURCE_SCHEMA_REGISTRATION_0 {
    pub pReserved: *mut core::ffi::c_void,
    pub pProviderGuid: *mut windows_core::GUID,
}
impl windows_core::TypeKind for AUTHZ_SOURCE_SCHEMA_REGISTRATION_0 {
    type TypeKind = windows_core::CopyType;
}
impl Default for AUTHZ_SOURCE_SCHEMA_REGISTRATION_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const AzAuthorizationStore: windows_core::GUID = windows_core::GUID::from_u128(0xb2bcff59_a757_4b0b_a1bc_ea69981da69e);
pub const AzBizRuleContext: windows_core::GUID = windows_core::GUID::from_u128(0x5c2dc96f_8d51_434b_b33c_379bccae77c3);
pub const AzPrincipalLocator: windows_core::GUID = windows_core::GUID::from_u128(0x483afb5d_70df_4e16_abdc_a1de4d015a3e);
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct EXPLICIT_ACCESS_A {
    pub grfAccessPermissions: u32,
    pub grfAccessMode: ACCESS_MODE,
    pub grfInheritance: super::ACE_FLAGS,
    pub Trustee: TRUSTEE_A,
}
impl windows_core::TypeKind for EXPLICIT_ACCESS_A {
    type TypeKind = windows_core::CopyType;
}
impl Default for EXPLICIT_ACCESS_A {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct EXPLICIT_ACCESS_W {
    pub grfAccessPermissions: u32,
    pub grfAccessMode: ACCESS_MODE,
    pub grfInheritance: super::ACE_FLAGS,
    pub Trustee: TRUSTEE_W,
}
impl windows_core::TypeKind for EXPLICIT_ACCESS_W {
    type TypeKind = windows_core::CopyType;
}
impl Default for EXPLICIT_ACCESS_W {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FN_OBJECT_MGR_FUNCTS {
    pub Placeholder: u32,
}
impl windows_core::TypeKind for FN_OBJECT_MGR_FUNCTS {
    type TypeKind = windows_core::CopyType;
}
impl Default for FN_OBJECT_MGR_FUNCTS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct INHERITED_FROMA {
    pub GenerationGap: i32,
    pub AncestorName: windows_core::PSTR,
}
impl windows_core::TypeKind for INHERITED_FROMA {
    type TypeKind = windows_core::CopyType;
}
impl Default for INHERITED_FROMA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct INHERITED_FROMW {
    pub GenerationGap: i32,
    pub AncestorName: windows_core::PWSTR,
}
impl windows_core::TypeKind for INHERITED_FROMW {
    type TypeKind = windows_core::CopyType;
}
impl Default for INHERITED_FROMW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct OBJECTS_AND_NAME_A {
    pub ObjectsPresent: super::SYSTEM_AUDIT_OBJECT_ACE_FLAGS,
    pub ObjectType: SE_OBJECT_TYPE,
    pub ObjectTypeName: windows_core::PSTR,
    pub InheritedObjectTypeName: windows_core::PSTR,
    pub ptstrName: windows_core::PSTR,
}
impl windows_core::TypeKind for OBJECTS_AND_NAME_A {
    type TypeKind = windows_core::CopyType;
}
impl Default for OBJECTS_AND_NAME_A {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct OBJECTS_AND_NAME_W {
    pub ObjectsPresent: super::SYSTEM_AUDIT_OBJECT_ACE_FLAGS,
    pub ObjectType: SE_OBJECT_TYPE,
    pub ObjectTypeName: windows_core::PWSTR,
    pub InheritedObjectTypeName: windows_core::PWSTR,
    pub ptstrName: windows_core::PWSTR,
}
impl windows_core::TypeKind for OBJECTS_AND_NAME_W {
    type TypeKind = windows_core::CopyType;
}
impl Default for OBJECTS_AND_NAME_W {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct OBJECTS_AND_SID {
    pub ObjectsPresent: super::SYSTEM_AUDIT_OBJECT_ACE_FLAGS,
    pub ObjectTypeGuid: windows_core::GUID,
    pub InheritedObjectTypeGuid: windows_core::GUID,
    pub pSid: *mut super::SID,
}
impl windows_core::TypeKind for OBJECTS_AND_SID {
    type TypeKind = windows_core::CopyType;
}
impl Default for OBJECTS_AND_SID {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct TRUSTEE_A {
    pub pMultipleTrustee: *mut TRUSTEE_A,
    pub MultipleTrusteeOperation: MULTIPLE_TRUSTEE_OPERATION,
    pub TrusteeForm: TRUSTEE_FORM,
    pub TrusteeType: TRUSTEE_TYPE,
    pub ptstrName: windows_core::PSTR,
}
impl windows_core::TypeKind for TRUSTEE_A {
    type TypeKind = windows_core::CopyType;
}
impl Default for TRUSTEE_A {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct TRUSTEE_ACCESSA {
    pub lpProperty: windows_core::PSTR,
    pub Access: u32,
    pub fAccessFlags: u32,
    pub fReturnedAccess: u32,
}
impl windows_core::TypeKind for TRUSTEE_ACCESSA {
    type TypeKind = windows_core::CopyType;
}
impl Default for TRUSTEE_ACCESSA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct TRUSTEE_ACCESSW {
    pub lpProperty: windows_core::PWSTR,
    pub Access: u32,
    pub fAccessFlags: u32,
    pub fReturnedAccess: u32,
}
impl windows_core::TypeKind for TRUSTEE_ACCESSW {
    type TypeKind = windows_core::CopyType;
}
impl Default for TRUSTEE_ACCESSW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct TRUSTEE_W {
    pub pMultipleTrustee: *mut TRUSTEE_W,
    pub MultipleTrusteeOperation: MULTIPLE_TRUSTEE_OPERATION,
    pub TrusteeForm: TRUSTEE_FORM,
    pub TrusteeType: TRUSTEE_TYPE,
    pub ptstrName: windows_core::PWSTR,
}
impl windows_core::TypeKind for TRUSTEE_W {
    type TypeKind = windows_core::CopyType;
}
impl Default for TRUSTEE_W {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type FN_PROGRESS = Option<unsafe extern "system" fn(pobjectname: windows_core::PCWSTR, status: u32, pinvokesetting: *mut PROG_INVOKE_SETTING, args: *const core::ffi::c_void, securityset: super::super::Foundation::BOOL)>;
pub type PFN_AUTHZ_COMPUTE_DYNAMIC_GROUPS = Option<unsafe extern "system" fn(hauthzclientcontext: AUTHZ_CLIENT_CONTEXT_HANDLE, args: *const core::ffi::c_void, psidattrarray: *mut *mut super::SID_AND_ATTRIBUTES, psidcount: *mut u32, prestrictedsidattrarray: *mut *mut super::SID_AND_ATTRIBUTES, prestrictedsidcount: *mut u32) -> super::super::Foundation::BOOL>;
pub type PFN_AUTHZ_DYNAMIC_ACCESS_CHECK = Option<unsafe extern "system" fn(hauthzclientcontext: AUTHZ_CLIENT_CONTEXT_HANDLE, pace: *const super::ACE_HEADER, pargs: *const core::ffi::c_void, pbaceapplicable: *mut super::super::Foundation::BOOL) -> super::super::Foundation::BOOL>;
pub type PFN_AUTHZ_FREE_CENTRAL_ACCESS_POLICY = Option<unsafe extern "system" fn(pcentralaccesspolicy: *const core::ffi::c_void)>;
pub type PFN_AUTHZ_FREE_DYNAMIC_GROUPS = Option<unsafe extern "system" fn(psidattrarray: *const super::SID_AND_ATTRIBUTES)>;
pub type PFN_AUTHZ_GET_CENTRAL_ACCESS_POLICY = Option<unsafe extern "system" fn(hauthzclientcontext: AUTHZ_CLIENT_CONTEXT_HANDLE, capid: super::PSID, pargs: *const core::ffi::c_void, pcentralaccesspolicyapplicable: *mut super::super::Foundation::BOOL, ppcentralaccesspolicy: *mut *mut core::ffi::c_void) -> super::super::Foundation::BOOL>;
#[cfg(feature = "implement")]
core::include!("impl.rs");
