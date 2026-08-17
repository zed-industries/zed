#[inline]
pub unsafe fn DtcGetTransactionManager<P0, P1>(i_pszhost: P0, i_psztmname: P1, i_riid: *const windows_core::GUID, i_dwreserved1: u32, i_wcbreserved2: u16, i_pvreserved2: Option<*const core::ffi::c_void>, o_ppvobject: *mut *mut core::ffi::c_void) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("xolehlp.dll" "C" fn DtcGetTransactionManager(i_pszhost : windows_core::PCSTR, i_psztmname : windows_core::PCSTR, i_riid : *const windows_core::GUID, i_dwreserved1 : u32, i_wcbreserved2 : u16, i_pvreserved2 : *const core::ffi::c_void, o_ppvobject : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe { DtcGetTransactionManager(i_pszhost.param().abi(), i_psztmname.param().abi(), i_riid, i_dwreserved1, i_wcbreserved2, i_pvreserved2.unwrap_or(core::mem::zeroed()) as _, o_ppvobject as _).ok() }
}
#[inline]
pub unsafe fn DtcGetTransactionManagerC<P0, P1>(i_pszhost: P0, i_psztmname: P1, i_riid: *const windows_core::GUID, i_dwreserved1: u32, i_wcbreserved2: u16, i_pvreserved2: Option<*const core::ffi::c_void>, o_ppvobject: *mut *mut core::ffi::c_void) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("xolehlp.dll" "C" fn DtcGetTransactionManagerC(i_pszhost : windows_core::PCSTR, i_psztmname : windows_core::PCSTR, i_riid : *const windows_core::GUID, i_dwreserved1 : u32, i_wcbreserved2 : u16, i_pvreserved2 : *const core::ffi::c_void, o_ppvobject : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe { DtcGetTransactionManagerC(i_pszhost.param().abi(), i_psztmname.param().abi(), i_riid, i_dwreserved1, i_wcbreserved2, i_pvreserved2.unwrap_or(core::mem::zeroed()) as _, o_ppvobject as _).ok() }
}
#[inline]
pub unsafe fn DtcGetTransactionManagerExA<P0, P1>(i_pszhost: P0, i_psztmname: P1, i_riid: *const windows_core::GUID, i_grfoptions: u32, i_pvconfigparams: *mut core::ffi::c_void, o_ppvobject: *mut *mut core::ffi::c_void) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("xolehlp.dll" "C" fn DtcGetTransactionManagerExA(i_pszhost : windows_core::PCSTR, i_psztmname : windows_core::PCSTR, i_riid : *const windows_core::GUID, i_grfoptions : u32, i_pvconfigparams : *mut core::ffi::c_void, o_ppvobject : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe { DtcGetTransactionManagerExA(i_pszhost.param().abi(), i_psztmname.param().abi(), i_riid, i_grfoptions, i_pvconfigparams as _, o_ppvobject as _).ok() }
}
#[inline]
pub unsafe fn DtcGetTransactionManagerExW<P0, P1>(i_pwszhost: P0, i_pwsztmname: P1, i_riid: *const windows_core::GUID, i_grfoptions: u32, i_pvconfigparams: *mut core::ffi::c_void, o_ppvobject: *mut *mut core::ffi::c_void) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("xolehlp.dll" "C" fn DtcGetTransactionManagerExW(i_pwszhost : windows_core::PCWSTR, i_pwsztmname : windows_core::PCWSTR, i_riid : *const windows_core::GUID, i_grfoptions : u32, i_pvconfigparams : *mut core::ffi::c_void, o_ppvobject : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe { DtcGetTransactionManagerExW(i_pwszhost.param().abi(), i_pwsztmname.param().abi(), i_riid, i_grfoptions, i_pvconfigparams as _, o_ppvobject as _).ok() }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct APPLICATIONTYPE(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct AUTHENTICATION_LEVEL(pub i32);
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct BOID {
    pub rgb: [u8; 16],
}
impl Default for BOID {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const CLSID_MSDtcTransaction: windows_core::GUID = windows_core::GUID::from_u128(0x39f8d76b_0928_11d1_97df_00c04fb9618a);
pub const CLSID_MSDtcTransactionManager: windows_core::GUID = windows_core::GUID::from_u128(0x5b18ab61_091d_11d1_97df_00c04fb9618a);
pub const CLUSTERRESOURCE_APPLICATIONTYPE: APPLICATIONTYPE = APPLICATIONTYPE(1i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct DTCINITIATEDRECOVERYWORK(pub i32);
pub const DTCINITIATEDRECOVERYWORK_CHECKLUSTATUS: DTCINITIATEDRECOVERYWORK = DTCINITIATEDRECOVERYWORK(1i32);
pub const DTCINITIATEDRECOVERYWORK_TMDOWN: DTCINITIATEDRECOVERYWORK = DTCINITIATEDRECOVERYWORK(3i32);
pub const DTCINITIATEDRECOVERYWORK_TRANS: DTCINITIATEDRECOVERYWORK = DTCINITIATEDRECOVERYWORK(2i32);
pub const DTCINSTALL_E_CLIENT_ALREADY_INSTALLED: i32 = 384i32;
pub const DTCINSTALL_E_SERVER_ALREADY_INSTALLED: i32 = 385i32;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct DTCLUCOMPARESTATE(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct DTCLUCOMPARESTATESCONFIRMATION(pub i32);
pub const DTCLUCOMPARESTATESCONFIRMATION_CONFIRM: DTCLUCOMPARESTATESCONFIRMATION = DTCLUCOMPARESTATESCONFIRMATION(1i32);
pub const DTCLUCOMPARESTATESCONFIRMATION_PROTOCOL: DTCLUCOMPARESTATESCONFIRMATION = DTCLUCOMPARESTATESCONFIRMATION(2i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct DTCLUCOMPARESTATESERROR(pub i32);
pub const DTCLUCOMPARESTATESERROR_PROTOCOL: DTCLUCOMPARESTATESERROR = DTCLUCOMPARESTATESERROR(1i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct DTCLUCOMPARESTATESRESPONSE(pub i32);
pub const DTCLUCOMPARESTATESRESPONSE_OK: DTCLUCOMPARESTATESRESPONSE = DTCLUCOMPARESTATESRESPONSE(1i32);
pub const DTCLUCOMPARESTATESRESPONSE_PROTOCOL: DTCLUCOMPARESTATESRESPONSE = DTCLUCOMPARESTATESRESPONSE(2i32);
pub const DTCLUCOMPARESTATE_COMMITTED: DTCLUCOMPARESTATE = DTCLUCOMPARESTATE(1i32);
pub const DTCLUCOMPARESTATE_HEURISTICCOMMITTED: DTCLUCOMPARESTATE = DTCLUCOMPARESTATE(2i32);
pub const DTCLUCOMPARESTATE_HEURISTICMIXED: DTCLUCOMPARESTATE = DTCLUCOMPARESTATE(3i32);
pub const DTCLUCOMPARESTATE_HEURISTICRESET: DTCLUCOMPARESTATE = DTCLUCOMPARESTATE(4i32);
pub const DTCLUCOMPARESTATE_INDOUBT: DTCLUCOMPARESTATE = DTCLUCOMPARESTATE(5i32);
pub const DTCLUCOMPARESTATE_RESET: DTCLUCOMPARESTATE = DTCLUCOMPARESTATE(6i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct DTCLUXLN(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct DTCLUXLNCONFIRMATION(pub i32);
pub const DTCLUXLNCONFIRMATION_COLDWARMMISMATCH: DTCLUXLNCONFIRMATION = DTCLUXLNCONFIRMATION(3i32);
pub const DTCLUXLNCONFIRMATION_CONFIRM: DTCLUXLNCONFIRMATION = DTCLUXLNCONFIRMATION(1i32);
pub const DTCLUXLNCONFIRMATION_LOGNAMEMISMATCH: DTCLUXLNCONFIRMATION = DTCLUXLNCONFIRMATION(2i32);
pub const DTCLUXLNCONFIRMATION_OBSOLETE: DTCLUXLNCONFIRMATION = DTCLUXLNCONFIRMATION(4i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct DTCLUXLNERROR(pub i32);
pub const DTCLUXLNERROR_COLDWARMMISMATCH: DTCLUXLNERROR = DTCLUXLNERROR(3i32);
pub const DTCLUXLNERROR_LOGNAMEMISMATCH: DTCLUXLNERROR = DTCLUXLNERROR(2i32);
pub const DTCLUXLNERROR_PROTOCOL: DTCLUXLNERROR = DTCLUXLNERROR(1i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct DTCLUXLNRESPONSE(pub i32);
pub const DTCLUXLNRESPONSE_COLDWARMMISMATCH: DTCLUXLNRESPONSE = DTCLUXLNRESPONSE(4i32);
pub const DTCLUXLNRESPONSE_LOGNAMEMISMATCH: DTCLUXLNRESPONSE = DTCLUXLNRESPONSE(3i32);
pub const DTCLUXLNRESPONSE_OK_SENDCONFIRMATION: DTCLUXLNRESPONSE = DTCLUXLNRESPONSE(2i32);
pub const DTCLUXLNRESPONSE_OK_SENDOURXLNBACK: DTCLUXLNRESPONSE = DTCLUXLNRESPONSE(1i32);
pub const DTCLUXLN_COLD: DTCLUXLN = DTCLUXLN(1i32);
pub const DTCLUXLN_WARM: DTCLUXLN = DTCLUXLN(2i32);
pub type DTC_GET_TRANSACTION_MANAGER = Option<unsafe extern "system" fn(pszhost: windows_core::PCSTR, psztmname: windows_core::PCSTR, rid: *const windows_core::GUID, dwreserved1: u32, wcbreserved2: u16, pvreserved2: *mut core::ffi::c_void, ppvobject: *mut *mut core::ffi::c_void) -> windows_core::HRESULT>;
pub type DTC_GET_TRANSACTION_MANAGER_EX_A = Option<unsafe extern "system" fn(i_pszhost: windows_core::PCSTR, i_psztmname: windows_core::PCSTR, i_riid: *const windows_core::GUID, i_grfoptions: u32, i_pvconfigparams: *mut core::ffi::c_void, o_ppvobject: *mut *mut core::ffi::c_void) -> windows_core::HRESULT>;
pub type DTC_GET_TRANSACTION_MANAGER_EX_W = Option<unsafe extern "system" fn(i_pwszhost: windows_core::PCWSTR, i_pwsztmname: windows_core::PCWSTR, i_riid: *const windows_core::GUID, i_grfoptions: u32, i_pvconfigparams: *mut core::ffi::c_void, o_ppvobject: *mut *mut core::ffi::c_void) -> windows_core::HRESULT>;
pub type DTC_INSTALL_CLIENT = Option<unsafe extern "system" fn(i_pszremotetmhostname: *mut i8, i_dwprotocol: u32, i_dwoverwrite: u32) -> windows_core::HRESULT>;
pub const DTC_INSTALL_OVERWRITE_CLIENT: u32 = 1u32;
pub const DTC_INSTALL_OVERWRITE_SERVER: u32 = 2u32;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct DTC_STATUS_(pub i32);
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
windows_core::imp::define_interface!(IDtcLuConfigure, IDtcLuConfigure_Vtbl, 0x4131e760_1aea_11d0_944b_00a0c905416e);
windows_core::imp::interface_hierarchy!(IDtcLuConfigure, windows_core::IUnknown);
impl IDtcLuConfigure {
    pub unsafe fn Add(&self, puclupair: &[u8]) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Add)(windows_core::Interface::as_raw(self), core::mem::transmute(puclupair.as_ptr()), puclupair.len().try_into().unwrap()).ok() }
    }
    pub unsafe fn Delete(&self, puclupair: &[u8]) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Delete)(windows_core::Interface::as_raw(self), core::mem::transmute(puclupair.as_ptr()), puclupair.len().try_into().unwrap()).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDtcLuConfigure_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Add: unsafe extern "system" fn(*mut core::ffi::c_void, *const u8, u32) -> windows_core::HRESULT,
    pub Delete: unsafe extern "system" fn(*mut core::ffi::c_void, *const u8, u32) -> windows_core::HRESULT,
}
pub trait IDtcLuConfigure_Impl: windows_core::IUnknownImpl {
    fn Add(&self, puclupair: *const u8, cblupair: u32) -> windows_core::Result<()>;
    fn Delete(&self, puclupair: *const u8, cblupair: u32) -> windows_core::Result<()>;
}
impl IDtcLuConfigure_Vtbl {
    pub const fn new<Identity: IDtcLuConfigure_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Add<Identity: IDtcLuConfigure_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, puclupair: *const u8, cblupair: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDtcLuConfigure_Impl::Add(this, core::mem::transmute_copy(&puclupair), core::mem::transmute_copy(&cblupair)).into()
            }
        }
        unsafe extern "system" fn Delete<Identity: IDtcLuConfigure_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, puclupair: *const u8, cblupair: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDtcLuConfigure_Impl::Delete(this, core::mem::transmute_copy(&puclupair), core::mem::transmute_copy(&cblupair)).into()
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), Add: Add::<Identity, OFFSET>, Delete: Delete::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDtcLuConfigure as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IDtcLuConfigure {}
windows_core::imp::define_interface!(IDtcLuRecovery, IDtcLuRecovery_Vtbl, 0xac2b8ad2_d6f0_11d0_b386_00a0c9083365);
windows_core::imp::interface_hierarchy!(IDtcLuRecovery, windows_core::IUnknown);
#[repr(C)]
#[doc(hidden)]
pub struct IDtcLuRecovery_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
}
pub trait IDtcLuRecovery_Impl: windows_core::IUnknownImpl {}
impl IDtcLuRecovery_Vtbl {
    pub const fn new<Identity: IDtcLuRecovery_Impl, const OFFSET: isize>() -> Self {
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>() }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDtcLuRecovery as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IDtcLuRecovery {}
windows_core::imp::define_interface!(IDtcLuRecoveryFactory, IDtcLuRecoveryFactory_Vtbl, 0x4131e762_1aea_11d0_944b_00a0c905416e);
windows_core::imp::interface_hierarchy!(IDtcLuRecoveryFactory, windows_core::IUnknown);
impl IDtcLuRecoveryFactory {
    pub unsafe fn Create(&self, puclupair: &[u8]) -> windows_core::Result<IDtcLuRecovery> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Create)(windows_core::Interface::as_raw(self), core::mem::transmute(puclupair.as_ptr()), puclupair.len().try_into().unwrap(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDtcLuRecoveryFactory_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Create: unsafe extern "system" fn(*mut core::ffi::c_void, *const u8, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IDtcLuRecoveryFactory_Impl: windows_core::IUnknownImpl {
    fn Create(&self, puclupair: *const u8, cblupair: u32) -> windows_core::Result<IDtcLuRecovery>;
}
impl IDtcLuRecoveryFactory_Vtbl {
    pub const fn new<Identity: IDtcLuRecoveryFactory_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Create<Identity: IDtcLuRecoveryFactory_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, puclupair: *const u8, cblupair: u32, pprecovery: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDtcLuRecoveryFactory_Impl::Create(this, core::mem::transmute_copy(&puclupair), core::mem::transmute_copy(&cblupair)) {
                    Ok(ok__) => {
                        pprecovery.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), Create: Create::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDtcLuRecoveryFactory as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IDtcLuRecoveryFactory {}
windows_core::imp::define_interface!(IDtcLuRecoveryInitiatedByDtc, IDtcLuRecoveryInitiatedByDtc_Vtbl, 0x4131e764_1aea_11d0_944b_00a0c905416e);
windows_core::imp::interface_hierarchy!(IDtcLuRecoveryInitiatedByDtc, windows_core::IUnknown);
impl IDtcLuRecoveryInitiatedByDtc {
    pub unsafe fn GetWork(&self, pwork: *mut DTCINITIATEDRECOVERYWORK, ppv: *mut *mut core::ffi::c_void) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetWork)(windows_core::Interface::as_raw(self), pwork as _, ppv as _).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDtcLuRecoveryInitiatedByDtc_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetWork: unsafe extern "system" fn(*mut core::ffi::c_void, *mut DTCINITIATEDRECOVERYWORK, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IDtcLuRecoveryInitiatedByDtc_Impl: windows_core::IUnknownImpl {
    fn GetWork(&self, pwork: *mut DTCINITIATEDRECOVERYWORK, ppv: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
}
impl IDtcLuRecoveryInitiatedByDtc_Vtbl {
    pub const fn new<Identity: IDtcLuRecoveryInitiatedByDtc_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetWork<Identity: IDtcLuRecoveryInitiatedByDtc_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwork: *mut DTCINITIATEDRECOVERYWORK, ppv: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDtcLuRecoveryInitiatedByDtc_Impl::GetWork(this, core::mem::transmute_copy(&pwork), core::mem::transmute_copy(&ppv)).into()
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), GetWork: GetWork::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDtcLuRecoveryInitiatedByDtc as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IDtcLuRecoveryInitiatedByDtc {}
windows_core::imp::define_interface!(IDtcLuRecoveryInitiatedByDtcStatusWork, IDtcLuRecoveryInitiatedByDtcStatusWork_Vtbl, 0x4131e766_1aea_11d0_944b_00a0c905416e);
windows_core::imp::interface_hierarchy!(IDtcLuRecoveryInitiatedByDtcStatusWork, windows_core::IUnknown);
impl IDtcLuRecoveryInitiatedByDtcStatusWork {
    pub unsafe fn HandleCheckLuStatus(&self, lrecoveryseqnum: i32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).HandleCheckLuStatus)(windows_core::Interface::as_raw(self), lrecoveryseqnum).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDtcLuRecoveryInitiatedByDtcStatusWork_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub HandleCheckLuStatus: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
}
pub trait IDtcLuRecoveryInitiatedByDtcStatusWork_Impl: windows_core::IUnknownImpl {
    fn HandleCheckLuStatus(&self, lrecoveryseqnum: i32) -> windows_core::Result<()>;
}
impl IDtcLuRecoveryInitiatedByDtcStatusWork_Vtbl {
    pub const fn new<Identity: IDtcLuRecoveryInitiatedByDtcStatusWork_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn HandleCheckLuStatus<Identity: IDtcLuRecoveryInitiatedByDtcStatusWork_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lrecoveryseqnum: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDtcLuRecoveryInitiatedByDtcStatusWork_Impl::HandleCheckLuStatus(this, core::mem::transmute_copy(&lrecoveryseqnum)).into()
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), HandleCheckLuStatus: HandleCheckLuStatus::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDtcLuRecoveryInitiatedByDtcStatusWork as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IDtcLuRecoveryInitiatedByDtcStatusWork {}
windows_core::imp::define_interface!(IDtcLuRecoveryInitiatedByDtcTransWork, IDtcLuRecoveryInitiatedByDtcTransWork_Vtbl, 0x4131e765_1aea_11d0_944b_00a0c905416e);
windows_core::imp::interface_hierarchy!(IDtcLuRecoveryInitiatedByDtcTransWork, windows_core::IUnknown);
impl IDtcLuRecoveryInitiatedByDtcTransWork {
    pub unsafe fn GetLogNameSizes(&self, pcbourlogname: *mut u32, pcbremotelogname: *mut u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetLogNameSizes)(windows_core::Interface::as_raw(self), pcbourlogname as _, pcbremotelogname as _).ok() }
    }
    pub unsafe fn GetOurXln(&self, pxln: *mut DTCLUXLN, pourlogname: *mut u8, premotelogname: *mut u8, pdwprotocol: *mut u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetOurXln)(windows_core::Interface::as_raw(self), pxln as _, pourlogname as _, premotelogname as _, pdwprotocol as _).ok() }
    }
    pub unsafe fn HandleConfirmationFromOurXln(&self, confirmation: DTCLUXLNCONFIRMATION) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).HandleConfirmationFromOurXln)(windows_core::Interface::as_raw(self), confirmation).ok() }
    }
    pub unsafe fn HandleTheirXlnResponse(&self, xln: DTCLUXLN, premotelogname: *mut u8, cbremotelogname: u32, dwprotocol: u32, pconfirmation: *mut DTCLUXLNCONFIRMATION) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).HandleTheirXlnResponse)(windows_core::Interface::as_raw(self), xln, premotelogname as _, cbremotelogname, dwprotocol, pconfirmation as _).ok() }
    }
    pub unsafe fn HandleErrorFromOurXln(&self, error: DTCLUXLNERROR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).HandleErrorFromOurXln)(windows_core::Interface::as_raw(self), error).ok() }
    }
    pub unsafe fn CheckForCompareStates(&self, fcomparestates: *mut windows_core::BOOL) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).CheckForCompareStates)(windows_core::Interface::as_raw(self), fcomparestates as _).ok() }
    }
    pub unsafe fn GetOurTransIdSize(&self, pcbourtransid: *mut u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetOurTransIdSize)(windows_core::Interface::as_raw(self), pcbourtransid as _).ok() }
    }
    pub unsafe fn GetOurCompareStates(&self, pourtransid: *mut u8, pcomparestate: *mut DTCLUCOMPARESTATE) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetOurCompareStates)(windows_core::Interface::as_raw(self), pourtransid as _, pcomparestate as _).ok() }
    }
    pub unsafe fn HandleTheirCompareStatesResponse(&self, comparestate: DTCLUCOMPARESTATE, pconfirmation: *mut DTCLUCOMPARESTATESCONFIRMATION) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).HandleTheirCompareStatesResponse)(windows_core::Interface::as_raw(self), comparestate, pconfirmation as _).ok() }
    }
    pub unsafe fn HandleErrorFromOurCompareStates(&self, error: DTCLUCOMPARESTATESERROR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).HandleErrorFromOurCompareStates)(windows_core::Interface::as_raw(self), error).ok() }
    }
    pub unsafe fn ConversationLost(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).ConversationLost)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn GetRecoverySeqNum(&self, plrecoveryseqnum: *mut i32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetRecoverySeqNum)(windows_core::Interface::as_raw(self), plrecoveryseqnum as _).ok() }
    }
    pub unsafe fn ObsoleteRecoverySeqNum(&self, lnewrecoveryseqnum: i32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).ObsoleteRecoverySeqNum)(windows_core::Interface::as_raw(self), lnewrecoveryseqnum).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDtcLuRecoveryInitiatedByDtcTransWork_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetLogNameSizes: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32, *mut u32) -> windows_core::HRESULT,
    pub GetOurXln: unsafe extern "system" fn(*mut core::ffi::c_void, *mut DTCLUXLN, *mut u8, *mut u8, *mut u32) -> windows_core::HRESULT,
    pub HandleConfirmationFromOurXln: unsafe extern "system" fn(*mut core::ffi::c_void, DTCLUXLNCONFIRMATION) -> windows_core::HRESULT,
    pub HandleTheirXlnResponse: unsafe extern "system" fn(*mut core::ffi::c_void, DTCLUXLN, *mut u8, u32, u32, *mut DTCLUXLNCONFIRMATION) -> windows_core::HRESULT,
    pub HandleErrorFromOurXln: unsafe extern "system" fn(*mut core::ffi::c_void, DTCLUXLNERROR) -> windows_core::HRESULT,
    pub CheckForCompareStates: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::BOOL) -> windows_core::HRESULT,
    pub GetOurTransIdSize: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub GetOurCompareStates: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u8, *mut DTCLUCOMPARESTATE) -> windows_core::HRESULT,
    pub HandleTheirCompareStatesResponse: unsafe extern "system" fn(*mut core::ffi::c_void, DTCLUCOMPARESTATE, *mut DTCLUCOMPARESTATESCONFIRMATION) -> windows_core::HRESULT,
    pub HandleErrorFromOurCompareStates: unsafe extern "system" fn(*mut core::ffi::c_void, DTCLUCOMPARESTATESERROR) -> windows_core::HRESULT,
    pub ConversationLost: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetRecoverySeqNum: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub ObsoleteRecoverySeqNum: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
}
pub trait IDtcLuRecoveryInitiatedByDtcTransWork_Impl: windows_core::IUnknownImpl {
    fn GetLogNameSizes(&self, pcbourlogname: *mut u32, pcbremotelogname: *mut u32) -> windows_core::Result<()>;
    fn GetOurXln(&self, pxln: *mut DTCLUXLN, pourlogname: *mut u8, premotelogname: *mut u8, pdwprotocol: *mut u32) -> windows_core::Result<()>;
    fn HandleConfirmationFromOurXln(&self, confirmation: DTCLUXLNCONFIRMATION) -> windows_core::Result<()>;
    fn HandleTheirXlnResponse(&self, xln: DTCLUXLN, premotelogname: *mut u8, cbremotelogname: u32, dwprotocol: u32, pconfirmation: *mut DTCLUXLNCONFIRMATION) -> windows_core::Result<()>;
    fn HandleErrorFromOurXln(&self, error: DTCLUXLNERROR) -> windows_core::Result<()>;
    fn CheckForCompareStates(&self, fcomparestates: *mut windows_core::BOOL) -> windows_core::Result<()>;
    fn GetOurTransIdSize(&self, pcbourtransid: *mut u32) -> windows_core::Result<()>;
    fn GetOurCompareStates(&self, pourtransid: *mut u8, pcomparestate: *mut DTCLUCOMPARESTATE) -> windows_core::Result<()>;
    fn HandleTheirCompareStatesResponse(&self, comparestate: DTCLUCOMPARESTATE, pconfirmation: *mut DTCLUCOMPARESTATESCONFIRMATION) -> windows_core::Result<()>;
    fn HandleErrorFromOurCompareStates(&self, error: DTCLUCOMPARESTATESERROR) -> windows_core::Result<()>;
    fn ConversationLost(&self) -> windows_core::Result<()>;
    fn GetRecoverySeqNum(&self, plrecoveryseqnum: *mut i32) -> windows_core::Result<()>;
    fn ObsoleteRecoverySeqNum(&self, lnewrecoveryseqnum: i32) -> windows_core::Result<()>;
}
impl IDtcLuRecoveryInitiatedByDtcTransWork_Vtbl {
    pub const fn new<Identity: IDtcLuRecoveryInitiatedByDtcTransWork_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetLogNameSizes<Identity: IDtcLuRecoveryInitiatedByDtcTransWork_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcbourlogname: *mut u32, pcbremotelogname: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDtcLuRecoveryInitiatedByDtcTransWork_Impl::GetLogNameSizes(this, core::mem::transmute_copy(&pcbourlogname), core::mem::transmute_copy(&pcbremotelogname)).into()
            }
        }
        unsafe extern "system" fn GetOurXln<Identity: IDtcLuRecoveryInitiatedByDtcTransWork_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pxln: *mut DTCLUXLN, pourlogname: *mut u8, premotelogname: *mut u8, pdwprotocol: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDtcLuRecoveryInitiatedByDtcTransWork_Impl::GetOurXln(this, core::mem::transmute_copy(&pxln), core::mem::transmute_copy(&pourlogname), core::mem::transmute_copy(&premotelogname), core::mem::transmute_copy(&pdwprotocol)).into()
            }
        }
        unsafe extern "system" fn HandleConfirmationFromOurXln<Identity: IDtcLuRecoveryInitiatedByDtcTransWork_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, confirmation: DTCLUXLNCONFIRMATION) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDtcLuRecoveryInitiatedByDtcTransWork_Impl::HandleConfirmationFromOurXln(this, core::mem::transmute_copy(&confirmation)).into()
            }
        }
        unsafe extern "system" fn HandleTheirXlnResponse<Identity: IDtcLuRecoveryInitiatedByDtcTransWork_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, xln: DTCLUXLN, premotelogname: *mut u8, cbremotelogname: u32, dwprotocol: u32, pconfirmation: *mut DTCLUXLNCONFIRMATION) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDtcLuRecoveryInitiatedByDtcTransWork_Impl::HandleTheirXlnResponse(this, core::mem::transmute_copy(&xln), core::mem::transmute_copy(&premotelogname), core::mem::transmute_copy(&cbremotelogname), core::mem::transmute_copy(&dwprotocol), core::mem::transmute_copy(&pconfirmation)).into()
            }
        }
        unsafe extern "system" fn HandleErrorFromOurXln<Identity: IDtcLuRecoveryInitiatedByDtcTransWork_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, error: DTCLUXLNERROR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDtcLuRecoveryInitiatedByDtcTransWork_Impl::HandleErrorFromOurXln(this, core::mem::transmute_copy(&error)).into()
            }
        }
        unsafe extern "system" fn CheckForCompareStates<Identity: IDtcLuRecoveryInitiatedByDtcTransWork_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, fcomparestates: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDtcLuRecoveryInitiatedByDtcTransWork_Impl::CheckForCompareStates(this, core::mem::transmute_copy(&fcomparestates)).into()
            }
        }
        unsafe extern "system" fn GetOurTransIdSize<Identity: IDtcLuRecoveryInitiatedByDtcTransWork_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcbourtransid: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDtcLuRecoveryInitiatedByDtcTransWork_Impl::GetOurTransIdSize(this, core::mem::transmute_copy(&pcbourtransid)).into()
            }
        }
        unsafe extern "system" fn GetOurCompareStates<Identity: IDtcLuRecoveryInitiatedByDtcTransWork_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pourtransid: *mut u8, pcomparestate: *mut DTCLUCOMPARESTATE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDtcLuRecoveryInitiatedByDtcTransWork_Impl::GetOurCompareStates(this, core::mem::transmute_copy(&pourtransid), core::mem::transmute_copy(&pcomparestate)).into()
            }
        }
        unsafe extern "system" fn HandleTheirCompareStatesResponse<Identity: IDtcLuRecoveryInitiatedByDtcTransWork_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, comparestate: DTCLUCOMPARESTATE, pconfirmation: *mut DTCLUCOMPARESTATESCONFIRMATION) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDtcLuRecoveryInitiatedByDtcTransWork_Impl::HandleTheirCompareStatesResponse(this, core::mem::transmute_copy(&comparestate), core::mem::transmute_copy(&pconfirmation)).into()
            }
        }
        unsafe extern "system" fn HandleErrorFromOurCompareStates<Identity: IDtcLuRecoveryInitiatedByDtcTransWork_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, error: DTCLUCOMPARESTATESERROR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDtcLuRecoveryInitiatedByDtcTransWork_Impl::HandleErrorFromOurCompareStates(this, core::mem::transmute_copy(&error)).into()
            }
        }
        unsafe extern "system" fn ConversationLost<Identity: IDtcLuRecoveryInitiatedByDtcTransWork_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDtcLuRecoveryInitiatedByDtcTransWork_Impl::ConversationLost(this).into()
            }
        }
        unsafe extern "system" fn GetRecoverySeqNum<Identity: IDtcLuRecoveryInitiatedByDtcTransWork_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, plrecoveryseqnum: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDtcLuRecoveryInitiatedByDtcTransWork_Impl::GetRecoverySeqNum(this, core::mem::transmute_copy(&plrecoveryseqnum)).into()
            }
        }
        unsafe extern "system" fn ObsoleteRecoverySeqNum<Identity: IDtcLuRecoveryInitiatedByDtcTransWork_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lnewrecoveryseqnum: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDtcLuRecoveryInitiatedByDtcTransWork_Impl::ObsoleteRecoverySeqNum(this, core::mem::transmute_copy(&lnewrecoveryseqnum)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetLogNameSizes: GetLogNameSizes::<Identity, OFFSET>,
            GetOurXln: GetOurXln::<Identity, OFFSET>,
            HandleConfirmationFromOurXln: HandleConfirmationFromOurXln::<Identity, OFFSET>,
            HandleTheirXlnResponse: HandleTheirXlnResponse::<Identity, OFFSET>,
            HandleErrorFromOurXln: HandleErrorFromOurXln::<Identity, OFFSET>,
            CheckForCompareStates: CheckForCompareStates::<Identity, OFFSET>,
            GetOurTransIdSize: GetOurTransIdSize::<Identity, OFFSET>,
            GetOurCompareStates: GetOurCompareStates::<Identity, OFFSET>,
            HandleTheirCompareStatesResponse: HandleTheirCompareStatesResponse::<Identity, OFFSET>,
            HandleErrorFromOurCompareStates: HandleErrorFromOurCompareStates::<Identity, OFFSET>,
            ConversationLost: ConversationLost::<Identity, OFFSET>,
            GetRecoverySeqNum: GetRecoverySeqNum::<Identity, OFFSET>,
            ObsoleteRecoverySeqNum: ObsoleteRecoverySeqNum::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDtcLuRecoveryInitiatedByDtcTransWork as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IDtcLuRecoveryInitiatedByDtcTransWork {}
windows_core::imp::define_interface!(IDtcLuRecoveryInitiatedByLu, IDtcLuRecoveryInitiatedByLu_Vtbl, 0x4131e768_1aea_11d0_944b_00a0c905416e);
windows_core::imp::interface_hierarchy!(IDtcLuRecoveryInitiatedByLu, windows_core::IUnknown);
impl IDtcLuRecoveryInitiatedByLu {
    pub unsafe fn GetObjectToHandleWorkFromLu(&self) -> windows_core::Result<IDtcLuRecoveryInitiatedByLuWork> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetObjectToHandleWorkFromLu)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDtcLuRecoveryInitiatedByLu_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetObjectToHandleWorkFromLu: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IDtcLuRecoveryInitiatedByLu_Impl: windows_core::IUnknownImpl {
    fn GetObjectToHandleWorkFromLu(&self) -> windows_core::Result<IDtcLuRecoveryInitiatedByLuWork>;
}
impl IDtcLuRecoveryInitiatedByLu_Vtbl {
    pub const fn new<Identity: IDtcLuRecoveryInitiatedByLu_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetObjectToHandleWorkFromLu<Identity: IDtcLuRecoveryInitiatedByLu_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppwork: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDtcLuRecoveryInitiatedByLu_Impl::GetObjectToHandleWorkFromLu(this) {
                    Ok(ok__) => {
                        ppwork.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), GetObjectToHandleWorkFromLu: GetObjectToHandleWorkFromLu::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDtcLuRecoveryInitiatedByLu as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IDtcLuRecoveryInitiatedByLu {}
windows_core::imp::define_interface!(IDtcLuRecoveryInitiatedByLuWork, IDtcLuRecoveryInitiatedByLuWork_Vtbl, 0xac2b8ad1_d6f0_11d0_b386_00a0c9083365);
windows_core::imp::interface_hierarchy!(IDtcLuRecoveryInitiatedByLuWork, windows_core::IUnknown);
impl IDtcLuRecoveryInitiatedByLuWork {
    pub unsafe fn HandleTheirXln(&self, lrecoveryseqnum: i32, xln: DTCLUXLN, premotelogname: *mut u8, cbremotelogname: u32, pourlogname: *mut u8, cbourlogname: u32, dwprotocol: u32, presponse: *mut DTCLUXLNRESPONSE) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).HandleTheirXln)(windows_core::Interface::as_raw(self), lrecoveryseqnum, xln, premotelogname as _, cbremotelogname, pourlogname as _, cbourlogname, dwprotocol, presponse as _).ok() }
    }
    pub unsafe fn GetOurLogNameSize(&self, pcbourlogname: *mut u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetOurLogNameSize)(windows_core::Interface::as_raw(self), pcbourlogname as _).ok() }
    }
    pub unsafe fn GetOurXln(&self, pxln: *mut DTCLUXLN, pourlogname: *mut u8, pdwprotocol: *mut u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetOurXln)(windows_core::Interface::as_raw(self), pxln as _, pourlogname as _, pdwprotocol as _).ok() }
    }
    pub unsafe fn HandleConfirmationOfOurXln(&self, confirmation: DTCLUXLNCONFIRMATION) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).HandleConfirmationOfOurXln)(windows_core::Interface::as_raw(self), confirmation).ok() }
    }
    pub unsafe fn HandleTheirCompareStates(&self, premotetransid: *mut u8, cbremotetransid: u32, comparestate: DTCLUCOMPARESTATE, presponse: *mut DTCLUCOMPARESTATESRESPONSE, pcomparestate: *mut DTCLUCOMPARESTATE) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).HandleTheirCompareStates)(windows_core::Interface::as_raw(self), premotetransid as _, cbremotetransid, comparestate, presponse as _, pcomparestate as _).ok() }
    }
    pub unsafe fn HandleConfirmationOfOurCompareStates(&self, confirmation: DTCLUCOMPARESTATESCONFIRMATION) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).HandleConfirmationOfOurCompareStates)(windows_core::Interface::as_raw(self), confirmation).ok() }
    }
    pub unsafe fn HandleErrorFromOurCompareStates(&self, error: DTCLUCOMPARESTATESERROR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).HandleErrorFromOurCompareStates)(windows_core::Interface::as_raw(self), error).ok() }
    }
    pub unsafe fn ConversationLost(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).ConversationLost)(windows_core::Interface::as_raw(self)).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
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
pub trait IDtcLuRecoveryInitiatedByLuWork_Impl: windows_core::IUnknownImpl {
    fn HandleTheirXln(&self, lrecoveryseqnum: i32, xln: DTCLUXLN, premotelogname: *mut u8, cbremotelogname: u32, pourlogname: *mut u8, cbourlogname: u32, dwprotocol: u32, presponse: *mut DTCLUXLNRESPONSE) -> windows_core::Result<()>;
    fn GetOurLogNameSize(&self, pcbourlogname: *mut u32) -> windows_core::Result<()>;
    fn GetOurXln(&self, pxln: *mut DTCLUXLN, pourlogname: *mut u8, pdwprotocol: *mut u32) -> windows_core::Result<()>;
    fn HandleConfirmationOfOurXln(&self, confirmation: DTCLUXLNCONFIRMATION) -> windows_core::Result<()>;
    fn HandleTheirCompareStates(&self, premotetransid: *mut u8, cbremotetransid: u32, comparestate: DTCLUCOMPARESTATE, presponse: *mut DTCLUCOMPARESTATESRESPONSE, pcomparestate: *mut DTCLUCOMPARESTATE) -> windows_core::Result<()>;
    fn HandleConfirmationOfOurCompareStates(&self, confirmation: DTCLUCOMPARESTATESCONFIRMATION) -> windows_core::Result<()>;
    fn HandleErrorFromOurCompareStates(&self, error: DTCLUCOMPARESTATESERROR) -> windows_core::Result<()>;
    fn ConversationLost(&self) -> windows_core::Result<()>;
}
impl IDtcLuRecoveryInitiatedByLuWork_Vtbl {
    pub const fn new<Identity: IDtcLuRecoveryInitiatedByLuWork_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn HandleTheirXln<Identity: IDtcLuRecoveryInitiatedByLuWork_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lrecoveryseqnum: i32, xln: DTCLUXLN, premotelogname: *mut u8, cbremotelogname: u32, pourlogname: *mut u8, cbourlogname: u32, dwprotocol: u32, presponse: *mut DTCLUXLNRESPONSE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDtcLuRecoveryInitiatedByLuWork_Impl::HandleTheirXln(this, core::mem::transmute_copy(&lrecoveryseqnum), core::mem::transmute_copy(&xln), core::mem::transmute_copy(&premotelogname), core::mem::transmute_copy(&cbremotelogname), core::mem::transmute_copy(&pourlogname), core::mem::transmute_copy(&cbourlogname), core::mem::transmute_copy(&dwprotocol), core::mem::transmute_copy(&presponse)).into()
            }
        }
        unsafe extern "system" fn GetOurLogNameSize<Identity: IDtcLuRecoveryInitiatedByLuWork_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcbourlogname: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDtcLuRecoveryInitiatedByLuWork_Impl::GetOurLogNameSize(this, core::mem::transmute_copy(&pcbourlogname)).into()
            }
        }
        unsafe extern "system" fn GetOurXln<Identity: IDtcLuRecoveryInitiatedByLuWork_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pxln: *mut DTCLUXLN, pourlogname: *mut u8, pdwprotocol: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDtcLuRecoveryInitiatedByLuWork_Impl::GetOurXln(this, core::mem::transmute_copy(&pxln), core::mem::transmute_copy(&pourlogname), core::mem::transmute_copy(&pdwprotocol)).into()
            }
        }
        unsafe extern "system" fn HandleConfirmationOfOurXln<Identity: IDtcLuRecoveryInitiatedByLuWork_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, confirmation: DTCLUXLNCONFIRMATION) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDtcLuRecoveryInitiatedByLuWork_Impl::HandleConfirmationOfOurXln(this, core::mem::transmute_copy(&confirmation)).into()
            }
        }
        unsafe extern "system" fn HandleTheirCompareStates<Identity: IDtcLuRecoveryInitiatedByLuWork_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, premotetransid: *mut u8, cbremotetransid: u32, comparestate: DTCLUCOMPARESTATE, presponse: *mut DTCLUCOMPARESTATESRESPONSE, pcomparestate: *mut DTCLUCOMPARESTATE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDtcLuRecoveryInitiatedByLuWork_Impl::HandleTheirCompareStates(this, core::mem::transmute_copy(&premotetransid), core::mem::transmute_copy(&cbremotetransid), core::mem::transmute_copy(&comparestate), core::mem::transmute_copy(&presponse), core::mem::transmute_copy(&pcomparestate)).into()
            }
        }
        unsafe extern "system" fn HandleConfirmationOfOurCompareStates<Identity: IDtcLuRecoveryInitiatedByLuWork_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, confirmation: DTCLUCOMPARESTATESCONFIRMATION) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDtcLuRecoveryInitiatedByLuWork_Impl::HandleConfirmationOfOurCompareStates(this, core::mem::transmute_copy(&confirmation)).into()
            }
        }
        unsafe extern "system" fn HandleErrorFromOurCompareStates<Identity: IDtcLuRecoveryInitiatedByLuWork_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, error: DTCLUCOMPARESTATESERROR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDtcLuRecoveryInitiatedByLuWork_Impl::HandleErrorFromOurCompareStates(this, core::mem::transmute_copy(&error)).into()
            }
        }
        unsafe extern "system" fn ConversationLost<Identity: IDtcLuRecoveryInitiatedByLuWork_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDtcLuRecoveryInitiatedByLuWork_Impl::ConversationLost(this).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            HandleTheirXln: HandleTheirXln::<Identity, OFFSET>,
            GetOurLogNameSize: GetOurLogNameSize::<Identity, OFFSET>,
            GetOurXln: GetOurXln::<Identity, OFFSET>,
            HandleConfirmationOfOurXln: HandleConfirmationOfOurXln::<Identity, OFFSET>,
            HandleTheirCompareStates: HandleTheirCompareStates::<Identity, OFFSET>,
            HandleConfirmationOfOurCompareStates: HandleConfirmationOfOurCompareStates::<Identity, OFFSET>,
            HandleErrorFromOurCompareStates: HandleErrorFromOurCompareStates::<Identity, OFFSET>,
            ConversationLost: ConversationLost::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDtcLuRecoveryInitiatedByLuWork as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IDtcLuRecoveryInitiatedByLuWork {}
