windows_core::imp::define_interface!(ITpmVirtualSmartCardManager, ITpmVirtualSmartCardManager_Vtbl, 0x112b1dff_d9dc_41f7_869f_d67fee7cb591);
windows_core::imp::interface_hierarchy!(ITpmVirtualSmartCardManager, windows_core::IUnknown);
impl ITpmVirtualSmartCardManager {
    pub unsafe fn CreateVirtualSmartCard<P0, P11>(&self, pszfriendlyname: P0, badminalgid: u8, pbadminkey: &[u8], pbadminkcv: &[u8], pbpuk: &[u8], pbpin: &[u8], fgenerate: bool, pstatuscallback: P11, ppszinstanceid: *mut windows_core::PWSTR, pfneedreboot: *mut windows_core::BOOL) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        P11: windows_core::Param<ITpmVirtualSmartCardManagerStatusCallback>,
    {
        unsafe { (windows_core::Interface::vtable(self).CreateVirtualSmartCard)(windows_core::Interface::as_raw(self), pszfriendlyname.param().abi(), badminalgid, core::mem::transmute(pbadminkey.as_ptr()), pbadminkey.len().try_into().unwrap(), core::mem::transmute(pbadminkcv.as_ptr()), pbadminkcv.len().try_into().unwrap(), core::mem::transmute(pbpuk.as_ptr()), pbpuk.len().try_into().unwrap(), core::mem::transmute(pbpin.as_ptr()), pbpin.len().try_into().unwrap(), fgenerate.into(), pstatuscallback.param().abi(), ppszinstanceid as _, pfneedreboot as _).ok() }
    }
    pub unsafe fn DestroyVirtualSmartCard<P0, P1>(&self, pszinstanceid: P0, pstatuscallback: P1) -> windows_core::Result<windows_core::BOOL>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        P1: windows_core::Param<ITpmVirtualSmartCardManagerStatusCallback>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).DestroyVirtualSmartCard)(windows_core::Interface::as_raw(self), pszinstanceid.param().abi(), pstatuscallback.param().abi(), &mut result__).map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ITpmVirtualSmartCardManager_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub CreateVirtualSmartCard: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, u8, *const u8, u32, *const u8, u32, *const u8, u32, *const u8, u32, windows_core::BOOL, *mut core::ffi::c_void, *mut windows_core::PWSTR, *mut windows_core::BOOL) -> windows_core::HRESULT,
    pub DestroyVirtualSmartCard: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, *mut core::ffi::c_void, *mut windows_core::BOOL) -> windows_core::HRESULT,
}
pub trait ITpmVirtualSmartCardManager_Impl: windows_core::IUnknownImpl {
    fn CreateVirtualSmartCard(&self, pszfriendlyname: &windows_core::PCWSTR, badminalgid: u8, pbadminkey: *const u8, cbadminkey: u32, pbadminkcv: *const u8, cbadminkcv: u32, pbpuk: *const u8, cbpuk: u32, pbpin: *const u8, cbpin: u32, fgenerate: windows_core::BOOL, pstatuscallback: windows_core::Ref<ITpmVirtualSmartCardManagerStatusCallback>, ppszinstanceid: *mut windows_core::PWSTR, pfneedreboot: *mut windows_core::BOOL) -> windows_core::Result<()>;
    fn DestroyVirtualSmartCard(&self, pszinstanceid: &windows_core::PCWSTR, pstatuscallback: windows_core::Ref<ITpmVirtualSmartCardManagerStatusCallback>) -> windows_core::Result<windows_core::BOOL>;
}
impl ITpmVirtualSmartCardManager_Vtbl {
    pub const fn new<Identity: ITpmVirtualSmartCardManager_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn CreateVirtualSmartCard<Identity: ITpmVirtualSmartCardManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszfriendlyname: windows_core::PCWSTR, badminalgid: u8, pbadminkey: *const u8, cbadminkey: u32, pbadminkcv: *const u8, cbadminkcv: u32, pbpuk: *const u8, cbpuk: u32, pbpin: *const u8, cbpin: u32, fgenerate: windows_core::BOOL, pstatuscallback: *mut core::ffi::c_void, ppszinstanceid: *mut windows_core::PWSTR, pfneedreboot: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITpmVirtualSmartCardManager_Impl::CreateVirtualSmartCard(
                    this,
                    core::mem::transmute(&pszfriendlyname),
                    core::mem::transmute_copy(&badminalgid),
                    core::mem::transmute_copy(&pbadminkey),
                    core::mem::transmute_copy(&cbadminkey),
                    core::mem::transmute_copy(&pbadminkcv),
                    core::mem::transmute_copy(&cbadminkcv),
                    core::mem::transmute_copy(&pbpuk),
                    core::mem::transmute_copy(&cbpuk),
                    core::mem::transmute_copy(&pbpin),
                    core::mem::transmute_copy(&cbpin),
                    core::mem::transmute_copy(&fgenerate),
                    core::mem::transmute_copy(&pstatuscallback),
                    core::mem::transmute_copy(&ppszinstanceid),
                    core::mem::transmute_copy(&pfneedreboot),
                )
                .into()
            }
        }
        unsafe extern "system" fn DestroyVirtualSmartCard<Identity: ITpmVirtualSmartCardManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszinstanceid: windows_core::PCWSTR, pstatuscallback: *mut core::ffi::c_void, pfneedreboot: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITpmVirtualSmartCardManager_Impl::DestroyVirtualSmartCard(this, core::mem::transmute(&pszinstanceid), core::mem::transmute_copy(&pstatuscallback)) {
                    Ok(ok__) => {
                        pfneedreboot.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            CreateVirtualSmartCard: CreateVirtualSmartCard::<Identity, OFFSET>,
            DestroyVirtualSmartCard: DestroyVirtualSmartCard::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITpmVirtualSmartCardManager as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for ITpmVirtualSmartCardManager {}
windows_core::imp::define_interface!(ITpmVirtualSmartCardManager2, ITpmVirtualSmartCardManager2_Vtbl, 0xfdf8a2b9_02de_47f4_bc26_aa85ab5e5267);
impl core::ops::Deref for ITpmVirtualSmartCardManager2 {
    type Target = ITpmVirtualSmartCardManager;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ITpmVirtualSmartCardManager2, windows_core::IUnknown, ITpmVirtualSmartCardManager);
impl ITpmVirtualSmartCardManager2 {
    pub unsafe fn CreateVirtualSmartCardWithPinPolicy<P0, P13>(&self, pszfriendlyname: P0, badminalgid: u8, pbadminkey: &[u8], pbadminkcv: &[u8], pbpuk: &[u8], pbpin: &[u8], pbpinpolicy: &[u8], fgenerate: bool, pstatuscallback: P13, ppszinstanceid: *mut windows_core::PWSTR, pfneedreboot: *mut windows_core::BOOL) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        P13: windows_core::Param<ITpmVirtualSmartCardManagerStatusCallback>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).CreateVirtualSmartCardWithPinPolicy)(
                windows_core::Interface::as_raw(self),
                pszfriendlyname.param().abi(),
                badminalgid,
                core::mem::transmute(pbadminkey.as_ptr()),
                pbadminkey.len().try_into().unwrap(),
                core::mem::transmute(pbadminkcv.as_ptr()),
                pbadminkcv.len().try_into().unwrap(),
                core::mem::transmute(pbpuk.as_ptr()),
                pbpuk.len().try_into().unwrap(),
                core::mem::transmute(pbpin.as_ptr()),
                pbpin.len().try_into().unwrap(),
                core::mem::transmute(pbpinpolicy.as_ptr()),
                pbpinpolicy.len().try_into().unwrap(),
                fgenerate.into(),
                pstatuscallback.param().abi(),
                ppszinstanceid as _,
                pfneedreboot as _,
            )
            .ok()
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ITpmVirtualSmartCardManager2_Vtbl {
    pub base__: ITpmVirtualSmartCardManager_Vtbl,
    pub CreateVirtualSmartCardWithPinPolicy: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, u8, *const u8, u32, *const u8, u32, *const u8, u32, *const u8, u32, *const u8, u32, windows_core::BOOL, *mut core::ffi::c_void, *mut windows_core::PWSTR, *mut windows_core::BOOL) -> windows_core::HRESULT,
}
pub trait ITpmVirtualSmartCardManager2_Impl: ITpmVirtualSmartCardManager_Impl {
    fn CreateVirtualSmartCardWithPinPolicy(&self, pszfriendlyname: &windows_core::PCWSTR, badminalgid: u8, pbadminkey: *const u8, cbadminkey: u32, pbadminkcv: *const u8, cbadminkcv: u32, pbpuk: *const u8, cbpuk: u32, pbpin: *const u8, cbpin: u32, pbpinpolicy: *const u8, cbpinpolicy: u32, fgenerate: windows_core::BOOL, pstatuscallback: windows_core::Ref<ITpmVirtualSmartCardManagerStatusCallback>, ppszinstanceid: *mut windows_core::PWSTR, pfneedreboot: *mut windows_core::BOOL) -> windows_core::Result<()>;
}
impl ITpmVirtualSmartCardManager2_Vtbl {
    pub const fn new<Identity: ITpmVirtualSmartCardManager2_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn CreateVirtualSmartCardWithPinPolicy<Identity: ITpmVirtualSmartCardManager2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszfriendlyname: windows_core::PCWSTR, badminalgid: u8, pbadminkey: *const u8, cbadminkey: u32, pbadminkcv: *const u8, cbadminkcv: u32, pbpuk: *const u8, cbpuk: u32, pbpin: *const u8, cbpin: u32, pbpinpolicy: *const u8, cbpinpolicy: u32, fgenerate: windows_core::BOOL, pstatuscallback: *mut core::ffi::c_void, ppszinstanceid: *mut windows_core::PWSTR, pfneedreboot: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITpmVirtualSmartCardManager2_Impl::CreateVirtualSmartCardWithPinPolicy(
                    this,
                    core::mem::transmute(&pszfriendlyname),
                    core::mem::transmute_copy(&badminalgid),
                    core::mem::transmute_copy(&pbadminkey),
                    core::mem::transmute_copy(&cbadminkey),
                    core::mem::transmute_copy(&pbadminkcv),
                    core::mem::transmute_copy(&cbadminkcv),
                    core::mem::transmute_copy(&pbpuk),
                    core::mem::transmute_copy(&cbpuk),
                    core::mem::transmute_copy(&pbpin),
                    core::mem::transmute_copy(&cbpin),
                    core::mem::transmute_copy(&pbpinpolicy),
                    core::mem::transmute_copy(&cbpinpolicy),
                    core::mem::transmute_copy(&fgenerate),
                    core::mem::transmute_copy(&pstatuscallback),
                    core::mem::transmute_copy(&ppszinstanceid),
                    core::mem::transmute_copy(&pfneedreboot),
                )
                .into()
            }
        }
        Self {
            base__: ITpmVirtualSmartCardManager_Vtbl::new::<Identity, OFFSET>(),
            CreateVirtualSmartCardWithPinPolicy: CreateVirtualSmartCardWithPinPolicy::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITpmVirtualSmartCardManager2 as windows_core::Interface>::IID || iid == &<ITpmVirtualSmartCardManager as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for ITpmVirtualSmartCardManager2 {}
windows_core::imp::define_interface!(ITpmVirtualSmartCardManager3, ITpmVirtualSmartCardManager3_Vtbl, 0x3c745a97_f375_4150_be17_5950f694c699);
impl core::ops::Deref for ITpmVirtualSmartCardManager3 {
    type Target = ITpmVirtualSmartCardManager2;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ITpmVirtualSmartCardManager3, windows_core::IUnknown, ITpmVirtualSmartCardManager, ITpmVirtualSmartCardManager2);
impl ITpmVirtualSmartCardManager3 {
    pub unsafe fn CreateVirtualSmartCardWithAttestation<P0, P14>(&self, pszfriendlyname: P0, badminalgid: u8, pbadminkey: &[u8], pbadminkcv: &[u8], pbpuk: &[u8], pbpin: &[u8], pbpinpolicy: &[u8], attestationtype: TPMVSC_ATTESTATION_TYPE, fgenerate: bool, pstatuscallback: P14) -> windows_core::Result<windows_core::PWSTR>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        P14: windows_core::Param<ITpmVirtualSmartCardManagerStatusCallback>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateVirtualSmartCardWithAttestation)(
                windows_core::Interface::as_raw(self),
                pszfriendlyname.param().abi(),
                badminalgid,
                core::mem::transmute(pbadminkey.as_ptr()),
                pbadminkey.len().try_into().unwrap(),
                core::mem::transmute(pbadminkcv.as_ptr()),
                pbadminkcv.len().try_into().unwrap(),
                core::mem::transmute(pbpuk.as_ptr()),
                pbpuk.len().try_into().unwrap(),
                core::mem::transmute(pbpin.as_ptr()),
                pbpin.len().try_into().unwrap(),
                core::mem::transmute(pbpinpolicy.as_ptr()),
                pbpinpolicy.len().try_into().unwrap(),
                attestationtype,
                fgenerate.into(),
                pstatuscallback.param().abi(),
                &mut result__,
            )
            .map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ITpmVirtualSmartCardManager3_Vtbl {
    pub base__: ITpmVirtualSmartCardManager2_Vtbl,
    pub CreateVirtualSmartCardWithAttestation: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, u8, *const u8, u32, *const u8, u32, *const u8, u32, *const u8, u32, *const u8, u32, TPMVSC_ATTESTATION_TYPE, windows_core::BOOL, *mut core::ffi::c_void, *mut windows_core::PWSTR) -> windows_core::HRESULT,
}
pub trait ITpmVirtualSmartCardManager3_Impl: ITpmVirtualSmartCardManager2_Impl {
    fn CreateVirtualSmartCardWithAttestation(&self, pszfriendlyname: &windows_core::PCWSTR, badminalgid: u8, pbadminkey: *const u8, cbadminkey: u32, pbadminkcv: *const u8, cbadminkcv: u32, pbpuk: *const u8, cbpuk: u32, pbpin: *const u8, cbpin: u32, pbpinpolicy: *const u8, cbpinpolicy: u32, attestationtype: TPMVSC_ATTESTATION_TYPE, fgenerate: windows_core::BOOL, pstatuscallback: windows_core::Ref<ITpmVirtualSmartCardManagerStatusCallback>) -> windows_core::Result<windows_core::PWSTR>;
}
impl ITpmVirtualSmartCardManager3_Vtbl {
    pub const fn new<Identity: ITpmVirtualSmartCardManager3_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn CreateVirtualSmartCardWithAttestation<Identity: ITpmVirtualSmartCardManager3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszfriendlyname: windows_core::PCWSTR, badminalgid: u8, pbadminkey: *const u8, cbadminkey: u32, pbadminkcv: *const u8, cbadminkcv: u32, pbpuk: *const u8, cbpuk: u32, pbpin: *const u8, cbpin: u32, pbpinpolicy: *const u8, cbpinpolicy: u32, attestationtype: TPMVSC_ATTESTATION_TYPE, fgenerate: windows_core::BOOL, pstatuscallback: *mut core::ffi::c_void, ppszinstanceid: *mut windows_core::PWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITpmVirtualSmartCardManager3_Impl::CreateVirtualSmartCardWithAttestation(
                    this,
                    core::mem::transmute(&pszfriendlyname),
                    core::mem::transmute_copy(&badminalgid),
                    core::mem::transmute_copy(&pbadminkey),
                    core::mem::transmute_copy(&cbadminkey),
                    core::mem::transmute_copy(&pbadminkcv),
                    core::mem::transmute_copy(&cbadminkcv),
                    core::mem::transmute_copy(&pbpuk),
                    core::mem::transmute_copy(&cbpuk),
                    core::mem::transmute_copy(&pbpin),
                    core::mem::transmute_copy(&cbpin),
                    core::mem::transmute_copy(&pbpinpolicy),
                    core::mem::transmute_copy(&cbpinpolicy),
                    core::mem::transmute_copy(&attestationtype),
                    core::mem::transmute_copy(&fgenerate),
                    core::mem::transmute_copy(&pstatuscallback),
                ) {
                    Ok(ok__) => {
                        ppszinstanceid.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: ITpmVirtualSmartCardManager2_Vtbl::new::<Identity, OFFSET>(),
            CreateVirtualSmartCardWithAttestation: CreateVirtualSmartCardWithAttestation::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITpmVirtualSmartCardManager3 as windows_core::Interface>::IID || iid == &<ITpmVirtualSmartCardManager as windows_core::Interface>::IID || iid == &<ITpmVirtualSmartCardManager2 as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for ITpmVirtualSmartCardManager3 {}
windows_core::imp::define_interface!(ITpmVirtualSmartCardManagerStatusCallback, ITpmVirtualSmartCardManagerStatusCallback_Vtbl, 0x1a1bb35f_abb8_451c_a1ae_33d98f1bef4a);
windows_core::imp::interface_hierarchy!(ITpmVirtualSmartCardManagerStatusCallback, windows_core::IUnknown);
impl ITpmVirtualSmartCardManagerStatusCallback {
    pub unsafe fn ReportProgress(&self, status: TPMVSCMGR_STATUS) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).ReportProgress)(windows_core::Interface::as_raw(self), status).ok() }
    }
    pub unsafe fn ReportError(&self, error: TPMVSCMGR_ERROR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).ReportError)(windows_core::Interface::as_raw(self), error).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ITpmVirtualSmartCardManagerStatusCallback_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub ReportProgress: unsafe extern "system" fn(*mut core::ffi::c_void, TPMVSCMGR_STATUS) -> windows_core::HRESULT,
    pub ReportError: unsafe extern "system" fn(*mut core::ffi::c_void, TPMVSCMGR_ERROR) -> windows_core::HRESULT,
}
pub trait ITpmVirtualSmartCardManagerStatusCallback_Impl: windows_core::IUnknownImpl {
    fn ReportProgress(&self, status: TPMVSCMGR_STATUS) -> windows_core::Result<()>;
    fn ReportError(&self, error: TPMVSCMGR_ERROR) -> windows_core::Result<()>;
}
impl ITpmVirtualSmartCardManagerStatusCallback_Vtbl {
    pub const fn new<Identity: ITpmVirtualSmartCardManagerStatusCallback_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn ReportProgress<Identity: ITpmVirtualSmartCardManagerStatusCallback_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, status: TPMVSCMGR_STATUS) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITpmVirtualSmartCardManagerStatusCallback_Impl::ReportProgress(this, core::mem::transmute_copy(&status)).into()
            }
        }
        unsafe extern "system" fn ReportError<Identity: ITpmVirtualSmartCardManagerStatusCallback_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, error: TPMVSCMGR_ERROR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITpmVirtualSmartCardManagerStatusCallback_Impl::ReportError(this, core::mem::transmute_copy(&error)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            ReportProgress: ReportProgress::<Identity, OFFSET>,
            ReportError: ReportError::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITpmVirtualSmartCardManagerStatusCallback as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for ITpmVirtualSmartCardManagerStatusCallback {}
pub const RemoteTpmVirtualSmartCardManager: windows_core::GUID = windows_core::GUID::from_u128(0x152ea2a8_70dc_4c59_8b2a_32aa3ca0dcac);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct TPMVSCMGR_ERROR(pub i32);
pub const TPMVSCMGR_ERROR_CARD_CREATE: TPMVSCMGR_ERROR = TPMVSCMGR_ERROR(17i32);
pub const TPMVSCMGR_ERROR_CARD_DESTROY: TPMVSCMGR_ERROR = TPMVSCMGR_ERROR(18i32);
pub const TPMVSCMGR_ERROR_GENERATE_FILESYSTEM: TPMVSCMGR_ERROR = TPMVSCMGR_ERROR(16i32);
pub const TPMVSCMGR_ERROR_GENERATE_LOCATE_READER: TPMVSCMGR_ERROR = TPMVSCMGR_ERROR(15i32);
pub const TPMVSCMGR_ERROR_IMPERSONATION: TPMVSCMGR_ERROR = TPMVSCMGR_ERROR(0i32);
pub const TPMVSCMGR_ERROR_PIN_COMPLEXITY: TPMVSCMGR_ERROR = TPMVSCMGR_ERROR(1i32);
pub const TPMVSCMGR_ERROR_READER_COUNT_LIMIT: TPMVSCMGR_ERROR = TPMVSCMGR_ERROR(2i32);
pub const TPMVSCMGR_ERROR_TERMINAL_SERVICES_SESSION: TPMVSCMGR_ERROR = TPMVSCMGR_ERROR(3i32);
pub const TPMVSCMGR_ERROR_VGIDSSIMULATOR_CREATE: TPMVSCMGR_ERROR = TPMVSCMGR_ERROR(8i32);
pub const TPMVSCMGR_ERROR_VGIDSSIMULATOR_DESTROY: TPMVSCMGR_ERROR = TPMVSCMGR_ERROR(9i32);
pub const TPMVSCMGR_ERROR_VGIDSSIMULATOR_INITIALIZE: TPMVSCMGR_ERROR = TPMVSCMGR_ERROR(7i32);
pub const TPMVSCMGR_ERROR_VGIDSSIMULATOR_READ_PROPERTY: TPMVSCMGR_ERROR = TPMVSCMGR_ERROR(11i32);
pub const TPMVSCMGR_ERROR_VGIDSSIMULATOR_WRITE_PROPERTY: TPMVSCMGR_ERROR = TPMVSCMGR_ERROR(10i32);
pub const TPMVSCMGR_ERROR_VREADER_CREATE: TPMVSCMGR_ERROR = TPMVSCMGR_ERROR(13i32);
pub const TPMVSCMGR_ERROR_VREADER_DESTROY: TPMVSCMGR_ERROR = TPMVSCMGR_ERROR(14i32);
pub const TPMVSCMGR_ERROR_VREADER_INITIALIZE: TPMVSCMGR_ERROR = TPMVSCMGR_ERROR(12i32);
pub const TPMVSCMGR_ERROR_VTPMSMARTCARD_CREATE: TPMVSCMGR_ERROR = TPMVSCMGR_ERROR(5i32);
pub const TPMVSCMGR_ERROR_VTPMSMARTCARD_DESTROY: TPMVSCMGR_ERROR = TPMVSCMGR_ERROR(6i32);
pub const TPMVSCMGR_ERROR_VTPMSMARTCARD_INITIALIZE: TPMVSCMGR_ERROR = TPMVSCMGR_ERROR(4i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct TPMVSCMGR_STATUS(pub i32);
pub const TPMVSCMGR_STATUS_CARD_CREATED: TPMVSCMGR_STATUS = TPMVSCMGR_STATUS(12i32);
pub const TPMVSCMGR_STATUS_CARD_DESTROYED: TPMVSCMGR_STATUS = TPMVSCMGR_STATUS(13i32);
pub const TPMVSCMGR_STATUS_GENERATE_AUTHENTICATING: TPMVSCMGR_STATUS = TPMVSCMGR_STATUS(10i32);
pub const TPMVSCMGR_STATUS_GENERATE_RUNNING: TPMVSCMGR_STATUS = TPMVSCMGR_STATUS(11i32);
pub const TPMVSCMGR_STATUS_GENERATE_WAITING: TPMVSCMGR_STATUS = TPMVSCMGR_STATUS(9i32);
pub const TPMVSCMGR_STATUS_VGIDSSIMULATOR_CREATING: TPMVSCMGR_STATUS = TPMVSCMGR_STATUS(4i32);
pub const TPMVSCMGR_STATUS_VGIDSSIMULATOR_DESTROYING: TPMVSCMGR_STATUS = TPMVSCMGR_STATUS(5i32);
pub const TPMVSCMGR_STATUS_VGIDSSIMULATOR_INITIALIZING: TPMVSCMGR_STATUS = TPMVSCMGR_STATUS(3i32);
pub const TPMVSCMGR_STATUS_VREADER_CREATING: TPMVSCMGR_STATUS = TPMVSCMGR_STATUS(7i32);
pub const TPMVSCMGR_STATUS_VREADER_DESTROYING: TPMVSCMGR_STATUS = TPMVSCMGR_STATUS(8i32);
pub const TPMVSCMGR_STATUS_VREADER_INITIALIZING: TPMVSCMGR_STATUS = TPMVSCMGR_STATUS(6i32);
pub const TPMVSCMGR_STATUS_VTPMSMARTCARD_CREATING: TPMVSCMGR_STATUS = TPMVSCMGR_STATUS(1i32);
pub const TPMVSCMGR_STATUS_VTPMSMARTCARD_DESTROYING: TPMVSCMGR_STATUS = TPMVSCMGR_STATUS(2i32);
pub const TPMVSCMGR_STATUS_VTPMSMARTCARD_INITIALIZING: TPMVSCMGR_STATUS = TPMVSCMGR_STATUS(0i32);
pub const TPMVSC_ATTESTATION_AIK_AND_CERTIFICATE: TPMVSC_ATTESTATION_TYPE = TPMVSC_ATTESTATION_TYPE(2i32);
pub const TPMVSC_ATTESTATION_AIK_ONLY: TPMVSC_ATTESTATION_TYPE = TPMVSC_ATTESTATION_TYPE(1i32);
pub const TPMVSC_ATTESTATION_NONE: TPMVSC_ATTESTATION_TYPE = TPMVSC_ATTESTATION_TYPE(0i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct TPMVSC_ATTESTATION_TYPE(pub i32);
pub const TPMVSC_DEFAULT_ADMIN_ALGORITHM_ID: u32 = 130u32;
pub const TpmVirtualSmartCardManager: windows_core::GUID = windows_core::GUID::from_u128(0x16a18e86_7f6e_4c20_ad89_4ffc0db7a96a);
