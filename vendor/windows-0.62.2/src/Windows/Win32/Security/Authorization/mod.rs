#[cfg(feature = "Win32_Security_Authorization_UI")]
pub mod UI;
#[inline]
pub unsafe fn AuthzAccessCheck(flags: AUTHZ_ACCESS_CHECK_FLAGS, hauthzclientcontext: AUTHZ_CLIENT_CONTEXT_HANDLE, prequest: *const AUTHZ_ACCESS_REQUEST, hauditevent: Option<AUTHZ_AUDIT_EVENT_HANDLE>, psecuritydescriptor: super::PSECURITY_DESCRIPTOR, optionalsecuritydescriptorarray: Option<&[super::PSECURITY_DESCRIPTOR]>, preply: *mut AUTHZ_ACCESS_REPLY, phaccesscheckresults: Option<*mut AUTHZ_ACCESS_CHECK_RESULTS_HANDLE>) -> windows_core::Result<()> {
    windows_core::link!("authz.dll" "system" fn AuthzAccessCheck(flags : AUTHZ_ACCESS_CHECK_FLAGS, hauthzclientcontext : AUTHZ_CLIENT_CONTEXT_HANDLE, prequest : *const AUTHZ_ACCESS_REQUEST, hauditevent : AUTHZ_AUDIT_EVENT_HANDLE, psecuritydescriptor : super:: PSECURITY_DESCRIPTOR, optionalsecuritydescriptorarray : *const super:: PSECURITY_DESCRIPTOR, optionalsecuritydescriptorcount : u32, preply : *mut AUTHZ_ACCESS_REPLY, phaccesscheckresults : *mut AUTHZ_ACCESS_CHECK_RESULTS_HANDLE) -> windows_core::BOOL);
    unsafe { AuthzAccessCheck(flags, hauthzclientcontext, prequest, hauditevent.unwrap_or(core::mem::zeroed()) as _, psecuritydescriptor, core::mem::transmute(optionalsecuritydescriptorarray.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), optionalsecuritydescriptorarray.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), preply as _, phaccesscheckresults.unwrap_or(core::mem::zeroed()) as _).ok() }
}
#[inline]
pub unsafe fn AuthzAddSidsToContext(hauthzclientcontext: AUTHZ_CLIENT_CONTEXT_HANDLE, sids: Option<*const super::SID_AND_ATTRIBUTES>, sidcount: u32, restrictedsids: Option<*const super::SID_AND_ATTRIBUTES>, restrictedsidcount: u32, phnewauthzclientcontext: *mut AUTHZ_CLIENT_CONTEXT_HANDLE) -> windows_core::Result<()> {
    windows_core::link!("authz.dll" "system" fn AuthzAddSidsToContext(hauthzclientcontext : AUTHZ_CLIENT_CONTEXT_HANDLE, sids : *const super:: SID_AND_ATTRIBUTES, sidcount : u32, restrictedsids : *const super:: SID_AND_ATTRIBUTES, restrictedsidcount : u32, phnewauthzclientcontext : *mut AUTHZ_CLIENT_CONTEXT_HANDLE) -> windows_core::BOOL);
    unsafe { AuthzAddSidsToContext(hauthzclientcontext, sids.unwrap_or(core::mem::zeroed()) as _, sidcount, restrictedsids.unwrap_or(core::mem::zeroed()) as _, restrictedsidcount, phnewauthzclientcontext as _).ok() }
}
#[inline]
pub unsafe fn AuthzCachedAccessCheck(flags: u32, haccesscheckresults: AUTHZ_ACCESS_CHECK_RESULTS_HANDLE, prequest: *const AUTHZ_ACCESS_REQUEST, hauditevent: Option<AUTHZ_AUDIT_EVENT_HANDLE>, preply: *mut AUTHZ_ACCESS_REPLY) -> windows_core::Result<()> {
    windows_core::link!("authz.dll" "system" fn AuthzCachedAccessCheck(flags : u32, haccesscheckresults : AUTHZ_ACCESS_CHECK_RESULTS_HANDLE, prequest : *const AUTHZ_ACCESS_REQUEST, hauditevent : AUTHZ_AUDIT_EVENT_HANDLE, preply : *mut AUTHZ_ACCESS_REPLY) -> windows_core::BOOL);
    unsafe { AuthzCachedAccessCheck(flags, haccesscheckresults, prequest, hauditevent.unwrap_or(core::mem::zeroed()) as _, preply as _).ok() }
}
#[inline]
pub unsafe fn AuthzEnumerateSecurityEventSources(dwflags: u32, buffer: *mut AUTHZ_SOURCE_SCHEMA_REGISTRATION, pdwcount: *mut u32, pdwlength: *mut u32) -> windows_core::Result<()> {
    windows_core::link!("authz.dll" "system" fn AuthzEnumerateSecurityEventSources(dwflags : u32, buffer : *mut AUTHZ_SOURCE_SCHEMA_REGISTRATION, pdwcount : *mut u32, pdwlength : *mut u32) -> windows_core::BOOL);
    unsafe { AuthzEnumerateSecurityEventSources(dwflags, buffer as _, pdwcount as _, pdwlength as _).ok() }
}
#[inline]
pub unsafe fn AuthzEvaluateSacl(authzclientcontext: AUTHZ_CLIENT_CONTEXT_HANDLE, prequest: *const AUTHZ_ACCESS_REQUEST, sacl: *const super::ACL, grantedaccess: u32, accessgranted: bool, pbgenerateaudit: *mut windows_core::BOOL) -> windows_core::BOOL {
    windows_core::link!("authz.dll" "system" fn AuthzEvaluateSacl(authzclientcontext : AUTHZ_CLIENT_CONTEXT_HANDLE, prequest : *const AUTHZ_ACCESS_REQUEST, sacl : *const super:: ACL, grantedaccess : u32, accessgranted : windows_core::BOOL, pbgenerateaudit : *mut windows_core::BOOL) -> windows_core::BOOL);
    unsafe { AuthzEvaluateSacl(authzclientcontext, prequest, sacl, grantedaccess, accessgranted.into(), pbgenerateaudit as _) }
}
#[inline]
pub unsafe fn AuthzFreeAuditEvent(hauditevent: AUTHZ_AUDIT_EVENT_HANDLE) -> windows_core::Result<()> {
    windows_core::link!("authz.dll" "system" fn AuthzFreeAuditEvent(hauditevent : AUTHZ_AUDIT_EVENT_HANDLE) -> windows_core::BOOL);
    unsafe { AuthzFreeAuditEvent(hauditevent).ok() }
}
#[inline]
pub unsafe fn AuthzFreeCentralAccessPolicyCache() -> windows_core::Result<()> {
    windows_core::link!("authz.dll" "system" fn AuthzFreeCentralAccessPolicyCache() -> windows_core::BOOL);
    unsafe { AuthzFreeCentralAccessPolicyCache().ok() }
}
#[inline]
pub unsafe fn AuthzFreeContext(hauthzclientcontext: AUTHZ_CLIENT_CONTEXT_HANDLE) -> windows_core::Result<()> {
    windows_core::link!("authz.dll" "system" fn AuthzFreeContext(hauthzclientcontext : AUTHZ_CLIENT_CONTEXT_HANDLE) -> windows_core::BOOL);
    unsafe { AuthzFreeContext(hauthzclientcontext).ok() }
}
#[inline]
pub unsafe fn AuthzFreeHandle(haccesscheckresults: AUTHZ_ACCESS_CHECK_RESULTS_HANDLE) -> windows_core::Result<()> {
    windows_core::link!("authz.dll" "system" fn AuthzFreeHandle(haccesscheckresults : AUTHZ_ACCESS_CHECK_RESULTS_HANDLE) -> windows_core::BOOL);
    unsafe { AuthzFreeHandle(haccesscheckresults as _).ok() }
}
#[inline]
pub unsafe fn AuthzFreeResourceManager(hauthzresourcemanager: AUTHZ_RESOURCE_MANAGER_HANDLE) -> windows_core::Result<()> {
    windows_core::link!("authz.dll" "system" fn AuthzFreeResourceManager(hauthzresourcemanager : AUTHZ_RESOURCE_MANAGER_HANDLE) -> windows_core::BOOL);
    unsafe { AuthzFreeResourceManager(hauthzresourcemanager).ok() }
}
#[inline]
pub unsafe fn AuthzGetInformationFromContext(hauthzclientcontext: AUTHZ_CLIENT_CONTEXT_HANDLE, infoclass: AUTHZ_CONTEXT_INFORMATION_CLASS, buffersize: u32, psizerequired: *mut u32, buffer: *mut core::ffi::c_void) -> windows_core::Result<()> {
    windows_core::link!("authz.dll" "system" fn AuthzGetInformationFromContext(hauthzclientcontext : AUTHZ_CLIENT_CONTEXT_HANDLE, infoclass : AUTHZ_CONTEXT_INFORMATION_CLASS, buffersize : u32, psizerequired : *mut u32, buffer : *mut core::ffi::c_void) -> windows_core::BOOL);
    unsafe { AuthzGetInformationFromContext(hauthzclientcontext, infoclass, buffersize, psizerequired as _, buffer as _).ok() }
}
#[inline]
pub unsafe fn AuthzInitializeCompoundContext(usercontext: AUTHZ_CLIENT_CONTEXT_HANDLE, devicecontext: AUTHZ_CLIENT_CONTEXT_HANDLE, phcompoundcontext: *mut AUTHZ_CLIENT_CONTEXT_HANDLE) -> windows_core::Result<()> {
    windows_core::link!("authz.dll" "system" fn AuthzInitializeCompoundContext(usercontext : AUTHZ_CLIENT_CONTEXT_HANDLE, devicecontext : AUTHZ_CLIENT_CONTEXT_HANDLE, phcompoundcontext : *mut AUTHZ_CLIENT_CONTEXT_HANDLE) -> windows_core::BOOL);
    unsafe { AuthzInitializeCompoundContext(usercontext, devicecontext, phcompoundcontext as _).ok() }
}
#[inline]
pub unsafe fn AuthzInitializeContextFromAuthzContext(flags: u32, hauthzclientcontext: AUTHZ_CLIENT_CONTEXT_HANDLE, pexpirationtime: Option<*const i64>, identifier: super::super::Foundation::LUID, dynamicgroupargs: *const core::ffi::c_void, phnewauthzclientcontext: *mut AUTHZ_CLIENT_CONTEXT_HANDLE) -> windows_core::Result<()> {
    windows_core::link!("authz.dll" "system" fn AuthzInitializeContextFromAuthzContext(flags : u32, hauthzclientcontext : AUTHZ_CLIENT_CONTEXT_HANDLE, pexpirationtime : *const i64, identifier : super::super::Foundation:: LUID, dynamicgroupargs : *const core::ffi::c_void, phnewauthzclientcontext : *mut AUTHZ_CLIENT_CONTEXT_HANDLE) -> windows_core::BOOL);
    unsafe { AuthzInitializeContextFromAuthzContext(flags, hauthzclientcontext, pexpirationtime.unwrap_or(core::mem::zeroed()) as _, core::mem::transmute(identifier), dynamicgroupargs, phnewauthzclientcontext as _).ok() }
}
#[inline]
pub unsafe fn AuthzInitializeContextFromSid(flags: u32, usersid: super::PSID, hauthzresourcemanager: AUTHZ_RESOURCE_MANAGER_HANDLE, pexpirationtime: Option<*const i64>, identifier: super::super::Foundation::LUID, dynamicgroupargs: Option<*const core::ffi::c_void>, phauthzclientcontext: *mut AUTHZ_CLIENT_CONTEXT_HANDLE) -> windows_core::Result<()> {
    windows_core::link!("authz.dll" "system" fn AuthzInitializeContextFromSid(flags : u32, usersid : super:: PSID, hauthzresourcemanager : AUTHZ_RESOURCE_MANAGER_HANDLE, pexpirationtime : *const i64, identifier : super::super::Foundation:: LUID, dynamicgroupargs : *const core::ffi::c_void, phauthzclientcontext : *mut AUTHZ_CLIENT_CONTEXT_HANDLE) -> windows_core::BOOL);
    unsafe { AuthzInitializeContextFromSid(flags, usersid, hauthzresourcemanager, pexpirationtime.unwrap_or(core::mem::zeroed()) as _, core::mem::transmute(identifier), dynamicgroupargs.unwrap_or(core::mem::zeroed()) as _, phauthzclientcontext as _).ok() }
}
#[inline]
pub unsafe fn AuthzInitializeContextFromToken(flags: u32, tokenhandle: super::super::Foundation::HANDLE, hauthzresourcemanager: AUTHZ_RESOURCE_MANAGER_HANDLE, pexpirationtime: Option<*const i64>, identifier: super::super::Foundation::LUID, dynamicgroupargs: Option<*const core::ffi::c_void>, phauthzclientcontext: *mut AUTHZ_CLIENT_CONTEXT_HANDLE) -> windows_core::Result<()> {
    windows_core::link!("authz.dll" "system" fn AuthzInitializeContextFromToken(flags : u32, tokenhandle : super::super::Foundation:: HANDLE, hauthzresourcemanager : AUTHZ_RESOURCE_MANAGER_HANDLE, pexpirationtime : *const i64, identifier : super::super::Foundation:: LUID, dynamicgroupargs : *const core::ffi::c_void, phauthzclientcontext : *mut AUTHZ_CLIENT_CONTEXT_HANDLE) -> windows_core::BOOL);
    unsafe { AuthzInitializeContextFromToken(flags, tokenhandle, hauthzresourcemanager, pexpirationtime.unwrap_or(core::mem::zeroed()) as _, core::mem::transmute(identifier), dynamicgroupargs.unwrap_or(core::mem::zeroed()) as _, phauthzclientcontext as _).ok() }
}
#[inline]
pub unsafe fn AuthzInitializeObjectAccessAuditEvent<P2, P3, P4, P5>(flags: AUTHZ_INITIALIZE_OBJECT_ACCESS_AUDIT_EVENT_FLAGS, hauditeventtype: AUTHZ_AUDIT_EVENT_TYPE_HANDLE, szoperationtype: P2, szobjecttype: P3, szobjectname: P4, szadditionalinfo: P5, phauditevent: *mut AUTHZ_AUDIT_EVENT_HANDLE, dwadditionalparametercount: u32) -> windows_core::Result<()>
where
    P2: windows_core::Param<windows_core::PCWSTR>,
    P3: windows_core::Param<windows_core::PCWSTR>,
    P4: windows_core::Param<windows_core::PCWSTR>,
    P5: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("authz.dll" "C" fn AuthzInitializeObjectAccessAuditEvent(flags : AUTHZ_INITIALIZE_OBJECT_ACCESS_AUDIT_EVENT_FLAGS, hauditeventtype : AUTHZ_AUDIT_EVENT_TYPE_HANDLE, szoperationtype : windows_core::PCWSTR, szobjecttype : windows_core::PCWSTR, szobjectname : windows_core::PCWSTR, szadditionalinfo : windows_core::PCWSTR, phauditevent : *mut AUTHZ_AUDIT_EVENT_HANDLE, dwadditionalparametercount : u32) -> windows_core::BOOL);
    unsafe { AuthzInitializeObjectAccessAuditEvent(flags, hauditeventtype, szoperationtype.param().abi(), szobjecttype.param().abi(), szobjectname.param().abi(), szadditionalinfo.param().abi(), phauditevent as _, dwadditionalparametercount).ok() }
}
#[inline]
pub unsafe fn AuthzInitializeObjectAccessAuditEvent2<P2, P3, P4, P5, P6>(flags: u32, hauditeventtype: AUTHZ_AUDIT_EVENT_TYPE_HANDLE, szoperationtype: P2, szobjecttype: P3, szobjectname: P4, szadditionalinfo: P5, szadditionalinfo2: P6, phauditevent: *mut AUTHZ_AUDIT_EVENT_HANDLE, dwadditionalparametercount: u32) -> windows_core::Result<()>
where
    P2: windows_core::Param<windows_core::PCWSTR>,
    P3: windows_core::Param<windows_core::PCWSTR>,
    P4: windows_core::Param<windows_core::PCWSTR>,
    P5: windows_core::Param<windows_core::PCWSTR>,
    P6: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("authz.dll" "C" fn AuthzInitializeObjectAccessAuditEvent2(flags : u32, hauditeventtype : AUTHZ_AUDIT_EVENT_TYPE_HANDLE, szoperationtype : windows_core::PCWSTR, szobjecttype : windows_core::PCWSTR, szobjectname : windows_core::PCWSTR, szadditionalinfo : windows_core::PCWSTR, szadditionalinfo2 : windows_core::PCWSTR, phauditevent : *mut AUTHZ_AUDIT_EVENT_HANDLE, dwadditionalparametercount : u32) -> windows_core::BOOL);
    unsafe { AuthzInitializeObjectAccessAuditEvent2(flags, hauditeventtype, szoperationtype.param().abi(), szobjecttype.param().abi(), szobjectname.param().abi(), szadditionalinfo.param().abi(), szadditionalinfo2.param().abi(), phauditevent as _, dwadditionalparametercount).ok() }
}
#[inline]
pub unsafe fn AuthzInitializeRemoteResourceManager(prpcinitinfo: *const AUTHZ_RPC_INIT_INFO_CLIENT, phauthzresourcemanager: *mut AUTHZ_RESOURCE_MANAGER_HANDLE) -> windows_core::Result<()> {
    windows_core::link!("authz.dll" "system" fn AuthzInitializeRemoteResourceManager(prpcinitinfo : *const AUTHZ_RPC_INIT_INFO_CLIENT, phauthzresourcemanager : *mut AUTHZ_RESOURCE_MANAGER_HANDLE) -> windows_core::BOOL);
    unsafe { AuthzInitializeRemoteResourceManager(prpcinitinfo, phauthzresourcemanager as _).ok() }
}
#[inline]
pub unsafe fn AuthzInitializeResourceManager<P4>(flags: u32, pfndynamicaccesscheck: PFN_AUTHZ_DYNAMIC_ACCESS_CHECK, pfncomputedynamicgroups: PFN_AUTHZ_COMPUTE_DYNAMIC_GROUPS, pfnfreedynamicgroups: PFN_AUTHZ_FREE_DYNAMIC_GROUPS, szresourcemanagername: P4, phauthzresourcemanager: *mut AUTHZ_RESOURCE_MANAGER_HANDLE) -> windows_core::Result<()>
where
    P4: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("authz.dll" "system" fn AuthzInitializeResourceManager(flags : u32, pfndynamicaccesscheck : PFN_AUTHZ_DYNAMIC_ACCESS_CHECK, pfncomputedynamicgroups : PFN_AUTHZ_COMPUTE_DYNAMIC_GROUPS, pfnfreedynamicgroups : PFN_AUTHZ_FREE_DYNAMIC_GROUPS, szresourcemanagername : windows_core::PCWSTR, phauthzresourcemanager : *mut AUTHZ_RESOURCE_MANAGER_HANDLE) -> windows_core::BOOL);
    unsafe { AuthzInitializeResourceManager(flags, pfndynamicaccesscheck, pfncomputedynamicgroups, pfnfreedynamicgroups, szresourcemanagername.param().abi(), phauthzresourcemanager as _).ok() }
}
#[inline]
pub unsafe fn AuthzInitializeResourceManagerEx(flags: Option<AUTHZ_RESOURCE_MANAGER_FLAGS>, pauthzinitinfo: Option<*const AUTHZ_INIT_INFO>, phauthzresourcemanager: *mut AUTHZ_RESOURCE_MANAGER_HANDLE) -> windows_core::Result<()> {
    windows_core::link!("authz.dll" "system" fn AuthzInitializeResourceManagerEx(flags : AUTHZ_RESOURCE_MANAGER_FLAGS, pauthzinitinfo : *const AUTHZ_INIT_INFO, phauthzresourcemanager : *mut AUTHZ_RESOURCE_MANAGER_HANDLE) -> windows_core::BOOL);
    unsafe { AuthzInitializeResourceManagerEx(flags.unwrap_or(core::mem::zeroed()) as _, pauthzinitinfo.unwrap_or(core::mem::zeroed()) as _, phauthzresourcemanager as _).ok() }
}
#[inline]
pub unsafe fn AuthzInstallSecurityEventSource(dwflags: u32, pregistration: *const AUTHZ_SOURCE_SCHEMA_REGISTRATION) -> windows_core::Result<()> {
    windows_core::link!("authz.dll" "system" fn AuthzInstallSecurityEventSource(dwflags : u32, pregistration : *const AUTHZ_SOURCE_SCHEMA_REGISTRATION) -> windows_core::BOOL);
    unsafe { AuthzInstallSecurityEventSource(dwflags, pregistration).ok() }
}
#[inline]
pub unsafe fn AuthzModifyClaims(hauthzclientcontext: AUTHZ_CLIENT_CONTEXT_HANDLE, claimclass: AUTHZ_CONTEXT_INFORMATION_CLASS, pclaimoperations: *const AUTHZ_SECURITY_ATTRIBUTE_OPERATION, pclaims: Option<*const AUTHZ_SECURITY_ATTRIBUTES_INFORMATION>) -> windows_core::Result<()> {
    windows_core::link!("authz.dll" "system" fn AuthzModifyClaims(hauthzclientcontext : AUTHZ_CLIENT_CONTEXT_HANDLE, claimclass : AUTHZ_CONTEXT_INFORMATION_CLASS, pclaimoperations : *const AUTHZ_SECURITY_ATTRIBUTE_OPERATION, pclaims : *const AUTHZ_SECURITY_ATTRIBUTES_INFORMATION) -> windows_core::BOOL);
    unsafe { AuthzModifyClaims(hauthzclientcontext, claimclass, pclaimoperations, pclaims.unwrap_or(core::mem::zeroed()) as _).ok() }
}
#[inline]
pub unsafe fn AuthzModifySecurityAttributes(hauthzclientcontext: AUTHZ_CLIENT_CONTEXT_HANDLE, poperations: *const AUTHZ_SECURITY_ATTRIBUTE_OPERATION, pattributes: Option<*const AUTHZ_SECURITY_ATTRIBUTES_INFORMATION>) -> windows_core::Result<()> {
    windows_core::link!("authz.dll" "system" fn AuthzModifySecurityAttributes(hauthzclientcontext : AUTHZ_CLIENT_CONTEXT_HANDLE, poperations : *const AUTHZ_SECURITY_ATTRIBUTE_OPERATION, pattributes : *const AUTHZ_SECURITY_ATTRIBUTES_INFORMATION) -> windows_core::BOOL);
    unsafe { AuthzModifySecurityAttributes(hauthzclientcontext, poperations, pattributes.unwrap_or(core::mem::zeroed()) as _).ok() }
}
#[inline]
pub unsafe fn AuthzModifySids(hauthzclientcontext: AUTHZ_CLIENT_CONTEXT_HANDLE, sidclass: AUTHZ_CONTEXT_INFORMATION_CLASS, psidoperations: *const AUTHZ_SID_OPERATION, psids: Option<*const super::TOKEN_GROUPS>) -> windows_core::Result<()> {
    windows_core::link!("authz.dll" "system" fn AuthzModifySids(hauthzclientcontext : AUTHZ_CLIENT_CONTEXT_HANDLE, sidclass : AUTHZ_CONTEXT_INFORMATION_CLASS, psidoperations : *const AUTHZ_SID_OPERATION, psids : *const super:: TOKEN_GROUPS) -> windows_core::BOOL);
    unsafe { AuthzModifySids(hauthzclientcontext, sidclass, psidoperations, psids.unwrap_or(core::mem::zeroed()) as _).ok() }
}
#[inline]
pub unsafe fn AuthzOpenObjectAudit(flags: u32, hauthzclientcontext: AUTHZ_CLIENT_CONTEXT_HANDLE, prequest: *const AUTHZ_ACCESS_REQUEST, hauditevent: AUTHZ_AUDIT_EVENT_HANDLE, psecuritydescriptor: super::PSECURITY_DESCRIPTOR, optionalsecuritydescriptorarray: Option<&[super::PSECURITY_DESCRIPTOR]>, preply: *const AUTHZ_ACCESS_REPLY) -> windows_core::Result<()> {
    windows_core::link!("authz.dll" "system" fn AuthzOpenObjectAudit(flags : u32, hauthzclientcontext : AUTHZ_CLIENT_CONTEXT_HANDLE, prequest : *const AUTHZ_ACCESS_REQUEST, hauditevent : AUTHZ_AUDIT_EVENT_HANDLE, psecuritydescriptor : super:: PSECURITY_DESCRIPTOR, optionalsecuritydescriptorarray : *const super:: PSECURITY_DESCRIPTOR, optionalsecuritydescriptorcount : u32, preply : *const AUTHZ_ACCESS_REPLY) -> windows_core::BOOL);
    unsafe { AuthzOpenObjectAudit(flags, hauthzclientcontext, prequest, hauditevent, psecuritydescriptor, core::mem::transmute(optionalsecuritydescriptorarray.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), optionalsecuritydescriptorarray.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), preply).ok() }
}
#[cfg(feature = "Win32_System_Threading")]
#[inline]
pub unsafe fn AuthzRegisterCapChangeNotification(phcapchangesubscription: *mut AUTHZ_CAP_CHANGE_SUBSCRIPTION_HANDLE, pfncapchangecallback: super::super::System::Threading::LPTHREAD_START_ROUTINE, pcallbackcontext: Option<*const core::ffi::c_void>) -> windows_core::Result<()> {
    windows_core::link!("authz.dll" "system" fn AuthzRegisterCapChangeNotification(phcapchangesubscription : *mut AUTHZ_CAP_CHANGE_SUBSCRIPTION_HANDLE, pfncapchangecallback : super::super::System::Threading:: LPTHREAD_START_ROUTINE, pcallbackcontext : *const core::ffi::c_void) -> windows_core::BOOL);
    unsafe { AuthzRegisterCapChangeNotification(phcapchangesubscription as _, pfncapchangecallback, pcallbackcontext.unwrap_or(core::mem::zeroed()) as _).ok() }
}
#[inline]
pub unsafe fn AuthzRegisterSecurityEventSource<P1>(dwflags: u32, szeventsourcename: P1, pheventprovider: *mut AUTHZ_SECURITY_EVENT_PROVIDER_HANDLE) -> windows_core::Result<()>
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("authz.dll" "system" fn AuthzRegisterSecurityEventSource(dwflags : u32, szeventsourcename : windows_core::PCWSTR, pheventprovider : *mut AUTHZ_SECURITY_EVENT_PROVIDER_HANDLE) -> windows_core::BOOL);
    unsafe { AuthzRegisterSecurityEventSource(dwflags, szeventsourcename.param().abi(), pheventprovider as _).ok() }
}
#[inline]
pub unsafe fn AuthzReportSecurityEvent(dwflags: u32, heventprovider: AUTHZ_SECURITY_EVENT_PROVIDER_HANDLE, dwauditid: u32, pusersid: Option<super::PSID>, dwcount: u32) -> windows_core::Result<()> {
    windows_core::link!("authz.dll" "C" fn AuthzReportSecurityEvent(dwflags : u32, heventprovider : AUTHZ_SECURITY_EVENT_PROVIDER_HANDLE, dwauditid : u32, pusersid : super:: PSID, dwcount : u32) -> windows_core::BOOL);
    unsafe { AuthzReportSecurityEvent(dwflags, heventprovider as _, dwauditid, pusersid.unwrap_or(core::mem::zeroed()) as _, dwcount).ok() }
}
#[inline]
pub unsafe fn AuthzReportSecurityEventFromParams(dwflags: u32, heventprovider: AUTHZ_SECURITY_EVENT_PROVIDER_HANDLE, dwauditid: u32, pusersid: Option<super::PSID>, pparams: *const AUDIT_PARAMS) -> windows_core::Result<()> {
    windows_core::link!("authz.dll" "system" fn AuthzReportSecurityEventFromParams(dwflags : u32, heventprovider : AUTHZ_SECURITY_EVENT_PROVIDER_HANDLE, dwauditid : u32, pusersid : super:: PSID, pparams : *const AUDIT_PARAMS) -> windows_core::BOOL);
    unsafe { AuthzReportSecurityEventFromParams(dwflags, heventprovider as _, dwauditid, pusersid.unwrap_or(core::mem::zeroed()) as _, pparams).ok() }
}
#[inline]
pub unsafe fn AuthzSetAppContainerInformation(hauthzclientcontext: AUTHZ_CLIENT_CONTEXT_HANDLE, pappcontainersid: super::PSID, pcapabilitysids: Option<&[super::SID_AND_ATTRIBUTES]>) -> windows_core::Result<()> {
    windows_core::link!("authz.dll" "system" fn AuthzSetAppContainerInformation(hauthzclientcontext : AUTHZ_CLIENT_CONTEXT_HANDLE, pappcontainersid : super:: PSID, capabilitycount : u32, pcapabilitysids : *const super:: SID_AND_ATTRIBUTES) -> windows_core::BOOL);
    unsafe { AuthzSetAppContainerInformation(hauthzclientcontext, pappcontainersid, pcapabilitysids.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), core::mem::transmute(pcapabilitysids.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr()))).ok() }
}
#[inline]
pub unsafe fn AuthzUninstallSecurityEventSource<P1>(dwflags: u32, szeventsourcename: P1) -> windows_core::Result<()>
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("authz.dll" "system" fn AuthzUninstallSecurityEventSource(dwflags : u32, szeventsourcename : windows_core::PCWSTR) -> windows_core::BOOL);
    unsafe { AuthzUninstallSecurityEventSource(dwflags, szeventsourcename.param().abi()).ok() }
}
#[inline]
pub unsafe fn AuthzUnregisterCapChangeNotification(hcapchangesubscription: AUTHZ_CAP_CHANGE_SUBSCRIPTION_HANDLE) -> windows_core::Result<()> {
    windows_core::link!("authz.dll" "system" fn AuthzUnregisterCapChangeNotification(hcapchangesubscription : AUTHZ_CAP_CHANGE_SUBSCRIPTION_HANDLE) -> windows_core::BOOL);
    unsafe { AuthzUnregisterCapChangeNotification(hcapchangesubscription).ok() }
}
#[inline]
pub unsafe fn AuthzUnregisterSecurityEventSource(dwflags: u32, pheventprovider: *mut AUTHZ_SECURITY_EVENT_PROVIDER_HANDLE) -> windows_core::Result<()> {
    windows_core::link!("authz.dll" "system" fn AuthzUnregisterSecurityEventSource(dwflags : u32, pheventprovider : *mut AUTHZ_SECURITY_EVENT_PROVIDER_HANDLE) -> windows_core::BOOL);
    unsafe { AuthzUnregisterSecurityEventSource(dwflags, pheventprovider as _).ok() }
}
#[inline]
pub unsafe fn BuildExplicitAccessWithNameA<P1>(pexplicitaccess: *mut EXPLICIT_ACCESS_A, ptrusteename: P1, accesspermissions: u32, accessmode: ACCESS_MODE, inheritance: super::ACE_FLAGS)
where
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("advapi32.dll" "system" fn BuildExplicitAccessWithNameA(pexplicitaccess : *mut EXPLICIT_ACCESS_A, ptrusteename : windows_core::PCSTR, accesspermissions : u32, accessmode : ACCESS_MODE, inheritance : super:: ACE_FLAGS));
    unsafe { BuildExplicitAccessWithNameA(pexplicitaccess as _, ptrusteename.param().abi(), accesspermissions, accessmode, inheritance) }
}
#[inline]
pub unsafe fn BuildExplicitAccessWithNameW<P1>(pexplicitaccess: *mut EXPLICIT_ACCESS_W, ptrusteename: P1, accesspermissions: u32, accessmode: ACCESS_MODE, inheritance: super::ACE_FLAGS)
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("advapi32.dll" "system" fn BuildExplicitAccessWithNameW(pexplicitaccess : *mut EXPLICIT_ACCESS_W, ptrusteename : windows_core::PCWSTR, accesspermissions : u32, accessmode : ACCESS_MODE, inheritance : super:: ACE_FLAGS));
    unsafe { BuildExplicitAccessWithNameW(pexplicitaccess as _, ptrusteename.param().abi(), accesspermissions, accessmode, inheritance) }
}
#[inline]
pub unsafe fn BuildImpersonateExplicitAccessWithNameA<P1>(pexplicitaccess: *mut EXPLICIT_ACCESS_A, ptrusteename: P1, ptrustee: Option<*const TRUSTEE_A>, accesspermissions: u32, accessmode: ACCESS_MODE, inheritance: u32)
where
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("advapi32.dll" "system" fn BuildImpersonateExplicitAccessWithNameA(pexplicitaccess : *mut EXPLICIT_ACCESS_A, ptrusteename : windows_core::PCSTR, ptrustee : *const TRUSTEE_A, accesspermissions : u32, accessmode : ACCESS_MODE, inheritance : u32));
    unsafe { BuildImpersonateExplicitAccessWithNameA(pexplicitaccess as _, ptrusteename.param().abi(), ptrustee.unwrap_or(core::mem::zeroed()) as _, accesspermissions, accessmode, inheritance) }
}
#[inline]
pub unsafe fn BuildImpersonateExplicitAccessWithNameW<P1>(pexplicitaccess: *mut EXPLICIT_ACCESS_W, ptrusteename: P1, ptrustee: Option<*const TRUSTEE_W>, accesspermissions: u32, accessmode: ACCESS_MODE, inheritance: u32)
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("advapi32.dll" "system" fn BuildImpersonateExplicitAccessWithNameW(pexplicitaccess : *mut EXPLICIT_ACCESS_W, ptrusteename : windows_core::PCWSTR, ptrustee : *const TRUSTEE_W, accesspermissions : u32, accessmode : ACCESS_MODE, inheritance : u32));
    unsafe { BuildImpersonateExplicitAccessWithNameW(pexplicitaccess as _, ptrusteename.param().abi(), ptrustee.unwrap_or(core::mem::zeroed()) as _, accesspermissions, accessmode, inheritance) }
}
#[inline]
pub unsafe fn BuildImpersonateTrusteeA(ptrustee: *mut TRUSTEE_A, pimpersonatetrustee: Option<*const TRUSTEE_A>) {
    windows_core::link!("advapi32.dll" "system" fn BuildImpersonateTrusteeA(ptrustee : *mut TRUSTEE_A, pimpersonatetrustee : *const TRUSTEE_A));
    unsafe { BuildImpersonateTrusteeA(ptrustee as _, pimpersonatetrustee.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn BuildImpersonateTrusteeW(ptrustee: *mut TRUSTEE_W, pimpersonatetrustee: Option<*const TRUSTEE_W>) {
    windows_core::link!("advapi32.dll" "system" fn BuildImpersonateTrusteeW(ptrustee : *mut TRUSTEE_W, pimpersonatetrustee : *const TRUSTEE_W));
    unsafe { BuildImpersonateTrusteeW(ptrustee as _, pimpersonatetrustee.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn BuildSecurityDescriptorA(powner: Option<*const TRUSTEE_A>, pgroup: Option<*const TRUSTEE_A>, plistofaccessentries: Option<&[EXPLICIT_ACCESS_A]>, plistofauditentries: Option<&[EXPLICIT_ACCESS_A]>, poldsd: Option<super::PSECURITY_DESCRIPTOR>, psizenewsd: *mut u32, pnewsd: *mut super::PSECURITY_DESCRIPTOR) -> super::super::Foundation::WIN32_ERROR {
    windows_core::link!("advapi32.dll" "system" fn BuildSecurityDescriptorA(powner : *const TRUSTEE_A, pgroup : *const TRUSTEE_A, ccountofaccessentries : u32, plistofaccessentries : *const EXPLICIT_ACCESS_A, ccountofauditentries : u32, plistofauditentries : *const EXPLICIT_ACCESS_A, poldsd : super:: PSECURITY_DESCRIPTOR, psizenewsd : *mut u32, pnewsd : *mut super:: PSECURITY_DESCRIPTOR) -> super::super::Foundation:: WIN32_ERROR);
    unsafe {
        BuildSecurityDescriptorA(
            powner.unwrap_or(core::mem::zeroed()) as _,
            pgroup.unwrap_or(core::mem::zeroed()) as _,
            plistofaccessentries.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()),
            core::mem::transmute(plistofaccessentries.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())),
            plistofauditentries.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()),
            core::mem::transmute(plistofauditentries.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())),
            poldsd.unwrap_or(core::mem::zeroed()) as _,
            psizenewsd as _,
            pnewsd as _,
        )
    }
}
#[inline]
pub unsafe fn BuildSecurityDescriptorW(powner: Option<*const TRUSTEE_W>, pgroup: Option<*const TRUSTEE_W>, plistofaccessentries: Option<&[EXPLICIT_ACCESS_W]>, plistofauditentries: Option<&[EXPLICIT_ACCESS_W]>, poldsd: Option<super::PSECURITY_DESCRIPTOR>, psizenewsd: *mut u32, pnewsd: *mut super::PSECURITY_DESCRIPTOR) -> super::super::Foundation::WIN32_ERROR {
    windows_core::link!("advapi32.dll" "system" fn BuildSecurityDescriptorW(powner : *const TRUSTEE_W, pgroup : *const TRUSTEE_W, ccountofaccessentries : u32, plistofaccessentries : *const EXPLICIT_ACCESS_W, ccountofauditentries : u32, plistofauditentries : *const EXPLICIT_ACCESS_W, poldsd : super:: PSECURITY_DESCRIPTOR, psizenewsd : *mut u32, pnewsd : *mut super:: PSECURITY_DESCRIPTOR) -> super::super::Foundation:: WIN32_ERROR);
    unsafe {
        BuildSecurityDescriptorW(
            powner.unwrap_or(core::mem::zeroed()) as _,
            pgroup.unwrap_or(core::mem::zeroed()) as _,
            plistofaccessentries.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()),
            core::mem::transmute(plistofaccessentries.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())),
            plistofauditentries.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()),
            core::mem::transmute(plistofauditentries.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())),
            poldsd.unwrap_or(core::mem::zeroed()) as _,
            psizenewsd as _,
            pnewsd as _,
        )
    }
}
#[inline]
pub unsafe fn BuildTrusteeWithNameA<P1>(ptrustee: *mut TRUSTEE_A, pname: P1)
where
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("advapi32.dll" "system" fn BuildTrusteeWithNameA(ptrustee : *mut TRUSTEE_A, pname : windows_core::PCSTR));
    unsafe { BuildTrusteeWithNameA(ptrustee as _, pname.param().abi()) }
}
#[inline]
pub unsafe fn BuildTrusteeWithNameW<P1>(ptrustee: *mut TRUSTEE_W, pname: P1)
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("advapi32.dll" "system" fn BuildTrusteeWithNameW(ptrustee : *mut TRUSTEE_W, pname : windows_core::PCWSTR));
    unsafe { BuildTrusteeWithNameW(ptrustee as _, pname.param().abi()) }
}
#[inline]
pub unsafe fn BuildTrusteeWithObjectsAndNameA<P3, P4, P5>(ptrustee: *mut TRUSTEE_A, pobjname: Option<*const OBJECTS_AND_NAME_A>, objecttype: Option<SE_OBJECT_TYPE>, objecttypename: P3, inheritedobjecttypename: P4, name: P5)
where
    P3: windows_core::Param<windows_core::PCSTR>,
    P4: windows_core::Param<windows_core::PCSTR>,
    P5: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("advapi32.dll" "system" fn BuildTrusteeWithObjectsAndNameA(ptrustee : *mut TRUSTEE_A, pobjname : *const OBJECTS_AND_NAME_A, objecttype : SE_OBJECT_TYPE, objecttypename : windows_core::PCSTR, inheritedobjecttypename : windows_core::PCSTR, name : windows_core::PCSTR));
    unsafe { BuildTrusteeWithObjectsAndNameA(ptrustee as _, pobjname.unwrap_or(core::mem::zeroed()) as _, objecttype.unwrap_or(core::mem::zeroed()) as _, objecttypename.param().abi(), inheritedobjecttypename.param().abi(), name.param().abi()) }
}
#[inline]
pub unsafe fn BuildTrusteeWithObjectsAndNameW<P3, P4, P5>(ptrustee: *mut TRUSTEE_W, pobjname: Option<*const OBJECTS_AND_NAME_W>, objecttype: Option<SE_OBJECT_TYPE>, objecttypename: P3, inheritedobjecttypename: P4, name: P5)
where
    P3: windows_core::Param<windows_core::PCWSTR>,
    P4: windows_core::Param<windows_core::PCWSTR>,
    P5: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("advapi32.dll" "system" fn BuildTrusteeWithObjectsAndNameW(ptrustee : *mut TRUSTEE_W, pobjname : *const OBJECTS_AND_NAME_W, objecttype : SE_OBJECT_TYPE, objecttypename : windows_core::PCWSTR, inheritedobjecttypename : windows_core::PCWSTR, name : windows_core::PCWSTR));
    unsafe { BuildTrusteeWithObjectsAndNameW(ptrustee as _, pobjname.unwrap_or(core::mem::zeroed()) as _, objecttype.unwrap_or(core::mem::zeroed()) as _, objecttypename.param().abi(), inheritedobjecttypename.param().abi(), name.param().abi()) }
}
#[inline]
pub unsafe fn BuildTrusteeWithObjectsAndSidA(ptrustee: *mut TRUSTEE_A, pobjsid: Option<*const OBJECTS_AND_SID>, pobjectguid: Option<*const windows_core::GUID>, pinheritedobjectguid: Option<*const windows_core::GUID>, psid: Option<super::PSID>) {
    windows_core::link!("advapi32.dll" "system" fn BuildTrusteeWithObjectsAndSidA(ptrustee : *mut TRUSTEE_A, pobjsid : *const OBJECTS_AND_SID, pobjectguid : *const windows_core::GUID, pinheritedobjectguid : *const windows_core::GUID, psid : super:: PSID));
    unsafe { BuildTrusteeWithObjectsAndSidA(ptrustee as _, pobjsid.unwrap_or(core::mem::zeroed()) as _, pobjectguid.unwrap_or(core::mem::zeroed()) as _, pinheritedobjectguid.unwrap_or(core::mem::zeroed()) as _, psid.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn BuildTrusteeWithObjectsAndSidW(ptrustee: *mut TRUSTEE_W, pobjsid: Option<*const OBJECTS_AND_SID>, pobjectguid: Option<*const windows_core::GUID>, pinheritedobjectguid: Option<*const windows_core::GUID>, psid: Option<super::PSID>) {
    windows_core::link!("advapi32.dll" "system" fn BuildTrusteeWithObjectsAndSidW(ptrustee : *mut TRUSTEE_W, pobjsid : *const OBJECTS_AND_SID, pobjectguid : *const windows_core::GUID, pinheritedobjectguid : *const windows_core::GUID, psid : super:: PSID));
    unsafe { BuildTrusteeWithObjectsAndSidW(ptrustee as _, pobjsid.unwrap_or(core::mem::zeroed()) as _, pobjectguid.unwrap_or(core::mem::zeroed()) as _, pinheritedobjectguid.unwrap_or(core::mem::zeroed()) as _, psid.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn BuildTrusteeWithSidA(ptrustee: *mut TRUSTEE_A, psid: Option<super::PSID>) {
    windows_core::link!("advapi32.dll" "system" fn BuildTrusteeWithSidA(ptrustee : *mut TRUSTEE_A, psid : super:: PSID));
    unsafe { BuildTrusteeWithSidA(ptrustee as _, psid.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn BuildTrusteeWithSidW(ptrustee: *mut TRUSTEE_W, psid: Option<super::PSID>) {
    windows_core::link!("advapi32.dll" "system" fn BuildTrusteeWithSidW(ptrustee : *mut TRUSTEE_W, psid : super:: PSID));
    unsafe { BuildTrusteeWithSidW(ptrustee as _, psid.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn ConvertSecurityDescriptorToStringSecurityDescriptorA(securitydescriptor: super::PSECURITY_DESCRIPTOR, requestedstringsdrevision: u32, securityinformation: super::OBJECT_SECURITY_INFORMATION, stringsecuritydescriptor: *mut windows_core::PSTR, stringsecuritydescriptorlen: Option<*mut u32>) -> windows_core::Result<()> {
    windows_core::link!("advapi32.dll" "system" fn ConvertSecurityDescriptorToStringSecurityDescriptorA(securitydescriptor : super:: PSECURITY_DESCRIPTOR, requestedstringsdrevision : u32, securityinformation : super:: OBJECT_SECURITY_INFORMATION, stringsecuritydescriptor : *mut windows_core::PSTR, stringsecuritydescriptorlen : *mut u32) -> windows_core::BOOL);
    unsafe { ConvertSecurityDescriptorToStringSecurityDescriptorA(securitydescriptor, requestedstringsdrevision, securityinformation, stringsecuritydescriptor as _, stringsecuritydescriptorlen.unwrap_or(core::mem::zeroed()) as _).ok() }
}
#[inline]
pub unsafe fn ConvertSecurityDescriptorToStringSecurityDescriptorW(securitydescriptor: super::PSECURITY_DESCRIPTOR, requestedstringsdrevision: u32, securityinformation: super::OBJECT_SECURITY_INFORMATION, stringsecuritydescriptor: *mut windows_core::PWSTR, stringsecuritydescriptorlen: Option<*mut u32>) -> windows_core::Result<()> {
    windows_core::link!("advapi32.dll" "system" fn ConvertSecurityDescriptorToStringSecurityDescriptorW(securitydescriptor : super:: PSECURITY_DESCRIPTOR, requestedstringsdrevision : u32, securityinformation : super:: OBJECT_SECURITY_INFORMATION, stringsecuritydescriptor : *mut windows_core::PWSTR, stringsecuritydescriptorlen : *mut u32) -> windows_core::BOOL);
    unsafe { ConvertSecurityDescriptorToStringSecurityDescriptorW(securitydescriptor, requestedstringsdrevision, securityinformation, stringsecuritydescriptor as _, stringsecuritydescriptorlen.unwrap_or(core::mem::zeroed()) as _).ok() }
}
#[inline]
pub unsafe fn ConvertSidToStringSidA(sid: super::PSID, stringsid: *mut windows_core::PSTR) -> windows_core::Result<()> {
    windows_core::link!("advapi32.dll" "system" fn ConvertSidToStringSidA(sid : super:: PSID, stringsid : *mut windows_core::PSTR) -> windows_core::BOOL);
    unsafe { ConvertSidToStringSidA(sid, stringsid as _).ok() }
}
#[inline]
pub unsafe fn ConvertSidToStringSidW(sid: super::PSID, stringsid: *mut windows_core::PWSTR) -> windows_core::Result<()> {
    windows_core::link!("advapi32.dll" "system" fn ConvertSidToStringSidW(sid : super:: PSID, stringsid : *mut windows_core::PWSTR) -> windows_core::BOOL);
    unsafe { ConvertSidToStringSidW(sid, stringsid as _).ok() }
}
#[inline]
pub unsafe fn ConvertStringSecurityDescriptorToSecurityDescriptorA<P0>(stringsecuritydescriptor: P0, stringsdrevision: u32, securitydescriptor: *mut super::PSECURITY_DESCRIPTOR, securitydescriptorsize: Option<*mut u32>) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("advapi32.dll" "system" fn ConvertStringSecurityDescriptorToSecurityDescriptorA(stringsecuritydescriptor : windows_core::PCSTR, stringsdrevision : u32, securitydescriptor : *mut super:: PSECURITY_DESCRIPTOR, securitydescriptorsize : *mut u32) -> windows_core::BOOL);
    unsafe { ConvertStringSecurityDescriptorToSecurityDescriptorA(stringsecuritydescriptor.param().abi(), stringsdrevision, securitydescriptor as _, securitydescriptorsize.unwrap_or(core::mem::zeroed()) as _).ok() }
}
#[inline]
pub unsafe fn ConvertStringSecurityDescriptorToSecurityDescriptorW<P0>(stringsecuritydescriptor: P0, stringsdrevision: u32, securitydescriptor: *mut super::PSECURITY_DESCRIPTOR, securitydescriptorsize: Option<*mut u32>) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("advapi32.dll" "system" fn ConvertStringSecurityDescriptorToSecurityDescriptorW(stringsecuritydescriptor : windows_core::PCWSTR, stringsdrevision : u32, securitydescriptor : *mut super:: PSECURITY_DESCRIPTOR, securitydescriptorsize : *mut u32) -> windows_core::BOOL);
    unsafe { ConvertStringSecurityDescriptorToSecurityDescriptorW(stringsecuritydescriptor.param().abi(), stringsdrevision, securitydescriptor as _, securitydescriptorsize.unwrap_or(core::mem::zeroed()) as _).ok() }
}
#[inline]
pub unsafe fn ConvertStringSidToSidA<P0>(stringsid: P0, sid: *mut super::PSID) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("advapi32.dll" "system" fn ConvertStringSidToSidA(stringsid : windows_core::PCSTR, sid : *mut super:: PSID) -> windows_core::BOOL);
    unsafe { ConvertStringSidToSidA(stringsid.param().abi(), sid as _).ok() }
}
#[inline]
pub unsafe fn ConvertStringSidToSidW<P0>(stringsid: P0, sid: *mut super::PSID) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("advapi32.dll" "system" fn ConvertStringSidToSidW(stringsid : windows_core::PCWSTR, sid : *mut super:: PSID) -> windows_core::BOOL);
    unsafe { ConvertStringSidToSidW(stringsid.param().abi(), sid as _).ok() }
}
#[inline]
pub unsafe fn FreeInheritedFromArray(pinheritarray: &[INHERITED_FROMW], pfnarray: Option<*const FN_OBJECT_MGR_FUNCTS>) -> super::super::Foundation::WIN32_ERROR {
    windows_core::link!("advapi32.dll" "system" fn FreeInheritedFromArray(pinheritarray : *const INHERITED_FROMW, acecnt : u16, pfnarray : *const FN_OBJECT_MGR_FUNCTS) -> super::super::Foundation:: WIN32_ERROR);
    unsafe { FreeInheritedFromArray(core::mem::transmute(pinheritarray.as_ptr()), pinheritarray.len().try_into().unwrap(), pfnarray.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn GetAuditedPermissionsFromAclA(pacl: *const super::ACL, ptrustee: *const TRUSTEE_A, psuccessfulauditedrights: *mut u32, pfailedauditrights: *mut u32) -> super::super::Foundation::WIN32_ERROR {
    windows_core::link!("advapi32.dll" "system" fn GetAuditedPermissionsFromAclA(pacl : *const super:: ACL, ptrustee : *const TRUSTEE_A, psuccessfulauditedrights : *mut u32, pfailedauditrights : *mut u32) -> super::super::Foundation:: WIN32_ERROR);
    unsafe { GetAuditedPermissionsFromAclA(pacl, ptrustee, psuccessfulauditedrights as _, pfailedauditrights as _) }
}
#[inline]
pub unsafe fn GetAuditedPermissionsFromAclW(pacl: *const super::ACL, ptrustee: *const TRUSTEE_W, psuccessfulauditedrights: *mut u32, pfailedauditrights: *mut u32) -> super::super::Foundation::WIN32_ERROR {
    windows_core::link!("advapi32.dll" "system" fn GetAuditedPermissionsFromAclW(pacl : *const super:: ACL, ptrustee : *const TRUSTEE_W, psuccessfulauditedrights : *mut u32, pfailedauditrights : *mut u32) -> super::super::Foundation:: WIN32_ERROR);
    unsafe { GetAuditedPermissionsFromAclW(pacl, ptrustee, psuccessfulauditedrights as _, pfailedauditrights as _) }
}
#[inline]
pub unsafe fn GetEffectiveRightsFromAclA(pacl: *const super::ACL, ptrustee: *const TRUSTEE_A, paccessrights: *mut u32) -> super::super::Foundation::WIN32_ERROR {
    windows_core::link!("advapi32.dll" "system" fn GetEffectiveRightsFromAclA(pacl : *const super:: ACL, ptrustee : *const TRUSTEE_A, paccessrights : *mut u32) -> super::super::Foundation:: WIN32_ERROR);
    unsafe { GetEffectiveRightsFromAclA(pacl, ptrustee, paccessrights as _) }
}
#[inline]
pub unsafe fn GetEffectiveRightsFromAclW(pacl: *const super::ACL, ptrustee: *const TRUSTEE_W, paccessrights: *mut u32) -> super::super::Foundation::WIN32_ERROR {
    windows_core::link!("advapi32.dll" "system" fn GetEffectiveRightsFromAclW(pacl : *const super:: ACL, ptrustee : *const TRUSTEE_W, paccessrights : *mut u32) -> super::super::Foundation:: WIN32_ERROR);
    unsafe { GetEffectiveRightsFromAclW(pacl, ptrustee, paccessrights as _) }
}
#[inline]
pub unsafe fn GetExplicitEntriesFromAclA(pacl: *const super::ACL, pccountofexplicitentries: *mut u32, plistofexplicitentries: *mut *mut EXPLICIT_ACCESS_A) -> super::super::Foundation::WIN32_ERROR {
    windows_core::link!("advapi32.dll" "system" fn GetExplicitEntriesFromAclA(pacl : *const super:: ACL, pccountofexplicitentries : *mut u32, plistofexplicitentries : *mut *mut EXPLICIT_ACCESS_A) -> super::super::Foundation:: WIN32_ERROR);
    unsafe { GetExplicitEntriesFromAclA(pacl, pccountofexplicitentries as _, plistofexplicitentries as _) }
}
#[inline]
pub unsafe fn GetExplicitEntriesFromAclW(pacl: *const super::ACL, pccountofexplicitentries: *mut u32, plistofexplicitentries: *mut *mut EXPLICIT_ACCESS_W) -> super::super::Foundation::WIN32_ERROR {
    windows_core::link!("advapi32.dll" "system" fn GetExplicitEntriesFromAclW(pacl : *const super:: ACL, pccountofexplicitentries : *mut u32, plistofexplicitentries : *mut *mut EXPLICIT_ACCESS_W) -> super::super::Foundation:: WIN32_ERROR);
    unsafe { GetExplicitEntriesFromAclW(pacl, pccountofexplicitentries as _, plistofexplicitentries as _) }
}
#[inline]
pub unsafe fn GetInheritanceSourceA<P0>(pobjectname: P0, objecttype: SE_OBJECT_TYPE, securityinfo: super::OBJECT_SECURITY_INFORMATION, container: bool, pobjectclassguids: Option<&[*const windows_core::GUID]>, pacl: *const super::ACL, pfnarray: Option<*const FN_OBJECT_MGR_FUNCTS>, pgenericmapping: *const super::GENERIC_MAPPING, pinheritarray: *mut INHERITED_FROMA) -> super::super::Foundation::WIN32_ERROR
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("advapi32.dll" "system" fn GetInheritanceSourceA(pobjectname : windows_core::PCSTR, objecttype : SE_OBJECT_TYPE, securityinfo : super:: OBJECT_SECURITY_INFORMATION, container : windows_core::BOOL, pobjectclassguids : *const *const windows_core::GUID, guidcount : u32, pacl : *const super:: ACL, pfnarray : *const FN_OBJECT_MGR_FUNCTS, pgenericmapping : *const super:: GENERIC_MAPPING, pinheritarray : *mut INHERITED_FROMA) -> super::super::Foundation:: WIN32_ERROR);
    unsafe { GetInheritanceSourceA(pobjectname.param().abi(), objecttype, securityinfo, container.into(), core::mem::transmute(pobjectclassguids.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), pobjectclassguids.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), pacl, pfnarray.unwrap_or(core::mem::zeroed()) as _, pgenericmapping, pinheritarray as _) }
}
#[inline]
pub unsafe fn GetInheritanceSourceW<P0>(pobjectname: P0, objecttype: SE_OBJECT_TYPE, securityinfo: super::OBJECT_SECURITY_INFORMATION, container: bool, pobjectclassguids: Option<&[*const windows_core::GUID]>, pacl: *const super::ACL, pfnarray: Option<*const FN_OBJECT_MGR_FUNCTS>, pgenericmapping: *const super::GENERIC_MAPPING, pinheritarray: *mut INHERITED_FROMW) -> super::super::Foundation::WIN32_ERROR
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("advapi32.dll" "system" fn GetInheritanceSourceW(pobjectname : windows_core::PCWSTR, objecttype : SE_OBJECT_TYPE, securityinfo : super:: OBJECT_SECURITY_INFORMATION, container : windows_core::BOOL, pobjectclassguids : *const *const windows_core::GUID, guidcount : u32, pacl : *const super:: ACL, pfnarray : *const FN_OBJECT_MGR_FUNCTS, pgenericmapping : *const super:: GENERIC_MAPPING, pinheritarray : *mut INHERITED_FROMW) -> super::super::Foundation:: WIN32_ERROR);
    unsafe { GetInheritanceSourceW(pobjectname.param().abi(), objecttype, securityinfo, container.into(), core::mem::transmute(pobjectclassguids.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), pobjectclassguids.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), pacl, pfnarray.unwrap_or(core::mem::zeroed()) as _, pgenericmapping, pinheritarray as _) }
}
#[inline]
pub unsafe fn GetMultipleTrusteeA(ptrustee: Option<*const TRUSTEE_A>) -> *mut TRUSTEE_A {
    windows_core::link!("advapi32.dll" "system" fn GetMultipleTrusteeA(ptrustee : *const TRUSTEE_A) -> *mut TRUSTEE_A);
    unsafe { GetMultipleTrusteeA(ptrustee.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn GetMultipleTrusteeOperationA(ptrustee: Option<*const TRUSTEE_A>) -> MULTIPLE_TRUSTEE_OPERATION {
    windows_core::link!("advapi32.dll" "system" fn GetMultipleTrusteeOperationA(ptrustee : *const TRUSTEE_A) -> MULTIPLE_TRUSTEE_OPERATION);
    unsafe { GetMultipleTrusteeOperationA(ptrustee.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn GetMultipleTrusteeOperationW(ptrustee: Option<*const TRUSTEE_W>) -> MULTIPLE_TRUSTEE_OPERATION {
    windows_core::link!("advapi32.dll" "system" fn GetMultipleTrusteeOperationW(ptrustee : *const TRUSTEE_W) -> MULTIPLE_TRUSTEE_OPERATION);
    unsafe { GetMultipleTrusteeOperationW(ptrustee.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn GetMultipleTrusteeW(ptrustee: Option<*const TRUSTEE_W>) -> *mut TRUSTEE_W {
    windows_core::link!("advapi32.dll" "system" fn GetMultipleTrusteeW(ptrustee : *const TRUSTEE_W) -> *mut TRUSTEE_W);
    unsafe { GetMultipleTrusteeW(ptrustee.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn GetNamedSecurityInfoA<P0>(pobjectname: P0, objecttype: SE_OBJECT_TYPE, securityinfo: super::OBJECT_SECURITY_INFORMATION, ppsidowner: Option<*mut super::PSID>, ppsidgroup: Option<*mut super::PSID>, ppdacl: Option<*mut *mut super::ACL>, ppsacl: Option<*mut *mut super::ACL>, ppsecuritydescriptor: *mut super::PSECURITY_DESCRIPTOR) -> super::super::Foundation::WIN32_ERROR
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("advapi32.dll" "system" fn GetNamedSecurityInfoA(pobjectname : windows_core::PCSTR, objecttype : SE_OBJECT_TYPE, securityinfo : super:: OBJECT_SECURITY_INFORMATION, ppsidowner : *mut super:: PSID, ppsidgroup : *mut super:: PSID, ppdacl : *mut *mut super:: ACL, ppsacl : *mut *mut super:: ACL, ppsecuritydescriptor : *mut super:: PSECURITY_DESCRIPTOR) -> super::super::Foundation:: WIN32_ERROR);
    unsafe { GetNamedSecurityInfoA(pobjectname.param().abi(), objecttype, securityinfo, ppsidowner.unwrap_or(core::mem::zeroed()) as _, ppsidgroup.unwrap_or(core::mem::zeroed()) as _, ppdacl.unwrap_or(core::mem::zeroed()) as _, ppsacl.unwrap_or(core::mem::zeroed()) as _, ppsecuritydescriptor as _) }
}
#[inline]
pub unsafe fn GetNamedSecurityInfoW<P0>(pobjectname: P0, objecttype: SE_OBJECT_TYPE, securityinfo: super::OBJECT_SECURITY_INFORMATION, ppsidowner: Option<*mut super::PSID>, ppsidgroup: Option<*mut super::PSID>, ppdacl: Option<*mut *mut super::ACL>, ppsacl: Option<*mut *mut super::ACL>, ppsecuritydescriptor: *mut super::PSECURITY_DESCRIPTOR) -> super::super::Foundation::WIN32_ERROR
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("advapi32.dll" "system" fn GetNamedSecurityInfoW(pobjectname : windows_core::PCWSTR, objecttype : SE_OBJECT_TYPE, securityinfo : super:: OBJECT_SECURITY_INFORMATION, ppsidowner : *mut super:: PSID, ppsidgroup : *mut super:: PSID, ppdacl : *mut *mut super:: ACL, ppsacl : *mut *mut super:: ACL, ppsecuritydescriptor : *mut super:: PSECURITY_DESCRIPTOR) -> super::super::Foundation:: WIN32_ERROR);
    unsafe { GetNamedSecurityInfoW(pobjectname.param().abi(), objecttype, securityinfo, ppsidowner.unwrap_or(core::mem::zeroed()) as _, ppsidgroup.unwrap_or(core::mem::zeroed()) as _, ppdacl.unwrap_or(core::mem::zeroed()) as _, ppsacl.unwrap_or(core::mem::zeroed()) as _, ppsecuritydescriptor as _) }
}
#[inline]
pub unsafe fn GetSecurityInfo(handle: super::super::Foundation::HANDLE, objecttype: SE_OBJECT_TYPE, securityinfo: super::OBJECT_SECURITY_INFORMATION, ppsidowner: Option<*mut super::PSID>, ppsidgroup: Option<*mut super::PSID>, ppdacl: Option<*mut *mut super::ACL>, ppsacl: Option<*mut *mut super::ACL>, ppsecuritydescriptor: Option<*mut super::PSECURITY_DESCRIPTOR>) -> super::super::Foundation::WIN32_ERROR {
    windows_core::link!("advapi32.dll" "system" fn GetSecurityInfo(handle : super::super::Foundation:: HANDLE, objecttype : SE_OBJECT_TYPE, securityinfo : super:: OBJECT_SECURITY_INFORMATION, ppsidowner : *mut super:: PSID, ppsidgroup : *mut super:: PSID, ppdacl : *mut *mut super:: ACL, ppsacl : *mut *mut super:: ACL, ppsecuritydescriptor : *mut super:: PSECURITY_DESCRIPTOR) -> super::super::Foundation:: WIN32_ERROR);
    unsafe { GetSecurityInfo(handle, objecttype, securityinfo, ppsidowner.unwrap_or(core::mem::zeroed()) as _, ppsidgroup.unwrap_or(core::mem::zeroed()) as _, ppdacl.unwrap_or(core::mem::zeroed()) as _, ppsacl.unwrap_or(core::mem::zeroed()) as _, ppsecuritydescriptor.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn GetTrusteeFormA(ptrustee: *const TRUSTEE_A) -> TRUSTEE_FORM {
    windows_core::link!("advapi32.dll" "system" fn GetTrusteeFormA(ptrustee : *const TRUSTEE_A) -> TRUSTEE_FORM);
    unsafe { GetTrusteeFormA(ptrustee) }
}
#[inline]
pub unsafe fn GetTrusteeFormW(ptrustee: *const TRUSTEE_W) -> TRUSTEE_FORM {
    windows_core::link!("advapi32.dll" "system" fn GetTrusteeFormW(ptrustee : *const TRUSTEE_W) -> TRUSTEE_FORM);
    unsafe { GetTrusteeFormW(ptrustee) }
}
#[inline]
pub unsafe fn GetTrusteeNameA(ptrustee: *const TRUSTEE_A) -> windows_core::PSTR {
    windows_core::link!("advapi32.dll" "system" fn GetTrusteeNameA(ptrustee : *const TRUSTEE_A) -> windows_core::PSTR);
    unsafe { GetTrusteeNameA(ptrustee) }
}
#[inline]
pub unsafe fn GetTrusteeNameW(ptrustee: *const TRUSTEE_W) -> windows_core::PWSTR {
    windows_core::link!("advapi32.dll" "system" fn GetTrusteeNameW(ptrustee : *const TRUSTEE_W) -> windows_core::PWSTR);
    unsafe { GetTrusteeNameW(ptrustee) }
}
#[inline]
pub unsafe fn GetTrusteeTypeA(ptrustee: Option<*const TRUSTEE_A>) -> TRUSTEE_TYPE {
    windows_core::link!("advapi32.dll" "system" fn GetTrusteeTypeA(ptrustee : *const TRUSTEE_A) -> TRUSTEE_TYPE);
    unsafe { GetTrusteeTypeA(ptrustee.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn GetTrusteeTypeW(ptrustee: Option<*const TRUSTEE_W>) -> TRUSTEE_TYPE {
    windows_core::link!("advapi32.dll" "system" fn GetTrusteeTypeW(ptrustee : *const TRUSTEE_W) -> TRUSTEE_TYPE);
    unsafe { GetTrusteeTypeW(ptrustee.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn LookupSecurityDescriptorPartsA(ppowner: Option<*mut *mut TRUSTEE_A>, ppgroup: Option<*mut *mut TRUSTEE_A>, pccountofaccessentries: Option<*mut u32>, pplistofaccessentries: *mut *mut EXPLICIT_ACCESS_A, pccountofauditentries: Option<*mut u32>, pplistofauditentries: *mut *mut EXPLICIT_ACCESS_A, psd: super::PSECURITY_DESCRIPTOR) -> super::super::Foundation::WIN32_ERROR {
    windows_core::link!("advapi32.dll" "system" fn LookupSecurityDescriptorPartsA(ppowner : *mut *mut TRUSTEE_A, ppgroup : *mut *mut TRUSTEE_A, pccountofaccessentries : *mut u32, pplistofaccessentries : *mut *mut EXPLICIT_ACCESS_A, pccountofauditentries : *mut u32, pplistofauditentries : *mut *mut EXPLICIT_ACCESS_A, psd : super:: PSECURITY_DESCRIPTOR) -> super::super::Foundation:: WIN32_ERROR);
    unsafe { LookupSecurityDescriptorPartsA(ppowner.unwrap_or(core::mem::zeroed()) as _, ppgroup.unwrap_or(core::mem::zeroed()) as _, pccountofaccessentries.unwrap_or(core::mem::zeroed()) as _, pplistofaccessentries as _, pccountofauditentries.unwrap_or(core::mem::zeroed()) as _, pplistofauditentries as _, psd) }
}
#[inline]
pub unsafe fn LookupSecurityDescriptorPartsW(ppowner: Option<*mut *mut TRUSTEE_W>, ppgroup: Option<*mut *mut TRUSTEE_W>, pccountofaccessentries: Option<*mut u32>, pplistofaccessentries: *mut *mut EXPLICIT_ACCESS_W, pccountofauditentries: Option<*mut u32>, pplistofauditentries: *mut *mut EXPLICIT_ACCESS_W, psd: super::PSECURITY_DESCRIPTOR) -> super::super::Foundation::WIN32_ERROR {
    windows_core::link!("advapi32.dll" "system" fn LookupSecurityDescriptorPartsW(ppowner : *mut *mut TRUSTEE_W, ppgroup : *mut *mut TRUSTEE_W, pccountofaccessentries : *mut u32, pplistofaccessentries : *mut *mut EXPLICIT_ACCESS_W, pccountofauditentries : *mut u32, pplistofauditentries : *mut *mut EXPLICIT_ACCESS_W, psd : super:: PSECURITY_DESCRIPTOR) -> super::super::Foundation:: WIN32_ERROR);
    unsafe { LookupSecurityDescriptorPartsW(ppowner.unwrap_or(core::mem::zeroed()) as _, ppgroup.unwrap_or(core::mem::zeroed()) as _, pccountofaccessentries.unwrap_or(core::mem::zeroed()) as _, pplistofaccessentries as _, pccountofauditentries.unwrap_or(core::mem::zeroed()) as _, pplistofauditentries as _, psd) }
}
#[inline]
pub unsafe fn SetEntriesInAclA(plistofexplicitentries: Option<&[EXPLICIT_ACCESS_A]>, oldacl: Option<*const super::ACL>, newacl: *mut *mut super::ACL) -> super::super::Foundation::WIN32_ERROR {
    windows_core::link!("advapi32.dll" "system" fn SetEntriesInAclA(ccountofexplicitentries : u32, plistofexplicitentries : *const EXPLICIT_ACCESS_A, oldacl : *const super:: ACL, newacl : *mut *mut super:: ACL) -> super::super::Foundation:: WIN32_ERROR);
    unsafe { SetEntriesInAclA(plistofexplicitentries.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), core::mem::transmute(plistofexplicitentries.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), oldacl.unwrap_or(core::mem::zeroed()) as _, newacl as _) }
}
#[inline]
pub unsafe fn SetEntriesInAclW(plistofexplicitentries: Option<&[EXPLICIT_ACCESS_W]>, oldacl: Option<*const super::ACL>, newacl: *mut *mut super::ACL) -> super::super::Foundation::WIN32_ERROR {
    windows_core::link!("advapi32.dll" "system" fn SetEntriesInAclW(ccountofexplicitentries : u32, plistofexplicitentries : *const EXPLICIT_ACCESS_W, oldacl : *const super:: ACL, newacl : *mut *mut super:: ACL) -> super::super::Foundation:: WIN32_ERROR);
    unsafe { SetEntriesInAclW(plistofexplicitentries.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), core::mem::transmute(plistofexplicitentries.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), oldacl.unwrap_or(core::mem::zeroed()) as _, newacl as _) }
}
#[inline]
pub unsafe fn SetNamedSecurityInfoA<P0>(pobjectname: P0, objecttype: SE_OBJECT_TYPE, securityinfo: super::OBJECT_SECURITY_INFORMATION, psidowner: Option<super::PSID>, psidgroup: Option<super::PSID>, pdacl: Option<*const super::ACL>, psacl: Option<*const super::ACL>) -> super::super::Foundation::WIN32_ERROR
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("advapi32.dll" "system" fn SetNamedSecurityInfoA(pobjectname : windows_core::PCSTR, objecttype : SE_OBJECT_TYPE, securityinfo : super:: OBJECT_SECURITY_INFORMATION, psidowner : super:: PSID, psidgroup : super:: PSID, pdacl : *const super:: ACL, psacl : *const super:: ACL) -> super::super::Foundation:: WIN32_ERROR);
    unsafe { SetNamedSecurityInfoA(pobjectname.param().abi(), objecttype, securityinfo, psidowner.unwrap_or(core::mem::zeroed()) as _, psidgroup.unwrap_or(core::mem::zeroed()) as _, pdacl.unwrap_or(core::mem::zeroed()) as _, psacl.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn SetNamedSecurityInfoW<P0>(pobjectname: P0, objecttype: SE_OBJECT_TYPE, securityinfo: super::OBJECT_SECURITY_INFORMATION, psidowner: Option<super::PSID>, psidgroup: Option<super::PSID>, pdacl: Option<*const super::ACL>, psacl: Option<*const super::ACL>) -> super::super::Foundation::WIN32_ERROR
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("advapi32.dll" "system" fn SetNamedSecurityInfoW(pobjectname : windows_core::PCWSTR, objecttype : SE_OBJECT_TYPE, securityinfo : super:: OBJECT_SECURITY_INFORMATION, psidowner : super:: PSID, psidgroup : super:: PSID, pdacl : *const super:: ACL, psacl : *const super:: ACL) -> super::super::Foundation:: WIN32_ERROR);
    unsafe { SetNamedSecurityInfoW(pobjectname.param().abi(), objecttype, securityinfo, psidowner.unwrap_or(core::mem::zeroed()) as _, psidgroup.unwrap_or(core::mem::zeroed()) as _, pdacl.unwrap_or(core::mem::zeroed()) as _, psacl.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn SetSecurityInfo(handle: super::super::Foundation::HANDLE, objecttype: SE_OBJECT_TYPE, securityinfo: super::OBJECT_SECURITY_INFORMATION, psidowner: Option<super::PSID>, psidgroup: Option<super::PSID>, pdacl: Option<*const super::ACL>, psacl: Option<*const super::ACL>) -> super::super::Foundation::WIN32_ERROR {
    windows_core::link!("advapi32.dll" "system" fn SetSecurityInfo(handle : super::super::Foundation:: HANDLE, objecttype : SE_OBJECT_TYPE, securityinfo : super:: OBJECT_SECURITY_INFORMATION, psidowner : super:: PSID, psidgroup : super:: PSID, pdacl : *const super:: ACL, psacl : *const super:: ACL) -> super::super::Foundation:: WIN32_ERROR);
    unsafe { SetSecurityInfo(handle, objecttype, securityinfo, psidowner.unwrap_or(core::mem::zeroed()) as _, psidgroup.unwrap_or(core::mem::zeroed()) as _, pdacl.unwrap_or(core::mem::zeroed()) as _, psacl.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn TreeResetNamedSecurityInfoA<P0>(pobjectname: P0, objecttype: SE_OBJECT_TYPE, securityinfo: super::OBJECT_SECURITY_INFORMATION, powner: Option<super::PSID>, pgroup: Option<super::PSID>, pdacl: Option<*const super::ACL>, psacl: Option<*const super::ACL>, keepexplicit: bool, fnprogress: FN_PROGRESS, progressinvokesetting: PROG_INVOKE_SETTING, args: Option<*const core::ffi::c_void>) -> super::super::Foundation::WIN32_ERROR
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("advapi32.dll" "system" fn TreeResetNamedSecurityInfoA(pobjectname : windows_core::PCSTR, objecttype : SE_OBJECT_TYPE, securityinfo : super:: OBJECT_SECURITY_INFORMATION, powner : super:: PSID, pgroup : super:: PSID, pdacl : *const super:: ACL, psacl : *const super:: ACL, keepexplicit : windows_core::BOOL, fnprogress : FN_PROGRESS, progressinvokesetting : PROG_INVOKE_SETTING, args : *const core::ffi::c_void) -> super::super::Foundation:: WIN32_ERROR);
    unsafe { TreeResetNamedSecurityInfoA(pobjectname.param().abi(), objecttype, securityinfo, powner.unwrap_or(core::mem::zeroed()) as _, pgroup.unwrap_or(core::mem::zeroed()) as _, pdacl.unwrap_or(core::mem::zeroed()) as _, psacl.unwrap_or(core::mem::zeroed()) as _, keepexplicit.into(), fnprogress, progressinvokesetting, args.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn TreeResetNamedSecurityInfoW<P0>(pobjectname: P0, objecttype: SE_OBJECT_TYPE, securityinfo: super::OBJECT_SECURITY_INFORMATION, powner: Option<super::PSID>, pgroup: Option<super::PSID>, pdacl: Option<*const super::ACL>, psacl: Option<*const super::ACL>, keepexplicit: bool, fnprogress: FN_PROGRESS, progressinvokesetting: PROG_INVOKE_SETTING, args: Option<*const core::ffi::c_void>) -> super::super::Foundation::WIN32_ERROR
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("advapi32.dll" "system" fn TreeResetNamedSecurityInfoW(pobjectname : windows_core::PCWSTR, objecttype : SE_OBJECT_TYPE, securityinfo : super:: OBJECT_SECURITY_INFORMATION, powner : super:: PSID, pgroup : super:: PSID, pdacl : *const super:: ACL, psacl : *const super:: ACL, keepexplicit : windows_core::BOOL, fnprogress : FN_PROGRESS, progressinvokesetting : PROG_INVOKE_SETTING, args : *const core::ffi::c_void) -> super::super::Foundation:: WIN32_ERROR);
    unsafe { TreeResetNamedSecurityInfoW(pobjectname.param().abi(), objecttype, securityinfo, powner.unwrap_or(core::mem::zeroed()) as _, pgroup.unwrap_or(core::mem::zeroed()) as _, pdacl.unwrap_or(core::mem::zeroed()) as _, psacl.unwrap_or(core::mem::zeroed()) as _, keepexplicit.into(), fnprogress, progressinvokesetting, args.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn TreeSetNamedSecurityInfoA<P0>(pobjectname: P0, objecttype: SE_OBJECT_TYPE, securityinfo: super::OBJECT_SECURITY_INFORMATION, powner: Option<super::PSID>, pgroup: Option<super::PSID>, pdacl: Option<*const super::ACL>, psacl: Option<*const super::ACL>, dwaction: TREE_SEC_INFO, fnprogress: FN_PROGRESS, progressinvokesetting: PROG_INVOKE_SETTING, args: Option<*const core::ffi::c_void>) -> super::super::Foundation::WIN32_ERROR
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("advapi32.dll" "system" fn TreeSetNamedSecurityInfoA(pobjectname : windows_core::PCSTR, objecttype : SE_OBJECT_TYPE, securityinfo : super:: OBJECT_SECURITY_INFORMATION, powner : super:: PSID, pgroup : super:: PSID, pdacl : *const super:: ACL, psacl : *const super:: ACL, dwaction : TREE_SEC_INFO, fnprogress : FN_PROGRESS, progressinvokesetting : PROG_INVOKE_SETTING, args : *const core::ffi::c_void) -> super::super::Foundation:: WIN32_ERROR);
    unsafe { TreeSetNamedSecurityInfoA(pobjectname.param().abi(), objecttype, securityinfo, powner.unwrap_or(core::mem::zeroed()) as _, pgroup.unwrap_or(core::mem::zeroed()) as _, pdacl.unwrap_or(core::mem::zeroed()) as _, psacl.unwrap_or(core::mem::zeroed()) as _, dwaction, fnprogress, progressinvokesetting, args.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn TreeSetNamedSecurityInfoW<P0>(pobjectname: P0, objecttype: SE_OBJECT_TYPE, securityinfo: super::OBJECT_SECURITY_INFORMATION, powner: Option<super::PSID>, pgroup: Option<super::PSID>, pdacl: Option<*const super::ACL>, psacl: Option<*const super::ACL>, dwaction: TREE_SEC_INFO, fnprogress: FN_PROGRESS, progressinvokesetting: PROG_INVOKE_SETTING, args: Option<*const core::ffi::c_void>) -> super::super::Foundation::WIN32_ERROR
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("advapi32.dll" "system" fn TreeSetNamedSecurityInfoW(pobjectname : windows_core::PCWSTR, objecttype : SE_OBJECT_TYPE, securityinfo : super:: OBJECT_SECURITY_INFORMATION, powner : super:: PSID, pgroup : super:: PSID, pdacl : *const super:: ACL, psacl : *const super:: ACL, dwaction : TREE_SEC_INFO, fnprogress : FN_PROGRESS, progressinvokesetting : PROG_INVOKE_SETTING, args : *const core::ffi::c_void) -> super::super::Foundation:: WIN32_ERROR);
    unsafe { TreeSetNamedSecurityInfoW(pobjectname.param().abi(), objecttype, securityinfo, powner.unwrap_or(core::mem::zeroed()) as _, pgroup.unwrap_or(core::mem::zeroed()) as _, pdacl.unwrap_or(core::mem::zeroed()) as _, psacl.unwrap_or(core::mem::zeroed()) as _, dwaction, fnprogress, progressinvokesetting, args.unwrap_or(core::mem::zeroed()) as _) }
}
pub const ACCCTRL_DEFAULT_PROVIDER: windows_core::PCWSTR = windows_core::w!("Windows NT Access Provider");
pub const ACCCTRL_DEFAULT_PROVIDERA: windows_core::PCSTR = windows_core::s!("Windows NT Access Provider");
pub const ACCCTRL_DEFAULT_PROVIDERW: windows_core::PCWSTR = windows_core::w!("Windows NT Access Provider");
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct ACCESS_MODE(pub i32);
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ACTRL_ACCESSA {
    pub cEntries: u32,
    pub pPropertyAccessList: *mut ACTRL_PROPERTY_ENTRYA,
}
impl Default for ACTRL_ACCESSA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ACTRL_ACCESSW {
    pub cEntries: u32,
    pub pPropertyAccessList: *mut ACTRL_PROPERTY_ENTRYW,
}
impl Default for ACTRL_ACCESSW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const ACTRL_ACCESS_ALLOWED: ACTRL_ACCESS_ENTRY_ACCESS_FLAGS = ACTRL_ACCESS_ENTRY_ACCESS_FLAGS(1u32);
pub const ACTRL_ACCESS_DENIED: ACTRL_ACCESS_ENTRY_ACCESS_FLAGS = ACTRL_ACCESS_ENTRY_ACCESS_FLAGS(2u32);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct ACTRL_ACCESS_ENTRYA {
    pub Trustee: TRUSTEE_A,
    pub fAccessFlags: ACTRL_ACCESS_ENTRY_ACCESS_FLAGS,
    pub Access: u32,
    pub ProvSpecificAccess: u32,
    pub Inheritance: super::ACE_FLAGS,
    pub lpInheritProperty: windows_core::PSTR,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct ACTRL_ACCESS_ENTRYW {
    pub Trustee: TRUSTEE_W,
    pub fAccessFlags: ACTRL_ACCESS_ENTRY_ACCESS_FLAGS,
    pub Access: u32,
    pub ProvSpecificAccess: u32,
    pub Inheritance: super::ACE_FLAGS,
    pub lpInheritProperty: windows_core::PWSTR,
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct ACTRL_ACCESS_ENTRY_ACCESS_FLAGS(pub u32);
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ACTRL_ACCESS_ENTRY_LISTA {
    pub cEntries: u32,
    pub pAccessList: *mut ACTRL_ACCESS_ENTRYA,
}
impl Default for ACTRL_ACCESS_ENTRY_LISTA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ACTRL_ACCESS_ENTRY_LISTW {
    pub cEntries: u32,
    pub pAccessList: *mut ACTRL_ACCESS_ENTRYW,
}
impl Default for ACTRL_ACCESS_ENTRY_LISTW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct ACTRL_ACCESS_INFOA {
    pub fAccessPermission: u32,
    pub lpAccessPermissionName: windows_core::PSTR,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct ACTRL_ACCESS_INFOW {
    pub fAccessPermission: u32,
    pub lpAccessPermissionName: windows_core::PWSTR,
}
pub const ACTRL_ACCESS_NO_OPTIONS: u32 = 0u32;
pub const ACTRL_ACCESS_PROTECTED: u32 = 1u32;
pub const ACTRL_ACCESS_SUPPORTS_OBJECT_ENTRIES: u32 = 1u32;
pub const ACTRL_AUDIT_FAILURE: ACTRL_ACCESS_ENTRY_ACCESS_FLAGS = ACTRL_ACCESS_ENTRY_ACCESS_FLAGS(8u32);
pub const ACTRL_AUDIT_SUCCESS: ACTRL_ACCESS_ENTRY_ACCESS_FLAGS = ACTRL_ACCESS_ENTRY_ACCESS_FLAGS(4u32);
pub const ACTRL_CHANGE_ACCESS: u32 = 536870912u32;
pub const ACTRL_CHANGE_OWNER: u32 = 1073741824u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct ACTRL_CONTROL_INFOA {
    pub lpControlId: windows_core::PSTR,
    pub lpControlName: windows_core::PSTR,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct ACTRL_CONTROL_INFOW {
    pub lpControlId: windows_core::PWSTR,
    pub lpControlName: windows_core::PWSTR,
}
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
#[repr(C)]
#[derive(Clone, Copy)]
pub struct ACTRL_OVERLAPPED {
    pub Anonymous: ACTRL_OVERLAPPED_0,
    pub Reserved2: u32,
    pub hEvent: super::super::Foundation::HANDLE,
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
impl Default for ACTRL_OVERLAPPED_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
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
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ACTRL_PROPERTY_ENTRYA {
    pub lpProperty: windows_core::PSTR,
    pub pAccessEntryList: *mut ACTRL_ACCESS_ENTRY_LISTA,
    pub fListFlags: u32,
}
impl Default for ACTRL_PROPERTY_ENTRYA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ACTRL_PROPERTY_ENTRYW {
    pub lpProperty: windows_core::PWSTR,
    pub pAccessEntryList: *mut ACTRL_ACCESS_ENTRY_LISTW,
    pub fListFlags: u32,
}
impl Default for ACTRL_PROPERTY_ENTRYW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
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
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct AUDIT_IP_ADDRESS {
    pub pIpAddress: [u8; 128],
}
impl Default for AUDIT_IP_ADDRESS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct AUDIT_OBJECT_TYPE {
    pub ObjectType: windows_core::GUID,
    pub Flags: u16,
    pub Level: u16,
    pub AccessMask: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct AUDIT_OBJECT_TYPES {
    pub Count: u16,
    pub Flags: u16,
    pub pObjectTypes: *mut AUDIT_OBJECT_TYPE,
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
impl Default for AUDIT_PARAM_1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct AUDIT_PARAMS {
    pub Length: u32,
    pub Flags: u32,
    pub Count: u16,
    pub Parameters: *mut AUDIT_PARAM,
}
impl Default for AUDIT_PARAMS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct AUDIT_PARAM_TYPE(pub i32);
pub const AUDIT_TYPE_LEGACY: u32 = 1u32;
pub const AUDIT_TYPE_WMI: u32 = 2u32;
pub const AUTHZP_WPD_EVENT: u32 = 16u32;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct AUTHZ_ACCESS_CHECK_FLAGS(pub u32);
pub const AUTHZ_ACCESS_CHECK_NO_DEEP_COPY_SD: AUTHZ_ACCESS_CHECK_FLAGS = AUTHZ_ACCESS_CHECK_FLAGS(1u32);
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
            windows_core::link!("authz.dll" "system" fn AuthzFreeHandle(haccesscheckresults : *mut core::ffi::c_void) -> i32);
            unsafe {
                AuthzFreeHandle(self.0);
            }
        }
    }
}
impl Default for AUTHZ_ACCESS_CHECK_RESULTS_HANDLE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct AUTHZ_ACCESS_REPLY {
    pub ResultListLength: u32,
    pub GrantedAccessMask: *mut u32,
    pub SaclEvaluationResults: *mut AUTHZ_GENERATE_RESULTS,
    pub Error: *mut u32,
}
impl Default for AUTHZ_ACCESS_REPLY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct AUTHZ_ACCESS_REQUEST {
    pub DesiredAccess: u32,
    pub PrincipalSelfSid: super::PSID,
    pub ObjectTypeList: *mut super::OBJECT_TYPE_LIST,
    pub ObjectTypeListLength: u32,
    pub OptionalArguments: *mut core::ffi::c_void,
}
impl Default for AUTHZ_ACCESS_REQUEST {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const AUTHZ_ALLOW_MULTIPLE_SOURCE_INSTANCES: u32 = 1u32;
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
            windows_core::link!("authz.dll" "system" fn AuthzFreeAuditEvent(hauditevent : *mut core::ffi::c_void) -> i32);
            unsafe {
                AuthzFreeAuditEvent(self.0);
            }
        }
    }
}
impl Default for AUTHZ_AUDIT_EVENT_HANDLE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct AUTHZ_AUDIT_EVENT_INFORMATION_CLASS(pub i32);
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
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct AUTHZ_AUDIT_EVENT_TYPE_LEGACY {
    pub CategoryId: u16,
    pub AuditId: u16,
    pub ParameterCount: u16,
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
impl Default for AUTHZ_AUDIT_EVENT_TYPE_UNION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const AUTHZ_AUDIT_INSTANCE_INFORMATION: u32 = 2u32;
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
            windows_core::link!("authz.dll" "system" fn AuthzUnregisterCapChangeNotification(hcapchangesubscription : *mut core::ffi::c_void) -> i32);
            unsafe {
                AuthzUnregisterCapChangeNotification(self.0);
            }
        }
    }
}
impl Default for AUTHZ_CAP_CHANGE_SUBSCRIPTION_HANDLE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
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
            windows_core::link!("authz.dll" "system" fn AuthzFreeContext(hauthzclientcontext : *mut core::ffi::c_void) -> i32);
            unsafe {
                AuthzFreeContext(self.0);
            }
        }
    }
}
impl Default for AUTHZ_CLIENT_CONTEXT_HANDLE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const AUTHZ_COMPUTE_PRIVILEGES: u32 = 8u32;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct AUTHZ_CONTEXT_INFORMATION_CLASS(pub i32);
pub const AUTHZ_FLAG_ALLOW_MULTIPLE_SOURCE_INSTANCES: u32 = 1u32;
pub const AUTHZ_GENERATE_FAILURE_AUDIT: AUTHZ_GENERATE_RESULTS = AUTHZ_GENERATE_RESULTS(2u32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct AUTHZ_GENERATE_RESULTS(pub u32);
pub const AUTHZ_GENERATE_SUCCESS_AUDIT: AUTHZ_GENERATE_RESULTS = AUTHZ_GENERATE_RESULTS(1u32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct AUTHZ_INITIALIZE_OBJECT_ACCESS_AUDIT_EVENT_FLAGS(pub u32);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default)]
pub struct AUTHZ_INIT_INFO {
    pub version: u16,
    pub szResourceManagerName: windows_core::PCWSTR,
    pub pfnDynamicAccessCheck: PFN_AUTHZ_DYNAMIC_ACCESS_CHECK,
    pub pfnComputeDynamicGroups: PFN_AUTHZ_COMPUTE_DYNAMIC_GROUPS,
    pub pfnFreeDynamicGroups: PFN_AUTHZ_FREE_DYNAMIC_GROUPS,
    pub pfnGetCentralAccessPolicy: PFN_AUTHZ_GET_CENTRAL_ACCESS_POLICY,
    pub pfnFreeCentralAccessPolicy: PFN_AUTHZ_FREE_CENTRAL_ACCESS_POLICY,
}
pub const AUTHZ_INIT_INFO_VERSION_V1: u32 = 1u32;
pub const AUTHZ_MIGRATED_LEGACY_PUBLISHER: u32 = 2u32;
pub const AUTHZ_NO_ALLOC_STRINGS: AUTHZ_INITIALIZE_OBJECT_ACCESS_AUDIT_EVENT_FLAGS = AUTHZ_INITIALIZE_OBJECT_ACCESS_AUDIT_EVENT_FLAGS(4u32);
pub const AUTHZ_NO_FAILURE_AUDIT: AUTHZ_INITIALIZE_OBJECT_ACCESS_AUDIT_EVENT_FLAGS = AUTHZ_INITIALIZE_OBJECT_ACCESS_AUDIT_EVENT_FLAGS(2u32);
pub const AUTHZ_NO_SUCCESS_AUDIT: AUTHZ_INITIALIZE_OBJECT_ACCESS_AUDIT_EVENT_FLAGS = AUTHZ_INITIALIZE_OBJECT_ACCESS_AUDIT_EVENT_FLAGS(1u32);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct AUTHZ_REGISTRATION_OBJECT_TYPE_NAME_OFFSET {
    pub szObjectTypeName: windows_core::PWSTR,
    pub dwOffset: u32,
}
pub const AUTHZ_REQUIRE_S4U_LOGON: u32 = 4u32;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct AUTHZ_RESOURCE_MANAGER_FLAGS(pub u32);
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
            windows_core::link!("authz.dll" "system" fn AuthzFreeResourceManager(hauthzresourcemanager : *mut core::ffi::c_void) -> i32);
            unsafe {
                AuthzFreeResourceManager(self.0);
            }
        }
    }
}
impl Default for AUTHZ_RESOURCE_MANAGER_HANDLE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const AUTHZ_RM_FLAG_INITIALIZE_UNDER_IMPERSONATION: AUTHZ_RESOURCE_MANAGER_FLAGS = AUTHZ_RESOURCE_MANAGER_FLAGS(2u32);
pub const AUTHZ_RM_FLAG_NO_AUDIT: AUTHZ_RESOURCE_MANAGER_FLAGS = AUTHZ_RESOURCE_MANAGER_FLAGS(1u32);
pub const AUTHZ_RM_FLAG_NO_CENTRAL_ACCESS_POLICIES: AUTHZ_RESOURCE_MANAGER_FLAGS = AUTHZ_RESOURCE_MANAGER_FLAGS(4u32);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct AUTHZ_RPC_INIT_INFO_CLIENT {
    pub version: u16,
    pub ObjectUuid: windows_core::PWSTR,
    pub ProtSeq: windows_core::PWSTR,
    pub NetworkAddr: windows_core::PWSTR,
    pub Endpoint: windows_core::PWSTR,
    pub Options: windows_core::PWSTR,
    pub ServerSpn: windows_core::PWSTR,
}
pub const AUTHZ_RPC_INIT_INFO_CLIENT_VERSION_V1: u32 = 1u32;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct AUTHZ_SECURITY_ATTRIBUTES_INFORMATION {
    pub Version: u16,
    pub Reserved: u16,
    pub AttributeCount: u32,
    pub Attribute: AUTHZ_SECURITY_ATTRIBUTES_INFORMATION_0,
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
impl Default for AUTHZ_SECURITY_ATTRIBUTES_INFORMATION_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const AUTHZ_SECURITY_ATTRIBUTES_INFORMATION_VERSION: u32 = 1u32;
pub const AUTHZ_SECURITY_ATTRIBUTES_INFORMATION_VERSION_V1: u32 = 1u32;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct AUTHZ_SECURITY_ATTRIBUTE_FLAGS(pub u32);
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
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct AUTHZ_SECURITY_ATTRIBUTE_FQBN_VALUE {
    pub Version: u64,
    pub pName: windows_core::PWSTR,
}
pub const AUTHZ_SECURITY_ATTRIBUTE_NON_INHERITABLE: AUTHZ_SECURITY_ATTRIBUTE_FLAGS = AUTHZ_SECURITY_ATTRIBUTE_FLAGS(1u32);
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct AUTHZ_SECURITY_ATTRIBUTE_OCTET_STRING_VALUE {
    pub pValue: *mut core::ffi::c_void,
    pub ValueLength: u32,
}
impl Default for AUTHZ_SECURITY_ATTRIBUTE_OCTET_STRING_VALUE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct AUTHZ_SECURITY_ATTRIBUTE_OPERATION(pub i32);
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
impl Default for AUTHZ_SECURITY_ATTRIBUTE_V1_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const AUTHZ_SECURITY_ATTRIBUTE_VALUE_CASE_SENSITIVE: AUTHZ_SECURITY_ATTRIBUTE_FLAGS = AUTHZ_SECURITY_ATTRIBUTE_FLAGS(2u32);
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
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct AUTHZ_SID_OPERATION(pub i32);
pub const AUTHZ_SID_OPERATION_ADD: AUTHZ_SID_OPERATION = AUTHZ_SID_OPERATION(2i32);
pub const AUTHZ_SID_OPERATION_DELETE: AUTHZ_SID_OPERATION = AUTHZ_SID_OPERATION(3i32);
pub const AUTHZ_SID_OPERATION_NONE: AUTHZ_SID_OPERATION = AUTHZ_SID_OPERATION(0i32);
pub const AUTHZ_SID_OPERATION_REPLACE: AUTHZ_SID_OPERATION = AUTHZ_SID_OPERATION(4i32);
pub const AUTHZ_SID_OPERATION_REPLACE_ALL: AUTHZ_SID_OPERATION = AUTHZ_SID_OPERATION(1i32);
pub const AUTHZ_SKIP_TOKEN_GROUPS: u32 = 2u32;
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
impl Default for AUTHZ_SOURCE_SCHEMA_REGISTRATION_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
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
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct AZ_PROP_CONSTANTS(pub i32);
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
pub const AzAuthorizationStore: windows_core::GUID = windows_core::GUID::from_u128(0xb2bcff59_a757_4b0b_a1bc_ea69981da69e);
pub const AzBizRuleContext: windows_core::GUID = windows_core::GUID::from_u128(0x5c2dc96f_8d51_434b_b33c_379bccae77c3);
pub const AzPrincipalLocator: windows_core::GUID = windows_core::GUID::from_u128(0x483afb5d_70df_4e16_abdc_a1de4d015a3e);
pub const DENY_ACCESS: ACCESS_MODE = ACCESS_MODE(3i32);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct EXPLICIT_ACCESS_A {
    pub grfAccessPermissions: u32,
    pub grfAccessMode: ACCESS_MODE,
    pub grfInheritance: super::ACE_FLAGS,
    pub Trustee: TRUSTEE_A,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct EXPLICIT_ACCESS_W {
    pub grfAccessPermissions: u32,
    pub grfAccessMode: ACCESS_MODE,
    pub grfInheritance: super::ACE_FLAGS,
    pub Trustee: TRUSTEE_W,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct FN_OBJECT_MGR_FUNCTS {
    pub Placeholder: u32,
}
pub type FN_PROGRESS = Option<unsafe extern "system" fn(pobjectname: windows_core::PCWSTR, status: u32, pinvokesetting: *mut PROG_INVOKE_SETTING, args: *const core::ffi::c_void, securityset: windows_core::BOOL)>;
pub const GRANT_ACCESS: ACCESS_MODE = ACCESS_MODE(1i32);
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
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Name)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn SetName(&self, bstrname: &windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetName)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrname)).ok() }
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
    pub unsafe fn ApplicationData(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ApplicationData)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn SetApplicationData(&self, bstrapplicationdata: &windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetApplicationData)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrapplicationdata)).ok() }
    }
    pub unsafe fn AuthzInterfaceClsid(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).AuthzInterfaceClsid)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn SetAuthzInterfaceClsid(&self, bstrprop: &windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetAuthzInterfaceClsid)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrprop)).ok() }
    }
    pub unsafe fn Version(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Version)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn SetVersion(&self, bstrprop: &windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetVersion)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrprop)).ok() }
    }
    pub unsafe fn GenerateAudits(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GenerateAudits)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetGenerateAudits(&self, bprop: bool) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetGenerateAudits)(windows_core::Interface::as_raw(self), bprop.into()).ok() }
    }
    pub unsafe fn ApplyStoreSacl(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ApplyStoreSacl)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetApplyStoreSacl(&self, bprop: bool) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetApplyStoreSacl)(windows_core::Interface::as_raw(self), bprop.into()).ok() }
    }
    pub unsafe fn Writable(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Writable)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn GetProperty(&self, lpropid: i32, varreserved: &super::super::System::Variant::VARIANT) -> windows_core::Result<super::super::System::Variant::VARIANT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetProperty)(windows_core::Interface::as_raw(self), lpropid, core::mem::transmute_copy(varreserved), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn SetProperty(&self, lpropid: i32, varprop: &super::super::System::Variant::VARIANT, varreserved: &super::super::System::Variant::VARIANT) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetProperty)(windows_core::Interface::as_raw(self), lpropid, core::mem::transmute_copy(varprop), core::mem::transmute_copy(varreserved)).ok() }
    }
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn PolicyAdministrators(&self) -> windows_core::Result<super::super::System::Variant::VARIANT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).PolicyAdministrators)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn PolicyReaders(&self) -> windows_core::Result<super::super::System::Variant::VARIANT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).PolicyReaders)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn AddPolicyAdministrator(&self, bstradmin: &windows_core::BSTR, varreserved: &super::super::System::Variant::VARIANT) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).AddPolicyAdministrator)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstradmin), core::mem::transmute_copy(varreserved)).ok() }
    }
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn DeletePolicyAdministrator(&self, bstradmin: &windows_core::BSTR, varreserved: &super::super::System::Variant::VARIANT) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).DeletePolicyAdministrator)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstradmin), core::mem::transmute_copy(varreserved)).ok() }
    }
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn AddPolicyReader(&self, bstrreader: &windows_core::BSTR, varreserved: &super::super::System::Variant::VARIANT) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).AddPolicyReader)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrreader), core::mem::transmute_copy(varreserved)).ok() }
    }
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn DeletePolicyReader(&self, bstrreader: &windows_core::BSTR, varreserved: &super::super::System::Variant::VARIANT) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).DeletePolicyReader)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrreader), core::mem::transmute_copy(varreserved)).ok() }
    }
    pub unsafe fn Scopes(&self) -> windows_core::Result<IAzScopes> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Scopes)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn OpenScope(&self, bstrscopename: &windows_core::BSTR, varreserved: &super::super::System::Variant::VARIANT) -> windows_core::Result<IAzScope> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).OpenScope)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrscopename), core::mem::transmute_copy(varreserved), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn CreateScope(&self, bstrscopename: &windows_core::BSTR, varreserved: &super::super::System::Variant::VARIANT) -> windows_core::Result<IAzScope> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateScope)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrscopename), core::mem::transmute_copy(varreserved), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn DeleteScope(&self, bstrscopename: &windows_core::BSTR, varreserved: &super::super::System::Variant::VARIANT) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).DeleteScope)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrscopename), core::mem::transmute_copy(varreserved)).ok() }
    }
    pub unsafe fn Operations(&self) -> windows_core::Result<IAzOperations> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Operations)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn OpenOperation(&self, bstroperationname: &windows_core::BSTR, varreserved: &super::super::System::Variant::VARIANT) -> windows_core::Result<IAzOperation> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).OpenOperation)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstroperationname), core::mem::transmute_copy(varreserved), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn CreateOperation(&self, bstroperationname: &windows_core::BSTR, varreserved: &super::super::System::Variant::VARIANT) -> windows_core::Result<IAzOperation> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateOperation)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstroperationname), core::mem::transmute_copy(varreserved), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn DeleteOperation(&self, bstroperationname: &windows_core::BSTR, varreserved: &super::super::System::Variant::VARIANT) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).DeleteOperation)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstroperationname), core::mem::transmute_copy(varreserved)).ok() }
    }
    pub unsafe fn Tasks(&self) -> windows_core::Result<IAzTasks> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Tasks)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn OpenTask(&self, bstrtaskname: &windows_core::BSTR, varreserved: &super::super::System::Variant::VARIANT) -> windows_core::Result<IAzTask> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).OpenTask)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrtaskname), core::mem::transmute_copy(varreserved), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn CreateTask(&self, bstrtaskname: &windows_core::BSTR, varreserved: &super::super::System::Variant::VARIANT) -> windows_core::Result<IAzTask> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateTask)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrtaskname), core::mem::transmute_copy(varreserved), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn DeleteTask(&self, bstrtaskname: &windows_core::BSTR, varreserved: &super::super::System::Variant::VARIANT) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).DeleteTask)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrtaskname), core::mem::transmute_copy(varreserved)).ok() }
    }
    pub unsafe fn ApplicationGroups(&self) -> windows_core::Result<IAzApplicationGroups> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ApplicationGroups)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn OpenApplicationGroup(&self, bstrgroupname: &windows_core::BSTR, varreserved: &super::super::System::Variant::VARIANT) -> windows_core::Result<IAzApplicationGroup> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).OpenApplicationGroup)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrgroupname), core::mem::transmute_copy(varreserved), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn CreateApplicationGroup(&self, bstrgroupname: &windows_core::BSTR, varreserved: &super::super::System::Variant::VARIANT) -> windows_core::Result<IAzApplicationGroup> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateApplicationGroup)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrgroupname), core::mem::transmute_copy(varreserved), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn DeleteApplicationGroup(&self, bstrgroupname: &windows_core::BSTR, varreserved: &super::super::System::Variant::VARIANT) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).DeleteApplicationGroup)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrgroupname), core::mem::transmute_copy(varreserved)).ok() }
    }
    pub unsafe fn Roles(&self) -> windows_core::Result<IAzRoles> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Roles)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn OpenRole(&self, bstrrolename: &windows_core::BSTR, varreserved: &super::super::System::Variant::VARIANT) -> windows_core::Result<IAzRole> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).OpenRole)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrrolename), core::mem::transmute_copy(varreserved), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn CreateRole(&self, bstrrolename: &windows_core::BSTR, varreserved: &super::super::System::Variant::VARIANT) -> windows_core::Result<IAzRole> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateRole)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrrolename), core::mem::transmute_copy(varreserved), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn DeleteRole(&self, bstrrolename: &windows_core::BSTR, varreserved: &super::super::System::Variant::VARIANT) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).DeleteRole)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrrolename), core::mem::transmute_copy(varreserved)).ok() }
    }
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn InitializeClientContextFromToken(&self, ulltokenhandle: u64, varreserved: &super::super::System::Variant::VARIANT) -> windows_core::Result<IAzClientContext> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).InitializeClientContextFromToken)(windows_core::Interface::as_raw(self), ulltokenhandle, core::mem::transmute_copy(varreserved), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn AddPropertyItem(&self, lpropid: i32, varprop: &super::super::System::Variant::VARIANT, varreserved: &super::super::System::Variant::VARIANT) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).AddPropertyItem)(windows_core::Interface::as_raw(self), lpropid, core::mem::transmute_copy(varprop), core::mem::transmute_copy(varreserved)).ok() }
    }
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn DeletePropertyItem(&self, lpropid: i32, varprop: &super::super::System::Variant::VARIANT, varreserved: &super::super::System::Variant::VARIANT) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).DeletePropertyItem)(windows_core::Interface::as_raw(self), lpropid, core::mem::transmute_copy(varprop), core::mem::transmute_copy(varreserved)).ok() }
    }
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn Submit(&self, lflags: i32, varreserved: &super::super::System::Variant::VARIANT) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Submit)(windows_core::Interface::as_raw(self), lflags, core::mem::transmute_copy(varreserved)).ok() }
    }
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn InitializeClientContextFromName(&self, clientname: &windows_core::BSTR, domainname: &windows_core::BSTR, varreserved: &super::super::System::Variant::VARIANT) -> windows_core::Result<IAzClientContext> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).InitializeClientContextFromName)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(clientname), core::mem::transmute_copy(domainname), core::mem::transmute_copy(varreserved), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn DelegatedPolicyUsers(&self) -> windows_core::Result<super::super::System::Variant::VARIANT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).DelegatedPolicyUsers)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn AddDelegatedPolicyUser(&self, bstrdelegatedpolicyuser: &windows_core::BSTR, varreserved: &super::super::System::Variant::VARIANT) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).AddDelegatedPolicyUser)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrdelegatedpolicyuser), core::mem::transmute_copy(varreserved)).ok() }
    }
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn DeleteDelegatedPolicyUser(&self, bstrdelegatedpolicyuser: &windows_core::BSTR, varreserved: &super::super::System::Variant::VARIANT) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).DeleteDelegatedPolicyUser)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrdelegatedpolicyuser), core::mem::transmute_copy(varreserved)).ok() }
    }
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn InitializeClientContextFromStringSid(&self, sidstring: &windows_core::BSTR, loptions: i32, varreserved: &super::super::System::Variant::VARIANT) -> windows_core::Result<IAzClientContext> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).InitializeClientContextFromStringSid)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(sidstring), loptions, core::mem::transmute_copy(varreserved), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn PolicyAdministratorsName(&self) -> windows_core::Result<super::super::System::Variant::VARIANT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).PolicyAdministratorsName)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn PolicyReadersName(&self) -> windows_core::Result<super::super::System::Variant::VARIANT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).PolicyReadersName)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn AddPolicyAdministratorName(&self, bstradmin: &windows_core::BSTR, varreserved: &super::super::System::Variant::VARIANT) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).AddPolicyAdministratorName)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstradmin), core::mem::transmute_copy(varreserved)).ok() }
    }
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn DeletePolicyAdministratorName(&self, bstradmin: &windows_core::BSTR, varreserved: &super::super::System::Variant::VARIANT) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).DeletePolicyAdministratorName)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstradmin), core::mem::transmute_copy(varreserved)).ok() }
    }
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn AddPolicyReaderName(&self, bstrreader: &windows_core::BSTR, varreserved: &super::super::System::Variant::VARIANT) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).AddPolicyReaderName)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrreader), core::mem::transmute_copy(varreserved)).ok() }
    }
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn DeletePolicyReaderName(&self, bstrreader: &windows_core::BSTR, varreserved: &super::super::System::Variant::VARIANT) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).DeletePolicyReaderName)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrreader), core::mem::transmute_copy(varreserved)).ok() }
    }
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn DelegatedPolicyUsersName(&self) -> windows_core::Result<super::super::System::Variant::VARIANT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).DelegatedPolicyUsersName)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn AddDelegatedPolicyUserName(&self, bstrdelegatedpolicyuser: &windows_core::BSTR, varreserved: &super::super::System::Variant::VARIANT) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).AddDelegatedPolicyUserName)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrdelegatedpolicyuser), core::mem::transmute_copy(varreserved)).ok() }
    }
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn DeleteDelegatedPolicyUserName(&self, bstrdelegatedpolicyuser: &windows_core::BSTR, varreserved: &super::super::System::Variant::VARIANT) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).DeleteDelegatedPolicyUserName)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrdelegatedpolicyuser), core::mem::transmute_copy(varreserved)).ok() }
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IAzApplication_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub Name: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Description: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetDescription: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ApplicationData: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetApplicationData: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub AuthzInterfaceClsid: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetAuthzInterfaceClsid: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Version: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetVersion: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GenerateAudits: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::BOOL) -> windows_core::HRESULT,
    pub SetGenerateAudits: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::BOOL) -> windows_core::HRESULT,
    pub ApplyStoreSacl: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::BOOL) -> windows_core::HRESULT,
    pub SetApplyStoreSacl: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::BOOL) -> windows_core::HRESULT,
    pub Writable: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::BOOL) -> windows_core::HRESULT,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub GetProperty: unsafe extern "system" fn(*mut core::ffi::c_void, i32, super::super::System::Variant::VARIANT, *mut super::super::System::Variant::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    GetProperty: usize,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub SetProperty: unsafe extern "system" fn(*mut core::ffi::c_void, i32, super::super::System::Variant::VARIANT, super::super::System::Variant::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    SetProperty: usize,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub PolicyAdministrators: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::System::Variant::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    PolicyAdministrators: usize,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub PolicyReaders: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::System::Variant::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    PolicyReaders: usize,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub AddPolicyAdministrator: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, super::super::System::Variant::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    AddPolicyAdministrator: usize,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub DeletePolicyAdministrator: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, super::super::System::Variant::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    DeletePolicyAdministrator: usize,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub AddPolicyReader: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, super::super::System::Variant::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    AddPolicyReader: usize,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub DeletePolicyReader: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, super::super::System::Variant::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    DeletePolicyReader: usize,
    pub Scopes: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub OpenScope: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, super::super::System::Variant::VARIANT, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    OpenScope: usize,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub CreateScope: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, super::super::System::Variant::VARIANT, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    CreateScope: usize,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub DeleteScope: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, super::super::System::Variant::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    DeleteScope: usize,
    pub Operations: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub OpenOperation: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, super::super::System::Variant::VARIANT, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    OpenOperation: usize,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub CreateOperation: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, super::super::System::Variant::VARIANT, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    CreateOperation: usize,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub DeleteOperation: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, super::super::System::Variant::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    DeleteOperation: usize,
    pub Tasks: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub OpenTask: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, super::super::System::Variant::VARIANT, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    OpenTask: usize,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub CreateTask: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, super::super::System::Variant::VARIANT, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    CreateTask: usize,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub DeleteTask: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, super::super::System::Variant::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    DeleteTask: usize,
    pub ApplicationGroups: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub OpenApplicationGroup: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, super::super::System::Variant::VARIANT, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    OpenApplicationGroup: usize,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub CreateApplicationGroup: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, super::super::System::Variant::VARIANT, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    CreateApplicationGroup: usize,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub DeleteApplicationGroup: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, super::super::System::Variant::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    DeleteApplicationGroup: usize,
    pub Roles: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub OpenRole: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, super::super::System::Variant::VARIANT, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    OpenRole: usize,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub CreateRole: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, super::super::System::Variant::VARIANT, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    CreateRole: usize,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub DeleteRole: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, super::super::System::Variant::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    DeleteRole: usize,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub InitializeClientContextFromToken: unsafe extern "system" fn(*mut core::ffi::c_void, u64, super::super::System::Variant::VARIANT, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    InitializeClientContextFromToken: usize,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub AddPropertyItem: unsafe extern "system" fn(*mut core::ffi::c_void, i32, super::super::System::Variant::VARIANT, super::super::System::Variant::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    AddPropertyItem: usize,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub DeletePropertyItem: unsafe extern "system" fn(*mut core::ffi::c_void, i32, super::super::System::Variant::VARIANT, super::super::System::Variant::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    DeletePropertyItem: usize,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub Submit: unsafe extern "system" fn(*mut core::ffi::c_void, i32, super::super::System::Variant::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    Submit: usize,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub InitializeClientContextFromName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, super::super::System::Variant::VARIANT, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    InitializeClientContextFromName: usize,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub DelegatedPolicyUsers: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::System::Variant::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    DelegatedPolicyUsers: usize,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub AddDelegatedPolicyUser: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, super::super::System::Variant::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    AddDelegatedPolicyUser: usize,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub DeleteDelegatedPolicyUser: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, super::super::System::Variant::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    DeleteDelegatedPolicyUser: usize,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub InitializeClientContextFromStringSid: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, i32, super::super::System::Variant::VARIANT, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    InitializeClientContextFromStringSid: usize,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub PolicyAdministratorsName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::System::Variant::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    PolicyAdministratorsName: usize,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub PolicyReadersName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::System::Variant::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    PolicyReadersName: usize,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub AddPolicyAdministratorName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, super::super::System::Variant::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    AddPolicyAdministratorName: usize,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub DeletePolicyAdministratorName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, super::super::System::Variant::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    DeletePolicyAdministratorName: usize,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub AddPolicyReaderName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, super::super::System::Variant::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    AddPolicyReaderName: usize,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub DeletePolicyReaderName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, super::super::System::Variant::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    DeletePolicyReaderName: usize,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub DelegatedPolicyUsersName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::System::Variant::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    DelegatedPolicyUsersName: usize,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub AddDelegatedPolicyUserName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, super::super::System::Variant::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    AddDelegatedPolicyUserName: usize,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub DeleteDelegatedPolicyUserName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, super::super::System::Variant::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    DeleteDelegatedPolicyUserName: usize,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IAzApplication_Impl: super::super::System::Com::IDispatch_Impl {
    fn Name(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetName(&self, bstrname: &windows_core::BSTR) -> windows_core::Result<()>;
    fn Description(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetDescription(&self, bstrdescription: &windows_core::BSTR) -> windows_core::Result<()>;
    fn ApplicationData(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetApplicationData(&self, bstrapplicationdata: &windows_core::BSTR) -> windows_core::Result<()>;
    fn AuthzInterfaceClsid(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetAuthzInterfaceClsid(&self, bstrprop: &windows_core::BSTR) -> windows_core::Result<()>;
    fn Version(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetVersion(&self, bstrprop: &windows_core::BSTR) -> windows_core::Result<()>;
    fn GenerateAudits(&self) -> windows_core::Result<windows_core::BOOL>;
    fn SetGenerateAudits(&self, bprop: windows_core::BOOL) -> windows_core::Result<()>;
    fn ApplyStoreSacl(&self) -> windows_core::Result<windows_core::BOOL>;
    fn SetApplyStoreSacl(&self, bprop: windows_core::BOOL) -> windows_core::Result<()>;
    fn Writable(&self) -> windows_core::Result<windows_core::BOOL>;
    fn GetProperty(&self, lpropid: i32, varreserved: &super::super::System::Variant::VARIANT) -> windows_core::Result<super::super::System::Variant::VARIANT>;
    fn SetProperty(&self, lpropid: i32, varprop: &super::super::System::Variant::VARIANT, varreserved: &super::super::System::Variant::VARIANT) -> windows_core::Result<()>;
    fn PolicyAdministrators(&self) -> windows_core::Result<super::super::System::Variant::VARIANT>;
    fn PolicyReaders(&self) -> windows_core::Result<super::super::System::Variant::VARIANT>;
    fn AddPolicyAdministrator(&self, bstradmin: &windows_core::BSTR, varreserved: &super::super::System::Variant::VARIANT) -> windows_core::Result<()>;
    fn DeletePolicyAdministrator(&self, bstradmin: &windows_core::BSTR, varreserved: &super::super::System::Variant::VARIANT) -> windows_core::Result<()>;
    fn AddPolicyReader(&self, bstrreader: &windows_core::BSTR, varreserved: &super::super::System::Variant::VARIANT) -> windows_core::Result<()>;
    fn DeletePolicyReader(&self, bstrreader: &windows_core::BSTR, varreserved: &super::super::System::Variant::VARIANT) -> windows_core::Result<()>;
    fn Scopes(&self) -> windows_core::Result<IAzScopes>;
    fn OpenScope(&self, bstrscopename: &windows_core::BSTR, varreserved: &super::super::System::Variant::VARIANT) -> windows_core::Result<IAzScope>;
    fn CreateScope(&self, bstrscopename: &windows_core::BSTR, varreserved: &super::super::System::Variant::VARIANT) -> windows_core::Result<IAzScope>;
    fn DeleteScope(&self, bstrscopename: &windows_core::BSTR, varreserved: &super::super::System::Variant::VARIANT) -> windows_core::Result<()>;
    fn Operations(&self) -> windows_core::Result<IAzOperations>;
    fn OpenOperation(&self, bstroperationname: &windows_core::BSTR, varreserved: &super::super::System::Variant::VARIANT) -> windows_core::Result<IAzOperation>;
    fn CreateOperation(&self, bstroperationname: &windows_core::BSTR, varreserved: &super::super::System::Variant::VARIANT) -> windows_core::Result<IAzOperation>;
    fn DeleteOperation(&self, bstroperationname: &windows_core::BSTR, varreserved: &super::super::System::Variant::VARIANT) -> windows_core::Result<()>;
    fn Tasks(&self) -> windows_core::Result<IAzTasks>;
    fn OpenTask(&self, bstrtaskname: &windows_core::BSTR, varreserved: &super::super::System::Variant::VARIANT) -> windows_core::Result<IAzTask>;
    fn CreateTask(&self, bstrtaskname: &windows_core::BSTR, varreserved: &super::super::System::Variant::VARIANT) -> windows_core::Result<IAzTask>;
    fn DeleteTask(&self, bstrtaskname: &windows_core::BSTR, varreserved: &super::super::System::Variant::VARIANT) -> windows_core::Result<()>;
    fn ApplicationGroups(&self) -> windows_core::Result<IAzApplicationGroups>;
    fn OpenApplicationGroup(&self, bstrgroupname: &windows_core::BSTR, varreserved: &super::super::System::Variant::VARIANT) -> windows_core::Result<IAzApplicationGroup>;
    fn CreateApplicationGroup(&self, bstrgroupname: &windows_core::BSTR, varreserved: &super::super::System::Variant::VARIANT) -> windows_core::Result<IAzApplicationGroup>;
    fn DeleteApplicationGroup(&self, bstrgroupname: &windows_core::BSTR, varreserved: &super::super::System::Variant::VARIANT) -> windows_core::Result<()>;
    fn Roles(&self) -> windows_core::Result<IAzRoles>;
    fn OpenRole(&self, bstrrolename: &windows_core::BSTR, varreserved: &super::super::System::Variant::VARIANT) -> windows_core::Result<IAzRole>;
    fn CreateRole(&self, bstrrolename: &windows_core::BSTR, varreserved: &super::super::System::Variant::VARIANT) -> windows_core::Result<IAzRole>;
    fn DeleteRole(&self, bstrrolename: &windows_core::BSTR, varreserved: &super::super::System::Variant::VARIANT) -> windows_core::Result<()>;
    fn InitializeClientContextFromToken(&self, ulltokenhandle: u64, varreserved: &super::super::System::Variant::VARIANT) -> windows_core::Result<IAzClientContext>;
    fn AddPropertyItem(&self, lpropid: i32, varprop: &super::super::System::Variant::VARIANT, varreserved: &super::super::System::Variant::VARIANT) -> windows_core::Result<()>;
    fn DeletePropertyItem(&self, lpropid: i32, varprop: &super::super::System::Variant::VARIANT, varreserved: &super::super::System::Variant::VARIANT) -> windows_core::Result<()>;
    fn Submit(&self, lflags: i32, varreserved: &super::super::System::Variant::VARIANT) -> windows_core::Result<()>;
    fn InitializeClientContextFromName(&self, clientname: &windows_core::BSTR, domainname: &windows_core::BSTR, varreserved: &super::super::System::Variant::VARIANT) -> windows_core::Result<IAzClientContext>;
    fn DelegatedPolicyUsers(&self) -> windows_core::Result<super::super::System::Variant::VARIANT>;
    fn AddDelegatedPolicyUser(&self, bstrdelegatedpolicyuser: &windows_core::BSTR, varreserved: &super::super::System::Variant::VARIANT) -> windows_core::Result<()>;
    fn DeleteDelegatedPolicyUser(&self, bstrdelegatedpolicyuser: &windows_core::BSTR, varreserved: &super::super::System::Variant::VARIANT) -> windows_core::Result<()>;
    fn InitializeClientContextFromStringSid(&self, sidstring: &windows_core::BSTR, loptions: i32, varreserved: &super::super::System::Variant::VARIANT) -> windows_core::Result<IAzClientContext>;
    fn PolicyAdministratorsName(&self) -> windows_core::Result<super::super::System::Variant::VARIANT>;
    fn PolicyReadersName(&self) -> windows_core::Result<super::super::System::Variant::VARIANT>;
    fn AddPolicyAdministratorName(&self, bstradmin: &windows_core::BSTR, varreserved: &super::super::System::Variant::VARIANT) -> windows_core::Result<()>;
    fn DeletePolicyAdministratorName(&self, bstradmin: &windows_core::BSTR, varreserved: &super::super::System::Variant::VARIANT) -> windows_core::Result<()>;
    fn AddPolicyReaderName(&self, bstrreader: &windows_core::BSTR, varreserved: &super::super::System::Variant::VARIANT) -> windows_core::Result<()>;
    fn DeletePolicyReaderName(&self, bstrreader: &windows_core::BSTR, varreserved: &super::super::System::Variant::VARIANT) -> windows_core::Result<()>;
    fn DelegatedPolicyUsersName(&self) -> windows_core::Result<super::super::System::Variant::VARIANT>;
    fn AddDelegatedPolicyUserName(&self, bstrdelegatedpolicyuser: &windows_core::BSTR, varreserved: &super::super::System::Variant::VARIANT) -> windows_core::Result<()>;
    fn DeleteDelegatedPolicyUserName(&self, bstrdelegatedpolicyuser: &windows_core::BSTR, varreserved: &super::super::System::Variant::VARIANT) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IAzApplication_Vtbl {
    pub const fn new<Identity: IAzApplication_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Name<Identity: IAzApplication_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrname: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IAzApplication_Impl::Name(this) {
                    Ok(ok__) => {
                        pbstrname.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetName<Identity: IAzApplication_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrname: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IAzApplication_Impl::SetName(this, core::mem::transmute(&bstrname)).into()
            }
        }
        unsafe extern "system" fn Description<Identity: IAzApplication_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrdescription: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IAzApplication_Impl::Description(this) {
                    Ok(ok__) => {
                        pbstrdescription.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetDescription<Identity: IAzApplication_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrdescription: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IAzApplication_Impl::SetDescription(this, core::mem::transmute(&bstrdescription)).into()
            }
        }
        unsafe extern "system" fn ApplicationData<Identity: IAzApplication_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrapplicationdata: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IAzApplication_Impl::ApplicationData(this) {
                    Ok(ok__) => {
                        pbstrapplicationdata.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetApplicationData<Identity: IAzApplication_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrapplicationdata: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IAzApplication_Impl::SetApplicationData(this, core::mem::transmute(&bstrapplicationdata)).into()
            }
        }
        unsafe extern "system" fn AuthzInterfaceClsid<Identity: IAzApplication_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrprop: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IAzApplication_Impl::AuthzInterfaceClsid(this) {
                    Ok(ok__) => {
                        pbstrprop.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetAuthzInterfaceClsid<Identity: IAzApplication_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrprop: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IAzApplication_Impl::SetAuthzInterfaceClsid(this, core::mem::transmute(&bstrprop)).into()
            }
        }
        unsafe extern "system" fn Version<Identity: IAzApplication_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrprop: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IAzApplication_Impl::Version(this) {
                    Ok(ok__) => {
                        pbstrprop.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetVersion<Identity: IAzApplication_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrprop: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IAzApplication_Impl::SetVersion(this, core::mem::transmute(&bstrprop)).into()
            }
        }
        unsafe extern "system" fn GenerateAudits<Identity: IAzApplication_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbprop: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IAzApplication_Impl::GenerateAudits(this) {
                    Ok(ok__) => {
                        pbprop.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetGenerateAudits<Identity: IAzApplication_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bprop: windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IAzApplication_Impl::SetGenerateAudits(this, core::mem::transmute_copy(&bprop)).into()
            }
        }
        unsafe extern "system" fn ApplyStoreSacl<Identity: IAzApplication_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbprop: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IAzApplication_Impl::ApplyStoreSacl(this) {
                    Ok(ok__) => {
                        pbprop.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetApplyStoreSacl<Identity: IAzApplication_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bprop: windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IAzApplication_Impl::SetApplyStoreSacl(this, core::mem::transmute_copy(&bprop)).into()
            }
        }
        unsafe extern "system" fn Writable<Identity: IAzApplication_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pfprop: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IAzApplication_Impl::Writable(this) {
                    Ok(ok__) => {
                        pfprop.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetProperty<Identity: IAzApplication_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lpropid: i32, varreserved: super::super::System::Variant::VARIANT, pvarprop: *mut super::super::System::Variant::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IAzApplication_Impl::GetProperty(this, core::mem::transmute_copy(&lpropid), core::mem::transmute(&varreserved)) {
                    Ok(ok__) => {
                        pvarprop.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetProperty<Identity: IAzApplication_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lpropid: i32, varprop: super::super::System::Variant::VARIANT, varreserved: super::super::System::Variant::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IAzApplication_Impl::SetProperty(this, core::mem::transmute_copy(&lpropid), core::mem::transmute(&varprop), core::mem::transmute(&varreserved)).into()
            }
        }
        unsafe extern "system" fn PolicyAdministrators<Identity: IAzApplication_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvaradmins: *mut super::super::System::Variant::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IAzApplication_Impl::PolicyAdministrators(this) {
                    Ok(ok__) => {
                        pvaradmins.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn PolicyReaders<Identity: IAzApplication_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvarreaders: *mut super::super::System::Variant::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IAzApplication_Impl::PolicyReaders(this) {
                    Ok(ok__) => {
                        pvarreaders.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn AddPolicyAdministrator<Identity: IAzApplication_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstradmin: *mut core::ffi::c_void, varreserved: super::super::System::Variant::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IAzApplication_Impl::AddPolicyAdministrator(this, core::mem::transmute(&bstradmin), core::mem::transmute(&varreserved)).into()
            }
        }
        unsafe extern "system" fn DeletePolicyAdministrator<Identity: IAzApplication_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstradmin: *mut core::ffi::c_void, varreserved: super::super::System::Variant::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IAzApplication_Impl::DeletePolicyAdministrator(this, core::mem::transmute(&bstradmin), core::mem::transmute(&varreserved)).into()
            }
        }
        unsafe extern "system" fn AddPolicyReader<Identity: IAzApplication_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrreader: *mut core::ffi::c_void, varreserved: super::super::System::Variant::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IAzApplication_Impl::AddPolicyReader(this, core::mem::transmute(&bstrreader), core::mem::transmute(&varreserved)).into()
            }
        }
        unsafe extern "system" fn DeletePolicyReader<Identity: IAzApplication_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrreader: *mut core::ffi::c_void, varreserved: super::super::System::Variant::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IAzApplication_Impl::DeletePolicyReader(this, core::mem::transmute(&bstrreader), core::mem::transmute(&varreserved)).into()
            }
        }
        unsafe extern "system" fn Scopes<Identity: IAzApplication_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppscopecollection: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IAzApplication_Impl::Scopes(this) {
                    Ok(ok__) => {
                        ppscopecollection.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn OpenScope<Identity: IAzApplication_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrscopename: *mut core::ffi::c_void, varreserved: super::super::System::Variant::VARIANT, ppscope: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IAzApplication_Impl::OpenScope(this, core::mem::transmute(&bstrscopename), core::mem::transmute(&varreserved)) {
                    Ok(ok__) => {
                        ppscope.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CreateScope<Identity: IAzApplication_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrscopename: *mut core::ffi::c_void, varreserved: super::super::System::Variant::VARIANT, ppscope: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IAzApplication_Impl::CreateScope(this, core::mem::transmute(&bstrscopename), core::mem::transmute(&varreserved)) {
                    Ok(ok__) => {
                        ppscope.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn DeleteScope<Identity: IAzApplication_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrscopename: *mut core::ffi::c_void, varreserved: super::super::System::Variant::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IAzApplication_Impl::DeleteScope(this, core::mem::transmute(&bstrscopename), core::mem::transmute(&varreserved)).into()
            }
        }
        unsafe extern "system" fn Operations<Identity: IAzApplication_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppoperationcollection: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IAzApplication_Impl::Operations(this) {
                    Ok(ok__) => {
                        ppoperationcollection.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn OpenOperation<Identity: IAzApplication_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstroperationname: *mut core::ffi::c_void, varreserved: super::super::System::Variant::VARIANT, ppoperation: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IAzApplication_Impl::OpenOperation(this, core::mem::transmute(&bstroperationname), core::mem::transmute(&varreserved)) {
                    Ok(ok__) => {
                        ppoperation.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CreateOperation<Identity: IAzApplication_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstroperationname: *mut core::ffi::c_void, varreserved: super::super::System::Variant::VARIANT, ppoperation: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IAzApplication_Impl::CreateOperation(this, core::mem::transmute(&bstroperationname), core::mem::transmute(&varreserved)) {
                    Ok(ok__) => {
                        ppoperation.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn DeleteOperation<Identity: IAzApplication_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstroperationname: *mut core::ffi::c_void, varreserved: super::super::System::Variant::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IAzApplication_Impl::DeleteOperation(this, core::mem::transmute(&bstroperationname), core::mem::transmute(&varreserved)).into()
            }
        }
        unsafe extern "system" fn Tasks<Identity: IAzApplication_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pptaskcollection: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IAzApplication_Impl::Tasks(this) {
                    Ok(ok__) => {
                        pptaskcollection.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn OpenTask<Identity: IAzApplication_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrtaskname: *mut core::ffi::c_void, varreserved: super::super::System::Variant::VARIANT, pptask: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IAzApplication_Impl::OpenTask(this, core::mem::transmute(&bstrtaskname), core::mem::transmute(&varreserved)) {
                    Ok(ok__) => {
                        pptask.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CreateTask<Identity: IAzApplication_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrtaskname: *mut core::ffi::c_void, varreserved: super::super::System::Variant::VARIANT, pptask: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IAzApplication_Impl::CreateTask(this, core::mem::transmute(&bstrtaskname), core::mem::transmute(&varreserved)) {
                    Ok(ok__) => {
                        pptask.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn DeleteTask<Identity: IAzApplication_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrtaskname: *mut core::ffi::c_void, varreserved: super::super::System::Variant::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IAzApplication_Impl::DeleteTask(this, core::mem::transmute(&bstrtaskname), core::mem::transmute(&varreserved)).into()
            }
        }
        unsafe extern "system" fn ApplicationGroups<Identity: IAzApplication_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppgroupcollection: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IAzApplication_Impl::ApplicationGroups(this) {
                    Ok(ok__) => {
                        ppgroupcollection.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn OpenApplicationGroup<Identity: IAzApplication_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrgroupname: *mut core::ffi::c_void, varreserved: super::super::System::Variant::VARIANT, ppgroup: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IAzApplication_Impl::OpenApplicationGroup(this, core::mem::transmute(&bstrgroupname), core::mem::transmute(&varreserved)) {
                    Ok(ok__) => {
                        ppgroup.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CreateApplicationGroup<Identity: IAzApplication_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrgroupname: *mut core::ffi::c_void, varreserved: super::super::System::Variant::VARIANT, ppgroup: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IAzApplication_Impl::CreateApplicationGroup(this, core::mem::transmute(&bstrgroupname), core::mem::transmute(&varreserved)) {
                    Ok(ok__) => {
                        ppgroup.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn DeleteApplicationGroup<Identity: IAzApplication_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrgroupname: *mut core::ffi::c_void, varreserved: super::super::System::Variant::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IAzApplication_Impl::DeleteApplicationGroup(this, core::mem::transmute(&bstrgroupname), core::mem::transmute(&varreserved)).into()
            }
        }
        unsafe extern "system" fn Roles<Identity: IAzApplication_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pprolecollection: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IAzApplication_Impl::Roles(this) {
                    Ok(ok__) => {
                        pprolecollection.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn OpenRole<Identity: IAzApplication_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrrolename: *mut core::ffi::c_void, varreserved: super::super::System::Variant::VARIANT, pprole: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IAzApplication_Impl::OpenRole(this, core::mem::transmute(&bstrrolename), core::mem::transmute(&varreserved)) {
                    Ok(ok__) => {
                        pprole.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CreateRole<Identity: IAzApplication_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrrolename: *mut core::ffi::c_void, varreserved: super::super::System::Variant::VARIANT, pprole: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IAzApplication_Impl::CreateRole(this, core::mem::transmute(&bstrrolename), core::mem::transmute(&varreserved)) {
                    Ok(ok__) => {
                        pprole.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn DeleteRole<Identity: IAzApplication_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrrolename: *mut core::ffi::c_void, varreserved: super::super::System::Variant::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IAzApplication_Impl::DeleteRole(this, core::mem::transmute(&bstrrolename), core::mem::transmute(&varreserved)).into()
            }
        }
        unsafe extern "system" fn InitializeClientContextFromToken<Identity: IAzApplication_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ulltokenhandle: u64, varreserved: super::super::System::Variant::VARIANT, ppclientcontext: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IAzApplication_Impl::InitializeClientContextFromToken(this, core::mem::transmute_copy(&ulltokenhandle), core::mem::transmute(&varreserved)) {
                    Ok(ok__) => {
                        ppclientcontext.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn AddPropertyItem<Identity: IAzApplication_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lpropid: i32, varprop: super::super::System::Variant::VARIANT, varreserved: super::super::System::Variant::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IAzApplication_Impl::AddPropertyItem(this, core::mem::transmute_copy(&lpropid), core::mem::transmute(&varprop), core::mem::transmute(&varreserved)).into()
            }
        }
        unsafe extern "system" fn DeletePropertyItem<Identity: IAzApplication_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lpropid: i32, varprop: super::super::System::Variant::VARIANT, varreserved: super::super::System::Variant::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IAzApplication_Impl::DeletePropertyItem(this, core::mem::transmute_copy(&lpropid), core::mem::transmute(&varprop), core::mem::transmute(&varreserved)).into()
            }
        }
        unsafe extern "system" fn Submit<Identity: IAzApplication_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lflags: i32, varreserved: super::super::System::Variant::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IAzApplication_Impl::Submit(this, core::mem::transmute_copy(&lflags), core::mem::transmute(&varreserved)).into()
            }
        }
        unsafe extern "system" fn InitializeClientContextFromName<Identity: IAzApplication_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, clientname: *mut core::ffi::c_void, domainname: *mut core::ffi::c_void, varreserved: super::super::System::Variant::VARIANT, ppclientcontext: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IAzApplication_Impl::InitializeClientContextFromName(this, core::mem::transmute(&clientname), core::mem::transmute(&domainname), core::mem::transmute(&varreserved)) {
                    Ok(ok__) => {
                        ppclientcontext.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn DelegatedPolicyUsers<Identity: IAzApplication_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvardelegatedpolicyusers: *mut super::super::System::Variant::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IAzApplication_Impl::DelegatedPolicyUsers(this) {
                    Ok(ok__) => {
                        pvardelegatedpolicyusers.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn AddDelegatedPolicyUser<Identity: IAzApplication_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrdelegatedpolicyuser: *mut core::ffi::c_void, varreserved: super::super::System::Variant::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IAzApplication_Impl::AddDelegatedPolicyUser(this, core::mem::transmute(&bstrdelegatedpolicyuser), core::mem::transmute(&varreserved)).into()
            }
        }
        unsafe extern "system" fn DeleteDelegatedPolicyUser<Identity: IAzApplication_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrdelegatedpolicyuser: *mut core::ffi::c_void, varreserved: super::super::System::Variant::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IAzApplication_Impl::DeleteDelegatedPolicyUser(this, core::mem::transmute(&bstrdelegatedpolicyuser), core::mem::transmute(&varreserved)).into()
            }
        }
        unsafe extern "system" fn InitializeClientContextFromStringSid<Identity: IAzApplication_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, sidstring: *mut core::ffi::c_void, loptions: i32, varreserved: super::super::System::Variant::VARIANT, ppclientcontext: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IAzApplication_Impl::InitializeClientContextFromStringSid(this, core::mem::transmute(&sidstring), core::mem::transmute_copy(&loptions), core::mem::transmute(&varreserved)) {
                    Ok(ok__) => {
                        ppclientcontext.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn PolicyAdministratorsName<Identity: IAzApplication_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvaradmins: *mut super::super::System::Variant::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IAzApplication_Impl::PolicyAdministratorsName(this) {
                    Ok(ok__) => {
                        pvaradmins.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn PolicyReadersName<Identity: IAzApplication_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvarreaders: *mut super::super::System::Variant::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IAzApplication_Impl::PolicyReadersName(this) {
                    Ok(ok__) => {
                        pvarreaders.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn AddPolicyAdministratorName<Identity: IAzApplication_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstradmin: *mut core::ffi::c_void, varreserved: super::super::System::Variant::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IAzApplication_Impl::AddPolicyAdministratorName(this, core::mem::transmute(&bstradmin), core::mem::transmute(&varreserved)).into()
            }
        }
        unsafe extern "system" fn DeletePolicyAdministratorName<Identity: IAzApplication_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstradmin: *mut core::ffi::c_void, varreserved: super::super::System::Variant::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IAzApplication_Impl::DeletePolicyAdministratorName(this, core::mem::transmute(&bstradmin), core::mem::transmute(&varreserved)).into()
            }
        }
        unsafe extern "system" fn AddPolicyReaderName<Identity: IAzApplication_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrreader: *mut core::ffi::c_void, varreserved: super::super::System::Variant::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IAzApplication_Impl::AddPolicyReaderName(this, core::mem::transmute(&bstrreader), core::mem::transmute(&varreserved)).into()
            }
        }
        unsafe extern "system" fn DeletePolicyReaderName<Identity: IAzApplication_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrreader: *mut core::ffi::c_void, varreserved: super::super::System::Variant::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IAzApplication_Impl::DeletePolicyReaderName(this, core::mem::transmute(&bstrreader), core::mem::transmute(&varreserved)).into()
            }
        }
        unsafe extern "system" fn DelegatedPolicyUsersName<Identity: IAzApplication_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvardelegatedpolicyusers: *mut super::super::System::Variant::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IAzApplication_Impl::DelegatedPolicyUsersName(this) {
                    Ok(ok__) => {
                        pvardelegatedpolicyusers.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn AddDelegatedPolicyUserName<Identity: IAzApplication_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrdelegatedpolicyuser: *mut core::ffi::c_void, varreserved: super::super::System::Variant::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IAzApplication_Impl::AddDelegatedPolicyUserName(this, core::mem::transmute(&bstrdelegatedpolicyuser), core::mem::transmute(&varreserved)).into()
            }
        }
        unsafe extern "system" fn DeleteDelegatedPolicyUserName<Identity: IAzApplication_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrdelegatedpolicyuser: *mut core::ffi::c_void, varreserved: super::super::System::Variant::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IAzApplication_Impl::DeleteDelegatedPolicyUserName(this, core::mem::transmute(&bstrdelegatedpolicyuser), core::mem::transmute(&varreserved)).into()
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            Name: Name::<Identity, OFFSET>,
            SetName: SetName::<Identity, OFFSET>,
            Description: Description::<Identity, OFFSET>,
            SetDescription: SetDescription::<Identity, OFFSET>,
            ApplicationData: ApplicationData::<Identity, OFFSET>,
            SetApplicationData: SetApplicationData::<Identity, OFFSET>,
            AuthzInterfaceClsid: AuthzInterfaceClsid::<Identity, OFFSET>,
            SetAuthzInterfaceClsid: SetAuthzInterfaceClsid::<Identity, OFFSET>,
            Version: Version::<Identity, OFFSET>,
            SetVersion: SetVersion::<Identity, OFFSET>,
            GenerateAudits: GenerateAudits::<Identity, OFFSET>,
            SetGenerateAudits: SetGenerateAudits::<Identity, OFFSET>,
            ApplyStoreSacl: ApplyStoreSacl::<Identity, OFFSET>,
            SetApplyStoreSacl: SetApplyStoreSacl::<Identity, OFFSET>,
            Writable: Writable::<Identity, OFFSET>,
            GetProperty: GetProperty::<Identity, OFFSET>,
            SetProperty: SetProperty::<Identity, OFFSET>,
            PolicyAdministrators: PolicyAdministrators::<Identity, OFFSET>,
            PolicyReaders: PolicyReaders::<Identity, OFFSET>,
            AddPolicyAdministrator: AddPolicyAdministrator::<Identity, OFFSET>,
            DeletePolicyAdministrator: DeletePolicyAdministrator::<Identity, OFFSET>,
            AddPolicyReader: AddPolicyReader::<Identity, OFFSET>,
            DeletePolicyReader: DeletePolicyReader::<Identity, OFFSET>,
            Scopes: Scopes::<Identity, OFFSET>,
            OpenScope: OpenScope::<Identity, OFFSET>,
            CreateScope: CreateScope::<Identity, OFFSET>,
            DeleteScope: DeleteScope::<Identity, OFFSET>,
            Operations: Operations::<Identity, OFFSET>,
            OpenOperation: OpenOperation::<Identity, OFFSET>,
            CreateOperation: CreateOperation::<Identity, OFFSET>,
            DeleteOperation: DeleteOperation::<Identity, OFFSET>,
            Tasks: Tasks::<Identity, OFFSET>,
            OpenTask: OpenTask::<Identity, OFFSET>,
            CreateTask: CreateTask::<Identity, OFFSET>,
            DeleteTask: DeleteTask::<Identity, OFFSET>,
            ApplicationGroups: ApplicationGroups::<Identity, OFFSET>,
            OpenApplicationGroup: OpenApplicationGroup::<Identity, OFFSET>,
            CreateApplicationGroup: CreateApplicationGroup::<Identity, OFFSET>,
            DeleteApplicationGroup: DeleteApplicationGroup::<Identity, OFFSET>,
            Roles: Roles::<Identity, OFFSET>,
            OpenRole: OpenRole::<Identity, OFFSET>,
            CreateRole: CreateRole::<Identity, OFFSET>,
            DeleteRole: DeleteRole::<Identity, OFFSET>,
            InitializeClientContextFromToken: InitializeClientContextFromToken::<Identity, OFFSET>,
            AddPropertyItem: AddPropertyItem::<Identity, OFFSET>,
            DeletePropertyItem: DeletePropertyItem::<Identity, OFFSET>,
            Submit: Submit::<Identity, OFFSET>,
            InitializeClientContextFromName: InitializeClientContextFromName::<Identity, OFFSET>,
            DelegatedPolicyUsers: DelegatedPolicyUsers::<Identity, OFFSET>,
            AddDelegatedPolicyUser: AddDelegatedPolicyUser::<Identity, OFFSET>,
            DeleteDelegatedPolicyUser: DeleteDelegatedPolicyUser::<Identity, OFFSET>,
            InitializeClientContextFromStringSid: InitializeClientContextFromStringSid::<Identity, OFFSET>,
            PolicyAdministratorsName: PolicyAdministratorsName::<Identity, OFFSET>,
            PolicyReadersName: PolicyReadersName::<Identity, OFFSET>,
            AddPolicyAdministratorName: AddPolicyAdministratorName::<Identity, OFFSET>,
            DeletePolicyAdministratorName: DeletePolicyAdministratorName::<Identity, OFFSET>,
            AddPolicyReaderName: AddPolicyReaderName::<Identity, OFFSET>,
            DeletePolicyReaderName: DeletePolicyReaderName::<Identity, OFFSET>,
            DelegatedPolicyUsersName: DelegatedPolicyUsersName::<Identity, OFFSET>,
            AddDelegatedPolicyUserName: AddDelegatedPolicyUserName::<Identity, OFFSET>,
            DeleteDelegatedPolicyUserName: DeleteDelegatedPolicyUserName::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IAzApplication as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for IAzApplication {}
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
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn InitializeClientContextFromToken2(&self, ultokenhandlelowpart: u32, ultokenhandlehighpart: u32, varreserved: &super::super::System::Variant::VARIANT) -> windows_core::Result<IAzClientContext2> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).InitializeClientContextFromToken2)(windows_core::Interface::as_raw(self), ultokenhandlelowpart, ultokenhandlehighpart, core::mem::transmute_copy(varreserved), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn InitializeClientContext2(&self, identifyingstring: &windows_core::BSTR, varreserved: &super::super::System::Variant::VARIANT) -> windows_core::Result<IAzClientContext2> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).InitializeClientContext2)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(identifyingstring), core::mem::transmute_copy(varreserved), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IAzApplication2_Vtbl {
    pub base__: IAzApplication_Vtbl,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub InitializeClientContextFromToken2: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, super::super::System::Variant::VARIANT, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    InitializeClientContextFromToken2: usize,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub InitializeClientContext2: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, super::super::System::Variant::VARIANT, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    InitializeClientContext2: usize,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IAzApplication2_Impl: IAzApplication_Impl {
    fn InitializeClientContextFromToken2(&self, ultokenhandlelowpart: u32, ultokenhandlehighpart: u32, varreserved: &super::super::System::Variant::VARIANT) -> windows_core::Result<IAzClientContext2>;
    fn InitializeClientContext2(&self, identifyingstring: &windows_core::BSTR, varreserved: &super::super::System::Variant::VARIANT) -> windows_core::Result<IAzClientContext2>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IAzApplication2_Vtbl {
    pub const fn new<Identity: IAzApplication2_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn InitializeClientContextFromToken2<Identity: IAzApplication2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ultokenhandlelowpart: u32, ultokenhandlehighpart: u32, varreserved: super::super::System::Variant::VARIANT, ppclientcontext: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IAzApplication2_Impl::InitializeClientContextFromToken2(this, core::mem::transmute_copy(&ultokenhandlelowpart), core::mem::transmute_copy(&ultokenhandlehighpart), core::mem::transmute(&varreserved)) {
                    Ok(ok__) => {
                        ppclientcontext.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn InitializeClientContext2<Identity: IAzApplication2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, identifyingstring: *mut core::ffi::c_void, varreserved: super::super::System::Variant::VARIANT, ppclientcontext: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IAzApplication2_Impl::InitializeClientContext2(this, core::mem::transmute(&identifyingstring), core::mem::transmute(&varreserved)) {
                    Ok(ok__) => {
                        ppclientcontext.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: IAzApplication_Vtbl::new::<Identity, OFFSET>(),
            InitializeClientContextFromToken2: InitializeClientContextFromToken2::<Identity, OFFSET>,
            InitializeClientContext2: InitializeClientContext2::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IAzApplication2 as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID || iid == &<IAzApplication as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for IAzApplication2 {}
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
    pub unsafe fn ScopeExists(&self, bstrscopename: &windows_core::BSTR) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ScopeExists)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrscopename), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn OpenScope2(&self, bstrscopename: &windows_core::BSTR) -> windows_core::Result<IAzScope2> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).OpenScope2)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrscopename), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn CreateScope2(&self, bstrscopename: &windows_core::BSTR) -> windows_core::Result<IAzScope2> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateScope2)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrscopename), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn DeleteScope2(&self, bstrscopename: &windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).DeleteScope2)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrscopename)).ok() }
    }
    pub unsafe fn RoleDefinitions(&self) -> windows_core::Result<IAzRoleDefinitions> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).RoleDefinitions)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn CreateRoleDefinition(&self, bstrroledefinitionname: &windows_core::BSTR) -> windows_core::Result<IAzRoleDefinition> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateRoleDefinition)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrroledefinitionname), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn OpenRoleDefinition(&self, bstrroledefinitionname: &windows_core::BSTR) -> windows_core::Result<IAzRoleDefinition> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).OpenRoleDefinition)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrroledefinitionname), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn DeleteRoleDefinition(&self, bstrroledefinitionname: &windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).DeleteRoleDefinition)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrroledefinitionname)).ok() }
    }
    pub unsafe fn RoleAssignments(&self) -> windows_core::Result<IAzRoleAssignments> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).RoleAssignments)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn CreateRoleAssignment(&self, bstrroleassignmentname: &windows_core::BSTR) -> windows_core::Result<IAzRoleAssignment> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateRoleAssignment)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrroleassignmentname), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn OpenRoleAssignment(&self, bstrroleassignmentname: &windows_core::BSTR) -> windows_core::Result<IAzRoleAssignment> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).OpenRoleAssignment)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrroleassignmentname), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn DeleteRoleAssignment(&self, bstrroleassignmentname: &windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).DeleteRoleAssignment)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrroleassignmentname)).ok() }
    }
    pub unsafe fn BizRulesEnabled(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).BizRulesEnabled)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetBizRulesEnabled(&self, benabled: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetBizRulesEnabled)(windows_core::Interface::as_raw(self), benabled).ok() }
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IAzApplication3_Vtbl {
    pub base__: IAzApplication2_Vtbl,
    pub ScopeExists: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub OpenScope2: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateScope2: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub DeleteScope2: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub RoleDefinitions: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateRoleDefinition: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub OpenRoleDefinition: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub DeleteRoleDefinition: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub RoleAssignments: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateRoleAssignment: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub OpenRoleAssignment: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub DeleteRoleAssignment: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub BizRulesEnabled: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub SetBizRulesEnabled: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IAzApplication3_Impl: IAzApplication2_Impl {
    fn ScopeExists(&self, bstrscopename: &windows_core::BSTR) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn OpenScope2(&self, bstrscopename: &windows_core::BSTR) -> windows_core::Result<IAzScope2>;
    fn CreateScope2(&self, bstrscopename: &windows_core::BSTR) -> windows_core::Result<IAzScope2>;
    fn DeleteScope2(&self, bstrscopename: &windows_core::BSTR) -> windows_core::Result<()>;
    fn RoleDefinitions(&self) -> windows_core::Result<IAzRoleDefinitions>;
    fn CreateRoleDefinition(&self, bstrroledefinitionname: &windows_core::BSTR) -> windows_core::Result<IAzRoleDefinition>;
    fn OpenRoleDefinition(&self, bstrroledefinitionname: &windows_core::BSTR) -> windows_core::Result<IAzRoleDefinition>;
    fn DeleteRoleDefinition(&self, bstrroledefinitionname: &windows_core::BSTR) -> windows_core::Result<()>;
    fn RoleAssignments(&self) -> windows_core::Result<IAzRoleAssignments>;
    fn CreateRoleAssignment(&self, bstrroleassignmentname: &windows_core::BSTR) -> windows_core::Result<IAzRoleAssignment>;
    fn OpenRoleAssignment(&self, bstrroleassignmentname: &windows_core::BSTR) -> windows_core::Result<IAzRoleAssignment>;
    fn DeleteRoleAssignment(&self, bstrroleassignmentname: &windows_core::BSTR) -> windows_core::Result<()>;
    fn BizRulesEnabled(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn SetBizRulesEnabled(&self, benabled: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IAzApplication3_Vtbl {
    pub const fn new<Identity: IAzApplication3_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn ScopeExists<Identity: IAzApplication3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrscopename: *mut core::ffi::c_void, pbexist: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IAzApplication3_Impl::ScopeExists(this, core::mem::transmute(&bstrscopename)) {
                    Ok(ok__) => {
                        pbexist.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn OpenScope2<Identity: IAzApplication3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrscopename: *mut core::ffi::c_void, ppscope2: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IAzApplication3_Impl::OpenScope2(this, core::mem::transmute(&bstrscopename)) {
                    Ok(ok__) => {
                        ppscope2.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CreateScope2<Identity: IAzApplication3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrscopename: *mut core::ffi::c_void, ppscope2: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IAzApplication3_Impl::CreateScope2(this, core::mem::transmute(&bstrscopename)) {
                    Ok(ok__) => {
                        ppscope2.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn DeleteScope2<Identity: IAzApplication3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrscopename: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IAzApplication3_Impl::DeleteScope2(this, core::mem::transmute(&bstrscopename)).into()
            }
        }
        unsafe extern "system" fn RoleDefinitions<Identity: IAzApplication3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pproledefinitions: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IAzApplication3_Impl::RoleDefinitions(this) {
                    Ok(ok__) => {
                        pproledefinitions.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CreateRoleDefinition<Identity: IAzApplication3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrroledefinitionname: *mut core::ffi::c_void, pproledefinitions: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IAzApplication3_Impl::CreateRoleDefinition(this, core::mem::transmute(&bstrroledefinitionname)) {
                    Ok(ok__) => {
                        pproledefinitions.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn OpenRoleDefinition<Identity: IAzApplication3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrroledefinitionname: *mut core::ffi::c_void, pproledefinitions: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IAzApplication3_Impl::OpenRoleDefinition(this, core::mem::transmute(&bstrroledefinitionname)) {
                    Ok(ok__) => {
                        pproledefinitions.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn DeleteRoleDefinition<Identity: IAzApplication3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrroledefinitionname: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IAzApplication3_Impl::DeleteRoleDefinition(this, core::mem::transmute(&bstrroledefinitionname)).into()
            }
        }
        unsafe extern "system" fn RoleAssignments<Identity: IAzApplication3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pproleassignments: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IAzApplication3_Impl::RoleAssignments(this) {
                    Ok(ok__) => {
                        pproleassignments.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CreateRoleAssignment<Identity: IAzApplication3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrroleassignmentname: *mut core::ffi::c_void, pproleassignment: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IAzApplication3_Impl::CreateRoleAssignment(this, core::mem::transmute(&bstrroleassignmentname)) {
                    Ok(ok__) => {
                        pproleassignment.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn OpenRoleAssignment<Identity: IAzApplication3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrroleassignmentname: *mut core::ffi::c_void, pproleassignment: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IAzApplication3_Impl::OpenRoleAssignment(this, core::mem::transmute(&bstrroleassignmentname)) {
                    Ok(ok__) => {
                        pproleassignment.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn DeleteRoleAssignment<Identity: IAzApplication3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrroleassignmentname: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IAzApplication3_Impl::DeleteRoleAssignment(this, core::mem::transmute(&bstrroleassignmentname)).into()
            }
        }
        unsafe extern "system" fn BizRulesEnabled<Identity: IAzApplication3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbenabled: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IAzApplication3_Impl::BizRulesEnabled(this) {
                    Ok(ok__) => {
                        pbenabled.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetBizRulesEnabled<Identity: IAzApplication3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, benabled: super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IAzApplication3_Impl::SetBizRulesEnabled(this, core::mem::transmute_copy(&benabled)).into()
            }
        }
        Self {
            base__: IAzApplication2_Vtbl::new::<Identity, OFFSET>(),
            ScopeExists: ScopeExists::<Identity, OFFSET>,
            OpenScope2: OpenScope2::<Identity, OFFSET>,
            CreateScope2: CreateScope2::<Identity, OFFSET>,
            DeleteScope2: DeleteScope2::<Identity, OFFSET>,
            RoleDefinitions: RoleDefinitions::<Identity, OFFSET>,
            CreateRoleDefinition: CreateRoleDefinition::<Identity, OFFSET>,
            OpenRoleDefinition: OpenRoleDefinition::<Identity, OFFSET>,
            DeleteRoleDefinition: DeleteRoleDefinition::<Identity, OFFSET>,
            RoleAssignments: RoleAssignments::<Identity, OFFSET>,
            CreateRoleAssignment: CreateRoleAssignment::<Identity, OFFSET>,
            OpenRoleAssignment: OpenRoleAssignment::<Identity, OFFSET>,
            DeleteRoleAssignment: DeleteRoleAssignment::<Identity, OFFSET>,
            BizRulesEnabled: BizRulesEnabled::<Identity, OFFSET>,
            SetBizRulesEnabled: SetBizRulesEnabled::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IAzApplication3 as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID || iid == &<IAzApplication as windows_core::Interface>::IID || iid == &<IAzApplication2 as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for IAzApplication3 {}
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
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Name)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn SetName(&self, bstrname: &windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetName)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrname)).ok() }
    }
    pub unsafe fn Type(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Type)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetType(&self, lprop: i32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetType)(windows_core::Interface::as_raw(self), lprop).ok() }
    }
    pub unsafe fn LdapQuery(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).LdapQuery)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn SetLdapQuery(&self, bstrprop: &windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetLdapQuery)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrprop)).ok() }
    }
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn AppMembers(&self) -> windows_core::Result<super::super::System::Variant::VARIANT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).AppMembers)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn AppNonMembers(&self) -> windows_core::Result<super::super::System::Variant::VARIANT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).AppNonMembers)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn Members(&self) -> windows_core::Result<super::super::System::Variant::VARIANT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Members)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn NonMembers(&self) -> windows_core::Result<super::super::System::Variant::VARIANT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).NonMembers)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
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
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn AddAppMember(&self, bstrprop: &windows_core::BSTR, varreserved: &super::super::System::Variant::VARIANT) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).AddAppMember)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrprop), core::mem::transmute_copy(varreserved)).ok() }
    }
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn DeleteAppMember(&self, bstrprop: &windows_core::BSTR, varreserved: &super::super::System::Variant::VARIANT) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).DeleteAppMember)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrprop), core::mem::transmute_copy(varreserved)).ok() }
    }
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn AddAppNonMember(&self, bstrprop: &windows_core::BSTR, varreserved: &super::super::System::Variant::VARIANT) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).AddAppNonMember)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrprop), core::mem::transmute_copy(varreserved)).ok() }
    }
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn DeleteAppNonMember(&self, bstrprop: &windows_core::BSTR, varreserved: &super::super::System::Variant::VARIANT) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).DeleteAppNonMember)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrprop), core::mem::transmute_copy(varreserved)).ok() }
    }
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn AddMember(&self, bstrprop: &windows_core::BSTR, varreserved: &super::super::System::Variant::VARIANT) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).AddMember)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrprop), core::mem::transmute_copy(varreserved)).ok() }
    }
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn DeleteMember(&self, bstrprop: &windows_core::BSTR, varreserved: &super::super::System::Variant::VARIANT) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).DeleteMember)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrprop), core::mem::transmute_copy(varreserved)).ok() }
    }
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn AddNonMember(&self, bstrprop: &windows_core::BSTR, varreserved: &super::super::System::Variant::VARIANT) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).AddNonMember)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrprop), core::mem::transmute_copy(varreserved)).ok() }
    }
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn DeleteNonMember(&self, bstrprop: &windows_core::BSTR, varreserved: &super::super::System::Variant::VARIANT) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).DeleteNonMember)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrprop), core::mem::transmute_copy(varreserved)).ok() }
    }
    pub unsafe fn Writable(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Writable)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn GetProperty(&self, lpropid: i32, varreserved: &super::super::System::Variant::VARIANT) -> windows_core::Result<super::super::System::Variant::VARIANT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetProperty)(windows_core::Interface::as_raw(self), lpropid, core::mem::transmute_copy(varreserved), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn SetProperty(&self, lpropid: i32, varprop: &super::super::System::Variant::VARIANT, varreserved: &super::super::System::Variant::VARIANT) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetProperty)(windows_core::Interface::as_raw(self), lpropid, core::mem::transmute_copy(varprop), core::mem::transmute_copy(varreserved)).ok() }
    }
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn AddPropertyItem(&self, lpropid: i32, varprop: &super::super::System::Variant::VARIANT, varreserved: &super::super::System::Variant::VARIANT) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).AddPropertyItem)(windows_core::Interface::as_raw(self), lpropid, core::mem::transmute_copy(varprop), core::mem::transmute_copy(varreserved)).ok() }
    }
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn DeletePropertyItem(&self, lpropid: i32, varprop: &super::super::System::Variant::VARIANT, varreserved: &super::super::System::Variant::VARIANT) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).DeletePropertyItem)(windows_core::Interface::as_raw(self), lpropid, core::mem::transmute_copy(varprop), core::mem::transmute_copy(varreserved)).ok() }
    }
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn Submit(&self, lflags: i32, varreserved: &super::super::System::Variant::VARIANT) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Submit)(windows_core::Interface::as_raw(self), lflags, core::mem::transmute_copy(varreserved)).ok() }
    }
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn AddMemberName(&self, bstrprop: &windows_core::BSTR, varreserved: &super::super::System::Variant::VARIANT) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).AddMemberName)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrprop), core::mem::transmute_copy(varreserved)).ok() }
    }
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn DeleteMemberName(&self, bstrprop: &windows_core::BSTR, varreserved: &super::super::System::Variant::VARIANT) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).DeleteMemberName)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrprop), core::mem::transmute_copy(varreserved)).ok() }
    }
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn AddNonMemberName(&self, bstrprop: &windows_core::BSTR, varreserved: &super::super::System::Variant::VARIANT) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).AddNonMemberName)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrprop), core::mem::transmute_copy(varreserved)).ok() }
    }
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn DeleteNonMemberName(&self, bstrprop: &windows_core::BSTR, varreserved: &super::super::System::Variant::VARIANT) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).DeleteNonMemberName)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrprop), core::mem::transmute_copy(varreserved)).ok() }
    }
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn MembersName(&self) -> windows_core::Result<super::super::System::Variant::VARIANT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).MembersName)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn NonMembersName(&self) -> windows_core::Result<super::super::System::Variant::VARIANT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).NonMembersName)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IAzApplicationGroup_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub Name: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Type: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetType: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub LdapQuery: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetLdapQuery: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub AppMembers: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::System::Variant::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    AppMembers: usize,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub AppNonMembers: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::System::Variant::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    AppNonMembers: usize,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub Members: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::System::Variant::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    Members: usize,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub NonMembers: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::System::Variant::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    NonMembers: usize,
    pub Description: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetDescription: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub AddAppMember: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, super::super::System::Variant::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    AddAppMember: usize,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub DeleteAppMember: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, super::super::System::Variant::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    DeleteAppMember: usize,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub AddAppNonMember: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, super::super::System::Variant::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    AddAppNonMember: usize,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub DeleteAppNonMember: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, super::super::System::Variant::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    DeleteAppNonMember: usize,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub AddMember: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, super::super::System::Variant::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    AddMember: usize,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub DeleteMember: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, super::super::System::Variant::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    DeleteMember: usize,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub AddNonMember: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, super::super::System::Variant::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    AddNonMember: usize,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub DeleteNonMember: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, super::super::System::Variant::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    DeleteNonMember: usize,
    pub Writable: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::BOOL) -> windows_core::HRESULT,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub GetProperty: unsafe extern "system" fn(*mut core::ffi::c_void, i32, super::super::System::Variant::VARIANT, *mut super::super::System::Variant::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    GetProperty: usize,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub SetProperty: unsafe extern "system" fn(*mut core::ffi::c_void, i32, super::super::System::Variant::VARIANT, super::super::System::Variant::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    SetProperty: usize,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub AddPropertyItem: unsafe extern "system" fn(*mut core::ffi::c_void, i32, super::super::System::Variant::VARIANT, super::super::System::Variant::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    AddPropertyItem: usize,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub DeletePropertyItem: unsafe extern "system" fn(*mut core::ffi::c_void, i32, super::super::System::Variant::VARIANT, super::super::System::Variant::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    DeletePropertyItem: usize,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub Submit: unsafe extern "system" fn(*mut core::ffi::c_void, i32, super::super::System::Variant::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    Submit: usize,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub AddMemberName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, super::super::System::Variant::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    AddMemberName: usize,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub DeleteMemberName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, super::super::System::Variant::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    DeleteMemberName: usize,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub AddNonMemberName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, super::super::System::Variant::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    AddNonMemberName: usize,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub DeleteNonMemberName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, super::super::System::Variant::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    DeleteNonMemberName: usize,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub MembersName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::System::Variant::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    MembersName: usize,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub NonMembersName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::System::Variant::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    NonMembersName: usize,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IAzApplicationGroup_Impl: super::super::System::Com::IDispatch_Impl {
    fn Name(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetName(&self, bstrname: &windows_core::BSTR) -> windows_core::Result<()>;
    fn Type(&self) -> windows_core::Result<i32>;
    fn SetType(&self, lprop: i32) -> windows_core::Result<()>;
    fn LdapQuery(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetLdapQuery(&self, bstrprop: &windows_core::BSTR) -> windows_core::Result<()>;
    fn AppMembers(&self) -> windows_core::Result<super::super::System::Variant::VARIANT>;
    fn AppNonMembers(&self) -> windows_core::Result<super::super::System::Variant::VARIANT>;
    fn Members(&self) -> windows_core::Result<super::super::System::Variant::VARIANT>;
    fn NonMembers(&self) -> windows_core::Result<super::super::System::Variant::VARIANT>;
    fn Description(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetDescription(&self, bstrdescription: &windows_core::BSTR) -> windows_core::Result<()>;
    fn AddAppMember(&self, bstrprop: &windows_core::BSTR, varreserved: &super::super::System::Variant::VARIANT) -> windows_core::Result<()>;
    fn DeleteAppMember(&self, bstrprop: &windows_core::BSTR, varreserved: &super::super::System::Variant::VARIANT) -> windows_core::Result<()>;
    fn AddAppNonMember(&self, bstrprop: &windows_core::BSTR, varreserved: &super::super::System::Variant::VARIANT) -> windows_core::Result<()>;
    fn DeleteAppNonMember(&self, bstrprop: &windows_core::BSTR, varreserved: &super::super::System::Variant::VARIANT) -> windows_core::Result<()>;
    fn AddMember(&self, bstrprop: &windows_core::BSTR, varreserved: &super::super::System::Variant::VARIANT) -> windows_core::Result<()>;
    fn DeleteMember(&self, bstrprop: &windows_core::BSTR, varreserved: &super::super::System::Variant::VARIANT) -> windows_core::Result<()>;
    fn AddNonMember(&self, bstrprop: &windows_core::BSTR, varreserved: &super::super::System::Variant::VARIANT) -> windows_core::Result<()>;
    fn DeleteNonMember(&self, bstrprop: &windows_core::BSTR, varreserved: &super::super::System::Variant::VARIANT) -> windows_core::Result<()>;
    fn Writable(&self) -> windows_core::Result<windows_core::BOOL>;
    fn GetProperty(&self, lpropid: i32, varreserved: &super::super::System::Variant::VARIANT) -> windows_core::Result<super::super::System::Variant::VARIANT>;
    fn SetProperty(&self, lpropid: i32, varprop: &super::super::System::Variant::VARIANT, varreserved: &super::super::System::Variant::VARIANT) -> windows_core::Result<()>;
    fn AddPropertyItem(&self, lpropid: i32, varprop: &super::super::System::Variant::VARIANT, varreserved: &super::super::System::Variant::VARIANT) -> windows_core::Result<()>;
    fn DeletePropertyItem(&self, lpropid: i32, varprop: &super::super::System::Variant::VARIANT, varreserved: &super::super::System::Variant::VARIANT) -> windows_core::Result<()>;
    fn Submit(&self, lflags: i32, varreserved: &super::super::System::Variant::VARIANT) -> windows_core::Result<()>;
    fn AddMemberName(&self, bstrprop: &windows_core::BSTR, varreserved: &super::super::System::Variant::VARIANT) -> windows_core::Result<()>;
    fn DeleteMemberName(&self, bstrprop: &windows_core::BSTR, varreserved: &super::super::System::Variant::VARIANT) -> windows_core::Result<()>;
    fn AddNonMemberName(&self, bstrprop: &windows_core::BSTR, varreserved: &super::super::System::Variant::VARIANT) -> windows_core::Result<()>;
    fn DeleteNonMemberName(&self, bstrprop: &windows_core::BSTR, varreserved: &super::super::System::Variant::VARIANT) -> windows_core::Result<()>;
    fn MembersName(&self) -> windows_core::Result<super::super::System::Variant::VARIANT>;
    fn NonMembersName(&self) -> windows_core::Result<super::super::System::Variant::VARIANT>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IAzApplicationGroup_Vtbl {
    pub const fn new<Identity: IAzApplicationGroup_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Name<Identity: IAzApplicationGroup_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrname: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IAzApplicationGroup_Impl::Name(this) {
                    Ok(ok__) => {
                        pbstrname.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetName<Identity: IAzApplicationGroup_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrname: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IAzApplicationGroup_Impl::SetName(this, core::mem::transmute(&bstrname)).into()
            }
        }
        unsafe extern "system" fn Type<Identity: IAzApplicationGroup_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, plprop: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IAzApplicationGroup_Impl::Type(this) {
                    Ok(ok__) => {
                        plprop.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetType<Identity: IAzApplicationGroup_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lprop: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IAzApplicationGroup_Impl::SetType(this, core::mem::transmute_copy(&lprop)).into()
            }
        }
        unsafe extern "system" fn LdapQuery<Identity: IAzApplicationGroup_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrprop: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IAzApplicationGroup_Impl::LdapQuery(this) {
                    Ok(ok__) => {
                        pbstrprop.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetLdapQuery<Identity: IAzApplicationGroup_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrprop: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IAzApplicationGroup_Impl::SetLdapQuery(this, core::mem::transmute(&bstrprop)).into()
            }
        }
        unsafe extern "system" fn AppMembers<Identity: IAzApplicationGroup_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvarprop: *mut super::super::System::Variant::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IAzApplicationGroup_Impl::AppMembers(this) {
                    Ok(ok__) => {
                        pvarprop.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn AppNonMembers<Identity: IAzApplicationGroup_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvarprop: *mut super::super::System::Variant::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IAzApplicationGroup_Impl::AppNonMembers(this) {
                    Ok(ok__) => {
                        pvarprop.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Members<Identity: IAzApplicationGroup_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvarprop: *mut super::super::System::Variant::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IAzApplicationGroup_Impl::Members(this) {
                    Ok(ok__) => {
                        pvarprop.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn NonMembers<Identity: IAzApplicationGroup_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvarprop: *mut super::super::System::Variant::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IAzApplicationGroup_Impl::NonMembers(this) {
                    Ok(ok__) => {
                        pvarprop.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Description<Identity: IAzApplicationGroup_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrdescription: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IAzApplicationGroup_Impl::Description(this) {
                    Ok(ok__) => {
                        pbstrdescription.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetDescription<Identity: IAzApplicationGroup_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrdescription: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IAzApplicationGroup_Impl::SetDescription(this, core::mem::transmute(&bstrdescription)).into()
            }
        }
        unsafe extern "system" fn AddAppMember<Identity: IAzApplicationGroup_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrprop: *mut core::ffi::c_void, varreserved: super::super::System::Variant::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IAzApplicationGroup_Impl::AddAppMember(this, core::mem::transmute(&bstrprop), core::mem::transmute(&varreserved)).into()
            }
        }
        unsafe extern "system" fn DeleteAppMember<Identity: IAzApplicationGroup_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrprop: *mut core::ffi::c_void, varreserved: super::super::System::Variant::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IAzApplicationGroup_Impl::DeleteAppMember(this, core::mem::transmute(&bstrprop), core::mem::transmute(&varreserved)).into()
            }
        }
        unsafe extern "system" fn AddAppNonMember<Identity: IAzApplicationGroup_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrprop: *mut core::ffi::c_void, varreserved: super::super::System::Variant::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IAzApplicationGroup_Impl::AddAppNonMember(this, core::mem::transmute(&bstrprop), core::mem::transmute(&varreserved)).into()
            }
        }
        unsafe extern "system" fn DeleteAppNonMember<Identity: IAzApplicationGroup_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrprop: *mut core::ffi::c_void, varreserved: super::super::System::Variant::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IAzApplicationGroup_Impl::DeleteAppNonMember(this, core::mem::transmute(&bstrprop), core::mem::transmute(&varreserved)).into()
            }
        }
        unsafe extern "system" fn AddMember<Identity: IAzApplicationGroup_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrprop: *mut core::ffi::c_void, varreserved: super::super::System::Variant::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IAzApplicationGroup_Impl::AddMember(this, core::mem::transmute(&bstrprop), core::mem::transmute(&varreserved)).into()
            }
        }
        unsafe extern "system" fn DeleteMember<Identity: IAzApplicationGroup_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrprop: *mut core::ffi::c_void, varreserved: super::super::System::Variant::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IAzApplicationGroup_Impl::DeleteMember(this, core::mem::transmute(&bstrprop), core::mem::transmute(&varreserved)).into()
            }
        }
        unsafe extern "system" fn AddNonMember<Identity: IAzApplicationGroup_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrprop: *mut core::ffi::c_void, varreserved: super::super::System::Variant::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IAzApplicationGroup_Impl::AddNonMember(this, core::mem::transmute(&bstrprop), core::mem::transmute(&varreserved)).into()
            }
        }
        unsafe extern "system" fn DeleteNonMember<Identity: IAzApplicationGroup_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrprop: *mut core::ffi::c_void, varreserved: super::super::System::Variant::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IAzApplicationGroup_Impl::DeleteNonMember(this, core::mem::transmute(&bstrprop), core::mem::transmute(&varreserved)).into()
            }
        }
        unsafe extern "system" fn Writable<Identity: IAzApplicationGroup_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pfprop: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IAzApplicationGroup_Impl::Writable(this) {
                    Ok(ok__) => {
                        pfprop.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetProperty<Identity: IAzApplicationGroup_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lpropid: i32, varreserved: super::super::System::Variant::VARIANT, pvarprop: *mut super::super::System::Variant::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IAzApplicationGroup_Impl::GetProperty(this, core::mem::transmute_copy(&lpropid), core::mem::transmute(&varreserved)) {
                    Ok(ok__) => {
                        pvarprop.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetProperty<Identity: IAzApplicationGroup_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lpropid: i32, varprop: super::super::System::Variant::VARIANT, varreserved: super::super::System::Variant::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IAzApplicationGroup_Impl::SetProperty(this, core::mem::transmute_copy(&lpropid), core::mem::transmute(&varprop), core::mem::transmute(&varreserved)).into()
            }
        }
        unsafe extern "system" fn AddPropertyItem<Identity: IAzApplicationGroup_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lpropid: i32, varprop: super::super::System::Variant::VARIANT, varreserved: super::super::System::Variant::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IAzApplicationGroup_Impl::AddPropertyItem(this, core::mem::transmute_copy(&lpropid), core::mem::transmute(&varprop), core::mem::transmute(&varreserved)).into()
            }
        }
        unsafe extern "system" fn DeletePropertyItem<Identity: IAzApplicationGroup_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lpropid: i32, varprop: super::super::System::Variant::VARIANT, varreserved: super::super::System::Variant::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IAzApplicationGroup_Impl::DeletePropertyItem(this, core::mem::transmute_copy(&lpropid), core::mem::transmute(&varprop), core::mem::transmute(&varreserved)).into()
            }
        }
        unsafe extern "system" fn Submit<Identity: IAzApplicationGroup_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lflags: i32, varreserved: super::super::System::Variant::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IAzApplicationGroup_Impl::Submit(this, core::mem::transmute_copy(&lflags), core::mem::transmute(&varreserved)).into()
            }
        }
        unsafe extern "system" fn AddMemberName<Identity: IAzApplicationGroup_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrprop: *mut core::ffi::c_void, varreserved: super::super::System::Variant::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IAzApplicationGroup_Impl::AddMemberName(this, core::mem::transmute(&bstrprop), core::mem::transmute(&varreserved)).into()
            }
        }
        unsafe extern "system" fn DeleteMemberName<Identity: IAzApplicationGroup_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrprop: *mut core::ffi::c_void, varreserved: super::super::System::Variant::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IAzApplicationGroup_Impl::DeleteMemberName(this, core::mem::transmute(&bstrprop), core::mem::transmute(&varreserved)).into()
            }
        }
        unsafe extern "system" fn AddNonMemberName<Identity: IAzApplicationGroup_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrprop: *mut core::ffi::c_void, varreserved: super::super::System::Variant::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IAzApplicationGroup_Impl::AddNonMemberName(this, core::mem::transmute(&bstrprop), core::mem::transmute(&varreserved)).into()
            }
        }
        unsafe extern "system" fn DeleteNonMemberName<Identity: IAzApplicationGroup_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrprop: *mut core::ffi::c_void, varreserved: super::super::System::Variant::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IAzApplicationGroup_Impl::DeleteNonMemberName(this, core::mem::transmute(&bstrprop), core::mem::transmute(&varreserved)).into()
            }
        }
        unsafe extern "system" fn MembersName<Identity: IAzApplicationGroup_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvarprop: *mut super::super::System::Variant::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IAzApplicationGroup_Impl::MembersName(this) {
                    Ok(ok__) => {
                        pvarprop.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn NonMembersName<Identity: IAzApplicationGroup_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvarprop: *mut super::super::System::Variant::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IAzApplicationGroup_Impl::NonMembersName(this) {
                    Ok(ok__) => {
                        pvarprop.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            Name: Name::<Identity, OFFSET>,
            SetName: SetName::<Identity, OFFSET>,
            Type: Type::<Identity, OFFSET>,
            SetType: SetType::<Identity, OFFSET>,
            LdapQuery: LdapQuery::<Identity, OFFSET>,
            SetLdapQuery: SetLdapQuery::<Identity, OFFSET>,
            AppMembers: AppMembers::<Identity, OFFSET>,
            AppNonMembers: AppNonMembers::<Identity, OFFSET>,
            Members: Members::<Identity, OFFSET>,
            NonMembers: NonMembers::<Identity, OFFSET>,
            Description: Description::<Identity, OFFSET>,
            SetDescription: SetDescription::<Identity, OFFSET>,
            AddAppMember: AddAppMember::<Identity, OFFSET>,
            DeleteAppMember: DeleteAppMember::<Identity, OFFSET>,
            AddAppNonMember: AddAppNonMember::<Identity, OFFSET>,
            DeleteAppNonMember: DeleteAppNonMember::<Identity, OFFSET>,
            AddMember: AddMember::<Identity, OFFSET>,
            DeleteMember: DeleteMember::<Identity, OFFSET>,
            AddNonMember: AddNonMember::<Identity, OFFSET>,
            DeleteNonMember: DeleteNonMember::<Identity, OFFSET>,
            Writable: Writable::<Identity, OFFSET>,
            GetProperty: GetProperty::<Identity, OFFSET>,
            SetProperty: SetProperty::<Identity, OFFSET>,
            AddPropertyItem: AddPropertyItem::<Identity, OFFSET>,
            DeletePropertyItem: DeletePropertyItem::<Identity, OFFSET>,
            Submit: Submit::<Identity, OFFSET>,
            AddMemberName: AddMemberName::<Identity, OFFSET>,
            DeleteMemberName: DeleteMemberName::<Identity, OFFSET>,
            AddNonMemberName: AddNonMemberName::<Identity, OFFSET>,
            DeleteNonMemberName: DeleteNonMemberName::<Identity, OFFSET>,
            MembersName: MembersName::<Identity, OFFSET>,
            NonMembersName: NonMembersName::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IAzApplicationGroup as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for IAzApplicationGroup {}
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
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).BizRule)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn SetBizRule(&self, bstrprop: &windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetBizRule)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrprop)).ok() }
    }
    pub unsafe fn BizRuleLanguage(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).BizRuleLanguage)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn SetBizRuleLanguage(&self, bstrprop: &windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetBizRuleLanguage)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrprop)).ok() }
    }
    pub unsafe fn BizRuleImportedPath(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).BizRuleImportedPath)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn SetBizRuleImportedPath(&self, bstrprop: &windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetBizRuleImportedPath)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrprop)).ok() }
    }
    pub unsafe fn RoleAssignments(&self, bstrscopename: &windows_core::BSTR, brecursive: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<IAzRoleAssignments> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).RoleAssignments)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrscopename), brecursive, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IAzApplicationGroup2_Vtbl {
    pub base__: IAzApplicationGroup_Vtbl,
    pub BizRule: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetBizRule: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub BizRuleLanguage: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetBizRuleLanguage: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub BizRuleImportedPath: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetBizRuleImportedPath: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub RoleAssignments: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, super::super::Foundation::VARIANT_BOOL, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IAzApplicationGroup2_Impl: IAzApplicationGroup_Impl {
    fn BizRule(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetBizRule(&self, bstrprop: &windows_core::BSTR) -> windows_core::Result<()>;
    fn BizRuleLanguage(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetBizRuleLanguage(&self, bstrprop: &windows_core::BSTR) -> windows_core::Result<()>;
    fn BizRuleImportedPath(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetBizRuleImportedPath(&self, bstrprop: &windows_core::BSTR) -> windows_core::Result<()>;
    fn RoleAssignments(&self, bstrscopename: &windows_core::BSTR, brecursive: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<IAzRoleAssignments>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IAzApplicationGroup2_Vtbl {
    pub const fn new<Identity: IAzApplicationGroup2_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn BizRule<Identity: IAzApplicationGroup2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrprop: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IAzApplicationGroup2_Impl::BizRule(this) {
                    Ok(ok__) => {
                        pbstrprop.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetBizRule<Identity: IAzApplicationGroup2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrprop: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IAzApplicationGroup2_Impl::SetBizRule(this, core::mem::transmute(&bstrprop)).into()
            }
        }
        unsafe extern "system" fn BizRuleLanguage<Identity: IAzApplicationGroup2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrprop: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IAzApplicationGroup2_Impl::BizRuleLanguage(this) {
                    Ok(ok__) => {
                        pbstrprop.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetBizRuleLanguage<Identity: IAzApplicationGroup2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrprop: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IAzApplicationGroup2_Impl::SetBizRuleLanguage(this, core::mem::transmute(&bstrprop)).into()
            }
        }
        unsafe extern "system" fn BizRuleImportedPath<Identity: IAzApplicationGroup2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrprop: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IAzApplicationGroup2_Impl::BizRuleImportedPath(this) {
                    Ok(ok__) => {
                        pbstrprop.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetBizRuleImportedPath<Identity: IAzApplicationGroup2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrprop: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IAzApplicationGroup2_Impl::SetBizRuleImportedPath(this, core::mem::transmute(&bstrprop)).into()
            }
        }
        unsafe extern "system" fn RoleAssignments<Identity: IAzApplicationGroup2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrscopename: *mut core::ffi::c_void, brecursive: super::super::Foundation::VARIANT_BOOL, pproleassignments: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IAzApplicationGroup2_Impl::RoleAssignments(this, core::mem::transmute(&bstrscopename), core::mem::transmute_copy(&brecursive)) {
                    Ok(ok__) => {
                        pproleassignments.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: IAzApplicationGroup_Vtbl::new::<Identity, OFFSET>(),
            BizRule: BizRule::<Identity, OFFSET>,
            SetBizRule: SetBizRule::<Identity, OFFSET>,
            BizRuleLanguage: BizRuleLanguage::<Identity, OFFSET>,
            SetBizRuleLanguage: SetBizRuleLanguage::<Identity, OFFSET>,
            BizRuleImportedPath: BizRuleImportedPath::<Identity, OFFSET>,
            SetBizRuleImportedPath: SetBizRuleImportedPath::<Identity, OFFSET>,
            RoleAssignments: RoleAssignments::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IAzApplicationGroup2 as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID || iid == &<IAzApplicationGroup as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for IAzApplicationGroup2 {}
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
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn get_Item(&self, index: i32) -> windows_core::Result<super::super::System::Variant::VARIANT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_Item)(windows_core::Interface::as_raw(self), index, &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn Count(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Count)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn _NewEnum(&self) -> windows_core::Result<windows_core::IUnknown> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self)._NewEnum)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IAzApplicationGroups_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub get_Item: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut super::super::System::Variant::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    get_Item: usize,
    pub Count: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub _NewEnum: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IAzApplicationGroups_Impl: super::super::System::Com::IDispatch_Impl {
    fn get_Item(&self, index: i32) -> windows_core::Result<super::super::System::Variant::VARIANT>;
    fn Count(&self) -> windows_core::Result<i32>;
    fn _NewEnum(&self) -> windows_core::Result<windows_core::IUnknown>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IAzApplicationGroups_Vtbl {
    pub const fn new<Identity: IAzApplicationGroups_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn get_Item<Identity: IAzApplicationGroups_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: i32, pvarobtptr: *mut super::super::System::Variant::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IAzApplicationGroups_Impl::get_Item(this, core::mem::transmute_copy(&index)) {
                    Ok(ok__) => {
                        pvarobtptr.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Count<Identity: IAzApplicationGroups_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, plcount: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IAzApplicationGroups_Impl::Count(this) {
                    Ok(ok__) => {
                        plcount.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn _NewEnum<Identity: IAzApplicationGroups_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppenumptr: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IAzApplicationGroups_Impl::_NewEnum(this) {
                    Ok(ok__) => {
                        ppenumptr.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            get_Item: get_Item::<Identity, OFFSET>,
            Count: Count::<Identity, OFFSET>,
            _NewEnum: _NewEnum::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IAzApplicationGroups as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for IAzApplicationGroups {}
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
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn get_Item(&self, index: i32) -> windows_core::Result<super::super::System::Variant::VARIANT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_Item)(windows_core::Interface::as_raw(self), index, &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn Count(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Count)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn _NewEnum(&self) -> windows_core::Result<windows_core::IUnknown> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self)._NewEnum)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IAzApplications_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub get_Item: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut super::super::System::Variant::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    get_Item: usize,
    pub Count: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub _NewEnum: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IAzApplications_Impl: super::super::System::Com::IDispatch_Impl {
    fn get_Item(&self, index: i32) -> windows_core::Result<super::super::System::Variant::VARIANT>;
    fn Count(&self) -> windows_core::Result<i32>;
    fn _NewEnum(&self) -> windows_core::Result<windows_core::IUnknown>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IAzApplications_Vtbl {
    pub const fn new<Identity: IAzApplications_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn get_Item<Identity: IAzApplications_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: i32, pvarobtptr: *mut super::super::System::Variant::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IAzApplications_Impl::get_Item(this, core::mem::transmute_copy(&index)) {
                    Ok(ok__) => {
                        pvarobtptr.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Count<Identity: IAzApplications_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, plcount: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IAzApplications_Impl::Count(this) {
                    Ok(ok__) => {
                        plcount.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn _NewEnum<Identity: IAzApplications_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppenumptr: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IAzApplications_Impl::_NewEnum(this) {
                    Ok(ok__) => {
                        ppenumptr.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            get_Item: get_Item::<Identity, OFFSET>,
            Count: Count::<Identity, OFFSET>,
            _NewEnum: _NewEnum::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IAzApplications as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for IAzApplications {}
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
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Description)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn SetDescription(&self, bstrdescription: &windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetDescription)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrdescription)).ok() }
    }
    pub unsafe fn ApplicationData(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ApplicationData)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn SetApplicationData(&self, bstrapplicationdata: &windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetApplicationData)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrapplicationdata)).ok() }
    }
    pub unsafe fn DomainTimeout(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).DomainTimeout)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetDomainTimeout(&self, lprop: i32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetDomainTimeout)(windows_core::Interface::as_raw(self), lprop).ok() }
    }
    pub unsafe fn ScriptEngineTimeout(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ScriptEngineTimeout)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetScriptEngineTimeout(&self, lprop: i32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetScriptEngineTimeout)(windows_core::Interface::as_raw(self), lprop).ok() }
    }
    pub unsafe fn MaxScriptEngines(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).MaxScriptEngines)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetMaxScriptEngines(&self, lprop: i32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetMaxScriptEngines)(windows_core::Interface::as_raw(self), lprop).ok() }
    }
    pub unsafe fn GenerateAudits(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GenerateAudits)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetGenerateAudits(&self, bprop: bool) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetGenerateAudits)(windows_core::Interface::as_raw(self), bprop.into()).ok() }
    }
    pub unsafe fn Writable(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Writable)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn GetProperty(&self, lpropid: i32, varreserved: &super::super::System::Variant::VARIANT) -> windows_core::Result<super::super::System::Variant::VARIANT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetProperty)(windows_core::Interface::as_raw(self), lpropid, core::mem::transmute_copy(varreserved), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn SetProperty(&self, lpropid: i32, varprop: &super::super::System::Variant::VARIANT, varreserved: &super::super::System::Variant::VARIANT) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetProperty)(windows_core::Interface::as_raw(self), lpropid, core::mem::transmute_copy(varprop), core::mem::transmute_copy(varreserved)).ok() }
    }
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn AddPropertyItem(&self, lpropid: AZ_PROP_CONSTANTS, varprop: &super::super::System::Variant::VARIANT, varreserved: &super::super::System::Variant::VARIANT) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).AddPropertyItem)(windows_core::Interface::as_raw(self), lpropid, core::mem::transmute_copy(varprop), core::mem::transmute_copy(varreserved)).ok() }
    }
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn DeletePropertyItem(&self, lpropid: i32, varprop: &super::super::System::Variant::VARIANT, varreserved: &super::super::System::Variant::VARIANT) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).DeletePropertyItem)(windows_core::Interface::as_raw(self), lpropid, core::mem::transmute_copy(varprop), core::mem::transmute_copy(varreserved)).ok() }
    }
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn PolicyAdministrators(&self) -> windows_core::Result<super::super::System::Variant::VARIANT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).PolicyAdministrators)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn PolicyReaders(&self) -> windows_core::Result<super::super::System::Variant::VARIANT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).PolicyReaders)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn AddPolicyAdministrator(&self, bstradmin: &windows_core::BSTR, varreserved: &super::super::System::Variant::VARIANT) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).AddPolicyAdministrator)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstradmin), core::mem::transmute_copy(varreserved)).ok() }
    }
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn DeletePolicyAdministrator(&self, bstradmin: &windows_core::BSTR, varreserved: &super::super::System::Variant::VARIANT) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).DeletePolicyAdministrator)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstradmin), core::mem::transmute_copy(varreserved)).ok() }
    }
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn AddPolicyReader(&self, bstrreader: &windows_core::BSTR, varreserved: &super::super::System::Variant::VARIANT) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).AddPolicyReader)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrreader), core::mem::transmute_copy(varreserved)).ok() }
    }
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn DeletePolicyReader(&self, bstrreader: &windows_core::BSTR, varreserved: &super::super::System::Variant::VARIANT) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).DeletePolicyReader)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrreader), core::mem::transmute_copy(varreserved)).ok() }
    }
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn Initialize(&self, lflags: AZ_PROP_CONSTANTS, bstrpolicyurl: &windows_core::BSTR, varreserved: &super::super::System::Variant::VARIANT) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Initialize)(windows_core::Interface::as_raw(self), lflags, core::mem::transmute_copy(bstrpolicyurl), core::mem::transmute_copy(varreserved)).ok() }
    }
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn UpdateCache(&self, varreserved: &super::super::System::Variant::VARIANT) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).UpdateCache)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(varreserved)).ok() }
    }
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn Delete(&self, varreserved: &super::super::System::Variant::VARIANT) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Delete)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(varreserved)).ok() }
    }
    pub unsafe fn Applications(&self) -> windows_core::Result<IAzApplications> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Applications)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn OpenApplication(&self, bstrapplicationname: &windows_core::BSTR, varreserved: &super::super::System::Variant::VARIANT) -> windows_core::Result<IAzApplication> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).OpenApplication)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrapplicationname), core::mem::transmute_copy(varreserved), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn CreateApplication(&self, bstrapplicationname: &windows_core::BSTR, varreserved: &super::super::System::Variant::VARIANT) -> windows_core::Result<IAzApplication> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateApplication)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrapplicationname), core::mem::transmute_copy(varreserved), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn DeleteApplication(&self, bstrapplicationname: &windows_core::BSTR, varreserved: &super::super::System::Variant::VARIANT) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).DeleteApplication)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrapplicationname), core::mem::transmute_copy(varreserved)).ok() }
    }
    pub unsafe fn ApplicationGroups(&self) -> windows_core::Result<IAzApplicationGroups> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ApplicationGroups)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn CreateApplicationGroup(&self, bstrgroupname: &windows_core::BSTR, varreserved: &super::super::System::Variant::VARIANT) -> windows_core::Result<IAzApplicationGroup> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateApplicationGroup)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrgroupname), core::mem::transmute_copy(varreserved), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn OpenApplicationGroup(&self, bstrgroupname: &windows_core::BSTR, varreserved: &super::super::System::Variant::VARIANT) -> windows_core::Result<IAzApplicationGroup> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).OpenApplicationGroup)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrgroupname), core::mem::transmute_copy(varreserved), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn DeleteApplicationGroup(&self, bstrgroupname: &windows_core::BSTR, varreserved: &super::super::System::Variant::VARIANT) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).DeleteApplicationGroup)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrgroupname), core::mem::transmute_copy(varreserved)).ok() }
    }
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn Submit(&self, lflags: i32, varreserved: &super::super::System::Variant::VARIANT) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Submit)(windows_core::Interface::as_raw(self), lflags, core::mem::transmute_copy(varreserved)).ok() }
    }
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn DelegatedPolicyUsers(&self) -> windows_core::Result<super::super::System::Variant::VARIANT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).DelegatedPolicyUsers)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn AddDelegatedPolicyUser(&self, bstrdelegatedpolicyuser: &windows_core::BSTR, varreserved: &super::super::System::Variant::VARIANT) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).AddDelegatedPolicyUser)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrdelegatedpolicyuser), core::mem::transmute_copy(varreserved)).ok() }
    }
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn DeleteDelegatedPolicyUser(&self, bstrdelegatedpolicyuser: &windows_core::BSTR, varreserved: &super::super::System::Variant::VARIANT) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).DeleteDelegatedPolicyUser)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrdelegatedpolicyuser), core::mem::transmute_copy(varreserved)).ok() }
    }
    pub unsafe fn TargetMachine(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).TargetMachine)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn ApplyStoreSacl(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ApplyStoreSacl)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetApplyStoreSacl(&self, bapplystoresacl: bool) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetApplyStoreSacl)(windows_core::Interface::as_raw(self), bapplystoresacl.into()).ok() }
    }
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn PolicyAdministratorsName(&self) -> windows_core::Result<super::super::System::Variant::VARIANT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).PolicyAdministratorsName)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn PolicyReadersName(&self) -> windows_core::Result<super::super::System::Variant::VARIANT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).PolicyReadersName)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn AddPolicyAdministratorName(&self, bstradmin: &windows_core::BSTR, varreserved: &super::super::System::Variant::VARIANT) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).AddPolicyAdministratorName)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstradmin), core::mem::transmute_copy(varreserved)).ok() }
    }
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn DeletePolicyAdministratorName(&self, bstradmin: &windows_core::BSTR, varreserved: &super::super::System::Variant::VARIANT) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).DeletePolicyAdministratorName)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstradmin), core::mem::transmute_copy(varreserved)).ok() }
    }
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn AddPolicyReaderName(&self, bstrreader: &windows_core::BSTR, varreserved: &super::super::System::Variant::VARIANT) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).AddPolicyReaderName)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrreader), core::mem::transmute_copy(varreserved)).ok() }
    }
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn DeletePolicyReaderName(&self, bstrreader: &windows_core::BSTR, varreserved: &super::super::System::Variant::VARIANT) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).DeletePolicyReaderName)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrreader), core::mem::transmute_copy(varreserved)).ok() }
    }
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn DelegatedPolicyUsersName(&self) -> windows_core::Result<super::super::System::Variant::VARIANT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).DelegatedPolicyUsersName)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn AddDelegatedPolicyUserName(&self, bstrdelegatedpolicyuser: &windows_core::BSTR, varreserved: &super::super::System::Variant::VARIANT) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).AddDelegatedPolicyUserName)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrdelegatedpolicyuser), core::mem::transmute_copy(varreserved)).ok() }
    }
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn DeleteDelegatedPolicyUserName(&self, bstrdelegatedpolicyuser: &windows_core::BSTR, varreserved: &super::super::System::Variant::VARIANT) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).DeleteDelegatedPolicyUserName)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrdelegatedpolicyuser), core::mem::transmute_copy(varreserved)).ok() }
    }
    pub unsafe fn CloseApplication(&self, bstrapplicationname: &windows_core::BSTR, lflag: i32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).CloseApplication)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrapplicationname), lflag).ok() }
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IAzAuthorizationStore_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub Description: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetDescription: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ApplicationData: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetApplicationData: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub DomainTimeout: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetDomainTimeout: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub ScriptEngineTimeout: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetScriptEngineTimeout: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub MaxScriptEngines: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetMaxScriptEngines: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub GenerateAudits: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::BOOL) -> windows_core::HRESULT,
    pub SetGenerateAudits: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::BOOL) -> windows_core::HRESULT,
    pub Writable: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::BOOL) -> windows_core::HRESULT,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub GetProperty: unsafe extern "system" fn(*mut core::ffi::c_void, i32, super::super::System::Variant::VARIANT, *mut super::super::System::Variant::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    GetProperty: usize,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub SetProperty: unsafe extern "system" fn(*mut core::ffi::c_void, i32, super::super::System::Variant::VARIANT, super::super::System::Variant::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    SetProperty: usize,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub AddPropertyItem: unsafe extern "system" fn(*mut core::ffi::c_void, AZ_PROP_CONSTANTS, super::super::System::Variant::VARIANT, super::super::System::Variant::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    AddPropertyItem: usize,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub DeletePropertyItem: unsafe extern "system" fn(*mut core::ffi::c_void, i32, super::super::System::Variant::VARIANT, super::super::System::Variant::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    DeletePropertyItem: usize,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub PolicyAdministrators: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::System::Variant::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    PolicyAdministrators: usize,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub PolicyReaders: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::System::Variant::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    PolicyReaders: usize,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub AddPolicyAdministrator: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, super::super::System::Variant::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    AddPolicyAdministrator: usize,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub DeletePolicyAdministrator: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, super::super::System::Variant::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    DeletePolicyAdministrator: usize,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub AddPolicyReader: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, super::super::System::Variant::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    AddPolicyReader: usize,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub DeletePolicyReader: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, super::super::System::Variant::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    DeletePolicyReader: usize,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub Initialize: unsafe extern "system" fn(*mut core::ffi::c_void, AZ_PROP_CONSTANTS, *mut core::ffi::c_void, super::super::System::Variant::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    Initialize: usize,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub UpdateCache: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::System::Variant::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    UpdateCache: usize,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub Delete: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::System::Variant::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    Delete: usize,
    pub Applications: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub OpenApplication: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, super::super::System::Variant::VARIANT, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    OpenApplication: usize,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub CreateApplication: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, super::super::System::Variant::VARIANT, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    CreateApplication: usize,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub DeleteApplication: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, super::super::System::Variant::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    DeleteApplication: usize,
    pub ApplicationGroups: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub CreateApplicationGroup: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, super::super::System::Variant::VARIANT, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    CreateApplicationGroup: usize,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub OpenApplicationGroup: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, super::super::System::Variant::VARIANT, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    OpenApplicationGroup: usize,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub DeleteApplicationGroup: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, super::super::System::Variant::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    DeleteApplicationGroup: usize,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub Submit: unsafe extern "system" fn(*mut core::ffi::c_void, i32, super::super::System::Variant::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    Submit: usize,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub DelegatedPolicyUsers: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::System::Variant::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    DelegatedPolicyUsers: usize,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub AddDelegatedPolicyUser: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, super::super::System::Variant::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    AddDelegatedPolicyUser: usize,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub DeleteDelegatedPolicyUser: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, super::super::System::Variant::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    DeleteDelegatedPolicyUser: usize,
    pub TargetMachine: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ApplyStoreSacl: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::BOOL) -> windows_core::HRESULT,
    pub SetApplyStoreSacl: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::BOOL) -> windows_core::HRESULT,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub PolicyAdministratorsName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::System::Variant::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    PolicyAdministratorsName: usize,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub PolicyReadersName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::System::Variant::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    PolicyReadersName: usize,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub AddPolicyAdministratorName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, super::super::System::Variant::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    AddPolicyAdministratorName: usize,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub DeletePolicyAdministratorName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, super::super::System::Variant::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    DeletePolicyAdministratorName: usize,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub AddPolicyReaderName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, super::super::System::Variant::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    AddPolicyReaderName: usize,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub DeletePolicyReaderName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, super::super::System::Variant::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    DeletePolicyReaderName: usize,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub DelegatedPolicyUsersName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::System::Variant::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    DelegatedPolicyUsersName: usize,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub AddDelegatedPolicyUserName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, super::super::System::Variant::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    AddDelegatedPolicyUserName: usize,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub DeleteDelegatedPolicyUserName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, super::super::System::Variant::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    DeleteDelegatedPolicyUserName: usize,
    pub CloseApplication: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, i32) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IAzAuthorizationStore_Impl: super::super::System::Com::IDispatch_Impl {
    fn Description(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetDescription(&self, bstrdescription: &windows_core::BSTR) -> windows_core::Result<()>;
    fn ApplicationData(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetApplicationData(&self, bstrapplicationdata: &windows_core::BSTR) -> windows_core::Result<()>;
    fn DomainTimeout(&self) -> windows_core::Result<i32>;
    fn SetDomainTimeout(&self, lprop: i32) -> windows_core::Result<()>;
    fn ScriptEngineTimeout(&self) -> windows_core::Result<i32>;
    fn SetScriptEngineTimeout(&self, lprop: i32) -> windows_core::Result<()>;
    fn MaxScriptEngines(&self) -> windows_core::Result<i32>;
    fn SetMaxScriptEngines(&self, lprop: i32) -> windows_core::Result<()>;
    fn GenerateAudits(&self) -> windows_core::Result<windows_core::BOOL>;
    fn SetGenerateAudits(&self, bprop: windows_core::BOOL) -> windows_core::Result<()>;
    fn Writable(&self) -> windows_core::Result<windows_core::BOOL>;
    fn GetProperty(&self, lpropid: i32, varreserved: &super::super::System::Variant::VARIANT) -> windows_core::Result<super::super::System::Variant::VARIANT>;
    fn SetProperty(&self, lpropid: i32, varprop: &super::super::System::Variant::VARIANT, varreserved: &super::super::System::Variant::VARIANT) -> windows_core::Result<()>;
    fn AddPropertyItem(&self, lpropid: AZ_PROP_CONSTANTS, varprop: &super::super::System::Variant::VARIANT, varreserved: &super::super::System::Variant::VARIANT) -> windows_core::Result<()>;
    fn DeletePropertyItem(&self, lpropid: i32, varprop: &super::super::System::Variant::VARIANT, varreserved: &super::super::System::Variant::VARIANT) -> windows_core::Result<()>;
    fn PolicyAdministrators(&self) -> windows_core::Result<super::super::System::Variant::VARIANT>;
    fn PolicyReaders(&self) -> windows_core::Result<super::super::System::Variant::VARIANT>;
    fn AddPolicyAdministrator(&self, bstradmin: &windows_core::BSTR, varreserved: &super::super::System::Variant::VARIANT) -> windows_core::Result<()>;
    fn DeletePolicyAdministrator(&self, bstradmin: &windows_core::BSTR, varreserved: &super::super::System::Variant::VARIANT) -> windows_core::Result<()>;
    fn AddPolicyReader(&self, bstrreader: &windows_core::BSTR, varreserved: &super::super::System::Variant::VARIANT) -> windows_core::Result<()>;
    fn DeletePolicyReader(&self, bstrreader: &windows_core::BSTR, varreserved: &super::super::System::Variant::VARIANT) -> windows_core::Result<()>;
    fn Initialize(&self, lflags: AZ_PROP_CONSTANTS, bstrpolicyurl: &windows_core::BSTR, varreserved: &super::super::System::Variant::VARIANT) -> windows_core::Result<()>;
    fn UpdateCache(&self, varreserved: &super::super::System::Variant::VARIANT) -> windows_core::Result<()>;
    fn Delete(&self, varreserved: &super::super::System::Variant::VARIANT) -> windows_core::Result<()>;
    fn Applications(&self) -> windows_core::Result<IAzApplications>;
    fn OpenApplication(&self, bstrapplicationname: &windows_core::BSTR, varreserved: &super::super::System::Variant::VARIANT) -> windows_core::Result<IAzApplication>;
    fn CreateApplication(&self, bstrapplicationname: &windows_core::BSTR, varreserved: &super::super::System::Variant::VARIANT) -> windows_core::Result<IAzApplication>;
    fn DeleteApplication(&self, bstrapplicationname: &windows_core::BSTR, varreserved: &super::super::System::Variant::VARIANT) -> windows_core::Result<()>;
    fn ApplicationGroups(&self) -> windows_core::Result<IAzApplicationGroups>;
    fn CreateApplicationGroup(&self, bstrgroupname: &windows_core::BSTR, varreserved: &super::super::System::Variant::VARIANT) -> windows_core::Result<IAzApplicationGroup>;
    fn OpenApplicationGroup(&self, bstrgroupname: &windows_core::BSTR, varreserved: &super::super::System::Variant::VARIANT) -> windows_core::Result<IAzApplicationGroup>;
    fn DeleteApplicationGroup(&self, bstrgroupname: &windows_core::BSTR, varreserved: &super::super::System::Variant::VARIANT) -> windows_core::Result<()>;
    fn Submit(&self, lflags: i32, varreserved: &super::super::System::Variant::VARIANT) -> windows_core::Result<()>;
    fn DelegatedPolicyUsers(&self) -> windows_core::Result<super::super::System::Variant::VARIANT>;
    fn AddDelegatedPolicyUser(&self, bstrdelegatedpolicyuser: &windows_core::BSTR, varreserved: &super::super::System::Variant::VARIANT) -> windows_core::Result<()>;
    fn DeleteDelegatedPolicyUser(&self, bstrdelegatedpolicyuser: &windows_core::BSTR, varreserved: &super::super::System::Variant::VARIANT) -> windows_core::Result<()>;
    fn TargetMachine(&self) -> windows_core::Result<windows_core::BSTR>;
    fn ApplyStoreSacl(&self) -> windows_core::Result<windows_core::BOOL>;
    fn SetApplyStoreSacl(&self, bapplystoresacl: windows_core::BOOL) -> windows_core::Result<()>;
    fn PolicyAdministratorsName(&self) -> windows_core::Result<super::super::System::Variant::VARIANT>;
    fn PolicyReadersName(&self) -> windows_core::Result<super::super::System::Variant::VARIANT>;
    fn AddPolicyAdministratorName(&self, bstradmin: &windows_core::BSTR, varreserved: &super::super::System::Variant::VARIANT) -> windows_core::Result<()>;
    fn DeletePolicyAdministratorName(&self, bstradmin: &windows_core::BSTR, varreserved: &super::super::System::Variant::VARIANT) -> windows_core::Result<()>;
    fn AddPolicyReaderName(&self, bstrreader: &windows_core::BSTR, varreserved: &super::super::System::Variant::VARIANT) -> windows_core::Result<()>;
    fn DeletePolicyReaderName(&self, bstrreader: &windows_core::BSTR, varreserved: &super::super::System::Variant::VARIANT) -> windows_core::Result<()>;
    fn DelegatedPolicyUsersName(&self) -> windows_core::Result<super::super::System::Variant::VARIANT>;
    fn AddDelegatedPolicyUserName(&self, bstrdelegatedpolicyuser: &windows_core::BSTR, varreserved: &super::super::System::Variant::VARIANT) -> windows_core::Result<()>;
    fn DeleteDelegatedPolicyUserName(&self, bstrdelegatedpolicyuser: &windows_core::BSTR, varreserved: &super::super::System::Variant::VARIANT) -> windows_core::Result<()>;
    fn CloseApplication(&self, bstrapplicationname: &windows_core::BSTR, lflag: i32) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IAzAuthorizationStore_Vtbl {
    pub const fn new<Identity: IAzAuthorizationStore_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Description<Identity: IAzAuthorizationStore_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrdescription: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IAzAuthorizationStore_Impl::Description(this) {
                    Ok(ok__) => {
                        pbstrdescription.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetDescription<Identity: IAzAuthorizationStore_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrdescription: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IAzAuthorizationStore_Impl::SetDescription(this, core::mem::transmute(&bstrdescription)).into()
            }
        }
        unsafe extern "system" fn ApplicationData<Identity: IAzAuthorizationStore_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrapplicationdata: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IAzAuthorizationStore_Impl::ApplicationData(this) {
                    Ok(ok__) => {
                        pbstrapplicationdata.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetApplicationData<Identity: IAzAuthorizationStore_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrapplicationdata: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IAzAuthorizationStore_Impl::SetApplicationData(this, core::mem::transmute(&bstrapplicationdata)).into()
            }
        }
        unsafe extern "system" fn DomainTimeout<Identity: IAzAuthorizationStore_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, plprop: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IAzAuthorizationStore_Impl::DomainTimeout(this) {
                    Ok(ok__) => {
                        plprop.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetDomainTimeout<Identity: IAzAuthorizationStore_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lprop: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IAzAuthorizationStore_Impl::SetDomainTimeout(this, core::mem::transmute_copy(&lprop)).into()
            }
        }
        unsafe extern "system" fn ScriptEngineTimeout<Identity: IAzAuthorizationStore_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, plprop: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IAzAuthorizationStore_Impl::ScriptEngineTimeout(this) {
                    Ok(ok__) => {
                        plprop.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetScriptEngineTimeout<Identity: IAzAuthorizationStore_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lprop: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IAzAuthorizationStore_Impl::SetScriptEngineTimeout(this, core::mem::transmute_copy(&lprop)).into()
            }
        }
        unsafe extern "system" fn MaxScriptEngines<Identity: IAzAuthorizationStore_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, plprop: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IAzAuthorizationStore_Impl::MaxScriptEngines(this) {
                    Ok(ok__) => {
                        plprop.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetMaxScriptEngines<Identity: IAzAuthorizationStore_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lprop: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IAzAuthorizationStore_Impl::SetMaxScriptEngines(this, core::mem::transmute_copy(&lprop)).into()
            }
        }
        unsafe extern "system" fn GenerateAudits<Identity: IAzAuthorizationStore_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbprop: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IAzAuthorizationStore_Impl::GenerateAudits(this) {
                    Ok(ok__) => {
                        pbprop.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetGenerateAudits<Identity: IAzAuthorizationStore_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bprop: windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IAzAuthorizationStore_Impl::SetGenerateAudits(this, core::mem::transmute_copy(&bprop)).into()
            }
        }
        unsafe extern "system" fn Writable<Identity: IAzAuthorizationStore_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pfprop: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IAzAuthorizationStore_Impl::Writable(this) {
                    Ok(ok__) => {
                        pfprop.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetProperty<Identity: IAzAuthorizationStore_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lpropid: i32, varreserved: super::super::System::Variant::VARIANT, pvarprop: *mut super::super::System::Variant::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IAzAuthorizationStore_Impl::GetProperty(this, core::mem::transmute_copy(&lpropid), core::mem::transmute(&varreserved)) {
                    Ok(ok__) => {
                        pvarprop.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetProperty<Identity: IAzAuthorizationStore_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lpropid: i32, varprop: super::super::System::Variant::VARIANT, varreserved: super::super::System::Variant::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IAzAuthorizationStore_Impl::SetProperty(this, core::mem::transmute_copy(&lpropid), core::mem::transmute(&varprop), core::mem::transmute(&varreserved)).into()
            }
        }
        unsafe extern "system" fn AddPropertyItem<Identity: IAzAuthorizationStore_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lpropid: AZ_PROP_CONSTANTS, varprop: super::super::System::Variant::VARIANT, varreserved: super::super::System::Variant::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IAzAuthorizationStore_Impl::AddPropertyItem(this, core::mem::transmute_copy(&lpropid), core::mem::transmute(&varprop), core::mem::transmute(&varreserved)).into()
            }
        }
        unsafe extern "system" fn DeletePropertyItem<Identity: IAzAuthorizationStore_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lpropid: i32, varprop: super::super::System::Variant::VARIANT, varreserved: super::super::System::Variant::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IAzAuthorizationStore_Impl::DeletePropertyItem(this, core::mem::transmute_copy(&lpropid), core::mem::transmute(&varprop), core::mem::transmute(&varreserved)).into()
            }
        }
        unsafe extern "system" fn PolicyAdministrators<Identity: IAzAuthorizationStore_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvaradmins: *mut super::super::System::Variant::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IAzAuthorizationStore_Impl::PolicyAdministrators(this) {
                    Ok(ok__) => {
                        pvaradmins.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn PolicyReaders<Identity: IAzAuthorizationStore_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvarreaders: *mut super::super::System::Variant::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IAzAuthorizationStore_Impl::PolicyReaders(this) {
                    Ok(ok__) => {
                        pvarreaders.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn AddPolicyAdministrator<Identity: IAzAuthorizationStore_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstradmin: *mut core::ffi::c_void, varreserved: super::super::System::Variant::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IAzAuthorizationStore_Impl::AddPolicyAdministrator(this, core::mem::transmute(&bstradmin), core::mem::transmute(&varreserved)).into()
            }
        }
        unsafe extern "system" fn DeletePolicyAdministrator<Identity: IAzAuthorizationStore_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstradmin: *mut core::ffi::c_void, varreserved: super::super::System::Variant::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IAzAuthorizationStore_Impl::DeletePolicyAdministrator(this, core::mem::transmute(&bstradmin), core::mem::transmute(&varreserved)).into()
            }
        }
        unsafe extern "system" fn AddPolicyReader<Identity: IAzAuthorizationStore_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrreader: *mut core::ffi::c_void, varreserved: super::super::System::Variant::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IAzAuthorizationStore_Impl::AddPolicyReader(this, core::mem::transmute(&bstrreader), core::mem::transmute(&varreserved)).into()
            }
        }
        unsafe extern "system" fn DeletePolicyReader<Identity: IAzAuthorizationStore_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrreader: *mut core::ffi::c_void, varreserved: super::super::System::Variant::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IAzAuthorizationStore_Impl::DeletePolicyReader(this, core::mem::transmute(&bstrreader), core::mem::transmute(&varreserved)).into()
            }
        }
        unsafe extern "system" fn Initialize<Identity: IAzAuthorizationStore_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lflags: AZ_PROP_CONSTANTS, bstrpolicyurl: *mut core::ffi::c_void, varreserved: super::super::System::Variant::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IAzAuthorizationStore_Impl::Initialize(this, core::mem::transmute_copy(&lflags), core::mem::transmute(&bstrpolicyurl), core::mem::transmute(&varreserved)).into()
            }
        }
        unsafe extern "system" fn UpdateCache<Identity: IAzAuthorizationStore_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, varreserved: super::super::System::Variant::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IAzAuthorizationStore_Impl::UpdateCache(this, core::mem::transmute(&varreserved)).into()
            }
        }
        unsafe extern "system" fn Delete<Identity: IAzAuthorizationStore_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, varreserved: super::super::System::Variant::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IAzAuthorizationStore_Impl::Delete(this, core::mem::transmute(&varreserved)).into()
            }
        }
        unsafe extern "system" fn Applications<Identity: IAzAuthorizationStore_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppappcollection: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IAzAuthorizationStore_Impl::Applications(this) {
                    Ok(ok__) => {
                        ppappcollection.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn OpenApplication<Identity: IAzAuthorizationStore_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrapplicationname: *mut core::ffi::c_void, varreserved: super::super::System::Variant::VARIANT, ppapplication: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IAzAuthorizationStore_Impl::OpenApplication(this, core::mem::transmute(&bstrapplicationname), core::mem::transmute(&varreserved)) {
                    Ok(ok__) => {
                        ppapplication.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CreateApplication<Identity: IAzAuthorizationStore_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrapplicationname: *mut core::ffi::c_void, varreserved: super::super::System::Variant::VARIANT, ppapplication: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IAzAuthorizationStore_Impl::CreateApplication(this, core::mem::transmute(&bstrapplicationname), core::mem::transmute(&varreserved)) {
                    Ok(ok__) => {
                        ppapplication.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn DeleteApplication<Identity: IAzAuthorizationStore_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrapplicationname: *mut core::ffi::c_void, varreserved: super::super::System::Variant::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IAzAuthorizationStore_Impl::DeleteApplication(this, core::mem::transmute(&bstrapplicationname), core::mem::transmute(&varreserved)).into()
            }
        }
        unsafe extern "system" fn ApplicationGroups<Identity: IAzAuthorizationStore_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppgroupcollection: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IAzAuthorizationStore_Impl::ApplicationGroups(this) {
                    Ok(ok__) => {
                        ppgroupcollection.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CreateApplicationGroup<Identity: IAzAuthorizationStore_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrgroupname: *mut core::ffi::c_void, varreserved: super::super::System::Variant::VARIANT, ppgroup: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IAzAuthorizationStore_Impl::CreateApplicationGroup(this, core::mem::transmute(&bstrgroupname), core::mem::transmute(&varreserved)) {
                    Ok(ok__) => {
                        ppgroup.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn OpenApplicationGroup<Identity: IAzAuthorizationStore_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrgroupname: *mut core::ffi::c_void, varreserved: super::super::System::Variant::VARIANT, ppgroup: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IAzAuthorizationStore_Impl::OpenApplicationGroup(this, core::mem::transmute(&bstrgroupname), core::mem::transmute(&varreserved)) {
                    Ok(ok__) => {
                        ppgroup.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn DeleteApplicationGroup<Identity: IAzAuthorizationStore_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrgroupname: *mut core::ffi::c_void, varreserved: super::super::System::Variant::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IAzAuthorizationStore_Impl::DeleteApplicationGroup(this, core::mem::transmute(&bstrgroupname), core::mem::transmute(&varreserved)).into()
            }
        }
        unsafe extern "system" fn Submit<Identity: IAzAuthorizationStore_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lflags: i32, varreserved: super::super::System::Variant::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IAzAuthorizationStore_Impl::Submit(this, core::mem::transmute_copy(&lflags), core::mem::transmute(&varreserved)).into()
            }
        }
        unsafe extern "system" fn DelegatedPolicyUsers<Identity: IAzAuthorizationStore_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvardelegatedpolicyusers: *mut super::super::System::Variant::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IAzAuthorizationStore_Impl::DelegatedPolicyUsers(this) {
                    Ok(ok__) => {
                        pvardelegatedpolicyusers.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn AddDelegatedPolicyUser<Identity: IAzAuthorizationStore_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrdelegatedpolicyuser: *mut core::ffi::c_void, varreserved: super::super::System::Variant::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IAzAuthorizationStore_Impl::AddDelegatedPolicyUser(this, core::mem::transmute(&bstrdelegatedpolicyuser), core::mem::transmute(&varreserved)).into()
            }
        }
        unsafe extern "system" fn DeleteDelegatedPolicyUser<Identity: IAzAuthorizationStore_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrdelegatedpolicyuser: *mut core::ffi::c_void, varreserved: super::super::System::Variant::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IAzAuthorizationStore_Impl::DeleteDelegatedPolicyUser(this, core::mem::transmute(&bstrdelegatedpolicyuser), core::mem::transmute(&varreserved)).into()
            }
        }
        unsafe extern "system" fn TargetMachine<Identity: IAzAuthorizationStore_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrtargetmachine: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IAzAuthorizationStore_Impl::TargetMachine(this) {
                    Ok(ok__) => {
                        pbstrtargetmachine.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn ApplyStoreSacl<Identity: IAzAuthorizationStore_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbapplystoresacl: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IAzAuthorizationStore_Impl::ApplyStoreSacl(this) {
                    Ok(ok__) => {
                        pbapplystoresacl.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetApplyStoreSacl<Identity: IAzAuthorizationStore_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bapplystoresacl: windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IAzAuthorizationStore_Impl::SetApplyStoreSacl(this, core::mem::transmute_copy(&bapplystoresacl)).into()
            }
        }
        unsafe extern "system" fn PolicyAdministratorsName<Identity: IAzAuthorizationStore_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvaradmins: *mut super::super::System::Variant::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IAzAuthorizationStore_Impl::PolicyAdministratorsName(this) {
                    Ok(ok__) => {
                        pvaradmins.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn PolicyReadersName<Identity: IAzAuthorizationStore_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvarreaders: *mut super::super::System::Variant::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IAzAuthorizationStore_Impl::PolicyReadersName(this) {
                    Ok(ok__) => {
                        pvarreaders.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn AddPolicyAdministratorName<Identity: IAzAuthorizationStore_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstradmin: *mut core::ffi::c_void, varreserved: super::super::System::Variant::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IAzAuthorizationStore_Impl::AddPolicyAdministratorName(this, core::mem::transmute(&bstradmin), core::mem::transmute(&varreserved)).into()
            }
        }
        unsafe extern "system" fn DeletePolicyAdministratorName<Identity: IAzAuthorizationStore_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstradmin: *mut core::ffi::c_void, varreserved: super::super::System::Variant::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IAzAuthorizationStore_Impl::DeletePolicyAdministratorName(this, core::mem::transmute(&bstradmin), core::mem::transmute(&varreserved)).into()
            }
        }
        unsafe extern "system" fn AddPolicyReaderName<Identity: IAzAuthorizationStore_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrreader: *mut core::ffi::c_void, varreserved: super::super::System::Variant::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IAzAuthorizationStore_Impl::AddPolicyReaderName(this, core::mem::transmute(&bstrreader), core::mem::transmute(&varreserved)).into()
            }
        }
        unsafe extern "system" fn DeletePolicyReaderName<Identity: IAzAuthorizationStore_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrreader: *mut core::ffi::c_void, varreserved: super::super::System::Variant::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IAzAuthorizationStore_Impl::DeletePolicyReaderName(this, core::mem::transmute(&bstrreader), core::mem::transmute(&varreserved)).into()
            }
        }
        unsafe extern "system" fn DelegatedPolicyUsersName<Identity: IAzAuthorizationStore_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvardelegatedpolicyusers: *mut super::super::System::Variant::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IAzAuthorizationStore_Impl::DelegatedPolicyUsersName(this) {
                    Ok(ok__) => {
                        pvardelegatedpolicyusers.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn AddDelegatedPolicyUserName<Identity: IAzAuthorizationStore_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrdelegatedpolicyuser: *mut core::ffi::c_void, varreserved: super::super::System::Variant::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IAzAuthorizationStore_Impl::AddDelegatedPolicyUserName(this, core::mem::transmute(&bstrdelegatedpolicyuser), core::mem::transmute(&varreserved)).into()
            }
        }
        unsafe extern "system" fn DeleteDelegatedPolicyUserName<Identity: IAzAuthorizationStore_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrdelegatedpolicyuser: *mut core::ffi::c_void, varreserved: super::super::System::Variant::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IAzAuthorizationStore_Impl::DeleteDelegatedPolicyUserName(this, core::mem::transmute(&bstrdelegatedpolicyuser), core::mem::transmute(&varreserved)).into()
            }
        }
        unsafe extern "system" fn CloseApplication<Identity: IAzAuthorizationStore_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrapplicationname: *mut core::ffi::c_void, lflag: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IAzAuthorizationStore_Impl::CloseApplication(this, core::mem::transmute(&bstrapplicationname), core::mem::transmute_copy(&lflag)).into()
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            Description: Description::<Identity, OFFSET>,
            SetDescription: SetDescription::<Identity, OFFSET>,
            ApplicationData: ApplicationData::<Identity, OFFSET>,
            SetApplicationData: SetApplicationData::<Identity, OFFSET>,
            DomainTimeout: DomainTimeout::<Identity, OFFSET>,
            SetDomainTimeout: SetDomainTimeout::<Identity, OFFSET>,
            ScriptEngineTimeout: ScriptEngineTimeout::<Identity, OFFSET>,
            SetScriptEngineTimeout: SetScriptEngineTimeout::<Identity, OFFSET>,
            MaxScriptEngines: MaxScriptEngines::<Identity, OFFSET>,
            SetMaxScriptEngines: SetMaxScriptEngines::<Identity, OFFSET>,
            GenerateAudits: GenerateAudits::<Identity, OFFSET>,
            SetGenerateAudits: SetGenerateAudits::<Identity, OFFSET>,
            Writable: Writable::<Identity, OFFSET>,
            GetProperty: GetProperty::<Identity, OFFSET>,
            SetProperty: SetProperty::<Identity, OFFSET>,
            AddPropertyItem: AddPropertyItem::<Identity, OFFSET>,
            DeletePropertyItem: DeletePropertyItem::<Identity, OFFSET>,
            PolicyAdministrators: PolicyAdministrators::<Identity, OFFSET>,
            PolicyReaders: PolicyReaders::<Identity, OFFSET>,
            AddPolicyAdministrator: AddPolicyAdministrator::<Identity, OFFSET>,
            DeletePolicyAdministrator: DeletePolicyAdministrator::<Identity, OFFSET>,
            AddPolicyReader: AddPolicyReader::<Identity, OFFSET>,
            DeletePolicyReader: DeletePolicyReader::<Identity, OFFSET>,
            Initialize: Initialize::<Identity, OFFSET>,
            UpdateCache: UpdateCache::<Identity, OFFSET>,
            Delete: Delete::<Identity, OFFSET>,
            Applications: Applications::<Identity, OFFSET>,
            OpenApplication: OpenApplication::<Identity, OFFSET>,
            CreateApplication: CreateApplication::<Identity, OFFSET>,
            DeleteApplication: DeleteApplication::<Identity, OFFSET>,
            ApplicationGroups: ApplicationGroups::<Identity, OFFSET>,
            CreateApplicationGroup: CreateApplicationGroup::<Identity, OFFSET>,
            OpenApplicationGroup: OpenApplicationGroup::<Identity, OFFSET>,
            DeleteApplicationGroup: DeleteApplicationGroup::<Identity, OFFSET>,
            Submit: Submit::<Identity, OFFSET>,
            DelegatedPolicyUsers: DelegatedPolicyUsers::<Identity, OFFSET>,
            AddDelegatedPolicyUser: AddDelegatedPolicyUser::<Identity, OFFSET>,
            DeleteDelegatedPolicyUser: DeleteDelegatedPolicyUser::<Identity, OFFSET>,
            TargetMachine: TargetMachine::<Identity, OFFSET>,
            ApplyStoreSacl: ApplyStoreSacl::<Identity, OFFSET>,
            SetApplyStoreSacl: SetApplyStoreSacl::<Identity, OFFSET>,
            PolicyAdministratorsName: PolicyAdministratorsName::<Identity, OFFSET>,
            PolicyReadersName: PolicyReadersName::<Identity, OFFSET>,
            AddPolicyAdministratorName: AddPolicyAdministratorName::<Identity, OFFSET>,
            DeletePolicyAdministratorName: DeletePolicyAdministratorName::<Identity, OFFSET>,
            AddPolicyReaderName: AddPolicyReaderName::<Identity, OFFSET>,
            DeletePolicyReaderName: DeletePolicyReaderName::<Identity, OFFSET>,
            DelegatedPolicyUsersName: DelegatedPolicyUsersName::<Identity, OFFSET>,
            AddDelegatedPolicyUserName: AddDelegatedPolicyUserName::<Identity, OFFSET>,
            DeleteDelegatedPolicyUserName: DeleteDelegatedPolicyUserName::<Identity, OFFSET>,
            CloseApplication: CloseApplication::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IAzAuthorizationStore as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for IAzAuthorizationStore {}
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
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn OpenApplication2(&self, bstrapplicationname: &windows_core::BSTR, varreserved: &super::super::System::Variant::VARIANT) -> windows_core::Result<IAzApplication2> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).OpenApplication2)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrapplicationname), core::mem::transmute_copy(varreserved), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn CreateApplication2(&self, bstrapplicationname: &windows_core::BSTR, varreserved: &super::super::System::Variant::VARIANT) -> windows_core::Result<IAzApplication2> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateApplication2)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrapplicationname), core::mem::transmute_copy(varreserved), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IAzAuthorizationStore2_Vtbl {
    pub base__: IAzAuthorizationStore_Vtbl,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub OpenApplication2: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, super::super::System::Variant::VARIANT, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    OpenApplication2: usize,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub CreateApplication2: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, super::super::System::Variant::VARIANT, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    CreateApplication2: usize,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IAzAuthorizationStore2_Impl: IAzAuthorizationStore_Impl {
    fn OpenApplication2(&self, bstrapplicationname: &windows_core::BSTR, varreserved: &super::super::System::Variant::VARIANT) -> windows_core::Result<IAzApplication2>;
    fn CreateApplication2(&self, bstrapplicationname: &windows_core::BSTR, varreserved: &super::super::System::Variant::VARIANT) -> windows_core::Result<IAzApplication2>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IAzAuthorizationStore2_Vtbl {
    pub const fn new<Identity: IAzAuthorizationStore2_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn OpenApplication2<Identity: IAzAuthorizationStore2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrapplicationname: *mut core::ffi::c_void, varreserved: super::super::System::Variant::VARIANT, ppapplication: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IAzAuthorizationStore2_Impl::OpenApplication2(this, core::mem::transmute(&bstrapplicationname), core::mem::transmute(&varreserved)) {
                    Ok(ok__) => {
                        ppapplication.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CreateApplication2<Identity: IAzAuthorizationStore2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrapplicationname: *mut core::ffi::c_void, varreserved: super::super::System::Variant::VARIANT, ppapplication: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IAzAuthorizationStore2_Impl::CreateApplication2(this, core::mem::transmute(&bstrapplicationname), core::mem::transmute(&varreserved)) {
                    Ok(ok__) => {
                        ppapplication.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: IAzAuthorizationStore_Vtbl::new::<Identity, OFFSET>(),
            OpenApplication2: OpenApplication2::<Identity, OFFSET>,
            CreateApplication2: CreateApplication2::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IAzAuthorizationStore2 as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID || iid == &<IAzAuthorizationStore as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for IAzAuthorizationStore2 {}
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
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).IsUpdateNeeded)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn BizruleGroupSupported(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).BizruleGroupSupported)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn UpgradeStoresFunctionalLevel(&self, lfunctionallevel: i32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).UpgradeStoresFunctionalLevel)(windows_core::Interface::as_raw(self), lfunctionallevel).ok() }
    }
    pub unsafe fn IsFunctionalLevelUpgradeSupported(&self, lfunctionallevel: i32) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).IsFunctionalLevelUpgradeSupported)(windows_core::Interface::as_raw(self), lfunctionallevel, &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetSchemaVersion(&self, plmajorversion: *mut i32, plminorversion: *mut i32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetSchemaVersion)(windows_core::Interface::as_raw(self), plmajorversion as _, plminorversion as _).ok() }
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IAzAuthorizationStore3_Vtbl {
    pub base__: IAzAuthorizationStore2_Vtbl,
    pub IsUpdateNeeded: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub BizruleGroupSupported: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub UpgradeStoresFunctionalLevel: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub IsFunctionalLevelUpgradeSupported: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub GetSchemaVersion: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32, *mut i32) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IAzAuthorizationStore3_Impl: IAzAuthorizationStore2_Impl {
    fn IsUpdateNeeded(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn BizruleGroupSupported(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn UpgradeStoresFunctionalLevel(&self, lfunctionallevel: i32) -> windows_core::Result<()>;
    fn IsFunctionalLevelUpgradeSupported(&self, lfunctionallevel: i32) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn GetSchemaVersion(&self, plmajorversion: *mut i32, plminorversion: *mut i32) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IAzAuthorizationStore3_Vtbl {
    pub const fn new<Identity: IAzAuthorizationStore3_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn IsUpdateNeeded<Identity: IAzAuthorizationStore3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbisupdateneeded: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IAzAuthorizationStore3_Impl::IsUpdateNeeded(this) {
                    Ok(ok__) => {
                        pbisupdateneeded.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn BizruleGroupSupported<Identity: IAzAuthorizationStore3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbsupported: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IAzAuthorizationStore3_Impl::BizruleGroupSupported(this) {
                    Ok(ok__) => {
                        pbsupported.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn UpgradeStoresFunctionalLevel<Identity: IAzAuthorizationStore3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lfunctionallevel: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IAzAuthorizationStore3_Impl::UpgradeStoresFunctionalLevel(this, core::mem::transmute_copy(&lfunctionallevel)).into()
            }
        }
        unsafe extern "system" fn IsFunctionalLevelUpgradeSupported<Identity: IAzAuthorizationStore3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lfunctionallevel: i32, pbsupported: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IAzAuthorizationStore3_Impl::IsFunctionalLevelUpgradeSupported(this, core::mem::transmute_copy(&lfunctionallevel)) {
                    Ok(ok__) => {
                        pbsupported.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetSchemaVersion<Identity: IAzAuthorizationStore3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, plmajorversion: *mut i32, plminorversion: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IAzAuthorizationStore3_Impl::GetSchemaVersion(this, core::mem::transmute_copy(&plmajorversion), core::mem::transmute_copy(&plminorversion)).into()
            }
        }
        Self {
            base__: IAzAuthorizationStore2_Vtbl::new::<Identity, OFFSET>(),
            IsUpdateNeeded: IsUpdateNeeded::<Identity, OFFSET>,
            BizruleGroupSupported: BizruleGroupSupported::<Identity, OFFSET>,
            UpgradeStoresFunctionalLevel: UpgradeStoresFunctionalLevel::<Identity, OFFSET>,
            IsFunctionalLevelUpgradeSupported: IsFunctionalLevelUpgradeSupported::<Identity, OFFSET>,
            GetSchemaVersion: GetSchemaVersion::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IAzAuthorizationStore3 as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID || iid == &<IAzAuthorizationStore as windows_core::Interface>::IID || iid == &<IAzAuthorizationStore2 as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for IAzAuthorizationStore3 {}
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
    pub unsafe fn SetBusinessRuleResult(&self, bresult: bool) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetBusinessRuleResult)(windows_core::Interface::as_raw(self), bresult.into()).ok() }
    }
    pub unsafe fn SetBusinessRuleString(&self, bstrbusinessrulestring: &windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetBusinessRuleString)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrbusinessrulestring)).ok() }
    }
    pub unsafe fn BusinessRuleString(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).BusinessRuleString)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn GetParameter(&self, bstrparametername: &windows_core::BSTR) -> windows_core::Result<super::super::System::Variant::VARIANT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetParameter)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrparametername), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IAzBizRuleContext_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub SetBusinessRuleResult: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::BOOL) -> windows_core::HRESULT,
    pub SetBusinessRuleString: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub BusinessRuleString: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub GetParameter: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::System::Variant::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    GetParameter: usize,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IAzBizRuleContext_Impl: super::super::System::Com::IDispatch_Impl {
    fn SetBusinessRuleResult(&self, bresult: windows_core::BOOL) -> windows_core::Result<()>;
    fn SetBusinessRuleString(&self, bstrbusinessrulestring: &windows_core::BSTR) -> windows_core::Result<()>;
    fn BusinessRuleString(&self) -> windows_core::Result<windows_core::BSTR>;
    fn GetParameter(&self, bstrparametername: &windows_core::BSTR) -> windows_core::Result<super::super::System::Variant::VARIANT>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IAzBizRuleContext_Vtbl {
    pub const fn new<Identity: IAzBizRuleContext_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn SetBusinessRuleResult<Identity: IAzBizRuleContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bresult: windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IAzBizRuleContext_Impl::SetBusinessRuleResult(this, core::mem::transmute_copy(&bresult)).into()
            }
        }
        unsafe extern "system" fn SetBusinessRuleString<Identity: IAzBizRuleContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrbusinessrulestring: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IAzBizRuleContext_Impl::SetBusinessRuleString(this, core::mem::transmute(&bstrbusinessrulestring)).into()
            }
        }
        unsafe extern "system" fn BusinessRuleString<Identity: IAzBizRuleContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrbusinessrulestring: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IAzBizRuleContext_Impl::BusinessRuleString(this) {
                    Ok(ok__) => {
                        pbstrbusinessrulestring.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetParameter<Identity: IAzBizRuleContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrparametername: *mut core::ffi::c_void, pvarparametervalue: *mut super::super::System::Variant::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IAzBizRuleContext_Impl::GetParameter(this, core::mem::transmute(&bstrparametername)) {
                    Ok(ok__) => {
                        pvarparametervalue.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            SetBusinessRuleResult: SetBusinessRuleResult::<Identity, OFFSET>,
            SetBusinessRuleString: SetBusinessRuleString::<Identity, OFFSET>,
            BusinessRuleString: BusinessRuleString::<Identity, OFFSET>,
            GetParameter: GetParameter::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IAzBizRuleContext as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for IAzBizRuleContext {}
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
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn AddInterface(&self, bstrinterfacename: &windows_core::BSTR, linterfaceflag: i32, varinterface: &super::super::System::Variant::VARIANT) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).AddInterface)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrinterfacename), linterfaceflag, core::mem::transmute_copy(varinterface)).ok() }
    }
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn AddInterfaces(&self, varinterfacenames: &super::super::System::Variant::VARIANT, varinterfaceflags: &super::super::System::Variant::VARIANT, varinterfaces: &super::super::System::Variant::VARIANT) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).AddInterfaces)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(varinterfacenames), core::mem::transmute_copy(varinterfaceflags), core::mem::transmute_copy(varinterfaces)).ok() }
    }
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn GetInterfaceValue(&self, bstrinterfacename: &windows_core::BSTR, linterfaceflag: *mut i32, varinterface: *mut super::super::System::Variant::VARIANT) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetInterfaceValue)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrinterfacename), linterfaceflag as _, core::mem::transmute(varinterface)).ok() }
    }
    pub unsafe fn Remove(&self, bstrinterfacename: &windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Remove)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrinterfacename)).ok() }
    }
    pub unsafe fn RemoveAll(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).RemoveAll)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn Count(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Count)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IAzBizRuleInterfaces_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub AddInterface: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, i32, super::super::System::Variant::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    AddInterface: usize,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub AddInterfaces: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::System::Variant::VARIANT, super::super::System::Variant::VARIANT, super::super::System::Variant::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    AddInterfaces: usize,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub GetInterfaceValue: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut i32, *mut super::super::System::Variant::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    GetInterfaceValue: usize,
    pub Remove: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub RemoveAll: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Count: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IAzBizRuleInterfaces_Impl: super::super::System::Com::IDispatch_Impl {
    fn AddInterface(&self, bstrinterfacename: &windows_core::BSTR, linterfaceflag: i32, varinterface: &super::super::System::Variant::VARIANT) -> windows_core::Result<()>;
    fn AddInterfaces(&self, varinterfacenames: &super::super::System::Variant::VARIANT, varinterfaceflags: &super::super::System::Variant::VARIANT, varinterfaces: &super::super::System::Variant::VARIANT) -> windows_core::Result<()>;
    fn GetInterfaceValue(&self, bstrinterfacename: &windows_core::BSTR, linterfaceflag: *mut i32, varinterface: *mut super::super::System::Variant::VARIANT) -> windows_core::Result<()>;
    fn Remove(&self, bstrinterfacename: &windows_core::BSTR) -> windows_core::Result<()>;
    fn RemoveAll(&self) -> windows_core::Result<()>;
    fn Count(&self) -> windows_core::Result<u32>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IAzBizRuleInterfaces_Vtbl {
    pub const fn new<Identity: IAzBizRuleInterfaces_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn AddInterface<Identity: IAzBizRuleInterfaces_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrinterfacename: *mut core::ffi::c_void, linterfaceflag: i32, varinterface: super::super::System::Variant::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IAzBizRuleInterfaces_Impl::AddInterface(this, core::mem::transmute(&bstrinterfacename), core::mem::transmute_copy(&linterfaceflag), core::mem::transmute(&varinterface)).into()
            }
        }
        unsafe extern "system" fn AddInterfaces<Identity: IAzBizRuleInterfaces_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, varinterfacenames: super::super::System::Variant::VARIANT, varinterfaceflags: super::super::System::Variant::VARIANT, varinterfaces: super::super::System::Variant::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IAzBizRuleInterfaces_Impl::AddInterfaces(this, core::mem::transmute(&varinterfacenames), core::mem::transmute(&varinterfaceflags), core::mem::transmute(&varinterfaces)).into()
            }
        }
        unsafe extern "system" fn GetInterfaceValue<Identity: IAzBizRuleInterfaces_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrinterfacename: *mut core::ffi::c_void, linterfaceflag: *mut i32, varinterface: *mut super::super::System::Variant::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IAzBizRuleInterfaces_Impl::GetInterfaceValue(this, core::mem::transmute(&bstrinterfacename), core::mem::transmute_copy(&linterfaceflag), core::mem::transmute_copy(&varinterface)).into()
            }
        }
        unsafe extern "system" fn Remove<Identity: IAzBizRuleInterfaces_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrinterfacename: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IAzBizRuleInterfaces_Impl::Remove(this, core::mem::transmute(&bstrinterfacename)).into()
            }
        }
        unsafe extern "system" fn RemoveAll<Identity: IAzBizRuleInterfaces_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IAzBizRuleInterfaces_Impl::RemoveAll(this).into()
            }
        }
        unsafe extern "system" fn Count<Identity: IAzBizRuleInterfaces_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, plcount: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IAzBizRuleInterfaces_Impl::Count(this) {
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
            AddInterface: AddInterface::<Identity, OFFSET>,
            AddInterfaces: AddInterfaces::<Identity, OFFSET>,
            GetInterfaceValue: GetInterfaceValue::<Identity, OFFSET>,
            Remove: Remove::<Identity, OFFSET>,
            RemoveAll: RemoveAll::<Identity, OFFSET>,
            Count: Count::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IAzBizRuleInterfaces as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for IAzBizRuleInterfaces {}
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
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn AddParameter(&self, bstrparametername: &windows_core::BSTR, varparametervalue: &super::super::System::Variant::VARIANT) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).AddParameter)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrparametername), core::mem::transmute_copy(varparametervalue)).ok() }
    }
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn AddParameters(&self, varparameternames: &super::super::System::Variant::VARIANT, varparametervalues: &super::super::System::Variant::VARIANT) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).AddParameters)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(varparameternames), core::mem::transmute_copy(varparametervalues)).ok() }
    }
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn GetParameterValue(&self, bstrparametername: &windows_core::BSTR) -> windows_core::Result<super::super::System::Variant::VARIANT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetParameterValue)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrparametername), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn Remove(&self, varparametername: &windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Remove)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(varparametername)).ok() }
    }
    pub unsafe fn RemoveAll(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).RemoveAll)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn Count(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Count)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IAzBizRuleParameters_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub AddParameter: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, super::super::System::Variant::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    AddParameter: usize,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub AddParameters: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::System::Variant::VARIANT, super::super::System::Variant::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    AddParameters: usize,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub GetParameterValue: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::System::Variant::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    GetParameterValue: usize,
    pub Remove: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub RemoveAll: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Count: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IAzBizRuleParameters_Impl: super::super::System::Com::IDispatch_Impl {
    fn AddParameter(&self, bstrparametername: &windows_core::BSTR, varparametervalue: &super::super::System::Variant::VARIANT) -> windows_core::Result<()>;
    fn AddParameters(&self, varparameternames: &super::super::System::Variant::VARIANT, varparametervalues: &super::super::System::Variant::VARIANT) -> windows_core::Result<()>;
    fn GetParameterValue(&self, bstrparametername: &windows_core::BSTR) -> windows_core::Result<super::super::System::Variant::VARIANT>;
    fn Remove(&self, varparametername: &windows_core::BSTR) -> windows_core::Result<()>;
    fn RemoveAll(&self) -> windows_core::Result<()>;
    fn Count(&self) -> windows_core::Result<u32>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IAzBizRuleParameters_Vtbl {
    pub const fn new<Identity: IAzBizRuleParameters_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn AddParameter<Identity: IAzBizRuleParameters_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrparametername: *mut core::ffi::c_void, varparametervalue: super::super::System::Variant::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IAzBizRuleParameters_Impl::AddParameter(this, core::mem::transmute(&bstrparametername), core::mem::transmute(&varparametervalue)).into()
            }
        }
        unsafe extern "system" fn AddParameters<Identity: IAzBizRuleParameters_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, varparameternames: super::super::System::Variant::VARIANT, varparametervalues: super::super::System::Variant::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IAzBizRuleParameters_Impl::AddParameters(this, core::mem::transmute(&varparameternames), core::mem::transmute(&varparametervalues)).into()
            }
        }
        unsafe extern "system" fn GetParameterValue<Identity: IAzBizRuleParameters_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrparametername: *mut core::ffi::c_void, pvarparametervalue: *mut super::super::System::Variant::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IAzBizRuleParameters_Impl::GetParameterValue(this, core::mem::transmute(&bstrparametername)) {
                    Ok(ok__) => {
                        pvarparametervalue.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Remove<Identity: IAzBizRuleParameters_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, varparametername: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IAzBizRuleParameters_Impl::Remove(this, core::mem::transmute(&varparametername)).into()
            }
        }
        unsafe extern "system" fn RemoveAll<Identity: IAzBizRuleParameters_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IAzBizRuleParameters_Impl::RemoveAll(this).into()
            }
        }
        unsafe extern "system" fn Count<Identity: IAzBizRuleParameters_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, plcount: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IAzBizRuleParameters_Impl::Count(this) {
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
            AddParameter: AddParameter::<Identity, OFFSET>,
            AddParameters: AddParameters::<Identity, OFFSET>,
            GetParameterValue: GetParameterValue::<Identity, OFFSET>,
            Remove: Remove::<Identity, OFFSET>,
            RemoveAll: RemoveAll::<Identity, OFFSET>,
            Count: Count::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IAzBizRuleParameters as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for IAzBizRuleParameters {}
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
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn AccessCheck(&self, bstrobjectname: &windows_core::BSTR, varscopenames: &super::super::System::Variant::VARIANT, varoperations: &super::super::System::Variant::VARIANT, varparameternames: &super::super::System::Variant::VARIANT, varparametervalues: &super::super::System::Variant::VARIANT, varinterfacenames: &super::super::System::Variant::VARIANT, varinterfaceflags: &super::super::System::Variant::VARIANT, varinterfaces: &super::super::System::Variant::VARIANT) -> windows_core::Result<super::super::System::Variant::VARIANT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).AccessCheck)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrobjectname), core::mem::transmute_copy(varscopenames), core::mem::transmute_copy(varoperations), core::mem::transmute_copy(varparameternames), core::mem::transmute_copy(varparametervalues), core::mem::transmute_copy(varinterfacenames), core::mem::transmute_copy(varinterfaceflags), core::mem::transmute_copy(varinterfaces), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn GetBusinessRuleString(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetBusinessRuleString)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn UserDn(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).UserDn)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn UserSamCompat(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).UserSamCompat)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn UserDisplay(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).UserDisplay)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn UserGuid(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).UserGuid)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn UserCanonical(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).UserCanonical)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn UserUpn(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).UserUpn)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn UserDnsSamCompat(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).UserDnsSamCompat)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn GetProperty(&self, lpropid: i32, varreserved: &super::super::System::Variant::VARIANT) -> windows_core::Result<super::super::System::Variant::VARIANT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetProperty)(windows_core::Interface::as_raw(self), lpropid, core::mem::transmute_copy(varreserved), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn GetRoles(&self, bstrscopename: &windows_core::BSTR) -> windows_core::Result<super::super::System::Variant::VARIANT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetRoles)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrscopename), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn RoleForAccessCheck(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).RoleForAccessCheck)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn SetRoleForAccessCheck(&self, bstrprop: &windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetRoleForAccessCheck)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrprop)).ok() }
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IAzClientContext_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub AccessCheck: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, super::super::System::Variant::VARIANT, super::super::System::Variant::VARIANT, super::super::System::Variant::VARIANT, super::super::System::Variant::VARIANT, super::super::System::Variant::VARIANT, super::super::System::Variant::VARIANT, super::super::System::Variant::VARIANT, *mut super::super::System::Variant::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    AccessCheck: usize,
    pub GetBusinessRuleString: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub UserDn: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub UserSamCompat: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub UserDisplay: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub UserGuid: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub UserCanonical: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub UserUpn: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub UserDnsSamCompat: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub GetProperty: unsafe extern "system" fn(*mut core::ffi::c_void, i32, super::super::System::Variant::VARIANT, *mut super::super::System::Variant::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    GetProperty: usize,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub GetRoles: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::System::Variant::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    GetRoles: usize,
    pub RoleForAccessCheck: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetRoleForAccessCheck: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IAzClientContext_Impl: super::super::System::Com::IDispatch_Impl {
    fn AccessCheck(&self, bstrobjectname: &windows_core::BSTR, varscopenames: &super::super::System::Variant::VARIANT, varoperations: &super::super::System::Variant::VARIANT, varparameternames: &super::super::System::Variant::VARIANT, varparametervalues: &super::super::System::Variant::VARIANT, varinterfacenames: &super::super::System::Variant::VARIANT, varinterfaceflags: &super::super::System::Variant::VARIANT, varinterfaces: &super::super::System::Variant::VARIANT) -> windows_core::Result<super::super::System::Variant::VARIANT>;
    fn GetBusinessRuleString(&self) -> windows_core::Result<windows_core::BSTR>;
    fn UserDn(&self) -> windows_core::Result<windows_core::BSTR>;
    fn UserSamCompat(&self) -> windows_core::Result<windows_core::BSTR>;
    fn UserDisplay(&self) -> windows_core::Result<windows_core::BSTR>;
    fn UserGuid(&self) -> windows_core::Result<windows_core::BSTR>;
    fn UserCanonical(&self) -> windows_core::Result<windows_core::BSTR>;
    fn UserUpn(&self) -> windows_core::Result<windows_core::BSTR>;
    fn UserDnsSamCompat(&self) -> windows_core::Result<windows_core::BSTR>;
    fn GetProperty(&self, lpropid: i32, varreserved: &super::super::System::Variant::VARIANT) -> windows_core::Result<super::super::System::Variant::VARIANT>;
    fn GetRoles(&self, bstrscopename: &windows_core::BSTR) -> windows_core::Result<super::super::System::Variant::VARIANT>;
    fn RoleForAccessCheck(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetRoleForAccessCheck(&self, bstrprop: &windows_core::BSTR) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IAzClientContext_Vtbl {
    pub const fn new<Identity: IAzClientContext_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn AccessCheck<Identity: IAzClientContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrobjectname: *mut core::ffi::c_void, varscopenames: super::super::System::Variant::VARIANT, varoperations: super::super::System::Variant::VARIANT, varparameternames: super::super::System::Variant::VARIANT, varparametervalues: super::super::System::Variant::VARIANT, varinterfacenames: super::super::System::Variant::VARIANT, varinterfaceflags: super::super::System::Variant::VARIANT, varinterfaces: super::super::System::Variant::VARIANT, pvarresults: *mut super::super::System::Variant::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IAzClientContext_Impl::AccessCheck(this, core::mem::transmute(&bstrobjectname), core::mem::transmute(&varscopenames), core::mem::transmute(&varoperations), core::mem::transmute(&varparameternames), core::mem::transmute(&varparametervalues), core::mem::transmute(&varinterfacenames), core::mem::transmute(&varinterfaceflags), core::mem::transmute(&varinterfaces)) {
                    Ok(ok__) => {
                        pvarresults.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetBusinessRuleString<Identity: IAzClientContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrbusinessrulestring: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IAzClientContext_Impl::GetBusinessRuleString(this) {
                    Ok(ok__) => {
                        pbstrbusinessrulestring.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn UserDn<Identity: IAzClientContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrprop: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IAzClientContext_Impl::UserDn(this) {
                    Ok(ok__) => {
                        pbstrprop.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn UserSamCompat<Identity: IAzClientContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrprop: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IAzClientContext_Impl::UserSamCompat(this) {
                    Ok(ok__) => {
                        pbstrprop.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn UserDisplay<Identity: IAzClientContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrprop: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IAzClientContext_Impl::UserDisplay(this) {
                    Ok(ok__) => {
                        pbstrprop.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn UserGuid<Identity: IAzClientContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrprop: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IAzClientContext_Impl::UserGuid(this) {
                    Ok(ok__) => {
                        pbstrprop.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn UserCanonical<Identity: IAzClientContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrprop: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IAzClientContext_Impl::UserCanonical(this) {
                    Ok(ok__) => {
                        pbstrprop.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn UserUpn<Identity: IAzClientContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrprop: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IAzClientContext_Impl::UserUpn(this) {
                    Ok(ok__) => {
                        pbstrprop.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn UserDnsSamCompat<Identity: IAzClientContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrprop: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IAzClientContext_Impl::UserDnsSamCompat(this) {
                    Ok(ok__) => {
                        pbstrprop.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetProperty<Identity: IAzClientContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lpropid: i32, varreserved: super::super::System::Variant::VARIANT, pvarprop: *mut super::super::System::Variant::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IAzClientContext_Impl::GetProperty(this, core::mem::transmute_copy(&lpropid), core::mem::transmute(&varreserved)) {
                    Ok(ok__) => {
                        pvarprop.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetRoles<Identity: IAzClientContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrscopename: *mut core::ffi::c_void, pvarrolenames: *mut super::super::System::Variant::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IAzClientContext_Impl::GetRoles(this, core::mem::transmute(&bstrscopename)) {
                    Ok(ok__) => {
                        pvarrolenames.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn RoleForAccessCheck<Identity: IAzClientContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrprop: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IAzClientContext_Impl::RoleForAccessCheck(this) {
                    Ok(ok__) => {
                        pbstrprop.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetRoleForAccessCheck<Identity: IAzClientContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrprop: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IAzClientContext_Impl::SetRoleForAccessCheck(this, core::mem::transmute(&bstrprop)).into()
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            AccessCheck: AccessCheck::<Identity, OFFSET>,
            GetBusinessRuleString: GetBusinessRuleString::<Identity, OFFSET>,
            UserDn: UserDn::<Identity, OFFSET>,
            UserSamCompat: UserSamCompat::<Identity, OFFSET>,
            UserDisplay: UserDisplay::<Identity, OFFSET>,
            UserGuid: UserGuid::<Identity, OFFSET>,
            UserCanonical: UserCanonical::<Identity, OFFSET>,
            UserUpn: UserUpn::<Identity, OFFSET>,
            UserDnsSamCompat: UserDnsSamCompat::<Identity, OFFSET>,
            GetProperty: GetProperty::<Identity, OFFSET>,
            GetRoles: GetRoles::<Identity, OFFSET>,
            RoleForAccessCheck: RoleForAccessCheck::<Identity, OFFSET>,
            SetRoleForAccessCheck: SetRoleForAccessCheck::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IAzClientContext as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for IAzClientContext {}
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
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn GetAssignedScopesPage(&self, loptions: i32, pagesize: i32, pvarcursor: *mut super::super::System::Variant::VARIANT, pvarscopenames: *mut super::super::System::Variant::VARIANT) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetAssignedScopesPage)(windows_core::Interface::as_raw(self), loptions, pagesize, core::mem::transmute(pvarcursor), core::mem::transmute(pvarscopenames)).ok() }
    }
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn AddRoles(&self, varroles: &super::super::System::Variant::VARIANT, bstrscopename: &windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).AddRoles)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(varroles), core::mem::transmute_copy(bstrscopename)).ok() }
    }
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn AddApplicationGroups(&self, varapplicationgroups: &super::super::System::Variant::VARIANT) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).AddApplicationGroups)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(varapplicationgroups)).ok() }
    }
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn AddStringSids(&self, varstringsids: &super::super::System::Variant::VARIANT) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).AddStringSids)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(varstringsids)).ok() }
    }
    pub unsafe fn SetLDAPQueryDN(&self, bstrldapquerydn: &windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetLDAPQueryDN)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrldapquerydn)).ok() }
    }
    pub unsafe fn LDAPQueryDN(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).LDAPQueryDN)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IAzClientContext2_Vtbl {
    pub base__: IAzClientContext_Vtbl,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub GetAssignedScopesPage: unsafe extern "system" fn(*mut core::ffi::c_void, i32, i32, *mut super::super::System::Variant::VARIANT, *mut super::super::System::Variant::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    GetAssignedScopesPage: usize,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub AddRoles: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::System::Variant::VARIANT, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    AddRoles: usize,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub AddApplicationGroups: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::System::Variant::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    AddApplicationGroups: usize,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub AddStringSids: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::System::Variant::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    AddStringSids: usize,
    pub SetLDAPQueryDN: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub LDAPQueryDN: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IAzClientContext2_Impl: IAzClientContext_Impl {
    fn GetAssignedScopesPage(&self, loptions: i32, pagesize: i32, pvarcursor: *mut super::super::System::Variant::VARIANT, pvarscopenames: *mut super::super::System::Variant::VARIANT) -> windows_core::Result<()>;
    fn AddRoles(&self, varroles: &super::super::System::Variant::VARIANT, bstrscopename: &windows_core::BSTR) -> windows_core::Result<()>;
    fn AddApplicationGroups(&self, varapplicationgroups: &super::super::System::Variant::VARIANT) -> windows_core::Result<()>;
    fn AddStringSids(&self, varstringsids: &super::super::System::Variant::VARIANT) -> windows_core::Result<()>;
    fn SetLDAPQueryDN(&self, bstrldapquerydn: &windows_core::BSTR) -> windows_core::Result<()>;
    fn LDAPQueryDN(&self) -> windows_core::Result<windows_core::BSTR>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IAzClientContext2_Vtbl {
    pub const fn new<Identity: IAzClientContext2_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetAssignedScopesPage<Identity: IAzClientContext2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, loptions: i32, pagesize: i32, pvarcursor: *mut super::super::System::Variant::VARIANT, pvarscopenames: *mut super::super::System::Variant::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IAzClientContext2_Impl::GetAssignedScopesPage(this, core::mem::transmute_copy(&loptions), core::mem::transmute_copy(&pagesize), core::mem::transmute_copy(&pvarcursor), core::mem::transmute_copy(&pvarscopenames)).into()
            }
        }
        unsafe extern "system" fn AddRoles<Identity: IAzClientContext2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, varroles: super::super::System::Variant::VARIANT, bstrscopename: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IAzClientContext2_Impl::AddRoles(this, core::mem::transmute(&varroles), core::mem::transmute(&bstrscopename)).into()
            }
        }
        unsafe extern "system" fn AddApplicationGroups<Identity: IAzClientContext2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, varapplicationgroups: super::super::System::Variant::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IAzClientContext2_Impl::AddApplicationGroups(this, core::mem::transmute(&varapplicationgroups)).into()
            }
        }
        unsafe extern "system" fn AddStringSids<Identity: IAzClientContext2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, varstringsids: super::super::System::Variant::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IAzClientContext2_Impl::AddStringSids(this, core::mem::transmute(&varstringsids)).into()
            }
        }
        unsafe extern "system" fn SetLDAPQueryDN<Identity: IAzClientContext2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrldapquerydn: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IAzClientContext2_Impl::SetLDAPQueryDN(this, core::mem::transmute(&bstrldapquerydn)).into()
            }
        }
        unsafe extern "system" fn LDAPQueryDN<Identity: IAzClientContext2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrldapquerydn: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IAzClientContext2_Impl::LDAPQueryDN(this) {
                    Ok(ok__) => {
                        pbstrldapquerydn.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: IAzClientContext_Vtbl::new::<Identity, OFFSET>(),
            GetAssignedScopesPage: GetAssignedScopesPage::<Identity, OFFSET>,
            AddRoles: AddRoles::<Identity, OFFSET>,
            AddApplicationGroups: AddApplicationGroups::<Identity, OFFSET>,
            AddStringSids: AddStringSids::<Identity, OFFSET>,
            SetLDAPQueryDN: SetLDAPQueryDN::<Identity, OFFSET>,
            LDAPQueryDN: LDAPQueryDN::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IAzClientContext2 as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID || iid == &<IAzClientContext as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for IAzClientContext2 {}
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
    pub unsafe fn AccessCheck2(&self, bstrobjectname: &windows_core::BSTR, bstrscopename: &windows_core::BSTR, loperation: i32) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).AccessCheck2)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrobjectname), core::mem::transmute_copy(bstrscopename), loperation, &mut result__).map(|| result__)
        }
    }
    pub unsafe fn IsInRoleAssignment(&self, bstrscopename: &windows_core::BSTR, bstrrolename: &windows_core::BSTR) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).IsInRoleAssignment)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrscopename), core::mem::transmute_copy(bstrrolename), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetOperations(&self, bstrscopename: &windows_core::BSTR) -> windows_core::Result<IAzOperations> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetOperations)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrscopename), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetTasks(&self, bstrscopename: &windows_core::BSTR) -> windows_core::Result<IAzTasks> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetTasks)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrscopename), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn BizRuleParameters(&self) -> windows_core::Result<IAzBizRuleParameters> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).BizRuleParameters)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn BizRuleInterfaces(&self) -> windows_core::Result<IAzBizRuleInterfaces> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).BizRuleInterfaces)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn GetGroups(&self, bstrscopename: &windows_core::BSTR, uloptions: AZ_PROP_CONSTANTS) -> windows_core::Result<super::super::System::Variant::VARIANT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetGroups)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrscopename), uloptions.0 as _, &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn Sids(&self) -> windows_core::Result<super::super::System::Variant::VARIANT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Sids)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IAzClientContext3_Vtbl {
    pub base__: IAzClientContext2_Vtbl,
    pub AccessCheck2: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, i32, *mut u32) -> windows_core::HRESULT,
    pub IsInRoleAssignment: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub GetOperations: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetTasks: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub BizRuleParameters: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub BizRuleInterfaces: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub GetGroups: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, u32, *mut super::super::System::Variant::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    GetGroups: usize,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub Sids: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::System::Variant::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    Sids: usize,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IAzClientContext3_Impl: IAzClientContext2_Impl {
    fn AccessCheck2(&self, bstrobjectname: &windows_core::BSTR, bstrscopename: &windows_core::BSTR, loperation: i32) -> windows_core::Result<u32>;
    fn IsInRoleAssignment(&self, bstrscopename: &windows_core::BSTR, bstrrolename: &windows_core::BSTR) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn GetOperations(&self, bstrscopename: &windows_core::BSTR) -> windows_core::Result<IAzOperations>;
    fn GetTasks(&self, bstrscopename: &windows_core::BSTR) -> windows_core::Result<IAzTasks>;
    fn BizRuleParameters(&self) -> windows_core::Result<IAzBizRuleParameters>;
    fn BizRuleInterfaces(&self) -> windows_core::Result<IAzBizRuleInterfaces>;
    fn GetGroups(&self, bstrscopename: &windows_core::BSTR, uloptions: &AZ_PROP_CONSTANTS) -> windows_core::Result<super::super::System::Variant::VARIANT>;
    fn Sids(&self) -> windows_core::Result<super::super::System::Variant::VARIANT>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IAzClientContext3_Vtbl {
    pub const fn new<Identity: IAzClientContext3_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn AccessCheck2<Identity: IAzClientContext3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrobjectname: *mut core::ffi::c_void, bstrscopename: *mut core::ffi::c_void, loperation: i32, plresult: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IAzClientContext3_Impl::AccessCheck2(this, core::mem::transmute(&bstrobjectname), core::mem::transmute(&bstrscopename), core::mem::transmute_copy(&loperation)) {
                    Ok(ok__) => {
                        plresult.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn IsInRoleAssignment<Identity: IAzClientContext3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrscopename: *mut core::ffi::c_void, bstrrolename: *mut core::ffi::c_void, pbisinrole: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IAzClientContext3_Impl::IsInRoleAssignment(this, core::mem::transmute(&bstrscopename), core::mem::transmute(&bstrrolename)) {
                    Ok(ok__) => {
                        pbisinrole.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetOperations<Identity: IAzClientContext3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrscopename: *mut core::ffi::c_void, ppoperationcollection: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IAzClientContext3_Impl::GetOperations(this, core::mem::transmute(&bstrscopename)) {
                    Ok(ok__) => {
                        ppoperationcollection.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetTasks<Identity: IAzClientContext3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrscopename: *mut core::ffi::c_void, pptaskcollection: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IAzClientContext3_Impl::GetTasks(this, core::mem::transmute(&bstrscopename)) {
                    Ok(ok__) => {
                        pptaskcollection.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn BizRuleParameters<Identity: IAzClientContext3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppbizruleparam: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IAzClientContext3_Impl::BizRuleParameters(this) {
                    Ok(ok__) => {
                        ppbizruleparam.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn BizRuleInterfaces<Identity: IAzClientContext3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppbizruleinterfaces: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IAzClientContext3_Impl::BizRuleInterfaces(this) {
                    Ok(ok__) => {
                        ppbizruleinterfaces.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetGroups<Identity: IAzClientContext3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrscopename: *mut core::ffi::c_void, uloptions: u32, pgrouparray: *mut super::super::System::Variant::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IAzClientContext3_Impl::GetGroups(this, core::mem::transmute(&bstrscopename), core::mem::transmute(&uloptions)) {
                    Ok(ok__) => {
                        pgrouparray.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Sids<Identity: IAzClientContext3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pstringsidarray: *mut super::super::System::Variant::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IAzClientContext3_Impl::Sids(this) {
                    Ok(ok__) => {
                        pstringsidarray.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: IAzClientContext2_Vtbl::new::<Identity, OFFSET>(),
            AccessCheck2: AccessCheck2::<Identity, OFFSET>,
            IsInRoleAssignment: IsInRoleAssignment::<Identity, OFFSET>,
            GetOperations: GetOperations::<Identity, OFFSET>,
            GetTasks: GetTasks::<Identity, OFFSET>,
            BizRuleParameters: BizRuleParameters::<Identity, OFFSET>,
            BizRuleInterfaces: BizRuleInterfaces::<Identity, OFFSET>,
            GetGroups: GetGroups::<Identity, OFFSET>,
            Sids: Sids::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IAzClientContext3 as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID || iid == &<IAzClientContext as windows_core::Interface>::IID || iid == &<IAzClientContext2 as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for IAzClientContext3 {}
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
    pub unsafe fn NameFromSid(&self, bstrsid: &windows_core::BSTR, psidtype: *mut i32, pbstrname: *mut windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).NameFromSid)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrsid), psidtype as _, core::mem::transmute(pbstrname)).ok() }
    }
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn NamesFromSids(&self, vsids: &super::super::System::Variant::VARIANT, pvsidtypes: *mut super::super::System::Variant::VARIANT, pvnames: *mut super::super::System::Variant::VARIANT) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).NamesFromSids)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(vsids), core::mem::transmute(pvsidtypes), core::mem::transmute(pvnames)).ok() }
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IAzNameResolver_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub NameFromSid: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut i32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub NamesFromSids: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::System::Variant::VARIANT, *mut super::super::System::Variant::VARIANT, *mut super::super::System::Variant::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    NamesFromSids: usize,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IAzNameResolver_Impl: super::super::System::Com::IDispatch_Impl {
    fn NameFromSid(&self, bstrsid: &windows_core::BSTR, psidtype: *mut i32, pbstrname: *mut windows_core::BSTR) -> windows_core::Result<()>;
    fn NamesFromSids(&self, vsids: &super::super::System::Variant::VARIANT, pvsidtypes: *mut super::super::System::Variant::VARIANT, pvnames: *mut super::super::System::Variant::VARIANT) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IAzNameResolver_Vtbl {
    pub const fn new<Identity: IAzNameResolver_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn NameFromSid<Identity: IAzNameResolver_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrsid: *mut core::ffi::c_void, psidtype: *mut i32, pbstrname: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IAzNameResolver_Impl::NameFromSid(this, core::mem::transmute(&bstrsid), core::mem::transmute_copy(&psidtype), core::mem::transmute_copy(&pbstrname)).into()
            }
        }
        unsafe extern "system" fn NamesFromSids<Identity: IAzNameResolver_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, vsids: super::super::System::Variant::VARIANT, pvsidtypes: *mut super::super::System::Variant::VARIANT, pvnames: *mut super::super::System::Variant::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IAzNameResolver_Impl::NamesFromSids(this, core::mem::transmute(&vsids), core::mem::transmute_copy(&pvsidtypes), core::mem::transmute_copy(&pvnames)).into()
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            NameFromSid: NameFromSid::<Identity, OFFSET>,
            NamesFromSids: NamesFromSids::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IAzNameResolver as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for IAzNameResolver {}
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
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn GetPrincipals(&self, hparentwnd: super::super::Foundation::HWND, bstrtitle: &windows_core::BSTR, pvsidtypes: *mut super::super::System::Variant::VARIANT, pvnames: *mut super::super::System::Variant::VARIANT, pvsids: *mut super::super::System::Variant::VARIANT) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetPrincipals)(windows_core::Interface::as_raw(self), hparentwnd, core::mem::transmute_copy(bstrtitle), core::mem::transmute(pvsidtypes), core::mem::transmute(pvnames), core::mem::transmute(pvsids)).ok() }
    }
    pub unsafe fn Name(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Name)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IAzObjectPicker_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub GetPrincipals: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::HWND, *mut core::ffi::c_void, *mut super::super::System::Variant::VARIANT, *mut super::super::System::Variant::VARIANT, *mut super::super::System::Variant::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    GetPrincipals: usize,
    pub Name: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IAzObjectPicker_Impl: super::super::System::Com::IDispatch_Impl {
    fn GetPrincipals(&self, hparentwnd: super::super::Foundation::HWND, bstrtitle: &windows_core::BSTR, pvsidtypes: *mut super::super::System::Variant::VARIANT, pvnames: *mut super::super::System::Variant::VARIANT, pvsids: *mut super::super::System::Variant::VARIANT) -> windows_core::Result<()>;
    fn Name(&self) -> windows_core::Result<windows_core::BSTR>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IAzObjectPicker_Vtbl {
    pub const fn new<Identity: IAzObjectPicker_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetPrincipals<Identity: IAzObjectPicker_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hparentwnd: super::super::Foundation::HWND, bstrtitle: *mut core::ffi::c_void, pvsidtypes: *mut super::super::System::Variant::VARIANT, pvnames: *mut super::super::System::Variant::VARIANT, pvsids: *mut super::super::System::Variant::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IAzObjectPicker_Impl::GetPrincipals(this, core::mem::transmute_copy(&hparentwnd), core::mem::transmute(&bstrtitle), core::mem::transmute_copy(&pvsidtypes), core::mem::transmute_copy(&pvnames), core::mem::transmute_copy(&pvsids)).into()
            }
        }
        unsafe extern "system" fn Name<Identity: IAzObjectPicker_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrname: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IAzObjectPicker_Impl::Name(this) {
                    Ok(ok__) => {
                        pbstrname.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            GetPrincipals: GetPrincipals::<Identity, OFFSET>,
            Name: Name::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IAzObjectPicker as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for IAzObjectPicker {}
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
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Name)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn SetName(&self, bstrname: &windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetName)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrname)).ok() }
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
    pub unsafe fn ApplicationData(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ApplicationData)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn SetApplicationData(&self, bstrapplicationdata: &windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetApplicationData)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrapplicationdata)).ok() }
    }
    pub unsafe fn OperationID(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).OperationID)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetOperationID(&self, lprop: i32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetOperationID)(windows_core::Interface::as_raw(self), lprop).ok() }
    }
    pub unsafe fn Writable(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Writable)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn GetProperty(&self, lpropid: i32, varreserved: &super::super::System::Variant::VARIANT) -> windows_core::Result<super::super::System::Variant::VARIANT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetProperty)(windows_core::Interface::as_raw(self), lpropid, core::mem::transmute_copy(varreserved), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn SetProperty(&self, lpropid: i32, varprop: &super::super::System::Variant::VARIANT, varreserved: &super::super::System::Variant::VARIANT) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetProperty)(windows_core::Interface::as_raw(self), lpropid, core::mem::transmute_copy(varprop), core::mem::transmute_copy(varreserved)).ok() }
    }
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn Submit(&self, lflags: i32, varreserved: &super::super::System::Variant::VARIANT) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Submit)(windows_core::Interface::as_raw(self), lflags, core::mem::transmute_copy(varreserved)).ok() }
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IAzOperation_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub Name: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Description: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetDescription: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ApplicationData: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetApplicationData: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub OperationID: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetOperationID: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub Writable: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::BOOL) -> windows_core::HRESULT,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub GetProperty: unsafe extern "system" fn(*mut core::ffi::c_void, i32, super::super::System::Variant::VARIANT, *mut super::super::System::Variant::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    GetProperty: usize,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub SetProperty: unsafe extern "system" fn(*mut core::ffi::c_void, i32, super::super::System::Variant::VARIANT, super::super::System::Variant::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    SetProperty: usize,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub Submit: unsafe extern "system" fn(*mut core::ffi::c_void, i32, super::super::System::Variant::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    Submit: usize,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IAzOperation_Impl: super::super::System::Com::IDispatch_Impl {
    fn Name(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetName(&self, bstrname: &windows_core::BSTR) -> windows_core::Result<()>;
    fn Description(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetDescription(&self, bstrdescription: &windows_core::BSTR) -> windows_core::Result<()>;
    fn ApplicationData(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetApplicationData(&self, bstrapplicationdata: &windows_core::BSTR) -> windows_core::Result<()>;
    fn OperationID(&self) -> windows_core::Result<i32>;
    fn SetOperationID(&self, lprop: i32) -> windows_core::Result<()>;
    fn Writable(&self) -> windows_core::Result<windows_core::BOOL>;
    fn GetProperty(&self, lpropid: i32, varreserved: &super::super::System::Variant::VARIANT) -> windows_core::Result<super::super::System::Variant::VARIANT>;
    fn SetProperty(&self, lpropid: i32, varprop: &super::super::System::Variant::VARIANT, varreserved: &super::super::System::Variant::VARIANT) -> windows_core::Result<()>;
    fn Submit(&self, lflags: i32, varreserved: &super::super::System::Variant::VARIANT) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IAzOperation_Vtbl {
    pub const fn new<Identity: IAzOperation_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Name<Identity: IAzOperation_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrname: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IAzOperation_Impl::Name(this) {
                    Ok(ok__) => {
                        pbstrname.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetName<Identity: IAzOperation_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrname: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IAzOperation_Impl::SetName(this, core::mem::transmute(&bstrname)).into()
            }
        }
        unsafe extern "system" fn Description<Identity: IAzOperation_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrdescription: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IAzOperation_Impl::Description(this) {
                    Ok(ok__) => {
                        pbstrdescription.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetDescription<Identity: IAzOperation_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrdescription: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IAzOperation_Impl::SetDescription(this, core::mem::transmute(&bstrdescription)).into()
            }
        }
        unsafe extern "system" fn ApplicationData<Identity: IAzOperation_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrapplicationdata: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IAzOperation_Impl::ApplicationData(this) {
                    Ok(ok__) => {
                        pbstrapplicationdata.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetApplicationData<Identity: IAzOperation_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrapplicationdata: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IAzOperation_Impl::SetApplicationData(this, core::mem::transmute(&bstrapplicationdata)).into()
            }
        }
        unsafe extern "system" fn OperationID<Identity: IAzOperation_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, plprop: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IAzOperation_Impl::OperationID(this) {
                    Ok(ok__) => {
                        plprop.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetOperationID<Identity: IAzOperation_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lprop: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IAzOperation_Impl::SetOperationID(this, core::mem::transmute_copy(&lprop)).into()
            }
        }
        unsafe extern "system" fn Writable<Identity: IAzOperation_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pfprop: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IAzOperation_Impl::Writable(this) {
                    Ok(ok__) => {
                        pfprop.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetProperty<Identity: IAzOperation_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lpropid: i32, varreserved: super::super::System::Variant::VARIANT, pvarprop: *mut super::super::System::Variant::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IAzOperation_Impl::GetProperty(this, core::mem::transmute_copy(&lpropid), core::mem::transmute(&varreserved)) {
                    Ok(ok__) => {
                        pvarprop.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetProperty<Identity: IAzOperation_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lpropid: i32, varprop: super::super::System::Variant::VARIANT, varreserved: super::super::System::Variant::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IAzOperation_Impl::SetProperty(this, core::mem::transmute_copy(&lpropid), core::mem::transmute(&varprop), core::mem::transmute(&varreserved)).into()
            }
        }
        unsafe extern "system" fn Submit<Identity: IAzOperation_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lflags: i32, varreserved: super::super::System::Variant::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IAzOperation_Impl::Submit(this, core::mem::transmute_copy(&lflags), core::mem::transmute(&varreserved)).into()
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            Name: Name::<Identity, OFFSET>,
            SetName: SetName::<Identity, OFFSET>,
            Description: Description::<Identity, OFFSET>,
            SetDescription: SetDescription::<Identity, OFFSET>,
            ApplicationData: ApplicationData::<Identity, OFFSET>,
            SetApplicationData: SetApplicationData::<Identity, OFFSET>,
            OperationID: OperationID::<Identity, OFFSET>,
            SetOperationID: SetOperationID::<Identity, OFFSET>,
            Writable: Writable::<Identity, OFFSET>,
            GetProperty: GetProperty::<Identity, OFFSET>,
            SetProperty: SetProperty::<Identity, OFFSET>,
            Submit: Submit::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IAzOperation as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for IAzOperation {}
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
    pub unsafe fn RoleAssignments(&self, bstrscopename: &windows_core::BSTR, brecursive: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<IAzRoleAssignments> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).RoleAssignments)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrscopename), brecursive, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IAzOperation2_Vtbl {
    pub base__: IAzOperation_Vtbl,
    pub RoleAssignments: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, super::super::Foundation::VARIANT_BOOL, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IAzOperation2_Impl: IAzOperation_Impl {
    fn RoleAssignments(&self, bstrscopename: &windows_core::BSTR, brecursive: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<IAzRoleAssignments>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IAzOperation2_Vtbl {
    pub const fn new<Identity: IAzOperation2_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn RoleAssignments<Identity: IAzOperation2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrscopename: *mut core::ffi::c_void, brecursive: super::super::Foundation::VARIANT_BOOL, pproleassignments: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IAzOperation2_Impl::RoleAssignments(this, core::mem::transmute(&bstrscopename), core::mem::transmute_copy(&brecursive)) {
                    Ok(ok__) => {
                        pproleassignments.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self { base__: IAzOperation_Vtbl::new::<Identity, OFFSET>(), RoleAssignments: RoleAssignments::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IAzOperation2 as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID || iid == &<IAzOperation as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for IAzOperation2 {}
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
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn get_Item(&self, index: i32) -> windows_core::Result<super::super::System::Variant::VARIANT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_Item)(windows_core::Interface::as_raw(self), index, &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn Count(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Count)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn _NewEnum(&self) -> windows_core::Result<windows_core::IUnknown> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self)._NewEnum)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IAzOperations_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub get_Item: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut super::super::System::Variant::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    get_Item: usize,
    pub Count: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub _NewEnum: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IAzOperations_Impl: super::super::System::Com::IDispatch_Impl {
    fn get_Item(&self, index: i32) -> windows_core::Result<super::super::System::Variant::VARIANT>;
    fn Count(&self) -> windows_core::Result<i32>;
    fn _NewEnum(&self) -> windows_core::Result<windows_core::IUnknown>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IAzOperations_Vtbl {
    pub const fn new<Identity: IAzOperations_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn get_Item<Identity: IAzOperations_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: i32, pvarobtptr: *mut super::super::System::Variant::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IAzOperations_Impl::get_Item(this, core::mem::transmute_copy(&index)) {
                    Ok(ok__) => {
                        pvarobtptr.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Count<Identity: IAzOperations_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, plcount: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IAzOperations_Impl::Count(this) {
                    Ok(ok__) => {
                        plcount.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn _NewEnum<Identity: IAzOperations_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppenumptr: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IAzOperations_Impl::_NewEnum(this) {
                    Ok(ok__) => {
                        ppenumptr.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            get_Item: get_Item::<Identity, OFFSET>,
            Count: Count::<Identity, OFFSET>,
            _NewEnum: _NewEnum::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IAzOperations as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for IAzOperations {}
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
    pub unsafe fn NameResolver(&self) -> windows_core::Result<IAzNameResolver> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).NameResolver)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn ObjectPicker(&self) -> windows_core::Result<IAzObjectPicker> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ObjectPicker)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IAzPrincipalLocator_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub NameResolver: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ObjectPicker: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IAzPrincipalLocator_Impl: super::super::System::Com::IDispatch_Impl {
    fn NameResolver(&self) -> windows_core::Result<IAzNameResolver>;
    fn ObjectPicker(&self) -> windows_core::Result<IAzObjectPicker>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IAzPrincipalLocator_Vtbl {
    pub const fn new<Identity: IAzPrincipalLocator_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn NameResolver<Identity: IAzPrincipalLocator_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppnameresolver: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IAzPrincipalLocator_Impl::NameResolver(this) {
                    Ok(ok__) => {
                        ppnameresolver.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn ObjectPicker<Identity: IAzPrincipalLocator_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppobjectpicker: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IAzPrincipalLocator_Impl::ObjectPicker(this) {
                    Ok(ok__) => {
                        ppobjectpicker.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            NameResolver: NameResolver::<Identity, OFFSET>,
            ObjectPicker: ObjectPicker::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IAzPrincipalLocator as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for IAzPrincipalLocator {}
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
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Name)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn SetName(&self, bstrname: &windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetName)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrname)).ok() }
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
    pub unsafe fn ApplicationData(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ApplicationData)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn SetApplicationData(&self, bstrapplicationdata: &windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetApplicationData)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrapplicationdata)).ok() }
    }
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn AddAppMember(&self, bstrprop: &windows_core::BSTR, varreserved: &super::super::System::Variant::VARIANT) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).AddAppMember)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrprop), core::mem::transmute_copy(varreserved)).ok() }
    }
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn DeleteAppMember(&self, bstrprop: &windows_core::BSTR, varreserved: &super::super::System::Variant::VARIANT) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).DeleteAppMember)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrprop), core::mem::transmute_copy(varreserved)).ok() }
    }
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn AddTask(&self, bstrprop: &windows_core::BSTR, varreserved: &super::super::System::Variant::VARIANT) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).AddTask)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrprop), core::mem::transmute_copy(varreserved)).ok() }
    }
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn DeleteTask(&self, bstrprop: &windows_core::BSTR, varreserved: &super::super::System::Variant::VARIANT) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).DeleteTask)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrprop), core::mem::transmute_copy(varreserved)).ok() }
    }
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn AddOperation(&self, bstrprop: &windows_core::BSTR, varreserved: &super::super::System::Variant::VARIANT) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).AddOperation)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrprop), core::mem::transmute_copy(varreserved)).ok() }
    }
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn DeleteOperation(&self, bstrprop: &windows_core::BSTR, varreserved: &super::super::System::Variant::VARIANT) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).DeleteOperation)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrprop), core::mem::transmute_copy(varreserved)).ok() }
    }
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn AddMember(&self, bstrprop: &windows_core::BSTR, varreserved: &super::super::System::Variant::VARIANT) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).AddMember)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrprop), core::mem::transmute_copy(varreserved)).ok() }
    }
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn DeleteMember(&self, bstrprop: &windows_core::BSTR, varreserved: &super::super::System::Variant::VARIANT) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).DeleteMember)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrprop), core::mem::transmute_copy(varreserved)).ok() }
    }
    pub unsafe fn Writable(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Writable)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn GetProperty(&self, lpropid: i32, varreserved: &super::super::System::Variant::VARIANT) -> windows_core::Result<super::super::System::Variant::VARIANT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetProperty)(windows_core::Interface::as_raw(self), lpropid, core::mem::transmute_copy(varreserved), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn SetProperty(&self, lpropid: i32, varprop: &super::super::System::Variant::VARIANT, varreserved: &super::super::System::Variant::VARIANT) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetProperty)(windows_core::Interface::as_raw(self), lpropid, core::mem::transmute_copy(varprop), core::mem::transmute_copy(varreserved)).ok() }
    }
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn AppMembers(&self) -> windows_core::Result<super::super::System::Variant::VARIANT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).AppMembers)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn Members(&self) -> windows_core::Result<super::super::System::Variant::VARIANT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Members)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn Operations(&self) -> windows_core::Result<super::super::System::Variant::VARIANT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Operations)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn Tasks(&self) -> windows_core::Result<super::super::System::Variant::VARIANT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Tasks)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn AddPropertyItem(&self, lpropid: i32, varprop: &super::super::System::Variant::VARIANT, varreserved: &super::super::System::Variant::VARIANT) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).AddPropertyItem)(windows_core::Interface::as_raw(self), lpropid, core::mem::transmute_copy(varprop), core::mem::transmute_copy(varreserved)).ok() }
    }
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn DeletePropertyItem(&self, lpropid: i32, varprop: &super::super::System::Variant::VARIANT, varreserved: &super::super::System::Variant::VARIANT) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).DeletePropertyItem)(windows_core::Interface::as_raw(self), lpropid, core::mem::transmute_copy(varprop), core::mem::transmute_copy(varreserved)).ok() }
    }
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn Submit(&self, lflags: i32, varreserved: &super::super::System::Variant::VARIANT) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Submit)(windows_core::Interface::as_raw(self), lflags, core::mem::transmute_copy(varreserved)).ok() }
    }
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn AddMemberName(&self, bstrprop: &windows_core::BSTR, varreserved: &super::super::System::Variant::VARIANT) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).AddMemberName)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrprop), core::mem::transmute_copy(varreserved)).ok() }
    }
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn DeleteMemberName(&self, bstrprop: &windows_core::BSTR, varreserved: &super::super::System::Variant::VARIANT) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).DeleteMemberName)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrprop), core::mem::transmute_copy(varreserved)).ok() }
    }
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn MembersName(&self) -> windows_core::Result<super::super::System::Variant::VARIANT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).MembersName)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IAzRole_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub Name: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Description: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetDescription: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ApplicationData: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetApplicationData: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub AddAppMember: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, super::super::System::Variant::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    AddAppMember: usize,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub DeleteAppMember: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, super::super::System::Variant::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    DeleteAppMember: usize,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub AddTask: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, super::super::System::Variant::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    AddTask: usize,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub DeleteTask: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, super::super::System::Variant::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    DeleteTask: usize,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub AddOperation: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, super::super::System::Variant::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    AddOperation: usize,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub DeleteOperation: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, super::super::System::Variant::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    DeleteOperation: usize,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub AddMember: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, super::super::System::Variant::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    AddMember: usize,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub DeleteMember: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, super::super::System::Variant::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    DeleteMember: usize,
    pub Writable: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::BOOL) -> windows_core::HRESULT,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub GetProperty: unsafe extern "system" fn(*mut core::ffi::c_void, i32, super::super::System::Variant::VARIANT, *mut super::super::System::Variant::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    GetProperty: usize,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub SetProperty: unsafe extern "system" fn(*mut core::ffi::c_void, i32, super::super::System::Variant::VARIANT, super::super::System::Variant::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    SetProperty: usize,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub AppMembers: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::System::Variant::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    AppMembers: usize,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub Members: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::System::Variant::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    Members: usize,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub Operations: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::System::Variant::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    Operations: usize,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub Tasks: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::System::Variant::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    Tasks: usize,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub AddPropertyItem: unsafe extern "system" fn(*mut core::ffi::c_void, i32, super::super::System::Variant::VARIANT, super::super::System::Variant::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    AddPropertyItem: usize,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub DeletePropertyItem: unsafe extern "system" fn(*mut core::ffi::c_void, i32, super::super::System::Variant::VARIANT, super::super::System::Variant::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    DeletePropertyItem: usize,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub Submit: unsafe extern "system" fn(*mut core::ffi::c_void, i32, super::super::System::Variant::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    Submit: usize,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub AddMemberName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, super::super::System::Variant::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    AddMemberName: usize,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub DeleteMemberName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, super::super::System::Variant::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    DeleteMemberName: usize,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub MembersName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::System::Variant::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    MembersName: usize,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IAzRole_Impl: super::super::System::Com::IDispatch_Impl {
    fn Name(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetName(&self, bstrname: &windows_core::BSTR) -> windows_core::Result<()>;
    fn Description(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetDescription(&self, bstrdescription: &windows_core::BSTR) -> windows_core::Result<()>;
    fn ApplicationData(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetApplicationData(&self, bstrapplicationdata: &windows_core::BSTR) -> windows_core::Result<()>;
    fn AddAppMember(&self, bstrprop: &windows_core::BSTR, varreserved: &super::super::System::Variant::VARIANT) -> windows_core::Result<()>;
    fn DeleteAppMember(&self, bstrprop: &windows_core::BSTR, varreserved: &super::super::System::Variant::VARIANT) -> windows_core::Result<()>;
    fn AddTask(&self, bstrprop: &windows_core::BSTR, varreserved: &super::super::System::Variant::VARIANT) -> windows_core::Result<()>;
    fn DeleteTask(&self, bstrprop: &windows_core::BSTR, varreserved: &super::super::System::Variant::VARIANT) -> windows_core::Result<()>;
    fn AddOperation(&self, bstrprop: &windows_core::BSTR, varreserved: &super::super::System::Variant::VARIANT) -> windows_core::Result<()>;
    fn DeleteOperation(&self, bstrprop: &windows_core::BSTR, varreserved: &super::super::System::Variant::VARIANT) -> windows_core::Result<()>;
    fn AddMember(&self, bstrprop: &windows_core::BSTR, varreserved: &super::super::System::Variant::VARIANT) -> windows_core::Result<()>;
    fn DeleteMember(&self, bstrprop: &windows_core::BSTR, varreserved: &super::super::System::Variant::VARIANT) -> windows_core::Result<()>;
    fn Writable(&self) -> windows_core::Result<windows_core::BOOL>;
    fn GetProperty(&self, lpropid: i32, varreserved: &super::super::System::Variant::VARIANT) -> windows_core::Result<super::super::System::Variant::VARIANT>;
    fn SetProperty(&self, lpropid: i32, varprop: &super::super::System::Variant::VARIANT, varreserved: &super::super::System::Variant::VARIANT) -> windows_core::Result<()>;
    fn AppMembers(&self) -> windows_core::Result<super::super::System::Variant::VARIANT>;
    fn Members(&self) -> windows_core::Result<super::super::System::Variant::VARIANT>;
    fn Operations(&self) -> windows_core::Result<super::super::System::Variant::VARIANT>;
    fn Tasks(&self) -> windows_core::Result<super::super::System::Variant::VARIANT>;
    fn AddPropertyItem(&self, lpropid: i32, varprop: &super::super::System::Variant::VARIANT, varreserved: &super::super::System::Variant::VARIANT) -> windows_core::Result<()>;
    fn DeletePropertyItem(&self, lpropid: i32, varprop: &super::super::System::Variant::VARIANT, varreserved: &super::super::System::Variant::VARIANT) -> windows_core::Result<()>;
    fn Submit(&self, lflags: i32, varreserved: &super::super::System::Variant::VARIANT) -> windows_core::Result<()>;
    fn AddMemberName(&self, bstrprop: &windows_core::BSTR, varreserved: &super::super::System::Variant::VARIANT) -> windows_core::Result<()>;
    fn DeleteMemberName(&self, bstrprop: &windows_core::BSTR, varreserved: &super::super::System::Variant::VARIANT) -> windows_core::Result<()>;
    fn MembersName(&self) -> windows_core::Result<super::super::System::Variant::VARIANT>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IAzRole_Vtbl {
    pub const fn new<Identity: IAzRole_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Name<Identity: IAzRole_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrname: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IAzRole_Impl::Name(this) {
                    Ok(ok__) => {
                        pbstrname.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetName<Identity: IAzRole_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrname: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IAzRole_Impl::SetName(this, core::mem::transmute(&bstrname)).into()
            }
        }
        unsafe extern "system" fn Description<Identity: IAzRole_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrdescription: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IAzRole_Impl::Description(this) {
                    Ok(ok__) => {
                        pbstrdescription.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetDescription<Identity: IAzRole_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrdescription: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IAzRole_Impl::SetDescription(this, core::mem::transmute(&bstrdescription)).into()
            }
        }
        unsafe extern "system" fn ApplicationData<Identity: IAzRole_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrapplicationdata: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IAzRole_Impl::ApplicationData(this) {
                    Ok(ok__) => {
                        pbstrapplicationdata.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetApplicationData<Identity: IAzRole_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrapplicationdata: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IAzRole_Impl::SetApplicationData(this, core::mem::transmute(&bstrapplicationdata)).into()
            }
        }
        unsafe extern "system" fn AddAppMember<Identity: IAzRole_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrprop: *mut core::ffi::c_void, varreserved: super::super::System::Variant::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IAzRole_Impl::AddAppMember(this, core::mem::transmute(&bstrprop), core::mem::transmute(&varreserved)).into()
            }
        }
        unsafe extern "system" fn DeleteAppMember<Identity: IAzRole_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrprop: *mut core::ffi::c_void, varreserved: super::super::System::Variant::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IAzRole_Impl::DeleteAppMember(this, core::mem::transmute(&bstrprop), core::mem::transmute(&varreserved)).into()
            }
        }
        unsafe extern "system" fn AddTask<Identity: IAzRole_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrprop: *mut core::ffi::c_void, varreserved: super::super::System::Variant::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IAzRole_Impl::AddTask(this, core::mem::transmute(&bstrprop), core::mem::transmute(&varreserved)).into()
            }
        }
        unsafe extern "system" fn DeleteTask<Identity: IAzRole_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrprop: *mut core::ffi::c_void, varreserved: super::super::System::Variant::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IAzRole_Impl::DeleteTask(this, core::mem::transmute(&bstrprop), core::mem::transmute(&varreserved)).into()
            }
        }
        unsafe extern "system" fn AddOperation<Identity: IAzRole_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrprop: *mut core::ffi::c_void, varreserved: super::super::System::Variant::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IAzRole_Impl::AddOperation(this, core::mem::transmute(&bstrprop), core::mem::transmute(&varreserved)).into()
            }
        }
        unsafe extern "system" fn DeleteOperation<Identity: IAzRole_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrprop: *mut core::ffi::c_void, varreserved: super::super::System::Variant::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IAzRole_Impl::DeleteOperation(this, core::mem::transmute(&bstrprop), core::mem::transmute(&varreserved)).into()
            }
        }
        unsafe extern "system" fn AddMember<Identity: IAzRole_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrprop: *mut core::ffi::c_void, varreserved: super::super::System::Variant::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IAzRole_Impl::AddMember(this, core::mem::transmute(&bstrprop), core::mem::transmute(&varreserved)).into()
            }
        }
        unsafe extern "system" fn DeleteMember<Identity: IAzRole_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrprop: *mut core::ffi::c_void, varreserved: super::super::System::Variant::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IAzRole_Impl::DeleteMember(this, core::mem::transmute(&bstrprop), core::mem::transmute(&varreserved)).into()
            }
        }
        unsafe extern "system" fn Writable<Identity: IAzRole_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pfprop: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IAzRole_Impl::Writable(this) {
                    Ok(ok__) => {
                        pfprop.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetProperty<Identity: IAzRole_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lpropid: i32, varreserved: super::super::System::Variant::VARIANT, pvarprop: *mut super::super::System::Variant::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IAzRole_Impl::GetProperty(this, core::mem::transmute_copy(&lpropid), core::mem::transmute(&varreserved)) {
                    Ok(ok__) => {
                        pvarprop.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetProperty<Identity: IAzRole_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lpropid: i32, varprop: super::super::System::Variant::VARIANT, varreserved: super::super::System::Variant::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IAzRole_Impl::SetProperty(this, core::mem::transmute_copy(&lpropid), core::mem::transmute(&varprop), core::mem::transmute(&varreserved)).into()
            }
        }
        unsafe extern "system" fn AppMembers<Identity: IAzRole_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvarprop: *mut super::super::System::Variant::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IAzRole_Impl::AppMembers(this) {
                    Ok(ok__) => {
                        pvarprop.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Members<Identity: IAzRole_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvarprop: *mut super::super::System::Variant::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IAzRole_Impl::Members(this) {
                    Ok(ok__) => {
                        pvarprop.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Operations<Identity: IAzRole_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvarprop: *mut super::super::System::Variant::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IAzRole_Impl::Operations(this) {
                    Ok(ok__) => {
                        pvarprop.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Tasks<Identity: IAzRole_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvarprop: *mut super::super::System::Variant::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IAzRole_Impl::Tasks(this) {
                    Ok(ok__) => {
                        pvarprop.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn AddPropertyItem<Identity: IAzRole_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lpropid: i32, varprop: super::super::System::Variant::VARIANT, varreserved: super::super::System::Variant::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IAzRole_Impl::AddPropertyItem(this, core::mem::transmute_copy(&lpropid), core::mem::transmute(&varprop), core::mem::transmute(&varreserved)).into()
            }
        }
        unsafe extern "system" fn DeletePropertyItem<Identity: IAzRole_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lpropid: i32, varprop: super::super::System::Variant::VARIANT, varreserved: super::super::System::Variant::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IAzRole_Impl::DeletePropertyItem(this, core::mem::transmute_copy(&lpropid), core::mem::transmute(&varprop), core::mem::transmute(&varreserved)).into()
            }
        }
        unsafe extern "system" fn Submit<Identity: IAzRole_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lflags: i32, varreserved: super::super::System::Variant::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IAzRole_Impl::Submit(this, core::mem::transmute_copy(&lflags), core::mem::transmute(&varreserved)).into()
            }
        }
        unsafe extern "system" fn AddMemberName<Identity: IAzRole_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrprop: *mut core::ffi::c_void, varreserved: super::super::System::Variant::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IAzRole_Impl::AddMemberName(this, core::mem::transmute(&bstrprop), core::mem::transmute(&varreserved)).into()
            }
        }
        unsafe extern "system" fn DeleteMemberName<Identity: IAzRole_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrprop: *mut core::ffi::c_void, varreserved: super::super::System::Variant::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IAzRole_Impl::DeleteMemberName(this, core::mem::transmute(&bstrprop), core::mem::transmute(&varreserved)).into()
            }
        }
        unsafe extern "system" fn MembersName<Identity: IAzRole_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvarprop: *mut super::super::System::Variant::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IAzRole_Impl::MembersName(this) {
                    Ok(ok__) => {
                        pvarprop.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            Name: Name::<Identity, OFFSET>,
            SetName: SetName::<Identity, OFFSET>,
            Description: Description::<Identity, OFFSET>,
            SetDescription: SetDescription::<Identity, OFFSET>,
            ApplicationData: ApplicationData::<Identity, OFFSET>,
            SetApplicationData: SetApplicationData::<Identity, OFFSET>,
            AddAppMember: AddAppMember::<Identity, OFFSET>,
            DeleteAppMember: DeleteAppMember::<Identity, OFFSET>,
            AddTask: AddTask::<Identity, OFFSET>,
            DeleteTask: DeleteTask::<Identity, OFFSET>,
            AddOperation: AddOperation::<Identity, OFFSET>,
            DeleteOperation: DeleteOperation::<Identity, OFFSET>,
            AddMember: AddMember::<Identity, OFFSET>,
            DeleteMember: DeleteMember::<Identity, OFFSET>,
            Writable: Writable::<Identity, OFFSET>,
            GetProperty: GetProperty::<Identity, OFFSET>,
            SetProperty: SetProperty::<Identity, OFFSET>,
            AppMembers: AppMembers::<Identity, OFFSET>,
            Members: Members::<Identity, OFFSET>,
            Operations: Operations::<Identity, OFFSET>,
            Tasks: Tasks::<Identity, OFFSET>,
            AddPropertyItem: AddPropertyItem::<Identity, OFFSET>,
            DeletePropertyItem: DeletePropertyItem::<Identity, OFFSET>,
            Submit: Submit::<Identity, OFFSET>,
            AddMemberName: AddMemberName::<Identity, OFFSET>,
            DeleteMemberName: DeleteMemberName::<Identity, OFFSET>,
            MembersName: MembersName::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IAzRole as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for IAzRole {}
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
    pub unsafe fn AddRoleDefinition(&self, bstrroledefinition: &windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).AddRoleDefinition)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrroledefinition)).ok() }
    }
    pub unsafe fn DeleteRoleDefinition(&self, bstrroledefinition: &windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).DeleteRoleDefinition)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrroledefinition)).ok() }
    }
    pub unsafe fn RoleDefinitions(&self) -> windows_core::Result<IAzRoleDefinitions> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).RoleDefinitions)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn Scope(&self) -> windows_core::Result<IAzScope> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Scope)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IAzRoleAssignment_Vtbl {
    pub base__: IAzRole_Vtbl,
    pub AddRoleDefinition: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub DeleteRoleDefinition: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub RoleDefinitions: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Scope: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IAzRoleAssignment_Impl: IAzRole_Impl {
    fn AddRoleDefinition(&self, bstrroledefinition: &windows_core::BSTR) -> windows_core::Result<()>;
    fn DeleteRoleDefinition(&self, bstrroledefinition: &windows_core::BSTR) -> windows_core::Result<()>;
    fn RoleDefinitions(&self) -> windows_core::Result<IAzRoleDefinitions>;
    fn Scope(&self) -> windows_core::Result<IAzScope>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IAzRoleAssignment_Vtbl {
    pub const fn new<Identity: IAzRoleAssignment_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn AddRoleDefinition<Identity: IAzRoleAssignment_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrroledefinition: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IAzRoleAssignment_Impl::AddRoleDefinition(this, core::mem::transmute(&bstrroledefinition)).into()
            }
        }
        unsafe extern "system" fn DeleteRoleDefinition<Identity: IAzRoleAssignment_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrroledefinition: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IAzRoleAssignment_Impl::DeleteRoleDefinition(this, core::mem::transmute(&bstrroledefinition)).into()
            }
        }
        unsafe extern "system" fn RoleDefinitions<Identity: IAzRoleAssignment_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pproledefinitions: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IAzRoleAssignment_Impl::RoleDefinitions(this) {
                    Ok(ok__) => {
                        pproledefinitions.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Scope<Identity: IAzRoleAssignment_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppscope: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IAzRoleAssignment_Impl::Scope(this) {
                    Ok(ok__) => {
                        ppscope.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: IAzRole_Vtbl::new::<Identity, OFFSET>(),
            AddRoleDefinition: AddRoleDefinition::<Identity, OFFSET>,
            DeleteRoleDefinition: DeleteRoleDefinition::<Identity, OFFSET>,
            RoleDefinitions: RoleDefinitions::<Identity, OFFSET>,
            Scope: Scope::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IAzRoleAssignment as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID || iid == &<IAzRole as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for IAzRoleAssignment {}
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
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn get_Item(&self, index: i32) -> windows_core::Result<super::super::System::Variant::VARIANT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_Item)(windows_core::Interface::as_raw(self), index, &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn Count(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Count)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn _NewEnum(&self) -> windows_core::Result<windows_core::IUnknown> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self)._NewEnum)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IAzRoleAssignments_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub get_Item: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut super::super::System::Variant::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    get_Item: usize,
    pub Count: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub _NewEnum: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IAzRoleAssignments_Impl: super::super::System::Com::IDispatch_Impl {
    fn get_Item(&self, index: i32) -> windows_core::Result<super::super::System::Variant::VARIANT>;
    fn Count(&self) -> windows_core::Result<i32>;
    fn _NewEnum(&self) -> windows_core::Result<windows_core::IUnknown>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IAzRoleAssignments_Vtbl {
    pub const fn new<Identity: IAzRoleAssignments_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn get_Item<Identity: IAzRoleAssignments_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: i32, pvarobtptr: *mut super::super::System::Variant::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IAzRoleAssignments_Impl::get_Item(this, core::mem::transmute_copy(&index)) {
                    Ok(ok__) => {
                        pvarobtptr.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Count<Identity: IAzRoleAssignments_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, plcount: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IAzRoleAssignments_Impl::Count(this) {
                    Ok(ok__) => {
                        plcount.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn _NewEnum<Identity: IAzRoleAssignments_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppenumptr: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IAzRoleAssignments_Impl::_NewEnum(this) {
                    Ok(ok__) => {
                        ppenumptr.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            get_Item: get_Item::<Identity, OFFSET>,
            Count: Count::<Identity, OFFSET>,
            _NewEnum: _NewEnum::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IAzRoleAssignments as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for IAzRoleAssignments {}
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
    pub unsafe fn RoleAssignments(&self, bstrscopename: &windows_core::BSTR, brecursive: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<IAzRoleAssignments> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).RoleAssignments)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrscopename), brecursive, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn AddRoleDefinition(&self, bstrroledefinition: &windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).AddRoleDefinition)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrroledefinition)).ok() }
    }
    pub unsafe fn DeleteRoleDefinition(&self, bstrroledefinition: &windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).DeleteRoleDefinition)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrroledefinition)).ok() }
    }
    pub unsafe fn RoleDefinitions(&self) -> windows_core::Result<IAzRoleDefinitions> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).RoleDefinitions)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IAzRoleDefinition_Vtbl {
    pub base__: IAzTask_Vtbl,
    pub RoleAssignments: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, super::super::Foundation::VARIANT_BOOL, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub AddRoleDefinition: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub DeleteRoleDefinition: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub RoleDefinitions: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IAzRoleDefinition_Impl: IAzTask_Impl {
    fn RoleAssignments(&self, bstrscopename: &windows_core::BSTR, brecursive: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<IAzRoleAssignments>;
    fn AddRoleDefinition(&self, bstrroledefinition: &windows_core::BSTR) -> windows_core::Result<()>;
    fn DeleteRoleDefinition(&self, bstrroledefinition: &windows_core::BSTR) -> windows_core::Result<()>;
    fn RoleDefinitions(&self) -> windows_core::Result<IAzRoleDefinitions>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IAzRoleDefinition_Vtbl {
    pub const fn new<Identity: IAzRoleDefinition_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn RoleAssignments<Identity: IAzRoleDefinition_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrscopename: *mut core::ffi::c_void, brecursive: super::super::Foundation::VARIANT_BOOL, pproleassignments: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IAzRoleDefinition_Impl::RoleAssignments(this, core::mem::transmute(&bstrscopename), core::mem::transmute_copy(&brecursive)) {
                    Ok(ok__) => {
                        pproleassignments.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn AddRoleDefinition<Identity: IAzRoleDefinition_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrroledefinition: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IAzRoleDefinition_Impl::AddRoleDefinition(this, core::mem::transmute(&bstrroledefinition)).into()
            }
        }
        unsafe extern "system" fn DeleteRoleDefinition<Identity: IAzRoleDefinition_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrroledefinition: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IAzRoleDefinition_Impl::DeleteRoleDefinition(this, core::mem::transmute(&bstrroledefinition)).into()
            }
        }
        unsafe extern "system" fn RoleDefinitions<Identity: IAzRoleDefinition_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pproledefinitions: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IAzRoleDefinition_Impl::RoleDefinitions(this) {
                    Ok(ok__) => {
                        pproledefinitions.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: IAzTask_Vtbl::new::<Identity, OFFSET>(),
            RoleAssignments: RoleAssignments::<Identity, OFFSET>,
            AddRoleDefinition: AddRoleDefinition::<Identity, OFFSET>,
            DeleteRoleDefinition: DeleteRoleDefinition::<Identity, OFFSET>,
            RoleDefinitions: RoleDefinitions::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IAzRoleDefinition as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID || iid == &<IAzTask as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for IAzRoleDefinition {}
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
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn get_Item(&self, index: i32) -> windows_core::Result<super::super::System::Variant::VARIANT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_Item)(windows_core::Interface::as_raw(self), index, &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn Count(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Count)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn _NewEnum(&self) -> windows_core::Result<windows_core::IUnknown> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self)._NewEnum)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IAzRoleDefinitions_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub get_Item: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut super::super::System::Variant::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    get_Item: usize,
    pub Count: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub _NewEnum: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IAzRoleDefinitions_Impl: super::super::System::Com::IDispatch_Impl {
    fn get_Item(&self, index: i32) -> windows_core::Result<super::super::System::Variant::VARIANT>;
    fn Count(&self) -> windows_core::Result<i32>;
    fn _NewEnum(&self) -> windows_core::Result<windows_core::IUnknown>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IAzRoleDefinitions_Vtbl {
    pub const fn new<Identity: IAzRoleDefinitions_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn get_Item<Identity: IAzRoleDefinitions_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: i32, pvarobtptr: *mut super::super::System::Variant::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IAzRoleDefinitions_Impl::get_Item(this, core::mem::transmute_copy(&index)) {
                    Ok(ok__) => {
                        pvarobtptr.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Count<Identity: IAzRoleDefinitions_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, plcount: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IAzRoleDefinitions_Impl::Count(this) {
                    Ok(ok__) => {
                        plcount.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn _NewEnum<Identity: IAzRoleDefinitions_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppenumptr: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IAzRoleDefinitions_Impl::_NewEnum(this) {
                    Ok(ok__) => {
                        ppenumptr.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            get_Item: get_Item::<Identity, OFFSET>,
            Count: Count::<Identity, OFFSET>,
            _NewEnum: _NewEnum::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IAzRoleDefinitions as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for IAzRoleDefinitions {}
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
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn get_Item(&self, index: i32) -> windows_core::Result<super::super::System::Variant::VARIANT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_Item)(windows_core::Interface::as_raw(self), index, &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn Count(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Count)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn _NewEnum(&self) -> windows_core::Result<windows_core::IUnknown> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self)._NewEnum)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IAzRoles_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub get_Item: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut super::super::System::Variant::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    get_Item: usize,
    pub Count: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub _NewEnum: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IAzRoles_Impl: super::super::System::Com::IDispatch_Impl {
    fn get_Item(&self, index: i32) -> windows_core::Result<super::super::System::Variant::VARIANT>;
    fn Count(&self) -> windows_core::Result<i32>;
    fn _NewEnum(&self) -> windows_core::Result<windows_core::IUnknown>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IAzRoles_Vtbl {
    pub const fn new<Identity: IAzRoles_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn get_Item<Identity: IAzRoles_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: i32, pvarobtptr: *mut super::super::System::Variant::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IAzRoles_Impl::get_Item(this, core::mem::transmute_copy(&index)) {
                    Ok(ok__) => {
                        pvarobtptr.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Count<Identity: IAzRoles_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, plcount: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IAzRoles_Impl::Count(this) {
                    Ok(ok__) => {
                        plcount.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn _NewEnum<Identity: IAzRoles_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppenumptr: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IAzRoles_Impl::_NewEnum(this) {
                    Ok(ok__) => {
                        ppenumptr.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            get_Item: get_Item::<Identity, OFFSET>,
            Count: Count::<Identity, OFFSET>,
            _NewEnum: _NewEnum::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IAzRoles as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for IAzRoles {}
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
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Name)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn SetName(&self, bstrname: &windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetName)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrname)).ok() }
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
    pub unsafe fn ApplicationData(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ApplicationData)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn SetApplicationData(&self, bstrapplicationdata: &windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetApplicationData)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrapplicationdata)).ok() }
    }
    pub unsafe fn Writable(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Writable)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn GetProperty(&self, lpropid: i32, varreserved: &super::super::System::Variant::VARIANT) -> windows_core::Result<super::super::System::Variant::VARIANT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetProperty)(windows_core::Interface::as_raw(self), lpropid, core::mem::transmute_copy(varreserved), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn SetProperty(&self, lpropid: i32, varprop: &super::super::System::Variant::VARIANT, varreserved: &super::super::System::Variant::VARIANT) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetProperty)(windows_core::Interface::as_raw(self), lpropid, core::mem::transmute_copy(varprop), core::mem::transmute_copy(varreserved)).ok() }
    }
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn AddPropertyItem(&self, lpropid: i32, varprop: &super::super::System::Variant::VARIANT, varreserved: &super::super::System::Variant::VARIANT) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).AddPropertyItem)(windows_core::Interface::as_raw(self), lpropid, core::mem::transmute_copy(varprop), core::mem::transmute_copy(varreserved)).ok() }
    }
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn DeletePropertyItem(&self, lpropid: i32, varprop: &super::super::System::Variant::VARIANT, varreserved: &super::super::System::Variant::VARIANT) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).DeletePropertyItem)(windows_core::Interface::as_raw(self), lpropid, core::mem::transmute_copy(varprop), core::mem::transmute_copy(varreserved)).ok() }
    }
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn PolicyAdministrators(&self) -> windows_core::Result<super::super::System::Variant::VARIANT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).PolicyAdministrators)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn PolicyReaders(&self) -> windows_core::Result<super::super::System::Variant::VARIANT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).PolicyReaders)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn AddPolicyAdministrator(&self, bstradmin: &windows_core::BSTR, varreserved: &super::super::System::Variant::VARIANT) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).AddPolicyAdministrator)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstradmin), core::mem::transmute_copy(varreserved)).ok() }
    }
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn DeletePolicyAdministrator(&self, bstradmin: &windows_core::BSTR, varreserved: &super::super::System::Variant::VARIANT) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).DeletePolicyAdministrator)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstradmin), core::mem::transmute_copy(varreserved)).ok() }
    }
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn AddPolicyReader(&self, bstrreader: &windows_core::BSTR, varreserved: &super::super::System::Variant::VARIANT) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).AddPolicyReader)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrreader), core::mem::transmute_copy(varreserved)).ok() }
    }
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn DeletePolicyReader(&self, bstrreader: &windows_core::BSTR, varreserved: &super::super::System::Variant::VARIANT) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).DeletePolicyReader)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrreader), core::mem::transmute_copy(varreserved)).ok() }
    }
    pub unsafe fn ApplicationGroups(&self) -> windows_core::Result<IAzApplicationGroups> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ApplicationGroups)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn OpenApplicationGroup(&self, bstrgroupname: &windows_core::BSTR, varreserved: &super::super::System::Variant::VARIANT) -> windows_core::Result<IAzApplicationGroup> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).OpenApplicationGroup)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrgroupname), core::mem::transmute_copy(varreserved), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn CreateApplicationGroup(&self, bstrgroupname: &windows_core::BSTR, varreserved: &super::super::System::Variant::VARIANT) -> windows_core::Result<IAzApplicationGroup> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateApplicationGroup)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrgroupname), core::mem::transmute_copy(varreserved), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn DeleteApplicationGroup(&self, bstrgroupname: &windows_core::BSTR, varreserved: &super::super::System::Variant::VARIANT) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).DeleteApplicationGroup)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrgroupname), core::mem::transmute_copy(varreserved)).ok() }
    }
    pub unsafe fn Roles(&self) -> windows_core::Result<IAzRoles> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Roles)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn OpenRole(&self, bstrrolename: &windows_core::BSTR, varreserved: &super::super::System::Variant::VARIANT) -> windows_core::Result<IAzRole> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).OpenRole)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrrolename), core::mem::transmute_copy(varreserved), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn CreateRole(&self, bstrrolename: &windows_core::BSTR, varreserved: &super::super::System::Variant::VARIANT) -> windows_core::Result<IAzRole> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateRole)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrrolename), core::mem::transmute_copy(varreserved), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn DeleteRole(&self, bstrrolename: &windows_core::BSTR, varreserved: &super::super::System::Variant::VARIANT) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).DeleteRole)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrrolename), core::mem::transmute_copy(varreserved)).ok() }
    }
    pub unsafe fn Tasks(&self) -> windows_core::Result<IAzTasks> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Tasks)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn OpenTask(&self, bstrtaskname: &windows_core::BSTR, varreserved: &super::super::System::Variant::VARIANT) -> windows_core::Result<IAzTask> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).OpenTask)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrtaskname), core::mem::transmute_copy(varreserved), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn CreateTask(&self, bstrtaskname: &windows_core::BSTR, varreserved: &super::super::System::Variant::VARIANT) -> windows_core::Result<IAzTask> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateTask)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrtaskname), core::mem::transmute_copy(varreserved), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn DeleteTask(&self, bstrtaskname: &windows_core::BSTR, varreserved: &super::super::System::Variant::VARIANT) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).DeleteTask)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrtaskname), core::mem::transmute_copy(varreserved)).ok() }
    }
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn Submit(&self, lflags: i32, varreserved: &super::super::System::Variant::VARIANT) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Submit)(windows_core::Interface::as_raw(self), lflags, core::mem::transmute_copy(varreserved)).ok() }
    }
    pub unsafe fn CanBeDelegated(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CanBeDelegated)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn BizrulesWritable(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).BizrulesWritable)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn PolicyAdministratorsName(&self) -> windows_core::Result<super::super::System::Variant::VARIANT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).PolicyAdministratorsName)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn PolicyReadersName(&self) -> windows_core::Result<super::super::System::Variant::VARIANT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).PolicyReadersName)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn AddPolicyAdministratorName(&self, bstradmin: &windows_core::BSTR, varreserved: &super::super::System::Variant::VARIANT) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).AddPolicyAdministratorName)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstradmin), core::mem::transmute_copy(varreserved)).ok() }
    }
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn DeletePolicyAdministratorName(&self, bstradmin: &windows_core::BSTR, varreserved: &super::super::System::Variant::VARIANT) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).DeletePolicyAdministratorName)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstradmin), core::mem::transmute_copy(varreserved)).ok() }
    }
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn AddPolicyReaderName(&self, bstrreader: &windows_core::BSTR, varreserved: &super::super::System::Variant::VARIANT) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).AddPolicyReaderName)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrreader), core::mem::transmute_copy(varreserved)).ok() }
    }
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn DeletePolicyReaderName(&self, bstrreader: &windows_core::BSTR, varreserved: &super::super::System::Variant::VARIANT) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).DeletePolicyReaderName)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrreader), core::mem::transmute_copy(varreserved)).ok() }
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IAzScope_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub Name: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Description: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetDescription: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ApplicationData: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetApplicationData: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Writable: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::BOOL) -> windows_core::HRESULT,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub GetProperty: unsafe extern "system" fn(*mut core::ffi::c_void, i32, super::super::System::Variant::VARIANT, *mut super::super::System::Variant::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    GetProperty: usize,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub SetProperty: unsafe extern "system" fn(*mut core::ffi::c_void, i32, super::super::System::Variant::VARIANT, super::super::System::Variant::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    SetProperty: usize,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub AddPropertyItem: unsafe extern "system" fn(*mut core::ffi::c_void, i32, super::super::System::Variant::VARIANT, super::super::System::Variant::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    AddPropertyItem: usize,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub DeletePropertyItem: unsafe extern "system" fn(*mut core::ffi::c_void, i32, super::super::System::Variant::VARIANT, super::super::System::Variant::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    DeletePropertyItem: usize,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub PolicyAdministrators: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::System::Variant::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    PolicyAdministrators: usize,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub PolicyReaders: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::System::Variant::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    PolicyReaders: usize,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub AddPolicyAdministrator: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, super::super::System::Variant::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    AddPolicyAdministrator: usize,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub DeletePolicyAdministrator: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, super::super::System::Variant::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    DeletePolicyAdministrator: usize,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub AddPolicyReader: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, super::super::System::Variant::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    AddPolicyReader: usize,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub DeletePolicyReader: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, super::super::System::Variant::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    DeletePolicyReader: usize,
    pub ApplicationGroups: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub OpenApplicationGroup: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, super::super::System::Variant::VARIANT, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    OpenApplicationGroup: usize,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub CreateApplicationGroup: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, super::super::System::Variant::VARIANT, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    CreateApplicationGroup: usize,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub DeleteApplicationGroup: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, super::super::System::Variant::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    DeleteApplicationGroup: usize,
    pub Roles: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub OpenRole: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, super::super::System::Variant::VARIANT, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    OpenRole: usize,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub CreateRole: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, super::super::System::Variant::VARIANT, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    CreateRole: usize,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub DeleteRole: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, super::super::System::Variant::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    DeleteRole: usize,
    pub Tasks: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub OpenTask: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, super::super::System::Variant::VARIANT, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    OpenTask: usize,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub CreateTask: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, super::super::System::Variant::VARIANT, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    CreateTask: usize,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub DeleteTask: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, super::super::System::Variant::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    DeleteTask: usize,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub Submit: unsafe extern "system" fn(*mut core::ffi::c_void, i32, super::super::System::Variant::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    Submit: usize,
    pub CanBeDelegated: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::BOOL) -> windows_core::HRESULT,
    pub BizrulesWritable: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::BOOL) -> windows_core::HRESULT,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub PolicyAdministratorsName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::System::Variant::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    PolicyAdministratorsName: usize,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub PolicyReadersName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::System::Variant::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    PolicyReadersName: usize,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub AddPolicyAdministratorName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, super::super::System::Variant::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    AddPolicyAdministratorName: usize,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub DeletePolicyAdministratorName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, super::super::System::Variant::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    DeletePolicyAdministratorName: usize,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub AddPolicyReaderName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, super::super::System::Variant::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    AddPolicyReaderName: usize,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub DeletePolicyReaderName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, super::super::System::Variant::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    DeletePolicyReaderName: usize,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IAzScope_Impl: super::super::System::Com::IDispatch_Impl {
    fn Name(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetName(&self, bstrname: &windows_core::BSTR) -> windows_core::Result<()>;
    fn Description(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetDescription(&self, bstrdescription: &windows_core::BSTR) -> windows_core::Result<()>;
    fn ApplicationData(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetApplicationData(&self, bstrapplicationdata: &windows_core::BSTR) -> windows_core::Result<()>;
    fn Writable(&self) -> windows_core::Result<windows_core::BOOL>;
    fn GetProperty(&self, lpropid: i32, varreserved: &super::super::System::Variant::VARIANT) -> windows_core::Result<super::super::System::Variant::VARIANT>;
    fn SetProperty(&self, lpropid: i32, varprop: &super::super::System::Variant::VARIANT, varreserved: &super::super::System::Variant::VARIANT) -> windows_core::Result<()>;
    fn AddPropertyItem(&self, lpropid: i32, varprop: &super::super::System::Variant::VARIANT, varreserved: &super::super::System::Variant::VARIANT) -> windows_core::Result<()>;
    fn DeletePropertyItem(&self, lpropid: i32, varprop: &super::super::System::Variant::VARIANT, varreserved: &super::super::System::Variant::VARIANT) -> windows_core::Result<()>;
    fn PolicyAdministrators(&self) -> windows_core::Result<super::super::System::Variant::VARIANT>;
    fn PolicyReaders(&self) -> windows_core::Result<super::super::System::Variant::VARIANT>;
    fn AddPolicyAdministrator(&self, bstradmin: &windows_core::BSTR, varreserved: &super::super::System::Variant::VARIANT) -> windows_core::Result<()>;
    fn DeletePolicyAdministrator(&self, bstradmin: &windows_core::BSTR, varreserved: &super::super::System::Variant::VARIANT) -> windows_core::Result<()>;
    fn AddPolicyReader(&self, bstrreader: &windows_core::BSTR, varreserved: &super::super::System::Variant::VARIANT) -> windows_core::Result<()>;
    fn DeletePolicyReader(&self, bstrreader: &windows_core::BSTR, varreserved: &super::super::System::Variant::VARIANT) -> windows_core::Result<()>;
    fn ApplicationGroups(&self) -> windows_core::Result<IAzApplicationGroups>;
    fn OpenApplicationGroup(&self, bstrgroupname: &windows_core::BSTR, varreserved: &super::super::System::Variant::VARIANT) -> windows_core::Result<IAzApplicationGroup>;
    fn CreateApplicationGroup(&self, bstrgroupname: &windows_core::BSTR, varreserved: &super::super::System::Variant::VARIANT) -> windows_core::Result<IAzApplicationGroup>;
    fn DeleteApplicationGroup(&self, bstrgroupname: &windows_core::BSTR, varreserved: &super::super::System::Variant::VARIANT) -> windows_core::Result<()>;
    fn Roles(&self) -> windows_core::Result<IAzRoles>;
    fn OpenRole(&self, bstrrolename: &windows_core::BSTR, varreserved: &super::super::System::Variant::VARIANT) -> windows_core::Result<IAzRole>;
    fn CreateRole(&self, bstrrolename: &windows_core::BSTR, varreserved: &super::super::System::Variant::VARIANT) -> windows_core::Result<IAzRole>;
    fn DeleteRole(&self, bstrrolename: &windows_core::BSTR, varreserved: &super::super::System::Variant::VARIANT) -> windows_core::Result<()>;
    fn Tasks(&self) -> windows_core::Result<IAzTasks>;
    fn OpenTask(&self, bstrtaskname: &windows_core::BSTR, varreserved: &super::super::System::Variant::VARIANT) -> windows_core::Result<IAzTask>;
    fn CreateTask(&self, bstrtaskname: &windows_core::BSTR, varreserved: &super::super::System::Variant::VARIANT) -> windows_core::Result<IAzTask>;
    fn DeleteTask(&self, bstrtaskname: &windows_core::BSTR, varreserved: &super::super::System::Variant::VARIANT) -> windows_core::Result<()>;
    fn Submit(&self, lflags: i32, varreserved: &super::super::System::Variant::VARIANT) -> windows_core::Result<()>;
    fn CanBeDelegated(&self) -> windows_core::Result<windows_core::BOOL>;
    fn BizrulesWritable(&self) -> windows_core::Result<windows_core::BOOL>;
    fn PolicyAdministratorsName(&self) -> windows_core::Result<super::super::System::Variant::VARIANT>;
    fn PolicyReadersName(&self) -> windows_core::Result<super::super::System::Variant::VARIANT>;
    fn AddPolicyAdministratorName(&self, bstradmin: &windows_core::BSTR, varreserved: &super::super::System::Variant::VARIANT) -> windows_core::Result<()>;
    fn DeletePolicyAdministratorName(&self, bstradmin: &windows_core::BSTR, varreserved: &super::super::System::Variant::VARIANT) -> windows_core::Result<()>;
    fn AddPolicyReaderName(&self, bstrreader: &windows_core::BSTR, varreserved: &super::super::System::Variant::VARIANT) -> windows_core::Result<()>;
    fn DeletePolicyReaderName(&self, bstrreader: &windows_core::BSTR, varreserved: &super::super::System::Variant::VARIANT) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IAzScope_Vtbl {
    pub const fn new<Identity: IAzScope_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Name<Identity: IAzScope_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrname: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IAzScope_Impl::Name(this) {
                    Ok(ok__) => {
                        pbstrname.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetName<Identity: IAzScope_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrname: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IAzScope_Impl::SetName(this, core::mem::transmute(&bstrname)).into()
            }
        }
        unsafe extern "system" fn Description<Identity: IAzScope_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrdescription: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IAzScope_Impl::Description(this) {
                    Ok(ok__) => {
                        pbstrdescription.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetDescription<Identity: IAzScope_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrdescription: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IAzScope_Impl::SetDescription(this, core::mem::transmute(&bstrdescription)).into()
            }
        }
        unsafe extern "system" fn ApplicationData<Identity: IAzScope_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrapplicationdata: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IAzScope_Impl::ApplicationData(this) {
                    Ok(ok__) => {
                        pbstrapplicationdata.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetApplicationData<Identity: IAzScope_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrapplicationdata: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IAzScope_Impl::SetApplicationData(this, core::mem::transmute(&bstrapplicationdata)).into()
            }
        }
        unsafe extern "system" fn Writable<Identity: IAzScope_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pfprop: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IAzScope_Impl::Writable(this) {
                    Ok(ok__) => {
                        pfprop.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetProperty<Identity: IAzScope_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lpropid: i32, varreserved: super::super::System::Variant::VARIANT, pvarprop: *mut super::super::System::Variant::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IAzScope_Impl::GetProperty(this, core::mem::transmute_copy(&lpropid), core::mem::transmute(&varreserved)) {
                    Ok(ok__) => {
                        pvarprop.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetProperty<Identity: IAzScope_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lpropid: i32, varprop: super::super::System::Variant::VARIANT, varreserved: super::super::System::Variant::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IAzScope_Impl::SetProperty(this, core::mem::transmute_copy(&lpropid), core::mem::transmute(&varprop), core::mem::transmute(&varreserved)).into()
            }
        }
        unsafe extern "system" fn AddPropertyItem<Identity: IAzScope_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lpropid: i32, varprop: super::super::System::Variant::VARIANT, varreserved: super::super::System::Variant::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IAzScope_Impl::AddPropertyItem(this, core::mem::transmute_copy(&lpropid), core::mem::transmute(&varprop), core::mem::transmute(&varreserved)).into()
            }
        }
        unsafe extern "system" fn DeletePropertyItem<Identity: IAzScope_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lpropid: i32, varprop: super::super::System::Variant::VARIANT, varreserved: super::super::System::Variant::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IAzScope_Impl::DeletePropertyItem(this, core::mem::transmute_copy(&lpropid), core::mem::transmute(&varprop), core::mem::transmute(&varreserved)).into()
            }
        }
        unsafe extern "system" fn PolicyAdministrators<Identity: IAzScope_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvaradmins: *mut super::super::System::Variant::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IAzScope_Impl::PolicyAdministrators(this) {
                    Ok(ok__) => {
                        pvaradmins.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn PolicyReaders<Identity: IAzScope_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvarreaders: *mut super::super::System::Variant::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IAzScope_Impl::PolicyReaders(this) {
                    Ok(ok__) => {
                        pvarreaders.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn AddPolicyAdministrator<Identity: IAzScope_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstradmin: *mut core::ffi::c_void, varreserved: super::super::System::Variant::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IAzScope_Impl::AddPolicyAdministrator(this, core::mem::transmute(&bstradmin), core::mem::transmute(&varreserved)).into()
            }
        }
        unsafe extern "system" fn DeletePolicyAdministrator<Identity: IAzScope_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstradmin: *mut core::ffi::c_void, varreserved: super::super::System::Variant::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IAzScope_Impl::DeletePolicyAdministrator(this, core::mem::transmute(&bstradmin), core::mem::transmute(&varreserved)).into()
            }
        }
        unsafe extern "system" fn AddPolicyReader<Identity: IAzScope_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrreader: *mut core::ffi::c_void, varreserved: super::super::System::Variant::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IAzScope_Impl::AddPolicyReader(this, core::mem::transmute(&bstrreader), core::mem::transmute(&varreserved)).into()
            }
        }
        unsafe extern "system" fn DeletePolicyReader<Identity: IAzScope_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrreader: *mut core::ffi::c_void, varreserved: super::super::System::Variant::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IAzScope_Impl::DeletePolicyReader(this, core::mem::transmute(&bstrreader), core::mem::transmute(&varreserved)).into()
            }
        }
        unsafe extern "system" fn ApplicationGroups<Identity: IAzScope_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppgroupcollection: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IAzScope_Impl::ApplicationGroups(this) {
                    Ok(ok__) => {
                        ppgroupcollection.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn OpenApplicationGroup<Identity: IAzScope_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrgroupname: *mut core::ffi::c_void, varreserved: super::super::System::Variant::VARIANT, ppgroup: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IAzScope_Impl::OpenApplicationGroup(this, core::mem::transmute(&bstrgroupname), core::mem::transmute(&varreserved)) {
                    Ok(ok__) => {
                        ppgroup.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CreateApplicationGroup<Identity: IAzScope_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrgroupname: *mut core::ffi::c_void, varreserved: super::super::System::Variant::VARIANT, ppgroup: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IAzScope_Impl::CreateApplicationGroup(this, core::mem::transmute(&bstrgroupname), core::mem::transmute(&varreserved)) {
                    Ok(ok__) => {
                        ppgroup.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn DeleteApplicationGroup<Identity: IAzScope_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrgroupname: *mut core::ffi::c_void, varreserved: super::super::System::Variant::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IAzScope_Impl::DeleteApplicationGroup(this, core::mem::transmute(&bstrgroupname), core::mem::transmute(&varreserved)).into()
            }
        }
        unsafe extern "system" fn Roles<Identity: IAzScope_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pprolecollection: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IAzScope_Impl::Roles(this) {
                    Ok(ok__) => {
                        pprolecollection.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn OpenRole<Identity: IAzScope_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrrolename: *mut core::ffi::c_void, varreserved: super::super::System::Variant::VARIANT, pprole: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IAzScope_Impl::OpenRole(this, core::mem::transmute(&bstrrolename), core::mem::transmute(&varreserved)) {
                    Ok(ok__) => {
                        pprole.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CreateRole<Identity: IAzScope_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrrolename: *mut core::ffi::c_void, varreserved: super::super::System::Variant::VARIANT, pprole: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IAzScope_Impl::CreateRole(this, core::mem::transmute(&bstrrolename), core::mem::transmute(&varreserved)) {
                    Ok(ok__) => {
                        pprole.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn DeleteRole<Identity: IAzScope_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrrolename: *mut core::ffi::c_void, varreserved: super::super::System::Variant::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IAzScope_Impl::DeleteRole(this, core::mem::transmute(&bstrrolename), core::mem::transmute(&varreserved)).into()
            }
        }
        unsafe extern "system" fn Tasks<Identity: IAzScope_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pptaskcollection: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IAzScope_Impl::Tasks(this) {
                    Ok(ok__) => {
                        pptaskcollection.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn OpenTask<Identity: IAzScope_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrtaskname: *mut core::ffi::c_void, varreserved: super::super::System::Variant::VARIANT, pptask: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IAzScope_Impl::OpenTask(this, core::mem::transmute(&bstrtaskname), core::mem::transmute(&varreserved)) {
                    Ok(ok__) => {
                        pptask.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CreateTask<Identity: IAzScope_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrtaskname: *mut core::ffi::c_void, varreserved: super::super::System::Variant::VARIANT, pptask: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IAzScope_Impl::CreateTask(this, core::mem::transmute(&bstrtaskname), core::mem::transmute(&varreserved)) {
                    Ok(ok__) => {
                        pptask.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn DeleteTask<Identity: IAzScope_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrtaskname: *mut core::ffi::c_void, varreserved: super::super::System::Variant::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IAzScope_Impl::DeleteTask(this, core::mem::transmute(&bstrtaskname), core::mem::transmute(&varreserved)).into()
            }
        }
        unsafe extern "system" fn Submit<Identity: IAzScope_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lflags: i32, varreserved: super::super::System::Variant::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IAzScope_Impl::Submit(this, core::mem::transmute_copy(&lflags), core::mem::transmute(&varreserved)).into()
            }
        }
        unsafe extern "system" fn CanBeDelegated<Identity: IAzScope_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pfprop: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IAzScope_Impl::CanBeDelegated(this) {
                    Ok(ok__) => {
                        pfprop.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn BizrulesWritable<Identity: IAzScope_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pfprop: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IAzScope_Impl::BizrulesWritable(this) {
                    Ok(ok__) => {
                        pfprop.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn PolicyAdministratorsName<Identity: IAzScope_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvaradmins: *mut super::super::System::Variant::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IAzScope_Impl::PolicyAdministratorsName(this) {
                    Ok(ok__) => {
                        pvaradmins.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn PolicyReadersName<Identity: IAzScope_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvarreaders: *mut super::super::System::Variant::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IAzScope_Impl::PolicyReadersName(this) {
                    Ok(ok__) => {
                        pvarreaders.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn AddPolicyAdministratorName<Identity: IAzScope_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstradmin: *mut core::ffi::c_void, varreserved: super::super::System::Variant::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IAzScope_Impl::AddPolicyAdministratorName(this, core::mem::transmute(&bstradmin), core::mem::transmute(&varreserved)).into()
            }
        }
        unsafe extern "system" fn DeletePolicyAdministratorName<Identity: IAzScope_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstradmin: *mut core::ffi::c_void, varreserved: super::super::System::Variant::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IAzScope_Impl::DeletePolicyAdministratorName(this, core::mem::transmute(&bstradmin), core::mem::transmute(&varreserved)).into()
            }
        }
        unsafe extern "system" fn AddPolicyReaderName<Identity: IAzScope_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrreader: *mut core::ffi::c_void, varreserved: super::super::System::Variant::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IAzScope_Impl::AddPolicyReaderName(this, core::mem::transmute(&bstrreader), core::mem::transmute(&varreserved)).into()
            }
        }
        unsafe extern "system" fn DeletePolicyReaderName<Identity: IAzScope_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrreader: *mut core::ffi::c_void, varreserved: super::super::System::Variant::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IAzScope_Impl::DeletePolicyReaderName(this, core::mem::transmute(&bstrreader), core::mem::transmute(&varreserved)).into()
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            Name: Name::<Identity, OFFSET>,
            SetName: SetName::<Identity, OFFSET>,
            Description: Description::<Identity, OFFSET>,
            SetDescription: SetDescription::<Identity, OFFSET>,
            ApplicationData: ApplicationData::<Identity, OFFSET>,
            SetApplicationData: SetApplicationData::<Identity, OFFSET>,
            Writable: Writable::<Identity, OFFSET>,
            GetProperty: GetProperty::<Identity, OFFSET>,
            SetProperty: SetProperty::<Identity, OFFSET>,
            AddPropertyItem: AddPropertyItem::<Identity, OFFSET>,
            DeletePropertyItem: DeletePropertyItem::<Identity, OFFSET>,
            PolicyAdministrators: PolicyAdministrators::<Identity, OFFSET>,
            PolicyReaders: PolicyReaders::<Identity, OFFSET>,
            AddPolicyAdministrator: AddPolicyAdministrator::<Identity, OFFSET>,
            DeletePolicyAdministrator: DeletePolicyAdministrator::<Identity, OFFSET>,
            AddPolicyReader: AddPolicyReader::<Identity, OFFSET>,
            DeletePolicyReader: DeletePolicyReader::<Identity, OFFSET>,
            ApplicationGroups: ApplicationGroups::<Identity, OFFSET>,
            OpenApplicationGroup: OpenApplicationGroup::<Identity, OFFSET>,
            CreateApplicationGroup: CreateApplicationGroup::<Identity, OFFSET>,
            DeleteApplicationGroup: DeleteApplicationGroup::<Identity, OFFSET>,
            Roles: Roles::<Identity, OFFSET>,
            OpenRole: OpenRole::<Identity, OFFSET>,
            CreateRole: CreateRole::<Identity, OFFSET>,
            DeleteRole: DeleteRole::<Identity, OFFSET>,
            Tasks: Tasks::<Identity, OFFSET>,
            OpenTask: OpenTask::<Identity, OFFSET>,
            CreateTask: CreateTask::<Identity, OFFSET>,
            DeleteTask: DeleteTask::<Identity, OFFSET>,
            Submit: Submit::<Identity, OFFSET>,
            CanBeDelegated: CanBeDelegated::<Identity, OFFSET>,
            BizrulesWritable: BizrulesWritable::<Identity, OFFSET>,
            PolicyAdministratorsName: PolicyAdministratorsName::<Identity, OFFSET>,
            PolicyReadersName: PolicyReadersName::<Identity, OFFSET>,
            AddPolicyAdministratorName: AddPolicyAdministratorName::<Identity, OFFSET>,
            DeletePolicyAdministratorName: DeletePolicyAdministratorName::<Identity, OFFSET>,
            AddPolicyReaderName: AddPolicyReaderName::<Identity, OFFSET>,
            DeletePolicyReaderName: DeletePolicyReaderName::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IAzScope as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for IAzScope {}
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
    pub unsafe fn RoleDefinitions(&self) -> windows_core::Result<IAzRoleDefinitions> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).RoleDefinitions)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn CreateRoleDefinition(&self, bstrroledefinitionname: &windows_core::BSTR) -> windows_core::Result<IAzRoleDefinition> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateRoleDefinition)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrroledefinitionname), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn OpenRoleDefinition(&self, bstrroledefinitionname: &windows_core::BSTR) -> windows_core::Result<IAzRoleDefinition> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).OpenRoleDefinition)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrroledefinitionname), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn DeleteRoleDefinition(&self, bstrroledefinitionname: &windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).DeleteRoleDefinition)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrroledefinitionname)).ok() }
    }
    pub unsafe fn RoleAssignments(&self) -> windows_core::Result<IAzRoleAssignments> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).RoleAssignments)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn CreateRoleAssignment(&self, bstrroleassignmentname: &windows_core::BSTR) -> windows_core::Result<IAzRoleAssignment> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateRoleAssignment)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrroleassignmentname), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn OpenRoleAssignment(&self, bstrroleassignmentname: &windows_core::BSTR) -> windows_core::Result<IAzRoleAssignment> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).OpenRoleAssignment)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrroleassignmentname), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn DeleteRoleAssignment(&self, bstrroleassignmentname: &windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).DeleteRoleAssignment)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrroleassignmentname)).ok() }
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IAzScope2_Vtbl {
    pub base__: IAzScope_Vtbl,
    pub RoleDefinitions: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateRoleDefinition: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub OpenRoleDefinition: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub DeleteRoleDefinition: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub RoleAssignments: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateRoleAssignment: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub OpenRoleAssignment: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub DeleteRoleAssignment: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IAzScope2_Impl: IAzScope_Impl {
    fn RoleDefinitions(&self) -> windows_core::Result<IAzRoleDefinitions>;
    fn CreateRoleDefinition(&self, bstrroledefinitionname: &windows_core::BSTR) -> windows_core::Result<IAzRoleDefinition>;
    fn OpenRoleDefinition(&self, bstrroledefinitionname: &windows_core::BSTR) -> windows_core::Result<IAzRoleDefinition>;
    fn DeleteRoleDefinition(&self, bstrroledefinitionname: &windows_core::BSTR) -> windows_core::Result<()>;
    fn RoleAssignments(&self) -> windows_core::Result<IAzRoleAssignments>;
    fn CreateRoleAssignment(&self, bstrroleassignmentname: &windows_core::BSTR) -> windows_core::Result<IAzRoleAssignment>;
    fn OpenRoleAssignment(&self, bstrroleassignmentname: &windows_core::BSTR) -> windows_core::Result<IAzRoleAssignment>;
    fn DeleteRoleAssignment(&self, bstrroleassignmentname: &windows_core::BSTR) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IAzScope2_Vtbl {
    pub const fn new<Identity: IAzScope2_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn RoleDefinitions<Identity: IAzScope2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pproledefinitions: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IAzScope2_Impl::RoleDefinitions(this) {
                    Ok(ok__) => {
                        pproledefinitions.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CreateRoleDefinition<Identity: IAzScope2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrroledefinitionname: *mut core::ffi::c_void, pproledefinitions: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IAzScope2_Impl::CreateRoleDefinition(this, core::mem::transmute(&bstrroledefinitionname)) {
                    Ok(ok__) => {
                        pproledefinitions.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn OpenRoleDefinition<Identity: IAzScope2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrroledefinitionname: *mut core::ffi::c_void, pproledefinitions: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IAzScope2_Impl::OpenRoleDefinition(this, core::mem::transmute(&bstrroledefinitionname)) {
                    Ok(ok__) => {
                        pproledefinitions.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn DeleteRoleDefinition<Identity: IAzScope2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrroledefinitionname: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IAzScope2_Impl::DeleteRoleDefinition(this, core::mem::transmute(&bstrroledefinitionname)).into()
            }
        }
        unsafe extern "system" fn RoleAssignments<Identity: IAzScope2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pproleassignments: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IAzScope2_Impl::RoleAssignments(this) {
                    Ok(ok__) => {
                        pproleassignments.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CreateRoleAssignment<Identity: IAzScope2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrroleassignmentname: *mut core::ffi::c_void, pproleassignment: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IAzScope2_Impl::CreateRoleAssignment(this, core::mem::transmute(&bstrroleassignmentname)) {
                    Ok(ok__) => {
                        pproleassignment.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn OpenRoleAssignment<Identity: IAzScope2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrroleassignmentname: *mut core::ffi::c_void, pproleassignment: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IAzScope2_Impl::OpenRoleAssignment(this, core::mem::transmute(&bstrroleassignmentname)) {
                    Ok(ok__) => {
                        pproleassignment.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn DeleteRoleAssignment<Identity: IAzScope2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrroleassignmentname: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IAzScope2_Impl::DeleteRoleAssignment(this, core::mem::transmute(&bstrroleassignmentname)).into()
            }
        }
        Self {
            base__: IAzScope_Vtbl::new::<Identity, OFFSET>(),
            RoleDefinitions: RoleDefinitions::<Identity, OFFSET>,
            CreateRoleDefinition: CreateRoleDefinition::<Identity, OFFSET>,
            OpenRoleDefinition: OpenRoleDefinition::<Identity, OFFSET>,
            DeleteRoleDefinition: DeleteRoleDefinition::<Identity, OFFSET>,
            RoleAssignments: RoleAssignments::<Identity, OFFSET>,
            CreateRoleAssignment: CreateRoleAssignment::<Identity, OFFSET>,
            OpenRoleAssignment: OpenRoleAssignment::<Identity, OFFSET>,
            DeleteRoleAssignment: DeleteRoleAssignment::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IAzScope2 as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID || iid == &<IAzScope as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for IAzScope2 {}
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
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn get_Item(&self, index: i32) -> windows_core::Result<super::super::System::Variant::VARIANT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_Item)(windows_core::Interface::as_raw(self), index, &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn Count(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Count)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn _NewEnum(&self) -> windows_core::Result<windows_core::IUnknown> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self)._NewEnum)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IAzScopes_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub get_Item: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut super::super::System::Variant::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    get_Item: usize,
    pub Count: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub _NewEnum: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IAzScopes_Impl: super::super::System::Com::IDispatch_Impl {
    fn get_Item(&self, index: i32) -> windows_core::Result<super::super::System::Variant::VARIANT>;
    fn Count(&self) -> windows_core::Result<i32>;
    fn _NewEnum(&self) -> windows_core::Result<windows_core::IUnknown>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IAzScopes_Vtbl {
    pub const fn new<Identity: IAzScopes_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn get_Item<Identity: IAzScopes_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: i32, pvarobtptr: *mut super::super::System::Variant::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IAzScopes_Impl::get_Item(this, core::mem::transmute_copy(&index)) {
                    Ok(ok__) => {
                        pvarobtptr.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Count<Identity: IAzScopes_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, plcount: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IAzScopes_Impl::Count(this) {
                    Ok(ok__) => {
                        plcount.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn _NewEnum<Identity: IAzScopes_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppenumptr: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IAzScopes_Impl::_NewEnum(this) {
                    Ok(ok__) => {
                        ppenumptr.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            get_Item: get_Item::<Identity, OFFSET>,
            Count: Count::<Identity, OFFSET>,
            _NewEnum: _NewEnum::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IAzScopes as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for IAzScopes {}
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
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Name)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn SetName(&self, bstrname: &windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetName)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrname)).ok() }
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
    pub unsafe fn ApplicationData(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ApplicationData)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn SetApplicationData(&self, bstrapplicationdata: &windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetApplicationData)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrapplicationdata)).ok() }
    }
    pub unsafe fn BizRule(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).BizRule)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn SetBizRule(&self, bstrprop: &windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetBizRule)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrprop)).ok() }
    }
    pub unsafe fn BizRuleLanguage(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).BizRuleLanguage)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn SetBizRuleLanguage(&self, bstrprop: &windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetBizRuleLanguage)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrprop)).ok() }
    }
    pub unsafe fn BizRuleImportedPath(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).BizRuleImportedPath)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn SetBizRuleImportedPath(&self, bstrprop: &windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetBizRuleImportedPath)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrprop)).ok() }
    }
    pub unsafe fn IsRoleDefinition(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).IsRoleDefinition)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetIsRoleDefinition(&self, fprop: bool) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetIsRoleDefinition)(windows_core::Interface::as_raw(self), fprop.into()).ok() }
    }
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn Operations(&self) -> windows_core::Result<super::super::System::Variant::VARIANT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Operations)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn Tasks(&self) -> windows_core::Result<super::super::System::Variant::VARIANT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Tasks)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn AddOperation(&self, bstrop: &windows_core::BSTR, varreserved: &super::super::System::Variant::VARIANT) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).AddOperation)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrop), core::mem::transmute_copy(varreserved)).ok() }
    }
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn DeleteOperation(&self, bstrop: &windows_core::BSTR, varreserved: &super::super::System::Variant::VARIANT) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).DeleteOperation)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrop), core::mem::transmute_copy(varreserved)).ok() }
    }
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn AddTask(&self, bstrtask: &windows_core::BSTR, varreserved: &super::super::System::Variant::VARIANT) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).AddTask)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrtask), core::mem::transmute_copy(varreserved)).ok() }
    }
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn DeleteTask(&self, bstrtask: &windows_core::BSTR, varreserved: &super::super::System::Variant::VARIANT) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).DeleteTask)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrtask), core::mem::transmute_copy(varreserved)).ok() }
    }
    pub unsafe fn Writable(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Writable)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn GetProperty(&self, lpropid: i32, varreserved: &super::super::System::Variant::VARIANT) -> windows_core::Result<super::super::System::Variant::VARIANT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetProperty)(windows_core::Interface::as_raw(self), lpropid, core::mem::transmute_copy(varreserved), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn SetProperty(&self, lpropid: i32, varprop: &super::super::System::Variant::VARIANT, varreserved: &super::super::System::Variant::VARIANT) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetProperty)(windows_core::Interface::as_raw(self), lpropid, core::mem::transmute_copy(varprop), core::mem::transmute_copy(varreserved)).ok() }
    }
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn AddPropertyItem(&self, lpropid: i32, varprop: &super::super::System::Variant::VARIANT, varreserved: &super::super::System::Variant::VARIANT) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).AddPropertyItem)(windows_core::Interface::as_raw(self), lpropid, core::mem::transmute_copy(varprop), core::mem::transmute_copy(varreserved)).ok() }
    }
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn DeletePropertyItem(&self, lpropid: i32, varprop: &super::super::System::Variant::VARIANT, varreserved: &super::super::System::Variant::VARIANT) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).DeletePropertyItem)(windows_core::Interface::as_raw(self), lpropid, core::mem::transmute_copy(varprop), core::mem::transmute_copy(varreserved)).ok() }
    }
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn Submit(&self, lflags: i32, varreserved: &super::super::System::Variant::VARIANT) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Submit)(windows_core::Interface::as_raw(self), lflags, core::mem::transmute_copy(varreserved)).ok() }
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IAzTask_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub Name: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Description: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetDescription: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ApplicationData: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetApplicationData: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub BizRule: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetBizRule: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub BizRuleLanguage: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetBizRuleLanguage: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub BizRuleImportedPath: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetBizRuleImportedPath: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub IsRoleDefinition: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::BOOL) -> windows_core::HRESULT,
    pub SetIsRoleDefinition: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::BOOL) -> windows_core::HRESULT,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub Operations: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::System::Variant::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    Operations: usize,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub Tasks: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::System::Variant::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    Tasks: usize,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub AddOperation: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, super::super::System::Variant::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    AddOperation: usize,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub DeleteOperation: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, super::super::System::Variant::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    DeleteOperation: usize,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub AddTask: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, super::super::System::Variant::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    AddTask: usize,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub DeleteTask: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, super::super::System::Variant::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    DeleteTask: usize,
    pub Writable: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::BOOL) -> windows_core::HRESULT,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub GetProperty: unsafe extern "system" fn(*mut core::ffi::c_void, i32, super::super::System::Variant::VARIANT, *mut super::super::System::Variant::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    GetProperty: usize,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub SetProperty: unsafe extern "system" fn(*mut core::ffi::c_void, i32, super::super::System::Variant::VARIANT, super::super::System::Variant::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    SetProperty: usize,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub AddPropertyItem: unsafe extern "system" fn(*mut core::ffi::c_void, i32, super::super::System::Variant::VARIANT, super::super::System::Variant::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    AddPropertyItem: usize,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub DeletePropertyItem: unsafe extern "system" fn(*mut core::ffi::c_void, i32, super::super::System::Variant::VARIANT, super::super::System::Variant::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    DeletePropertyItem: usize,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub Submit: unsafe extern "system" fn(*mut core::ffi::c_void, i32, super::super::System::Variant::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    Submit: usize,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IAzTask_Impl: super::super::System::Com::IDispatch_Impl {
    fn Name(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetName(&self, bstrname: &windows_core::BSTR) -> windows_core::Result<()>;
    fn Description(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetDescription(&self, bstrdescription: &windows_core::BSTR) -> windows_core::Result<()>;
    fn ApplicationData(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetApplicationData(&self, bstrapplicationdata: &windows_core::BSTR) -> windows_core::Result<()>;
    fn BizRule(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetBizRule(&self, bstrprop: &windows_core::BSTR) -> windows_core::Result<()>;
    fn BizRuleLanguage(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetBizRuleLanguage(&self, bstrprop: &windows_core::BSTR) -> windows_core::Result<()>;
    fn BizRuleImportedPath(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetBizRuleImportedPath(&self, bstrprop: &windows_core::BSTR) -> windows_core::Result<()>;
    fn IsRoleDefinition(&self) -> windows_core::Result<windows_core::BOOL>;
    fn SetIsRoleDefinition(&self, fprop: windows_core::BOOL) -> windows_core::Result<()>;
    fn Operations(&self) -> windows_core::Result<super::super::System::Variant::VARIANT>;
    fn Tasks(&self) -> windows_core::Result<super::super::System::Variant::VARIANT>;
    fn AddOperation(&self, bstrop: &windows_core::BSTR, varreserved: &super::super::System::Variant::VARIANT) -> windows_core::Result<()>;
    fn DeleteOperation(&self, bstrop: &windows_core::BSTR, varreserved: &super::super::System::Variant::VARIANT) -> windows_core::Result<()>;
    fn AddTask(&self, bstrtask: &windows_core::BSTR, varreserved: &super::super::System::Variant::VARIANT) -> windows_core::Result<()>;
    fn DeleteTask(&self, bstrtask: &windows_core::BSTR, varreserved: &super::super::System::Variant::VARIANT) -> windows_core::Result<()>;
    fn Writable(&self) -> windows_core::Result<windows_core::BOOL>;
    fn GetProperty(&self, lpropid: i32, varreserved: &super::super::System::Variant::VARIANT) -> windows_core::Result<super::super::System::Variant::VARIANT>;
    fn SetProperty(&self, lpropid: i32, varprop: &super::super::System::Variant::VARIANT, varreserved: &super::super::System::Variant::VARIANT) -> windows_core::Result<()>;
    fn AddPropertyItem(&self, lpropid: i32, varprop: &super::super::System::Variant::VARIANT, varreserved: &super::super::System::Variant::VARIANT) -> windows_core::Result<()>;
    fn DeletePropertyItem(&self, lpropid: i32, varprop: &super::super::System::Variant::VARIANT, varreserved: &super::super::System::Variant::VARIANT) -> windows_core::Result<()>;
    fn Submit(&self, lflags: i32, varreserved: &super::super::System::Variant::VARIANT) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IAzTask_Vtbl {
    pub const fn new<Identity: IAzTask_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Name<Identity: IAzTask_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrname: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IAzTask_Impl::Name(this) {
                    Ok(ok__) => {
                        pbstrname.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetName<Identity: IAzTask_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrname: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IAzTask_Impl::SetName(this, core::mem::transmute(&bstrname)).into()
            }
        }
        unsafe extern "system" fn Description<Identity: IAzTask_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrdescription: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IAzTask_Impl::Description(this) {
                    Ok(ok__) => {
                        pbstrdescription.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetDescription<Identity: IAzTask_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrdescription: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IAzTask_Impl::SetDescription(this, core::mem::transmute(&bstrdescription)).into()
            }
        }
        unsafe extern "system" fn ApplicationData<Identity: IAzTask_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrapplicationdata: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IAzTask_Impl::ApplicationData(this) {
                    Ok(ok__) => {
                        pbstrapplicationdata.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetApplicationData<Identity: IAzTask_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrapplicationdata: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IAzTask_Impl::SetApplicationData(this, core::mem::transmute(&bstrapplicationdata)).into()
            }
        }
        unsafe extern "system" fn BizRule<Identity: IAzTask_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrprop: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IAzTask_Impl::BizRule(this) {
                    Ok(ok__) => {
                        pbstrprop.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetBizRule<Identity: IAzTask_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrprop: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IAzTask_Impl::SetBizRule(this, core::mem::transmute(&bstrprop)).into()
            }
        }
        unsafe extern "system" fn BizRuleLanguage<Identity: IAzTask_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrprop: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IAzTask_Impl::BizRuleLanguage(this) {
                    Ok(ok__) => {
                        pbstrprop.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetBizRuleLanguage<Identity: IAzTask_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrprop: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IAzTask_Impl::SetBizRuleLanguage(this, core::mem::transmute(&bstrprop)).into()
            }
        }
        unsafe extern "system" fn BizRuleImportedPath<Identity: IAzTask_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrprop: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IAzTask_Impl::BizRuleImportedPath(this) {
                    Ok(ok__) => {
                        pbstrprop.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetBizRuleImportedPath<Identity: IAzTask_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrprop: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IAzTask_Impl::SetBizRuleImportedPath(this, core::mem::transmute(&bstrprop)).into()
            }
        }
        unsafe extern "system" fn IsRoleDefinition<Identity: IAzTask_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pfprop: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IAzTask_Impl::IsRoleDefinition(this) {
                    Ok(ok__) => {
                        pfprop.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetIsRoleDefinition<Identity: IAzTask_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, fprop: windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IAzTask_Impl::SetIsRoleDefinition(this, core::mem::transmute_copy(&fprop)).into()
            }
        }
        unsafe extern "system" fn Operations<Identity: IAzTask_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvarprop: *mut super::super::System::Variant::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IAzTask_Impl::Operations(this) {
                    Ok(ok__) => {
                        pvarprop.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Tasks<Identity: IAzTask_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvarprop: *mut super::super::System::Variant::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IAzTask_Impl::Tasks(this) {
                    Ok(ok__) => {
                        pvarprop.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn AddOperation<Identity: IAzTask_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrop: *mut core::ffi::c_void, varreserved: super::super::System::Variant::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IAzTask_Impl::AddOperation(this, core::mem::transmute(&bstrop), core::mem::transmute(&varreserved)).into()
            }
        }
        unsafe extern "system" fn DeleteOperation<Identity: IAzTask_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrop: *mut core::ffi::c_void, varreserved: super::super::System::Variant::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IAzTask_Impl::DeleteOperation(this, core::mem::transmute(&bstrop), core::mem::transmute(&varreserved)).into()
            }
        }
        unsafe extern "system" fn AddTask<Identity: IAzTask_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrtask: *mut core::ffi::c_void, varreserved: super::super::System::Variant::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IAzTask_Impl::AddTask(this, core::mem::transmute(&bstrtask), core::mem::transmute(&varreserved)).into()
            }
        }
        unsafe extern "system" fn DeleteTask<Identity: IAzTask_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrtask: *mut core::ffi::c_void, varreserved: super::super::System::Variant::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IAzTask_Impl::DeleteTask(this, core::mem::transmute(&bstrtask), core::mem::transmute(&varreserved)).into()
            }
        }
        unsafe extern "system" fn Writable<Identity: IAzTask_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pfprop: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IAzTask_Impl::Writable(this) {
                    Ok(ok__) => {
                        pfprop.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetProperty<Identity: IAzTask_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lpropid: i32, varreserved: super::super::System::Variant::VARIANT, pvarprop: *mut super::super::System::Variant::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IAzTask_Impl::GetProperty(this, core::mem::transmute_copy(&lpropid), core::mem::transmute(&varreserved)) {
                    Ok(ok__) => {
                        pvarprop.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetProperty<Identity: IAzTask_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lpropid: i32, varprop: super::super::System::Variant::VARIANT, varreserved: super::super::System::Variant::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IAzTask_Impl::SetProperty(this, core::mem::transmute_copy(&lpropid), core::mem::transmute(&varprop), core::mem::transmute(&varreserved)).into()
            }
        }
        unsafe extern "system" fn AddPropertyItem<Identity: IAzTask_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lpropid: i32, varprop: super::super::System::Variant::VARIANT, varreserved: super::super::System::Variant::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IAzTask_Impl::AddPropertyItem(this, core::mem::transmute_copy(&lpropid), core::mem::transmute(&varprop), core::mem::transmute(&varreserved)).into()
            }
        }
        unsafe extern "system" fn DeletePropertyItem<Identity: IAzTask_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lpropid: i32, varprop: super::super::System::Variant::VARIANT, varreserved: super::super::System::Variant::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IAzTask_Impl::DeletePropertyItem(this, core::mem::transmute_copy(&lpropid), core::mem::transmute(&varprop), core::mem::transmute(&varreserved)).into()
            }
        }
        unsafe extern "system" fn Submit<Identity: IAzTask_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lflags: i32, varreserved: super::super::System::Variant::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IAzTask_Impl::Submit(this, core::mem::transmute_copy(&lflags), core::mem::transmute(&varreserved)).into()
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            Name: Name::<Identity, OFFSET>,
            SetName: SetName::<Identity, OFFSET>,
            Description: Description::<Identity, OFFSET>,
            SetDescription: SetDescription::<Identity, OFFSET>,
            ApplicationData: ApplicationData::<Identity, OFFSET>,
            SetApplicationData: SetApplicationData::<Identity, OFFSET>,
            BizRule: BizRule::<Identity, OFFSET>,
            SetBizRule: SetBizRule::<Identity, OFFSET>,
            BizRuleLanguage: BizRuleLanguage::<Identity, OFFSET>,
            SetBizRuleLanguage: SetBizRuleLanguage::<Identity, OFFSET>,
            BizRuleImportedPath: BizRuleImportedPath::<Identity, OFFSET>,
            SetBizRuleImportedPath: SetBizRuleImportedPath::<Identity, OFFSET>,
            IsRoleDefinition: IsRoleDefinition::<Identity, OFFSET>,
            SetIsRoleDefinition: SetIsRoleDefinition::<Identity, OFFSET>,
            Operations: Operations::<Identity, OFFSET>,
            Tasks: Tasks::<Identity, OFFSET>,
            AddOperation: AddOperation::<Identity, OFFSET>,
            DeleteOperation: DeleteOperation::<Identity, OFFSET>,
            AddTask: AddTask::<Identity, OFFSET>,
            DeleteTask: DeleteTask::<Identity, OFFSET>,
            Writable: Writable::<Identity, OFFSET>,
            GetProperty: GetProperty::<Identity, OFFSET>,
            SetProperty: SetProperty::<Identity, OFFSET>,
            AddPropertyItem: AddPropertyItem::<Identity, OFFSET>,
            DeletePropertyItem: DeletePropertyItem::<Identity, OFFSET>,
            Submit: Submit::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IAzTask as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for IAzTask {}
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
    pub unsafe fn RoleAssignments(&self, bstrscopename: &windows_core::BSTR, brecursive: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<IAzRoleAssignments> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).RoleAssignments)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrscopename), brecursive, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IAzTask2_Vtbl {
    pub base__: IAzTask_Vtbl,
    pub RoleAssignments: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, super::super::Foundation::VARIANT_BOOL, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IAzTask2_Impl: IAzTask_Impl {
    fn RoleAssignments(&self, bstrscopename: &windows_core::BSTR, brecursive: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<IAzRoleAssignments>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IAzTask2_Vtbl {
    pub const fn new<Identity: IAzTask2_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn RoleAssignments<Identity: IAzTask2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrscopename: *mut core::ffi::c_void, brecursive: super::super::Foundation::VARIANT_BOOL, pproleassignments: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IAzTask2_Impl::RoleAssignments(this, core::mem::transmute(&bstrscopename), core::mem::transmute_copy(&brecursive)) {
                    Ok(ok__) => {
                        pproleassignments.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self { base__: IAzTask_Vtbl::new::<Identity, OFFSET>(), RoleAssignments: RoleAssignments::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IAzTask2 as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID || iid == &<IAzTask as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for IAzTask2 {}
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
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn get_Item(&self, index: i32) -> windows_core::Result<super::super::System::Variant::VARIANT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_Item)(windows_core::Interface::as_raw(self), index, &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn Count(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Count)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn _NewEnum(&self) -> windows_core::Result<windows_core::IUnknown> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self)._NewEnum)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IAzTasks_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub get_Item: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut super::super::System::Variant::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    get_Item: usize,
    pub Count: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub _NewEnum: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IAzTasks_Impl: super::super::System::Com::IDispatch_Impl {
    fn get_Item(&self, index: i32) -> windows_core::Result<super::super::System::Variant::VARIANT>;
    fn Count(&self) -> windows_core::Result<i32>;
    fn _NewEnum(&self) -> windows_core::Result<windows_core::IUnknown>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IAzTasks_Vtbl {
    pub const fn new<Identity: IAzTasks_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn get_Item<Identity: IAzTasks_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: i32, pvarobtptr: *mut super::super::System::Variant::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IAzTasks_Impl::get_Item(this, core::mem::transmute_copy(&index)) {
                    Ok(ok__) => {
                        pvarobtptr.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Count<Identity: IAzTasks_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, plcount: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IAzTasks_Impl::Count(this) {
                    Ok(ok__) => {
                        plcount.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn _NewEnum<Identity: IAzTasks_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppenumptr: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IAzTasks_Impl::_NewEnum(this) {
                    Ok(ok__) => {
                        ppenumptr.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            get_Item: get_Item::<Identity, OFFSET>,
            Count: Count::<Identity, OFFSET>,
            _NewEnum: _NewEnum::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IAzTasks as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for IAzTasks {}
pub const INHERITED_ACCESS_ENTRY: u32 = 16u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct INHERITED_FROMA {
    pub GenerationGap: i32,
    pub AncestorName: windows_core::PSTR,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct INHERITED_FROMW {
    pub GenerationGap: i32,
    pub AncestorName: windows_core::PWSTR,
}
pub const INHERITED_GRANDPARENT: u32 = 536870912u32;
pub const INHERITED_PARENT: u32 = 268435456u32;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct MULTIPLE_TRUSTEE_OPERATION(pub i32);
pub const NOT_USED_ACCESS: ACCESS_MODE = ACCESS_MODE(0i32);
pub const NO_MULTIPLE_TRUSTEE: MULTIPLE_TRUSTEE_OPERATION = MULTIPLE_TRUSTEE_OPERATION(0i32);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct OBJECTS_AND_NAME_A {
    pub ObjectsPresent: super::SYSTEM_AUDIT_OBJECT_ACE_FLAGS,
    pub ObjectType: SE_OBJECT_TYPE,
    pub ObjectTypeName: windows_core::PSTR,
    pub InheritedObjectTypeName: windows_core::PSTR,
    pub ptstrName: windows_core::PSTR,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct OBJECTS_AND_NAME_W {
    pub ObjectsPresent: super::SYSTEM_AUDIT_OBJECT_ACE_FLAGS,
    pub ObjectType: SE_OBJECT_TYPE,
    pub ObjectTypeName: windows_core::PWSTR,
    pub InheritedObjectTypeName: windows_core::PWSTR,
    pub ptstrName: windows_core::PWSTR,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct OBJECTS_AND_SID {
    pub ObjectsPresent: super::SYSTEM_AUDIT_OBJECT_ACE_FLAGS,
    pub ObjectTypeGuid: windows_core::GUID,
    pub InheritedObjectTypeGuid: windows_core::GUID,
    pub pSid: *mut super::SID,
}
impl Default for OBJECTS_AND_SID {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const OLESCRIPT_E_SYNTAX: windows_core::HRESULT = windows_core::HRESULT(0x80020101_u32 as _);
pub type PFN_AUTHZ_COMPUTE_DYNAMIC_GROUPS = Option<unsafe extern "system" fn(hauthzclientcontext: AUTHZ_CLIENT_CONTEXT_HANDLE, args: *const core::ffi::c_void, psidattrarray: *mut *mut super::SID_AND_ATTRIBUTES, psidcount: *mut u32, prestrictedsidattrarray: *mut *mut super::SID_AND_ATTRIBUTES, prestrictedsidcount: *mut u32) -> windows_core::BOOL>;
pub type PFN_AUTHZ_DYNAMIC_ACCESS_CHECK = Option<unsafe extern "system" fn(hauthzclientcontext: AUTHZ_CLIENT_CONTEXT_HANDLE, pace: *const super::ACE_HEADER, pargs: *const core::ffi::c_void, pbaceapplicable: *mut windows_core::BOOL) -> windows_core::BOOL>;
pub type PFN_AUTHZ_FREE_CENTRAL_ACCESS_POLICY = Option<unsafe extern "system" fn(pcentralaccesspolicy: *const core::ffi::c_void)>;
pub type PFN_AUTHZ_FREE_DYNAMIC_GROUPS = Option<unsafe extern "system" fn(psidattrarray: *const super::SID_AND_ATTRIBUTES)>;
pub type PFN_AUTHZ_GET_CENTRAL_ACCESS_POLICY = Option<unsafe extern "system" fn(hauthzclientcontext: AUTHZ_CLIENT_CONTEXT_HANDLE, capid: super::PSID, pargs: *const core::ffi::c_void, pcentralaccesspolicyapplicable: *mut windows_core::BOOL, ppcentralaccesspolicy: *mut *mut core::ffi::c_void) -> windows_core::BOOL>;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct PROG_INVOKE_SETTING(pub i32);
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
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct SE_OBJECT_TYPE(pub i32);
pub const SE_PRINTER: SE_OBJECT_TYPE = SE_OBJECT_TYPE(3i32);
pub const SE_PROVIDER_DEFINED_OBJECT: SE_OBJECT_TYPE = SE_OBJECT_TYPE(10i32);
pub const SE_REGISTRY_KEY: SE_OBJECT_TYPE = SE_OBJECT_TYPE(4i32);
pub const SE_REGISTRY_WOW64_32KEY: SE_OBJECT_TYPE = SE_OBJECT_TYPE(12i32);
pub const SE_REGISTRY_WOW64_64KEY: SE_OBJECT_TYPE = SE_OBJECT_TYPE(13i32);
pub const SE_SERVICE: SE_OBJECT_TYPE = SE_OBJECT_TYPE(2i32);
pub const SE_UNKNOWN_OBJECT_TYPE: SE_OBJECT_TYPE = SE_OBJECT_TYPE(0i32);
pub const SE_WINDOW_OBJECT: SE_OBJECT_TYPE = SE_OBJECT_TYPE(7i32);
pub const SE_WMIGUID_OBJECT: SE_OBJECT_TYPE = SE_OBJECT_TYPE(11i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct TREE_SEC_INFO(pub u32);
pub const TREE_SEC_INFO_RESET: TREE_SEC_INFO = TREE_SEC_INFO(2u32);
pub const TREE_SEC_INFO_RESET_KEEP_EXPLICIT: TREE_SEC_INFO = TREE_SEC_INFO(3u32);
pub const TREE_SEC_INFO_SET: TREE_SEC_INFO = TREE_SEC_INFO(1u32);
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct TRUSTEE_A {
    pub pMultipleTrustee: *mut TRUSTEE_A,
    pub MultipleTrusteeOperation: MULTIPLE_TRUSTEE_OPERATION,
    pub TrusteeForm: TRUSTEE_FORM,
    pub TrusteeType: TRUSTEE_TYPE,
    pub ptstrName: windows_core::PSTR,
}
impl Default for TRUSTEE_A {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct TRUSTEE_ACCESSA {
    pub lpProperty: windows_core::PSTR,
    pub Access: u32,
    pub fAccessFlags: u32,
    pub fReturnedAccess: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct TRUSTEE_ACCESSW {
    pub lpProperty: windows_core::PWSTR,
    pub Access: u32,
    pub fAccessFlags: u32,
    pub fReturnedAccess: u32,
}
pub const TRUSTEE_ACCESS_ALL: i32 = -1i32;
pub const TRUSTEE_ACCESS_ALLOWED: i32 = 1i32;
pub const TRUSTEE_ACCESS_EXPLICIT: i32 = 1i32;
pub const TRUSTEE_ACCESS_READ: i32 = 2i32;
pub const TRUSTEE_ACCESS_WRITE: i32 = 4i32;
pub const TRUSTEE_BAD_FORM: TRUSTEE_FORM = TRUSTEE_FORM(2i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct TRUSTEE_FORM(pub i32);
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
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct TRUSTEE_TYPE(pub i32);
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct TRUSTEE_W {
    pub pMultipleTrustee: *mut TRUSTEE_W,
    pub MultipleTrusteeOperation: MULTIPLE_TRUSTEE_OPERATION,
    pub TrusteeForm: TRUSTEE_FORM,
    pub TrusteeType: TRUSTEE_TYPE,
    pub ptstrName: windows_core::PWSTR,
}
impl Default for TRUSTEE_W {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const _AUTHZ_SS_MAXSIZE: u32 = 128u32;