windows_core::imp::define_interface!(IDtcLuRmEnlistment, IDtcLuRmEnlistment_Vtbl, 0x4131e769_1aea_11d0_944b_00a0c905416e);
windows_core::imp::interface_hierarchy!(IDtcLuRmEnlistment, windows_core::IUnknown);
impl IDtcLuRmEnlistment {
    pub unsafe fn Unplug(&self, fconversationlost: bool) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Unplug)(windows_core::Interface::as_raw(self), fconversationlost.into()).ok() }
    }
    pub unsafe fn BackedOut(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).BackedOut)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn BackOut(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).BackOut)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn Committed(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Committed)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn Forget(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Forget)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn RequestCommit(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).RequestCommit)(windows_core::Interface::as_raw(self)).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDtcLuRmEnlistment_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Unplug: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::BOOL) -> windows_core::HRESULT,
    pub BackedOut: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub BackOut: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Committed: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Forget: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub RequestCommit: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IDtcLuRmEnlistment_Impl: windows_core::IUnknownImpl {
    fn Unplug(&self, fconversationlost: windows_core::BOOL) -> windows_core::Result<()>;
    fn BackedOut(&self) -> windows_core::Result<()>;
    fn BackOut(&self) -> windows_core::Result<()>;
    fn Committed(&self) -> windows_core::Result<()>;
    fn Forget(&self) -> windows_core::Result<()>;
    fn RequestCommit(&self) -> windows_core::Result<()>;
}
impl IDtcLuRmEnlistment_Vtbl {
    pub const fn new<Identity: IDtcLuRmEnlistment_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Unplug<Identity: IDtcLuRmEnlistment_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, fconversationlost: windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDtcLuRmEnlistment_Impl::Unplug(this, core::mem::transmute_copy(&fconversationlost)).into()
            }
        }
        unsafe extern "system" fn BackedOut<Identity: IDtcLuRmEnlistment_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDtcLuRmEnlistment_Impl::BackedOut(this).into()
            }
        }
        unsafe extern "system" fn BackOut<Identity: IDtcLuRmEnlistment_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDtcLuRmEnlistment_Impl::BackOut(this).into()
            }
        }
        unsafe extern "system" fn Committed<Identity: IDtcLuRmEnlistment_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDtcLuRmEnlistment_Impl::Committed(this).into()
            }
        }
        unsafe extern "system" fn Forget<Identity: IDtcLuRmEnlistment_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDtcLuRmEnlistment_Impl::Forget(this).into()
            }
        }
        unsafe extern "system" fn RequestCommit<Identity: IDtcLuRmEnlistment_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDtcLuRmEnlistment_Impl::RequestCommit(this).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Unplug: Unplug::<Identity, OFFSET>,
            BackedOut: BackedOut::<Identity, OFFSET>,
            BackOut: BackOut::<Identity, OFFSET>,
            Committed: Committed::<Identity, OFFSET>,
            Forget: Forget::<Identity, OFFSET>,
            RequestCommit: RequestCommit::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDtcLuRmEnlistment as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IDtcLuRmEnlistment {}
