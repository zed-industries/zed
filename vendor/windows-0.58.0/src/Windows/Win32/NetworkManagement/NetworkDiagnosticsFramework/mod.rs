#[inline]
pub unsafe fn NdfCancelIncident(handle: *const core::ffi::c_void) -> windows_core::Result<()> {
    windows_targets::link!("ndfapi.dll" "system" fn NdfCancelIncident(handle : *const core::ffi::c_void) -> windows_core::HRESULT);
    NdfCancelIncident(handle).ok()
}
#[inline]
pub unsafe fn NdfCloseIncident(handle: *mut core::ffi::c_void) -> windows_core::Result<()> {
    windows_targets::link!("ndfapi.dll" "system" fn NdfCloseIncident(handle : *mut core::ffi::c_void) -> windows_core::HRESULT);
    NdfCloseIncident(handle).ok()
}
#[inline]
pub unsafe fn NdfCreateConnectivityIncident(handle: *mut *mut core::ffi::c_void) -> windows_core::Result<()> {
    windows_targets::link!("ndfapi.dll" "system" fn NdfCreateConnectivityIncident(handle : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    NdfCreateConnectivityIncident(handle).ok()
}
#[inline]
pub unsafe fn NdfCreateDNSIncident<P0>(hostname: P0, querytype: u16, handle: *mut *mut core::ffi::c_void) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("ndfapi.dll" "system" fn NdfCreateDNSIncident(hostname : windows_core::PCWSTR, querytype : u16, handle : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    NdfCreateDNSIncident(hostname.param().abi(), querytype, handle).ok()
}
#[cfg(feature = "Win32_Networking_WinSock")]
#[inline]
pub unsafe fn NdfCreateGroupingIncident<P0, P1, P2, P3, P4>(cloudname: P0, groupname: P1, identity: P2, invitation: P3, addresses: Option<*const super::super::Networking::WinSock::SOCKET_ADDRESS_LIST>, appid: P4, handle: *mut *mut core::ffi::c_void) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
    P3: windows_core::Param<windows_core::PCWSTR>,
    P4: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("ndfapi.dll" "system" fn NdfCreateGroupingIncident(cloudname : windows_core::PCWSTR, groupname : windows_core::PCWSTR, identity : windows_core::PCWSTR, invitation : windows_core::PCWSTR, addresses : *const super::super::Networking::WinSock:: SOCKET_ADDRESS_LIST, appid : windows_core::PCWSTR, handle : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    NdfCreateGroupingIncident(cloudname.param().abi(), groupname.param().abi(), identity.param().abi(), invitation.param().abi(), core::mem::transmute(addresses.unwrap_or(std::ptr::null())), appid.param().abi(), handle).ok()
}
#[inline]
pub unsafe fn NdfCreateIncident<P0>(helperclassname: P0, attributes: &[HELPER_ATTRIBUTE], handle: *mut *mut core::ffi::c_void) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("ndfapi.dll" "system" fn NdfCreateIncident(helperclassname : windows_core::PCWSTR, celt : u32, attributes : *const HELPER_ATTRIBUTE, handle : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    NdfCreateIncident(helperclassname.param().abi(), attributes.len().try_into().unwrap(), core::mem::transmute(attributes.as_ptr()), handle).ok()
}
#[inline]
pub unsafe fn NdfCreateNetConnectionIncident(handle: *mut *mut core::ffi::c_void, id: windows_core::GUID) -> windows_core::Result<()> {
    windows_targets::link!("ndfapi.dll" "system" fn NdfCreateNetConnectionIncident(handle : *mut *mut core::ffi::c_void, id : windows_core::GUID) -> windows_core::HRESULT);
    NdfCreateNetConnectionIncident(handle, core::mem::transmute(id)).ok()
}
#[inline]
pub unsafe fn NdfCreatePnrpIncident<P0, P1, P2, P3>(cloudname: P0, peername: P1, diagnosepublish: P2, appid: P3, handle: *mut *mut core::ffi::c_void) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<super::super::Foundation::BOOL>,
    P3: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("ndfapi.dll" "system" fn NdfCreatePnrpIncident(cloudname : windows_core::PCWSTR, peername : windows_core::PCWSTR, diagnosepublish : super::super::Foundation:: BOOL, appid : windows_core::PCWSTR, handle : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    NdfCreatePnrpIncident(cloudname.param().abi(), peername.param().abi(), diagnosepublish.param().abi(), appid.param().abi(), handle).ok()
}
#[inline]
pub unsafe fn NdfCreateSharingIncident<P0>(uncpath: P0, handle: *mut *mut core::ffi::c_void) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("ndfapi.dll" "system" fn NdfCreateSharingIncident(uncpath : windows_core::PCWSTR, handle : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    NdfCreateSharingIncident(uncpath.param().abi(), handle).ok()
}
#[inline]
pub unsafe fn NdfCreateWebIncident<P0>(url: P0, handle: *mut *mut core::ffi::c_void) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("ndfapi.dll" "system" fn NdfCreateWebIncident(url : windows_core::PCWSTR, handle : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    NdfCreateWebIncident(url.param().abi(), handle).ok()
}
#[inline]
pub unsafe fn NdfCreateWebIncidentEx<P0, P1, P2>(url: P0, usewinhttp: P1, modulename: P2, handle: *mut *mut core::ffi::c_void) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<super::super::Foundation::BOOL>,
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("ndfapi.dll" "system" fn NdfCreateWebIncidentEx(url : windows_core::PCWSTR, usewinhttp : super::super::Foundation:: BOOL, modulename : windows_core::PCWSTR, handle : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    NdfCreateWebIncidentEx(url.param().abi(), usewinhttp.param().abi(), modulename.param().abi(), handle).ok()
}
#[cfg(all(feature = "Win32_Networking_WinSock", feature = "Win32_Security"))]
#[inline]
pub unsafe fn NdfCreateWinSockIncident<P0, P1, P2>(sock: P0, host: P1, port: u16, appid: P2, userid: Option<*const super::super::Security::SID>, handle: *mut *mut core::ffi::c_void) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::super::Networking::WinSock::SOCKET>,
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("ndfapi.dll" "system" fn NdfCreateWinSockIncident(sock : super::super::Networking::WinSock:: SOCKET, host : windows_core::PCWSTR, port : u16, appid : windows_core::PCWSTR, userid : *const super::super::Security:: SID, handle : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    NdfCreateWinSockIncident(sock.param().abi(), host.param().abi(), port, appid.param().abi(), core::mem::transmute(userid.unwrap_or(std::ptr::null())), handle).ok()
}
#[inline]
pub unsafe fn NdfDiagnoseIncident(handle: *const core::ffi::c_void, rootcausecount: *mut u32, rootcauses: *mut *mut RootCauseInfo, dwwait: u32, dwflags: u32) -> windows_core::Result<()> {
    windows_targets::link!("ndfapi.dll" "system" fn NdfDiagnoseIncident(handle : *const core::ffi::c_void, rootcausecount : *mut u32, rootcauses : *mut *mut RootCauseInfo, dwwait : u32, dwflags : u32) -> windows_core::HRESULT);
    NdfDiagnoseIncident(handle, rootcausecount, rootcauses, dwwait, dwflags).ok()
}
#[inline]
pub unsafe fn NdfExecuteDiagnosis<P0>(handle: *const core::ffi::c_void, hwnd: P0) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::super::Foundation::HWND>,
{
    windows_targets::link!("ndfapi.dll" "system" fn NdfExecuteDiagnosis(handle : *const core::ffi::c_void, hwnd : super::super::Foundation:: HWND) -> windows_core::HRESULT);
    NdfExecuteDiagnosis(handle, hwnd.param().abi()).ok()
}
#[inline]
pub unsafe fn NdfGetTraceFile(handle: *const core::ffi::c_void) -> windows_core::Result<windows_core::PCWSTR> {
    windows_targets::link!("ndfapi.dll" "system" fn NdfGetTraceFile(handle : *const core::ffi::c_void, tracefilelocation : *mut windows_core::PCWSTR) -> windows_core::HRESULT);
    let mut result__ = core::mem::zeroed();
    NdfGetTraceFile(handle, &mut result__).map(|| result__)
}
#[inline]
pub unsafe fn NdfRepairIncident(handle: *const core::ffi::c_void, repairex: *const RepairInfoEx, dwwait: u32) -> windows_core::Result<()> {
    windows_targets::link!("ndfapi.dll" "system" fn NdfRepairIncident(handle : *const core::ffi::c_void, repairex : *const RepairInfoEx, dwwait : u32) -> windows_core::HRESULT);
    NdfRepairIncident(handle, repairex, dwwait).ok()
}
windows_core::imp::define_interface!(INetDiagExtensibleHelper, INetDiagExtensibleHelper_Vtbl, 0xc0b35748_ebf5_11d8_bbe9_505054503030);
impl core::ops::Deref for INetDiagExtensibleHelper {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(INetDiagExtensibleHelper, windows_core::IUnknown);
impl INetDiagExtensibleHelper {
    pub unsafe fn ResolveAttributes(&self, rgkeyattributes: &[HELPER_ATTRIBUTE], pcelt: *mut u32, prgmatchvalues: *mut *mut HELPER_ATTRIBUTE) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).ResolveAttributes)(windows_core::Interface::as_raw(self), rgkeyattributes.len().try_into().unwrap(), core::mem::transmute(rgkeyattributes.as_ptr()), pcelt, prgmatchvalues).ok()
    }
}
#[repr(C)]
pub struct INetDiagExtensibleHelper_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub ResolveAttributes: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const HELPER_ATTRIBUTE, *mut u32, *mut *mut HELPER_ATTRIBUTE) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(INetDiagHelper, INetDiagHelper_Vtbl, 0xc0b35746_ebf5_11d8_bbe9_505054503030);
impl core::ops::Deref for INetDiagHelper {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(INetDiagHelper, windows_core::IUnknown);
impl INetDiagHelper {
    pub unsafe fn Initialize(&self, rgattributes: &[HELPER_ATTRIBUTE]) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Initialize)(windows_core::Interface::as_raw(self), rgattributes.len().try_into().unwrap(), core::mem::transmute(rgattributes.as_ptr())).ok()
    }
    pub unsafe fn GetDiagnosticsInfo(&self) -> windows_core::Result<*mut DiagnosticsInfo> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetDiagnosticsInfo)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn GetKeyAttributes(&self, pcelt: *mut u32, pprgattributes: *mut *mut HELPER_ATTRIBUTE) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetKeyAttributes)(windows_core::Interface::as_raw(self), pcelt, pprgattributes).ok()
    }
    pub unsafe fn LowHealth<P0>(&self, pwszinstancedescription: P0, ppwszdescription: *mut windows_core::PWSTR, pdeferredtime: *mut i32, pstatus: *mut DIAGNOSIS_STATUS) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).LowHealth)(windows_core::Interface::as_raw(self), pwszinstancedescription.param().abi(), ppwszdescription, pdeferredtime, pstatus).ok()
    }
    pub unsafe fn HighUtilization<P0>(&self, pwszinstancedescription: P0, ppwszdescription: *mut windows_core::PWSTR, pdeferredtime: *mut i32, pstatus: *mut DIAGNOSIS_STATUS) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).HighUtilization)(windows_core::Interface::as_raw(self), pwszinstancedescription.param().abi(), ppwszdescription, pdeferredtime, pstatus).ok()
    }
    pub unsafe fn GetLowerHypotheses(&self, pcelt: *mut u32, pprghypotheses: *mut *mut HYPOTHESIS) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetLowerHypotheses)(windows_core::Interface::as_raw(self), pcelt, pprghypotheses).ok()
    }
    pub unsafe fn GetDownStreamHypotheses(&self, pcelt: *mut u32, pprghypotheses: *mut *mut HYPOTHESIS) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetDownStreamHypotheses)(windows_core::Interface::as_raw(self), pcelt, pprghypotheses).ok()
    }
    pub unsafe fn GetHigherHypotheses(&self, pcelt: *mut u32, pprghypotheses: *mut *mut HYPOTHESIS) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetHigherHypotheses)(windows_core::Interface::as_raw(self), pcelt, pprghypotheses).ok()
    }
    pub unsafe fn GetUpStreamHypotheses(&self, pcelt: *mut u32, pprghypotheses: *mut *mut HYPOTHESIS) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetUpStreamHypotheses)(windows_core::Interface::as_raw(self), pcelt, pprghypotheses).ok()
    }
    pub unsafe fn Repair(&self, pinfo: *const RepairInfo, pdeferredtime: *mut i32, pstatus: *mut REPAIR_STATUS) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Repair)(windows_core::Interface::as_raw(self), pinfo, pdeferredtime, pstatus).ok()
    }
    pub unsafe fn Validate(&self, problem: PROBLEM_TYPE, pdeferredtime: *mut i32, pstatus: *mut REPAIR_STATUS) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Validate)(windows_core::Interface::as_raw(self), problem, pdeferredtime, pstatus).ok()
    }
    pub unsafe fn GetRepairInfo(&self, problem: PROBLEM_TYPE, pcelt: *mut u32, ppinfo: *mut *mut RepairInfo) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetRepairInfo)(windows_core::Interface::as_raw(self), problem, pcelt, ppinfo).ok()
    }
    pub unsafe fn GetLifeTime(&self) -> windows_core::Result<LIFE_TIME> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetLifeTime)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetLifeTime(&self, lifetime: LIFE_TIME) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetLifeTime)(windows_core::Interface::as_raw(self), core::mem::transmute(lifetime)).ok()
    }
    pub unsafe fn GetCacheTime(&self) -> windows_core::Result<super::super::Foundation::FILETIME> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetCacheTime)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn GetAttributes(&self, pcelt: *mut u32, pprgattributes: *mut *mut HELPER_ATTRIBUTE) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetAttributes)(windows_core::Interface::as_raw(self), pcelt, pprgattributes).ok()
    }
    pub unsafe fn Cancel(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Cancel)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Cleanup(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Cleanup)(windows_core::Interface::as_raw(self)).ok()
    }
}
#[repr(C)]
pub struct INetDiagHelper_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Initialize: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const HELPER_ATTRIBUTE) -> windows_core::HRESULT,
    pub GetDiagnosticsInfo: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut DiagnosticsInfo) -> windows_core::HRESULT,
    pub GetKeyAttributes: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32, *mut *mut HELPER_ATTRIBUTE) -> windows_core::HRESULT,
    pub LowHealth: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, *mut windows_core::PWSTR, *mut i32, *mut DIAGNOSIS_STATUS) -> windows_core::HRESULT,
    pub HighUtilization: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, *mut windows_core::PWSTR, *mut i32, *mut DIAGNOSIS_STATUS) -> windows_core::HRESULT,
    pub GetLowerHypotheses: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32, *mut *mut HYPOTHESIS) -> windows_core::HRESULT,
    pub GetDownStreamHypotheses: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32, *mut *mut HYPOTHESIS) -> windows_core::HRESULT,
    pub GetHigherHypotheses: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32, *mut *mut HYPOTHESIS) -> windows_core::HRESULT,
    pub GetUpStreamHypotheses: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32, *mut *mut HYPOTHESIS) -> windows_core::HRESULT,
    pub Repair: unsafe extern "system" fn(*mut core::ffi::c_void, *const RepairInfo, *mut i32, *mut REPAIR_STATUS) -> windows_core::HRESULT,
    pub Validate: unsafe extern "system" fn(*mut core::ffi::c_void, PROBLEM_TYPE, *mut i32, *mut REPAIR_STATUS) -> windows_core::HRESULT,
    pub GetRepairInfo: unsafe extern "system" fn(*mut core::ffi::c_void, PROBLEM_TYPE, *mut u32, *mut *mut RepairInfo) -> windows_core::HRESULT,
    pub GetLifeTime: unsafe extern "system" fn(*mut core::ffi::c_void, *mut LIFE_TIME) -> windows_core::HRESULT,
    pub SetLifeTime: unsafe extern "system" fn(*mut core::ffi::c_void, LIFE_TIME) -> windows_core::HRESULT,
    pub GetCacheTime: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::FILETIME) -> windows_core::HRESULT,
    pub GetAttributes: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32, *mut *mut HELPER_ATTRIBUTE) -> windows_core::HRESULT,
    pub Cancel: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Cleanup: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(INetDiagHelperEx, INetDiagHelperEx_Vtbl, 0x972dab4d_e4e3_4fc6_ae54_5f65ccde4a15);
