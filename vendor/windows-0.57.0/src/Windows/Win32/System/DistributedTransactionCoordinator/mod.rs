#[inline]
pub unsafe fn DtcGetTransactionManager<P0, P1>(i_pszhost: P0, i_psztmname: P1, i_riid: *const windows_core::GUID, i_dwreserved1: u32, i_wcbreserved2: u16, i_pvreserved2: Option<*const core::ffi::c_void>, o_ppvobject: *mut *mut core::ffi::c_void) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("xolehlp.dll" "cdecl" fn DtcGetTransactionManager(i_pszhost : windows_core::PCSTR, i_psztmname : windows_core::PCSTR, i_riid : *const windows_core::GUID, i_dwreserved1 : u32, i_wcbreserved2 : u16, i_pvreserved2 : *const core::ffi::c_void, o_ppvobject : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    DtcGetTransactionManager(i_pszhost.param().abi(), i_psztmname.param().abi(), i_riid, i_dwreserved1, i_wcbreserved2, core::mem::transmute(i_pvreserved2.unwrap_or(std::ptr::null())), o_ppvobject).ok()
}
#[inline]
pub unsafe fn DtcGetTransactionManagerC<P0, P1>(i_pszhost: P0, i_psztmname: P1, i_riid: *const windows_core::GUID, i_dwreserved1: u32, i_wcbreserved2: u16, i_pvreserved2: Option<*const core::ffi::c_void>, o_ppvobject: *mut *mut core::ffi::c_void) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("xolehlp.dll" "cdecl" fn DtcGetTransactionManagerC(i_pszhost : windows_core::PCSTR, i_psztmname : windows_core::PCSTR, i_riid : *const windows_core::GUID, i_dwreserved1 : u32, i_wcbreserved2 : u16, i_pvreserved2 : *const core::ffi::c_void, o_ppvobject : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    DtcGetTransactionManagerC(i_pszhost.param().abi(), i_psztmname.param().abi(), i_riid, i_dwreserved1, i_wcbreserved2, core::mem::transmute(i_pvreserved2.unwrap_or(std::ptr::null())), o_ppvobject).ok()
}
#[inline]
pub unsafe fn DtcGetTransactionManagerExA<P0, P1>(i_pszhost: P0, i_psztmname: P1, i_riid: *const windows_core::GUID, i_grfoptions: u32, i_pvconfigparams: *mut core::ffi::c_void, o_ppvobject: *mut *mut core::ffi::c_void) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("xolehlp.dll" "cdecl" fn DtcGetTransactionManagerExA(i_pszhost : windows_core::PCSTR, i_psztmname : windows_core::PCSTR, i_riid : *const windows_core::GUID, i_grfoptions : u32, i_pvconfigparams : *mut core::ffi::c_void, o_ppvobject : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    DtcGetTransactionManagerExA(i_pszhost.param().abi(), i_psztmname.param().abi(), i_riid, i_grfoptions, i_pvconfigparams, o_ppvobject).ok()
}
#[inline]
pub unsafe fn DtcGetTransactionManagerExW<P0, P1>(i_pwszhost: P0, i_pwsztmname: P1, i_riid: *const windows_core::GUID, i_grfoptions: u32, i_pvconfigparams: *mut core::ffi::c_void, o_ppvobject: *mut *mut core::ffi::c_void) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("xolehlp.dll" "cdecl" fn DtcGetTransactionManagerExW(i_pwszhost : windows_core::PCWSTR, i_pwsztmname : windows_core::PCWSTR, i_riid : *const windows_core::GUID, i_grfoptions : u32, i_pvconfigparams : *mut core::ffi::c_void, o_ppvobject : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    DtcGetTransactionManagerExW(i_pwszhost.param().abi(), i_pwsztmname.param().abi(), i_riid, i_grfoptions, i_pvconfigparams, o_ppvobject).ok()
}
windows_core::imp::define_interface!(IDtcLuConfigure, IDtcLuConfigure_Vtbl, 0x4131e760_1aea_11d0_944b_00a0c905416e);
impl core::ops::Deref for IDtcLuConfigure {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IDtcLuConfigure, windows_core::IUnknown);
impl IDtcLuConfigure {
    pub unsafe fn Add(&self, puclupair: &[u8]) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Add)(windows_core::Interface::as_raw(self), core::mem::transmute(puclupair.as_ptr()), puclupair.len().try_into().unwrap()).ok()
    }
    pub unsafe fn Delete(&self, puclupair: &[u8]) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Delete)(windows_core::Interface::as_raw(self), core::mem::transmute(puclupair.as_ptr()), puclupair.len().try_into().unwrap()).ok()
    }
}
#[repr(C)]
pub struct IDtcLuConfigure_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Add: unsafe extern "system" fn(*mut core::ffi::c_void, *const u8, u32) -> windows_core::HRESULT,
    pub Delete: unsafe extern "system" fn(*mut core::ffi::c_void, *const u8, u32) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IDtcLuRecovery, IDtcLuRecovery_Vtbl, 0xac2b8ad2_d6f0_11d0_b386_00a0c9083365);
impl core::ops::Deref for IDtcLuRecovery {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IDtcLuRecovery, windows_core::IUnknown);
impl IDtcLuRecovery {}
#[repr(C)]
pub struct IDtcLuRecovery_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
}
windows_core::imp::define_interface!(IDtcLuRecoveryFactory, IDtcLuRecoveryFactory_Vtbl, 0x4131e762_1aea_11d0_944b_00a0c905416e);
impl core::ops::Deref for IDtcLuRecoveryFactory {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IDtcLuRecoveryFactory, windows_core::IUnknown);
impl IDtcLuRecoveryFactory {
    pub unsafe fn Create(&self, puclupair: &[u8]) -> windows_core::Result<IDtcLuRecovery> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Create)(windows_core::Interface::as_raw(self), core::mem::transmute(puclupair.as_ptr()), puclupair.len().try_into().unwrap(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[repr(C)]
pub struct IDtcLuRecoveryFactory_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Create: unsafe extern "system" fn(*mut core::ffi::c_void, *const u8, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IDtcLuRecoveryInitiatedByDtc, IDtcLuRecoveryInitiatedByDtc_Vtbl, 0x4131e764_1aea_11d0_944b_00a0c905416e);
impl core::ops::Deref for IDtcLuRecoveryInitiatedByDtc {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IDtcLuRecoveryInitiatedByDtc, windows_core::IUnknown);
impl IDtcLuRecoveryInitiatedByDtc {
    pub unsafe fn GetWork(&self, pwork: *mut DTCINITIATEDRECOVERYWORK, ppv: *mut *mut core::ffi::c_void) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetWork)(windows_core::Interface::as_raw(self), pwork, ppv).ok()
    }
}
#[repr(C)]
pub struct IDtcLuRecoveryInitiatedByDtc_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetWork: unsafe extern "system" fn(*mut core::ffi::c_void, *mut DTCINITIATEDRECOVERYWORK, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IDtcLuRecoveryInitiatedByDtcStatusWork, IDtcLuRecoveryInitiatedByDtcStatusWork_Vtbl, 0x4131e766_1aea_11d0_944b_00a0c905416e);
impl core::ops::Deref for IDtcLuRecoveryInitiatedByDtcStatusWork {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IDtcLuRecoveryInitiatedByDtcStatusWork, windows_core::IUnknown);
impl IDtcLuRecoveryInitiatedByDtcStatusWork {
    pub unsafe fn HandleCheckLuStatus(&self, lrecoveryseqnum: i32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).HandleCheckLuStatus)(windows_core::Interface::as_raw(self), lrecoveryseqnum).ok()
    }
}
#[repr(C)]
pub struct IDtcLuRecoveryInitiatedByDtcStatusWork_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub HandleCheckLuStatus: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IDtcLuRecoveryInitiatedByDtcTransWork, IDtcLuRecoveryInitiatedByDtcTransWork_Vtbl, 0x4131e765_1aea_11d0_944b_00a0c905416e);
impl core::ops::Deref for IDtcLuRecoveryInitiatedByDtcTransWork {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IDtcLuRecoveryInitiatedByDtcTransWork, windows_core::IUnknown);
impl IDtcLuRecoveryInitiatedByDtcTransWork {
    pub unsafe fn GetLogNameSizes(&self, pcbourlogname: *mut u32, pcbremotelogname: *mut u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetLogNameSizes)(windows_core::Interface::as_raw(self), pcbourlogname, pcbremotelogname).ok()
    }
    pub unsafe fn GetOurXln(&self, pxln: *mut DTCLUXLN, pourlogname: *mut u8, premotelogname: *mut u8, pdwprotocol: *mut u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetOurXln)(windows_core::Interface::as_raw(self), pxln, pourlogname, premotelogname, pdwprotocol).ok()
    }
    pub unsafe fn HandleConfirmationFromOurXln(&self, confirmation: DTCLUXLNCONFIRMATION) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).HandleConfirmationFromOurXln)(windows_core::Interface::as_raw(self), confirmation).ok()
    }
    pub unsafe fn HandleTheirXlnResponse(&self, xln: DTCLUXLN, premotelogname: *mut u8, cbremotelogname: u32, dwprotocol: u32, pconfirmation: *mut DTCLUXLNCONFIRMATION) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).HandleTheirXlnResponse)(windows_core::Interface::as_raw(self), xln, premotelogname, cbremotelogname, dwprotocol, pconfirmation).ok()
    }
    pub unsafe fn HandleErrorFromOurXln(&self, error: DTCLUXLNERROR) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).HandleErrorFromOurXln)(windows_core::Interface::as_raw(self), error).ok()
    }
    pub unsafe fn CheckForCompareStates(&self, fcomparestates: *mut super::super::Foundation::BOOL) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).CheckForCompareStates)(windows_core::Interface::as_raw(self), fcomparestates).ok()
    }
    pub unsafe fn GetOurTransIdSize(&self, pcbourtransid: *mut u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetOurTransIdSize)(windows_core::Interface::as_raw(self), pcbourtransid).ok()
    }
    pub unsafe fn GetOurCompareStates(&self, pourtransid: *mut u8, pcomparestate: *mut DTCLUCOMPARESTATE) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetOurCompareStates)(windows_core::Interface::as_raw(self), pourtransid, pcomparestate).ok()
    }
    pub unsafe fn HandleTheirCompareStatesResponse(&self, comparestate: DTCLUCOMPARESTATE, pconfirmation: *mut DTCLUCOMPARESTATESCONFIRMATION) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).HandleTheirCompareStatesResponse)(windows_core::Interface::as_raw(self), comparestate, pconfirmation).ok()
    }
    pub unsafe fn HandleErrorFromOurCompareStates(&self, error: DTCLUCOMPARESTATESERROR) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).HandleErrorFromOurCompareStates)(windows_core::Interface::as_raw(self), error).ok()
    }
    pub unsafe fn ConversationLost(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).ConversationLost)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn GetRecoverySeqNum(&self, plrecoveryseqnum: *mut i32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetRecoverySeqNum)(windows_core::Interface::as_raw(self), plrecoveryseqnum).ok()
    }
    pub unsafe fn ObsoleteRecoverySeqNum(&self, lnewrecoveryseqnum: i32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).ObsoleteRecoverySeqNum)(windows_core::Interface::as_raw(self), lnewrecoveryseqnum).ok()
    }
}
#[repr(C)]
pub struct IDtcLuRecoveryInitiatedByDtcTransWork_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetLogNameSizes: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32, *mut u32) -> windows_core::HRESULT,
    pub GetOurXln: unsafe extern "system" fn(*mut core::ffi::c_void, *mut DTCLUXLN, *mut u8, *mut u8, *mut u32) -> windows_core::HRESULT,
    pub HandleConfirmationFromOurXln: unsafe extern "system" fn(*mut core::ffi::c_void, DTCLUXLNCONFIRMATION) -> windows_core::HRESULT,
    pub HandleTheirXlnResponse: unsafe extern "system" fn(*mut core::ffi::c_void, DTCLUXLN, *mut u8, u32, u32, *mut DTCLUXLNCONFIRMATION) -> windows_core::HRESULT,
    pub HandleErrorFromOurXln: unsafe extern "system" fn(*mut core::ffi::c_void, DTCLUXLNERROR) -> windows_core::HRESULT,
    pub CheckForCompareStates: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::BOOL) -> windows_core::HRESULT,
    pub GetOurTransIdSize: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub GetOurCompareStates: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u8, *mut DTCLUCOMPARESTATE) -> windows_core::HRESULT,
    pub HandleTheirCompareStatesResponse: unsafe extern "system" fn(*mut core::ffi::c_void, DTCLUCOMPARESTATE, *mut DTCLUCOMPARESTATESCONFIRMATION) -> windows_core::HRESULT,
    pub HandleErrorFromOurCompareStates: unsafe extern "system" fn(*mut core::ffi::c_void, DTCLUCOMPARESTATESERROR) -> windows_core::HRESULT,
    pub ConversationLost: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetRecoverySeqNum: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub ObsoleteRecoverySeqNum: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IDtcLuRecoveryInitiatedByLu, IDtcLuRecoveryInitiatedByLu_Vtbl, 0x4131e768_1aea_11d0_944b_00a0c905416e);
impl core::ops::Deref for IDtcLuRecoveryInitiatedByLu {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IDtcLuRecoveryInitiatedByLu, windows_core::IUnknown);
impl IDtcLuRecoveryInitiatedByLu {
    pub unsafe fn GetObjectToHandleWorkFromLu(&self) -> windows_core::Result<IDtcLuRecoveryInitiatedByLuWork> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetObjectToHandleWorkFromLu)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[repr(C)]