windows_core::imp::define_interface!(IDtcLuRmEnlistmentFactory, IDtcLuRmEnlistmentFactory_Vtbl, 0x4131e771_1aea_11d0_944b_00a0c905416e);
windows_core::imp::interface_hierarchy!(IDtcLuRmEnlistmentFactory, windows_core::IUnknown);
impl IDtcLuRmEnlistmentFactory {
    pub unsafe fn Create<P2, P5>(&self, puclupair: *mut u8, cblupair: u32, pitransaction: P2, ptransid: *mut u8, cbtransid: u32, prmenlistmentsink: P5, pprmenlistment: *mut Option<IDtcLuRmEnlistment>) -> windows_core::Result<()>
    where
        P2: windows_core::Param<ITransaction>,
        P5: windows_core::Param<IDtcLuRmEnlistmentSink>,
    {
        unsafe { (windows_core::Interface::vtable(self).Create)(windows_core::Interface::as_raw(self), puclupair as _, cblupair, pitransaction.param().abi(), ptransid as _, cbtransid, prmenlistmentsink.param().abi(), core::mem::transmute(pprmenlistment)).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDtcLuRmEnlistmentFactory_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Create: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u8, u32, *mut core::ffi::c_void, *mut u8, u32, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IDtcLuRmEnlistmentFactory_Impl: windows_core::IUnknownImpl {
    fn Create(&self, puclupair: *mut u8, cblupair: u32, pitransaction: windows_core::Ref<ITransaction>, ptransid: *mut u8, cbtransid: u32, prmenlistmentsink: windows_core::Ref<IDtcLuRmEnlistmentSink>, pprmenlistment: windows_core::OutRef<IDtcLuRmEnlistment>) -> windows_core::Result<()>;
}
impl IDtcLuRmEnlistmentFactory_Vtbl {
    pub const fn new<Identity: IDtcLuRmEnlistmentFactory_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Create<Identity: IDtcLuRmEnlistmentFactory_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, puclupair: *mut u8, cblupair: u32, pitransaction: *mut core::ffi::c_void, ptransid: *mut u8, cbtransid: u32, prmenlistmentsink: *mut core::ffi::c_void, pprmenlistment: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDtcLuRmEnlistmentFactory_Impl::Create(this, core::mem::transmute_copy(&puclupair), core::mem::transmute_copy(&cblupair), core::mem::transmute_copy(&pitransaction), core::mem::transmute_copy(&ptransid), core::mem::transmute_copy(&cbtransid), core::mem::transmute_copy(&prmenlistmentsink), core::mem::transmute_copy(&pprmenlistment)).into()
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), Create: Create::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDtcLuRmEnlistmentFactory as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IDtcLuRmEnlistmentFactory {}
windows_core::imp::define_interface!(IDtcLuRmEnlistmentSink, IDtcLuRmEnlistmentSink_Vtbl, 0x4131e770_1aea_11d0_944b_00a0c905416e);
windows_core::imp::interface_hierarchy!(IDtcLuRmEnlistmentSink, windows_core::IUnknown);
impl IDtcLuRmEnlistmentSink {
    pub unsafe fn AckUnplug(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).AckUnplug)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn TmDown(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).TmDown)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn SessionLost(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SessionLost)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn BackedOut(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).BackedOut)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn BackOut(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).BackOut)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn Committed(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Committed)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn Forget(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Forget)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn Prepare(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Prepare)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn RequestCommit(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).RequestCommit)(windows_core::Interface::as_raw(self)).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
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
pub trait IDtcLuRmEnlistmentSink_Impl: windows_core::IUnknownImpl {
    fn AckUnplug(&self) -> windows_core::Result<()>;
    fn TmDown(&self) -> windows_core::Result<()>;
    fn SessionLost(&self) -> windows_core::Result<()>;
    fn BackedOut(&self) -> windows_core::Result<()>;
    fn BackOut(&self) -> windows_core::Result<()>;
    fn Committed(&self) -> windows_core::Result<()>;
    fn Forget(&self) -> windows_core::Result<()>;
    fn Prepare(&self) -> windows_core::Result<()>;
    fn RequestCommit(&self) -> windows_core::Result<()>;
}
impl IDtcLuRmEnlistmentSink_Vtbl {
    pub const fn new<Identity: IDtcLuRmEnlistmentSink_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn AckUnplug<Identity: IDtcLuRmEnlistmentSink_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDtcLuRmEnlistmentSink_Impl::AckUnplug(this).into()
            }
        }
        unsafe extern "system" fn TmDown<Identity: IDtcLuRmEnlistmentSink_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDtcLuRmEnlistmentSink_Impl::TmDown(this).into()
            }
        }
        unsafe extern "system" fn SessionLost<Identity: IDtcLuRmEnlistmentSink_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDtcLuRmEnlistmentSink_Impl::SessionLost(this).into()
            }
        }
        unsafe extern "system" fn BackedOut<Identity: IDtcLuRmEnlistmentSink_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDtcLuRmEnlistmentSink_Impl::BackedOut(this).into()
            }
        }
        unsafe extern "system" fn BackOut<Identity: IDtcLuRmEnlistmentSink_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDtcLuRmEnlistmentSink_Impl::BackOut(this).into()
            }
        }
        unsafe extern "system" fn Committed<Identity: IDtcLuRmEnlistmentSink_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDtcLuRmEnlistmentSink_Impl::Committed(this).into()
            }
        }
        unsafe extern "system" fn Forget<Identity: IDtcLuRmEnlistmentSink_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDtcLuRmEnlistmentSink_Impl::Forget(this).into()
            }
        }
        unsafe extern "system" fn Prepare<Identity: IDtcLuRmEnlistmentSink_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDtcLuRmEnlistmentSink_Impl::Prepare(this).into()
            }
        }
        unsafe extern "system" fn RequestCommit<Identity: IDtcLuRmEnlistmentSink_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDtcLuRmEnlistmentSink_Impl::RequestCommit(this).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            AckUnplug: AckUnplug::<Identity, OFFSET>,
            TmDown: TmDown::<Identity, OFFSET>,
            SessionLost: SessionLost::<Identity, OFFSET>,
            BackedOut: BackedOut::<Identity, OFFSET>,
            BackOut: BackOut::<Identity, OFFSET>,
            Committed: Committed::<Identity, OFFSET>,
            Forget: Forget::<Identity, OFFSET>,
            Prepare: Prepare::<Identity, OFFSET>,
            RequestCommit: RequestCommit::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDtcLuRmEnlistmentSink as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IDtcLuRmEnlistmentSink {}
windows_core::imp::define_interface!(IDtcLuSubordinateDtc, IDtcLuSubordinateDtc_Vtbl, 0x4131e773_1aea_11d0_944b_00a0c905416e);
windows_core::imp::interface_hierarchy!(IDtcLuSubordinateDtc, windows_core::IUnknown);
impl IDtcLuSubordinateDtc {
    pub unsafe fn Unplug(&self, fconversationlost: bool) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Unplug)(windows_core::Interface::as_raw(self), fconversationlost.into()).ok() }
    }
    pub unsafe fn BackedOut(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).BackedOut)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn BackOut(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).BackOut)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn Committed(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Committed)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn Forget(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Forget)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn Prepare(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Prepare)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn RequestCommit(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).RequestCommit)(windows_core::Interface::as_raw(self)).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDtcLuSubordinateDtc_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Unplug: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::BOOL) -> windows_core::HRESULT,
    pub BackedOut: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub BackOut: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Committed: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Forget: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Prepare: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub RequestCommit: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IDtcLuSubordinateDtc_Impl: windows_core::IUnknownImpl {
    fn Unplug(&self, fconversationlost: windows_core::BOOL) -> windows_core::Result<()>;
    fn BackedOut(&self) -> windows_core::Result<()>;
    fn BackOut(&self) -> windows_core::Result<()>;
    fn Committed(&self) -> windows_core::Result<()>;
    fn Forget(&self) -> windows_core::Result<()>;
    fn Prepare(&self) -> windows_core::Result<()>;
    fn RequestCommit(&self) -> windows_core::Result<()>;
}
impl IDtcLuSubordinateDtc_Vtbl {
    pub const fn new<Identity: IDtcLuSubordinateDtc_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Unplug<Identity: IDtcLuSubordinateDtc_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, fconversationlost: windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDtcLuSubordinateDtc_Impl::Unplug(this, core::mem::transmute_copy(&fconversationlost)).into()
            }
        }
        unsafe extern "system" fn BackedOut<Identity: IDtcLuSubordinateDtc_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDtcLuSubordinateDtc_Impl::BackedOut(this).into()
            }
        }
        unsafe extern "system" fn BackOut<Identity: IDtcLuSubordinateDtc_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDtcLuSubordinateDtc_Impl::BackOut(this).into()
            }
        }
        unsafe extern "system" fn Committed<Identity: IDtcLuSubordinateDtc_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDtcLuSubordinateDtc_Impl::Committed(this).into()
            }
        }
        unsafe extern "system" fn Forget<Identity: IDtcLuSubordinateDtc_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDtcLuSubordinateDtc_Impl::Forget(this).into()
            }
        }
        unsafe extern "system" fn Prepare<Identity: IDtcLuSubordinateDtc_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDtcLuSubordinateDtc_Impl::Prepare(this).into()
            }
        }
        unsafe extern "system" fn RequestCommit<Identity: IDtcLuSubordinateDtc_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDtcLuSubordinateDtc_Impl::RequestCommit(this).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Unplug: Unplug::<Identity, OFFSET>,
            BackedOut: BackedOut::<Identity, OFFSET>,
            BackOut: BackOut::<Identity, OFFSET>,
            Committed: Committed::<Identity, OFFSET>,
            Forget: Forget::<Identity, OFFSET>,
            Prepare: Prepare::<Identity, OFFSET>,
            RequestCommit: RequestCommit::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDtcLuSubordinateDtc as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IDtcLuSubordinateDtc {}
windows_core::imp::define_interface!(IDtcLuSubordinateDtcFactory, IDtcLuSubordinateDtcFactory_Vtbl, 0x4131e775_1aea_11d0_944b_00a0c905416e);
windows_core::imp::interface_hierarchy!(IDtcLuSubordinateDtcFactory, windows_core::IUnknown);
impl IDtcLuSubordinateDtcFactory {
    pub unsafe fn Create<P2, P5, P9>(&self, puclupair: *mut u8, cblupair: u32, punktransactionouter: P2, isolevel: i32, isoflags: u32, poptions: P5, pptransaction: *mut Option<ITransaction>, ptransid: *mut u8, cbtransid: u32, psubordinatedtcsink: P9, ppsubordinatedtc: *mut Option<IDtcLuSubordinateDtc>) -> windows_core::Result<()>
    where
        P2: windows_core::Param<windows_core::IUnknown>,
        P5: windows_core::Param<ITransactionOptions>,
        P9: windows_core::Param<IDtcLuSubordinateDtcSink>,
    {
        unsafe { (windows_core::Interface::vtable(self).Create)(windows_core::Interface::as_raw(self), puclupair as _, cblupair, punktransactionouter.param().abi(), isolevel, isoflags, poptions.param().abi(), core::mem::transmute(pptransaction), ptransid as _, cbtransid, psubordinatedtcsink.param().abi(), core::mem::transmute(ppsubordinatedtc)).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDtcLuSubordinateDtcFactory_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Create: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u8, u32, *mut core::ffi::c_void, i32, u32, *mut core::ffi::c_void, *mut *mut core::ffi::c_void, *mut u8, u32, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IDtcLuSubordinateDtcFactory_Impl: windows_core::IUnknownImpl {
    fn Create(&self, puclupair: *mut u8, cblupair: u32, punktransactionouter: windows_core::Ref<windows_core::IUnknown>, isolevel: i32, isoflags: u32, poptions: windows_core::Ref<ITransactionOptions>, pptransaction: windows_core::OutRef<ITransaction>, ptransid: *mut u8, cbtransid: u32, psubordinatedtcsink: windows_core::Ref<IDtcLuSubordinateDtcSink>, ppsubordinatedtc: windows_core::OutRef<IDtcLuSubordinateDtc>) -> windows_core::Result<()>;
}
impl IDtcLuSubordinateDtcFactory_Vtbl {
    pub const fn new<Identity: IDtcLuSubordinateDtcFactory_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Create<Identity: IDtcLuSubordinateDtcFactory_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, puclupair: *mut u8, cblupair: u32, punktransactionouter: *mut core::ffi::c_void, isolevel: i32, isoflags: u32, poptions: *mut core::ffi::c_void, pptransaction: *mut *mut core::ffi::c_void, ptransid: *mut u8, cbtransid: u32, psubordinatedtcsink: *mut core::ffi::c_void, ppsubordinatedtc: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDtcLuSubordinateDtcFactory_Impl::Create(this, core::mem::transmute_copy(&puclupair), core::mem::transmute_copy(&cblupair), core::mem::transmute_copy(&punktransactionouter), core::mem::transmute_copy(&isolevel), core::mem::transmute_copy(&isoflags), core::mem::transmute_copy(&poptions), core::mem::transmute_copy(&pptransaction), core::mem::transmute_copy(&ptransid), core::mem::transmute_copy(&cbtransid), core::mem::transmute_copy(&psubordinatedtcsink), core::mem::transmute_copy(&ppsubordinatedtc)).into()
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), Create: Create::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDtcLuSubordinateDtcFactory as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IDtcLuSubordinateDtcFactory {}
windows_core::imp::define_interface!(IDtcLuSubordinateDtcSink, IDtcLuSubordinateDtcSink_Vtbl, 0x4131e774_1aea_11d0_944b_00a0c905416e);
windows_core::imp::interface_hierarchy!(IDtcLuSubordinateDtcSink, windows_core::IUnknown);
impl IDtcLuSubordinateDtcSink {
    pub unsafe fn AckUnplug(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).AckUnplug)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn TmDown(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).TmDown)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn SessionLost(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SessionLost)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn BackedOut(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).BackedOut)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn BackOut(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).BackOut)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn Committed(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Committed)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn Forget(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Forget)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn RequestCommit(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).RequestCommit)(windows_core::Interface::as_raw(self)).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
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
pub trait IDtcLuSubordinateDtcSink_Impl: windows_core::IUnknownImpl {
    fn AckUnplug(&self) -> windows_core::Result<()>;
    fn TmDown(&self) -> windows_core::Result<()>;
    fn SessionLost(&self) -> windows_core::Result<()>;
    fn BackedOut(&self) -> windows_core::Result<()>;
    fn BackOut(&self) -> windows_core::Result<()>;
    fn Committed(&self) -> windows_core::Result<()>;
    fn Forget(&self) -> windows_core::Result<()>;
    fn RequestCommit(&self) -> windows_core::Result<()>;
}
impl IDtcLuSubordinateDtcSink_Vtbl {
    pub const fn new<Identity: IDtcLuSubordinateDtcSink_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn AckUnplug<Identity: IDtcLuSubordinateDtcSink_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDtcLuSubordinateDtcSink_Impl::AckUnplug(this).into()
            }
        }
        unsafe extern "system" fn TmDown<Identity: IDtcLuSubordinateDtcSink_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDtcLuSubordinateDtcSink_Impl::TmDown(this).into()
            }
        }
        unsafe extern "system" fn SessionLost<Identity: IDtcLuSubordinateDtcSink_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDtcLuSubordinateDtcSink_Impl::SessionLost(this).into()
            }
        }
        unsafe extern "system" fn BackedOut<Identity: IDtcLuSubordinateDtcSink_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDtcLuSubordinateDtcSink_Impl::BackedOut(this).into()
            }
        }
        unsafe extern "system" fn BackOut<Identity: IDtcLuSubordinateDtcSink_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDtcLuSubordinateDtcSink_Impl::BackOut(this).into()
            }
        }
        unsafe extern "system" fn Committed<Identity: IDtcLuSubordinateDtcSink_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDtcLuSubordinateDtcSink_Impl::Committed(this).into()
            }
        }
        unsafe extern "system" fn Forget<Identity: IDtcLuSubordinateDtcSink_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDtcLuSubordinateDtcSink_Impl::Forget(this).into()
            }
        }
        unsafe extern "system" fn RequestCommit<Identity: IDtcLuSubordinateDtcSink_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDtcLuSubordinateDtcSink_Impl::RequestCommit(this).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            AckUnplug: AckUnplug::<Identity, OFFSET>,
            TmDown: TmDown::<Identity, OFFSET>,
            SessionLost: SessionLost::<Identity, OFFSET>,
            BackedOut: BackedOut::<Identity, OFFSET>,
            BackOut: BackOut::<Identity, OFFSET>,
            Committed: Committed::<Identity, OFFSET>,
            Forget: Forget::<Identity, OFFSET>,
            RequestCommit: RequestCommit::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDtcLuSubordinateDtcSink as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IDtcLuSubordinateDtcSink {}
