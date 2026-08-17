pub trait ITpmVirtualSmartCardManager_Impl: Sized {
    fn CreateVirtualSmartCard(&self, pszfriendlyname: &windows_core::PCWSTR, badminalgid: u8, pbadminkey: *const u8, cbadminkey: u32, pbadminkcv: *const u8, cbadminkcv: u32, pbpuk: *const u8, cbpuk: u32, pbpin: *const u8, cbpin: u32, fgenerate: super::super::Foundation::BOOL, pstatuscallback: Option<&ITpmVirtualSmartCardManagerStatusCallback>, ppszinstanceid: *mut windows_core::PWSTR, pfneedreboot: *mut super::super::Foundation::BOOL) -> windows_core::Result<()>;
    fn DestroyVirtualSmartCard(&self, pszinstanceid: &windows_core::PCWSTR, pstatuscallback: Option<&ITpmVirtualSmartCardManagerStatusCallback>) -> windows_core::Result<super::super::Foundation::BOOL>;
}
impl windows_core::RuntimeName for ITpmVirtualSmartCardManager {}
impl ITpmVirtualSmartCardManager_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ITpmVirtualSmartCardManager_Vtbl
    where
        Identity: ITpmVirtualSmartCardManager_Impl,
    {
        unsafe extern "system" fn CreateVirtualSmartCard<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszfriendlyname: windows_core::PCWSTR, badminalgid: u8, pbadminkey: *const u8, cbadminkey: u32, pbadminkcv: *const u8, cbadminkcv: u32, pbpuk: *const u8, cbpuk: u32, pbpin: *const u8, cbpin: u32, fgenerate: super::super::Foundation::BOOL, pstatuscallback: *mut core::ffi::c_void, ppszinstanceid: *mut windows_core::PWSTR, pfneedreboot: *mut super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: ITpmVirtualSmartCardManager_Impl,
        {
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
                windows_core::from_raw_borrowed(&pstatuscallback),
                core::mem::transmute_copy(&ppszinstanceid),
                core::mem::transmute_copy(&pfneedreboot),
            )
            .into()
        }
        unsafe extern "system" fn DestroyVirtualSmartCard<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszinstanceid: windows_core::PCWSTR, pstatuscallback: *mut core::ffi::c_void, pfneedreboot: *mut super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: ITpmVirtualSmartCardManager_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITpmVirtualSmartCardManager_Impl::DestroyVirtualSmartCard(this, core::mem::transmute(&pszinstanceid), windows_core::from_raw_borrowed(&pstatuscallback)) {
                Ok(ok__) => {
                    pfneedreboot.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
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
pub trait ITpmVirtualSmartCardManager2_Impl: Sized + ITpmVirtualSmartCardManager_Impl {
    fn CreateVirtualSmartCardWithPinPolicy(&self, pszfriendlyname: &windows_core::PCWSTR, badminalgid: u8, pbadminkey: *const u8, cbadminkey: u32, pbadminkcv: *const u8, cbadminkcv: u32, pbpuk: *const u8, cbpuk: u32, pbpin: *const u8, cbpin: u32, pbpinpolicy: *const u8, cbpinpolicy: u32, fgenerate: super::super::Foundation::BOOL, pstatuscallback: Option<&ITpmVirtualSmartCardManagerStatusCallback>, ppszinstanceid: *mut windows_core::PWSTR, pfneedreboot: *mut super::super::Foundation::BOOL) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for ITpmVirtualSmartCardManager2 {}
impl ITpmVirtualSmartCardManager2_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ITpmVirtualSmartCardManager2_Vtbl
    where
        Identity: ITpmVirtualSmartCardManager2_Impl,
    {
        unsafe extern "system" fn CreateVirtualSmartCardWithPinPolicy<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszfriendlyname: windows_core::PCWSTR, badminalgid: u8, pbadminkey: *const u8, cbadminkey: u32, pbadminkcv: *const u8, cbadminkcv: u32, pbpuk: *const u8, cbpuk: u32, pbpin: *const u8, cbpin: u32, pbpinpolicy: *const u8, cbpinpolicy: u32, fgenerate: super::super::Foundation::BOOL, pstatuscallback: *mut core::ffi::c_void, ppszinstanceid: *mut windows_core::PWSTR, pfneedreboot: *mut super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: ITpmVirtualSmartCardManager2_Impl,
        {
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
                windows_core::from_raw_borrowed(&pstatuscallback),
                core::mem::transmute_copy(&ppszinstanceid),
                core::mem::transmute_copy(&pfneedreboot),
            )
            .into()
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
pub trait ITpmVirtualSmartCardManager3_Impl: Sized + ITpmVirtualSmartCardManager2_Impl {
    fn CreateVirtualSmartCardWithAttestation(&self, pszfriendlyname: &windows_core::PCWSTR, badminalgid: u8, pbadminkey: *const u8, cbadminkey: u32, pbadminkcv: *const u8, cbadminkcv: u32, pbpuk: *const u8, cbpuk: u32, pbpin: *const u8, cbpin: u32, pbpinpolicy: *const u8, cbpinpolicy: u32, attestationtype: TPMVSC_ATTESTATION_TYPE, fgenerate: super::super::Foundation::BOOL, pstatuscallback: Option<&ITpmVirtualSmartCardManagerStatusCallback>) -> windows_core::Result<windows_core::PWSTR>;
}
impl windows_core::RuntimeName for ITpmVirtualSmartCardManager3 {}
impl ITpmVirtualSmartCardManager3_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ITpmVirtualSmartCardManager3_Vtbl
    where
        Identity: ITpmVirtualSmartCardManager3_Impl,
    {
        unsafe extern "system" fn CreateVirtualSmartCardWithAttestation<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszfriendlyname: windows_core::PCWSTR, badminalgid: u8, pbadminkey: *const u8, cbadminkey: u32, pbadminkcv: *const u8, cbadminkcv: u32, pbpuk: *const u8, cbpuk: u32, pbpin: *const u8, cbpin: u32, pbpinpolicy: *const u8, cbpinpolicy: u32, attestationtype: TPMVSC_ATTESTATION_TYPE, fgenerate: super::super::Foundation::BOOL, pstatuscallback: *mut core::ffi::c_void, ppszinstanceid: *mut windows_core::PWSTR) -> windows_core::HRESULT
        where
            Identity: ITpmVirtualSmartCardManager3_Impl,
        {
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
                windows_core::from_raw_borrowed(&pstatuscallback),
            ) {
                Ok(ok__) => {
                    ppszinstanceid.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
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
pub trait ITpmVirtualSmartCardManagerStatusCallback_Impl: Sized {
    fn ReportProgress(&self, status: TPMVSCMGR_STATUS) -> windows_core::Result<()>;
    fn ReportError(&self, error: TPMVSCMGR_ERROR) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for ITpmVirtualSmartCardManagerStatusCallback {}
impl ITpmVirtualSmartCardManagerStatusCallback_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ITpmVirtualSmartCardManagerStatusCallback_Vtbl
    where
        Identity: ITpmVirtualSmartCardManagerStatusCallback_Impl,
    {
        unsafe extern "system" fn ReportProgress<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, status: TPMVSCMGR_STATUS) -> windows_core::HRESULT
        where
            Identity: ITpmVirtualSmartCardManagerStatusCallback_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITpmVirtualSmartCardManagerStatusCallback_Impl::ReportProgress(this, core::mem::transmute_copy(&status)).into()
        }
        unsafe extern "system" fn ReportError<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, error: TPMVSCMGR_ERROR) -> windows_core::HRESULT
        where
            Identity: ITpmVirtualSmartCardManagerStatusCallback_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITpmVirtualSmartCardManagerStatusCallback_Impl::ReportError(this, core::mem::transmute_copy(&error)).into()
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