impl core::ops::Deref for INetDiagHelperEx {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(INetDiagHelperEx, windows_core::IUnknown);
impl INetDiagHelperEx {
    pub unsafe fn ReconfirmLowHealth(&self, presults: &[HypothesisResult], ppwszupdateddescription: *mut windows_core::PWSTR, pupdatedstatus: *mut DIAGNOSIS_STATUS) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).ReconfirmLowHealth)(windows_core::Interface::as_raw(self), presults.len().try_into().unwrap(), core::mem::transmute(presults.as_ptr()), ppwszupdateddescription, pupdatedstatus).ok()
    }
    pub unsafe fn SetUtilities<P0>(&self, putilities: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<INetDiagHelperUtilFactory>,
    {
        (windows_core::Interface::vtable(self).SetUtilities)(windows_core::Interface::as_raw(self), putilities.param().abi()).ok()
    }
    pub unsafe fn ReproduceFailure(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).ReproduceFailure)(windows_core::Interface::as_raw(self)).ok()
    }
}
#[repr(C)]
pub struct INetDiagHelperEx_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub ReconfirmLowHealth: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const HypothesisResult, *mut windows_core::PWSTR, *mut DIAGNOSIS_STATUS) -> windows_core::HRESULT,
    pub SetUtilities: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ReproduceFailure: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(INetDiagHelperInfo, INetDiagHelperInfo_Vtbl, 0xc0b35747_ebf5_11d8_bbe9_505054503030);