pub struct IDtcLuRecoveryInitiatedByLu_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetObjectToHandleWorkFromLu: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IDtcLuRecoveryInitiatedByLuWork, IDtcLuRecoveryInitiatedByLuWork_Vtbl, 0xac2b8ad1_d6f0_11d0_b386_00a0c9083365);
impl core::ops::Deref for IDtcLuRecoveryInitiatedByLuWork {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IDtcLuRecoveryInitiatedByLuWork, windows_core::IUnknown);
impl IDtcLuRecoveryInitiatedByLuWork {
    pub unsafe fn HandleTheirXln(&self, lrecoveryseqnum: i32, xln: DTCLUXLN, premotelogname: *mut u8, cbremotelogname: u32, pourlogname: *mut u8, cbourlogname: u32, dwprotocol: u32, presponse: *mut DTCLUXLNRESPONSE) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).HandleTheirXln)(windows_core::Interface::as_raw(self), lrecoveryseqnum, xln, premotelogname, cbremotelogname, pourlogname, cbourlogname, dwprotocol, presponse).ok()
    }
    pub unsafe fn GetOurLogNameSize(&self, pcbourlogname: *mut u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetOurLogNameSize)(windows_core::Interface::as_raw(self), pcbourlogname).ok()
    }
    pub unsafe fn GetOurXln(&self, pxln: *mut DTCLUXLN, pourlogname: *mut u8, pdwprotocol: *mut u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetOurXln)(windows_core::Interface::as_raw(self), pxln, pourlogname, pdwprotocol).ok()
    }
    pub unsafe fn HandleConfirmationOfOurXln(&self, confirmation: DTCLUXLNCONFIRMATION) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).HandleConfirmationOfOurXln)(windows_core::Interface::as_raw(self), confirmation).ok()
    }
    pub unsafe fn HandleTheirCompareStates(&self, premotetransid: *mut u8, cbremotetransid: u32, comparestate: DTCLUCOMPARESTATE, presponse: *mut DTCLUCOMPARESTATESRESPONSE, pcomparestate: *mut DTCLUCOMPARESTATE) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).HandleTheirCompareStates)(windows_core::Interface::as_raw(self), premotetransid, cbremotetransid, comparestate, presponse, pcomparestate).ok()
    }
    pub unsafe fn HandleConfirmationOfOurCompareStates(&self, confirmation: DTCLUCOMPARESTATESCONFIRMATION) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).HandleConfirmationOfOurCompareStates)(windows_core::Interface::as_raw(self), confirmation).ok()
    }
    pub unsafe fn HandleErrorFromOurCompareStates(&self, error: DTCLUCOMPARESTATESERROR) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).HandleErrorFromOurCompareStates)(windows_core::Interface::as_raw(self), error).ok()
    }
    pub unsafe fn ConversationLost(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).ConversationLost)(windows_core::Interface::as_raw(self)).ok()
    }
}
#[repr(C)]
pub struct IDtcLuRecoveryInitiatedByLuWork_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub HandleTheirXln: unsafe extern "system" fn(*mut core::ffi::c_void, i32, DTCLUXLN, *mut u8, u32, *mut u8, u32, u32, *mut DTCLUXLNRESPONSE) -> windows_core::HRESULT,
    pub GetOurLogNameSize: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub GetOurXln: unsafe extern "system" fn(*mut core::ffi::c_void, *mut DTCLUXLN, *mut u8, *mut u32) -> windows_core::HRESULT,
    pub HandleConfirmationOfOurXln: unsafe extern "system" fn(*mut core::ffi::c_void, DTCLUXLNCONFIRMATION) -> windows_core::HRESULT,
    pub HandleTheirCompareStates: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u8, u32, DTCLUCOMPARESTATE, *mut DTCLUCOMPARESTATESRESPONSE, *mut DTCLUCOMPARESTATE) -> windows_core::HRESULT,
    pub HandleConfirmationOfOurCompareStates: unsafe extern "system" fn(*mut core::ffi::c_void, DTCLUCOMPARESTATESCONFIRMATION) -> windows_core::HRESULT,
    pub HandleErrorFromOurCompareStates: unsafe extern "system" fn(*mut core::ffi::c_void, DTCLUCOMPARESTATESERROR) -> windows_core::HRESULT,
    pub ConversationLost: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IDtcLuRmEnlistment, IDtcLuRmEnlistment_Vtbl, 0x4131e769_1aea_11d0_944b_00a0c905416e);
impl core::ops::Deref for IDtcLuRmEnlistment {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IDtcLuRmEnlistment, windows_core::IUnknown);
impl IDtcLuRmEnlistment {
    pub unsafe fn Unplug<P0>(&self, fconversationlost: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::BOOL>,
    {
        (windows_core::Interface::vtable(self).Unplug)(windows_core::Interface::as_raw(self), fconversationlost.param().abi()).ok()
    }
    pub unsafe fn BackedOut(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).BackedOut)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn BackOut(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).BackOut)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Committed(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Committed)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Forget(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Forget)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn RequestCommit(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).RequestCommit)(windows_core::Interface::as_raw(self)).ok()
    }
}
#[repr(C)]
pub struct IDtcLuRmEnlistment_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Unplug: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::BOOL) -> windows_core::HRESULT,
    pub BackedOut: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub BackOut: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Committed: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Forget: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub RequestCommit: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IDtcLuRmEnlistmentFactory, IDtcLuRmEnlistmentFactory_Vtbl, 0x4131e771_1aea_11d0_944b_00a0c905416e);
impl core::ops::Deref for IDtcLuRmEnlistmentFactory {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IDtcLuRmEnlistmentFactory, windows_core::IUnknown);
impl IDtcLuRmEnlistmentFactory {
    pub unsafe fn Create<P0, P1>(&self, puclupair: *mut u8, cblupair: u32, pitransaction: P0, ptransid: *mut u8, cbtransid: u32, prmenlistmentsink: P1, pprmenlistment: *mut Option<IDtcLuRmEnlistment>) -> windows_core::Result<()>
    where
        P0: windows_core::Param<ITransaction>,
        P1: windows_core::Param<IDtcLuRmEnlistmentSink>,
    {
        (windows_core::Interface::vtable(self).Create)(windows_core::Interface::as_raw(self), puclupair, cblupair, pitransaction.param().abi(), ptransid, cbtransid, prmenlistmentsink.param().abi(), core::mem::transmute(pprmenlistment)).ok()
    }
}
#[repr(C)]
pub struct IDtcLuRmEnlistmentFactory_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Create: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u8, u32, *mut core::ffi::c_void, *mut u8, u32, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IDtcLuRmEnlistmentSink, IDtcLuRmEnlistmentSink_Vtbl, 0x4131e770_1aea_11d0_944b_00a0c905416e);
impl core::ops::Deref for IDtcLuRmEnlistmentSink {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IDtcLuRmEnlistmentSink, windows_core::IUnknown);
impl IDtcLuRmEnlistmentSink {
    pub unsafe fn AckUnplug(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).AckUnplug)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn TmDown(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).TmDown)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn SessionLost(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SessionLost)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn BackedOut(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).BackedOut)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn BackOut(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).BackOut)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Committed(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Committed)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Forget(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Forget)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Prepare(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Prepare)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn RequestCommit(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).RequestCommit)(windows_core::Interface::as_raw(self)).ok()
    }
}
#[repr(C)]
pub struct IDtcLuRmEnlistmentSink_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub AckUnplug: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub TmDown: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SessionLost: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub BackedOut: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub BackOut: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Committed: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Forget: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Prepare: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub RequestCommit: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IDtcLuSubordinateDtc, IDtcLuSubordinateDtc_Vtbl, 0x4131e773_1aea_11d0_944b_00a0c905416e);
impl core::ops::Deref for IDtcLuSubordinateDtc {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IDtcLuSubordinateDtc, windows_core::IUnknown);
impl IDtcLuSubordinateDtc {
    pub unsafe fn Unplug<P0>(&self, fconversationlost: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::BOOL>,
    {
        (windows_core::Interface::vtable(self).Unplug)(windows_core::Interface::as_raw(self), fconversationlost.param().abi()).ok()
    }
    pub unsafe fn BackedOut(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).BackedOut)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn BackOut(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).BackOut)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Committed(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Committed)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Forget(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Forget)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Prepare(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Prepare)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn RequestCommit(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).RequestCommit)(windows_core::Interface::as_raw(self)).ok()
    }
}
#[repr(C)]
pub struct IDtcLuSubordinateDtc_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Unplug: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::BOOL) -> windows_core::HRESULT,
    pub BackedOut: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub BackOut: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Committed: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Forget: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Prepare: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub RequestCommit: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IDtcLuSubordinateDtcFactory, IDtcLuSubordinateDtcFactory_Vtbl, 0x4131e775_1aea_11d0_944b_00a0c905416e);
impl core::ops::Deref for IDtcLuSubordinateDtcFactory {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IDtcLuSubordinateDtcFactory, windows_core::IUnknown);
impl IDtcLuSubordinateDtcFactory {
    pub unsafe fn Create<P0, P1, P2>(&self, puclupair: *mut u8, cblupair: u32, punktransactionouter: P0, isolevel: i32, isoflags: u32, poptions: P1, pptransaction: *mut Option<ITransaction>, ptransid: *mut u8, cbtransid: u32, psubordinatedtcsink: P2, ppsubordinatedtc: *mut Option<IDtcLuSubordinateDtc>) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::IUnknown>,
        P1: windows_core::Param<ITransactionOptions>,
        P2: windows_core::Param<IDtcLuSubordinateDtcSink>,
    {
        (windows_core::Interface::vtable(self).Create)(windows_core::Interface::as_raw(self), puclupair, cblupair, punktransactionouter.param().abi(), isolevel, isoflags, poptions.param().abi(), core::mem::transmute(pptransaction), ptransid, cbtransid, psubordinatedtcsink.param().abi(), core::mem::transmute(ppsubordinatedtc)).ok()
    }
}
#[repr(C)]
pub struct IDtcLuSubordinateDtcFactory_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Create: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u8, u32, *mut core::ffi::c_void, i32, u32, *mut core::ffi::c_void, *mut *mut core::ffi::c_void, *mut u8, u32, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IDtcLuSubordinateDtcSink, IDtcLuSubordinateDtcSink_Vtbl, 0x4131e774_1aea_11d0_944b_00a0c905416e);
impl core::ops::Deref for IDtcLuSubordinateDtcSink {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IDtcLuSubordinateDtcSink, windows_core::IUnknown);
impl IDtcLuSubordinateDtcSink {
    pub unsafe fn AckUnplug(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).AckUnplug)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn TmDown(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).TmDown)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn SessionLost(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SessionLost)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn BackedOut(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).BackedOut)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn BackOut(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).BackOut)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Committed(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Committed)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Forget(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Forget)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn RequestCommit(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).RequestCommit)(windows_core::Interface::as_raw(self)).ok()
    }
}
#[repr(C)]
pub struct IDtcLuSubordinateDtcSink_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub AckUnplug: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub TmDown: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SessionLost: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub BackedOut: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub BackOut: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Committed: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Forget: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub RequestCommit: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IDtcNetworkAccessConfig, IDtcNetworkAccessConfig_Vtbl, 0x9797c15d_a428_4291_87b6_0995031a678d);
impl core::ops::Deref for IDtcNetworkAccessConfig {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IDtcNetworkAccessConfig, windows_core::IUnknown);
impl IDtcNetworkAccessConfig {
    pub unsafe fn GetAnyNetworkAccess(&self) -> windows_core::Result<super::super::Foundation::BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetAnyNetworkAccess)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetAnyNetworkAccess<P0>(&self, banynetworkaccess: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::BOOL>,
    {
        (windows_core::Interface::vtable(self).SetAnyNetworkAccess)(windows_core::Interface::as_raw(self), banynetworkaccess.param().abi()).ok()
    }
    pub unsafe fn GetNetworkAdministrationAccess(&self) -> windows_core::Result<super::super::Foundation::BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetNetworkAdministrationAccess)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetNetworkAdministrationAccess<P0>(&self, bnetworkadministrationaccess: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::BOOL>,
    {
        (windows_core::Interface::vtable(self).SetNetworkAdministrationAccess)(windows_core::Interface::as_raw(self), bnetworkadministrationaccess.param().abi()).ok()
    }
    pub unsafe fn GetNetworkTransactionAccess(&self) -> windows_core::Result<super::super::Foundation::BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetNetworkTransactionAccess)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetNetworkTransactionAccess<P0>(&self, bnetworktransactionaccess: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::BOOL>,
    {
        (windows_core::Interface::vtable(self).SetNetworkTransactionAccess)(windows_core::Interface::as_raw(self), bnetworktransactionaccess.param().abi()).ok()
    }
    pub unsafe fn GetNetworkClientAccess(&self) -> windows_core::Result<super::super::Foundation::BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetNetworkClientAccess)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetNetworkClientAccess<P0>(&self, bnetworkclientaccess: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::BOOL>,
    {
        (windows_core::Interface::vtable(self).SetNetworkClientAccess)(windows_core::Interface::as_raw(self), bnetworkclientaccess.param().abi()).ok()
    }
    pub unsafe fn GetNetworkTIPAccess(&self) -> windows_core::Result<super::super::Foundation::BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetNetworkTIPAccess)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetNetworkTIPAccess<P0>(&self, bnetworktipaccess: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::BOOL>,
    {
        (windows_core::Interface::vtable(self).SetNetworkTIPAccess)(windows_core::Interface::as_raw(self), bnetworktipaccess.param().abi()).ok()
    }
    pub unsafe fn GetXAAccess(&self) -> windows_core::Result<super::super::Foundation::BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetXAAccess)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetXAAccess<P0>(&self, bxaaccess: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::BOOL>,
    {
        (windows_core::Interface::vtable(self).SetXAAccess)(windows_core::Interface::as_raw(self), bxaaccess.param().abi()).ok()
    }
    pub unsafe fn RestartDtcService(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).RestartDtcService)(windows_core::Interface::as_raw(self)).ok()
    }
}
#[repr(C)]
pub struct IDtcNetworkAccessConfig_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetAnyNetworkAccess: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::BOOL) -> windows_core::HRESULT,
    pub SetAnyNetworkAccess: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::BOOL) -> windows_core::HRESULT,
    pub GetNetworkAdministrationAccess: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::BOOL) -> windows_core::HRESULT,
    pub SetNetworkAdministrationAccess: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::BOOL) -> windows_core::HRESULT,
    pub GetNetworkTransactionAccess: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::BOOL) -> windows_core::HRESULT,
    pub SetNetworkTransactionAccess: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::BOOL) -> windows_core::HRESULT,
    pub GetNetworkClientAccess: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::BOOL) -> windows_core::HRESULT,
    pub SetNetworkClientAccess: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::BOOL) -> windows_core::HRESULT,
    pub GetNetworkTIPAccess: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::BOOL) -> windows_core::HRESULT,
    pub SetNetworkTIPAccess: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::BOOL) -> windows_core::HRESULT,
    pub GetXAAccess: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::BOOL) -> windows_core::HRESULT,
    pub SetXAAccess: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::BOOL) -> windows_core::HRESULT,
    pub RestartDtcService: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IDtcNetworkAccessConfig2, IDtcNetworkAccessConfig2_Vtbl, 0xa7aa013b_eb7d_4f42_b41c_b2dec09ae034);
