#[inline]
pub unsafe fn NdfCancelIncident(handle: *const core::ffi::c_void) -> windows_core::Result<()> {
    windows_core::link!("ndfapi.dll" "system" fn NdfCancelIncident(handle : *const core::ffi::c_void) -> windows_core::HRESULT);
    unsafe { NdfCancelIncident(handle).ok() }
}
#[inline]
pub unsafe fn NdfCloseIncident(handle: *mut core::ffi::c_void) -> windows_core::Result<()> {
    windows_core::link!("ndfapi.dll" "system" fn NdfCloseIncident(handle : *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe { NdfCloseIncident(handle as _).ok() }
}
#[inline]
pub unsafe fn NdfCreateConnectivityIncident(handle: *mut *mut core::ffi::c_void) -> windows_core::Result<()> {
    windows_core::link!("ndfapi.dll" "system" fn NdfCreateConnectivityIncident(handle : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe { NdfCreateConnectivityIncident(handle as _).ok() }
}
#[inline]
pub unsafe fn NdfCreateDNSIncident<P0>(hostname: P0, querytype: u16, handle: *mut *mut core::ffi::c_void) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("ndfapi.dll" "system" fn NdfCreateDNSIncident(hostname : windows_core::PCWSTR, querytype : u16, handle : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe { NdfCreateDNSIncident(hostname.param().abi(), querytype, handle as _).ok() }
}
#[cfg(feature = "Win32_Networking_WinSock")]
#[inline]
pub unsafe fn NdfCreateGroupingIncident<P0, P1, P2, P3, P5>(cloudname: P0, groupname: P1, identity: P2, invitation: P3, addresses: Option<*const super::super::Networking::WinSock::SOCKET_ADDRESS_LIST>, appid: P5, handle: *mut *mut core::ffi::c_void) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
    P3: windows_core::Param<windows_core::PCWSTR>,
    P5: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("ndfapi.dll" "system" fn NdfCreateGroupingIncident(cloudname : windows_core::PCWSTR, groupname : windows_core::PCWSTR, identity : windows_core::PCWSTR, invitation : windows_core::PCWSTR, addresses : *const super::super::Networking::WinSock:: SOCKET_ADDRESS_LIST, appid : windows_core::PCWSTR, handle : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe { NdfCreateGroupingIncident(cloudname.param().abi(), groupname.param().abi(), identity.param().abi(), invitation.param().abi(), addresses.unwrap_or(core::mem::zeroed()) as _, appid.param().abi(), handle as _).ok() }
}
#[inline]
pub unsafe fn NdfCreateIncident<P0>(helperclassname: P0, attributes: &[HELPER_ATTRIBUTE], handle: *mut *mut core::ffi::c_void) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("ndfapi.dll" "system" fn NdfCreateIncident(helperclassname : windows_core::PCWSTR, celt : u32, attributes : *const HELPER_ATTRIBUTE, handle : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe { NdfCreateIncident(helperclassname.param().abi(), attributes.len().try_into().unwrap(), core::mem::transmute(attributes.as_ptr()), handle as _).ok() }
}
#[inline]
pub unsafe fn NdfCreateNetConnectionIncident(handle: *mut *mut core::ffi::c_void, id: windows_core::GUID) -> windows_core::Result<()> {
    windows_core::link!("ndfapi.dll" "system" fn NdfCreateNetConnectionIncident(handle : *mut *mut core::ffi::c_void, id : windows_core::GUID) -> windows_core::HRESULT);
    unsafe { NdfCreateNetConnectionIncident(handle as _, core::mem::transmute(id)).ok() }
}
#[inline]
pub unsafe fn NdfCreatePnrpIncident<P0, P1, P3>(cloudname: P0, peername: P1, diagnosepublish: bool, appid: P3, handle: *mut *mut core::ffi::c_void) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
    P3: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("ndfapi.dll" "system" fn NdfCreatePnrpIncident(cloudname : windows_core::PCWSTR, peername : windows_core::PCWSTR, diagnosepublish : windows_core::BOOL, appid : windows_core::PCWSTR, handle : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe { NdfCreatePnrpIncident(cloudname.param().abi(), peername.param().abi(), diagnosepublish.into(), appid.param().abi(), handle as _).ok() }
}
#[inline]
pub unsafe fn NdfCreateSharingIncident<P0>(uncpath: P0, handle: *mut *mut core::ffi::c_void) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("ndfapi.dll" "system" fn NdfCreateSharingIncident(uncpath : windows_core::PCWSTR, handle : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe { NdfCreateSharingIncident(uncpath.param().abi(), handle as _).ok() }
}
#[inline]
pub unsafe fn NdfCreateWebIncident<P0>(url: P0, handle: *mut *mut core::ffi::c_void) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("ndfapi.dll" "system" fn NdfCreateWebIncident(url : windows_core::PCWSTR, handle : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe { NdfCreateWebIncident(url.param().abi(), handle as _).ok() }
}
#[inline]
pub unsafe fn NdfCreateWebIncidentEx<P0, P2>(url: P0, usewinhttp: bool, modulename: P2, handle: *mut *mut core::ffi::c_void) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("ndfapi.dll" "system" fn NdfCreateWebIncidentEx(url : windows_core::PCWSTR, usewinhttp : windows_core::BOOL, modulename : windows_core::PCWSTR, handle : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe { NdfCreateWebIncidentEx(url.param().abi(), usewinhttp.into(), modulename.param().abi(), handle as _).ok() }
}
#[cfg(all(feature = "Win32_Networking_WinSock", feature = "Win32_Security"))]
#[inline]
pub unsafe fn NdfCreateWinSockIncident<P1, P3>(sock: super::super::Networking::WinSock::SOCKET, host: P1, port: u16, appid: P3, userid: Option<*const super::super::Security::SID>, handle: *mut *mut core::ffi::c_void) -> windows_core::Result<()>
where
    P1: windows_core::Param<windows_core::PCWSTR>,
    P3: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("ndfapi.dll" "system" fn NdfCreateWinSockIncident(sock : super::super::Networking::WinSock:: SOCKET, host : windows_core::PCWSTR, port : u16, appid : windows_core::PCWSTR, userid : *const super::super::Security:: SID, handle : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe { NdfCreateWinSockIncident(sock, host.param().abi(), port, appid.param().abi(), userid.unwrap_or(core::mem::zeroed()) as _, handle as _).ok() }
}
#[inline]
pub unsafe fn NdfDiagnoseIncident(handle: *const core::ffi::c_void, rootcausecount: *mut u32, rootcauses: *mut *mut RootCauseInfo, dwwait: u32, dwflags: u32) -> windows_core::Result<()> {
    windows_core::link!("ndfapi.dll" "system" fn NdfDiagnoseIncident(handle : *const core::ffi::c_void, rootcausecount : *mut u32, rootcauses : *mut *mut RootCauseInfo, dwwait : u32, dwflags : u32) -> windows_core::HRESULT);
    unsafe { NdfDiagnoseIncident(handle, rootcausecount as _, rootcauses as _, dwwait, dwflags).ok() }
}
#[inline]
pub unsafe fn NdfExecuteDiagnosis(handle: *const core::ffi::c_void, hwnd: Option<super::super::Foundation::HWND>) -> windows_core::Result<()> {
    windows_core::link!("ndfapi.dll" "system" fn NdfExecuteDiagnosis(handle : *const core::ffi::c_void, hwnd : super::super::Foundation:: HWND) -> windows_core::HRESULT);
    unsafe { NdfExecuteDiagnosis(handle, hwnd.unwrap_or(core::mem::zeroed()) as _).ok() }
}
#[inline]
pub unsafe fn NdfGetTraceFile(handle: *const core::ffi::c_void) -> windows_core::Result<windows_core::PCWSTR> {
    windows_core::link!("ndfapi.dll" "system" fn NdfGetTraceFile(handle : *const core::ffi::c_void, tracefilelocation : *mut windows_core::PCWSTR) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        NdfGetTraceFile(handle, &mut result__).map(|| result__)
    }
}
#[inline]
pub unsafe fn NdfRepairIncident(handle: *const core::ffi::c_void, repairex: *const RepairInfoEx, dwwait: u32) -> windows_core::Result<()> {
    windows_core::link!("ndfapi.dll" "system" fn NdfRepairIncident(handle : *const core::ffi::c_void, repairex : *const RepairInfoEx, dwwait : u32) -> windows_core::HRESULT);
    unsafe { NdfRepairIncident(handle, repairex, dwwait).ok() }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct ATTRIBUTE_TYPE(pub i32);
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
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct DIAGNOSIS_STATUS(pub i32);
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DIAG_SOCKADDR {
    pub family: u16,
    pub data: [i8; 126],
}
impl Default for DIAG_SOCKADDR {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const DS_CONFIRMED: DIAGNOSIS_STATUS = DIAGNOSIS_STATUS(1i32);
pub const DS_DEFERRED: DIAGNOSIS_STATUS = DIAGNOSIS_STATUS(4i32);
pub const DS_INDETERMINATE: DIAGNOSIS_STATUS = DIAGNOSIS_STATUS(3i32);
pub const DS_NOT_IMPLEMENTED: DIAGNOSIS_STATUS = DIAGNOSIS_STATUS(0i32);
pub const DS_PASSTHROUGH: DIAGNOSIS_STATUS = DIAGNOSIS_STATUS(5i32);
pub const DS_REJECTED: DIAGNOSIS_STATUS = DIAGNOSIS_STATUS(2i32);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DiagnosticsInfo {
    pub cost: i32,
    pub flags: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct HELPER_ATTRIBUTE {
    pub pwszName: windows_core::PWSTR,
    pub r#type: ATTRIBUTE_TYPE,
    pub Anonymous: HELPER_ATTRIBUTE_0,
}
impl Default for HELPER_ATTRIBUTE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union HELPER_ATTRIBUTE_0 {
    pub Boolean: windows_core::BOOL,
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
impl Default for HELPER_ATTRIBUTE_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct HYPOTHESIS {
    pub pwszClassName: windows_core::PWSTR,
    pub pwszDescription: windows_core::PWSTR,
    pub celt: u32,
    pub rgAttributes: *mut HELPER_ATTRIBUTE,
}
impl Default for HYPOTHESIS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct HelperAttributeInfo {
    pub pwszName: windows_core::PWSTR,
    pub r#type: ATTRIBUTE_TYPE,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct HypothesisResult {
    pub hypothesis: HYPOTHESIS,
    pub pathStatus: DIAGNOSIS_STATUS,
}
windows_core::imp::define_interface!(INetDiagExtensibleHelper, INetDiagExtensibleHelper_Vtbl, 0xc0b35748_ebf5_11d8_bbe9_505054503030);
windows_core::imp::interface_hierarchy!(INetDiagExtensibleHelper, windows_core::IUnknown);
impl INetDiagExtensibleHelper {
    pub unsafe fn ResolveAttributes(&self, rgkeyattributes: &[HELPER_ATTRIBUTE], pcelt: *mut u32, prgmatchvalues: *mut *mut HELPER_ATTRIBUTE) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).ResolveAttributes)(windows_core::Interface::as_raw(self), rgkeyattributes.len().try_into().unwrap(), core::mem::transmute(rgkeyattributes.as_ptr()), pcelt as _, prgmatchvalues as _).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct INetDiagExtensibleHelper_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub ResolveAttributes: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const HELPER_ATTRIBUTE, *mut u32, *mut *mut HELPER_ATTRIBUTE) -> windows_core::HRESULT,
}
pub trait INetDiagExtensibleHelper_Impl: windows_core::IUnknownImpl {
    fn ResolveAttributes(&self, celt: u32, rgkeyattributes: *const HELPER_ATTRIBUTE, pcelt: *mut u32, prgmatchvalues: *mut *mut HELPER_ATTRIBUTE) -> windows_core::Result<()>;
}
impl INetDiagExtensibleHelper_Vtbl {
    pub const fn new<Identity: INetDiagExtensibleHelper_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn ResolveAttributes<Identity: INetDiagExtensibleHelper_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, celt: u32, rgkeyattributes: *const HELPER_ATTRIBUTE, pcelt: *mut u32, prgmatchvalues: *mut *mut HELPER_ATTRIBUTE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                INetDiagExtensibleHelper_Impl::ResolveAttributes(this, core::mem::transmute_copy(&celt), core::mem::transmute_copy(&rgkeyattributes), core::mem::transmute_copy(&pcelt), core::mem::transmute_copy(&prgmatchvalues)).into()
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), ResolveAttributes: ResolveAttributes::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<INetDiagExtensibleHelper as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for INetDiagExtensibleHelper {}
windows_core::imp::define_interface!(INetDiagHelper, INetDiagHelper_Vtbl, 0xc0b35746_ebf5_11d8_bbe9_505054503030);
windows_core::imp::interface_hierarchy!(INetDiagHelper, windows_core::IUnknown);
impl INetDiagHelper {
    pub unsafe fn Initialize(&self, rgattributes: &[HELPER_ATTRIBUTE]) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Initialize)(windows_core::Interface::as_raw(self), rgattributes.len().try_into().unwrap(), core::mem::transmute(rgattributes.as_ptr())).ok() }
    }
    pub unsafe fn GetDiagnosticsInfo(&self) -> windows_core::Result<*mut DiagnosticsInfo> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetDiagnosticsInfo)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetKeyAttributes(&self, pcelt: *mut u32, pprgattributes: *mut *mut HELPER_ATTRIBUTE) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetKeyAttributes)(windows_core::Interface::as_raw(self), pcelt as _, pprgattributes as _).ok() }
    }
    pub unsafe fn LowHealth<P0>(&self, pwszinstancedescription: P0, ppwszdescription: *mut windows_core::PWSTR, pdeferredtime: *mut i32, pstatus: *mut DIAGNOSIS_STATUS) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).LowHealth)(windows_core::Interface::as_raw(self), pwszinstancedescription.param().abi(), ppwszdescription as _, pdeferredtime as _, pstatus as _).ok() }
    }
    pub unsafe fn HighUtilization<P0>(&self, pwszinstancedescription: P0, ppwszdescription: *mut windows_core::PWSTR, pdeferredtime: *mut i32, pstatus: *mut DIAGNOSIS_STATUS) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).HighUtilization)(windows_core::Interface::as_raw(self), pwszinstancedescription.param().abi(), ppwszdescription as _, pdeferredtime as _, pstatus as _).ok() }
    }
    pub unsafe fn GetLowerHypotheses(&self, pcelt: *mut u32, pprghypotheses: *mut *mut HYPOTHESIS) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetLowerHypotheses)(windows_core::Interface::as_raw(self), pcelt as _, pprghypotheses as _).ok() }
    }
    pub unsafe fn GetDownStreamHypotheses(&self, pcelt: *mut u32, pprghypotheses: *mut *mut HYPOTHESIS) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetDownStreamHypotheses)(windows_core::Interface::as_raw(self), pcelt as _, pprghypotheses as _).ok() }
    }
    pub unsafe fn GetHigherHypotheses(&self, pcelt: *mut u32, pprghypotheses: *mut *mut HYPOTHESIS) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetHigherHypotheses)(windows_core::Interface::as_raw(self), pcelt as _, pprghypotheses as _).ok() }
    }
    pub unsafe fn GetUpStreamHypotheses(&self, pcelt: *mut u32, pprghypotheses: *mut *mut HYPOTHESIS) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetUpStreamHypotheses)(windows_core::Interface::as_raw(self), pcelt as _, pprghypotheses as _).ok() }
    }
    pub unsafe fn Repair(&self, pinfo: *const RepairInfo, pdeferredtime: *mut i32, pstatus: *mut REPAIR_STATUS) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Repair)(windows_core::Interface::as_raw(self), pinfo, pdeferredtime as _, pstatus as _).ok() }
    }
    pub unsafe fn Validate(&self, problem: PROBLEM_TYPE, pdeferredtime: *mut i32, pstatus: *mut REPAIR_STATUS) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Validate)(windows_core::Interface::as_raw(self), problem, pdeferredtime as _, pstatus as _).ok() }
    }
    pub unsafe fn GetRepairInfo(&self, problem: PROBLEM_TYPE, pcelt: *mut u32, ppinfo: *mut *mut RepairInfo) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetRepairInfo)(windows_core::Interface::as_raw(self), problem, pcelt as _, ppinfo as _).ok() }
    }
    pub unsafe fn GetLifeTime(&self) -> windows_core::Result<LIFE_TIME> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetLifeTime)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetLifeTime(&self, lifetime: LIFE_TIME) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetLifeTime)(windows_core::Interface::as_raw(self), core::mem::transmute(lifetime)).ok() }
    }
    pub unsafe fn GetCacheTime(&self) -> windows_core::Result<super::super::Foundation::FILETIME> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetCacheTime)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetAttributes(&self, pcelt: *mut u32, pprgattributes: *mut *mut HELPER_ATTRIBUTE) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetAttributes)(windows_core::Interface::as_raw(self), pcelt as _, pprgattributes as _).ok() }
    }
    pub unsafe fn Cancel(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Cancel)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn Cleanup(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Cleanup)(windows_core::Interface::as_raw(self)).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
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
pub trait INetDiagHelper_Impl: windows_core::IUnknownImpl {
    fn Initialize(&self, celt: u32, rgattributes: *const HELPER_ATTRIBUTE) -> windows_core::Result<()>;
    fn GetDiagnosticsInfo(&self) -> windows_core::Result<*mut DiagnosticsInfo>;
    fn GetKeyAttributes(&self, pcelt: *mut u32, pprgattributes: *mut *mut HELPER_ATTRIBUTE) -> windows_core::Result<()>;
    fn LowHealth(&self, pwszinstancedescription: &windows_core::PCWSTR, ppwszdescription: *mut windows_core::PWSTR, pdeferredtime: *mut i32, pstatus: *mut DIAGNOSIS_STATUS) -> windows_core::Result<()>;
    fn HighUtilization(&self, pwszinstancedescription: &windows_core::PCWSTR, ppwszdescription: *mut windows_core::PWSTR, pdeferredtime: *mut i32, pstatus: *mut DIAGNOSIS_STATUS) -> windows_core::Result<()>;
    fn GetLowerHypotheses(&self, pcelt: *mut u32, pprghypotheses: *mut *mut HYPOTHESIS) -> windows_core::Result<()>;
    fn GetDownStreamHypotheses(&self, pcelt: *mut u32, pprghypotheses: *mut *mut HYPOTHESIS) -> windows_core::Result<()>;
    fn GetHigherHypotheses(&self, pcelt: *mut u32, pprghypotheses: *mut *mut HYPOTHESIS) -> windows_core::Result<()>;
    fn GetUpStreamHypotheses(&self, pcelt: *mut u32, pprghypotheses: *mut *mut HYPOTHESIS) -> windows_core::Result<()>;
    fn Repair(&self, pinfo: *const RepairInfo, pdeferredtime: *mut i32, pstatus: *mut REPAIR_STATUS) -> windows_core::Result<()>;
    fn Validate(&self, problem: PROBLEM_TYPE, pdeferredtime: *mut i32, pstatus: *mut REPAIR_STATUS) -> windows_core::Result<()>;
    fn GetRepairInfo(&self, problem: PROBLEM_TYPE, pcelt: *mut u32, ppinfo: *mut *mut RepairInfo) -> windows_core::Result<()>;
    fn GetLifeTime(&self) -> windows_core::Result<LIFE_TIME>;
    fn SetLifeTime(&self, lifetime: &LIFE_TIME) -> windows_core::Result<()>;
    fn GetCacheTime(&self) -> windows_core::Result<super::super::Foundation::FILETIME>;
    fn GetAttributes(&self, pcelt: *mut u32, pprgattributes: *mut *mut HELPER_ATTRIBUTE) -> windows_core::Result<()>;
    fn Cancel(&self) -> windows_core::Result<()>;
    fn Cleanup(&self) -> windows_core::Result<()>;
}
impl INetDiagHelper_Vtbl {
    pub const fn new<Identity: INetDiagHelper_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Initialize<Identity: INetDiagHelper_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, celt: u32, rgattributes: *const HELPER_ATTRIBUTE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                INetDiagHelper_Impl::Initialize(this, core::mem::transmute_copy(&celt), core::mem::transmute_copy(&rgattributes)).into()
            }
        }
        unsafe extern "system" fn GetDiagnosticsInfo<Identity: INetDiagHelper_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppinfo: *mut *mut DiagnosticsInfo) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match INetDiagHelper_Impl::GetDiagnosticsInfo(this) {
                    Ok(ok__) => {
                        ppinfo.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetKeyAttributes<Identity: INetDiagHelper_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcelt: *mut u32, pprgattributes: *mut *mut HELPER_ATTRIBUTE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                INetDiagHelper_Impl::GetKeyAttributes(this, core::mem::transmute_copy(&pcelt), core::mem::transmute_copy(&pprgattributes)).into()
            }
        }
        unsafe extern "system" fn LowHealth<Identity: INetDiagHelper_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwszinstancedescription: windows_core::PCWSTR, ppwszdescription: *mut windows_core::PWSTR, pdeferredtime: *mut i32, pstatus: *mut DIAGNOSIS_STATUS) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                INetDiagHelper_Impl::LowHealth(this, core::mem::transmute(&pwszinstancedescription), core::mem::transmute_copy(&ppwszdescription), core::mem::transmute_copy(&pdeferredtime), core::mem::transmute_copy(&pstatus)).into()
            }
        }
        unsafe extern "system" fn HighUtilization<Identity: INetDiagHelper_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwszinstancedescription: windows_core::PCWSTR, ppwszdescription: *mut windows_core::PWSTR, pdeferredtime: *mut i32, pstatus: *mut DIAGNOSIS_STATUS) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                INetDiagHelper_Impl::HighUtilization(this, core::mem::transmute(&pwszinstancedescription), core::mem::transmute_copy(&ppwszdescription), core::mem::transmute_copy(&pdeferredtime), core::mem::transmute_copy(&pstatus)).into()
            }
        }
        unsafe extern "system" fn GetLowerHypotheses<Identity: INetDiagHelper_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcelt: *mut u32, pprghypotheses: *mut *mut HYPOTHESIS) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                INetDiagHelper_Impl::GetLowerHypotheses(this, core::mem::transmute_copy(&pcelt), core::mem::transmute_copy(&pprghypotheses)).into()
            }
        }
        unsafe extern "system" fn GetDownStreamHypotheses<Identity: INetDiagHelper_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcelt: *mut u32, pprghypotheses: *mut *mut HYPOTHESIS) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                INetDiagHelper_Impl::GetDownStreamHypotheses(this, core::mem::transmute_copy(&pcelt), core::mem::transmute_copy(&pprghypotheses)).into()
            }
        }
        unsafe extern "system" fn GetHigherHypotheses<Identity: INetDiagHelper_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcelt: *mut u32, pprghypotheses: *mut *mut HYPOTHESIS) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                INetDiagHelper_Impl::GetHigherHypotheses(this, core::mem::transmute_copy(&pcelt), core::mem::transmute_copy(&pprghypotheses)).into()
            }
        }
        unsafe extern "system" fn GetUpStreamHypotheses<Identity: INetDiagHelper_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcelt: *mut u32, pprghypotheses: *mut *mut HYPOTHESIS) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                INetDiagHelper_Impl::GetUpStreamHypotheses(this, core::mem::transmute_copy(&pcelt), core::mem::transmute_copy(&pprghypotheses)).into()
            }
        }
        unsafe extern "system" fn Repair<Identity: INetDiagHelper_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pinfo: *const RepairInfo, pdeferredtime: *mut i32, pstatus: *mut REPAIR_STATUS) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                INetDiagHelper_Impl::Repair(this, core::mem::transmute_copy(&pinfo), core::mem::transmute_copy(&pdeferredtime), core::mem::transmute_copy(&pstatus)).into()
            }
        }
        unsafe extern "system" fn Validate<Identity: INetDiagHelper_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, problem: PROBLEM_TYPE, pdeferredtime: *mut i32, pstatus: *mut REPAIR_STATUS) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                INetDiagHelper_Impl::Validate(this, core::mem::transmute_copy(&problem), core::mem::transmute_copy(&pdeferredtime), core::mem::transmute_copy(&pstatus)).into()
            }
        }
        unsafe extern "system" fn GetRepairInfo<Identity: INetDiagHelper_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, problem: PROBLEM_TYPE, pcelt: *mut u32, ppinfo: *mut *mut RepairInfo) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                INetDiagHelper_Impl::GetRepairInfo(this, core::mem::transmute_copy(&problem), core::mem::transmute_copy(&pcelt), core::mem::transmute_copy(&ppinfo)).into()
            }
        }
        unsafe extern "system" fn GetLifeTime<Identity: INetDiagHelper_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, plifetime: *mut LIFE_TIME) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match INetDiagHelper_Impl::GetLifeTime(this) {
                    Ok(ok__) => {
                        plifetime.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetLifeTime<Identity: INetDiagHelper_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lifetime: LIFE_TIME) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                INetDiagHelper_Impl::SetLifeTime(this, core::mem::transmute(&lifetime)).into()
            }
        }
        unsafe extern "system" fn GetCacheTime<Identity: INetDiagHelper_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcachetime: *mut super::super::Foundation::FILETIME) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match INetDiagHelper_Impl::GetCacheTime(this) {
                    Ok(ok__) => {
                        pcachetime.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetAttributes<Identity: INetDiagHelper_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcelt: *mut u32, pprgattributes: *mut *mut HELPER_ATTRIBUTE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                INetDiagHelper_Impl::GetAttributes(this, core::mem::transmute_copy(&pcelt), core::mem::transmute_copy(&pprgattributes)).into()
            }
        }
        unsafe extern "system" fn Cancel<Identity: INetDiagHelper_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                INetDiagHelper_Impl::Cancel(this).into()
            }
        }
        unsafe extern "system" fn Cleanup<Identity: INetDiagHelper_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                INetDiagHelper_Impl::Cleanup(this).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Initialize: Initialize::<Identity, OFFSET>,
            GetDiagnosticsInfo: GetDiagnosticsInfo::<Identity, OFFSET>,
            GetKeyAttributes: GetKeyAttributes::<Identity, OFFSET>,
            LowHealth: LowHealth::<Identity, OFFSET>,
            HighUtilization: HighUtilization::<Identity, OFFSET>,
            GetLowerHypotheses: GetLowerHypotheses::<Identity, OFFSET>,
            GetDownStreamHypotheses: GetDownStreamHypotheses::<Identity, OFFSET>,
            GetHigherHypotheses: GetHigherHypotheses::<Identity, OFFSET>,
            GetUpStreamHypotheses: GetUpStreamHypotheses::<Identity, OFFSET>,
            Repair: Repair::<Identity, OFFSET>,
            Validate: Validate::<Identity, OFFSET>,
            GetRepairInfo: GetRepairInfo::<Identity, OFFSET>,
            GetLifeTime: GetLifeTime::<Identity, OFFSET>,
            SetLifeTime: SetLifeTime::<Identity, OFFSET>,
            GetCacheTime: GetCacheTime::<Identity, OFFSET>,
            GetAttributes: GetAttributes::<Identity, OFFSET>,
            Cancel: Cancel::<Identity, OFFSET>,
            Cleanup: Cleanup::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<INetDiagHelper as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for INetDiagHelper {}