impl core::ops::Deref for INetDiagHelperInfo {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(INetDiagHelperInfo, windows_core::IUnknown);
impl INetDiagHelperInfo {
    pub unsafe fn GetAttributeInfo(&self, pcelt: *mut u32, pprgattributeinfos: *mut *mut HelperAttributeInfo) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetAttributeInfo)(windows_core::Interface::as_raw(self), pcelt, pprgattributeinfos).ok()
    }
}
#[repr(C)]
pub struct INetDiagHelperInfo_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetAttributeInfo: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32, *mut *mut HelperAttributeInfo) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(INetDiagHelperUtilFactory, INetDiagHelperUtilFactory_Vtbl, 0x104613fb_bc57_4178_95ba_88809698354a);
impl core::ops::Deref for INetDiagHelperUtilFactory {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(INetDiagHelperUtilFactory, windows_core::IUnknown);
impl INetDiagHelperUtilFactory {
    pub unsafe fn CreateUtilityInstance<T>(&self) -> windows_core::Result<T>
    where
        T: windows_core::Interface,
    {
        let mut result__ = core::ptr::null_mut();
        (windows_core::Interface::vtable(self).CreateUtilityInstance)(windows_core::Interface::as_raw(self), &T::IID, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[repr(C)]
pub struct INetDiagHelperUtilFactory_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub CreateUtilityInstance: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub const AT_BOOLEAN: ATTRIBUTE_TYPE = ATTRIBUTE_TYPE(1i32);
pub const AT_GUID: ATTRIBUTE_TYPE = ATTRIBUTE_TYPE(11i32);
pub const AT_INT16: ATTRIBUTE_TYPE = ATTRIBUTE_TYPE(4i32);
pub const AT_INT32: ATTRIBUTE_TYPE = ATTRIBUTE_TYPE(6i32);
pub const AT_INT64: ATTRIBUTE_TYPE = ATTRIBUTE_TYPE(8i32);
pub const AT_INT8: ATTRIBUTE_TYPE = ATTRIBUTE_TYPE(2i32);
pub const AT_INVALID: ATTRIBUTE_TYPE = ATTRIBUTE_TYPE(0i32);
pub const AT_LIFE_TIME: ATTRIBUTE_TYPE = ATTRIBUTE_TYPE(12i32);
pub const AT_OCTET_STRING: ATTRIBUTE_TYPE = ATTRIBUTE_TYPE(14i32);
pub const AT_SOCKADDR: ATTRIBUTE_TYPE = ATTRIBUTE_TYPE(13i32);
pub const AT_STRING: ATTRIBUTE_TYPE = ATTRIBUTE_TYPE(10i32);
pub const AT_UINT16: ATTRIBUTE_TYPE = ATTRIBUTE_TYPE(5i32);
pub const AT_UINT32: ATTRIBUTE_TYPE = ATTRIBUTE_TYPE(7i32);
pub const AT_UINT64: ATTRIBUTE_TYPE = ATTRIBUTE_TYPE(9i32);
pub const AT_UINT8: ATTRIBUTE_TYPE = ATTRIBUTE_TYPE(3i32);
pub const DF_IMPERSONATION: u32 = 2147483648u32;
pub const DF_TRACELESS: u32 = 1073741824u32;
pub const DS_CONFIRMED: DIAGNOSIS_STATUS = DIAGNOSIS_STATUS(1i32);
pub const DS_DEFERRED: DIAGNOSIS_STATUS = DIAGNOSIS_STATUS(4i32);
pub const DS_INDETERMINATE: DIAGNOSIS_STATUS = DIAGNOSIS_STATUS(3i32);
pub const DS_NOT_IMPLEMENTED: DIAGNOSIS_STATUS = DIAGNOSIS_STATUS(0i32);
pub const DS_PASSTHROUGH: DIAGNOSIS_STATUS = DIAGNOSIS_STATUS(5i32);
pub const DS_REJECTED: DIAGNOSIS_STATUS = DIAGNOSIS_STATUS(2i32);
pub const NDF_ADD_CAPTURE_TRACE: u32 = 1u32;
pub const NDF_APPLY_INCLUSION_LIST_FILTER: u32 = 2u32;
pub const NDF_ERROR_START: u32 = 63744u32;
pub const NDF_E_BAD_PARAM: windows_core::HRESULT = windows_core::HRESULT(0x8008F905_u32 as _);
pub const NDF_E_CANCELLED: windows_core::HRESULT = windows_core::HRESULT(0x8008F902_u32 as _);
pub const NDF_E_DISABLED: windows_core::HRESULT = windows_core::HRESULT(0x8008F904_u32 as _);
pub const NDF_E_LENGTH_EXCEEDED: windows_core::HRESULT = windows_core::HRESULT(0x8008F900_u32 as _);
pub const NDF_E_NOHELPERCLASS: windows_core::HRESULT = windows_core::HRESULT(0x8008F901_u32 as _);
pub const NDF_E_PROBLEM_PRESENT: windows_core::HRESULT = windows_core::HRESULT(0x8008F908_u32 as _);
pub const NDF_E_UNKNOWN: windows_core::HRESULT = windows_core::HRESULT(0x8008F907_u32 as _);
pub const NDF_E_VALIDATION: windows_core::HRESULT = windows_core::HRESULT(0x8008F906_u32 as _);
pub const NDF_INBOUND_FLAG_EDGETRAVERSAL: u32 = 1u32;
pub const NDF_INBOUND_FLAG_HEALTHCHECK: u32 = 2u32;
pub const PT_DOWN_STREAM_HEALTH: PROBLEM_TYPE = PROBLEM_TYPE(4i32);
pub const PT_HIGHER_UTILIZATION: PROBLEM_TYPE = PROBLEM_TYPE(16i32);
pub const PT_HIGH_UTILIZATION: PROBLEM_TYPE = PROBLEM_TYPE(8i32);
pub const PT_INVALID: PROBLEM_TYPE = PROBLEM_TYPE(0i32);
pub const PT_LOWER_HEALTH: PROBLEM_TYPE = PROBLEM_TYPE(2i32);
pub const PT_LOW_HEALTH: PROBLEM_TYPE = PROBLEM_TYPE(1i32);
pub const PT_UP_STREAM_UTILIZATION: PROBLEM_TYPE = PROBLEM_TYPE(32i32);
pub const RCF_ISCONFIRMED: u32 = 2u32;
pub const RCF_ISLEAF: u32 = 1u32;
pub const RCF_ISTHIRDPARTY: u32 = 4u32;
pub const RF_CONTACT_ADMIN: u32 = 131072u32;
pub const RF_INFORMATION_ONLY: u32 = 33554432u32;
pub const RF_REPRO: u32 = 2097152u32;
pub const RF_RESERVED: u32 = 1073741824u32;
pub const RF_RESERVED_CA: u32 = 2147483648u32;
pub const RF_RESERVED_LNI: u32 = 65536u32;
pub const RF_SHOW_EVENTS: u32 = 8388608u32;
pub const RF_UI_ONLY: u32 = 16777216u32;
pub const RF_USER_ACTION: u32 = 268435456u32;
pub const RF_USER_CONFIRMATION: u32 = 134217728u32;
pub const RF_VALIDATE_HELPTOPIC: u32 = 4194304u32;
pub const RF_WORKAROUND: u32 = 536870912u32;
pub const RR_NORISK: REPAIR_RISK = REPAIR_RISK(2i32);
pub const RR_NOROLLBACK: REPAIR_RISK = REPAIR_RISK(0i32);
pub const RR_ROLLBACK: REPAIR_RISK = REPAIR_RISK(1i32);
pub const RS_APPLICATION: REPAIR_SCOPE = REPAIR_SCOPE(2i32);
pub const RS_DEFERRED: REPAIR_STATUS = REPAIR_STATUS(3i32);
pub const RS_NOT_IMPLEMENTED: REPAIR_STATUS = REPAIR_STATUS(0i32);
pub const RS_PROCESS: REPAIR_SCOPE = REPAIR_SCOPE(3i32);
pub const RS_REPAIRED: REPAIR_STATUS = REPAIR_STATUS(1i32);
pub const RS_SYSTEM: REPAIR_SCOPE = REPAIR_SCOPE(0i32);
pub const RS_UNREPAIRED: REPAIR_STATUS = REPAIR_STATUS(2i32);
pub const RS_USER: REPAIR_SCOPE = REPAIR_SCOPE(1i32);
pub const RS_USER_ACTION: REPAIR_STATUS = REPAIR_STATUS(4i32);
pub const UIT_DUI: UI_INFO_TYPE = UI_INFO_TYPE(4i32);
pub const UIT_HELP_PANE: UI_INFO_TYPE = UI_INFO_TYPE(3i32);
pub const UIT_INVALID: UI_INFO_TYPE = UI_INFO_TYPE(0i32);
pub const UIT_NONE: UI_INFO_TYPE = UI_INFO_TYPE(1i32);
pub const UIT_SHELL_COMMAND: UI_INFO_TYPE = UI_INFO_TYPE(2i32);
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct ATTRIBUTE_TYPE(pub i32);
impl windows_core::TypeKind for ATTRIBUTE_TYPE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for ATTRIBUTE_TYPE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("ATTRIBUTE_TYPE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct DIAGNOSIS_STATUS(pub i32);
impl windows_core::TypeKind for DIAGNOSIS_STATUS {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for DIAGNOSIS_STATUS {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("DIAGNOSIS_STATUS").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct PROBLEM_TYPE(pub i32);
impl windows_core::TypeKind for PROBLEM_TYPE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for PROBLEM_TYPE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("PROBLEM_TYPE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct REPAIR_RISK(pub i32);
impl windows_core::TypeKind for REPAIR_RISK {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for REPAIR_RISK {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("REPAIR_RISK").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct REPAIR_SCOPE(pub i32);
impl windows_core::TypeKind for REPAIR_SCOPE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for REPAIR_SCOPE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("REPAIR_SCOPE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct REPAIR_STATUS(pub i32);
impl windows_core::TypeKind for REPAIR_STATUS {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for REPAIR_STATUS {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("REPAIR_STATUS").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct UI_INFO_TYPE(pub i32);
impl windows_core::TypeKind for UI_INFO_TYPE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for UI_INFO_TYPE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("UI_INFO_TYPE").field(&self.0).finish()
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DIAG_SOCKADDR {
    pub family: u16,
    pub data: [i8; 126],
}
impl windows_core::TypeKind for DIAG_SOCKADDR {
    type TypeKind = windows_core::CopyType;
}
impl Default for DIAG_SOCKADDR {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DiagnosticsInfo {
    pub cost: i32,
    pub flags: u32,
}
impl windows_core::TypeKind for DiagnosticsInfo {
    type TypeKind = windows_core::CopyType;
}
impl Default for DiagnosticsInfo {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct HELPER_ATTRIBUTE {
    pub pwszName: windows_core::PWSTR,
    pub r#type: ATTRIBUTE_TYPE,
    pub Anonymous: HELPER_ATTRIBUTE_0,
}
impl windows_core::TypeKind for HELPER_ATTRIBUTE {
    type TypeKind = windows_core::CopyType;
}
impl Default for HELPER_ATTRIBUTE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union HELPER_ATTRIBUTE_0 {
    pub Boolean: super::super::Foundation::BOOL,
    pub Char: u8,
    pub Byte: u8,
    pub Short: i16,
    pub Word: u16,
    pub Int: i32,
    pub DWord: u32,
    pub Int64: i64,
    pub UInt64: u64,
    pub PWStr: windows_core::PWSTR,
    pub Guid: windows_core::GUID,
    pub LifeTime: LIFE_TIME,
    pub Address: DIAG_SOCKADDR,
    pub OctetString: OCTET_STRING,
}
impl windows_core::TypeKind for HELPER_ATTRIBUTE_0 {
    type TypeKind = windows_core::CopyType;
}
impl Default for HELPER_ATTRIBUTE_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct HYPOTHESIS {
    pub pwszClassName: windows_core::PWSTR,
    pub pwszDescription: windows_core::PWSTR,
    pub celt: u32,
    pub rgAttributes: *mut HELPER_ATTRIBUTE,
}
impl windows_core::TypeKind for HYPOTHESIS {
    type TypeKind = windows_core::CopyType;
}
impl Default for HYPOTHESIS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct HelperAttributeInfo {
    pub pwszName: windows_core::PWSTR,
    pub r#type: ATTRIBUTE_TYPE,
}
impl windows_core::TypeKind for HelperAttributeInfo {
    type TypeKind = windows_core::CopyType;
}
impl Default for HelperAttributeInfo {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct HypothesisResult {
    pub hypothesis: HYPOTHESIS,
    pub pathStatus: DIAGNOSIS_STATUS,
}
impl windows_core::TypeKind for HypothesisResult {
    type TypeKind = windows_core::CopyType;
}
impl Default for HypothesisResult {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct LIFE_TIME {
    pub startTime: super::super::Foundation::FILETIME,
    pub endTime: super::super::Foundation::FILETIME,
}
impl windows_core::TypeKind for LIFE_TIME {
    type TypeKind = windows_core::CopyType;
}
impl Default for LIFE_TIME {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct OCTET_STRING {
    pub dwLength: u32,
    pub lpValue: *mut u8,
}
impl windows_core::TypeKind for OCTET_STRING {
    type TypeKind = windows_core::CopyType;
}
impl Default for OCTET_STRING {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct RepairInfo {
    pub guid: windows_core::GUID,
    pub pwszClassName: windows_core::PWSTR,
    pub pwszDescription: windows_core::PWSTR,
    pub sidType: u32,
    pub cost: i32,
    pub flags: u32,
    pub scope: REPAIR_SCOPE,
    pub risk: REPAIR_RISK,
    pub UiInfo: UiInfo,
    pub rootCauseIndex: i32,
}
impl windows_core::TypeKind for RepairInfo {
    type TypeKind = windows_core::CopyType;
}
impl Default for RepairInfo {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct RepairInfoEx {
    pub repair: RepairInfo,
    pub repairRank: u16,
}
impl windows_core::TypeKind for RepairInfoEx {
    type TypeKind = windows_core::CopyType;
}
impl Default for RepairInfoEx {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct RootCauseInfo {
    pub pwszDescription: windows_core::PWSTR,
    pub rootCauseID: windows_core::GUID,
    pub rootCauseFlags: u32,
    pub networkInterfaceID: windows_core::GUID,
    pub pRepairs: *mut RepairInfoEx,
    pub repairCount: u16,
}
impl windows_core::TypeKind for RootCauseInfo {
    type TypeKind = windows_core::CopyType;
}
impl Default for RootCauseInfo {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ShellCommandInfo {
    pub pwszOperation: windows_core::PWSTR,
    pub pwszFile: windows_core::PWSTR,
    pub pwszParameters: windows_core::PWSTR,
    pub pwszDirectory: windows_core::PWSTR,
    pub nShowCmd: u32,
}
impl windows_core::TypeKind for ShellCommandInfo {
    type TypeKind = windows_core::CopyType;
}
impl Default for ShellCommandInfo {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct UiInfo {
    pub r#type: UI_INFO_TYPE,
    pub Anonymous: UiInfo_0,
}
impl windows_core::TypeKind for UiInfo {
    type TypeKind = windows_core::CopyType;
}
impl Default for UiInfo {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union UiInfo_0 {
    pub pwzNull: windows_core::PWSTR,
    pub ShellInfo: ShellCommandInfo,
    pub pwzHelpUrl: windows_core::PWSTR,
    pub pwzDui: windows_core::PWSTR,
}
impl windows_core::TypeKind for UiInfo_0 {
    type TypeKind = windows_core::CopyType;
}
impl Default for UiInfo_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "implement")]
core::include!("impl.rs");