impl core::ops::Deref for IDtcNetworkAccessConfig2 {
    type Target = IDtcNetworkAccessConfig;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IDtcNetworkAccessConfig2, windows_core::IUnknown, IDtcNetworkAccessConfig);
impl IDtcNetworkAccessConfig2 {
    pub unsafe fn GetNetworkInboundAccess(&self) -> windows_core::Result<super::super::Foundation::BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetNetworkInboundAccess)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn GetNetworkOutboundAccess(&self) -> windows_core::Result<super::super::Foundation::BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetNetworkOutboundAccess)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetNetworkInboundAccess<P0>(&self, binbound: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::BOOL>,
    {
        (windows_core::Interface::vtable(self).SetNetworkInboundAccess)(windows_core::Interface::as_raw(self), binbound.param().abi()).ok()
    }
    pub unsafe fn SetNetworkOutboundAccess<P0>(&self, boutbound: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::BOOL>,
    {
        (windows_core::Interface::vtable(self).SetNetworkOutboundAccess)(windows_core::Interface::as_raw(self), boutbound.param().abi()).ok()
    }
    pub unsafe fn GetAuthenticationLevel(&self) -> windows_core::Result<AUTHENTICATION_LEVEL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetAuthenticationLevel)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetAuthenticationLevel(&self, authlevel: AUTHENTICATION_LEVEL) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetAuthenticationLevel)(windows_core::Interface::as_raw(self), authlevel).ok()
    }
}
#[repr(C)]
pub struct IDtcNetworkAccessConfig2_Vtbl {
    pub base__: IDtcNetworkAccessConfig_Vtbl,
    pub GetNetworkInboundAccess: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::BOOL) -> windows_core::HRESULT,
    pub GetNetworkOutboundAccess: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::BOOL) -> windows_core::HRESULT,
    pub SetNetworkInboundAccess: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::BOOL) -> windows_core::HRESULT,
    pub SetNetworkOutboundAccess: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::BOOL) -> windows_core::HRESULT,
    pub GetAuthenticationLevel: unsafe extern "system" fn(*mut core::ffi::c_void, *mut AUTHENTICATION_LEVEL) -> windows_core::HRESULT,
    pub SetAuthenticationLevel: unsafe extern "system" fn(*mut core::ffi::c_void, AUTHENTICATION_LEVEL) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IDtcNetworkAccessConfig3, IDtcNetworkAccessConfig3_Vtbl, 0x76e4b4f3_2ca5_466b_89d5_fd218ee75b49);
impl core::ops::Deref for IDtcNetworkAccessConfig3 {
    type Target = IDtcNetworkAccessConfig2;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IDtcNetworkAccessConfig3, windows_core::IUnknown, IDtcNetworkAccessConfig, IDtcNetworkAccessConfig2);
impl IDtcNetworkAccessConfig3 {
    pub unsafe fn GetLUAccess(&self) -> windows_core::Result<super::super::Foundation::BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetLUAccess)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetLUAccess<P0>(&self, bluaccess: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::BOOL>,
    {
        (windows_core::Interface::vtable(self).SetLUAccess)(windows_core::Interface::as_raw(self), bluaccess.param().abi()).ok()
    }
}
#[repr(C)]
pub struct IDtcNetworkAccessConfig3_Vtbl {
    pub base__: IDtcNetworkAccessConfig2_Vtbl,
    pub GetLUAccess: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::BOOL) -> windows_core::HRESULT,
    pub SetLUAccess: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::BOOL) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IDtcToXaHelper, IDtcToXaHelper_Vtbl, 0xa9861611_304a_11d1_9813_00a0c905416e);
impl core::ops::Deref for IDtcToXaHelper {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IDtcToXaHelper, windows_core::IUnknown);
impl IDtcToXaHelper {
    pub unsafe fn Close<P0>(&self, i_fdorecovery: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::BOOL>,
    {
        (windows_core::Interface::vtable(self).Close)(windows_core::Interface::as_raw(self), i_fdorecovery.param().abi()).ok()
    }
    pub unsafe fn TranslateTridToXid<P0>(&self, pitransaction: P0, pguidbqual: *const windows_core::GUID, pxid: *mut XID) -> windows_core::Result<()>
    where
        P0: windows_core::Param<ITransaction>,
    {
        (windows_core::Interface::vtable(self).TranslateTridToXid)(windows_core::Interface::as_raw(self), pitransaction.param().abi(), pguidbqual, pxid).ok()
    }
}
#[repr(C)]
pub struct IDtcToXaHelper_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Close: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::BOOL) -> windows_core::HRESULT,
    pub TranslateTridToXid: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *const windows_core::GUID, *mut XID) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IDtcToXaHelperFactory, IDtcToXaHelperFactory_Vtbl, 0xa9861610_304a_11d1_9813_00a0c905416e);
impl core::ops::Deref for IDtcToXaHelperFactory {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IDtcToXaHelperFactory, windows_core::IUnknown);
impl IDtcToXaHelperFactory {
    pub unsafe fn Create<P0, P1>(&self, pszdsn: P0, pszclientdllname: P1, pguidrm: *mut windows_core::GUID, ppxahelper: *mut Option<IDtcToXaHelper>) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCSTR>,
        P1: windows_core::Param<windows_core::PCSTR>,
    {
        (windows_core::Interface::vtable(self).Create)(windows_core::Interface::as_raw(self), pszdsn.param().abi(), pszclientdllname.param().abi(), pguidrm, core::mem::transmute(ppxahelper)).ok()
    }
}
#[repr(C)]
pub struct IDtcToXaHelperFactory_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Create: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCSTR, windows_core::PCSTR, *mut windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IDtcToXaHelperSinglePipe, IDtcToXaHelperSinglePipe_Vtbl, 0x47ed4971_53b3_11d1_bbb9_00c04fd658f6);
impl core::ops::Deref for IDtcToXaHelperSinglePipe {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IDtcToXaHelperSinglePipe, windows_core::IUnknown);
impl IDtcToXaHelperSinglePipe {
    pub unsafe fn XARMCreate<P0, P1>(&self, pszdsn: P0, pszclientdll: P1, pdwrmcookie: *mut u32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCSTR>,
        P1: windows_core::Param<windows_core::PCSTR>,
    {
        (windows_core::Interface::vtable(self).XARMCreate)(windows_core::Interface::as_raw(self), pszdsn.param().abi(), pszclientdll.param().abi(), pdwrmcookie).ok()
    }
    pub unsafe fn ConvertTridToXID(&self, pdwitrans: *mut u32, dwrmcookie: u32, pxid: *mut XID) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).ConvertTridToXID)(windows_core::Interface::as_raw(self), pdwitrans, dwrmcookie, pxid).ok()
    }
    pub unsafe fn EnlistWithRM<P0, P1>(&self, dwrmcookie: u32, i_pitransaction: P0, i_pitransres: P1) -> windows_core::Result<ITransactionEnlistmentAsync>
    where
        P0: windows_core::Param<ITransaction>,
        P1: windows_core::Param<ITransactionResourceAsync>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).EnlistWithRM)(windows_core::Interface::as_raw(self), dwrmcookie, i_pitransaction.param().abi(), i_pitransres.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn ReleaseRMCookie<P0>(&self, i_dwrmcookie: u32, i_fnormal: P0)
    where
        P0: windows_core::Param<super::super::Foundation::BOOL>,
    {
        (windows_core::Interface::vtable(self).ReleaseRMCookie)(windows_core::Interface::as_raw(self), i_dwrmcookie, i_fnormal.param().abi())
    }
}
#[repr(C)]
pub struct IDtcToXaHelperSinglePipe_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub XARMCreate: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCSTR, windows_core::PCSTR, *mut u32) -> windows_core::HRESULT,
    pub ConvertTridToXID: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32, u32, *mut XID) -> windows_core::HRESULT,
    pub EnlistWithRM: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ReleaseRMCookie: unsafe extern "system" fn(*mut core::ffi::c_void, u32, super::super::Foundation::BOOL),
}
windows_core::imp::define_interface!(IDtcToXaMapper, IDtcToXaMapper_Vtbl, 0x64ffabe0_7ce9_11d0_8ce6_00c04fdc877e);
impl core::ops::Deref for IDtcToXaMapper {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IDtcToXaMapper, windows_core::IUnknown);
impl IDtcToXaMapper {
    pub unsafe fn RequestNewResourceManager<P0, P1>(&self, pszdsn: P0, pszclientdllname: P1, pdwrmcookie: *mut u32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCSTR>,
        P1: windows_core::Param<windows_core::PCSTR>,
    {
        (windows_core::Interface::vtable(self).RequestNewResourceManager)(windows_core::Interface::as_raw(self), pszdsn.param().abi(), pszclientdllname.param().abi(), pdwrmcookie).ok()
    }
    pub unsafe fn TranslateTridToXid(&self, pdwitransaction: *const u32, dwrmcookie: u32, pxid: *mut XID) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).TranslateTridToXid)(windows_core::Interface::as_raw(self), pdwitransaction, dwrmcookie, pxid).ok()
    }
    pub unsafe fn EnlistResourceManager(&self, dwrmcookie: u32, pdwitransaction: *const u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).EnlistResourceManager)(windows_core::Interface::as_raw(self), dwrmcookie, pdwitransaction).ok()
    }
    pub unsafe fn ReleaseResourceManager(&self, dwrmcookie: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).ReleaseResourceManager)(windows_core::Interface::as_raw(self), dwrmcookie).ok()
    }
}
#[repr(C)]
pub struct IDtcToXaMapper_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub RequestNewResourceManager: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCSTR, windows_core::PCSTR, *mut u32) -> windows_core::HRESULT,
    pub TranslateTridToXid: unsafe extern "system" fn(*mut core::ffi::c_void, *const u32, u32, *mut XID) -> windows_core::HRESULT,
    pub EnlistResourceManager: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const u32) -> windows_core::HRESULT,
    pub ReleaseResourceManager: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IGetDispenser, IGetDispenser_Vtbl, 0xc23cc370_87ef_11ce_8081_0080c758527e);
impl core::ops::Deref for IGetDispenser {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IGetDispenser, windows_core::IUnknown);
impl IGetDispenser {
    pub unsafe fn GetDispenser(&self, iid: *const windows_core::GUID, ppvobject: *mut *mut core::ffi::c_void) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetDispenser)(windows_core::Interface::as_raw(self), iid, ppvobject).ok()
    }
}
#[repr(C)]
pub struct IGetDispenser_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetDispenser: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IKernelTransaction, IKernelTransaction_Vtbl, 0x79427a2b_f895_40e0_be79_b57dc82ed231);
impl core::ops::Deref for IKernelTransaction {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IKernelTransaction, windows_core::IUnknown);
impl IKernelTransaction {
    pub unsafe fn GetHandle(&self) -> windows_core::Result<super::super::Foundation::HANDLE> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetHandle)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
}
#[repr(C)]
pub struct IKernelTransaction_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetHandle: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::HANDLE) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ILastResourceManager, ILastResourceManager_Vtbl, 0x4d964ad4_5b33_11d3_8a91_00c04f79eb6d);
impl core::ops::Deref for ILastResourceManager {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ILastResourceManager, windows_core::IUnknown);
impl ILastResourceManager {
    pub unsafe fn TransactionCommitted(&self, pprepinfo: &[u8]) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).TransactionCommitted)(windows_core::Interface::as_raw(self), core::mem::transmute(pprepinfo.as_ptr()), pprepinfo.len().try_into().unwrap()).ok()
    }
    pub unsafe fn RecoveryDone(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).RecoveryDone)(windows_core::Interface::as_raw(self)).ok()
    }
}
#[repr(C)]
pub struct ILastResourceManager_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub TransactionCommitted: unsafe extern "system" fn(*mut core::ffi::c_void, *const u8, u32) -> windows_core::HRESULT,
    pub RecoveryDone: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IPrepareInfo, IPrepareInfo_Vtbl, 0x80c7bfd0_87ee_11ce_8081_0080c758527e);