windows_core::imp::define_interface!(IDtcNetworkAccessConfig, IDtcNetworkAccessConfig_Vtbl, 0x9797c15d_a428_4291_87b6_0995031a678d);
windows_core::imp::interface_hierarchy!(IDtcNetworkAccessConfig, windows_core::IUnknown);
impl IDtcNetworkAccessConfig {
    pub unsafe fn GetAnyNetworkAccess(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetAnyNetworkAccess)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetAnyNetworkAccess(&self, banynetworkaccess: bool) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetAnyNetworkAccess)(windows_core::Interface::as_raw(self), banynetworkaccess.into()).ok() }
    }
    pub unsafe fn GetNetworkAdministrationAccess(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetNetworkAdministrationAccess)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetNetworkAdministrationAccess(&self, bnetworkadministrationaccess: bool) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetNetworkAdministrationAccess)(windows_core::Interface::as_raw(self), bnetworkadministrationaccess.into()).ok() }
    }
    pub unsafe fn GetNetworkTransactionAccess(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetNetworkTransactionAccess)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetNetworkTransactionAccess(&self, bnetworktransactionaccess: bool) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetNetworkTransactionAccess)(windows_core::Interface::as_raw(self), bnetworktransactionaccess.into()).ok() }
    }
    pub unsafe fn GetNetworkClientAccess(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetNetworkClientAccess)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetNetworkClientAccess(&self, bnetworkclientaccess: bool) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetNetworkClientAccess)(windows_core::Interface::as_raw(self), bnetworkclientaccess.into()).ok() }
    }
    pub unsafe fn GetNetworkTIPAccess(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetNetworkTIPAccess)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetNetworkTIPAccess(&self, bnetworktipaccess: bool) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetNetworkTIPAccess)(windows_core::Interface::as_raw(self), bnetworktipaccess.into()).ok() }
    }
    pub unsafe fn GetXAAccess(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetXAAccess)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetXAAccess(&self, bxaaccess: bool) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetXAAccess)(windows_core::Interface::as_raw(self), bxaaccess.into()).ok() }
    }
    pub unsafe fn RestartDtcService(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).RestartDtcService)(windows_core::Interface::as_raw(self)).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDtcNetworkAccessConfig_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetAnyNetworkAccess: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::BOOL) -> windows_core::HRESULT,
    pub SetAnyNetworkAccess: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::BOOL) -> windows_core::HRESULT,
    pub GetNetworkAdministrationAccess: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::BOOL) -> windows_core::HRESULT,
    pub SetNetworkAdministrationAccess: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::BOOL) -> windows_core::HRESULT,
    pub GetNetworkTransactionAccess: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::BOOL) -> windows_core::HRESULT,
    pub SetNetworkTransactionAccess: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::BOOL) -> windows_core::HRESULT,
    pub GetNetworkClientAccess: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::BOOL) -> windows_core::HRESULT,
    pub SetNetworkClientAccess: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::BOOL) -> windows_core::HRESULT,
    pub GetNetworkTIPAccess: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::BOOL) -> windows_core::HRESULT,
    pub SetNetworkTIPAccess: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::BOOL) -> windows_core::HRESULT,
    pub GetXAAccess: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::BOOL) -> windows_core::HRESULT,
    pub SetXAAccess: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::BOOL) -> windows_core::HRESULT,
    pub RestartDtcService: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IDtcNetworkAccessConfig_Impl: windows_core::IUnknownImpl {
    fn GetAnyNetworkAccess(&self) -> windows_core::Result<windows_core::BOOL>;
    fn SetAnyNetworkAccess(&self, banynetworkaccess: windows_core::BOOL) -> windows_core::Result<()>;
    fn GetNetworkAdministrationAccess(&self) -> windows_core::Result<windows_core::BOOL>;
    fn SetNetworkAdministrationAccess(&self, bnetworkadministrationaccess: windows_core::BOOL) -> windows_core::Result<()>;
    fn GetNetworkTransactionAccess(&self) -> windows_core::Result<windows_core::BOOL>;
    fn SetNetworkTransactionAccess(&self, bnetworktransactionaccess: windows_core::BOOL) -> windows_core::Result<()>;
    fn GetNetworkClientAccess(&self) -> windows_core::Result<windows_core::BOOL>;
    fn SetNetworkClientAccess(&self, bnetworkclientaccess: windows_core::BOOL) -> windows_core::Result<()>;
    fn GetNetworkTIPAccess(&self) -> windows_core::Result<windows_core::BOOL>;
    fn SetNetworkTIPAccess(&self, bnetworktipaccess: windows_core::BOOL) -> windows_core::Result<()>;
    fn GetXAAccess(&self) -> windows_core::Result<windows_core::BOOL>;
    fn SetXAAccess(&self, bxaaccess: windows_core::BOOL) -> windows_core::Result<()>;
    fn RestartDtcService(&self) -> windows_core::Result<()>;
}
impl IDtcNetworkAccessConfig_Vtbl {
    pub const fn new<Identity: IDtcNetworkAccessConfig_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetAnyNetworkAccess<Identity: IDtcNetworkAccessConfig_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbanynetworkaccess: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDtcNetworkAccessConfig_Impl::GetAnyNetworkAccess(this) {
                    Ok(ok__) => {
                        pbanynetworkaccess.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetAnyNetworkAccess<Identity: IDtcNetworkAccessConfig_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, banynetworkaccess: windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDtcNetworkAccessConfig_Impl::SetAnyNetworkAccess(this, core::mem::transmute_copy(&banynetworkaccess)).into()
            }
        }
        unsafe extern "system" fn GetNetworkAdministrationAccess<Identity: IDtcNetworkAccessConfig_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbnetworkadministrationaccess: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDtcNetworkAccessConfig_Impl::GetNetworkAdministrationAccess(this) {
                    Ok(ok__) => {
                        pbnetworkadministrationaccess.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetNetworkAdministrationAccess<Identity: IDtcNetworkAccessConfig_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bnetworkadministrationaccess: windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDtcNetworkAccessConfig_Impl::SetNetworkAdministrationAccess(this, core::mem::transmute_copy(&bnetworkadministrationaccess)).into()
            }
        }
        unsafe extern "system" fn GetNetworkTransactionAccess<Identity: IDtcNetworkAccessConfig_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbnetworktransactionaccess: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDtcNetworkAccessConfig_Impl::GetNetworkTransactionAccess(this) {
                    Ok(ok__) => {
                        pbnetworktransactionaccess.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetNetworkTransactionAccess<Identity: IDtcNetworkAccessConfig_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bnetworktransactionaccess: windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDtcNetworkAccessConfig_Impl::SetNetworkTransactionAccess(this, core::mem::transmute_copy(&bnetworktransactionaccess)).into()
            }
        }
        unsafe extern "system" fn GetNetworkClientAccess<Identity: IDtcNetworkAccessConfig_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbnetworkclientaccess: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDtcNetworkAccessConfig_Impl::GetNetworkClientAccess(this) {
                    Ok(ok__) => {
                        pbnetworkclientaccess.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetNetworkClientAccess<Identity: IDtcNetworkAccessConfig_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bnetworkclientaccess: windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDtcNetworkAccessConfig_Impl::SetNetworkClientAccess(this, core::mem::transmute_copy(&bnetworkclientaccess)).into()
            }
        }
        unsafe extern "system" fn GetNetworkTIPAccess<Identity: IDtcNetworkAccessConfig_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbnetworktipaccess: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDtcNetworkAccessConfig_Impl::GetNetworkTIPAccess(this) {
                    Ok(ok__) => {
                        pbnetworktipaccess.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetNetworkTIPAccess<Identity: IDtcNetworkAccessConfig_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bnetworktipaccess: windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDtcNetworkAccessConfig_Impl::SetNetworkTIPAccess(this, core::mem::transmute_copy(&bnetworktipaccess)).into()
            }
        }
        unsafe extern "system" fn GetXAAccess<Identity: IDtcNetworkAccessConfig_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbxaaccess: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDtcNetworkAccessConfig_Impl::GetXAAccess(this) {
                    Ok(ok__) => {
                        pbxaaccess.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetXAAccess<Identity: IDtcNetworkAccessConfig_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bxaaccess: windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDtcNetworkAccessConfig_Impl::SetXAAccess(this, core::mem::transmute_copy(&bxaaccess)).into()
            }
        }
        unsafe extern "system" fn RestartDtcService<Identity: IDtcNetworkAccessConfig_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDtcNetworkAccessConfig_Impl::RestartDtcService(this).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetAnyNetworkAccess: GetAnyNetworkAccess::<Identity, OFFSET>,
            SetAnyNetworkAccess: SetAnyNetworkAccess::<Identity, OFFSET>,
            GetNetworkAdministrationAccess: GetNetworkAdministrationAccess::<Identity, OFFSET>,
            SetNetworkAdministrationAccess: SetNetworkAdministrationAccess::<Identity, OFFSET>,
            GetNetworkTransactionAccess: GetNetworkTransactionAccess::<Identity, OFFSET>,
            SetNetworkTransactionAccess: SetNetworkTransactionAccess::<Identity, OFFSET>,
            GetNetworkClientAccess: GetNetworkClientAccess::<Identity, OFFSET>,
            SetNetworkClientAccess: SetNetworkClientAccess::<Identity, OFFSET>,
            GetNetworkTIPAccess: GetNetworkTIPAccess::<Identity, OFFSET>,
            SetNetworkTIPAccess: SetNetworkTIPAccess::<Identity, OFFSET>,
            GetXAAccess: GetXAAccess::<Identity, OFFSET>,
            SetXAAccess: SetXAAccess::<Identity, OFFSET>,
            RestartDtcService: RestartDtcService::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDtcNetworkAccessConfig as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IDtcNetworkAccessConfig {}
windows_core::imp::define_interface!(IDtcNetworkAccessConfig2, IDtcNetworkAccessConfig2_Vtbl, 0xa7aa013b_eb7d_4f42_b41c_b2dec09ae034);
impl core::ops::Deref for IDtcNetworkAccessConfig2 {
    type Target = IDtcNetworkAccessConfig;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IDtcNetworkAccessConfig2, windows_core::IUnknown, IDtcNetworkAccessConfig);
impl IDtcNetworkAccessConfig2 {
    pub unsafe fn GetNetworkInboundAccess(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetNetworkInboundAccess)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetNetworkOutboundAccess(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetNetworkOutboundAccess)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetNetworkInboundAccess(&self, binbound: bool) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetNetworkInboundAccess)(windows_core::Interface::as_raw(self), binbound.into()).ok() }
    }
    pub unsafe fn SetNetworkOutboundAccess(&self, boutbound: bool) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetNetworkOutboundAccess)(windows_core::Interface::as_raw(self), boutbound.into()).ok() }
    }
    pub unsafe fn GetAuthenticationLevel(&self) -> windows_core::Result<AUTHENTICATION_LEVEL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetAuthenticationLevel)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetAuthenticationLevel(&self, authlevel: AUTHENTICATION_LEVEL) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetAuthenticationLevel)(windows_core::Interface::as_raw(self), authlevel).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDtcNetworkAccessConfig2_Vtbl {
    pub base__: IDtcNetworkAccessConfig_Vtbl,
    pub GetNetworkInboundAccess: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::BOOL) -> windows_core::HRESULT,
    pub GetNetworkOutboundAccess: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::BOOL) -> windows_core::HRESULT,
    pub SetNetworkInboundAccess: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::BOOL) -> windows_core::HRESULT,
    pub SetNetworkOutboundAccess: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::BOOL) -> windows_core::HRESULT,
    pub GetAuthenticationLevel: unsafe extern "system" fn(*mut core::ffi::c_void, *mut AUTHENTICATION_LEVEL) -> windows_core::HRESULT,
    pub SetAuthenticationLevel: unsafe extern "system" fn(*mut core::ffi::c_void, AUTHENTICATION_LEVEL) -> windows_core::HRESULT,
}
pub trait IDtcNetworkAccessConfig2_Impl: IDtcNetworkAccessConfig_Impl {
    fn GetNetworkInboundAccess(&self) -> windows_core::Result<windows_core::BOOL>;
    fn GetNetworkOutboundAccess(&self) -> windows_core::Result<windows_core::BOOL>;
    fn SetNetworkInboundAccess(&self, binbound: windows_core::BOOL) -> windows_core::Result<()>;
    fn SetNetworkOutboundAccess(&self, boutbound: windows_core::BOOL) -> windows_core::Result<()>;
    fn GetAuthenticationLevel(&self) -> windows_core::Result<AUTHENTICATION_LEVEL>;
    fn SetAuthenticationLevel(&self, authlevel: AUTHENTICATION_LEVEL) -> windows_core::Result<()>;
}
impl IDtcNetworkAccessConfig2_Vtbl {
    pub const fn new<Identity: IDtcNetworkAccessConfig2_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetNetworkInboundAccess<Identity: IDtcNetworkAccessConfig2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbinbound: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDtcNetworkAccessConfig2_Impl::GetNetworkInboundAccess(this) {
                    Ok(ok__) => {
                        pbinbound.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetNetworkOutboundAccess<Identity: IDtcNetworkAccessConfig2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pboutbound: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDtcNetworkAccessConfig2_Impl::GetNetworkOutboundAccess(this) {
                    Ok(ok__) => {
                        pboutbound.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetNetworkInboundAccess<Identity: IDtcNetworkAccessConfig2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, binbound: windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDtcNetworkAccessConfig2_Impl::SetNetworkInboundAccess(this, core::mem::transmute_copy(&binbound)).into()
            }
        }
        unsafe extern "system" fn SetNetworkOutboundAccess<Identity: IDtcNetworkAccessConfig2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, boutbound: windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDtcNetworkAccessConfig2_Impl::SetNetworkOutboundAccess(this, core::mem::transmute_copy(&boutbound)).into()
            }
        }
        unsafe extern "system" fn GetAuthenticationLevel<Identity: IDtcNetworkAccessConfig2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pauthlevel: *mut AUTHENTICATION_LEVEL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDtcNetworkAccessConfig2_Impl::GetAuthenticationLevel(this) {
                    Ok(ok__) => {
                        pauthlevel.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetAuthenticationLevel<Identity: IDtcNetworkAccessConfig2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, authlevel: AUTHENTICATION_LEVEL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDtcNetworkAccessConfig2_Impl::SetAuthenticationLevel(this, core::mem::transmute_copy(&authlevel)).into()
            }
        }
        Self {
            base__: IDtcNetworkAccessConfig_Vtbl::new::<Identity, OFFSET>(),
            GetNetworkInboundAccess: GetNetworkInboundAccess::<Identity, OFFSET>,
            GetNetworkOutboundAccess: GetNetworkOutboundAccess::<Identity, OFFSET>,
            SetNetworkInboundAccess: SetNetworkInboundAccess::<Identity, OFFSET>,
            SetNetworkOutboundAccess: SetNetworkOutboundAccess::<Identity, OFFSET>,
            GetAuthenticationLevel: GetAuthenticationLevel::<Identity, OFFSET>,
            SetAuthenticationLevel: SetAuthenticationLevel::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDtcNetworkAccessConfig2 as windows_core::Interface>::IID || iid == &<IDtcNetworkAccessConfig as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IDtcNetworkAccessConfig2 {}
windows_core::imp::define_interface!(IDtcNetworkAccessConfig3, IDtcNetworkAccessConfig3_Vtbl, 0x76e4b4f3_2ca5_466b_89d5_fd218ee75b49);
impl core::ops::Deref for IDtcNetworkAccessConfig3 {
    type Target = IDtcNetworkAccessConfig2;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IDtcNetworkAccessConfig3, windows_core::IUnknown, IDtcNetworkAccessConfig, IDtcNetworkAccessConfig2);
impl IDtcNetworkAccessConfig3 {
    pub unsafe fn GetLUAccess(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetLUAccess)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetLUAccess(&self, bluaccess: bool) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetLUAccess)(windows_core::Interface::as_raw(self), bluaccess.into()).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDtcNetworkAccessConfig3_Vtbl {
    pub base__: IDtcNetworkAccessConfig2_Vtbl,
    pub GetLUAccess: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::BOOL) -> windows_core::HRESULT,
    pub SetLUAccess: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::BOOL) -> windows_core::HRESULT,
}
pub trait IDtcNetworkAccessConfig3_Impl: IDtcNetworkAccessConfig2_Impl {
    fn GetLUAccess(&self) -> windows_core::Result<windows_core::BOOL>;
    fn SetLUAccess(&self, bluaccess: windows_core::BOOL) -> windows_core::Result<()>;
}
impl IDtcNetworkAccessConfig3_Vtbl {
    pub const fn new<Identity: IDtcNetworkAccessConfig3_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetLUAccess<Identity: IDtcNetworkAccessConfig3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbluaccess: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDtcNetworkAccessConfig3_Impl::GetLUAccess(this) {
                    Ok(ok__) => {
                        pbluaccess.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetLUAccess<Identity: IDtcNetworkAccessConfig3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bluaccess: windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDtcNetworkAccessConfig3_Impl::SetLUAccess(this, core::mem::transmute_copy(&bluaccess)).into()
            }
        }
        Self {
            base__: IDtcNetworkAccessConfig2_Vtbl::new::<Identity, OFFSET>(),
            GetLUAccess: GetLUAccess::<Identity, OFFSET>,
            SetLUAccess: SetLUAccess::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDtcNetworkAccessConfig3 as windows_core::Interface>::IID || iid == &<IDtcNetworkAccessConfig as windows_core::Interface>::IID || iid == &<IDtcNetworkAccessConfig2 as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IDtcNetworkAccessConfig3 {}
windows_core::imp::define_interface!(IDtcToXaHelper, IDtcToXaHelper_Vtbl, 0xa9861611_304a_11d1_9813_00a0c905416e);
windows_core::imp::interface_hierarchy!(IDtcToXaHelper, windows_core::IUnknown);
impl IDtcToXaHelper {
    pub unsafe fn Close(&self, i_fdorecovery: bool) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Close)(windows_core::Interface::as_raw(self), i_fdorecovery.into()).ok() }
    }
    pub unsafe fn TranslateTridToXid<P0>(&self, pitransaction: P0, pguidbqual: *const windows_core::GUID, pxid: *mut XID) -> windows_core::Result<()>
    where
        P0: windows_core::Param<ITransaction>,
    {
        unsafe { (windows_core::Interface::vtable(self).TranslateTridToXid)(windows_core::Interface::as_raw(self), pitransaction.param().abi(), pguidbqual, pxid as _).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDtcToXaHelper_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Close: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::BOOL) -> windows_core::HRESULT,
    pub TranslateTridToXid: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *const windows_core::GUID, *mut XID) -> windows_core::HRESULT,
}
pub trait IDtcToXaHelper_Impl: windows_core::IUnknownImpl {
    fn Close(&self, i_fdorecovery: windows_core::BOOL) -> windows_core::Result<()>;
    fn TranslateTridToXid(&self, pitransaction: windows_core::Ref<ITransaction>, pguidbqual: *const windows_core::GUID, pxid: *mut XID) -> windows_core::Result<()>;
}
impl IDtcToXaHelper_Vtbl {
    pub const fn new<Identity: IDtcToXaHelper_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Close<Identity: IDtcToXaHelper_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, i_fdorecovery: windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDtcToXaHelper_Impl::Close(this, core::mem::transmute_copy(&i_fdorecovery)).into()
            }
        }
        unsafe extern "system" fn TranslateTridToXid<Identity: IDtcToXaHelper_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pitransaction: *mut core::ffi::c_void, pguidbqual: *const windows_core::GUID, pxid: *mut XID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDtcToXaHelper_Impl::TranslateTridToXid(this, core::mem::transmute_copy(&pitransaction), core::mem::transmute_copy(&pguidbqual), core::mem::transmute_copy(&pxid)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Close: Close::<Identity, OFFSET>,
            TranslateTridToXid: TranslateTridToXid::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDtcToXaHelper as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IDtcToXaHelper {}
windows_core::imp::define_interface!(IDtcToXaHelperFactory, IDtcToXaHelperFactory_Vtbl, 0xa9861610_304a_11d1_9813_00a0c905416e);
windows_core::imp::interface_hierarchy!(IDtcToXaHelperFactory, windows_core::IUnknown);
impl IDtcToXaHelperFactory {
    pub unsafe fn Create<P0, P1>(&self, pszdsn: P0, pszclientdllname: P1, pguidrm: *mut windows_core::GUID, ppxahelper: *mut Option<IDtcToXaHelper>) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCSTR>,
        P1: windows_core::Param<windows_core::PCSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).Create)(windows_core::Interface::as_raw(self), pszdsn.param().abi(), pszclientdllname.param().abi(), pguidrm as _, core::mem::transmute(ppxahelper)).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDtcToXaHelperFactory_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Create: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCSTR, windows_core::PCSTR, *mut windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IDtcToXaHelperFactory_Impl: windows_core::IUnknownImpl {
    fn Create(&self, pszdsn: &windows_core::PCSTR, pszclientdllname: &windows_core::PCSTR, pguidrm: *mut windows_core::GUID, ppxahelper: windows_core::OutRef<IDtcToXaHelper>) -> windows_core::Result<()>;
}
impl IDtcToXaHelperFactory_Vtbl {
    pub const fn new<Identity: IDtcToXaHelperFactory_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Create<Identity: IDtcToXaHelperFactory_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszdsn: windows_core::PCSTR, pszclientdllname: windows_core::PCSTR, pguidrm: *mut windows_core::GUID, ppxahelper: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDtcToXaHelperFactory_Impl::Create(this, core::mem::transmute(&pszdsn), core::mem::transmute(&pszclientdllname), core::mem::transmute_copy(&pguidrm), core::mem::transmute_copy(&ppxahelper)).into()
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), Create: Create::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDtcToXaHelperFactory as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IDtcToXaHelperFactory {}
windows_core::imp::define_interface!(IDtcToXaHelperSinglePipe, IDtcToXaHelperSinglePipe_Vtbl, 0x47ed4971_53b3_11d1_bbb9_00c04fd658f6);
windows_core::imp::interface_hierarchy!(IDtcToXaHelperSinglePipe, windows_core::IUnknown);
impl IDtcToXaHelperSinglePipe {
    pub unsafe fn XARMCreate<P0, P1>(&self, pszdsn: P0, pszclientdll: P1, pdwrmcookie: *mut u32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCSTR>,
        P1: windows_core::Param<windows_core::PCSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).XARMCreate)(windows_core::Interface::as_raw(self), pszdsn.param().abi(), pszclientdll.param().abi(), pdwrmcookie as _).ok() }
    }
    pub unsafe fn ConvertTridToXID(&self, pdwitrans: *mut u32, dwrmcookie: u32, pxid: *mut XID) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).ConvertTridToXID)(windows_core::Interface::as_raw(self), pdwitrans as _, dwrmcookie, pxid as _).ok() }
    }
    pub unsafe fn EnlistWithRM<P1, P2>(&self, dwrmcookie: u32, i_pitransaction: P1, i_pitransres: P2) -> windows_core::Result<ITransactionEnlistmentAsync>
    where
        P1: windows_core::Param<ITransaction>,
        P2: windows_core::Param<ITransactionResourceAsync>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).EnlistWithRM)(windows_core::Interface::as_raw(self), dwrmcookie, i_pitransaction.param().abi(), i_pitransres.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn ReleaseRMCookie(&self, i_dwrmcookie: u32, i_fnormal: bool) {
        unsafe { (windows_core::Interface::vtable(self).ReleaseRMCookie)(windows_core::Interface::as_raw(self), i_dwrmcookie, i_fnormal.into()) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDtcToXaHelperSinglePipe_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub XARMCreate: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCSTR, windows_core::PCSTR, *mut u32) -> windows_core::HRESULT,
    pub ConvertTridToXID: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32, u32, *mut XID) -> windows_core::HRESULT,
    pub EnlistWithRM: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ReleaseRMCookie: unsafe extern "system" fn(*mut core::ffi::c_void, u32, windows_core::BOOL),
}
pub trait IDtcToXaHelperSinglePipe_Impl: windows_core::IUnknownImpl {
    fn XARMCreate(&self, pszdsn: &windows_core::PCSTR, pszclientdll: &windows_core::PCSTR, pdwrmcookie: *mut u32) -> windows_core::Result<()>;
    fn ConvertTridToXID(&self, pdwitrans: *mut u32, dwrmcookie: u32, pxid: *mut XID) -> windows_core::Result<()>;
    fn EnlistWithRM(&self, dwrmcookie: u32, i_pitransaction: windows_core::Ref<ITransaction>, i_pitransres: windows_core::Ref<ITransactionResourceAsync>) -> windows_core::Result<ITransactionEnlistmentAsync>;
    fn ReleaseRMCookie(&self, i_dwrmcookie: u32, i_fnormal: windows_core::BOOL);
}
impl IDtcToXaHelperSinglePipe_Vtbl {
    pub const fn new<Identity: IDtcToXaHelperSinglePipe_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn XARMCreate<Identity: IDtcToXaHelperSinglePipe_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszdsn: windows_core::PCSTR, pszclientdll: windows_core::PCSTR, pdwrmcookie: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDtcToXaHelperSinglePipe_Impl::XARMCreate(this, core::mem::transmute(&pszdsn), core::mem::transmute(&pszclientdll), core::mem::transmute_copy(&pdwrmcookie)).into()
            }
        }
        unsafe extern "system" fn ConvertTridToXID<Identity: IDtcToXaHelperSinglePipe_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwitrans: *mut u32, dwrmcookie: u32, pxid: *mut XID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDtcToXaHelperSinglePipe_Impl::ConvertTridToXID(this, core::mem::transmute_copy(&pdwitrans), core::mem::transmute_copy(&dwrmcookie), core::mem::transmute_copy(&pxid)).into()
            }
        }
        unsafe extern "system" fn EnlistWithRM<Identity: IDtcToXaHelperSinglePipe_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwrmcookie: u32, i_pitransaction: *mut core::ffi::c_void, i_pitransres: *mut core::ffi::c_void, o_ppitransenslitment: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDtcToXaHelperSinglePipe_Impl::EnlistWithRM(this, core::mem::transmute_copy(&dwrmcookie), core::mem::transmute_copy(&i_pitransaction), core::mem::transmute_copy(&i_pitransres)) {
                    Ok(ok__) => {
                        o_ppitransenslitment.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn ReleaseRMCookie<Identity: IDtcToXaHelperSinglePipe_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, i_dwrmcookie: u32, i_fnormal: windows_core::BOOL) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDtcToXaHelperSinglePipe_Impl::ReleaseRMCookie(this, core::mem::transmute_copy(&i_dwrmcookie), core::mem::transmute_copy(&i_fnormal))
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            XARMCreate: XARMCreate::<Identity, OFFSET>,
            ConvertTridToXID: ConvertTridToXID::<Identity, OFFSET>,
            EnlistWithRM: EnlistWithRM::<Identity, OFFSET>,
            ReleaseRMCookie: ReleaseRMCookie::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDtcToXaHelperSinglePipe as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IDtcToXaHelperSinglePipe {}
windows_core::imp::define_interface!(IDtcToXaMapper, IDtcToXaMapper_Vtbl, 0x64ffabe0_7ce9_11d0_8ce6_00c04fdc877e);
windows_core::imp::interface_hierarchy!(IDtcToXaMapper, windows_core::IUnknown);
impl IDtcToXaMapper {
    pub unsafe fn RequestNewResourceManager<P0, P1>(&self, pszdsn: P0, pszclientdllname: P1, pdwrmcookie: *mut u32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCSTR>,
        P1: windows_core::Param<windows_core::PCSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).RequestNewResourceManager)(windows_core::Interface::as_raw(self), pszdsn.param().abi(), pszclientdllname.param().abi(), pdwrmcookie as _).ok() }
    }
    pub unsafe fn TranslateTridToXid(&self, pdwitransaction: *const u32, dwrmcookie: u32, pxid: *mut XID) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).TranslateTridToXid)(windows_core::Interface::as_raw(self), pdwitransaction, dwrmcookie, pxid as _).ok() }
    }
    pub unsafe fn EnlistResourceManager(&self, dwrmcookie: u32, pdwitransaction: *const u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).EnlistResourceManager)(windows_core::Interface::as_raw(self), dwrmcookie, pdwitransaction).ok() }
    }
    pub unsafe fn ReleaseResourceManager(&self, dwrmcookie: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).ReleaseResourceManager)(windows_core::Interface::as_raw(self), dwrmcookie).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDtcToXaMapper_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub RequestNewResourceManager: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCSTR, windows_core::PCSTR, *mut u32) -> windows_core::HRESULT,
    pub TranslateTridToXid: unsafe extern "system" fn(*mut core::ffi::c_void, *const u32, u32, *mut XID) -> windows_core::HRESULT,
    pub EnlistResourceManager: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const u32) -> windows_core::HRESULT,
    pub ReleaseResourceManager: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
}
pub trait IDtcToXaMapper_Impl: windows_core::IUnknownImpl {
    fn RequestNewResourceManager(&self, pszdsn: &windows_core::PCSTR, pszclientdllname: &windows_core::PCSTR, pdwrmcookie: *mut u32) -> windows_core::Result<()>;
    fn TranslateTridToXid(&self, pdwitransaction: *const u32, dwrmcookie: u32, pxid: *mut XID) -> windows_core::Result<()>;
    fn EnlistResourceManager(&self, dwrmcookie: u32, pdwitransaction: *const u32) -> windows_core::Result<()>;
    fn ReleaseResourceManager(&self, dwrmcookie: u32) -> windows_core::Result<()>;
}
impl IDtcToXaMapper_Vtbl {
    pub const fn new<Identity: IDtcToXaMapper_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn RequestNewResourceManager<Identity: IDtcToXaMapper_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszdsn: windows_core::PCSTR, pszclientdllname: windows_core::PCSTR, pdwrmcookie: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDtcToXaMapper_Impl::RequestNewResourceManager(this, core::mem::transmute(&pszdsn), core::mem::transmute(&pszclientdllname), core::mem::transmute_copy(&pdwrmcookie)).into()
            }
        }
        unsafe extern "system" fn TranslateTridToXid<Identity: IDtcToXaMapper_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwitransaction: *const u32, dwrmcookie: u32, pxid: *mut XID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDtcToXaMapper_Impl::TranslateTridToXid(this, core::mem::transmute_copy(&pdwitransaction), core::mem::transmute_copy(&dwrmcookie), core::mem::transmute_copy(&pxid)).into()
            }
        }
        unsafe extern "system" fn EnlistResourceManager<Identity: IDtcToXaMapper_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwrmcookie: u32, pdwitransaction: *const u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDtcToXaMapper_Impl::EnlistResourceManager(this, core::mem::transmute_copy(&dwrmcookie), core::mem::transmute_copy(&pdwitransaction)).into()
            }
        }
        unsafe extern "system" fn ReleaseResourceManager<Identity: IDtcToXaMapper_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwrmcookie: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDtcToXaMapper_Impl::ReleaseResourceManager(this, core::mem::transmute_copy(&dwrmcookie)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            RequestNewResourceManager: RequestNewResourceManager::<Identity, OFFSET>,
            TranslateTridToXid: TranslateTridToXid::<Identity, OFFSET>,
            EnlistResourceManager: EnlistResourceManager::<Identity, OFFSET>,
            ReleaseResourceManager: ReleaseResourceManager::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDtcToXaMapper as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IDtcToXaMapper {}
windows_core::imp::define_interface!(IGetDispenser, IGetDispenser_Vtbl, 0xc23cc370_87ef_11ce_8081_0080c758527e);
windows_core::imp::interface_hierarchy!(IGetDispenser, windows_core::IUnknown);
impl IGetDispenser {
    pub unsafe fn GetDispenser(&self, iid: *const windows_core::GUID, ppvobject: *mut *mut core::ffi::c_void) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetDispenser)(windows_core::Interface::as_raw(self), iid, ppvobject as _).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IGetDispenser_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetDispenser: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IGetDispenser_Impl: windows_core::IUnknownImpl {
    fn GetDispenser(&self, iid: *const windows_core::GUID, ppvobject: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
}
impl IGetDispenser_Vtbl {
    pub const fn new<Identity: IGetDispenser_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetDispenser<Identity: IGetDispenser_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, iid: *const windows_core::GUID, ppvobject: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IGetDispenser_Impl::GetDispenser(this, core::mem::transmute_copy(&iid), core::mem::transmute_copy(&ppvobject)).into()
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), GetDispenser: GetDispenser::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IGetDispenser as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IGetDispenser {}
windows_core::imp::define_interface!(IKernelTransaction, IKernelTransaction_Vtbl, 0x79427a2b_f895_40e0_be79_b57dc82ed231);
windows_core::imp::interface_hierarchy!(IKernelTransaction, windows_core::IUnknown);
impl IKernelTransaction {
    pub unsafe fn GetHandle(&self) -> windows_core::Result<super::super::Foundation::HANDLE> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetHandle)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IKernelTransaction_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetHandle: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::HANDLE) -> windows_core::HRESULT,
}
pub trait IKernelTransaction_Impl: windows_core::IUnknownImpl {
    fn GetHandle(&self) -> windows_core::Result<super::super::Foundation::HANDLE>;
}
impl IKernelTransaction_Vtbl {
    pub const fn new<Identity: IKernelTransaction_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetHandle<Identity: IKernelTransaction_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, phandle: *mut super::super::Foundation::HANDLE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IKernelTransaction_Impl::GetHandle(this) {
                    Ok(ok__) => {
                        phandle.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), GetHandle: GetHandle::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IKernelTransaction as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IKernelTransaction {}
windows_core::imp::define_interface!(ILastResourceManager, ILastResourceManager_Vtbl, 0x4d964ad4_5b33_11d3_8a91_00c04f79eb6d);
windows_core::imp::interface_hierarchy!(ILastResourceManager, windows_core::IUnknown);
impl ILastResourceManager {
    pub unsafe fn TransactionCommitted(&self, pprepinfo: &[u8]) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).TransactionCommitted)(windows_core::Interface::as_raw(self), core::mem::transmute(pprepinfo.as_ptr()), pprepinfo.len().try_into().unwrap()).ok() }
    }
    pub unsafe fn RecoveryDone(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).RecoveryDone)(windows_core::Interface::as_raw(self)).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ILastResourceManager_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub TransactionCommitted: unsafe extern "system" fn(*mut core::ffi::c_void, *const u8, u32) -> windows_core::HRESULT,
    pub RecoveryDone: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait ILastResourceManager_Impl: windows_core::IUnknownImpl {
    fn TransactionCommitted(&self, pprepinfo: *const u8, cbprepinfo: u32) -> windows_core::Result<()>;
    fn RecoveryDone(&self) -> windows_core::Result<()>;
}
impl ILastResourceManager_Vtbl {
    pub const fn new<Identity: ILastResourceManager_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn TransactionCommitted<Identity: ILastResourceManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pprepinfo: *const u8, cbprepinfo: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ILastResourceManager_Impl::TransactionCommitted(this, core::mem::transmute_copy(&pprepinfo), core::mem::transmute_copy(&cbprepinfo)).into()
            }
        }
        unsafe extern "system" fn RecoveryDone<Identity: ILastResourceManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ILastResourceManager_Impl::RecoveryDone(this).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            TransactionCommitted: TransactionCommitted::<Identity, OFFSET>,
            RecoveryDone: RecoveryDone::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ILastResourceManager as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for ILastResourceManager {}
