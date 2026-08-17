pub trait IWsbApplicationAsync_Impl: Sized {
    fn QueryStatus(&self) -> windows_core::Result<windows_core::HRESULT>;
    fn Abort(&self) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IWsbApplicationAsync {}
impl IWsbApplicationAsync_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IWsbApplicationAsync_Vtbl
    where
        Identity: IWsbApplicationAsync_Impl,
    {
        unsafe extern "system" fn QueryStatus<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, phrresult: *mut windows_core::HRESULT) -> windows_core::HRESULT
        where
            Identity: IWsbApplicationAsync_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWsbApplicationAsync_Impl::QueryStatus(this) {
                Ok(ok__) => {
                    phrresult.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Abort<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IWsbApplicationAsync_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWsbApplicationAsync_Impl::Abort(this).into()
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), QueryStatus: QueryStatus::<Identity, OFFSET>, Abort: Abort::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWsbApplicationAsync as windows_core::Interface>::IID
    }
}
pub trait IWsbApplicationBackupSupport_Impl: Sized {
    fn CheckConsistency(&self, wszwritermetadata: &windows_core::PCWSTR, wszcomponentname: &windows_core::PCWSTR, wszcomponentlogicalpath: &windows_core::PCWSTR, cvolumes: u32, rgwszsourcevolumepath: *const windows_core::PCWSTR, rgwszsnapshotvolumepath: *const windows_core::PCWSTR) -> windows_core::Result<IWsbApplicationAsync>;
}
impl windows_core::RuntimeName for IWsbApplicationBackupSupport {}
impl IWsbApplicationBackupSupport_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IWsbApplicationBackupSupport_Vtbl
    where
        Identity: IWsbApplicationBackupSupport_Impl,
    {
        unsafe extern "system" fn CheckConsistency<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, wszwritermetadata: windows_core::PCWSTR, wszcomponentname: windows_core::PCWSTR, wszcomponentlogicalpath: windows_core::PCWSTR, cvolumes: u32, rgwszsourcevolumepath: *const windows_core::PCWSTR, rgwszsnapshotvolumepath: *const windows_core::PCWSTR, ppasync: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IWsbApplicationBackupSupport_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWsbApplicationBackupSupport_Impl::CheckConsistency(this, core::mem::transmute(&wszwritermetadata), core::mem::transmute(&wszcomponentname), core::mem::transmute(&wszcomponentlogicalpath), core::mem::transmute_copy(&cvolumes), core::mem::transmute_copy(&rgwszsourcevolumepath), core::mem::transmute_copy(&rgwszsnapshotvolumepath)) {
                Ok(ok__) => {
                    ppasync.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), CheckConsistency: CheckConsistency::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWsbApplicationBackupSupport as windows_core::Interface>::IID
    }
}
pub trait IWsbApplicationRestoreSupport_Impl: Sized {
    fn PreRestore(&self, wszwritermetadata: &windows_core::PCWSTR, wszcomponentname: &windows_core::PCWSTR, wszcomponentlogicalpath: &windows_core::PCWSTR, bnorollforward: super::super::Foundation::BOOLEAN) -> windows_core::Result<()>;
    fn PostRestore(&self, wszwritermetadata: &windows_core::PCWSTR, wszcomponentname: &windows_core::PCWSTR, wszcomponentlogicalpath: &windows_core::PCWSTR, bnorollforward: super::super::Foundation::BOOLEAN) -> windows_core::Result<()>;
    fn OrderComponents(&self, ccomponents: u32, rgcomponentname: *const windows_core::PCWSTR, rgcomponentlogicalpaths: *const windows_core::PCWSTR, prgcomponentname: *mut *mut windows_core::PWSTR, prgcomponentlogicalpath: *mut *mut windows_core::PWSTR) -> windows_core::Result<()>;
    fn IsRollForwardSupported(&self) -> windows_core::Result<u8>;
}
impl windows_core::RuntimeName for IWsbApplicationRestoreSupport {}
impl IWsbApplicationRestoreSupport_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IWsbApplicationRestoreSupport_Vtbl
    where
        Identity: IWsbApplicationRestoreSupport_Impl,
    {
        unsafe extern "system" fn PreRestore<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, wszwritermetadata: windows_core::PCWSTR, wszcomponentname: windows_core::PCWSTR, wszcomponentlogicalpath: windows_core::PCWSTR, bnorollforward: super::super::Foundation::BOOLEAN) -> windows_core::HRESULT
        where
            Identity: IWsbApplicationRestoreSupport_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWsbApplicationRestoreSupport_Impl::PreRestore(this, core::mem::transmute(&wszwritermetadata), core::mem::transmute(&wszcomponentname), core::mem::transmute(&wszcomponentlogicalpath), core::mem::transmute_copy(&bnorollforward)).into()
        }
        unsafe extern "system" fn PostRestore<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, wszwritermetadata: windows_core::PCWSTR, wszcomponentname: windows_core::PCWSTR, wszcomponentlogicalpath: windows_core::PCWSTR, bnorollforward: super::super::Foundation::BOOLEAN) -> windows_core::HRESULT
        where
            Identity: IWsbApplicationRestoreSupport_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWsbApplicationRestoreSupport_Impl::PostRestore(this, core::mem::transmute(&wszwritermetadata), core::mem::transmute(&wszcomponentname), core::mem::transmute(&wszcomponentlogicalpath), core::mem::transmute_copy(&bnorollforward)).into()
        }
        unsafe extern "system" fn OrderComponents<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ccomponents: u32, rgcomponentname: *const windows_core::PCWSTR, rgcomponentlogicalpaths: *const windows_core::PCWSTR, prgcomponentname: *mut *mut windows_core::PWSTR, prgcomponentlogicalpath: *mut *mut windows_core::PWSTR) -> windows_core::HRESULT
        where
            Identity: IWsbApplicationRestoreSupport_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWsbApplicationRestoreSupport_Impl::OrderComponents(this, core::mem::transmute_copy(&ccomponents), core::mem::transmute_copy(&rgcomponentname), core::mem::transmute_copy(&rgcomponentlogicalpaths), core::mem::transmute_copy(&prgcomponentname), core::mem::transmute_copy(&prgcomponentlogicalpath)).into()
        }
        unsafe extern "system" fn IsRollForwardSupported<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbrollforwardsupported: *mut u8) -> windows_core::HRESULT
        where
            Identity: IWsbApplicationRestoreSupport_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWsbApplicationRestoreSupport_Impl::IsRollForwardSupported(this) {
                Ok(ok__) => {
                    pbrollforwardsupported.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            PreRestore: PreRestore::<Identity, OFFSET>,
            PostRestore: PostRestore::<Identity, OFFSET>,
            OrderComponents: OrderComponents::<Identity, OFFSET>,
            IsRollForwardSupported: IsRollForwardSupported::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWsbApplicationRestoreSupport as windows_core::Interface>::IID
    }
}