impl core::ops::Deref for IPrepareInfo {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IPrepareInfo, windows_core::IUnknown);
impl IPrepareInfo {
    pub unsafe fn GetPrepareInfoSize(&self, pcbprepinfo: *mut u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetPrepareInfoSize)(windows_core::Interface::as_raw(self), pcbprepinfo).ok()
    }
    pub unsafe fn GetPrepareInfo(&self, pprepinfo: *mut u8) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetPrepareInfo)(windows_core::Interface::as_raw(self), pprepinfo).ok()
    }
}
#[repr(C)]
pub struct IPrepareInfo_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetPrepareInfoSize: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub GetPrepareInfo: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u8) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IPrepareInfo2, IPrepareInfo2_Vtbl, 0x5fab2547_9779_11d1_b886_00c04fb9618a);
impl core::ops::Deref for IPrepareInfo2 {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IPrepareInfo2, windows_core::IUnknown);
impl IPrepareInfo2 {
    pub unsafe fn GetPrepareInfoSize(&self) -> windows_core::Result<u32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetPrepareInfoSize)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn GetPrepareInfo(&self, pprepinfo: &mut [u8]) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetPrepareInfo)(windows_core::Interface::as_raw(self), pprepinfo.len().try_into().unwrap(), core::mem::transmute(pprepinfo.as_ptr())).ok()
    }
}
#[repr(C)]
pub struct IPrepareInfo2_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetPrepareInfoSize: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub GetPrepareInfo: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut u8) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IRMHelper, IRMHelper_Vtbl, 0xe793f6d1_f53d_11cf_a60d_00a0c905416e);
impl core::ops::Deref for IRMHelper {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IRMHelper, windows_core::IUnknown);
impl IRMHelper {
    pub unsafe fn RMCount(&self, dwctotalnumberofrms: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).RMCount)(windows_core::Interface::as_raw(self), dwctotalnumberofrms).ok()
    }
    pub unsafe fn RMInfo<P0, P1, P2>(&self, pxa_switch: *mut xa_switch_t, fcdeclcallingconv: P0, pszopenstring: P1, pszclosestring: P2, guidrmrecovery: windows_core::GUID) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::BOOL>,
        P1: windows_core::Param<windows_core::PCSTR>,
        P2: windows_core::Param<windows_core::PCSTR>,
    {
        (windows_core::Interface::vtable(self).RMInfo)(windows_core::Interface::as_raw(self), pxa_switch, fcdeclcallingconv.param().abi(), pszopenstring.param().abi(), pszclosestring.param().abi(), core::mem::transmute(guidrmrecovery)).ok()
    }
}
#[repr(C)]
pub struct IRMHelper_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub RMCount: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub RMInfo: unsafe extern "system" fn(*mut core::ffi::c_void, *mut xa_switch_t, super::super::Foundation::BOOL, windows_core::PCSTR, windows_core::PCSTR, windows_core::GUID) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IResourceManager, IResourceManager_Vtbl, 0x13741d21_87eb_11ce_8081_0080c758527e);
impl core::ops::Deref for IResourceManager {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IResourceManager, windows_core::IUnknown);
impl IResourceManager {
    pub unsafe fn Enlist<P0, P1>(&self, ptransaction: P0, pres: P1, puow: *mut BOID, pisolevel: *mut i32, ppenlist: *mut Option<ITransactionEnlistmentAsync>) -> windows_core::Result<()>
    where
        P0: windows_core::Param<ITransaction>,
        P1: windows_core::Param<ITransactionResourceAsync>,
    {
        (windows_core::Interface::vtable(self).Enlist)(windows_core::Interface::as_raw(self), ptransaction.param().abi(), pres.param().abi(), puow, pisolevel, core::mem::transmute(ppenlist)).ok()
    }
    pub unsafe fn Reenlist(&self, pprepinfo: &[u8], ltimeout: u32) -> windows_core::Result<XACTSTAT> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Reenlist)(windows_core::Interface::as_raw(self), core::mem::transmute(pprepinfo.as_ptr()), pprepinfo.len().try_into().unwrap(), ltimeout, &mut result__).map(|| result__)
    }
    pub unsafe fn ReenlistmentComplete(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).ReenlistmentComplete)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn GetDistributedTransactionManager(&self, iid: *const windows_core::GUID, ppvobject: *mut *mut core::ffi::c_void) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetDistributedTransactionManager)(windows_core::Interface::as_raw(self), iid, ppvobject).ok()
    }
}
#[repr(C)]
pub struct IResourceManager_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Enlist: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut BOID, *mut i32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Reenlist: unsafe extern "system" fn(*mut core::ffi::c_void, *const u8, u32, u32, *mut XACTSTAT) -> windows_core::HRESULT,
    pub ReenlistmentComplete: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetDistributedTransactionManager: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IResourceManager2, IResourceManager2_Vtbl, 0xd136c69a_f749_11d1_8f47_00c04f8ee57d);
impl core::ops::Deref for IResourceManager2 {
    type Target = IResourceManager;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IResourceManager2, windows_core::IUnknown, IResourceManager);
impl IResourceManager2 {
    pub unsafe fn Enlist2<P0, P1>(&self, ptransaction: P0, presasync: P1, puow: *mut BOID, pisolevel: *mut i32, pxid: *mut XID, ppenlist: *mut Option<ITransactionEnlistmentAsync>) -> windows_core::Result<()>
    where
        P0: windows_core::Param<ITransaction>,
        P1: windows_core::Param<ITransactionResourceAsync>,
    {
        (windows_core::Interface::vtable(self).Enlist2)(windows_core::Interface::as_raw(self), ptransaction.param().abi(), presasync.param().abi(), puow, pisolevel, pxid, core::mem::transmute(ppenlist)).ok()
    }
    pub unsafe fn Reenlist2(&self, pxid: *const XID, dwtimeout: u32) -> windows_core::Result<XACTSTAT> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Reenlist2)(windows_core::Interface::as_raw(self), pxid, dwtimeout, &mut result__).map(|| result__)
    }
}
#[repr(C)]
pub struct IResourceManager2_Vtbl {
    pub base__: IResourceManager_Vtbl,
    pub Enlist2: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut BOID, *mut i32, *mut XID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Reenlist2: unsafe extern "system" fn(*mut core::ffi::c_void, *const XID, u32, *mut XACTSTAT) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IResourceManagerFactory, IResourceManagerFactory_Vtbl, 0x13741d20_87eb_11ce_8081_0080c758527e);
impl core::ops::Deref for IResourceManagerFactory {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IResourceManagerFactory, windows_core::IUnknown);
impl IResourceManagerFactory {
    pub unsafe fn Create<P0, P1>(&self, pguidrm: *const windows_core::GUID, pszrmname: P0, piresmgrsink: P1) -> windows_core::Result<IResourceManager>
    where
        P0: windows_core::Param<windows_core::PCSTR>,
        P1: windows_core::Param<IResourceManagerSink>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Create)(windows_core::Interface::as_raw(self), pguidrm, pszrmname.param().abi(), piresmgrsink.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[repr(C)]
pub struct IResourceManagerFactory_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Create: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, windows_core::PCSTR, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IResourceManagerFactory2, IResourceManagerFactory2_Vtbl, 0x6b369c21_fbd2_11d1_8f47_00c04f8ee57d);
impl core::ops::Deref for IResourceManagerFactory2 {
    type Target = IResourceManagerFactory;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IResourceManagerFactory2, windows_core::IUnknown, IResourceManagerFactory);
impl IResourceManagerFactory2 {
    pub unsafe fn CreateEx<P0, P1>(&self, pguidrm: *const windows_core::GUID, pszrmname: P0, piresmgrsink: P1, riidrequested: *const windows_core::GUID, ppvresmgr: *mut *mut core::ffi::c_void) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCSTR>,
        P1: windows_core::Param<IResourceManagerSink>,
    {
        (windows_core::Interface::vtable(self).CreateEx)(windows_core::Interface::as_raw(self), pguidrm, pszrmname.param().abi(), piresmgrsink.param().abi(), riidrequested, ppvresmgr).ok()
    }
}
#[repr(C)]
pub struct IResourceManagerFactory2_Vtbl {
    pub base__: IResourceManagerFactory_Vtbl,
    pub CreateEx: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, windows_core::PCSTR, *mut core::ffi::c_void, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IResourceManagerRejoinable, IResourceManagerRejoinable_Vtbl, 0x6f6de620_b5df_4f3e_9cfa_c8aebd05172b);
impl core::ops::Deref for IResourceManagerRejoinable {
    type Target = IResourceManager2;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IResourceManagerRejoinable, windows_core::IUnknown, IResourceManager, IResourceManager2);
impl IResourceManagerRejoinable {
    pub unsafe fn Rejoin(&self, pprepinfo: &[u8], ltimeout: u32) -> windows_core::Result<XACTSTAT> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Rejoin)(windows_core::Interface::as_raw(self), core::mem::transmute(pprepinfo.as_ptr()), pprepinfo.len().try_into().unwrap(), ltimeout, &mut result__).map(|| result__)
    }
}
#[repr(C)]
pub struct IResourceManagerRejoinable_Vtbl {
    pub base__: IResourceManager2_Vtbl,
    pub Rejoin: unsafe extern "system" fn(*mut core::ffi::c_void, *const u8, u32, u32, *mut XACTSTAT) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IResourceManagerSink, IResourceManagerSink_Vtbl, 0x0d563181_defb_11ce_aed1_00aa0051e2c4);
impl core::ops::Deref for IResourceManagerSink {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IResourceManagerSink, windows_core::IUnknown);
impl IResourceManagerSink {
    pub unsafe fn TMDown(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).TMDown)(windows_core::Interface::as_raw(self)).ok()
    }
}
#[repr(C)]
pub struct IResourceManagerSink_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub TMDown: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ITipHelper, ITipHelper_Vtbl, 0x17cf72d1_bac5_11d1_b1bf_00c04fc2f3ef);
impl core::ops::Deref for ITipHelper {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ITipHelper, windows_core::IUnknown);
impl ITipHelper {
    pub unsafe fn Pull(&self, i_psztxurl: *const u8) -> windows_core::Result<ITransaction> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Pull)(windows_core::Interface::as_raw(self), i_psztxurl, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn PullAsync<P0>(&self, i_psztxurl: *const u8, i_ptippullsink: P0) -> windows_core::Result<ITransaction>
    where
        P0: windows_core::Param<ITipPullSink>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).PullAsync)(windows_core::Interface::as_raw(self), i_psztxurl, i_ptippullsink.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn GetLocalTmUrl(&self) -> windows_core::Result<*mut u8> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetLocalTmUrl)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
}
#[repr(C)]
pub struct ITipHelper_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Pull: unsafe extern "system" fn(*mut core::ffi::c_void, *const u8, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub PullAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *const u8, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetLocalTmUrl: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut u8) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ITipPullSink, ITipPullSink_Vtbl, 0x17cf72d2_bac5_11d1_b1bf_00c04fc2f3ef);
impl core::ops::Deref for ITipPullSink {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ITipPullSink, windows_core::IUnknown);
impl ITipPullSink {
    pub unsafe fn PullComplete(&self, i_hrpull: windows_core::HRESULT) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).PullComplete)(windows_core::Interface::as_raw(self), i_hrpull).ok()
    }
}
#[repr(C)]
pub struct ITipPullSink_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub PullComplete: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::HRESULT) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ITipTransaction, ITipTransaction_Vtbl, 0x17cf72d0_bac5_11d1_b1bf_00c04fc2f3ef);
impl core::ops::Deref for ITipTransaction {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ITipTransaction, windows_core::IUnknown);
impl ITipTransaction {
    pub unsafe fn Push(&self, i_pszremotetmurl: *const u8) -> windows_core::Result<windows_core::PSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Push)(windows_core::Interface::as_raw(self), i_pszremotetmurl, &mut result__).map(|| result__)
    }
    pub unsafe fn GetTransactionUrl(&self) -> windows_core::Result<windows_core::PSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetTransactionUrl)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
}
#[repr(C)]
pub struct ITipTransaction_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Push: unsafe extern "system" fn(*mut core::ffi::c_void, *const u8, *mut windows_core::PSTR) -> windows_core::HRESULT,
    pub GetTransactionUrl: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::PSTR) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ITmNodeName, ITmNodeName_Vtbl, 0x30274f88_6ee4_474e_9b95_7807bc9ef8cf);
impl core::ops::Deref for ITmNodeName {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ITmNodeName, windows_core::IUnknown);
impl ITmNodeName {
    pub unsafe fn GetNodeNameSize(&self) -> windows_core::Result<u32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetNodeNameSize)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn GetNodeName(&self, cbnodenamebuffersize: u32, pnodenamebuffer: windows_core::PWSTR) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetNodeName)(windows_core::Interface::as_raw(self), cbnodenamebuffersize, core::mem::transmute(pnodenamebuffer)).ok()
    }
}
#[repr(C)]
pub struct ITmNodeName_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetNodeNameSize: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub GetNodeName: unsafe extern "system" fn(*mut core::ffi::c_void, u32, windows_core::PWSTR) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ITransaction, ITransaction_Vtbl, 0x0fb15084_af41_11ce_bd2b_204c4f4f5020);
impl core::ops::Deref for ITransaction {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ITransaction, windows_core::IUnknown);
impl ITransaction {
    pub unsafe fn Commit<P0>(&self, fretaining: P0, grftc: u32, grfrm: u32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::BOOL>,
    {
        (windows_core::Interface::vtable(self).Commit)(windows_core::Interface::as_raw(self), fretaining.param().abi(), grftc, grfrm).ok()
    }
    pub unsafe fn Abort<P0, P1>(&self, pboidreason: *const BOID, fretaining: P0, fasync: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::BOOL>,
        P1: windows_core::Param<super::super::Foundation::BOOL>,
    {
        (windows_core::Interface::vtable(self).Abort)(windows_core::Interface::as_raw(self), pboidreason, fretaining.param().abi(), fasync.param().abi()).ok()
    }
    pub unsafe fn GetTransactionInfo(&self, pinfo: *mut XACTTRANSINFO) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetTransactionInfo)(windows_core::Interface::as_raw(self), pinfo).ok()
    }
}
#[repr(C)]
pub struct ITransaction_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Commit: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::BOOL, u32, u32) -> windows_core::HRESULT,
    pub Abort: unsafe extern "system" fn(*mut core::ffi::c_void, *const BOID, super::super::Foundation::BOOL, super::super::Foundation::BOOL) -> windows_core::HRESULT,
    pub GetTransactionInfo: unsafe extern "system" fn(*mut core::ffi::c_void, *mut XACTTRANSINFO) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ITransaction2, ITransaction2_Vtbl, 0x34021548_0065_11d3_bac1_00c04f797be2);
impl core::ops::Deref for ITransaction2 {
    type Target = ITransactionCloner;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ITransaction2, windows_core::IUnknown, ITransaction, ITransactionCloner);
impl ITransaction2 {
    pub unsafe fn GetTransactionInfo2(&self, pinfo: *mut XACTTRANSINFO) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetTransactionInfo2)(windows_core::Interface::as_raw(self), pinfo).ok()
    }
}
#[repr(C)]
pub struct ITransaction2_Vtbl {
    pub base__: ITransactionCloner_Vtbl,
    pub GetTransactionInfo2: unsafe extern "system" fn(*mut core::ffi::c_void, *mut XACTTRANSINFO) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ITransactionCloner, ITransactionCloner_Vtbl, 0x02656950_2152_11d0_944c_00a0c905416e);
impl core::ops::Deref for ITransactionCloner {
    type Target = ITransaction;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ITransactionCloner, windows_core::IUnknown, ITransaction);
impl ITransactionCloner {
    pub unsafe fn CloneWithCommitDisabled(&self) -> windows_core::Result<ITransaction> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CloneWithCommitDisabled)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[repr(C)]