pub const INCOMING_AUTHENTICATION_REQUIRED: AUTHENTICATION_LEVEL = AUTHENTICATION_LEVEL(1i32);
windows_core::imp::define_interface!(IPrepareInfo, IPrepareInfo_Vtbl, 0x80c7bfd0_87ee_11ce_8081_0080c758527e);
windows_core::imp::interface_hierarchy!(IPrepareInfo, windows_core::IUnknown);
impl IPrepareInfo {
    pub unsafe fn GetPrepareInfoSize(&self, pcbprepinfo: *mut u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetPrepareInfoSize)(windows_core::Interface::as_raw(self), pcbprepinfo as _).ok() }
    }
    pub unsafe fn GetPrepareInfo(&self, pprepinfo: *mut u8) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetPrepareInfo)(windows_core::Interface::as_raw(self), pprepinfo as _).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IPrepareInfo_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetPrepareInfoSize: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub GetPrepareInfo: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u8) -> windows_core::HRESULT,
}
pub trait IPrepareInfo_Impl: windows_core::IUnknownImpl {
    fn GetPrepareInfoSize(&self, pcbprepinfo: *mut u32) -> windows_core::Result<()>;
    fn GetPrepareInfo(&self, pprepinfo: *mut u8) -> windows_core::Result<()>;
}
impl IPrepareInfo_Vtbl {
    pub const fn new<Identity: IPrepareInfo_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetPrepareInfoSize<Identity: IPrepareInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcbprepinfo: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPrepareInfo_Impl::GetPrepareInfoSize(this, core::mem::transmute_copy(&pcbprepinfo)).into()
            }
        }
        unsafe extern "system" fn GetPrepareInfo<Identity: IPrepareInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pprepinfo: *mut u8) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPrepareInfo_Impl::GetPrepareInfo(this, core::mem::transmute_copy(&pprepinfo)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetPrepareInfoSize: GetPrepareInfoSize::<Identity, OFFSET>,
            GetPrepareInfo: GetPrepareInfo::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IPrepareInfo as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IPrepareInfo {}
windows_core::imp::define_interface!(IPrepareInfo2, IPrepareInfo2_Vtbl, 0x5fab2547_9779_11d1_b886_00c04fb9618a);
windows_core::imp::interface_hierarchy!(IPrepareInfo2, windows_core::IUnknown);
impl IPrepareInfo2 {
    pub unsafe fn GetPrepareInfoSize(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetPrepareInfoSize)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetPrepareInfo(&self, pprepinfo: &mut [u8]) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetPrepareInfo)(windows_core::Interface::as_raw(self), pprepinfo.len().try_into().unwrap(), core::mem::transmute(pprepinfo.as_ptr())).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IPrepareInfo2_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetPrepareInfoSize: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub GetPrepareInfo: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut u8) -> windows_core::HRESULT,
}
pub trait IPrepareInfo2_Impl: windows_core::IUnknownImpl {
    fn GetPrepareInfoSize(&self) -> windows_core::Result<u32>;
    fn GetPrepareInfo(&self, cbprepareinfo: u32, pprepinfo: *mut u8) -> windows_core::Result<()>;
}
impl IPrepareInfo2_Vtbl {
    pub const fn new<Identity: IPrepareInfo2_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetPrepareInfoSize<Identity: IPrepareInfo2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcbprepinfo: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPrepareInfo2_Impl::GetPrepareInfoSize(this) {
                    Ok(ok__) => {
                        pcbprepinfo.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetPrepareInfo<Identity: IPrepareInfo2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, cbprepareinfo: u32, pprepinfo: *mut u8) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPrepareInfo2_Impl::GetPrepareInfo(this, core::mem::transmute_copy(&cbprepareinfo), core::mem::transmute_copy(&pprepinfo)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetPrepareInfoSize: GetPrepareInfoSize::<Identity, OFFSET>,
            GetPrepareInfo: GetPrepareInfo::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IPrepareInfo2 as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IPrepareInfo2 {}
windows_core::imp::define_interface!(IRMHelper, IRMHelper_Vtbl, 0xe793f6d1_f53d_11cf_a60d_00a0c905416e);
windows_core::imp::interface_hierarchy!(IRMHelper, windows_core::IUnknown);
impl IRMHelper {
    pub unsafe fn RMCount(&self, dwctotalnumberofrms: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).RMCount)(windows_core::Interface::as_raw(self), dwctotalnumberofrms).ok() }
    }
    pub unsafe fn RMInfo<P2, P3>(&self, pxa_switch: *mut xa_switch_t, fcdeclcallingconv: bool, pszopenstring: P2, pszclosestring: P3, guidrmrecovery: windows_core::GUID) -> windows_core::Result<()>
    where
        P2: windows_core::Param<windows_core::PCSTR>,
        P3: windows_core::Param<windows_core::PCSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).RMInfo)(windows_core::Interface::as_raw(self), pxa_switch as _, fcdeclcallingconv.into(), pszopenstring.param().abi(), pszclosestring.param().abi(), core::mem::transmute(guidrmrecovery)).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IRMHelper_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub RMCount: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub RMInfo: unsafe extern "system" fn(*mut core::ffi::c_void, *mut xa_switch_t, windows_core::BOOL, windows_core::PCSTR, windows_core::PCSTR, windows_core::GUID) -> windows_core::HRESULT,
}
pub trait IRMHelper_Impl: windows_core::IUnknownImpl {
    fn RMCount(&self, dwctotalnumberofrms: u32) -> windows_core::Result<()>;
    fn RMInfo(&self, pxa_switch: *mut xa_switch_t, fcdeclcallingconv: windows_core::BOOL, pszopenstring: &windows_core::PCSTR, pszclosestring: &windows_core::PCSTR, guidrmrecovery: &windows_core::GUID) -> windows_core::Result<()>;
}
impl IRMHelper_Vtbl {
    pub const fn new<Identity: IRMHelper_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn RMCount<Identity: IRMHelper_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwctotalnumberofrms: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IRMHelper_Impl::RMCount(this, core::mem::transmute_copy(&dwctotalnumberofrms)).into()
            }
        }
        unsafe extern "system" fn RMInfo<Identity: IRMHelper_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pxa_switch: *mut xa_switch_t, fcdeclcallingconv: windows_core::BOOL, pszopenstring: windows_core::PCSTR, pszclosestring: windows_core::PCSTR, guidrmrecovery: windows_core::GUID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IRMHelper_Impl::RMInfo(this, core::mem::transmute_copy(&pxa_switch), core::mem::transmute_copy(&fcdeclcallingconv), core::mem::transmute(&pszopenstring), core::mem::transmute(&pszclosestring), core::mem::transmute(&guidrmrecovery)).into()
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), RMCount: RMCount::<Identity, OFFSET>, RMInfo: RMInfo::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IRMHelper as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IRMHelper {}
windows_core::imp::define_interface!(IResourceManager, IResourceManager_Vtbl, 0x13741d21_87eb_11ce_8081_0080c758527e);
windows_core::imp::interface_hierarchy!(IResourceManager, windows_core::IUnknown);
impl IResourceManager {
    pub unsafe fn Enlist<P0, P1>(&self, ptransaction: P0, pres: P1, puow: *mut BOID, pisolevel: *mut i32, ppenlist: *mut Option<ITransactionEnlistmentAsync>) -> windows_core::Result<()>
    where
        P0: windows_core::Param<ITransaction>,
        P1: windows_core::Param<ITransactionResourceAsync>,
    {
        unsafe { (windows_core::Interface::vtable(self).Enlist)(windows_core::Interface::as_raw(self), ptransaction.param().abi(), pres.param().abi(), puow as _, pisolevel as _, core::mem::transmute(ppenlist)).ok() }
    }
    pub unsafe fn Reenlist(&self, pprepinfo: &[u8], ltimeout: u32) -> windows_core::Result<XACTSTAT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Reenlist)(windows_core::Interface::as_raw(self), core::mem::transmute(pprepinfo.as_ptr()), pprepinfo.len().try_into().unwrap(), ltimeout, &mut result__).map(|| result__)
        }
    }
    pub unsafe fn ReenlistmentComplete(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).ReenlistmentComplete)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn GetDistributedTransactionManager(&self, iid: *const windows_core::GUID, ppvobject: *mut *mut core::ffi::c_void) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetDistributedTransactionManager)(windows_core::Interface::as_raw(self), iid, ppvobject as _).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IResourceManager_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Enlist: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut BOID, *mut i32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Reenlist: unsafe extern "system" fn(*mut core::ffi::c_void, *const u8, u32, u32, *mut XACTSTAT) -> windows_core::HRESULT,
    pub ReenlistmentComplete: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetDistributedTransactionManager: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IResourceManager_Impl: windows_core::IUnknownImpl {
    fn Enlist(&self, ptransaction: windows_core::Ref<ITransaction>, pres: windows_core::Ref<ITransactionResourceAsync>, puow: *mut BOID, pisolevel: *mut i32, ppenlist: windows_core::OutRef<ITransactionEnlistmentAsync>) -> windows_core::Result<()>;
    fn Reenlist(&self, pprepinfo: *const u8, cbprepinfo: u32, ltimeout: u32) -> windows_core::Result<XACTSTAT>;
    fn ReenlistmentComplete(&self) -> windows_core::Result<()>;
    fn GetDistributedTransactionManager(&self, iid: *const windows_core::GUID, ppvobject: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
}
impl IResourceManager_Vtbl {
    pub const fn new<Identity: IResourceManager_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Enlist<Identity: IResourceManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ptransaction: *mut core::ffi::c_void, pres: *mut core::ffi::c_void, puow: *mut BOID, pisolevel: *mut i32, ppenlist: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IResourceManager_Impl::Enlist(this, core::mem::transmute_copy(&ptransaction), core::mem::transmute_copy(&pres), core::mem::transmute_copy(&puow), core::mem::transmute_copy(&pisolevel), core::mem::transmute_copy(&ppenlist)).into()
            }
        }
        unsafe extern "system" fn Reenlist<Identity: IResourceManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pprepinfo: *const u8, cbprepinfo: u32, ltimeout: u32, pxactstat: *mut XACTSTAT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IResourceManager_Impl::Reenlist(this, core::mem::transmute_copy(&pprepinfo), core::mem::transmute_copy(&cbprepinfo), core::mem::transmute_copy(&ltimeout)) {
                    Ok(ok__) => {
                        pxactstat.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn ReenlistmentComplete<Identity: IResourceManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IResourceManager_Impl::ReenlistmentComplete(this).into()
            }
        }
        unsafe extern "system" fn GetDistributedTransactionManager<Identity: IResourceManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, iid: *const windows_core::GUID, ppvobject: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IResourceManager_Impl::GetDistributedTransactionManager(this, core::mem::transmute_copy(&iid), core::mem::transmute_copy(&ppvobject)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Enlist: Enlist::<Identity, OFFSET>,
            Reenlist: Reenlist::<Identity, OFFSET>,
            ReenlistmentComplete: ReenlistmentComplete::<Identity, OFFSET>,
            GetDistributedTransactionManager: GetDistributedTransactionManager::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IResourceManager as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IResourceManager {}
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
        unsafe { (windows_core::Interface::vtable(self).Enlist2)(windows_core::Interface::as_raw(self), ptransaction.param().abi(), presasync.param().abi(), puow as _, pisolevel as _, pxid as _, core::mem::transmute(ppenlist)).ok() }
    }
    pub unsafe fn Reenlist2(&self, pxid: *const XID, dwtimeout: u32) -> windows_core::Result<XACTSTAT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Reenlist2)(windows_core::Interface::as_raw(self), pxid, dwtimeout, &mut result__).map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IResourceManager2_Vtbl {
    pub base__: IResourceManager_Vtbl,
    pub Enlist2: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut BOID, *mut i32, *mut XID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Reenlist2: unsafe extern "system" fn(*mut core::ffi::c_void, *const XID, u32, *mut XACTSTAT) -> windows_core::HRESULT,
}
pub trait IResourceManager2_Impl: IResourceManager_Impl {
    fn Enlist2(&self, ptransaction: windows_core::Ref<ITransaction>, presasync: windows_core::Ref<ITransactionResourceAsync>, puow: *mut BOID, pisolevel: *mut i32, pxid: *mut XID, ppenlist: windows_core::OutRef<ITransactionEnlistmentAsync>) -> windows_core::Result<()>;
    fn Reenlist2(&self, pxid: *const XID, dwtimeout: u32) -> windows_core::Result<XACTSTAT>;
}
impl IResourceManager2_Vtbl {
    pub const fn new<Identity: IResourceManager2_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Enlist2<Identity: IResourceManager2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ptransaction: *mut core::ffi::c_void, presasync: *mut core::ffi::c_void, puow: *mut BOID, pisolevel: *mut i32, pxid: *mut XID, ppenlist: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IResourceManager2_Impl::Enlist2(this, core::mem::transmute_copy(&ptransaction), core::mem::transmute_copy(&presasync), core::mem::transmute_copy(&puow), core::mem::transmute_copy(&pisolevel), core::mem::transmute_copy(&pxid), core::mem::transmute_copy(&ppenlist)).into()
            }
        }
        unsafe extern "system" fn Reenlist2<Identity: IResourceManager2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pxid: *const XID, dwtimeout: u32, pxactstat: *mut XACTSTAT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IResourceManager2_Impl::Reenlist2(this, core::mem::transmute_copy(&pxid), core::mem::transmute_copy(&dwtimeout)) {
                    Ok(ok__) => {
                        pxactstat.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self { base__: IResourceManager_Vtbl::new::<Identity, OFFSET>(), Enlist2: Enlist2::<Identity, OFFSET>, Reenlist2: Reenlist2::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IResourceManager2 as windows_core::Interface>::IID || iid == &<IResourceManager as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IResourceManager2 {}
windows_core::imp::define_interface!(IResourceManagerFactory, IResourceManagerFactory_Vtbl, 0x13741d20_87eb_11ce_8081_0080c758527e);
windows_core::imp::interface_hierarchy!(IResourceManagerFactory, windows_core::IUnknown);
impl IResourceManagerFactory {
    pub unsafe fn Create<P1, P2>(&self, pguidrm: *const windows_core::GUID, pszrmname: P1, piresmgrsink: P2) -> windows_core::Result<IResourceManager>
    where
        P1: windows_core::Param<windows_core::PCSTR>,
        P2: windows_core::Param<IResourceManagerSink>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Create)(windows_core::Interface::as_raw(self), pguidrm, pszrmname.param().abi(), piresmgrsink.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IResourceManagerFactory_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Create: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, windows_core::PCSTR, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IResourceManagerFactory_Impl: windows_core::IUnknownImpl {
    fn Create(&self, pguidrm: *const windows_core::GUID, pszrmname: &windows_core::PCSTR, piresmgrsink: windows_core::Ref<IResourceManagerSink>) -> windows_core::Result<IResourceManager>;
}
impl IResourceManagerFactory_Vtbl {
    pub const fn new<Identity: IResourceManagerFactory_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Create<Identity: IResourceManagerFactory_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pguidrm: *const windows_core::GUID, pszrmname: windows_core::PCSTR, piresmgrsink: *mut core::ffi::c_void, ppresmgr: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IResourceManagerFactory_Impl::Create(this, core::mem::transmute_copy(&pguidrm), core::mem::transmute(&pszrmname), core::mem::transmute_copy(&piresmgrsink)) {
                    Ok(ok__) => {
                        ppresmgr.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), Create: Create::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IResourceManagerFactory as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IResourceManagerFactory {}
windows_core::imp::define_interface!(IResourceManagerFactory2, IResourceManagerFactory2_Vtbl, 0x6b369c21_fbd2_11d1_8f47_00c04f8ee57d);
impl core::ops::Deref for IResourceManagerFactory2 {
    type Target = IResourceManagerFactory;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IResourceManagerFactory2, windows_core::IUnknown, IResourceManagerFactory);
impl IResourceManagerFactory2 {
    pub unsafe fn CreateEx<P1, P2>(&self, pguidrm: *const windows_core::GUID, pszrmname: P1, piresmgrsink: P2, riidrequested: *const windows_core::GUID, ppvresmgr: *mut *mut core::ffi::c_void) -> windows_core::Result<()>
    where
        P1: windows_core::Param<windows_core::PCSTR>,
        P2: windows_core::Param<IResourceManagerSink>,
    {
        unsafe { (windows_core::Interface::vtable(self).CreateEx)(windows_core::Interface::as_raw(self), pguidrm, pszrmname.param().abi(), piresmgrsink.param().abi(), riidrequested, ppvresmgr as _).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IResourceManagerFactory2_Vtbl {
    pub base__: IResourceManagerFactory_Vtbl,
    pub CreateEx: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, windows_core::PCSTR, *mut core::ffi::c_void, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IResourceManagerFactory2_Impl: IResourceManagerFactory_Impl {
    fn CreateEx(&self, pguidrm: *const windows_core::GUID, pszrmname: &windows_core::PCSTR, piresmgrsink: windows_core::Ref<IResourceManagerSink>, riidrequested: *const windows_core::GUID, ppvresmgr: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
}
impl IResourceManagerFactory2_Vtbl {
    pub const fn new<Identity: IResourceManagerFactory2_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn CreateEx<Identity: IResourceManagerFactory2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pguidrm: *const windows_core::GUID, pszrmname: windows_core::PCSTR, piresmgrsink: *mut core::ffi::c_void, riidrequested: *const windows_core::GUID, ppvresmgr: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IResourceManagerFactory2_Impl::CreateEx(this, core::mem::transmute_copy(&pguidrm), core::mem::transmute(&pszrmname), core::mem::transmute_copy(&piresmgrsink), core::mem::transmute_copy(&riidrequested), core::mem::transmute_copy(&ppvresmgr)).into()
            }
        }
        Self { base__: IResourceManagerFactory_Vtbl::new::<Identity, OFFSET>(), CreateEx: CreateEx::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IResourceManagerFactory2 as windows_core::Interface>::IID || iid == &<IResourceManagerFactory as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IResourceManagerFactory2 {}
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
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Rejoin)(windows_core::Interface::as_raw(self), core::mem::transmute(pprepinfo.as_ptr()), pprepinfo.len().try_into().unwrap(), ltimeout, &mut result__).map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IResourceManagerRejoinable_Vtbl {
    pub base__: IResourceManager2_Vtbl,
    pub Rejoin: unsafe extern "system" fn(*mut core::ffi::c_void, *const u8, u32, u32, *mut XACTSTAT) -> windows_core::HRESULT,
}
pub trait IResourceManagerRejoinable_Impl: IResourceManager2_Impl {
    fn Rejoin(&self, pprepinfo: *const u8, cbprepinfo: u32, ltimeout: u32) -> windows_core::Result<XACTSTAT>;
}
impl IResourceManagerRejoinable_Vtbl {
    pub const fn new<Identity: IResourceManagerRejoinable_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Rejoin<Identity: IResourceManagerRejoinable_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pprepinfo: *const u8, cbprepinfo: u32, ltimeout: u32, pxactstat: *mut XACTSTAT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IResourceManagerRejoinable_Impl::Rejoin(this, core::mem::transmute_copy(&pprepinfo), core::mem::transmute_copy(&cbprepinfo), core::mem::transmute_copy(&ltimeout)) {
                    Ok(ok__) => {
                        pxactstat.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self { base__: IResourceManager2_Vtbl::new::<Identity, OFFSET>(), Rejoin: Rejoin::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IResourceManagerRejoinable as windows_core::Interface>::IID || iid == &<IResourceManager as windows_core::Interface>::IID || iid == &<IResourceManager2 as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IResourceManagerRejoinable {}
windows_core::imp::define_interface!(IResourceManagerSink, IResourceManagerSink_Vtbl, 0x0d563181_defb_11ce_aed1_00aa0051e2c4);
windows_core::imp::interface_hierarchy!(IResourceManagerSink, windows_core::IUnknown);
impl IResourceManagerSink {
    pub unsafe fn TMDown(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).TMDown)(windows_core::Interface::as_raw(self)).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IResourceManagerSink_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub TMDown: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IResourceManagerSink_Impl: windows_core::IUnknownImpl {
    fn TMDown(&self) -> windows_core::Result<()>;
}
impl IResourceManagerSink_Vtbl {
    pub const fn new<Identity: IResourceManagerSink_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn TMDown<Identity: IResourceManagerSink_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IResourceManagerSink_Impl::TMDown(this).into()
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), TMDown: TMDown::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IResourceManagerSink as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IResourceManagerSink {}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct ISOFLAG(pub i32);
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
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct ISOLATIONLEVEL(pub i32);
pub const ISOLATIONLEVEL_BROWSE: ISOLATIONLEVEL = ISOLATIONLEVEL(256i32);
pub const ISOLATIONLEVEL_CHAOS: ISOLATIONLEVEL = ISOLATIONLEVEL(16i32);
pub const ISOLATIONLEVEL_CURSORSTABILITY: ISOLATIONLEVEL = ISOLATIONLEVEL(4096i32);
pub const ISOLATIONLEVEL_ISOLATED: ISOLATIONLEVEL = ISOLATIONLEVEL(1048576i32);
pub const ISOLATIONLEVEL_READCOMMITTED: ISOLATIONLEVEL = ISOLATIONLEVEL(4096i32);
pub const ISOLATIONLEVEL_READUNCOMMITTED: ISOLATIONLEVEL = ISOLATIONLEVEL(256i32);
pub const ISOLATIONLEVEL_REPEATABLEREAD: ISOLATIONLEVEL = ISOLATIONLEVEL(65536i32);
pub const ISOLATIONLEVEL_SERIALIZABLE: ISOLATIONLEVEL = ISOLATIONLEVEL(1048576i32);
pub const ISOLATIONLEVEL_UNSPECIFIED: ISOLATIONLEVEL = ISOLATIONLEVEL(-1i32);
windows_core::imp::define_interface!(ITipHelper, ITipHelper_Vtbl, 0x17cf72d1_bac5_11d1_b1bf_00c04fc2f3ef);
windows_core::imp::interface_hierarchy!(ITipHelper, windows_core::IUnknown);
impl ITipHelper {
    pub unsafe fn Pull(&self, i_psztxurl: *const u8) -> windows_core::Result<ITransaction> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Pull)(windows_core::Interface::as_raw(self), i_psztxurl, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn PullAsync<P1>(&self, i_psztxurl: *const u8, i_ptippullsink: P1) -> windows_core::Result<ITransaction>
    where
        P1: windows_core::Param<ITipPullSink>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).PullAsync)(windows_core::Interface::as_raw(self), i_psztxurl, i_ptippullsink.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetLocalTmUrl(&self) -> windows_core::Result<*mut u8> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetLocalTmUrl)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ITipHelper_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Pull: unsafe extern "system" fn(*mut core::ffi::c_void, *const u8, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub PullAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *const u8, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetLocalTmUrl: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut u8) -> windows_core::HRESULT,
}
pub trait ITipHelper_Impl: windows_core::IUnknownImpl {
    fn Pull(&self, i_psztxurl: *const u8) -> windows_core::Result<ITransaction>;
    fn PullAsync(&self, i_psztxurl: *const u8, i_ptippullsink: windows_core::Ref<ITipPullSink>) -> windows_core::Result<ITransaction>;
    fn GetLocalTmUrl(&self) -> windows_core::Result<*mut u8>;
}
impl ITipHelper_Vtbl {
    pub const fn new<Identity: ITipHelper_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Pull<Identity: ITipHelper_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, i_psztxurl: *const u8, o_ppitransaction: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITipHelper_Impl::Pull(this, core::mem::transmute_copy(&i_psztxurl)) {
                    Ok(ok__) => {
                        o_ppitransaction.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn PullAsync<Identity: ITipHelper_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, i_psztxurl: *const u8, i_ptippullsink: *mut core::ffi::c_void, o_ppitransaction: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITipHelper_Impl::PullAsync(this, core::mem::transmute_copy(&i_psztxurl), core::mem::transmute_copy(&i_ptippullsink)) {
                    Ok(ok__) => {
                        o_ppitransaction.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetLocalTmUrl<Identity: ITipHelper_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, o_ppszlocaltmurl: *mut *mut u8) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITipHelper_Impl::GetLocalTmUrl(this) {
                    Ok(ok__) => {
                        o_ppszlocaltmurl.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Pull: Pull::<Identity, OFFSET>,
            PullAsync: PullAsync::<Identity, OFFSET>,
            GetLocalTmUrl: GetLocalTmUrl::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITipHelper as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for ITipHelper {}
windows_core::imp::define_interface!(ITipPullSink, ITipPullSink_Vtbl, 0x17cf72d2_bac5_11d1_b1bf_00c04fc2f3ef);
windows_core::imp::interface_hierarchy!(ITipPullSink, windows_core::IUnknown);
impl ITipPullSink {
    pub unsafe fn PullComplete(&self, i_hrpull: windows_core::HRESULT) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).PullComplete)(windows_core::Interface::as_raw(self), i_hrpull).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ITipPullSink_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub PullComplete: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::HRESULT) -> windows_core::HRESULT,
}
pub trait ITipPullSink_Impl: windows_core::IUnknownImpl {
    fn PullComplete(&self, i_hrpull: windows_core::HRESULT) -> windows_core::Result<()>;
}
impl ITipPullSink_Vtbl {
    pub const fn new<Identity: ITipPullSink_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn PullComplete<Identity: ITipPullSink_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, i_hrpull: windows_core::HRESULT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITipPullSink_Impl::PullComplete(this, core::mem::transmute_copy(&i_hrpull)).into()
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), PullComplete: PullComplete::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITipPullSink as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for ITipPullSink {}
windows_core::imp::define_interface!(ITipTransaction, ITipTransaction_Vtbl, 0x17cf72d0_bac5_11d1_b1bf_00c04fc2f3ef);
windows_core::imp::interface_hierarchy!(ITipTransaction, windows_core::IUnknown);
impl ITipTransaction {
    pub unsafe fn Push(&self, i_pszremotetmurl: *const u8) -> windows_core::Result<windows_core::PSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Push)(windows_core::Interface::as_raw(self), i_pszremotetmurl, &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetTransactionUrl(&self) -> windows_core::Result<windows_core::PSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetTransactionUrl)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ITipTransaction_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Push: unsafe extern "system" fn(*mut core::ffi::c_void, *const u8, *mut windows_core::PSTR) -> windows_core::HRESULT,
    pub GetTransactionUrl: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::PSTR) -> windows_core::HRESULT,
}
pub trait ITipTransaction_Impl: windows_core::IUnknownImpl {
    fn Push(&self, i_pszremotetmurl: *const u8) -> windows_core::Result<windows_core::PSTR>;
    fn GetTransactionUrl(&self) -> windows_core::Result<windows_core::PSTR>;
}
impl ITipTransaction_Vtbl {
    pub const fn new<Identity: ITipTransaction_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Push<Identity: ITipTransaction_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, i_pszremotetmurl: *const u8, o_ppszremotetxurl: *mut windows_core::PSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITipTransaction_Impl::Push(this, core::mem::transmute_copy(&i_pszremotetmurl)) {
                    Ok(ok__) => {
                        o_ppszremotetxurl.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetTransactionUrl<Identity: ITipTransaction_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, o_ppszlocaltxurl: *mut windows_core::PSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITipTransaction_Impl::GetTransactionUrl(this) {
                    Ok(ok__) => {
                        o_ppszlocaltxurl.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Push: Push::<Identity, OFFSET>,
            GetTransactionUrl: GetTransactionUrl::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITipTransaction as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for ITipTransaction {}
windows_core::imp::define_interface!(ITmNodeName, ITmNodeName_Vtbl, 0x30274f88_6ee4_474e_9b95_7807bc9ef8cf);
windows_core::imp::interface_hierarchy!(ITmNodeName, windows_core::IUnknown);
impl ITmNodeName {
    pub unsafe fn GetNodeNameSize(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetNodeNameSize)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetNodeName(&self, cbnodenamebuffersize: u32, pnodenamebuffer: windows_core::PWSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetNodeName)(windows_core::Interface::as_raw(self), cbnodenamebuffersize, core::mem::transmute(pnodenamebuffer)).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ITmNodeName_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetNodeNameSize: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub GetNodeName: unsafe extern "system" fn(*mut core::ffi::c_void, u32, windows_core::PWSTR) -> windows_core::HRESULT,
}
pub trait ITmNodeName_Impl: windows_core::IUnknownImpl {
    fn GetNodeNameSize(&self) -> windows_core::Result<u32>;
    fn GetNodeName(&self, cbnodenamebuffersize: u32, pnodenamebuffer: windows_core::PWSTR) -> windows_core::Result<()>;
}
impl ITmNodeName_Vtbl {
    pub const fn new<Identity: ITmNodeName_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetNodeNameSize<Identity: ITmNodeName_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcbnodenamesize: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITmNodeName_Impl::GetNodeNameSize(this) {
                    Ok(ok__) => {
                        pcbnodenamesize.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetNodeName<Identity: ITmNodeName_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, cbnodenamebuffersize: u32, pnodenamebuffer: windows_core::PWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITmNodeName_Impl::GetNodeName(this, core::mem::transmute_copy(&cbnodenamebuffersize), core::mem::transmute_copy(&pnodenamebuffer)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetNodeNameSize: GetNodeNameSize::<Identity, OFFSET>,
            GetNodeName: GetNodeName::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITmNodeName as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for ITmNodeName {}
windows_core::imp::define_interface!(ITransaction, ITransaction_Vtbl, 0x0fb15084_af41_11ce_bd2b_204c4f4f5020);
windows_core::imp::interface_hierarchy!(ITransaction, windows_core::IUnknown);
impl ITransaction {
    pub unsafe fn Commit(&self, fretaining: bool, grftc: u32, grfrm: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Commit)(windows_core::Interface::as_raw(self), fretaining.into(), grftc, grfrm).ok() }
    }
    pub unsafe fn Abort(&self, pboidreason: *const BOID, fretaining: bool, fasync: bool) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Abort)(windows_core::Interface::as_raw(self), pboidreason, fretaining.into(), fasync.into()).ok() }
    }
    pub unsafe fn GetTransactionInfo(&self, pinfo: *mut XACTTRANSINFO) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetTransactionInfo)(windows_core::Interface::as_raw(self), pinfo as _).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ITransaction_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Commit: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::BOOL, u32, u32) -> windows_core::HRESULT,
    pub Abort: unsafe extern "system" fn(*mut core::ffi::c_void, *const BOID, windows_core::BOOL, windows_core::BOOL) -> windows_core::HRESULT,
    pub GetTransactionInfo: unsafe extern "system" fn(*mut core::ffi::c_void, *mut XACTTRANSINFO) -> windows_core::HRESULT,
}
pub trait ITransaction_Impl: windows_core::IUnknownImpl {
    fn Commit(&self, fretaining: windows_core::BOOL, grftc: u32, grfrm: u32) -> windows_core::Result<()>;
    fn Abort(&self, pboidreason: *const BOID, fretaining: windows_core::BOOL, fasync: windows_core::BOOL) -> windows_core::Result<()>;
    fn GetTransactionInfo(&self, pinfo: *mut XACTTRANSINFO) -> windows_core::Result<()>;
}
impl ITransaction_Vtbl {
    pub const fn new<Identity: ITransaction_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Commit<Identity: ITransaction_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, fretaining: windows_core::BOOL, grftc: u32, grfrm: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITransaction_Impl::Commit(this, core::mem::transmute_copy(&fretaining), core::mem::transmute_copy(&grftc), core::mem::transmute_copy(&grfrm)).into()
            }
        }
        unsafe extern "system" fn Abort<Identity: ITransaction_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pboidreason: *const BOID, fretaining: windows_core::BOOL, fasync: windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITransaction_Impl::Abort(this, core::mem::transmute_copy(&pboidreason), core::mem::transmute_copy(&fretaining), core::mem::transmute_copy(&fasync)).into()
            }
        }
        unsafe extern "system" fn GetTransactionInfo<Identity: ITransaction_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pinfo: *mut XACTTRANSINFO) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITransaction_Impl::GetTransactionInfo(this, core::mem::transmute_copy(&pinfo)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Commit: Commit::<Identity, OFFSET>,
            Abort: Abort::<Identity, OFFSET>,
            GetTransactionInfo: GetTransactionInfo::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITransaction as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for ITransaction {}
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
        unsafe { (windows_core::Interface::vtable(self).GetTransactionInfo2)(windows_core::Interface::as_raw(self), pinfo as _).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ITransaction2_Vtbl {
    pub base__: ITransactionCloner_Vtbl,
    pub GetTransactionInfo2: unsafe extern "system" fn(*mut core::ffi::c_void, *mut XACTTRANSINFO) -> windows_core::HRESULT,
}
pub trait ITransaction2_Impl: ITransactionCloner_Impl {
    fn GetTransactionInfo2(&self, pinfo: *mut XACTTRANSINFO) -> windows_core::Result<()>;
}
impl ITransaction2_Vtbl {
    pub const fn new<Identity: ITransaction2_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetTransactionInfo2<Identity: ITransaction2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pinfo: *mut XACTTRANSINFO) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITransaction2_Impl::GetTransactionInfo2(this, core::mem::transmute_copy(&pinfo)).into()
            }
        }
        Self { base__: ITransactionCloner_Vtbl::new::<Identity, OFFSET>(), GetTransactionInfo2: GetTransactionInfo2::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITransaction2 as windows_core::Interface>::IID || iid == &<ITransaction as windows_core::Interface>::IID || iid == &<ITransactionCloner as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for ITransaction2 {}
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
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CloneWithCommitDisabled)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ITransactionCloner_Vtbl {
    pub base__: ITransaction_Vtbl,
    pub CloneWithCommitDisabled: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait ITransactionCloner_Impl: ITransaction_Impl {
    fn CloneWithCommitDisabled(&self) -> windows_core::Result<ITransaction>;
}
impl ITransactionCloner_Vtbl {
    pub const fn new<Identity: ITransactionCloner_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn CloneWithCommitDisabled<Identity: ITransactionCloner_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppitransaction: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITransactionCloner_Impl::CloneWithCommitDisabled(this) {
                    Ok(ok__) => {
                        ppitransaction.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self { base__: ITransaction_Vtbl::new::<Identity, OFFSET>(), CloneWithCommitDisabled: CloneWithCommitDisabled::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITransactionCloner as windows_core::Interface>::IID || iid == &<ITransaction as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for ITransactionCloner {}
windows_core::imp::define_interface!(ITransactionDispenser, ITransactionDispenser_Vtbl, 0x3a6ad9e1_23b9_11cf_ad60_00aa00a74ccd);
windows_core::imp::interface_hierarchy!(ITransactionDispenser, windows_core::IUnknown);
impl ITransactionDispenser {
    pub unsafe fn GetOptionsObject(&self) -> windows_core::Result<ITransactionOptions> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetOptionsObject)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn BeginTransaction<P0, P3>(&self, punkouter: P0, isolevel: i32, isoflags: u32, poptions: P3) -> windows_core::Result<ITransaction>
    where
        P0: windows_core::Param<windows_core::IUnknown>,
        P3: windows_core::Param<ITransactionOptions>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).BeginTransaction)(windows_core::Interface::as_raw(self), punkouter.param().abi(), isolevel, isoflags, poptions.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ITransactionDispenser_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetOptionsObject: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub BeginTransaction: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, i32, u32, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait ITransactionDispenser_Impl: windows_core::IUnknownImpl {
    fn GetOptionsObject(&self) -> windows_core::Result<ITransactionOptions>;
    fn BeginTransaction(&self, punkouter: windows_core::Ref<windows_core::IUnknown>, isolevel: i32, isoflags: u32, poptions: windows_core::Ref<ITransactionOptions>) -> windows_core::Result<ITransaction>;
}
impl ITransactionDispenser_Vtbl {
    pub const fn new<Identity: ITransactionDispenser_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetOptionsObject<Identity: ITransactionDispenser_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppoptions: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITransactionDispenser_Impl::GetOptionsObject(this) {
                    Ok(ok__) => {
                        ppoptions.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn BeginTransaction<Identity: ITransactionDispenser_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, punkouter: *mut core::ffi::c_void, isolevel: i32, isoflags: u32, poptions: *mut core::ffi::c_void, pptransaction: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITransactionDispenser_Impl::BeginTransaction(this, core::mem::transmute_copy(&punkouter), core::mem::transmute_copy(&isolevel), core::mem::transmute_copy(&isoflags), core::mem::transmute_copy(&poptions)) {
                    Ok(ok__) => {
                        pptransaction.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetOptionsObject: GetOptionsObject::<Identity, OFFSET>,
            BeginTransaction: BeginTransaction::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITransactionDispenser as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for ITransactionDispenser {}
windows_core::imp::define_interface!(ITransactionEnlistmentAsync, ITransactionEnlistmentAsync_Vtbl, 0x0fb15081_af41_11ce_bd2b_204c4f4f5020);
windows_core::imp::interface_hierarchy!(ITransactionEnlistmentAsync, windows_core::IUnknown);
impl ITransactionEnlistmentAsync {
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn PrepareRequestDone<P1>(&self, hr: windows_core::HRESULT, pmk: P1, pboidreason: *const BOID) -> windows_core::Result<()>
    where
        P1: windows_core::Param<super::Com::IMoniker>,
    {
        unsafe { (windows_core::Interface::vtable(self).PrepareRequestDone)(windows_core::Interface::as_raw(self), hr, pmk.param().abi(), pboidreason).ok() }
    }
    pub unsafe fn CommitRequestDone(&self, hr: windows_core::HRESULT) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).CommitRequestDone)(windows_core::Interface::as_raw(self), hr).ok() }
    }
    pub unsafe fn AbortRequestDone(&self, hr: windows_core::HRESULT) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).AbortRequestDone)(windows_core::Interface::as_raw(self), hr).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ITransactionEnlistmentAsync_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub PrepareRequestDone: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::HRESULT, *mut core::ffi::c_void, *const BOID) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    PrepareRequestDone: usize,
    pub CommitRequestDone: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::HRESULT) -> windows_core::HRESULT,
    pub AbortRequestDone: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::HRESULT) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
pub trait ITransactionEnlistmentAsync_Impl: windows_core::IUnknownImpl {
    fn PrepareRequestDone(&self, hr: windows_core::HRESULT, pmk: windows_core::Ref<super::Com::IMoniker>, pboidreason: *const BOID) -> windows_core::Result<()>;
    fn CommitRequestDone(&self, hr: windows_core::HRESULT) -> windows_core::Result<()>;
    fn AbortRequestDone(&self, hr: windows_core::HRESULT) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl ITransactionEnlistmentAsync_Vtbl {
    pub const fn new<Identity: ITransactionEnlistmentAsync_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn PrepareRequestDone<Identity: ITransactionEnlistmentAsync_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hr: windows_core::HRESULT, pmk: *mut core::ffi::c_void, pboidreason: *const BOID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITransactionEnlistmentAsync_Impl::PrepareRequestDone(this, core::mem::transmute_copy(&hr), core::mem::transmute_copy(&pmk), core::mem::transmute_copy(&pboidreason)).into()
            }
        }
        unsafe extern "system" fn CommitRequestDone<Identity: ITransactionEnlistmentAsync_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hr: windows_core::HRESULT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITransactionEnlistmentAsync_Impl::CommitRequestDone(this, core::mem::transmute_copy(&hr)).into()
            }
        }
        unsafe extern "system" fn AbortRequestDone<Identity: ITransactionEnlistmentAsync_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hr: windows_core::HRESULT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITransactionEnlistmentAsync_Impl::AbortRequestDone(this, core::mem::transmute_copy(&hr)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            PrepareRequestDone: PrepareRequestDone::<Identity, OFFSET>,
            CommitRequestDone: CommitRequestDone::<Identity, OFFSET>,
            AbortRequestDone: AbortRequestDone::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITransactionEnlistmentAsync as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for ITransactionEnlistmentAsync {}
windows_core::imp::define_interface!(ITransactionExport, ITransactionExport_Vtbl, 0x0141fda5_8fc0_11ce_bd18_204c4f4f5020);
windows_core::imp::interface_hierarchy!(ITransactionExport, windows_core::IUnknown);
impl ITransactionExport {
    pub unsafe fn Export<P0>(&self, punktransaction: P0) -> windows_core::Result<u32>
    where
        P0: windows_core::Param<windows_core::IUnknown>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Export)(windows_core::Interface::as_raw(self), punktransaction.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetTransactionCookie<P0>(&self, punktransaction: P0, rgbtransactioncookie: &mut [u8], pcbused: *mut u32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::IUnknown>,
    {
        unsafe { (windows_core::Interface::vtable(self).GetTransactionCookie)(windows_core::Interface::as_raw(self), punktransaction.param().abi(), rgbtransactioncookie.len().try_into().unwrap(), core::mem::transmute(rgbtransactioncookie.as_ptr()), pcbused as _).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ITransactionExport_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Export: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub GetTransactionCookie: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, u32, *mut u8, *mut u32) -> windows_core::HRESULT,
}
pub trait ITransactionExport_Impl: windows_core::IUnknownImpl {
    fn Export(&self, punktransaction: windows_core::Ref<windows_core::IUnknown>) -> windows_core::Result<u32>;
    fn GetTransactionCookie(&self, punktransaction: windows_core::Ref<windows_core::IUnknown>, cbtransactioncookie: u32, rgbtransactioncookie: *mut u8, pcbused: *mut u32) -> windows_core::Result<()>;
}
impl ITransactionExport_Vtbl {
    pub const fn new<Identity: ITransactionExport_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Export<Identity: ITransactionExport_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, punktransaction: *mut core::ffi::c_void, pcbtransactioncookie: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITransactionExport_Impl::Export(this, core::mem::transmute_copy(&punktransaction)) {
                    Ok(ok__) => {
                        pcbtransactioncookie.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetTransactionCookie<Identity: ITransactionExport_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, punktransaction: *mut core::ffi::c_void, cbtransactioncookie: u32, rgbtransactioncookie: *mut u8, pcbused: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITransactionExport_Impl::GetTransactionCookie(this, core::mem::transmute_copy(&punktransaction), core::mem::transmute_copy(&cbtransactioncookie), core::mem::transmute_copy(&rgbtransactioncookie), core::mem::transmute_copy(&pcbused)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Export: Export::<Identity, OFFSET>,
            GetTransactionCookie: GetTransactionCookie::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITransactionExport as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for ITransactionExport {}
windows_core::imp::define_interface!(ITransactionExportFactory, ITransactionExportFactory_Vtbl, 0xe1cf9b53_8745_11ce_a9ba_00aa006c3706);
windows_core::imp::interface_hierarchy!(ITransactionExportFactory, windows_core::IUnknown);
impl ITransactionExportFactory {
    pub unsafe fn GetRemoteClassId(&self) -> windows_core::Result<windows_core::GUID> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetRemoteClassId)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn Create(&self, rgbwhereabouts: &[u8]) -> windows_core::Result<ITransactionExport> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Create)(windows_core::Interface::as_raw(self), rgbwhereabouts.len().try_into().unwrap(), core::mem::transmute(rgbwhereabouts.as_ptr()), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ITransactionExportFactory_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetRemoteClassId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::GUID) -> windows_core::HRESULT,
    pub Create: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const u8, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait ITransactionExportFactory_Impl: windows_core::IUnknownImpl {
    fn GetRemoteClassId(&self) -> windows_core::Result<windows_core::GUID>;
    fn Create(&self, cbwhereabouts: u32, rgbwhereabouts: *const u8) -> windows_core::Result<ITransactionExport>;
}
impl ITransactionExportFactory_Vtbl {
    pub const fn new<Identity: ITransactionExportFactory_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetRemoteClassId<Identity: ITransactionExportFactory_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pclsid: *mut windows_core::GUID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITransactionExportFactory_Impl::GetRemoteClassId(this) {
                    Ok(ok__) => {
                        pclsid.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Create<Identity: ITransactionExportFactory_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, cbwhereabouts: u32, rgbwhereabouts: *const u8, ppexport: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITransactionExportFactory_Impl::Create(this, core::mem::transmute_copy(&cbwhereabouts), core::mem::transmute_copy(&rgbwhereabouts)) {
                    Ok(ok__) => {
                        ppexport.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetRemoteClassId: GetRemoteClassId::<Identity, OFFSET>,
            Create: Create::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITransactionExportFactory as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for ITransactionExportFactory {}
windows_core::imp::define_interface!(ITransactionImport, ITransactionImport_Vtbl, 0xe1cf9b5a_8745_11ce_a9ba_00aa006c3706);
windows_core::imp::interface_hierarchy!(ITransactionImport, windows_core::IUnknown);
impl ITransactionImport {
    pub unsafe fn Import<T>(&self, rgbtransactioncookie: &[u8]) -> windows_core::Result<T>
    where
        T: windows_core::Interface,
    {
        let mut result__ = core::ptr::null_mut();
        unsafe { (windows_core::Interface::vtable(self).Import)(windows_core::Interface::as_raw(self), rgbtransactioncookie.len().try_into().unwrap(), core::mem::transmute(rgbtransactioncookie.as_ptr()), &T::IID, &mut result__).and_then(|| windows_core::Type::from_abi(result__)) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ITransactionImport_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Import: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const u8, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait ITransactionImport_Impl: windows_core::IUnknownImpl {
    fn Import(&self, cbtransactioncookie: u32, rgbtransactioncookie: *const u8, piid: *const windows_core::GUID, ppvtransaction: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
}
impl ITransactionImport_Vtbl {
    pub const fn new<Identity: ITransactionImport_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Import<Identity: ITransactionImport_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, cbtransactioncookie: u32, rgbtransactioncookie: *const u8, piid: *const windows_core::GUID, ppvtransaction: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITransactionImport_Impl::Import(this, core::mem::transmute_copy(&cbtransactioncookie), core::mem::transmute_copy(&rgbtransactioncookie), core::mem::transmute_copy(&piid), core::mem::transmute_copy(&ppvtransaction)).into()
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), Import: Import::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITransactionImport as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for ITransactionImport {}
windows_core::imp::define_interface!(ITransactionImportWhereabouts, ITransactionImportWhereabouts_Vtbl, 0x0141fda4_8fc0_11ce_bd18_204c4f4f5020);
windows_core::imp::interface_hierarchy!(ITransactionImportWhereabouts, windows_core::IUnknown);
impl ITransactionImportWhereabouts {
    pub unsafe fn GetWhereaboutsSize(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetWhereaboutsSize)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetWhereabouts(&self, rgbwhereabouts: &mut [u8], pcbused: *mut u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetWhereabouts)(windows_core::Interface::as_raw(self), rgbwhereabouts.len().try_into().unwrap(), core::mem::transmute(rgbwhereabouts.as_ptr()), pcbused as _).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ITransactionImportWhereabouts_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetWhereaboutsSize: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub GetWhereabouts: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut u8, *mut u32) -> windows_core::HRESULT,
}
pub trait ITransactionImportWhereabouts_Impl: windows_core::IUnknownImpl {
    fn GetWhereaboutsSize(&self) -> windows_core::Result<u32>;
    fn GetWhereabouts(&self, cbwhereabouts: u32, rgbwhereabouts: *mut u8, pcbused: *mut u32) -> windows_core::Result<()>;
}
impl ITransactionImportWhereabouts_Vtbl {
    pub const fn new<Identity: ITransactionImportWhereabouts_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetWhereaboutsSize<Identity: ITransactionImportWhereabouts_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcbwhereabouts: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITransactionImportWhereabouts_Impl::GetWhereaboutsSize(this) {
                    Ok(ok__) => {
                        pcbwhereabouts.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetWhereabouts<Identity: ITransactionImportWhereabouts_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, cbwhereabouts: u32, rgbwhereabouts: *mut u8, pcbused: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITransactionImportWhereabouts_Impl::GetWhereabouts(this, core::mem::transmute_copy(&cbwhereabouts), core::mem::transmute_copy(&rgbwhereabouts), core::mem::transmute_copy(&pcbused)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetWhereaboutsSize: GetWhereaboutsSize::<Identity, OFFSET>,
            GetWhereabouts: GetWhereabouts::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITransactionImportWhereabouts as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for ITransactionImportWhereabouts {}
windows_core::imp::define_interface!(ITransactionLastEnlistmentAsync, ITransactionLastEnlistmentAsync_Vtbl, 0xc82bd533_5b30_11d3_8a91_00c04f79eb6d);
windows_core::imp::interface_hierarchy!(ITransactionLastEnlistmentAsync, windows_core::IUnknown);
impl ITransactionLastEnlistmentAsync {
    pub unsafe fn TransactionOutcome(&self, xactstat: XACTSTAT, pboidreason: *const BOID) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).TransactionOutcome)(windows_core::Interface::as_raw(self), xactstat, pboidreason).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ITransactionLastEnlistmentAsync_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub TransactionOutcome: unsafe extern "system" fn(*mut core::ffi::c_void, XACTSTAT, *const BOID) -> windows_core::HRESULT,
}
pub trait ITransactionLastEnlistmentAsync_Impl: windows_core::IUnknownImpl {
    fn TransactionOutcome(&self, xactstat: XACTSTAT, pboidreason: *const BOID) -> windows_core::Result<()>;
}
impl ITransactionLastEnlistmentAsync_Vtbl {
    pub const fn new<Identity: ITransactionLastEnlistmentAsync_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn TransactionOutcome<Identity: ITransactionLastEnlistmentAsync_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, xactstat: XACTSTAT, pboidreason: *const BOID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITransactionLastEnlistmentAsync_Impl::TransactionOutcome(this, core::mem::transmute_copy(&xactstat), core::mem::transmute_copy(&pboidreason)).into()
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), TransactionOutcome: TransactionOutcome::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITransactionLastEnlistmentAsync as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for ITransactionLastEnlistmentAsync {}
windows_core::imp::define_interface!(ITransactionLastResourceAsync, ITransactionLastResourceAsync_Vtbl, 0xc82bd532_5b30_11d3_8a91_00c04f79eb6d);
windows_core::imp::interface_hierarchy!(ITransactionLastResourceAsync, windows_core::IUnknown);
impl ITransactionLastResourceAsync {
    pub unsafe fn DelegateCommit(&self, grfrm: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).DelegateCommit)(windows_core::Interface::as_raw(self), grfrm).ok() }
    }
    pub unsafe fn ForgetRequest(&self, pnewuow: *const BOID) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).ForgetRequest)(windows_core::Interface::as_raw(self), pnewuow).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ITransactionLastResourceAsync_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub DelegateCommit: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub ForgetRequest: unsafe extern "system" fn(*mut core::ffi::c_void, *const BOID) -> windows_core::HRESULT,
}
pub trait ITransactionLastResourceAsync_Impl: windows_core::IUnknownImpl {
    fn DelegateCommit(&self, grfrm: u32) -> windows_core::Result<()>;
    fn ForgetRequest(&self, pnewuow: *const BOID) -> windows_core::Result<()>;
}
impl ITransactionLastResourceAsync_Vtbl {
    pub const fn new<Identity: ITransactionLastResourceAsync_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn DelegateCommit<Identity: ITransactionLastResourceAsync_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, grfrm: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITransactionLastResourceAsync_Impl::DelegateCommit(this, core::mem::transmute_copy(&grfrm)).into()
            }
        }
        unsafe extern "system" fn ForgetRequest<Identity: ITransactionLastResourceAsync_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pnewuow: *const BOID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITransactionLastResourceAsync_Impl::ForgetRequest(this, core::mem::transmute_copy(&pnewuow)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            DelegateCommit: DelegateCommit::<Identity, OFFSET>,
            ForgetRequest: ForgetRequest::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITransactionLastResourceAsync as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for ITransactionLastResourceAsync {}
windows_core::imp::define_interface!(ITransactionOptions, ITransactionOptions_Vtbl, 0x3a6ad9e0_23b9_11cf_ad60_00aa00a74ccd);
windows_core::imp::interface_hierarchy!(ITransactionOptions, windows_core::IUnknown);
impl ITransactionOptions {
    pub unsafe fn SetOptions(&self, poptions: *const XACTOPT) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetOptions)(windows_core::Interface::as_raw(self), poptions).ok() }
    }
    pub unsafe fn GetOptions(&self, poptions: *mut XACTOPT) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetOptions)(windows_core::Interface::as_raw(self), poptions as _).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ITransactionOptions_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub SetOptions: unsafe extern "system" fn(*mut core::ffi::c_void, *const XACTOPT) -> windows_core::HRESULT,
    pub GetOptions: unsafe extern "system" fn(*mut core::ffi::c_void, *mut XACTOPT) -> windows_core::HRESULT,
}
pub trait ITransactionOptions_Impl: windows_core::IUnknownImpl {
    fn SetOptions(&self, poptions: *const XACTOPT) -> windows_core::Result<()>;
    fn GetOptions(&self, poptions: *mut XACTOPT) -> windows_core::Result<()>;
}
impl ITransactionOptions_Vtbl {
    pub const fn new<Identity: ITransactionOptions_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn SetOptions<Identity: ITransactionOptions_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, poptions: *const XACTOPT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITransactionOptions_Impl::SetOptions(this, core::mem::transmute_copy(&poptions)).into()
            }
        }
        unsafe extern "system" fn GetOptions<Identity: ITransactionOptions_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, poptions: *mut XACTOPT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITransactionOptions_Impl::GetOptions(this, core::mem::transmute_copy(&poptions)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            SetOptions: SetOptions::<Identity, OFFSET>,
            GetOptions: GetOptions::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITransactionOptions as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for ITransactionOptions {}
windows_core::imp::define_interface!(ITransactionOutcomeEvents, ITransactionOutcomeEvents_Vtbl, 0x3a6ad9e2_23b9_11cf_ad60_00aa00a74ccd);
windows_core::imp::interface_hierarchy!(ITransactionOutcomeEvents, windows_core::IUnknown);
impl ITransactionOutcomeEvents {
    pub unsafe fn Committed(&self, fretaining: bool, pnewuow: *const BOID, hr: windows_core::HRESULT) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Committed)(windows_core::Interface::as_raw(self), fretaining.into(), pnewuow, hr).ok() }
    }
    pub unsafe fn Aborted(&self, pboidreason: *const BOID, fretaining: bool, pnewuow: *const BOID, hr: windows_core::HRESULT) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Aborted)(windows_core::Interface::as_raw(self), pboidreason, fretaining.into(), pnewuow, hr).ok() }
    }
    pub unsafe fn HeuristicDecision(&self, dwdecision: u32, pboidreason: *const BOID, hr: windows_core::HRESULT) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).HeuristicDecision)(windows_core::Interface::as_raw(self), dwdecision, pboidreason, hr).ok() }
    }
    pub unsafe fn Indoubt(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Indoubt)(windows_core::Interface::as_raw(self)).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ITransactionOutcomeEvents_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Committed: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::BOOL, *const BOID, windows_core::HRESULT) -> windows_core::HRESULT,
    pub Aborted: unsafe extern "system" fn(*mut core::ffi::c_void, *const BOID, windows_core::BOOL, *const BOID, windows_core::HRESULT) -> windows_core::HRESULT,
    pub HeuristicDecision: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const BOID, windows_core::HRESULT) -> windows_core::HRESULT,
    pub Indoubt: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait ITransactionOutcomeEvents_Impl: windows_core::IUnknownImpl {
    fn Committed(&self, fretaining: windows_core::BOOL, pnewuow: *const BOID, hr: windows_core::HRESULT) -> windows_core::Result<()>;
    fn Aborted(&self, pboidreason: *const BOID, fretaining: windows_core::BOOL, pnewuow: *const BOID, hr: windows_core::HRESULT) -> windows_core::Result<()>;
    fn HeuristicDecision(&self, dwdecision: u32, pboidreason: *const BOID, hr: windows_core::HRESULT) -> windows_core::Result<()>;
    fn Indoubt(&self) -> windows_core::Result<()>;
}
impl ITransactionOutcomeEvents_Vtbl {
    pub const fn new<Identity: ITransactionOutcomeEvents_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Committed<Identity: ITransactionOutcomeEvents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, fretaining: windows_core::BOOL, pnewuow: *const BOID, hr: windows_core::HRESULT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITransactionOutcomeEvents_Impl::Committed(this, core::mem::transmute_copy(&fretaining), core::mem::transmute_copy(&pnewuow), core::mem::transmute_copy(&hr)).into()
            }
        }
        unsafe extern "system" fn Aborted<Identity: ITransactionOutcomeEvents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pboidreason: *const BOID, fretaining: windows_core::BOOL, pnewuow: *const BOID, hr: windows_core::HRESULT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITransactionOutcomeEvents_Impl::Aborted(this, core::mem::transmute_copy(&pboidreason), core::mem::transmute_copy(&fretaining), core::mem::transmute_copy(&pnewuow), core::mem::transmute_copy(&hr)).into()
            }
        }
        unsafe extern "system" fn HeuristicDecision<Identity: ITransactionOutcomeEvents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwdecision: u32, pboidreason: *const BOID, hr: windows_core::HRESULT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITransactionOutcomeEvents_Impl::HeuristicDecision(this, core::mem::transmute_copy(&dwdecision), core::mem::transmute_copy(&pboidreason), core::mem::transmute_copy(&hr)).into()
            }
        }
        unsafe extern "system" fn Indoubt<Identity: ITransactionOutcomeEvents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITransactionOutcomeEvents_Impl::Indoubt(this).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Committed: Committed::<Identity, OFFSET>,
            Aborted: Aborted::<Identity, OFFSET>,
            HeuristicDecision: HeuristicDecision::<Identity, OFFSET>,
            Indoubt: Indoubt::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITransactionOutcomeEvents as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for ITransactionOutcomeEvents {}
windows_core::imp::define_interface!(ITransactionPhase0EnlistmentAsync, ITransactionPhase0EnlistmentAsync_Vtbl, 0x82dc88e1_a954_11d1_8f88_00600895e7d5);
windows_core::imp::interface_hierarchy!(ITransactionPhase0EnlistmentAsync, windows_core::IUnknown);
impl ITransactionPhase0EnlistmentAsync {
    pub unsafe fn Enable(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Enable)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn WaitForEnlistment(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).WaitForEnlistment)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn Phase0Done(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Phase0Done)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn Unenlist(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Unenlist)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn GetTransaction(&self) -> windows_core::Result<ITransaction> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetTransaction)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ITransactionPhase0EnlistmentAsync_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Enable: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub WaitForEnlistment: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Phase0Done: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Unenlist: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetTransaction: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait ITransactionPhase0EnlistmentAsync_Impl: windows_core::IUnknownImpl {
    fn Enable(&self) -> windows_core::Result<()>;
    fn WaitForEnlistment(&self) -> windows_core::Result<()>;
    fn Phase0Done(&self) -> windows_core::Result<()>;
    fn Unenlist(&self) -> windows_core::Result<()>;
    fn GetTransaction(&self) -> windows_core::Result<ITransaction>;
}
impl ITransactionPhase0EnlistmentAsync_Vtbl {
    pub const fn new<Identity: ITransactionPhase0EnlistmentAsync_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Enable<Identity: ITransactionPhase0EnlistmentAsync_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITransactionPhase0EnlistmentAsync_Impl::Enable(this).into()
            }
        }
        unsafe extern "system" fn WaitForEnlistment<Identity: ITransactionPhase0EnlistmentAsync_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITransactionPhase0EnlistmentAsync_Impl::WaitForEnlistment(this).into()
            }
        }
        unsafe extern "system" fn Phase0Done<Identity: ITransactionPhase0EnlistmentAsync_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITransactionPhase0EnlistmentAsync_Impl::Phase0Done(this).into()
            }
        }
        unsafe extern "system" fn Unenlist<Identity: ITransactionPhase0EnlistmentAsync_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITransactionPhase0EnlistmentAsync_Impl::Unenlist(this).into()
            }
        }
        unsafe extern "system" fn GetTransaction<Identity: ITransactionPhase0EnlistmentAsync_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppitransaction: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITransactionPhase0EnlistmentAsync_Impl::GetTransaction(this) {
                    Ok(ok__) => {
                        ppitransaction.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Enable: Enable::<Identity, OFFSET>,
            WaitForEnlistment: WaitForEnlistment::<Identity, OFFSET>,
            Phase0Done: Phase0Done::<Identity, OFFSET>,
            Unenlist: Unenlist::<Identity, OFFSET>,
            GetTransaction: GetTransaction::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITransactionPhase0EnlistmentAsync as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for ITransactionPhase0EnlistmentAsync {}
windows_core::imp::define_interface!(ITransactionPhase0Factory, ITransactionPhase0Factory_Vtbl, 0x82dc88e0_a954_11d1_8f88_00600895e7d5);
windows_core::imp::interface_hierarchy!(ITransactionPhase0Factory, windows_core::IUnknown);
impl ITransactionPhase0Factory {
    pub unsafe fn Create<P0>(&self, pphase0notify: P0) -> windows_core::Result<ITransactionPhase0EnlistmentAsync>
    where
        P0: windows_core::Param<ITransactionPhase0NotifyAsync>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Create)(windows_core::Interface::as_raw(self), pphase0notify.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ITransactionPhase0Factory_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Create: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait ITransactionPhase0Factory_Impl: windows_core::IUnknownImpl {
    fn Create(&self, pphase0notify: windows_core::Ref<ITransactionPhase0NotifyAsync>) -> windows_core::Result<ITransactionPhase0EnlistmentAsync>;
}
impl ITransactionPhase0Factory_Vtbl {
    pub const fn new<Identity: ITransactionPhase0Factory_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Create<Identity: ITransactionPhase0Factory_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pphase0notify: *mut core::ffi::c_void, ppphase0enlistment: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITransactionPhase0Factory_Impl::Create(this, core::mem::transmute_copy(&pphase0notify)) {
                    Ok(ok__) => {
                        ppphase0enlistment.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), Create: Create::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITransactionPhase0Factory as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for ITransactionPhase0Factory {}
windows_core::imp::define_interface!(ITransactionPhase0NotifyAsync, ITransactionPhase0NotifyAsync_Vtbl, 0xef081809_0c76_11d2_87a6_00c04f990f34);
windows_core::imp::interface_hierarchy!(ITransactionPhase0NotifyAsync, windows_core::IUnknown);
impl ITransactionPhase0NotifyAsync {
    pub unsafe fn Phase0Request(&self, fabortinghint: bool) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Phase0Request)(windows_core::Interface::as_raw(self), fabortinghint.into()).ok() }
    }
    pub unsafe fn EnlistCompleted(&self, status: windows_core::HRESULT) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).EnlistCompleted)(windows_core::Interface::as_raw(self), status).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ITransactionPhase0NotifyAsync_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Phase0Request: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::BOOL) -> windows_core::HRESULT,
    pub EnlistCompleted: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::HRESULT) -> windows_core::HRESULT,
}
pub trait ITransactionPhase0NotifyAsync_Impl: windows_core::IUnknownImpl {
    fn Phase0Request(&self, fabortinghint: windows_core::BOOL) -> windows_core::Result<()>;
    fn EnlistCompleted(&self, status: windows_core::HRESULT) -> windows_core::Result<()>;
}
impl ITransactionPhase0NotifyAsync_Vtbl {
    pub const fn new<Identity: ITransactionPhase0NotifyAsync_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Phase0Request<Identity: ITransactionPhase0NotifyAsync_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, fabortinghint: windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITransactionPhase0NotifyAsync_Impl::Phase0Request(this, core::mem::transmute_copy(&fabortinghint)).into()
            }
        }
        unsafe extern "system" fn EnlistCompleted<Identity: ITransactionPhase0NotifyAsync_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, status: windows_core::HRESULT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITransactionPhase0NotifyAsync_Impl::EnlistCompleted(this, core::mem::transmute_copy(&status)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Phase0Request: Phase0Request::<Identity, OFFSET>,
            EnlistCompleted: EnlistCompleted::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITransactionPhase0NotifyAsync as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for ITransactionPhase0NotifyAsync {}
windows_core::imp::define_interface!(ITransactionReceiver, ITransactionReceiver_Vtbl, 0x59313e03_b36c_11cf_a539_00aa006887c3);
windows_core::imp::interface_hierarchy!(ITransactionReceiver, windows_core::IUnknown);
impl ITransactionReceiver {
    pub unsafe fn UnmarshalPropagationToken(&self, rgbtoken: &[u8]) -> windows_core::Result<ITransaction> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).UnmarshalPropagationToken)(windows_core::Interface::as_raw(self), rgbtoken.len().try_into().unwrap(), core::mem::transmute(rgbtoken.as_ptr()), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetReturnTokenSize(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetReturnTokenSize)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn MarshalReturnToken(&self, rgbreturntoken: &mut [u8], pcbused: *mut u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).MarshalReturnToken)(windows_core::Interface::as_raw(self), rgbreturntoken.len().try_into().unwrap(), core::mem::transmute(rgbreturntoken.as_ptr()), pcbused as _).ok() }
    }
    pub unsafe fn Reset(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Reset)(windows_core::Interface::as_raw(self)).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ITransactionReceiver_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub UnmarshalPropagationToken: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const u8, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetReturnTokenSize: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub MarshalReturnToken: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut u8, *mut u32) -> windows_core::HRESULT,
    pub Reset: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait ITransactionReceiver_Impl: windows_core::IUnknownImpl {
    fn UnmarshalPropagationToken(&self, cbtoken: u32, rgbtoken: *const u8) -> windows_core::Result<ITransaction>;
    fn GetReturnTokenSize(&self) -> windows_core::Result<u32>;
    fn MarshalReturnToken(&self, cbreturntoken: u32, rgbreturntoken: *mut u8, pcbused: *mut u32) -> windows_core::Result<()>;
    fn Reset(&self) -> windows_core::Result<()>;
}
impl ITransactionReceiver_Vtbl {
    pub const fn new<Identity: ITransactionReceiver_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn UnmarshalPropagationToken<Identity: ITransactionReceiver_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, cbtoken: u32, rgbtoken: *const u8, pptransaction: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITransactionReceiver_Impl::UnmarshalPropagationToken(this, core::mem::transmute_copy(&cbtoken), core::mem::transmute_copy(&rgbtoken)) {
                    Ok(ok__) => {
                        pptransaction.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetReturnTokenSize<Identity: ITransactionReceiver_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcbreturntoken: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITransactionReceiver_Impl::GetReturnTokenSize(this) {
                    Ok(ok__) => {
                        pcbreturntoken.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn MarshalReturnToken<Identity: ITransactionReceiver_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, cbreturntoken: u32, rgbreturntoken: *mut u8, pcbused: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITransactionReceiver_Impl::MarshalReturnToken(this, core::mem::transmute_copy(&cbreturntoken), core::mem::transmute_copy(&rgbreturntoken), core::mem::transmute_copy(&pcbused)).into()
            }
        }
        unsafe extern "system" fn Reset<Identity: ITransactionReceiver_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITransactionReceiver_Impl::Reset(this).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            UnmarshalPropagationToken: UnmarshalPropagationToken::<Identity, OFFSET>,
            GetReturnTokenSize: GetReturnTokenSize::<Identity, OFFSET>,
            MarshalReturnToken: MarshalReturnToken::<Identity, OFFSET>,
            Reset: Reset::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITransactionReceiver as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for ITransactionReceiver {}
windows_core::imp::define_interface!(ITransactionReceiverFactory, ITransactionReceiverFactory_Vtbl, 0x59313e02_b36c_11cf_a539_00aa006887c3);
windows_core::imp::interface_hierarchy!(ITransactionReceiverFactory, windows_core::IUnknown);
impl ITransactionReceiverFactory {
    pub unsafe fn Create(&self) -> windows_core::Result<ITransactionReceiver> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Create)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ITransactionReceiverFactory_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Create: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait ITransactionReceiverFactory_Impl: windows_core::IUnknownImpl {
    fn Create(&self) -> windows_core::Result<ITransactionReceiver>;
}
impl ITransactionReceiverFactory_Vtbl {
    pub const fn new<Identity: ITransactionReceiverFactory_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Create<Identity: ITransactionReceiverFactory_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppreceiver: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITransactionReceiverFactory_Impl::Create(this) {
                    Ok(ok__) => {
                        ppreceiver.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), Create: Create::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITransactionReceiverFactory as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for ITransactionReceiverFactory {}
windows_core::imp::define_interface!(ITransactionResource, ITransactionResource_Vtbl, 0xee5ff7b3_4572_11d0_9452_00a0c905416e);
windows_core::imp::interface_hierarchy!(ITransactionResource, windows_core::IUnknown);
impl ITransactionResource {
    pub unsafe fn PrepareRequest(&self, fretaining: bool, grfrm: u32, fwantmoniker: bool, fsinglephase: bool) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).PrepareRequest)(windows_core::Interface::as_raw(self), fretaining.into(), grfrm, fwantmoniker.into(), fsinglephase.into()).ok() }
    }
    pub unsafe fn CommitRequest(&self, grfrm: u32, pnewuow: *const BOID) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).CommitRequest)(windows_core::Interface::as_raw(self), grfrm, pnewuow).ok() }
    }
    pub unsafe fn AbortRequest(&self, pboidreason: *const BOID, fretaining: bool, pnewuow: *const BOID) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).AbortRequest)(windows_core::Interface::as_raw(self), pboidreason, fretaining.into(), pnewuow).ok() }
    }
    pub unsafe fn TMDown(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).TMDown)(windows_core::Interface::as_raw(self)).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ITransactionResource_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub PrepareRequest: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::BOOL, u32, windows_core::BOOL, windows_core::BOOL) -> windows_core::HRESULT,
    pub CommitRequest: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const BOID) -> windows_core::HRESULT,
    pub AbortRequest: unsafe extern "system" fn(*mut core::ffi::c_void, *const BOID, windows_core::BOOL, *const BOID) -> windows_core::HRESULT,
    pub TMDown: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait ITransactionResource_Impl: windows_core::IUnknownImpl {
    fn PrepareRequest(&self, fretaining: windows_core::BOOL, grfrm: u32, fwantmoniker: windows_core::BOOL, fsinglephase: windows_core::BOOL) -> windows_core::Result<()>;
    fn CommitRequest(&self, grfrm: u32, pnewuow: *const BOID) -> windows_core::Result<()>;
    fn AbortRequest(&self, pboidreason: *const BOID, fretaining: windows_core::BOOL, pnewuow: *const BOID) -> windows_core::Result<()>;
    fn TMDown(&self) -> windows_core::Result<()>;
}
impl ITransactionResource_Vtbl {
    pub const fn new<Identity: ITransactionResource_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn PrepareRequest<Identity: ITransactionResource_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, fretaining: windows_core::BOOL, grfrm: u32, fwantmoniker: windows_core::BOOL, fsinglephase: windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITransactionResource_Impl::PrepareRequest(this, core::mem::transmute_copy(&fretaining), core::mem::transmute_copy(&grfrm), core::mem::transmute_copy(&fwantmoniker), core::mem::transmute_copy(&fsinglephase)).into()
            }
        }
        unsafe extern "system" fn CommitRequest<Identity: ITransactionResource_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, grfrm: u32, pnewuow: *const BOID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITransactionResource_Impl::CommitRequest(this, core::mem::transmute_copy(&grfrm), core::mem::transmute_copy(&pnewuow)).into()
            }
        }
        unsafe extern "system" fn AbortRequest<Identity: ITransactionResource_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pboidreason: *const BOID, fretaining: windows_core::BOOL, pnewuow: *const BOID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITransactionResource_Impl::AbortRequest(this, core::mem::transmute_copy(&pboidreason), core::mem::transmute_copy(&fretaining), core::mem::transmute_copy(&pnewuow)).into()
            }
        }
        unsafe extern "system" fn TMDown<Identity: ITransactionResource_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITransactionResource_Impl::TMDown(this).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            PrepareRequest: PrepareRequest::<Identity, OFFSET>,
            CommitRequest: CommitRequest::<Identity, OFFSET>,
            AbortRequest: AbortRequest::<Identity, OFFSET>,
            TMDown: TMDown::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITransactionResource as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for ITransactionResource {}
