pub trait ICloneViewHelper_Impl: Sized {
    fn GetConnectedIDs(&self, wszadaptorname: &windows_core::PCWSTR, pulcount: *mut u32, pulid: *mut u32, ulflags: u32) -> windows_core::Result<()>;
    fn GetActiveTopology(&self, wszadaptorname: &windows_core::PCWSTR, ulsourceid: u32, pulcount: *mut u32, pultargetid: *mut u32) -> windows_core::Result<()>;
    fn SetActiveTopology(&self, wszadaptorname: &windows_core::PCWSTR, ulsourceid: u32, ulcount: u32, pultargetid: *const u32) -> windows_core::Result<()>;
    fn Commit(&self, ffinalcall: super::super::Foundation::BOOL) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for ICloneViewHelper {}
impl ICloneViewHelper_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ICloneViewHelper_Vtbl
    where
        Identity: ICloneViewHelper_Impl,
    {
        unsafe extern "system" fn GetConnectedIDs<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, wszadaptorname: windows_core::PCWSTR, pulcount: *mut u32, pulid: *mut u32, ulflags: u32) -> windows_core::HRESULT
        where
            Identity: ICloneViewHelper_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICloneViewHelper_Impl::GetConnectedIDs(this, core::mem::transmute(&wszadaptorname), core::mem::transmute_copy(&pulcount), core::mem::transmute_copy(&pulid), core::mem::transmute_copy(&ulflags)).into()
        }
        unsafe extern "system" fn GetActiveTopology<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, wszadaptorname: windows_core::PCWSTR, ulsourceid: u32, pulcount: *mut u32, pultargetid: *mut u32) -> windows_core::HRESULT
        where
            Identity: ICloneViewHelper_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICloneViewHelper_Impl::GetActiveTopology(this, core::mem::transmute(&wszadaptorname), core::mem::transmute_copy(&ulsourceid), core::mem::transmute_copy(&pulcount), core::mem::transmute_copy(&pultargetid)).into()
        }
        unsafe extern "system" fn SetActiveTopology<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, wszadaptorname: windows_core::PCWSTR, ulsourceid: u32, ulcount: u32, pultargetid: *const u32) -> windows_core::HRESULT
        where
            Identity: ICloneViewHelper_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICloneViewHelper_Impl::SetActiveTopology(this, core::mem::transmute(&wszadaptorname), core::mem::transmute_copy(&ulsourceid), core::mem::transmute_copy(&ulcount), core::mem::transmute_copy(&pultargetid)).into()
        }
        unsafe extern "system" fn Commit<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ffinalcall: super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: ICloneViewHelper_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICloneViewHelper_Impl::Commit(this, core::mem::transmute_copy(&ffinalcall)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetConnectedIDs: GetConnectedIDs::<Identity, OFFSET>,
            GetActiveTopology: GetActiveTopology::<Identity, OFFSET>,
            SetActiveTopology: SetActiveTopology::<Identity, OFFSET>,
            Commit: Commit::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ICloneViewHelper as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IViewHelper_Impl: Sized {
    fn GetConnectedIDs(&self, wszadaptorname: &windows_core::PCWSTR, pulcount: *mut u32, pulid: *mut u32, ulflags: u32) -> windows_core::Result<()>;
    fn GetActiveTopology(&self, wszadaptorname: &windows_core::PCWSTR, ulsourceid: u32, pulcount: *mut u32, pultargetid: *mut u32) -> windows_core::Result<()>;
    fn SetActiveTopology(&self, wszadaptorname: &windows_core::PCWSTR, ulsourceid: u32, ulcount: u32, pultargetid: *const u32) -> windows_core::Result<()>;
    fn Commit(&self) -> windows_core::Result<()>;
    fn SetConfiguration(&self, pistream: Option<&super::super::System::Com::IStream>) -> windows_core::Result<u32>;
    fn GetProceedOnNewConfiguration(&self) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IViewHelper {}
#[cfg(feature = "Win32_System_Com")]
impl IViewHelper_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IViewHelper_Vtbl
    where
        Identity: IViewHelper_Impl,
    {
        unsafe extern "system" fn GetConnectedIDs<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, wszadaptorname: windows_core::PCWSTR, pulcount: *mut u32, pulid: *mut u32, ulflags: u32) -> windows_core::HRESULT
        where
            Identity: IViewHelper_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IViewHelper_Impl::GetConnectedIDs(this, core::mem::transmute(&wszadaptorname), core::mem::transmute_copy(&pulcount), core::mem::transmute_copy(&pulid), core::mem::transmute_copy(&ulflags)).into()
        }
        unsafe extern "system" fn GetActiveTopology<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, wszadaptorname: windows_core::PCWSTR, ulsourceid: u32, pulcount: *mut u32, pultargetid: *mut u32) -> windows_core::HRESULT
        where
            Identity: IViewHelper_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IViewHelper_Impl::GetActiveTopology(this, core::mem::transmute(&wszadaptorname), core::mem::transmute_copy(&ulsourceid), core::mem::transmute_copy(&pulcount), core::mem::transmute_copy(&pultargetid)).into()
        }
        unsafe extern "system" fn SetActiveTopology<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, wszadaptorname: windows_core::PCWSTR, ulsourceid: u32, ulcount: u32, pultargetid: *const u32) -> windows_core::HRESULT
        where
            Identity: IViewHelper_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IViewHelper_Impl::SetActiveTopology(this, core::mem::transmute(&wszadaptorname), core::mem::transmute_copy(&ulsourceid), core::mem::transmute_copy(&ulcount), core::mem::transmute_copy(&pultargetid)).into()
        }
        unsafe extern "system" fn Commit<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IViewHelper_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IViewHelper_Impl::Commit(this).into()
        }
        unsafe extern "system" fn SetConfiguration<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pistream: *mut core::ffi::c_void, pulstatus: *mut u32) -> windows_core::HRESULT
        where
            Identity: IViewHelper_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IViewHelper_Impl::SetConfiguration(this, windows_core::from_raw_borrowed(&pistream)) {
                Ok(ok__) => {
                    pulstatus.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetProceedOnNewConfiguration<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IViewHelper_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IViewHelper_Impl::GetProceedOnNewConfiguration(this).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetConnectedIDs: GetConnectedIDs::<Identity, OFFSET>,
            GetActiveTopology: GetActiveTopology::<Identity, OFFSET>,
            SetActiveTopology: SetActiveTopology::<Identity, OFFSET>,
            Commit: Commit::<Identity, OFFSET>,
            SetConfiguration: SetConfiguration::<Identity, OFFSET>,
            GetProceedOnNewConfiguration: GetProceedOnNewConfiguration::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IViewHelper as windows_core::Interface>::IID
    }
}