pub struct ITransactionCloner_Vtbl {
    pub base__: ITransaction_Vtbl,
    pub CloneWithCommitDisabled: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ITransactionDispenser, ITransactionDispenser_Vtbl, 0x3a6ad9e1_23b9_11cf_ad60_00aa00a74ccd);
impl core::ops::Deref for ITransactionDispenser {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ITransactionDispenser, windows_core::IUnknown);
impl ITransactionDispenser {
    pub unsafe fn GetOptionsObject(&self) -> windows_core::Result<ITransactionOptions> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetOptionsObject)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn BeginTransaction<P0, P1>(&self, punkouter: P0, isolevel: i32, isoflags: u32, poptions: P1) -> windows_core::Result<ITransaction>
    where
        P0: windows_core::Param<windows_core::IUnknown>,
        P1: windows_core::Param<ITransactionOptions>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).BeginTransaction)(windows_core::Interface::as_raw(self), punkouter.param().abi(), isolevel, isoflags, poptions.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[repr(C)]
pub struct ITransactionDispenser_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetOptionsObject: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub BeginTransaction: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, i32, u32, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ITransactionEnlistmentAsync, ITransactionEnlistmentAsync_Vtbl, 0x0fb15081_af41_11ce_bd2b_204c4f4f5020);
impl core::ops::Deref for ITransactionEnlistmentAsync {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ITransactionEnlistmentAsync, windows_core::IUnknown);
impl ITransactionEnlistmentAsync {
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn PrepareRequestDone<P0>(&self, hr: windows_core::HRESULT, pmk: P0, pboidreason: *const BOID) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::Com::IMoniker>,
    {
        (windows_core::Interface::vtable(self).PrepareRequestDone)(windows_core::Interface::as_raw(self), hr, pmk.param().abi(), pboidreason).ok()
    }
    pub unsafe fn CommitRequestDone(&self, hr: windows_core::HRESULT) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).CommitRequestDone)(windows_core::Interface::as_raw(self), hr).ok()
    }
    pub unsafe fn AbortRequestDone(&self, hr: windows_core::HRESULT) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).AbortRequestDone)(windows_core::Interface::as_raw(self), hr).ok()
    }
}
#[repr(C)]
pub struct ITransactionEnlistmentAsync_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub PrepareRequestDone: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::HRESULT, *mut core::ffi::c_void, *const BOID) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    PrepareRequestDone: usize,
    pub CommitRequestDone: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::HRESULT) -> windows_core::HRESULT,
    pub AbortRequestDone: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::HRESULT) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ITransactionExport, ITransactionExport_Vtbl, 0x0141fda5_8fc0_11ce_bd18_204c4f4f5020);
impl core::ops::Deref for ITransactionExport {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ITransactionExport, windows_core::IUnknown);
impl ITransactionExport {
    pub unsafe fn Export<P0>(&self, punktransaction: P0) -> windows_core::Result<u32>
    where
        P0: windows_core::Param<windows_core::IUnknown>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Export)(windows_core::Interface::as_raw(self), punktransaction.param().abi(), &mut result__).map(|| result__)
    }
    pub unsafe fn GetTransactionCookie<P0>(&self, punktransaction: P0, rgbtransactioncookie: &mut [u8], pcbused: *mut u32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::IUnknown>,
    {
        (windows_core::Interface::vtable(self).GetTransactionCookie)(windows_core::Interface::as_raw(self), punktransaction.param().abi(), rgbtransactioncookie.len().try_into().unwrap(), core::mem::transmute(rgbtransactioncookie.as_ptr()), pcbused).ok()
    }
}
#[repr(C)]
pub struct ITransactionExport_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Export: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub GetTransactionCookie: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, u32, *mut u8, *mut u32) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ITransactionExportFactory, ITransactionExportFactory_Vtbl, 0xe1cf9b53_8745_11ce_a9ba_00aa006c3706);
impl core::ops::Deref for ITransactionExportFactory {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ITransactionExportFactory, windows_core::IUnknown);
impl ITransactionExportFactory {
    pub unsafe fn GetRemoteClassId(&self) -> windows_core::Result<windows_core::GUID> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetRemoteClassId)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn Create(&self, rgbwhereabouts: &[u8]) -> windows_core::Result<ITransactionExport> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Create)(windows_core::Interface::as_raw(self), rgbwhereabouts.len().try_into().unwrap(), core::mem::transmute(rgbwhereabouts.as_ptr()), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[repr(C)]
pub struct ITransactionExportFactory_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetRemoteClassId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::GUID) -> windows_core::HRESULT,
    pub Create: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const u8, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ITransactionImport, ITransactionImport_Vtbl, 0xe1cf9b5a_8745_11ce_a9ba_00aa006c3706);
impl core::ops::Deref for ITransactionImport {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ITransactionImport, windows_core::IUnknown);
impl ITransactionImport {
    pub unsafe fn Import<T>(&self, rgbtransactioncookie: &[u8]) -> windows_core::Result<T>
    where
        T: windows_core::Interface,
    {
        let mut result__ = core::ptr::null_mut();
        (windows_core::Interface::vtable(self).Import)(windows_core::Interface::as_raw(self), rgbtransactioncookie.len().try_into().unwrap(), core::mem::transmute(rgbtransactioncookie.as_ptr()), &T::IID, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[repr(C)]
pub struct ITransactionImport_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Import: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const u8, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ITransactionImportWhereabouts, ITransactionImportWhereabouts_Vtbl, 0x0141fda4_8fc0_11ce_bd18_204c4f4f5020);
impl core::ops::Deref for ITransactionImportWhereabouts {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ITransactionImportWhereabouts, windows_core::IUnknown);
impl ITransactionImportWhereabouts {
    pub unsafe fn GetWhereaboutsSize(&self) -> windows_core::Result<u32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetWhereaboutsSize)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn GetWhereabouts(&self, rgbwhereabouts: &mut [u8], pcbused: *mut u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetWhereabouts)(windows_core::Interface::as_raw(self), rgbwhereabouts.len().try_into().unwrap(), core::mem::transmute(rgbwhereabouts.as_ptr()), pcbused).ok()
    }
}
#[repr(C)]
pub struct ITransactionImportWhereabouts_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetWhereaboutsSize: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub GetWhereabouts: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut u8, *mut u32) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ITransactionLastEnlistmentAsync, ITransactionLastEnlistmentAsync_Vtbl, 0xc82bd533_5b30_11d3_8a91_00c04f79eb6d);
impl core::ops::Deref for ITransactionLastEnlistmentAsync {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ITransactionLastEnlistmentAsync, windows_core::IUnknown);
impl ITransactionLastEnlistmentAsync {
    pub unsafe fn TransactionOutcome(&self, xactstat: XACTSTAT, pboidreason: *const BOID) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).TransactionOutcome)(windows_core::Interface::as_raw(self), xactstat, pboidreason).ok()
    }
}
#[repr(C)]
pub struct ITransactionLastEnlistmentAsync_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub TransactionOutcome: unsafe extern "system" fn(*mut core::ffi::c_void, XACTSTAT, *const BOID) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ITransactionLastResourceAsync, ITransactionLastResourceAsync_Vtbl, 0xc82bd532_5b30_11d3_8a91_00c04f79eb6d);
impl core::ops::Deref for ITransactionLastResourceAsync {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ITransactionLastResourceAsync, windows_core::IUnknown);
impl ITransactionLastResourceAsync {
    pub unsafe fn DelegateCommit(&self, grfrm: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).DelegateCommit)(windows_core::Interface::as_raw(self), grfrm).ok()
    }
    pub unsafe fn ForgetRequest(&self, pnewuow: *const BOID) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).ForgetRequest)(windows_core::Interface::as_raw(self), pnewuow).ok()
    }
}
#[repr(C)]
pub struct ITransactionLastResourceAsync_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub DelegateCommit: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub ForgetRequest: unsafe extern "system" fn(*mut core::ffi::c_void, *const BOID) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ITransactionOptions, ITransactionOptions_Vtbl, 0x3a6ad9e0_23b9_11cf_ad60_00aa00a74ccd);
impl core::ops::Deref for ITransactionOptions {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ITransactionOptions, windows_core::IUnknown);
impl ITransactionOptions {
    pub unsafe fn SetOptions(&self, poptions: *const XACTOPT) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetOptions)(windows_core::Interface::as_raw(self), poptions).ok()
    }
    pub unsafe fn GetOptions(&self, poptions: *mut XACTOPT) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetOptions)(windows_core::Interface::as_raw(self), poptions).ok()
    }
}
#[repr(C)]
pub struct ITransactionOptions_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub SetOptions: unsafe extern "system" fn(*mut core::ffi::c_void, *const XACTOPT) -> windows_core::HRESULT,
    pub GetOptions: unsafe extern "system" fn(*mut core::ffi::c_void, *mut XACTOPT) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ITransactionOutcomeEvents, ITransactionOutcomeEvents_Vtbl, 0x3a6ad9e2_23b9_11cf_ad60_00aa00a74ccd);
impl core::ops::Deref for ITransactionOutcomeEvents {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ITransactionOutcomeEvents, windows_core::IUnknown);
impl ITransactionOutcomeEvents {
    pub unsafe fn Committed<P0>(&self, fretaining: P0, pnewuow: *const BOID, hr: windows_core::HRESULT) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::BOOL>,
    {
        (windows_core::Interface::vtable(self).Committed)(windows_core::Interface::as_raw(self), fretaining.param().abi(), pnewuow, hr).ok()
    }
    pub unsafe fn Aborted<P0>(&self, pboidreason: *const BOID, fretaining: P0, pnewuow: *const BOID, hr: windows_core::HRESULT) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::BOOL>,
    {
        (windows_core::Interface::vtable(self).Aborted)(windows_core::Interface::as_raw(self), pboidreason, fretaining.param().abi(), pnewuow, hr).ok()
    }
    pub unsafe fn HeuristicDecision(&self, dwdecision: u32, pboidreason: *const BOID, hr: windows_core::HRESULT) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).HeuristicDecision)(windows_core::Interface::as_raw(self), dwdecision, pboidreason, hr).ok()
    }
    pub unsafe fn Indoubt(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Indoubt)(windows_core::Interface::as_raw(self)).ok()
    }
}
#[repr(C)]
pub struct ITransactionOutcomeEvents_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Committed: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::BOOL, *const BOID, windows_core::HRESULT) -> windows_core::HRESULT,
    pub Aborted: unsafe extern "system" fn(*mut core::ffi::c_void, *const BOID, super::super::Foundation::BOOL, *const BOID, windows_core::HRESULT) -> windows_core::HRESULT,
    pub HeuristicDecision: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const BOID, windows_core::HRESULT) -> windows_core::HRESULT,
    pub Indoubt: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ITransactionPhase0EnlistmentAsync, ITransactionPhase0EnlistmentAsync_Vtbl, 0x82dc88e1_a954_11d1_8f88_00600895e7d5);
impl core::ops::Deref for ITransactionPhase0EnlistmentAsync {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ITransactionPhase0EnlistmentAsync, windows_core::IUnknown);
impl ITransactionPhase0EnlistmentAsync {
    pub unsafe fn Enable(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Enable)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn WaitForEnlistment(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).WaitForEnlistment)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Phase0Done(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Phase0Done)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Unenlist(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Unenlist)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn GetTransaction(&self) -> windows_core::Result<ITransaction> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetTransaction)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[repr(C)]
pub struct ITransactionPhase0EnlistmentAsync_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Enable: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub WaitForEnlistment: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Phase0Done: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Unenlist: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetTransaction: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ITransactionPhase0Factory, ITransactionPhase0Factory_Vtbl, 0x82dc88e0_a954_11d1_8f88_00600895e7d5);
impl core::ops::Deref for ITransactionPhase0Factory {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ITransactionPhase0Factory, windows_core::IUnknown);
impl ITransactionPhase0Factory {
    pub unsafe fn Create<P0>(&self, pphase0notify: P0) -> windows_core::Result<ITransactionPhase0EnlistmentAsync>
    where
        P0: windows_core::Param<ITransactionPhase0NotifyAsync>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Create)(windows_core::Interface::as_raw(self), pphase0notify.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[repr(C)]
pub struct ITransactionPhase0Factory_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Create: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ITransactionPhase0NotifyAsync, ITransactionPhase0NotifyAsync_Vtbl, 0xef081809_0c76_11d2_87a6_00c04f990f34);
impl core::ops::Deref for ITransactionPhase0NotifyAsync {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ITransactionPhase0NotifyAsync, windows_core::IUnknown);
impl ITransactionPhase0NotifyAsync {
    pub unsafe fn Phase0Request<P0>(&self, fabortinghint: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::BOOL>,
    {
        (windows_core::Interface::vtable(self).Phase0Request)(windows_core::Interface::as_raw(self), fabortinghint.param().abi()).ok()
    }
    pub unsafe fn EnlistCompleted(&self, status: windows_core::HRESULT) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).EnlistCompleted)(windows_core::Interface::as_raw(self), status).ok()
    }
}
#[repr(C)]
pub struct ITransactionPhase0NotifyAsync_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Phase0Request: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::BOOL) -> windows_core::HRESULT,
    pub EnlistCompleted: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::HRESULT) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ITransactionReceiver, ITransactionReceiver_Vtbl, 0x59313e03_b36c_11cf_a539_00aa006887c3);
impl core::ops::Deref for ITransactionReceiver {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ITransactionReceiver, windows_core::IUnknown);
impl ITransactionReceiver {
    pub unsafe fn UnmarshalPropagationToken(&self, rgbtoken: &[u8]) -> windows_core::Result<ITransaction> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).UnmarshalPropagationToken)(windows_core::Interface::as_raw(self), rgbtoken.len().try_into().unwrap(), core::mem::transmute(rgbtoken.as_ptr()), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn GetReturnTokenSize(&self) -> windows_core::Result<u32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetReturnTokenSize)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn MarshalReturnToken(&self, rgbreturntoken: &mut [u8], pcbused: *mut u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).MarshalReturnToken)(windows_core::Interface::as_raw(self), rgbreturntoken.len().try_into().unwrap(), core::mem::transmute(rgbreturntoken.as_ptr()), pcbused).ok()
    }
    pub unsafe fn Reset(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Reset)(windows_core::Interface::as_raw(self)).ok()
    }
}
#[repr(C)]
pub struct ITransactionReceiver_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub UnmarshalPropagationToken: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const u8, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetReturnTokenSize: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub MarshalReturnToken: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut u8, *mut u32) -> windows_core::HRESULT,
    pub Reset: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ITransactionReceiverFactory, ITransactionReceiverFactory_Vtbl, 0x59313e02_b36c_11cf_a539_00aa006887c3);