windows_core::imp::define_interface!(ITransactionResourceAsync, ITransactionResourceAsync_Vtbl, 0x69e971f0_23ce_11cf_ad60_00aa00a74ccd);
windows_core::imp::interface_hierarchy!(ITransactionResourceAsync, windows_core::IUnknown);
impl ITransactionResourceAsync {
    pub unsafe fn PrepareRequest(&self, fretaining: bool, grfrm: u32, fwantmoniker: bool, fsinglephase: bool) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).PrepareRequest)(windows_core::Interface::as_raw(self), fretaining.into(), grfrm, fwantmoniker.into(), fsinglephase.into()).ok() }
    }
    pub unsafe fn CommitRequest(&self, grfrm: u32, pnewuow: *const BOID) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).CommitRequest)(windows_core::Interface::as_raw(self), grfrm, pnewuow).ok() }
    }
    pub unsafe fn AbortRequest(&self, pboidreason: *const BOID, fretaining: bool, pnewuow: *const BOID) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).AbortRequest)(windows_core::Interface::as_raw(self), pboidreason, fretaining.into(), pnewuow).ok() }
    }
    pub unsafe fn TMDown(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).TMDown)(windows_core::Interface::as_raw(self)).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ITransactionResourceAsync_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub PrepareRequest: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::BOOL, u32, windows_core::BOOL, windows_core::BOOL) -> windows_core::HRESULT,
    pub CommitRequest: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const BOID) -> windows_core::HRESULT,
    pub AbortRequest: unsafe extern "system" fn(*mut core::ffi::c_void, *const BOID, windows_core::BOOL, *const BOID) -> windows_core::HRESULT,
    pub TMDown: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait ITransactionResourceAsync_Impl: windows_core::IUnknownImpl {
    fn PrepareRequest(&self, fretaining: windows_core::BOOL, grfrm: u32, fwantmoniker: windows_core::BOOL, fsinglephase: windows_core::BOOL) -> windows_core::Result<()>;
    fn CommitRequest(&self, grfrm: u32, pnewuow: *const BOID) -> windows_core::Result<()>;
    fn AbortRequest(&self, pboidreason: *const BOID, fretaining: windows_core::BOOL, pnewuow: *const BOID) -> windows_core::Result<()>;
    fn TMDown(&self) -> windows_core::Result<()>;
}
impl ITransactionResourceAsync_Vtbl {
    pub const fn new<Identity: ITransactionResourceAsync_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn PrepareRequest<Identity: ITransactionResourceAsync_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, fretaining: windows_core::BOOL, grfrm: u32, fwantmoniker: windows_core::BOOL, fsinglephase: windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITransactionResourceAsync_Impl::PrepareRequest(this, core::mem::transmute_copy(&fretaining), core::mem::transmute_copy(&grfrm), core::mem::transmute_copy(&fwantmoniker), core::mem::transmute_copy(&fsinglephase)).into()
            }
        }
        unsafe extern "system" fn CommitRequest<Identity: ITransactionResourceAsync_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, grfrm: u32, pnewuow: *const BOID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITransactionResourceAsync_Impl::CommitRequest(this, core::mem::transmute_copy(&grfrm), core::mem::transmute_copy(&pnewuow)).into()
            }
        }
        unsafe extern "system" fn AbortRequest<Identity: ITransactionResourceAsync_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pboidreason: *const BOID, fretaining: windows_core::BOOL, pnewuow: *const BOID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITransactionResourceAsync_Impl::AbortRequest(this, core::mem::transmute_copy(&pboidreason), core::mem::transmute_copy(&fretaining), core::mem::transmute_copy(&pnewuow)).into()
            }
        }
        unsafe extern "system" fn TMDown<Identity: ITransactionResourceAsync_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITransactionResourceAsync_Impl::TMDown(this).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            PrepareRequest: PrepareRequest::<Identity, OFFSET>,
            CommitRequest: CommitRequest::<Identity, OFFSET>,
            AbortRequest: AbortRequest::<Identity, OFFSET>,
            TMDown: TMDown::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITransactionResourceAsync as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for ITransactionResourceAsync {}