windows_core::imp::define_interface!(INetDiagHelperEx, INetDiagHelperEx_Vtbl, 0x972dab4d_e4e3_4fc6_ae54_5f65ccde4a15);
windows_core::imp::interface_hierarchy!(INetDiagHelperEx, windows_core::IUnknown);
impl INetDiagHelperEx {
    pub unsafe fn ReconfirmLowHealth(&self, presults: &[HypothesisResult], ppwszupdateddescription: *mut windows_core::PWSTR, pupdatedstatus: *mut DIAGNOSIS_STATUS) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).ReconfirmLowHealth)(windows_core::Interface::as_raw(self), presults.len().try_into().unwrap(), core::mem::transmute(presults.as_ptr()), ppwszupdateddescription as _, pupdatedstatus as _).ok() }
    }
    pub unsafe fn SetUtilities<P0>(&self, putilities: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<INetDiagHelperUtilFactory>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetUtilities)(windows_core::Interface::as_raw(self), putilities.param().abi()).ok() }
    }
    pub unsafe fn ReproduceFailure(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).ReproduceFailure)(windows_core::Interface::as_raw(self)).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct INetDiagHelperEx_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub ReconfirmLowHealth: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const HypothesisResult, *mut windows_core::PWSTR, *mut DIAGNOSIS_STATUS) -> windows_core::HRESULT,
    pub SetUtilities: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ReproduceFailure: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait INetDiagHelperEx_Impl: windows_core::IUnknownImpl {
    fn ReconfirmLowHealth(&self, celt: u32, presults: *const HypothesisResult, ppwszupdateddescription: *mut windows_core::PWSTR, pupdatedstatus: *mut DIAGNOSIS_STATUS) -> windows_core::Result<()>;
    fn SetUtilities(&self, putilities: windows_core::Ref<INetDiagHelperUtilFactory>) -> windows_core::Result<()>;
    fn ReproduceFailure(&self) -> windows_core::Result<()>;
}
impl INetDiagHelperEx_Vtbl {
    pub const fn new<Identity: INetDiagHelperEx_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn ReconfirmLowHealth<Identity: INetDiagHelperEx_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, celt: u32, presults: *const HypothesisResult, ppwszupdateddescription: *mut windows_core::PWSTR, pupdatedstatus: *mut DIAGNOSIS_STATUS) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                INetDiagHelperEx_Impl::ReconfirmLowHealth(this, core::mem::transmute_copy(&celt), core::mem::transmute_copy(&presults), core::mem::transmute_copy(&ppwszupdateddescription), core::mem::transmute_copy(&pupdatedstatus)).into()
            }
        }
        unsafe extern "system" fn SetUtilities<Identity: INetDiagHelperEx_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, putilities: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                INetDiagHelperEx_Impl::SetUtilities(this, core::mem::transmute_copy(&putilities)).into()
            }
        }
        unsafe extern "system" fn ReproduceFailure<Identity: INetDiagHelperEx_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                INetDiagHelperEx_Impl::ReproduceFailure(this).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            ReconfirmLowHealth: ReconfirmLowHealth::<Identity, OFFSET>,
            SetUtilities: SetUtilities::<Identity, OFFSET>,
            ReproduceFailure: ReproduceFailure::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<INetDiagHelperEx as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for INetDiagHelperEx {}