impl core::ops::Deref for ITransactionReceiverFactory {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ITransactionReceiverFactory, windows_core::IUnknown);
impl ITransactionReceiverFactory {
    pub unsafe fn Create(&self) -> windows_core::Result<ITransactionReceiver> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Create)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[repr(C)]
pub struct ITransactionReceiverFactory_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Create: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ITransactionResource, ITransactionResource_Vtbl, 0xee5ff7b3_4572_11d0_9452_00a0c905416e);
impl core::ops::Deref for ITransactionResource {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ITransactionResource, windows_core::IUnknown);
impl ITransactionResource {
    pub unsafe fn PrepareRequest<P0, P1, P2>(&self, fretaining: P0, grfrm: u32, fwantmoniker: P1, fsinglephase: P2) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::BOOL>,
        P1: windows_core::Param<super::super::Foundation::BOOL>,
        P2: windows_core::Param<super::super::Foundation::BOOL>,
    {
        (windows_core::Interface::vtable(self).PrepareRequest)(windows_core::Interface::as_raw(self), fretaining.param().abi(), grfrm, fwantmoniker.param().abi(), fsinglephase.param().abi()).ok()
    }
    pub unsafe fn CommitRequest(&self, grfrm: u32, pnewuow: *const BOID) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).CommitRequest)(windows_core::Interface::as_raw(self), grfrm, pnewuow).ok()
    }
    pub unsafe fn AbortRequest<P0>(&self, pboidreason: *const BOID, fretaining: P0, pnewuow: *const BOID) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::BOOL>,
    {
        (windows_core::Interface::vtable(self).AbortRequest)(windows_core::Interface::as_raw(self), pboidreason, fretaining.param().abi(), pnewuow).ok()
    }
    pub unsafe fn TMDown(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).TMDown)(windows_core::Interface::as_raw(self)).ok()
    }
}
#[repr(C)]
pub struct ITransactionResource_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub PrepareRequest: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::BOOL, u32, super::super::Foundation::BOOL, super::super::Foundation::BOOL) -> windows_core::HRESULT,
    pub CommitRequest: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const BOID) -> windows_core::HRESULT,
    pub AbortRequest: unsafe extern "system" fn(*mut core::ffi::c_void, *const BOID, super::super::Foundation::BOOL, *const BOID) -> windows_core::HRESULT,
    pub TMDown: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ITransactionResourceAsync, ITransactionResourceAsync_Vtbl, 0x69e971f0_23ce_11cf_ad60_00aa00a74ccd);
impl core::ops::Deref for ITransactionResourceAsync {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ITransactionResourceAsync, windows_core::IUnknown);
impl ITransactionResourceAsync {
    pub unsafe fn PrepareRequest<P0, P1, P2>(&self, fretaining: P0, grfrm: u32, fwantmoniker: P1, fsinglephase: P2) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::BOOL>,
        P1: windows_core::Param<super::super::Foundation::BOOL>,
        P2: windows_core::Param<super::super::Foundation::BOOL>,
    {
        (windows_core::Interface::vtable(self).PrepareRequest)(windows_core::Interface::as_raw(self), fretaining.param().abi(), grfrm, fwantmoniker.param().abi(), fsinglephase.param().abi()).ok()
    }
    pub unsafe fn CommitRequest(&self, grfrm: u32, pnewuow: *const BOID) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).CommitRequest)(windows_core::Interface::as_raw(self), grfrm, pnewuow).ok()
    }
    pub unsafe fn AbortRequest<P0>(&self, pboidreason: *const BOID, fretaining: P0, pnewuow: *const BOID) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::BOOL>,
    {
        (windows_core::Interface::vtable(self).AbortRequest)(windows_core::Interface::as_raw(self), pboidreason, fretaining.param().abi(), pnewuow).ok()
    }
    pub unsafe fn TMDown(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).TMDown)(windows_core::Interface::as_raw(self)).ok()
    }
}
#[repr(C)]
pub struct ITransactionResourceAsync_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub PrepareRequest: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::BOOL, u32, super::super::Foundation::BOOL, super::super::Foundation::BOOL) -> windows_core::HRESULT,
    pub CommitRequest: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const BOID) -> windows_core::HRESULT,
    pub AbortRequest: unsafe extern "system" fn(*mut core::ffi::c_void, *const BOID, super::super::Foundation::BOOL, *const BOID) -> windows_core::HRESULT,
    pub TMDown: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ITransactionTransmitter, ITransactionTransmitter_Vtbl, 0x59313e01_b36c_11cf_a539_00aa006887c3);
impl core::ops::Deref for ITransactionTransmitter {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ITransactionTransmitter, windows_core::IUnknown);
impl ITransactionTransmitter {
    pub unsafe fn Set<P0>(&self, ptransaction: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<ITransaction>,
    {
        (windows_core::Interface::vtable(self).Set)(windows_core::Interface::as_raw(self), ptransaction.param().abi()).ok()
    }
    pub unsafe fn GetPropagationTokenSize(&self) -> windows_core::Result<u32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetPropagationTokenSize)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn MarshalPropagationToken(&self, rgbtoken: &mut [u8], pcbused: *mut u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).MarshalPropagationToken)(windows_core::Interface::as_raw(self), rgbtoken.len().try_into().unwrap(), core::mem::transmute(rgbtoken.as_ptr()), pcbused).ok()
    }
    pub unsafe fn UnmarshalReturnToken(&self, rgbreturntoken: &[u8]) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).UnmarshalReturnToken)(windows_core::Interface::as_raw(self), rgbreturntoken.len().try_into().unwrap(), core::mem::transmute(rgbreturntoken.as_ptr())).ok()
    }
    pub unsafe fn Reset(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Reset)(windows_core::Interface::as_raw(self)).ok()
    }
}
#[repr(C)]
pub struct ITransactionTransmitter_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Set: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetPropagationTokenSize: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub MarshalPropagationToken: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut u8, *mut u32) -> windows_core::HRESULT,
    pub UnmarshalReturnToken: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const u8) -> windows_core::HRESULT,
    pub Reset: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ITransactionTransmitterFactory, ITransactionTransmitterFactory_Vtbl, 0x59313e00_b36c_11cf_a539_00aa006887c3);
impl core::ops::Deref for ITransactionTransmitterFactory {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ITransactionTransmitterFactory, windows_core::IUnknown);
impl ITransactionTransmitterFactory {
    pub unsafe fn Create(&self) -> windows_core::Result<ITransactionTransmitter> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Create)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[repr(C)]
pub struct ITransactionTransmitterFactory_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Create: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ITransactionVoterBallotAsync2, ITransactionVoterBallotAsync2_Vtbl, 0x5433376c_414d_11d3_b206_00c04fc2f3ef);
impl core::ops::Deref for ITransactionVoterBallotAsync2 {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ITransactionVoterBallotAsync2, windows_core::IUnknown);
impl ITransactionVoterBallotAsync2 {
    pub unsafe fn VoteRequestDone(&self, hr: windows_core::HRESULT, pboidreason: Option<*const BOID>) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).VoteRequestDone)(windows_core::Interface::as_raw(self), hr, core::mem::transmute(pboidreason.unwrap_or(std::ptr::null()))).ok()
    }
}
#[repr(C)]
pub struct ITransactionVoterBallotAsync2_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub VoteRequestDone: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::HRESULT, *const BOID) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ITransactionVoterFactory2, ITransactionVoterFactory2_Vtbl, 0x5433376a_414d_11d3_b206_00c04fc2f3ef);
impl core::ops::Deref for ITransactionVoterFactory2 {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ITransactionVoterFactory2, windows_core::IUnknown);
impl ITransactionVoterFactory2 {
    pub unsafe fn Create<P0, P1>(&self, ptransaction: P0, pvoternotify: P1) -> windows_core::Result<ITransactionVoterBallotAsync2>
    where
        P0: windows_core::Param<ITransaction>,
        P1: windows_core::Param<ITransactionVoterNotifyAsync2>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Create)(windows_core::Interface::as_raw(self), ptransaction.param().abi(), pvoternotify.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[repr(C)]
pub struct ITransactionVoterFactory2_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Create: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ITransactionVoterNotifyAsync2, ITransactionVoterNotifyAsync2_Vtbl, 0x5433376b_414d_11d3_b206_00c04fc2f3ef);
impl core::ops::Deref for ITransactionVoterNotifyAsync2 {
    type Target = ITransactionOutcomeEvents;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ITransactionVoterNotifyAsync2, windows_core::IUnknown, ITransactionOutcomeEvents);
impl ITransactionVoterNotifyAsync2 {
    pub unsafe fn VoteRequest(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).VoteRequest)(windows_core::Interface::as_raw(self)).ok()
    }
}
#[repr(C)]
pub struct ITransactionVoterNotifyAsync2_Vtbl {
    pub base__: ITransactionOutcomeEvents_Vtbl,
    pub VoteRequest: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IXAConfig, IXAConfig_Vtbl, 0xc8a6e3a1_9a8c_11cf_a308_00a0c905416e);
impl core::ops::Deref for IXAConfig {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IXAConfig, windows_core::IUnknown);
impl IXAConfig {
    pub unsafe fn Initialize(&self, clsidhelperdll: windows_core::GUID) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Initialize)(windows_core::Interface::as_raw(self), core::mem::transmute(clsidhelperdll)).ok()
    }
    pub unsafe fn Terminate(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Terminate)(windows_core::Interface::as_raw(self)).ok()
    }
}
#[repr(C)]
pub struct IXAConfig_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Initialize: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::GUID) -> windows_core::HRESULT,
    pub Terminate: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IXAObtainRMInfo, IXAObtainRMInfo_Vtbl, 0xe793f6d2_f53d_11cf_a60d_00a0c905416e);
impl core::ops::Deref for IXAObtainRMInfo {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IXAObtainRMInfo, windows_core::IUnknown);
impl IXAObtainRMInfo {
    pub unsafe fn ObtainRMInfo<P0>(&self, pirmhelper: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IRMHelper>,
    {
        (windows_core::Interface::vtable(self).ObtainRMInfo)(windows_core::Interface::as_raw(self), pirmhelper.param().abi()).ok()
    }
}
#[repr(C)]
pub struct IXAObtainRMInfo_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub ObtainRMInfo: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IXATransLookup, IXATransLookup_Vtbl, 0xf3b1f131_eeda_11ce_aed4_00aa0051e2c4);
impl core::ops::Deref for IXATransLookup {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IXATransLookup, windows_core::IUnknown);
impl IXATransLookup {
    pub unsafe fn Lookup(&self) -> windows_core::Result<ITransaction> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Lookup)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[repr(C)]