windows_core::imp::define_interface!(ITransactionTransmitter, ITransactionTransmitter_Vtbl, 0x59313e01_b36c_11cf_a539_00aa006887c3);
windows_core::imp::interface_hierarchy!(ITransactionTransmitter, windows_core::IUnknown);
impl ITransactionTransmitter {
    pub unsafe fn Set<P0>(&self, ptransaction: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<ITransaction>,
    {
        unsafe { (windows_core::Interface::vtable(self).Set)(windows_core::Interface::as_raw(self), ptransaction.param().abi()).ok() }
    }
    pub unsafe fn GetPropagationTokenSize(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetPropagationTokenSize)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn MarshalPropagationToken(&self, rgbtoken: &mut [u8], pcbused: *mut u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).MarshalPropagationToken)(windows_core::Interface::as_raw(self), rgbtoken.len().try_into().unwrap(), core::mem::transmute(rgbtoken.as_ptr()), pcbused as _).ok() }
    }
    pub unsafe fn UnmarshalReturnToken(&self, rgbreturntoken: &[u8]) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).UnmarshalReturnToken)(windows_core::Interface::as_raw(self), rgbreturntoken.len().try_into().unwrap(), core::mem::transmute(rgbreturntoken.as_ptr())).ok() }
    }
    pub unsafe fn Reset(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Reset)(windows_core::Interface::as_raw(self)).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ITransactionTransmitter_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Set: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetPropagationTokenSize: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub MarshalPropagationToken: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut u8, *mut u32) -> windows_core::HRESULT,
    pub UnmarshalReturnToken: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const u8) -> windows_core::HRESULT,
    pub Reset: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait ITransactionTransmitter_Impl: windows_core::IUnknownImpl {
    fn Set(&self, ptransaction: windows_core::Ref<ITransaction>) -> windows_core::Result<()>;
    fn GetPropagationTokenSize(&self) -> windows_core::Result<u32>;
    fn MarshalPropagationToken(&self, cbtoken: u32, rgbtoken: *mut u8, pcbused: *mut u32) -> windows_core::Result<()>;
    fn UnmarshalReturnToken(&self, cbreturntoken: u32, rgbreturntoken: *const u8) -> windows_core::Result<()>;
    fn Reset(&self) -> windows_core::Result<()>;
}
impl ITransactionTransmitter_Vtbl {
    pub const fn new<Identity: ITransactionTransmitter_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Set<Identity: ITransactionTransmitter_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ptransaction: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITransactionTransmitter_Impl::Set(this, core::mem::transmute_copy(&ptransaction)).into()
            }
        }
        unsafe extern "system" fn GetPropagationTokenSize<Identity: ITransactionTransmitter_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcbtoken: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITransactionTransmitter_Impl::GetPropagationTokenSize(this) {
                    Ok(ok__) => {
                        pcbtoken.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn MarshalPropagationToken<Identity: ITransactionTransmitter_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, cbtoken: u32, rgbtoken: *mut u8, pcbused: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITransactionTransmitter_Impl::MarshalPropagationToken(this, core::mem::transmute_copy(&cbtoken), core::mem::transmute_copy(&rgbtoken), core::mem::transmute_copy(&pcbused)).into()
            }
        }
        unsafe extern "system" fn UnmarshalReturnToken<Identity: ITransactionTransmitter_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, cbreturntoken: u32, rgbreturntoken: *const u8) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITransactionTransmitter_Impl::UnmarshalReturnToken(this, core::mem::transmute_copy(&cbreturntoken), core::mem::transmute_copy(&rgbreturntoken)).into()
            }
        }
        unsafe extern "system" fn Reset<Identity: ITransactionTransmitter_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITransactionTransmitter_Impl::Reset(this).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Set: Set::<Identity, OFFSET>,
            GetPropagationTokenSize: GetPropagationTokenSize::<Identity, OFFSET>,
            MarshalPropagationToken: MarshalPropagationToken::<Identity, OFFSET>,
            UnmarshalReturnToken: UnmarshalReturnToken::<Identity, OFFSET>,
            Reset: Reset::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITransactionTransmitter as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for ITransactionTransmitter {}