windows_core::imp::define_interface!(INetDiagHelperInfo, INetDiagHelperInfo_Vtbl, 0xc0b35747_ebf5_11d8_bbe9_505054503030);
windows_core::imp::interface_hierarchy!(INetDiagHelperInfo, windows_core::IUnknown);
impl INetDiagHelperInfo {
    pub unsafe fn GetAttributeInfo(&self, pcelt: *mut u32, pprgattributeinfos: *mut *mut HelperAttributeInfo) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetAttributeInfo)(windows_core::Interface::as_raw(self), pcelt as _, pprgattributeinfos as _).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct INetDiagHelperInfo_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetAttributeInfo: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32, *mut *mut HelperAttributeInfo) -> windows_core::HRESULT,
}
pub trait INetDiagHelperInfo_Impl: windows_core::IUnknownImpl {
    fn GetAttributeInfo(&self, pcelt: *mut u32, pprgattributeinfos: *mut *mut HelperAttributeInfo) -> windows_core::Result<()>;
}
impl INetDiagHelperInfo_Vtbl {
    pub const fn new<Identity: INetDiagHelperInfo_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetAttributeInfo<Identity: INetDiagHelperInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcelt: *mut u32, pprgattributeinfos: *mut *mut HelperAttributeInfo) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                INetDiagHelperInfo_Impl::GetAttributeInfo(this, core::mem::transmute_copy(&pcelt), core::mem::transmute_copy(&pprgattributeinfos)).into()
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), GetAttributeInfo: GetAttributeInfo::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<INetDiagHelperInfo as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for INetDiagHelperInfo {}
windows_core::imp::define_interface!(INetDiagHelperUtilFactory, INetDiagHelperUtilFactory_Vtbl, 0x104613fb_bc57_4178_95ba_88809698354a);
windows_core::imp::interface_hierarchy!(INetDiagHelperUtilFactory, windows_core::IUnknown);
impl INetDiagHelperUtilFactory {
    pub unsafe fn CreateUtilityInstance<T>(&self) -> windows_core::Result<T>
    where
        T: windows_core::Interface,
    {
        let mut result__ = core::ptr::null_mut();
        unsafe { (windows_core::Interface::vtable(self).CreateUtilityInstance)(windows_core::Interface::as_raw(self), &T::IID, &mut result__).and_then(|| windows_core::Type::from_abi(result__)) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct INetDiagHelperUtilFactory_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub CreateUtilityInstance: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait INetDiagHelperUtilFactory_Impl: windows_core::IUnknownImpl {
    fn CreateUtilityInstance(&self, riid: *const windows_core::GUID, ppvobject: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
}
impl INetDiagHelperUtilFactory_Vtbl {
    pub const fn new<Identity: INetDiagHelperUtilFactory_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn CreateUtilityInstance<Identity: INetDiagHelperUtilFactory_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, riid: *const windows_core::GUID, ppvobject: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                INetDiagHelperUtilFactory_Impl::CreateUtilityInstance(this, core::mem::transmute_copy(&riid), core::mem::transmute_copy(&ppvobject)).into()
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), CreateUtilityInstance: CreateUtilityInstance::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<INetDiagHelperUtilFactory as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for INetDiagHelperUtilFactory {}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct LIFE_TIME {
    pub startTime: super::super::Foundation::FILETIME,
    pub endTime: super::super::Foundation::FILETIME,
}
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
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct OCTET_STRING {
    pub dwLength: u32,
    pub lpValue: *mut u8,
}
impl Default for OCTET_STRING {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct PROBLEM_TYPE(pub i32);
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
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct REPAIR_RISK(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct REPAIR_SCOPE(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct REPAIR_STATUS(pub i32);
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
impl Default for RepairInfoEx {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct RootCauseInfo {
    pub pwszDescription: windows_core::PWSTR,
    pub rootCauseID: windows_core::GUID,
    pub rootCauseFlags: u32,
    pub networkInterfaceID: windows_core::GUID,
    pub pRepairs: *mut RepairInfoEx,
    pub repairCount: u16,
}
impl Default for RootCauseInfo {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct ShellCommandInfo {
    pub pwszOperation: windows_core::PWSTR,
    pub pwszFile: windows_core::PWSTR,
    pub pwszParameters: windows_core::PWSTR,
    pub pwszDirectory: windows_core::PWSTR,
    pub nShowCmd: u32,
}
pub const UIT_DUI: UI_INFO_TYPE = UI_INFO_TYPE(4i32);
pub const UIT_HELP_PANE: UI_INFO_TYPE = UI_INFO_TYPE(3i32);
pub const UIT_INVALID: UI_INFO_TYPE = UI_INFO_TYPE(0i32);
pub const UIT_NONE: UI_INFO_TYPE = UI_INFO_TYPE(1i32);
pub const UIT_SHELL_COMMAND: UI_INFO_TYPE = UI_INFO_TYPE(2i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct UI_INFO_TYPE(pub i32);
#[repr(C)]
#[derive(Clone, Copy)]
pub struct UiInfo {
    pub r#type: UI_INFO_TYPE,
    pub Anonymous: UiInfo_0,
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
impl Default for UiInfo_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