pub struct IXATransLookup_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Lookup: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IXATransLookup2, IXATransLookup2_Vtbl, 0xbf193c85_0d1a_4290_b88f_d2cb8873d1e7);
impl core::ops::Deref for IXATransLookup2 {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IXATransLookup2, windows_core::IUnknown);
impl IXATransLookup2 {
    pub unsafe fn Lookup(&self, pxid: *const XID) -> windows_core::Result<ITransaction> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Lookup)(windows_core::Interface::as_raw(self), pxid, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[repr(C)]
pub struct IXATransLookup2_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Lookup: unsafe extern "system" fn(*mut core::ffi::c_void, *const XID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub const CLSID_MSDtcTransaction: windows_core::GUID = windows_core::GUID::from_u128(0x39f8d76b_0928_11d1_97df_00c04fb9618a);
pub const CLSID_MSDtcTransactionManager: windows_core::GUID = windows_core::GUID::from_u128(0x5b18ab61_091d_11d1_97df_00c04fb9618a);
pub const CLUSTERRESOURCE_APPLICATIONTYPE: APPLICATIONTYPE = APPLICATIONTYPE(1i32);
pub const DTCINITIATEDRECOVERYWORK_CHECKLUSTATUS: DTCINITIATEDRECOVERYWORK = DTCINITIATEDRECOVERYWORK(1i32);
pub const DTCINITIATEDRECOVERYWORK_TMDOWN: DTCINITIATEDRECOVERYWORK = DTCINITIATEDRECOVERYWORK(3i32);
pub const DTCINITIATEDRECOVERYWORK_TRANS: DTCINITIATEDRECOVERYWORK = DTCINITIATEDRECOVERYWORK(2i32);
pub const DTCINSTALL_E_CLIENT_ALREADY_INSTALLED: i32 = 384i32;
pub const DTCINSTALL_E_SERVER_ALREADY_INSTALLED: i32 = 385i32;
pub const DTCLUCOMPARESTATESCONFIRMATION_CONFIRM: DTCLUCOMPARESTATESCONFIRMATION = DTCLUCOMPARESTATESCONFIRMATION(1i32);
pub const DTCLUCOMPARESTATESCONFIRMATION_PROTOCOL: DTCLUCOMPARESTATESCONFIRMATION = DTCLUCOMPARESTATESCONFIRMATION(2i32);
pub const DTCLUCOMPARESTATESERROR_PROTOCOL: DTCLUCOMPARESTATESERROR = DTCLUCOMPARESTATESERROR(1i32);
pub const DTCLUCOMPARESTATESRESPONSE_OK: DTCLUCOMPARESTATESRESPONSE = DTCLUCOMPARESTATESRESPONSE(1i32);
pub const DTCLUCOMPARESTATESRESPONSE_PROTOCOL: DTCLUCOMPARESTATESRESPONSE = DTCLUCOMPARESTATESRESPONSE(2i32);
pub const DTCLUCOMPARESTATE_COMMITTED: DTCLUCOMPARESTATE = DTCLUCOMPARESTATE(1i32);
pub const DTCLUCOMPARESTATE_HEURISTICCOMMITTED: DTCLUCOMPARESTATE = DTCLUCOMPARESTATE(2i32);
pub const DTCLUCOMPARESTATE_HEURISTICMIXED: DTCLUCOMPARESTATE = DTCLUCOMPARESTATE(3i32);
pub const DTCLUCOMPARESTATE_HEURISTICRESET: DTCLUCOMPARESTATE = DTCLUCOMPARESTATE(4i32);
pub const DTCLUCOMPARESTATE_INDOUBT: DTCLUCOMPARESTATE = DTCLUCOMPARESTATE(5i32);
pub const DTCLUCOMPARESTATE_RESET: DTCLUCOMPARESTATE = DTCLUCOMPARESTATE(6i32);
pub const DTCLUXLNCONFIRMATION_COLDWARMMISMATCH: DTCLUXLNCONFIRMATION = DTCLUXLNCONFIRMATION(3i32);
pub const DTCLUXLNCONFIRMATION_CONFIRM: DTCLUXLNCONFIRMATION = DTCLUXLNCONFIRMATION(1i32);
pub const DTCLUXLNCONFIRMATION_LOGNAMEMISMATCH: DTCLUXLNCONFIRMATION = DTCLUXLNCONFIRMATION(2i32);
pub const DTCLUXLNCONFIRMATION_OBSOLETE: DTCLUXLNCONFIRMATION = DTCLUXLNCONFIRMATION(4i32);
pub const DTCLUXLNERROR_COLDWARMMISMATCH: DTCLUXLNERROR = DTCLUXLNERROR(3i32);
pub const DTCLUXLNERROR_LOGNAMEMISMATCH: DTCLUXLNERROR = DTCLUXLNERROR(2i32);
pub const DTCLUXLNERROR_PROTOCOL: DTCLUXLNERROR = DTCLUXLNERROR(1i32);
pub const DTCLUXLNRESPONSE_COLDWARMMISMATCH: DTCLUXLNRESPONSE = DTCLUXLNRESPONSE(4i32);
pub const DTCLUXLNRESPONSE_LOGNAMEMISMATCH: DTCLUXLNRESPONSE = DTCLUXLNRESPONSE(3i32);
pub const DTCLUXLNRESPONSE_OK_SENDCONFIRMATION: DTCLUXLNRESPONSE = DTCLUXLNRESPONSE(2i32);
pub const DTCLUXLNRESPONSE_OK_SENDOURXLNBACK: DTCLUXLNRESPONSE = DTCLUXLNRESPONSE(1i32);
pub const DTCLUXLN_COLD: DTCLUXLN = DTCLUXLN(1i32);
pub const DTCLUXLN_WARM: DTCLUXLN = DTCLUXLN(2i32);
pub const DTC_INSTALL_OVERWRITE_CLIENT: u32 = 1u32;
pub const DTC_INSTALL_OVERWRITE_SERVER: u32 = 2u32;
pub const DTC_STATUS_CONTINUING: DTC_STATUS_ = DTC_STATUS_(5i32);
pub const DTC_STATUS_E_CANTCONTROL: DTC_STATUS_ = DTC_STATUS_(8i32);
pub const DTC_STATUS_FAILED: DTC_STATUS_ = DTC_STATUS_(9i32);
pub const DTC_STATUS_PAUSED: DTC_STATUS_ = DTC_STATUS_(4i32);
pub const DTC_STATUS_PAUSING: DTC_STATUS_ = DTC_STATUS_(3i32);
pub const DTC_STATUS_STARTED: DTC_STATUS_ = DTC_STATUS_(2i32);
pub const DTC_STATUS_STARTING: DTC_STATUS_ = DTC_STATUS_(1i32);
pub const DTC_STATUS_STOPPED: DTC_STATUS_ = DTC_STATUS_(7i32);
pub const DTC_STATUS_STOPPING: DTC_STATUS_ = DTC_STATUS_(6i32);
pub const DTC_STATUS_UNKNOWN: DTC_STATUS_ = DTC_STATUS_(0i32);
pub const INCOMING_AUTHENTICATION_REQUIRED: AUTHENTICATION_LEVEL = AUTHENTICATION_LEVEL(1i32);
pub const ISOFLAG_OPTIMISTIC: ISOFLAG = ISOFLAG(16i32);
pub const ISOFLAG_READONLY: ISOFLAG = ISOFLAG(32i32);
pub const ISOFLAG_RETAIN_ABORT: ISOFLAG = ISOFLAG(8i32);
pub const ISOFLAG_RETAIN_ABORT_DC: ISOFLAG = ISOFLAG(4i32);
pub const ISOFLAG_RETAIN_ABORT_NO: ISOFLAG = ISOFLAG(12i32);
pub const ISOFLAG_RETAIN_BOTH: ISOFLAG = ISOFLAG(10i32);
pub const ISOFLAG_RETAIN_COMMIT: ISOFLAG = ISOFLAG(2i32);
pub const ISOFLAG_RETAIN_COMMIT_DC: ISOFLAG = ISOFLAG(1i32);
pub const ISOFLAG_RETAIN_COMMIT_NO: ISOFLAG = ISOFLAG(3i32);
pub const ISOFLAG_RETAIN_DONTCARE: ISOFLAG = ISOFLAG(5i32);
pub const ISOFLAG_RETAIN_NONE: ISOFLAG = ISOFLAG(15i32);
pub const ISOLATIONLEVEL_BROWSE: ISOLATIONLEVEL = ISOLATIONLEVEL(256i32);
pub const ISOLATIONLEVEL_CHAOS: ISOLATIONLEVEL = ISOLATIONLEVEL(16i32);
pub const ISOLATIONLEVEL_CURSORSTABILITY: ISOLATIONLEVEL = ISOLATIONLEVEL(4096i32);
pub const ISOLATIONLEVEL_ISOLATED: ISOLATIONLEVEL = ISOLATIONLEVEL(1048576i32);
pub const ISOLATIONLEVEL_READCOMMITTED: ISOLATIONLEVEL = ISOLATIONLEVEL(4096i32);
pub const ISOLATIONLEVEL_READUNCOMMITTED: ISOLATIONLEVEL = ISOLATIONLEVEL(256i32);
pub const ISOLATIONLEVEL_REPEATABLEREAD: ISOLATIONLEVEL = ISOLATIONLEVEL(65536i32);
pub const ISOLATIONLEVEL_SERIALIZABLE: ISOLATIONLEVEL = ISOLATIONLEVEL(1048576i32);
pub const ISOLATIONLEVEL_UNSPECIFIED: ISOLATIONLEVEL = ISOLATIONLEVEL(-1i32);
pub const LOCAL_APPLICATIONTYPE: APPLICATIONTYPE = APPLICATIONTYPE(0i32);
pub const MAXBQUALSIZE: u32 = 64u32;
pub const MAXGTRIDSIZE: u32 = 64u32;
pub const MAXINFOSIZE: u32 = 256u32;
pub const MAX_TRAN_DESC: TX_MISC_CONSTANTS = TX_MISC_CONSTANTS(40i32);
pub const MUTUAL_AUTHENTICATION_REQUIRED: AUTHENTICATION_LEVEL = AUTHENTICATION_LEVEL(2i32);
pub const NO_AUTHENTICATION_REQUIRED: AUTHENTICATION_LEVEL = AUTHENTICATION_LEVEL(0i32);
pub const OLE_TM_CONFIG_VERSION_1: u32 = 1u32;
pub const OLE_TM_CONFIG_VERSION_2: u32 = 2u32;
pub const OLE_TM_FLAG_INTERNAL_TO_TM: u32 = 1073741824u32;
pub const OLE_TM_FLAG_NOAGILERECOVERY: u32 = 2u32;
pub const OLE_TM_FLAG_NODEMANDSTART: u32 = 1u32;
pub const OLE_TM_FLAG_NONE: u32 = 0u32;
pub const OLE_TM_FLAG_QUERY_SERVICE_LOCKSTATUS: u32 = 2147483648u32;
pub const RMNAMESZ: u32 = 32u32;
pub const TMASYNC: i32 = -2147483648i32;
pub const TMENDRSCAN: i32 = 8388608i32;
pub const TMER_INVAL: i32 = -2i32;
pub const TMER_PROTO: i32 = -3i32;
pub const TMER_TMERR: i32 = -1i32;
pub const TMFAIL: i32 = 536870912i32;
pub const TMJOIN: i32 = 2097152i32;
pub const TMMIGRATE: i32 = 1048576i32;
pub const TMMULTIPLE: i32 = 4194304i32;
pub const TMNOFLAGS: i32 = 0i32;
pub const TMNOMIGRATE: i32 = 2i32;
pub const TMNOWAIT: i32 = 268435456i32;
pub const TMONEPHASE: i32 = 1073741824i32;
pub const TMREGISTER: i32 = 1i32;
pub const TMRESUME: i32 = 134217728i32;
pub const TMSTARTRSCAN: i32 = 16777216i32;
pub const TMSUCCESS: i32 = 67108864i32;
pub const TMSUSPEND: i32 = 33554432i32;
pub const TMUSEASYNC: i32 = 4i32;
pub const TM_JOIN: u32 = 2u32;
pub const TM_OK: u32 = 0u32;
pub const TM_RESUME: u32 = 1u32;
pub const XACTCONST_TIMEOUTINFINITE: XACTCONST = XACTCONST(0i32);
pub const XACTHEURISTIC_ABORT: XACTHEURISTIC = XACTHEURISTIC(1i32);
pub const XACTHEURISTIC_COMMIT: XACTHEURISTIC = XACTHEURISTIC(2i32);
pub const XACTHEURISTIC_DAMAGE: XACTHEURISTIC = XACTHEURISTIC(3i32);
pub const XACTHEURISTIC_DANGER: XACTHEURISTIC = XACTHEURISTIC(4i32);
pub const XACTRM_NOREADONLYPREPARES: XACTRM = XACTRM(2i32);
pub const XACTRM_OPTIMISTICLASTWINS: XACTRM = XACTRM(1i32);
pub const XACTSTAT_ABORTED: XACTSTAT = XACTSTAT(512i32);
pub const XACTSTAT_ABORTING: XACTSTAT = XACTSTAT(256i32);
pub const XACTSTAT_ALL: XACTSTAT = XACTSTAT(524287i32);
pub const XACTSTAT_CLOSED: XACTSTAT = XACTSTAT(262144i32);
pub const XACTSTAT_COMMITRETAINING: XACTSTAT = XACTSTAT(128i32);
pub const XACTSTAT_COMMITTED: XACTSTAT = XACTSTAT(1024i32);
pub const XACTSTAT_COMMITTING: XACTSTAT = XACTSTAT(64i32);
pub const XACTSTAT_FORCED_ABORT: XACTSTAT = XACTSTAT(32768i32);
pub const XACTSTAT_FORCED_COMMIT: XACTSTAT = XACTSTAT(65536i32);
pub const XACTSTAT_HEURISTIC_ABORT: XACTSTAT = XACTSTAT(2048i32);
pub const XACTSTAT_HEURISTIC_COMMIT: XACTSTAT = XACTSTAT(4096i32);
pub const XACTSTAT_HEURISTIC_DAMAGE: XACTSTAT = XACTSTAT(8192i32);
pub const XACTSTAT_HEURISTIC_DANGER: XACTSTAT = XACTSTAT(16384i32);
pub const XACTSTAT_INDOUBT: XACTSTAT = XACTSTAT(131072i32);
pub const XACTSTAT_NONE: XACTSTAT = XACTSTAT(0i32);
pub const XACTSTAT_NOTPREPARED: XACTSTAT = XACTSTAT(524227i32);
pub const XACTSTAT_OPEN: XACTSTAT = XACTSTAT(3i32);
pub const XACTSTAT_OPENNORMAL: XACTSTAT = XACTSTAT(1i32);
pub const XACTSTAT_OPENREFUSED: XACTSTAT = XACTSTAT(2i32);
pub const XACTSTAT_PREPARED: XACTSTAT = XACTSTAT(8i32);
pub const XACTSTAT_PREPARERETAINED: XACTSTAT = XACTSTAT(32i32);
pub const XACTSTAT_PREPARERETAINING: XACTSTAT = XACTSTAT(16i32);
pub const XACTSTAT_PREPARING: XACTSTAT = XACTSTAT(4i32);
pub const XACTTC_ASYNC: XACTTC = XACTTC(4i32);
pub const XACTTC_ASYNC_PHASEONE: XACTTC = XACTTC(4i32);
pub const XACTTC_NONE: XACTTC = XACTTC(0i32);
pub const XACTTC_SYNC: XACTTC = XACTTC(2i32);
pub const XACTTC_SYNC_PHASEONE: XACTTC = XACTTC(1i32);
pub const XACTTC_SYNC_PHASETWO: XACTTC = XACTTC(2i32);
pub const XACT_E_CONNECTION_REQUEST_DENIED: XACT_DTC_CONSTANTS = XACT_DTC_CONSTANTS(-2147168000i32);
pub const XACT_E_DUPLICATE_GUID: XACT_DTC_CONSTANTS = XACT_DTC_CONSTANTS(-2147167998i32);
pub const XACT_E_DUPLICATE_LU: XACT_DTC_CONSTANTS = XACT_DTC_CONSTANTS(-2147167991i32);
pub const XACT_E_DUPLICATE_TRANSID: XACT_DTC_CONSTANTS = XACT_DTC_CONSTANTS(-2147167989i32);
pub const XACT_E_LRMRECOVERYALREADYDONE: XACT_DTC_CONSTANTS = XACT_DTC_CONSTANTS(-2147167982i32);
pub const XACT_E_LU_BUSY: XACT_DTC_CONSTANTS = XACT_DTC_CONSTANTS(-2147167988i32);
pub const XACT_E_LU_DOWN: XACT_DTC_CONSTANTS = XACT_DTC_CONSTANTS(-2147167986i32);
pub const XACT_E_LU_NOT_CONNECTED: XACT_DTC_CONSTANTS = XACT_DTC_CONSTANTS(-2147167990i32);
pub const XACT_E_LU_NOT_FOUND: XACT_DTC_CONSTANTS = XACT_DTC_CONSTANTS(-2147167992i32);
pub const XACT_E_LU_NO_RECOVERY_PROCESS: XACT_DTC_CONSTANTS = XACT_DTC_CONSTANTS(-2147167987i32);
pub const XACT_E_LU_RECOVERING: XACT_DTC_CONSTANTS = XACT_DTC_CONSTANTS(-2147167985i32);
pub const XACT_E_LU_RECOVERY_MISMATCH: XACT_DTC_CONSTANTS = XACT_DTC_CONSTANTS(-2147167984i32);
pub const XACT_E_NOLASTRESOURCEINTERFACE: XACT_DTC_CONSTANTS = XACT_DTC_CONSTANTS(-2147167981i32);
pub const XACT_E_NOTSINGLEPHASE: XACT_DTC_CONSTANTS = XACT_DTC_CONSTANTS(-2147167997i32);
pub const XACT_E_PROTOCOL: XACT_DTC_CONSTANTS = XACT_DTC_CONSTANTS(-2147167995i32);
pub const XACT_E_RECOVERYALREADYDONE: XACT_DTC_CONSTANTS = XACT_DTC_CONSTANTS(-2147167996i32);
pub const XACT_E_RECOVERY_FAILED: XACT_DTC_CONSTANTS = XACT_DTC_CONSTANTS(-2147167993i32);
pub const XACT_E_RM_FAILURE: XACT_DTC_CONSTANTS = XACT_DTC_CONSTANTS(-2147167994i32);
pub const XACT_E_RM_UNAVAILABLE: XACT_DTC_CONSTANTS = XACT_DTC_CONSTANTS(-2147167983i32);
pub const XACT_E_TOOMANY_ENLISTMENTS: XACT_DTC_CONSTANTS = XACT_DTC_CONSTANTS(-2147167999i32);
pub const XACT_OK_NONOTIFY: XACT_DTC_CONSTANTS = XACT_DTC_CONSTANTS(315649i32);
pub const XACT_S_NONOTIFY: XACT_DTC_CONSTANTS = XACT_DTC_CONSTANTS(315648i32);
pub const XAER_ASYNC: i32 = -2i32;
pub const XAER_DUPID: i32 = -8i32;
pub const XAER_INVAL: i32 = -5i32;
pub const XAER_NOTA: i32 = -4i32;
pub const XAER_OUTSIDE: i32 = -9i32;
pub const XAER_PROTO: i32 = -6i32;
pub const XAER_RMERR: i32 = -3i32;
pub const XAER_RMFAIL: i32 = -7i32;
pub const XA_FMTID_DTC: u32 = 4478019u32;
pub const XA_FMTID_DTC_VER1: u32 = 21255235u32;
pub const XA_HEURCOM: u32 = 7u32;
pub const XA_HEURHAZ: u32 = 8u32;
pub const XA_HEURMIX: u32 = 5u32;
pub const XA_HEURRB: u32 = 6u32;
pub const XA_NOMIGRATE: u32 = 9u32;
pub const XA_OK: u32 = 0u32;
pub const XA_RBBASE: u32 = 100u32;
pub const XA_RBCOMMFAIL: u32 = 101u32;
pub const XA_RBDEADLOCK: u32 = 102u32;
pub const XA_RBEND: u32 = 107u32;
pub const XA_RBINTEGRITY: u32 = 103u32;
pub const XA_RBOTHER: u32 = 104u32;
pub const XA_RBPROTO: u32 = 105u32;
pub const XA_RBROLLBACK: u32 = 100u32;
pub const XA_RBTIMEOUT: u32 = 106u32;
pub const XA_RBTRANSIENT: u32 = 107u32;
pub const XA_RDONLY: u32 = 3u32;
pub const XA_RETRY: u32 = 4u32;
pub const XA_SWITCH_F_DTC: u32 = 1u32;
pub const XIDDATASIZE: u32 = 128u32;
pub const dwUSER_MS_SQLSERVER: XACT_DTC_CONSTANTS = XACT_DTC_CONSTANTS(65535i32);
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct APPLICATIONTYPE(pub i32);
impl windows_core::TypeKind for APPLICATIONTYPE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for APPLICATIONTYPE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("APPLICATIONTYPE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct AUTHENTICATION_LEVEL(pub i32);
impl windows_core::TypeKind for AUTHENTICATION_LEVEL {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for AUTHENTICATION_LEVEL {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("AUTHENTICATION_LEVEL").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct DTCINITIATEDRECOVERYWORK(pub i32);
impl windows_core::TypeKind for DTCINITIATEDRECOVERYWORK {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for DTCINITIATEDRECOVERYWORK {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("DTCINITIATEDRECOVERYWORK").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct DTCLUCOMPARESTATE(pub i32);
impl windows_core::TypeKind for DTCLUCOMPARESTATE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for DTCLUCOMPARESTATE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("DTCLUCOMPARESTATE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct DTCLUCOMPARESTATESCONFIRMATION(pub i32);
impl windows_core::TypeKind for DTCLUCOMPARESTATESCONFIRMATION {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for DTCLUCOMPARESTATESCONFIRMATION {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("DTCLUCOMPARESTATESCONFIRMATION").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct DTCLUCOMPARESTATESERROR(pub i32);
impl windows_core::TypeKind for DTCLUCOMPARESTATESERROR {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for DTCLUCOMPARESTATESERROR {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("DTCLUCOMPARESTATESERROR").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct DTCLUCOMPARESTATESRESPONSE(pub i32);
impl windows_core::TypeKind for DTCLUCOMPARESTATESRESPONSE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for DTCLUCOMPARESTATESRESPONSE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("DTCLUCOMPARESTATESRESPONSE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct DTCLUXLN(pub i32);
impl windows_core::TypeKind for DTCLUXLN {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for DTCLUXLN {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("DTCLUXLN").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct DTCLUXLNCONFIRMATION(pub i32);
impl windows_core::TypeKind for DTCLUXLNCONFIRMATION {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for DTCLUXLNCONFIRMATION {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("DTCLUXLNCONFIRMATION").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct DTCLUXLNERROR(pub i32);
impl windows_core::TypeKind for DTCLUXLNERROR {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for DTCLUXLNERROR {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("DTCLUXLNERROR").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct DTCLUXLNRESPONSE(pub i32);
impl windows_core::TypeKind for DTCLUXLNRESPONSE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for DTCLUXLNRESPONSE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("DTCLUXLNRESPONSE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct DTC_STATUS_(pub i32);
impl windows_core::TypeKind for DTC_STATUS_ {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for DTC_STATUS_ {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("DTC_STATUS_").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct ISOFLAG(pub i32);
impl windows_core::TypeKind for ISOFLAG {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for ISOFLAG {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("ISOFLAG").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct ISOLATIONLEVEL(pub i32);
impl windows_core::TypeKind for ISOLATIONLEVEL {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for ISOLATIONLEVEL {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("ISOLATIONLEVEL").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct TX_MISC_CONSTANTS(pub i32);
impl windows_core::TypeKind for TX_MISC_CONSTANTS {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for TX_MISC_CONSTANTS {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("TX_MISC_CONSTANTS").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct XACTCONST(pub i32);
impl windows_core::TypeKind for XACTCONST {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for XACTCONST {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("XACTCONST").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct XACTHEURISTIC(pub i32);
impl windows_core::TypeKind for XACTHEURISTIC {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for XACTHEURISTIC {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("XACTHEURISTIC").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct XACTRM(pub i32);
impl windows_core::TypeKind for XACTRM {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for XACTRM {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("XACTRM").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct XACTSTAT(pub i32);
impl windows_core::TypeKind for XACTSTAT {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for XACTSTAT {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("XACTSTAT").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct XACTTC(pub i32);
impl windows_core::TypeKind for XACTTC {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for XACTTC {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("XACTTC").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct XACT_DTC_CONSTANTS(pub i32);
impl windows_core::TypeKind for XACT_DTC_CONSTANTS {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for XACT_DTC_CONSTANTS {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("XACT_DTC_CONSTANTS").field(&self.0).finish()
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct BOID {
    pub rgb: [u8; 16],
}
impl windows_core::TypeKind for BOID {
    type TypeKind = windows_core::CopyType;
}
impl Default for BOID {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct OLE_TM_CONFIG_PARAMS_V1 {
    pub dwVersion: u32,
    pub dwcConcurrencyHint: u32,
}
impl windows_core::TypeKind for OLE_TM_CONFIG_PARAMS_V1 {
    type TypeKind = windows_core::CopyType;
}
impl Default for OLE_TM_CONFIG_PARAMS_V1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct OLE_TM_CONFIG_PARAMS_V2 {
    pub dwVersion: u32,
    pub dwcConcurrencyHint: u32,
    pub applicationType: APPLICATIONTYPE,
    pub clusterResourceId: windows_core::GUID,
}
impl windows_core::TypeKind for OLE_TM_CONFIG_PARAMS_V2 {
    type TypeKind = windows_core::CopyType;
}
impl Default for OLE_TM_CONFIG_PARAMS_V2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct PROXY_CONFIG_PARAMS {
    pub wcThreadsMax: u16,
}
impl windows_core::TypeKind for PROXY_CONFIG_PARAMS {
    type TypeKind = windows_core::CopyType;
}
impl Default for PROXY_CONFIG_PARAMS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct XACTOPT {
    pub ulTimeout: u32,
    pub szDescription: [u8; 40],
}
impl windows_core::TypeKind for XACTOPT {
    type TypeKind = windows_core::CopyType;
}
impl Default for XACTOPT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct XACTSTATS {
    pub cOpen: u32,
    pub cCommitting: u32,
    pub cCommitted: u32,
    pub cAborting: u32,
    pub cAborted: u32,
    pub cInDoubt: u32,
    pub cHeuristicDecision: u32,
    pub timeTransactionsUp: super::super::Foundation::FILETIME,
}
impl windows_core::TypeKind for XACTSTATS {
    type TypeKind = windows_core::CopyType;
}
impl Default for XACTSTATS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct XACTTRANSINFO {
    pub uow: BOID,
    pub isoLevel: i32,
    pub isoFlags: u32,
    pub grfTCSupported: u32,
    pub grfRMSupported: u32,
    pub grfTCSupportedRetaining: u32,
    pub grfRMSupportedRetaining: u32,
}
impl windows_core::TypeKind for XACTTRANSINFO {
    type TypeKind = windows_core::CopyType;
}
impl Default for XACTTRANSINFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct XID {
    pub formatID: i32,
    pub gtrid_length: i32,
    pub bqual_length: i32,
    pub data: [i8; 128],
}
impl windows_core::TypeKind for XID {
    type TypeKind = windows_core::CopyType;
}
impl Default for XID {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct xa_switch_t {
    pub name: [i8; 32],
    pub flags: i32,
    pub version: i32,
    pub xa_open_entry: isize,
    pub xa_close_entry: isize,
    pub xa_start_entry: isize,
    pub xa_end_entry: isize,
    pub xa_rollback_entry: isize,
    pub xa_prepare_entry: isize,
    pub xa_commit_entry: isize,
    pub xa_recover_entry: isize,
    pub xa_forget_entry: isize,
    pub xa_complete_entry: isize,
}
impl windows_core::TypeKind for xa_switch_t {
    type TypeKind = windows_core::CopyType;
}
impl Default for xa_switch_t {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type DTC_GET_TRANSACTION_MANAGER = Option<unsafe extern "system" fn(pszhost: windows_core::PCSTR, psztmname: windows_core::PCSTR, rid: *const windows_core::GUID, dwreserved1: u32, wcbreserved2: u16, pvreserved2: *mut core::ffi::c_void, ppvobject: *mut *mut core::ffi::c_void) -> windows_core::HRESULT>;
pub type DTC_GET_TRANSACTION_MANAGER_EX_A = Option<unsafe extern "system" fn(i_pszhost: windows_core::PCSTR, i_psztmname: windows_core::PCSTR, i_riid: *const windows_core::GUID, i_grfoptions: u32, i_pvconfigparams: *mut core::ffi::c_void, o_ppvobject: *mut *mut core::ffi::c_void) -> windows_core::HRESULT>;
pub type DTC_GET_TRANSACTION_MANAGER_EX_W = Option<unsafe extern "system" fn(i_pwszhost: windows_core::PCWSTR, i_pwsztmname: windows_core::PCWSTR, i_riid: *const windows_core::GUID, i_grfoptions: u32, i_pvconfigparams: *mut core::ffi::c_void, o_ppvobject: *mut *mut core::ffi::c_void) -> windows_core::HRESULT>;
pub type DTC_INSTALL_CLIENT = Option<unsafe extern "system" fn(i_pszremotetmhostname: *mut i8, i_dwprotocol: u32, i_dwoverwrite: u32) -> windows_core::HRESULT>;
pub type XA_CLOSE_EPT = Option<unsafe extern "system" fn(param0: windows_core::PCSTR, param1: i32, param2: i32) -> i32>;
pub type XA_COMMIT_EPT = Option<unsafe extern "system" fn(param0: *mut XID, param1: i32, param2: i32) -> i32>;
pub type XA_COMPLETE_EPT = Option<unsafe extern "system" fn(param0: *mut i32, param1: *mut i32, param2: i32, param3: i32) -> i32>;
pub type XA_END_EPT = Option<unsafe extern "system" fn(param0: *mut XID, param1: i32, param2: i32) -> i32>;
pub type XA_FORGET_EPT = Option<unsafe extern "system" fn(param0: *mut XID, param1: i32, param2: i32) -> i32>;
pub type XA_OPEN_EPT = Option<unsafe extern "system" fn(param0: windows_core::PCSTR, param1: i32, param2: i32) -> i32>;
pub type XA_PREPARE_EPT = Option<unsafe extern "system" fn(param0: *mut XID, param1: i32, param2: i32) -> i32>;
pub type XA_RECOVER_EPT = Option<unsafe extern "system" fn(param0: *mut XID, param1: i32, param2: i32, param3: i32) -> i32>;
pub type XA_ROLLBACK_EPT = Option<unsafe extern "system" fn(param0: *mut XID, param1: i32, param2: i32) -> i32>;
pub type XA_START_EPT = Option<unsafe extern "system" fn(param0: *mut XID, param1: i32, param2: i32) -> i32>;
#[cfg(feature = "implement")]
core::include!("impl.rs");