windows_core::imp::define_interface!(ITransactionTransmitterFactory, ITransactionTransmitterFactory_Vtbl, 0x59313e00_b36c_11cf_a539_00aa006887c3);
windows_core::imp::interface_hierarchy!(ITransactionTransmitterFactory, windows_core::IUnknown);
impl ITransactionTransmitterFactory {
    pub unsafe fn Create(&self) -> windows_core::Result<ITransactionTransmitter> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Create)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ITransactionTransmitterFactory_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Create: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait ITransactionTransmitterFactory_Impl: windows_core::IUnknownImpl {
    fn Create(&self) -> windows_core::Result<ITransactionTransmitter>;
}
impl ITransactionTransmitterFactory_Vtbl {
    pub const fn new<Identity: ITransactionTransmitterFactory_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Create<Identity: ITransactionTransmitterFactory_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pptransmitter: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITransactionTransmitterFactory_Impl::Create(this) {
                    Ok(ok__) => {
                        pptransmitter.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), Create: Create::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITransactionTransmitterFactory as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for ITransactionTransmitterFactory {}
windows_core::imp::define_interface!(ITransactionVoterBallotAsync2, ITransactionVoterBallotAsync2_Vtbl, 0x5433376c_414d_11d3_b206_00c04fc2f3ef);
windows_core::imp::interface_hierarchy!(ITransactionVoterBallotAsync2, windows_core::IUnknown);
impl ITransactionVoterBallotAsync2 {
    pub unsafe fn VoteRequestDone(&self, hr: windows_core::HRESULT, pboidreason: Option<*const BOID>) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).VoteRequestDone)(windows_core::Interface::as_raw(self), hr, pboidreason.unwrap_or(core::mem::zeroed()) as _).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ITransactionVoterBallotAsync2_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub VoteRequestDone: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::HRESULT, *const BOID) -> windows_core::HRESULT,
}
pub trait ITransactionVoterBallotAsync2_Impl: windows_core::IUnknownImpl {
    fn VoteRequestDone(&self, hr: windows_core::HRESULT, pboidreason: *const BOID) -> windows_core::Result<()>;
}
impl ITransactionVoterBallotAsync2_Vtbl {
    pub const fn new<Identity: ITransactionVoterBallotAsync2_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn VoteRequestDone<Identity: ITransactionVoterBallotAsync2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hr: windows_core::HRESULT, pboidreason: *const BOID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITransactionVoterBallotAsync2_Impl::VoteRequestDone(this, core::mem::transmute_copy(&hr), core::mem::transmute_copy(&pboidreason)).into()
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), VoteRequestDone: VoteRequestDone::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITransactionVoterBallotAsync2 as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for ITransactionVoterBallotAsync2 {}
windows_core::imp::define_interface!(ITransactionVoterFactory2, ITransactionVoterFactory2_Vtbl, 0x5433376a_414d_11d3_b206_00c04fc2f3ef);
windows_core::imp::interface_hierarchy!(ITransactionVoterFactory2, windows_core::IUnknown);
impl ITransactionVoterFactory2 {
    pub unsafe fn Create<P0, P1>(&self, ptransaction: P0, pvoternotify: P1) -> windows_core::Result<ITransactionVoterBallotAsync2>
    where
        P0: windows_core::Param<ITransaction>,
        P1: windows_core::Param<ITransactionVoterNotifyAsync2>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Create)(windows_core::Interface::as_raw(self), ptransaction.param().abi(), pvoternotify.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ITransactionVoterFactory2_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Create: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait ITransactionVoterFactory2_Impl: windows_core::IUnknownImpl {
    fn Create(&self, ptransaction: windows_core::Ref<ITransaction>, pvoternotify: windows_core::Ref<ITransactionVoterNotifyAsync2>) -> windows_core::Result<ITransactionVoterBallotAsync2>;
}
impl ITransactionVoterFactory2_Vtbl {
    pub const fn new<Identity: ITransactionVoterFactory2_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Create<Identity: ITransactionVoterFactory2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ptransaction: *mut core::ffi::c_void, pvoternotify: *mut core::ffi::c_void, ppvoterballot: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITransactionVoterFactory2_Impl::Create(this, core::mem::transmute_copy(&ptransaction), core::mem::transmute_copy(&pvoternotify)) {
                    Ok(ok__) => {
                        ppvoterballot.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), Create: Create::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITransactionVoterFactory2 as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for ITransactionVoterFactory2 {}
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
        unsafe { (windows_core::Interface::vtable(self).VoteRequest)(windows_core::Interface::as_raw(self)).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ITransactionVoterNotifyAsync2_Vtbl {
    pub base__: ITransactionOutcomeEvents_Vtbl,
    pub VoteRequest: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait ITransactionVoterNotifyAsync2_Impl: ITransactionOutcomeEvents_Impl {
    fn VoteRequest(&self) -> windows_core::Result<()>;
}
impl ITransactionVoterNotifyAsync2_Vtbl {
    pub const fn new<Identity: ITransactionVoterNotifyAsync2_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn VoteRequest<Identity: ITransactionVoterNotifyAsync2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITransactionVoterNotifyAsync2_Impl::VoteRequest(this).into()
            }
        }
        Self { base__: ITransactionOutcomeEvents_Vtbl::new::<Identity, OFFSET>(), VoteRequest: VoteRequest::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITransactionVoterNotifyAsync2 as windows_core::Interface>::IID || iid == &<ITransactionOutcomeEvents as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for ITransactionVoterNotifyAsync2 {}
windows_core::imp::define_interface!(IXAConfig, IXAConfig_Vtbl, 0xc8a6e3a1_9a8c_11cf_a308_00a0c905416e);
windows_core::imp::interface_hierarchy!(IXAConfig, windows_core::IUnknown);
impl IXAConfig {
    pub unsafe fn Initialize(&self, clsidhelperdll: windows_core::GUID) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Initialize)(windows_core::Interface::as_raw(self), core::mem::transmute(clsidhelperdll)).ok() }
    }
    pub unsafe fn Terminate(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Terminate)(windows_core::Interface::as_raw(self)).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IXAConfig_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Initialize: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::GUID) -> windows_core::HRESULT,
    pub Terminate: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IXAConfig_Impl: windows_core::IUnknownImpl {
    fn Initialize(&self, clsidhelperdll: &windows_core::GUID) -> windows_core::Result<()>;
    fn Terminate(&self) -> windows_core::Result<()>;
}
impl IXAConfig_Vtbl {
    pub const fn new<Identity: IXAConfig_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Initialize<Identity: IXAConfig_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, clsidhelperdll: windows_core::GUID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IXAConfig_Impl::Initialize(this, core::mem::transmute(&clsidhelperdll)).into()
            }
        }
        unsafe extern "system" fn Terminate<Identity: IXAConfig_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IXAConfig_Impl::Terminate(this).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Initialize: Initialize::<Identity, OFFSET>,
            Terminate: Terminate::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IXAConfig as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IXAConfig {}
windows_core::imp::define_interface!(IXAObtainRMInfo, IXAObtainRMInfo_Vtbl, 0xe793f6d2_f53d_11cf_a60d_00a0c905416e);
windows_core::imp::interface_hierarchy!(IXAObtainRMInfo, windows_core::IUnknown);
impl IXAObtainRMInfo {
    pub unsafe fn ObtainRMInfo<P0>(&self, pirmhelper: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IRMHelper>,
    {
        unsafe { (windows_core::Interface::vtable(self).ObtainRMInfo)(windows_core::Interface::as_raw(self), pirmhelper.param().abi()).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IXAObtainRMInfo_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub ObtainRMInfo: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IXAObtainRMInfo_Impl: windows_core::IUnknownImpl {
    fn ObtainRMInfo(&self, pirmhelper: windows_core::Ref<IRMHelper>) -> windows_core::Result<()>;
}
impl IXAObtainRMInfo_Vtbl {
    pub const fn new<Identity: IXAObtainRMInfo_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn ObtainRMInfo<Identity: IXAObtainRMInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pirmhelper: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IXAObtainRMInfo_Impl::ObtainRMInfo(this, core::mem::transmute_copy(&pirmhelper)).into()
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), ObtainRMInfo: ObtainRMInfo::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IXAObtainRMInfo as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IXAObtainRMInfo {}
windows_core::imp::define_interface!(IXATransLookup, IXATransLookup_Vtbl, 0xf3b1f131_eeda_11ce_aed4_00aa0051e2c4);
windows_core::imp::interface_hierarchy!(IXATransLookup, windows_core::IUnknown);
impl IXATransLookup {
    pub unsafe fn Lookup(&self) -> windows_core::Result<ITransaction> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Lookup)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IXATransLookup_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Lookup: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IXATransLookup_Impl: windows_core::IUnknownImpl {
    fn Lookup(&self) -> windows_core::Result<ITransaction>;
}
impl IXATransLookup_Vtbl {
    pub const fn new<Identity: IXATransLookup_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Lookup<Identity: IXATransLookup_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pptransaction: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IXATransLookup_Impl::Lookup(this) {
                    Ok(ok__) => {
                        pptransaction.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), Lookup: Lookup::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IXATransLookup as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IXATransLookup {}
windows_core::imp::define_interface!(IXATransLookup2, IXATransLookup2_Vtbl, 0xbf193c85_0d1a_4290_b88f_d2cb8873d1e7);
windows_core::imp::interface_hierarchy!(IXATransLookup2, windows_core::IUnknown);
impl IXATransLookup2 {
    pub unsafe fn Lookup(&self, pxid: *const XID) -> windows_core::Result<ITransaction> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Lookup)(windows_core::Interface::as_raw(self), pxid, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IXATransLookup2_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Lookup: unsafe extern "system" fn(*mut core::ffi::c_void, *const XID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IXATransLookup2_Impl: windows_core::IUnknownImpl {
    fn Lookup(&self, pxid: *const XID) -> windows_core::Result<ITransaction>;
}
impl IXATransLookup2_Vtbl {
    pub const fn new<Identity: IXATransLookup2_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Lookup<Identity: IXATransLookup2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pxid: *const XID, pptransaction: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IXATransLookup2_Impl::Lookup(this, core::mem::transmute_copy(&pxid)) {
                    Ok(ok__) => {
                        pptransaction.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), Lookup: Lookup::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IXATransLookup2 as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IXATransLookup2 {}
pub const LOCAL_APPLICATIONTYPE: APPLICATIONTYPE = APPLICATIONTYPE(0i32);
pub const MAXBQUALSIZE: u32 = 64u32;
pub const MAXGTRIDSIZE: u32 = 64u32;
pub const MAXINFOSIZE: u32 = 256u32;
pub const MAX_TRAN_DESC: TX_MISC_CONSTANTS = TX_MISC_CONSTANTS(40i32);
pub const MUTUAL_AUTHENTICATION_REQUIRED: AUTHENTICATION_LEVEL = AUTHENTICATION_LEVEL(2i32);
pub const NO_AUTHENTICATION_REQUIRED: AUTHENTICATION_LEVEL = AUTHENTICATION_LEVEL(0i32);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct OLE_TM_CONFIG_PARAMS_V1 {
    pub dwVersion: u32,
    pub dwcConcurrencyHint: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct OLE_TM_CONFIG_PARAMS_V2 {
    pub dwVersion: u32,
    pub dwcConcurrencyHint: u32,
    pub applicationType: APPLICATIONTYPE,
    pub clusterResourceId: windows_core::GUID,
}
pub const OLE_TM_CONFIG_VERSION_1: u32 = 1u32;
pub const OLE_TM_CONFIG_VERSION_2: u32 = 2u32;
pub const OLE_TM_FLAG_INTERNAL_TO_TM: u32 = 1073741824u32;
pub const OLE_TM_FLAG_NOAGILERECOVERY: u32 = 2u32;
pub const OLE_TM_FLAG_NODEMANDSTART: u32 = 1u32;
pub const OLE_TM_FLAG_NONE: u32 = 0u32;
pub const OLE_TM_FLAG_QUERY_SERVICE_LOCKSTATUS: u32 = 2147483648u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct PROXY_CONFIG_PARAMS {
    pub wcThreadsMax: u16,
}
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
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct TX_MISC_CONSTANTS(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct XACTCONST(pub i32);
pub const XACTCONST_TIMEOUTINFINITE: XACTCONST = XACTCONST(0i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct XACTHEURISTIC(pub i32);
pub const XACTHEURISTIC_ABORT: XACTHEURISTIC = XACTHEURISTIC(1i32);
pub const XACTHEURISTIC_COMMIT: XACTHEURISTIC = XACTHEURISTIC(2i32);
pub const XACTHEURISTIC_DAMAGE: XACTHEURISTIC = XACTHEURISTIC(3i32);
pub const XACTHEURISTIC_DANGER: XACTHEURISTIC = XACTHEURISTIC(4i32);
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct XACTOPT {
    pub ulTimeout: u32,
    pub szDescription: [u8; 40],
}
impl Default for XACTOPT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct XACTRM(pub i32);
pub const XACTRM_NOREADONLYPREPARES: XACTRM = XACTRM(2i32);
pub const XACTRM_OPTIMISTICLASTWINS: XACTRM = XACTRM(1i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct XACTSTAT(pub i32);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
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
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct XACTTC(pub i32);
pub const XACTTC_ASYNC: XACTTC = XACTTC(4i32);
pub const XACTTC_ASYNC_PHASEONE: XACTTC = XACTTC(4i32);
pub const XACTTC_NONE: XACTTC = XACTTC(0i32);
pub const XACTTC_SYNC: XACTTC = XACTTC(2i32);
pub const XACTTC_SYNC_PHASEONE: XACTTC = XACTTC(1i32);
pub const XACTTC_SYNC_PHASETWO: XACTTC = XACTTC(2i32);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct XACTTRANSINFO {
    pub uow: BOID,
    pub isoLevel: i32,
    pub isoFlags: u32,
    pub grfTCSupported: u32,
    pub grfRMSupported: u32,
    pub grfTCSupportedRetaining: u32,
    pub grfRMSupportedRetaining: u32,
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct XACT_DTC_CONSTANTS(pub i32);
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
pub type XA_CLOSE_EPT = Option<unsafe extern "system" fn(param0: windows_core::PCSTR, param1: i32, param2: i32) -> i32>;
pub type XA_COMMIT_EPT = Option<unsafe extern "system" fn(param0: *mut XID, param1: i32, param2: i32) -> i32>;
pub type XA_COMPLETE_EPT = Option<unsafe extern "system" fn(param0: *mut i32, param1: *mut i32, param2: i32, param3: i32) -> i32>;
pub type XA_END_EPT = Option<unsafe extern "system" fn(param0: *mut XID, param1: i32, param2: i32) -> i32>;
pub const XA_FMTID_DTC: u32 = 4478019u32;
pub const XA_FMTID_DTC_VER1: u32 = 21255235u32;
pub type XA_FORGET_EPT = Option<unsafe extern "system" fn(param0: *mut XID, param1: i32, param2: i32) -> i32>;
pub const XA_HEURCOM: u32 = 7u32;
pub const XA_HEURHAZ: u32 = 8u32;
pub const XA_HEURMIX: u32 = 5u32;
pub const XA_HEURRB: u32 = 6u32;
pub const XA_NOMIGRATE: u32 = 9u32;
pub const XA_OK: u32 = 0u32;
pub type XA_OPEN_EPT = Option<unsafe extern "system" fn(param0: windows_core::PCSTR, param1: i32, param2: i32) -> i32>;
pub type XA_PREPARE_EPT = Option<unsafe extern "system" fn(param0: *mut XID, param1: i32, param2: i32) -> i32>;
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
pub type XA_RECOVER_EPT = Option<unsafe extern "system" fn(param0: *mut XID, param1: i32, param2: i32, param3: i32) -> i32>;
pub const XA_RETRY: u32 = 4u32;
pub type XA_ROLLBACK_EPT = Option<unsafe extern "system" fn(param0: *mut XID, param1: i32, param2: i32) -> i32>;
pub type XA_START_EPT = Option<unsafe extern "system" fn(param0: *mut XID, param1: i32, param2: i32) -> i32>;
pub const XA_SWITCH_F_DTC: u32 = 1u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct XID {
    pub formatID: i32,
    pub gtrid_length: i32,
    pub bqual_length: i32,
    pub data: [i8; 128],
}
impl Default for XID {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const XIDDATASIZE: u32 = 128u32;
pub const dwUSER_MS_SQLSERVER: XACT_DTC_CONSTANTS = XACT_DTC_CONSTANTS(65535i32);
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
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
impl Default for xa_switch_t {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
