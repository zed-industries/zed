#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com_StructuredStorage"))]
pub trait AsyncIAdviseSink_Impl: Sized {
    fn Begin_OnDataChange(&self, pformatetc: *const FORMATETC, pstgmed: *const STGMEDIUM);
    fn Finish_OnDataChange(&self);
    fn Begin_OnViewChange(&self, dwaspect: u32, lindex: i32);
    fn Finish_OnViewChange(&self);
    fn Begin_OnRename(&self, pmk: Option<&IMoniker>);
    fn Finish_OnRename(&self);
    fn Begin_OnSave(&self);
    fn Finish_OnSave(&self);
    fn Begin_OnClose(&self);
    fn Finish_OnClose(&self);
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com_StructuredStorage"))]
impl windows_core::RuntimeName for AsyncIAdviseSink {}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com_StructuredStorage"))]
impl AsyncIAdviseSink_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> AsyncIAdviseSink_Vtbl
    where
        Identity: AsyncIAdviseSink_Impl,
    {
        unsafe extern "system" fn Begin_OnDataChange<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pformatetc: *const FORMATETC, pstgmed: *const STGMEDIUM)
        where
            Identity: AsyncIAdviseSink_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            AsyncIAdviseSink_Impl::Begin_OnDataChange(this, core::mem::transmute_copy(&pformatetc), core::mem::transmute_copy(&pstgmed))
        }
        unsafe extern "system" fn Finish_OnDataChange<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void)
        where
            Identity: AsyncIAdviseSink_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            AsyncIAdviseSink_Impl::Finish_OnDataChange(this)
        }
        unsafe extern "system" fn Begin_OnViewChange<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwaspect: u32, lindex: i32)
        where
            Identity: AsyncIAdviseSink_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            AsyncIAdviseSink_Impl::Begin_OnViewChange(this, core::mem::transmute_copy(&dwaspect), core::mem::transmute_copy(&lindex))
        }
        unsafe extern "system" fn Finish_OnViewChange<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void)
        where
            Identity: AsyncIAdviseSink_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            AsyncIAdviseSink_Impl::Finish_OnViewChange(this)
        }
        unsafe extern "system" fn Begin_OnRename<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pmk: *mut core::ffi::c_void)
        where
            Identity: AsyncIAdviseSink_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            AsyncIAdviseSink_Impl::Begin_OnRename(this, windows_core::from_raw_borrowed(&pmk))
        }
        unsafe extern "system" fn Finish_OnRename<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void)
        where
            Identity: AsyncIAdviseSink_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            AsyncIAdviseSink_Impl::Finish_OnRename(this)
        }
        unsafe extern "system" fn Begin_OnSave<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void)
        where
            Identity: AsyncIAdviseSink_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            AsyncIAdviseSink_Impl::Begin_OnSave(this)
        }
        unsafe extern "system" fn Finish_OnSave<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void)
        where
            Identity: AsyncIAdviseSink_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            AsyncIAdviseSink_Impl::Finish_OnSave(this)
        }
        unsafe extern "system" fn Begin_OnClose<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void)
        where
            Identity: AsyncIAdviseSink_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            AsyncIAdviseSink_Impl::Begin_OnClose(this)
        }
        unsafe extern "system" fn Finish_OnClose<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void)
        where
            Identity: AsyncIAdviseSink_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            AsyncIAdviseSink_Impl::Finish_OnClose(this)
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Begin_OnDataChange: Begin_OnDataChange::<Identity, OFFSET>,
            Finish_OnDataChange: Finish_OnDataChange::<Identity, OFFSET>,
            Begin_OnViewChange: Begin_OnViewChange::<Identity, OFFSET>,
            Finish_OnViewChange: Finish_OnViewChange::<Identity, OFFSET>,
            Begin_OnRename: Begin_OnRename::<Identity, OFFSET>,
            Finish_OnRename: Finish_OnRename::<Identity, OFFSET>,
            Begin_OnSave: Begin_OnSave::<Identity, OFFSET>,
            Finish_OnSave: Finish_OnSave::<Identity, OFFSET>,
            Begin_OnClose: Begin_OnClose::<Identity, OFFSET>,
            Finish_OnClose: Finish_OnClose::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<AsyncIAdviseSink as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com_StructuredStorage"))]
pub trait AsyncIAdviseSink2_Impl: Sized + AsyncIAdviseSink_Impl {
    fn Begin_OnLinkSrcChange(&self, pmk: Option<&IMoniker>);
    fn Finish_OnLinkSrcChange(&self);
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com_StructuredStorage"))]
impl windows_core::RuntimeName for AsyncIAdviseSink2 {}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com_StructuredStorage"))]
impl AsyncIAdviseSink2_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> AsyncIAdviseSink2_Vtbl
    where
        Identity: AsyncIAdviseSink2_Impl,
    {
        unsafe extern "system" fn Begin_OnLinkSrcChange<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pmk: *mut core::ffi::c_void)
        where
            Identity: AsyncIAdviseSink2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            AsyncIAdviseSink2_Impl::Begin_OnLinkSrcChange(this, windows_core::from_raw_borrowed(&pmk))
        }
        unsafe extern "system" fn Finish_OnLinkSrcChange<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void)
        where
            Identity: AsyncIAdviseSink2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            AsyncIAdviseSink2_Impl::Finish_OnLinkSrcChange(this)
        }
        Self {
            base__: AsyncIAdviseSink_Vtbl::new::<Identity, OFFSET>(),
            Begin_OnLinkSrcChange: Begin_OnLinkSrcChange::<Identity, OFFSET>,
            Finish_OnLinkSrcChange: Finish_OnLinkSrcChange::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<AsyncIAdviseSink2 as windows_core::Interface>::IID || iid == &<AsyncIAdviseSink as windows_core::Interface>::IID
    }
}
pub trait AsyncIMultiQI_Impl: Sized {
    fn Begin_QueryMultipleInterfaces(&self, cmqis: u32, pmqis: *mut MULTI_QI) -> windows_core::Result<()>;
    fn Finish_QueryMultipleInterfaces(&self, pmqis: *mut MULTI_QI) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for AsyncIMultiQI {}
impl AsyncIMultiQI_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> AsyncIMultiQI_Vtbl
    where
        Identity: AsyncIMultiQI_Impl,
    {
        unsafe extern "system" fn Begin_QueryMultipleInterfaces<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, cmqis: u32, pmqis: *mut MULTI_QI) -> windows_core::HRESULT
        where
            Identity: AsyncIMultiQI_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            AsyncIMultiQI_Impl::Begin_QueryMultipleInterfaces(this, core::mem::transmute_copy(&cmqis), core::mem::transmute_copy(&pmqis)).into()
        }
        unsafe extern "system" fn Finish_QueryMultipleInterfaces<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pmqis: *mut MULTI_QI) -> windows_core::HRESULT
        where
            Identity: AsyncIMultiQI_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            AsyncIMultiQI_Impl::Finish_QueryMultipleInterfaces(this, core::mem::transmute_copy(&pmqis)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Begin_QueryMultipleInterfaces: Begin_QueryMultipleInterfaces::<Identity, OFFSET>,
            Finish_QueryMultipleInterfaces: Finish_QueryMultipleInterfaces::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<AsyncIMultiQI as windows_core::Interface>::IID
    }
}
pub trait AsyncIPipeByte_Impl: Sized {
    fn Begin_Pull(&self, crequest: u32) -> windows_core::Result<()>;
    fn Finish_Pull(&self, buf: *mut u8, pcreturned: *mut u32) -> windows_core::Result<()>;
    fn Begin_Push(&self, buf: *const u8, csent: u32) -> windows_core::Result<()>;
    fn Finish_Push(&self) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for AsyncIPipeByte {}
impl AsyncIPipeByte_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> AsyncIPipeByte_Vtbl
    where
        Identity: AsyncIPipeByte_Impl,
    {
        unsafe extern "system" fn Begin_Pull<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, crequest: u32) -> windows_core::HRESULT
        where
            Identity: AsyncIPipeByte_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            AsyncIPipeByte_Impl::Begin_Pull(this, core::mem::transmute_copy(&crequest)).into()
        }
        unsafe extern "system" fn Finish_Pull<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, buf: *mut u8, pcreturned: *mut u32) -> windows_core::HRESULT
        where
            Identity: AsyncIPipeByte_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            AsyncIPipeByte_Impl::Finish_Pull(this, core::mem::transmute_copy(&buf), core::mem::transmute_copy(&pcreturned)).into()
        }
        unsafe extern "system" fn Begin_Push<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, buf: *const u8, csent: u32) -> windows_core::HRESULT
        where
            Identity: AsyncIPipeByte_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            AsyncIPipeByte_Impl::Begin_Push(this, core::mem::transmute_copy(&buf), core::mem::transmute_copy(&csent)).into()
        }
        unsafe extern "system" fn Finish_Push<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: AsyncIPipeByte_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            AsyncIPipeByte_Impl::Finish_Push(this).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Begin_Pull: Begin_Pull::<Identity, OFFSET>,
            Finish_Pull: Finish_Pull::<Identity, OFFSET>,
            Begin_Push: Begin_Push::<Identity, OFFSET>,
            Finish_Push: Finish_Push::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<AsyncIPipeByte as windows_core::Interface>::IID
    }
}
pub trait AsyncIPipeDouble_Impl: Sized {
    fn Begin_Pull(&self, crequest: u32) -> windows_core::Result<()>;
    fn Finish_Pull(&self, buf: *mut f64, pcreturned: *mut u32) -> windows_core::Result<()>;
    fn Begin_Push(&self, buf: *const f64, csent: u32) -> windows_core::Result<()>;
    fn Finish_Push(&self) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for AsyncIPipeDouble {}
impl AsyncIPipeDouble_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> AsyncIPipeDouble_Vtbl
    where
        Identity: AsyncIPipeDouble_Impl,
    {
        unsafe extern "system" fn Begin_Pull<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, crequest: u32) -> windows_core::HRESULT
        where
            Identity: AsyncIPipeDouble_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            AsyncIPipeDouble_Impl::Begin_Pull(this, core::mem::transmute_copy(&crequest)).into()
        }
        unsafe extern "system" fn Finish_Pull<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, buf: *mut f64, pcreturned: *mut u32) -> windows_core::HRESULT
        where
            Identity: AsyncIPipeDouble_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            AsyncIPipeDouble_Impl::Finish_Pull(this, core::mem::transmute_copy(&buf), core::mem::transmute_copy(&pcreturned)).into()
        }
        unsafe extern "system" fn Begin_Push<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, buf: *const f64, csent: u32) -> windows_core::HRESULT
        where
            Identity: AsyncIPipeDouble_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            AsyncIPipeDouble_Impl::Begin_Push(this, core::mem::transmute_copy(&buf), core::mem::transmute_copy(&csent)).into()
        }
        unsafe extern "system" fn Finish_Push<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: AsyncIPipeDouble_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            AsyncIPipeDouble_Impl::Finish_Push(this).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Begin_Pull: Begin_Pull::<Identity, OFFSET>,
            Finish_Pull: Finish_Pull::<Identity, OFFSET>,
            Begin_Push: Begin_Push::<Identity, OFFSET>,
            Finish_Push: Finish_Push::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<AsyncIPipeDouble as windows_core::Interface>::IID
    }
}
pub trait AsyncIPipeLong_Impl: Sized {
    fn Begin_Pull(&self, crequest: u32) -> windows_core::Result<()>;
    fn Finish_Pull(&self, buf: *mut i32, pcreturned: *mut u32) -> windows_core::Result<()>;
    fn Begin_Push(&self, buf: *const i32, csent: u32) -> windows_core::Result<()>;
    fn Finish_Push(&self) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for AsyncIPipeLong {}
impl AsyncIPipeLong_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> AsyncIPipeLong_Vtbl
    where
        Identity: AsyncIPipeLong_Impl,
    {
        unsafe extern "system" fn Begin_Pull<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, crequest: u32) -> windows_core::HRESULT
        where
            Identity: AsyncIPipeLong_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            AsyncIPipeLong_Impl::Begin_Pull(this, core::mem::transmute_copy(&crequest)).into()
        }
        unsafe extern "system" fn Finish_Pull<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, buf: *mut i32, pcreturned: *mut u32) -> windows_core::HRESULT
        where
            Identity: AsyncIPipeLong_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            AsyncIPipeLong_Impl::Finish_Pull(this, core::mem::transmute_copy(&buf), core::mem::transmute_copy(&pcreturned)).into()
        }
        unsafe extern "system" fn Begin_Push<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, buf: *const i32, csent: u32) -> windows_core::HRESULT
        where
            Identity: AsyncIPipeLong_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            AsyncIPipeLong_Impl::Begin_Push(this, core::mem::transmute_copy(&buf), core::mem::transmute_copy(&csent)).into()
        }
        unsafe extern "system" fn Finish_Push<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: AsyncIPipeLong_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            AsyncIPipeLong_Impl::Finish_Push(this).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Begin_Pull: Begin_Pull::<Identity, OFFSET>,
            Finish_Pull: Finish_Pull::<Identity, OFFSET>,
            Begin_Push: Begin_Push::<Identity, OFFSET>,
            Finish_Push: Finish_Push::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<AsyncIPipeLong as windows_core::Interface>::IID
    }
}
pub trait AsyncIUnknown_Impl: Sized {
    fn Begin_QueryInterface(&self, riid: *const windows_core::GUID) -> windows_core::Result<()>;
    fn Finish_QueryInterface(&self, ppvobject: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
    fn Begin_AddRef(&self) -> windows_core::Result<()>;
    fn Finish_AddRef(&self) -> u32;
    fn Begin_Release(&self) -> windows_core::Result<()>;
    fn Finish_Release(&self) -> u32;
}
impl windows_core::RuntimeName for AsyncIUnknown {}
impl AsyncIUnknown_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> AsyncIUnknown_Vtbl
    where
        Identity: AsyncIUnknown_Impl,
    {
        unsafe extern "system" fn Begin_QueryInterface<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, riid: *const windows_core::GUID) -> windows_core::HRESULT
        where
            Identity: AsyncIUnknown_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            AsyncIUnknown_Impl::Begin_QueryInterface(this, core::mem::transmute_copy(&riid)).into()
        }
        unsafe extern "system" fn Finish_QueryInterface<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppvobject: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: AsyncIUnknown_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            AsyncIUnknown_Impl::Finish_QueryInterface(this, core::mem::transmute_copy(&ppvobject)).into()
        }
        unsafe extern "system" fn Begin_AddRef<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: AsyncIUnknown_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            AsyncIUnknown_Impl::Begin_AddRef(this).into()
        }
        unsafe extern "system" fn Finish_AddRef<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> u32
        where
            Identity: AsyncIUnknown_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            AsyncIUnknown_Impl::Finish_AddRef(this)
        }
        unsafe extern "system" fn Begin_Release<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: AsyncIUnknown_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            AsyncIUnknown_Impl::Begin_Release(this).into()
        }
        unsafe extern "system" fn Finish_Release<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> u32
        where
            Identity: AsyncIUnknown_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            AsyncIUnknown_Impl::Finish_Release(this)
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Begin_QueryInterface: Begin_QueryInterface::<Identity, OFFSET>,
            Finish_QueryInterface: Finish_QueryInterface::<Identity, OFFSET>,
            Begin_AddRef: Begin_AddRef::<Identity, OFFSET>,
            Finish_AddRef: Finish_AddRef::<Identity, OFFSET>,
            Begin_Release: Begin_Release::<Identity, OFFSET>,
            Finish_Release: Finish_Release::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<AsyncIUnknown as windows_core::Interface>::IID
    }
}
pub trait IActivationFilter_Impl: Sized {
    fn HandleActivation(&self, dwactivationtype: u32, rclsid: *const windows_core::GUID) -> windows_core::Result<windows_core::GUID>;
}
impl windows_core::RuntimeName for IActivationFilter {}
impl IActivationFilter_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IActivationFilter_Vtbl
    where
        Identity: IActivationFilter_Impl,
    {
        unsafe extern "system" fn HandleActivation<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwactivationtype: u32, rclsid: *const windows_core::GUID, preplacementclsid: *mut windows_core::GUID) -> windows_core::HRESULT
        where
            Identity: IActivationFilter_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IActivationFilter_Impl::HandleActivation(this, core::mem::transmute_copy(&dwactivationtype), core::mem::transmute_copy(&rclsid)) {
                Ok(ok__) => {
                    preplacementclsid.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), HandleActivation: HandleActivation::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IActivationFilter as windows_core::Interface>::IID
    }
}
pub trait IAddrExclusionControl_Impl: Sized {
    fn GetCurrentAddrExclusionList(&self, riid: *const windows_core::GUID, ppenumerator: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
    fn UpdateAddrExclusionList(&self, penumerator: Option<&windows_core::IUnknown>) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IAddrExclusionControl {}
impl IAddrExclusionControl_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IAddrExclusionControl_Vtbl
    where
        Identity: IAddrExclusionControl_Impl,
    {
        unsafe extern "system" fn GetCurrentAddrExclusionList<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, riid: *const windows_core::GUID, ppenumerator: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IAddrExclusionControl_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IAddrExclusionControl_Impl::GetCurrentAddrExclusionList(this, core::mem::transmute_copy(&riid), core::mem::transmute_copy(&ppenumerator)).into()
        }
        unsafe extern "system" fn UpdateAddrExclusionList<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, penumerator: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IAddrExclusionControl_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IAddrExclusionControl_Impl::UpdateAddrExclusionList(this, windows_core::from_raw_borrowed(&penumerator)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetCurrentAddrExclusionList: GetCurrentAddrExclusionList::<Identity, OFFSET>,
            UpdateAddrExclusionList: UpdateAddrExclusionList::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IAddrExclusionControl as windows_core::Interface>::IID
    }
}
pub trait IAddrTrackingControl_Impl: Sized {
    fn EnableCOMDynamicAddrTracking(&self) -> windows_core::Result<()>;
    fn DisableCOMDynamicAddrTracking(&self) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IAddrTrackingControl {}
impl IAddrTrackingControl_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IAddrTrackingControl_Vtbl
    where
        Identity: IAddrTrackingControl_Impl,
    {
        unsafe extern "system" fn EnableCOMDynamicAddrTracking<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IAddrTrackingControl_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IAddrTrackingControl_Impl::EnableCOMDynamicAddrTracking(this).into()
        }
        unsafe extern "system" fn DisableCOMDynamicAddrTracking<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IAddrTrackingControl_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IAddrTrackingControl_Impl::DisableCOMDynamicAddrTracking(this).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            EnableCOMDynamicAddrTracking: EnableCOMDynamicAddrTracking::<Identity, OFFSET>,
            DisableCOMDynamicAddrTracking: DisableCOMDynamicAddrTracking::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IAddrTrackingControl as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com_StructuredStorage"))]
pub trait IAdviseSink_Impl: Sized {
    fn OnDataChange(&self, pformatetc: *const FORMATETC, pstgmed: *const STGMEDIUM);
    fn OnViewChange(&self, dwaspect: u32, lindex: i32);
    fn OnRename(&self, pmk: Option<&IMoniker>);
    fn OnSave(&self);
    fn OnClose(&self);
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com_StructuredStorage"))]
impl windows_core::RuntimeName for IAdviseSink {}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com_StructuredStorage"))]
impl IAdviseSink_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IAdviseSink_Vtbl
    where
        Identity: IAdviseSink_Impl,
    {
        unsafe extern "system" fn OnDataChange<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pformatetc: *const FORMATETC, pstgmed: *const STGMEDIUM)
        where
            Identity: IAdviseSink_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IAdviseSink_Impl::OnDataChange(this, core::mem::transmute_copy(&pformatetc), core::mem::transmute_copy(&pstgmed))
        }
        unsafe extern "system" fn OnViewChange<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwaspect: u32, lindex: i32)
        where
            Identity: IAdviseSink_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IAdviseSink_Impl::OnViewChange(this, core::mem::transmute_copy(&dwaspect), core::mem::transmute_copy(&lindex))
        }
        unsafe extern "system" fn OnRename<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pmk: *mut core::ffi::c_void)
        where
            Identity: IAdviseSink_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IAdviseSink_Impl::OnRename(this, windows_core::from_raw_borrowed(&pmk))
        }
        unsafe extern "system" fn OnSave<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void)
        where
            Identity: IAdviseSink_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IAdviseSink_Impl::OnSave(this)
        }
        unsafe extern "system" fn OnClose<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void)
        where
            Identity: IAdviseSink_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IAdviseSink_Impl::OnClose(this)
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            OnDataChange: OnDataChange::<Identity, OFFSET>,
            OnViewChange: OnViewChange::<Identity, OFFSET>,
            OnRename: OnRename::<Identity, OFFSET>,
            OnSave: OnSave::<Identity, OFFSET>,
            OnClose: OnClose::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IAdviseSink as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com_StructuredStorage"))]
pub trait IAdviseSink2_Impl: Sized + IAdviseSink_Impl {
    fn OnLinkSrcChange(&self, pmk: Option<&IMoniker>);
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com_StructuredStorage"))]
impl windows_core::RuntimeName for IAdviseSink2 {}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com_StructuredStorage"))]
impl IAdviseSink2_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IAdviseSink2_Vtbl
    where
        Identity: IAdviseSink2_Impl,
    {
        unsafe extern "system" fn OnLinkSrcChange<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pmk: *mut core::ffi::c_void)
        where
            Identity: IAdviseSink2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IAdviseSink2_Impl::OnLinkSrcChange(this, windows_core::from_raw_borrowed(&pmk))
        }
        Self { base__: IAdviseSink_Vtbl::new::<Identity, OFFSET>(), OnLinkSrcChange: OnLinkSrcChange::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IAdviseSink2 as windows_core::Interface>::IID || iid == &<IAdviseSink as windows_core::Interface>::IID
    }
}
pub trait IAgileObject_Impl: Sized {}
impl windows_core::RuntimeName for IAgileObject {}
impl IAgileObject_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IAgileObject_Vtbl
    where
        Identity: IAgileObject_Impl,
    {
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>() }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IAgileObject as windows_core::Interface>::IID
    }
}
pub trait IAsyncManager_Impl: Sized {
    fn CompleteCall(&self, result: windows_core::HRESULT) -> windows_core::Result<()>;
    fn GetCallContext(&self, riid: *const windows_core::GUID, pinterface: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
    fn GetState(&self) -> windows_core::Result<u32>;
}
impl windows_core::RuntimeName for IAsyncManager {}
impl IAsyncManager_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IAsyncManager_Vtbl
    where
        Identity: IAsyncManager_Impl,
    {
        unsafe extern "system" fn CompleteCall<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, result: windows_core::HRESULT) -> windows_core::HRESULT
        where
            Identity: IAsyncManager_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IAsyncManager_Impl::CompleteCall(this, core::mem::transmute_copy(&result)).into()
        }
        unsafe extern "system" fn GetCallContext<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, riid: *const windows_core::GUID, pinterface: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IAsyncManager_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IAsyncManager_Impl::GetCallContext(this, core::mem::transmute_copy(&riid), core::mem::transmute_copy(&pinterface)).into()
        }
        unsafe extern "system" fn GetState<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pulstateflags: *mut u32) -> windows_core::HRESULT
        where
            Identity: IAsyncManager_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IAsyncManager_Impl::GetState(this) {
                Ok(ok__) => {
                    pulstateflags.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            CompleteCall: CompleteCall::<Identity, OFFSET>,
            GetCallContext: GetCallContext::<Identity, OFFSET>,
            GetState: GetState::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IAsyncManager as windows_core::Interface>::IID
    }
}
pub trait IAsyncRpcChannelBuffer_Impl: Sized + IRpcChannelBuffer2_Impl {
    fn Send(&self, pmsg: *mut RPCOLEMESSAGE, psync: Option<&ISynchronize>, pulstatus: *mut u32) -> windows_core::Result<()>;
    fn Receive(&self, pmsg: *mut RPCOLEMESSAGE, pulstatus: *mut u32) -> windows_core::Result<()>;
    fn GetDestCtxEx(&self, pmsg: *const RPCOLEMESSAGE, pdwdestcontext: *mut u32, ppvdestcontext: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IAsyncRpcChannelBuffer {}
impl IAsyncRpcChannelBuffer_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IAsyncRpcChannelBuffer_Vtbl
    where
        Identity: IAsyncRpcChannelBuffer_Impl,
    {
        unsafe extern "system" fn Send<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pmsg: *mut RPCOLEMESSAGE, psync: *mut core::ffi::c_void, pulstatus: *mut u32) -> windows_core::HRESULT
        where
            Identity: IAsyncRpcChannelBuffer_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IAsyncRpcChannelBuffer_Impl::Send(this, core::mem::transmute_copy(&pmsg), windows_core::from_raw_borrowed(&psync), core::mem::transmute_copy(&pulstatus)).into()
        }
        unsafe extern "system" fn Receive<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pmsg: *mut RPCOLEMESSAGE, pulstatus: *mut u32) -> windows_core::HRESULT
        where
            Identity: IAsyncRpcChannelBuffer_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IAsyncRpcChannelBuffer_Impl::Receive(this, core::mem::transmute_copy(&pmsg), core::mem::transmute_copy(&pulstatus)).into()
        }
        unsafe extern "system" fn GetDestCtxEx<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pmsg: *const RPCOLEMESSAGE, pdwdestcontext: *mut u32, ppvdestcontext: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IAsyncRpcChannelBuffer_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IAsyncRpcChannelBuffer_Impl::GetDestCtxEx(this, core::mem::transmute_copy(&pmsg), core::mem::transmute_copy(&pdwdestcontext), core::mem::transmute_copy(&ppvdestcontext)).into()
        }
        Self {
            base__: IRpcChannelBuffer2_Vtbl::new::<Identity, OFFSET>(),
            Send: Send::<Identity, OFFSET>,
            Receive: Receive::<Identity, OFFSET>,
            GetDestCtxEx: GetDestCtxEx::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IAsyncRpcChannelBuffer as windows_core::Interface>::IID || iid == &<IRpcChannelBuffer as windows_core::Interface>::IID || iid == &<IRpcChannelBuffer2 as windows_core::Interface>::IID
    }
}
pub trait IAuthenticate_Impl: Sized {
    fn Authenticate(&self, phwnd: *mut super::super::Foundation::HWND, pszusername: *mut windows_core::PWSTR, pszpassword: *mut windows_core::PWSTR) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IAuthenticate {}
impl IAuthenticate_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IAuthenticate_Vtbl
    where
        Identity: IAuthenticate_Impl,
    {
        unsafe extern "system" fn Authenticate<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, phwnd: *mut super::super::Foundation::HWND, pszusername: *mut windows_core::PWSTR, pszpassword: *mut windows_core::PWSTR) -> windows_core::HRESULT
        where
            Identity: IAuthenticate_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IAuthenticate_Impl::Authenticate(this, core::mem::transmute_copy(&phwnd), core::mem::transmute_copy(&pszusername), core::mem::transmute_copy(&pszpassword)).into()
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), Authenticate: Authenticate::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IAuthenticate as windows_core::Interface>::IID
    }
}
pub trait IAuthenticateEx_Impl: Sized + IAuthenticate_Impl {
    fn AuthenticateEx(&self, phwnd: *mut super::super::Foundation::HWND, pszusername: *mut windows_core::PWSTR, pszpassword: *mut windows_core::PWSTR, pauthinfo: *const AUTHENTICATEINFO) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IAuthenticateEx {}
impl IAuthenticateEx_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IAuthenticateEx_Vtbl
    where
        Identity: IAuthenticateEx_Impl,
    {
        unsafe extern "system" fn AuthenticateEx<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, phwnd: *mut super::super::Foundation::HWND, pszusername: *mut windows_core::PWSTR, pszpassword: *mut windows_core::PWSTR, pauthinfo: *const AUTHENTICATEINFO) -> windows_core::HRESULT
        where
            Identity: IAuthenticateEx_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IAuthenticateEx_Impl::AuthenticateEx(this, core::mem::transmute_copy(&phwnd), core::mem::transmute_copy(&pszusername), core::mem::transmute_copy(&pszpassword), core::mem::transmute_copy(&pauthinfo)).into()
        }
        Self { base__: IAuthenticate_Vtbl::new::<Identity, OFFSET>(), AuthenticateEx: AuthenticateEx::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IAuthenticateEx as windows_core::Interface>::IID || iid == &<IAuthenticate as windows_core::Interface>::IID
    }
}
pub trait IBindCtx_Impl: Sized {
    fn RegisterObjectBound(&self, punk: Option<&windows_core::IUnknown>) -> windows_core::Result<()>;
    fn RevokeObjectBound(&self, punk: Option<&windows_core::IUnknown>) -> windows_core::Result<()>;
    fn ReleaseBoundObjects(&self) -> windows_core::Result<()>;
    fn SetBindOptions(&self, pbindopts: *const BIND_OPTS) -> windows_core::Result<()>;
    fn GetBindOptions(&self, pbindopts: *mut BIND_OPTS) -> windows_core::Result<()>;
    fn GetRunningObjectTable(&self) -> windows_core::Result<IRunningObjectTable>;
    fn RegisterObjectParam(&self, pszkey: &windows_core::PCWSTR, punk: Option<&windows_core::IUnknown>) -> windows_core::Result<()>;
    fn GetObjectParam(&self, pszkey: &windows_core::PCWSTR) -> windows_core::Result<windows_core::IUnknown>;
    fn EnumObjectParam(&self) -> windows_core::Result<IEnumString>;
    fn RevokeObjectParam(&self, pszkey: &windows_core::PCWSTR) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IBindCtx {}
impl IBindCtx_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IBindCtx_Vtbl
    where
        Identity: IBindCtx_Impl,
    {
        unsafe extern "system" fn RegisterObjectBound<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, punk: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IBindCtx_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IBindCtx_Impl::RegisterObjectBound(this, windows_core::from_raw_borrowed(&punk)).into()
        }
        unsafe extern "system" fn RevokeObjectBound<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, punk: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IBindCtx_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IBindCtx_Impl::RevokeObjectBound(this, windows_core::from_raw_borrowed(&punk)).into()
        }
        unsafe extern "system" fn ReleaseBoundObjects<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IBindCtx_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IBindCtx_Impl::ReleaseBoundObjects(this).into()
        }
        unsafe extern "system" fn SetBindOptions<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbindopts: *const BIND_OPTS) -> windows_core::HRESULT
        where
            Identity: IBindCtx_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IBindCtx_Impl::SetBindOptions(this, core::mem::transmute_copy(&pbindopts)).into()
        }
        unsafe extern "system" fn GetBindOptions<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbindopts: *mut BIND_OPTS) -> windows_core::HRESULT
        where
            Identity: IBindCtx_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IBindCtx_Impl::GetBindOptions(this, core::mem::transmute_copy(&pbindopts)).into()
        }
        unsafe extern "system" fn GetRunningObjectTable<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pprot: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IBindCtx_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IBindCtx_Impl::GetRunningObjectTable(this) {
                Ok(ok__) => {
                    pprot.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RegisterObjectParam<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszkey: windows_core::PCWSTR, punk: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IBindCtx_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IBindCtx_Impl::RegisterObjectParam(this, core::mem::transmute(&pszkey), windows_core::from_raw_borrowed(&punk)).into()
        }
        unsafe extern "system" fn GetObjectParam<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszkey: windows_core::PCWSTR, ppunk: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IBindCtx_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IBindCtx_Impl::GetObjectParam(this, core::mem::transmute(&pszkey)) {
                Ok(ok__) => {
                    ppunk.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnumObjectParam<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppenum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IBindCtx_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IBindCtx_Impl::EnumObjectParam(this) {
                Ok(ok__) => {
                    ppenum.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RevokeObjectParam<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszkey: windows_core::PCWSTR) -> windows_core::HRESULT
        where
            Identity: IBindCtx_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IBindCtx_Impl::RevokeObjectParam(this, core::mem::transmute(&pszkey)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            RegisterObjectBound: RegisterObjectBound::<Identity, OFFSET>,
            RevokeObjectBound: RevokeObjectBound::<Identity, OFFSET>,
            ReleaseBoundObjects: ReleaseBoundObjects::<Identity, OFFSET>,
            SetBindOptions: SetBindOptions::<Identity, OFFSET>,
            GetBindOptions: GetBindOptions::<Identity, OFFSET>,
            GetRunningObjectTable: GetRunningObjectTable::<Identity, OFFSET>,
            RegisterObjectParam: RegisterObjectParam::<Identity, OFFSET>,
            GetObjectParam: GetObjectParam::<Identity, OFFSET>,
            EnumObjectParam: EnumObjectParam::<Identity, OFFSET>,
            RevokeObjectParam: RevokeObjectParam::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IBindCtx as windows_core::Interface>::IID
    }
}
pub trait IBindHost_Impl: Sized {
    fn CreateMoniker(&self, szname: &windows_core::PCWSTR, pbc: Option<&IBindCtx>, ppmk: *mut Option<IMoniker>, dwreserved: u32) -> windows_core::Result<()>;
    fn MonikerBindToStorage(&self, pmk: Option<&IMoniker>, pbc: Option<&IBindCtx>, pbsc: Option<&IBindStatusCallback>, riid: *const windows_core::GUID, ppvobj: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
    fn MonikerBindToObject(&self, pmk: Option<&IMoniker>, pbc: Option<&IBindCtx>, pbsc: Option<&IBindStatusCallback>, riid: *const windows_core::GUID, ppvobj: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IBindHost {}
impl IBindHost_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IBindHost_Vtbl
    where
        Identity: IBindHost_Impl,
    {
        unsafe extern "system" fn CreateMoniker<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, szname: windows_core::PCWSTR, pbc: *mut core::ffi::c_void, ppmk: *mut *mut core::ffi::c_void, dwreserved: u32) -> windows_core::HRESULT
        where
            Identity: IBindHost_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IBindHost_Impl::CreateMoniker(this, core::mem::transmute(&szname), windows_core::from_raw_borrowed(&pbc), core::mem::transmute_copy(&ppmk), core::mem::transmute_copy(&dwreserved)).into()
        }
        unsafe extern "system" fn MonikerBindToStorage<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pmk: *mut core::ffi::c_void, pbc: *mut core::ffi::c_void, pbsc: *mut core::ffi::c_void, riid: *const windows_core::GUID, ppvobj: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IBindHost_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IBindHost_Impl::MonikerBindToStorage(this, windows_core::from_raw_borrowed(&pmk), windows_core::from_raw_borrowed(&pbc), windows_core::from_raw_borrowed(&pbsc), core::mem::transmute_copy(&riid), core::mem::transmute_copy(&ppvobj)).into()
        }
        unsafe extern "system" fn MonikerBindToObject<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pmk: *mut core::ffi::c_void, pbc: *mut core::ffi::c_void, pbsc: *mut core::ffi::c_void, riid: *const windows_core::GUID, ppvobj: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IBindHost_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IBindHost_Impl::MonikerBindToObject(this, windows_core::from_raw_borrowed(&pmk), windows_core::from_raw_borrowed(&pbc), windows_core::from_raw_borrowed(&pbsc), core::mem::transmute_copy(&riid), core::mem::transmute_copy(&ppvobj)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            CreateMoniker: CreateMoniker::<Identity, OFFSET>,
            MonikerBindToStorage: MonikerBindToStorage::<Identity, OFFSET>,
            MonikerBindToObject: MonikerBindToObject::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IBindHost as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_Security", feature = "Win32_System_Com_StructuredStorage"))]
pub trait IBindStatusCallback_Impl: Sized {
    fn OnStartBinding(&self, dwreserved: u32, pib: Option<&IBinding>) -> windows_core::Result<()>;
    fn GetPriority(&self) -> windows_core::Result<i32>;
    fn OnLowResource(&self, reserved: u32) -> windows_core::Result<()>;
    fn OnProgress(&self, ulprogress: u32, ulprogressmax: u32, ulstatuscode: u32, szstatustext: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn OnStopBinding(&self, hresult: windows_core::HRESULT, szerror: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn GetBindInfo(&self, grfbindf: *mut u32, pbindinfo: *mut BINDINFO) -> windows_core::Result<()>;
    fn OnDataAvailable(&self, grfbscf: u32, dwsize: u32, pformatetc: *const FORMATETC, pstgmed: *const STGMEDIUM) -> windows_core::Result<()>;
    fn OnObjectAvailable(&self, riid: *const windows_core::GUID, punk: Option<&windows_core::IUnknown>) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_Security", feature = "Win32_System_Com_StructuredStorage"))]
impl windows_core::RuntimeName for IBindStatusCallback {}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_Security", feature = "Win32_System_Com_StructuredStorage"))]
impl IBindStatusCallback_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IBindStatusCallback_Vtbl
    where
        Identity: IBindStatusCallback_Impl,
    {
        unsafe extern "system" fn OnStartBinding<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwreserved: u32, pib: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IBindStatusCallback_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IBindStatusCallback_Impl::OnStartBinding(this, core::mem::transmute_copy(&dwreserved), windows_core::from_raw_borrowed(&pib)).into()
        }
        unsafe extern "system" fn GetPriority<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pnpriority: *mut i32) -> windows_core::HRESULT
        where
            Identity: IBindStatusCallback_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IBindStatusCallback_Impl::GetPriority(this) {
                Ok(ok__) => {
                    pnpriority.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OnLowResource<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, reserved: u32) -> windows_core::HRESULT
        where
            Identity: IBindStatusCallback_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IBindStatusCallback_Impl::OnLowResource(this, core::mem::transmute_copy(&reserved)).into()
        }
        unsafe extern "system" fn OnProgress<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ulprogress: u32, ulprogressmax: u32, ulstatuscode: u32, szstatustext: windows_core::PCWSTR) -> windows_core::HRESULT
        where
            Identity: IBindStatusCallback_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IBindStatusCallback_Impl::OnProgress(this, core::mem::transmute_copy(&ulprogress), core::mem::transmute_copy(&ulprogressmax), core::mem::transmute_copy(&ulstatuscode), core::mem::transmute(&szstatustext)).into()
        }
        unsafe extern "system" fn OnStopBinding<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, hresult: windows_core::HRESULT, szerror: windows_core::PCWSTR) -> windows_core::HRESULT
        where
            Identity: IBindStatusCallback_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IBindStatusCallback_Impl::OnStopBinding(this, core::mem::transmute_copy(&hresult), core::mem::transmute(&szerror)).into()
        }
        unsafe extern "system" fn GetBindInfo<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, grfbindf: *mut u32, pbindinfo: *mut BINDINFO) -> windows_core::HRESULT
        where
            Identity: IBindStatusCallback_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IBindStatusCallback_Impl::GetBindInfo(this, core::mem::transmute_copy(&grfbindf), core::mem::transmute_copy(&pbindinfo)).into()
        }
        unsafe extern "system" fn OnDataAvailable<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, grfbscf: u32, dwsize: u32, pformatetc: *const FORMATETC, pstgmed: *const STGMEDIUM) -> windows_core::HRESULT
        where
            Identity: IBindStatusCallback_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IBindStatusCallback_Impl::OnDataAvailable(this, core::mem::transmute_copy(&grfbscf), core::mem::transmute_copy(&dwsize), core::mem::transmute_copy(&pformatetc), core::mem::transmute_copy(&pstgmed)).into()
        }
        unsafe extern "system" fn OnObjectAvailable<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, riid: *const windows_core::GUID, punk: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IBindStatusCallback_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IBindStatusCallback_Impl::OnObjectAvailable(this, core::mem::transmute_copy(&riid), windows_core::from_raw_borrowed(&punk)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            OnStartBinding: OnStartBinding::<Identity, OFFSET>,
            GetPriority: GetPriority::<Identity, OFFSET>,
            OnLowResource: OnLowResource::<Identity, OFFSET>,
            OnProgress: OnProgress::<Identity, OFFSET>,
            OnStopBinding: OnStopBinding::<Identity, OFFSET>,
            GetBindInfo: GetBindInfo::<Identity, OFFSET>,
            OnDataAvailable: OnDataAvailable::<Identity, OFFSET>,
            OnObjectAvailable: OnObjectAvailable::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IBindStatusCallback as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_Security", feature = "Win32_System_Com_StructuredStorage"))]
pub trait IBindStatusCallbackEx_Impl: Sized + IBindStatusCallback_Impl {
    fn GetBindInfoEx(&self, grfbindf: *mut u32, pbindinfo: *mut BINDINFO, grfbindf2: *mut u32, pdwreserved: *mut u32) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_Security", feature = "Win32_System_Com_StructuredStorage"))]
impl windows_core::RuntimeName for IBindStatusCallbackEx {}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_Security", feature = "Win32_System_Com_StructuredStorage"))]
impl IBindStatusCallbackEx_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IBindStatusCallbackEx_Vtbl
    where
        Identity: IBindStatusCallbackEx_Impl,
    {
        unsafe extern "system" fn GetBindInfoEx<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, grfbindf: *mut u32, pbindinfo: *mut BINDINFO, grfbindf2: *mut u32, pdwreserved: *mut u32) -> windows_core::HRESULT
        where
            Identity: IBindStatusCallbackEx_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IBindStatusCallbackEx_Impl::GetBindInfoEx(this, core::mem::transmute_copy(&grfbindf), core::mem::transmute_copy(&pbindinfo), core::mem::transmute_copy(&grfbindf2), core::mem::transmute_copy(&pdwreserved)).into()
        }
        Self { base__: IBindStatusCallback_Vtbl::new::<Identity, OFFSET>(), GetBindInfoEx: GetBindInfoEx::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IBindStatusCallbackEx as windows_core::Interface>::IID || iid == &<IBindStatusCallback as windows_core::Interface>::IID
    }
}
pub trait IBinding_Impl: Sized {
    fn Abort(&self) -> windows_core::Result<()>;
    fn Suspend(&self) -> windows_core::Result<()>;
    fn Resume(&self) -> windows_core::Result<()>;
    fn SetPriority(&self, npriority: i32) -> windows_core::Result<()>;
    fn GetPriority(&self) -> windows_core::Result<i32>;
    fn GetBindResult(&self, pclsidprotocol: *mut windows_core::GUID, pdwresult: *mut u32, pszresult: *mut windows_core::PWSTR, pdwreserved: *mut u32) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IBinding {}
impl IBinding_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IBinding_Vtbl
    where
        Identity: IBinding_Impl,
    {
        unsafe extern "system" fn Abort<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IBinding_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IBinding_Impl::Abort(this).into()
        }
        unsafe extern "system" fn Suspend<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IBinding_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IBinding_Impl::Suspend(this).into()
        }
        unsafe extern "system" fn Resume<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IBinding_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IBinding_Impl::Resume(this).into()
        }
        unsafe extern "system" fn SetPriority<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, npriority: i32) -> windows_core::HRESULT
        where
            Identity: IBinding_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IBinding_Impl::SetPriority(this, core::mem::transmute_copy(&npriority)).into()
        }
        unsafe extern "system" fn GetPriority<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pnpriority: *mut i32) -> windows_core::HRESULT
        where
            Identity: IBinding_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IBinding_Impl::GetPriority(this) {
                Ok(ok__) => {
                    pnpriority.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetBindResult<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pclsidprotocol: *mut windows_core::GUID, pdwresult: *mut u32, pszresult: *mut windows_core::PWSTR, pdwreserved: *mut u32) -> windows_core::HRESULT
        where
            Identity: IBinding_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IBinding_Impl::GetBindResult(this, core::mem::transmute_copy(&pclsidprotocol), core::mem::transmute_copy(&pdwresult), core::mem::transmute_copy(&pszresult), core::mem::transmute_copy(&pdwreserved)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Abort: Abort::<Identity, OFFSET>,
            Suspend: Suspend::<Identity, OFFSET>,
            Resume: Resume::<Identity, OFFSET>,
            SetPriority: SetPriority::<Identity, OFFSET>,
            GetPriority: GetPriority::<Identity, OFFSET>,
            GetBindResult: GetBindResult::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IBinding as windows_core::Interface>::IID
    }
}
pub trait IBlockingLock_Impl: Sized {
    fn Lock(&self, dwtimeout: u32) -> windows_core::Result<()>;
    fn Unlock(&self) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IBlockingLock {}
impl IBlockingLock_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IBlockingLock_Vtbl
    where
        Identity: IBlockingLock_Impl,
    {
        unsafe extern "system" fn Lock<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwtimeout: u32) -> windows_core::HRESULT
        where
            Identity: IBlockingLock_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IBlockingLock_Impl::Lock(this, core::mem::transmute_copy(&dwtimeout)).into()
        }
        unsafe extern "system" fn Unlock<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IBlockingLock_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IBlockingLock_Impl::Unlock(this).into()
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), Lock: Lock::<Identity, OFFSET>, Unlock: Unlock::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IBlockingLock as windows_core::Interface>::IID
    }
}
pub trait ICallFactory_Impl: Sized {
    fn CreateCall(&self, riid: *const windows_core::GUID, pctrlunk: Option<&windows_core::IUnknown>, riid2: *const windows_core::GUID) -> windows_core::Result<windows_core::IUnknown>;
}
impl windows_core::RuntimeName for ICallFactory {}
impl ICallFactory_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ICallFactory_Vtbl
    where
        Identity: ICallFactory_Impl,
    {
        unsafe extern "system" fn CreateCall<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, riid: *const windows_core::GUID, pctrlunk: *mut core::ffi::c_void, riid2: *const windows_core::GUID, ppv: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ICallFactory_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ICallFactory_Impl::CreateCall(this, core::mem::transmute_copy(&riid), windows_core::from_raw_borrowed(&pctrlunk), core::mem::transmute_copy(&riid2)) {
                Ok(ok__) => {
                    ppv.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), CreateCall: CreateCall::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ICallFactory as windows_core::Interface>::IID
    }
}
pub trait ICancelMethodCalls_Impl: Sized {
    fn Cancel(&self, ulseconds: u32) -> windows_core::Result<()>;
    fn TestCancel(&self) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for ICancelMethodCalls {}
impl ICancelMethodCalls_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ICancelMethodCalls_Vtbl
    where
        Identity: ICancelMethodCalls_Impl,
    {
        unsafe extern "system" fn Cancel<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ulseconds: u32) -> windows_core::HRESULT
        where
            Identity: ICancelMethodCalls_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICancelMethodCalls_Impl::Cancel(this, core::mem::transmute_copy(&ulseconds)).into()
        }
        unsafe extern "system" fn TestCancel<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ICancelMethodCalls_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICancelMethodCalls_Impl::TestCancel(this).into()
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), Cancel: Cancel::<Identity, OFFSET>, TestCancel: TestCancel::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ICancelMethodCalls as windows_core::Interface>::IID
    }
}
pub trait ICatInformation_Impl: Sized {
    fn EnumCategories(&self, lcid: u32) -> windows_core::Result<IEnumCATEGORYINFO>;
    fn GetCategoryDesc(&self, rcatid: *const windows_core::GUID, lcid: u32) -> windows_core::Result<windows_core::PWSTR>;
    fn EnumClassesOfCategories(&self, cimplemented: u32, rgcatidimpl: *const windows_core::GUID, crequired: u32, rgcatidreq: *const windows_core::GUID) -> windows_core::Result<IEnumGUID>;
    fn IsClassOfCategories(&self, rclsid: *const windows_core::GUID, cimplemented: u32, rgcatidimpl: *const windows_core::GUID, crequired: u32, rgcatidreq: *const windows_core::GUID) -> windows_core::Result<()>;
    fn EnumImplCategoriesOfClass(&self, rclsid: *const windows_core::GUID) -> windows_core::Result<IEnumGUID>;
    fn EnumReqCategoriesOfClass(&self, rclsid: *const windows_core::GUID) -> windows_core::Result<IEnumGUID>;
}
impl windows_core::RuntimeName for ICatInformation {}
impl ICatInformation_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ICatInformation_Vtbl
    where
        Identity: ICatInformation_Impl,
    {
        unsafe extern "system" fn EnumCategories<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, lcid: u32, ppenumcategoryinfo: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ICatInformation_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ICatInformation_Impl::EnumCategories(this, core::mem::transmute_copy(&lcid)) {
                Ok(ok__) => {
                    ppenumcategoryinfo.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCategoryDesc<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, rcatid: *const windows_core::GUID, lcid: u32, pszdesc: *mut windows_core::PWSTR) -> windows_core::HRESULT
        where
            Identity: ICatInformation_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ICatInformation_Impl::GetCategoryDesc(this, core::mem::transmute_copy(&rcatid), core::mem::transmute_copy(&lcid)) {
                Ok(ok__) => {
                    pszdesc.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnumClassesOfCategories<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, cimplemented: u32, rgcatidimpl: *const windows_core::GUID, crequired: u32, rgcatidreq: *const windows_core::GUID, ppenumclsid: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ICatInformation_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ICatInformation_Impl::EnumClassesOfCategories(this, core::mem::transmute_copy(&cimplemented), core::mem::transmute_copy(&rgcatidimpl), core::mem::transmute_copy(&crequired), core::mem::transmute_copy(&rgcatidreq)) {
                Ok(ok__) => {
                    ppenumclsid.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsClassOfCategories<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, rclsid: *const windows_core::GUID, cimplemented: u32, rgcatidimpl: *const windows_core::GUID, crequired: u32, rgcatidreq: *const windows_core::GUID) -> windows_core::HRESULT
        where
            Identity: ICatInformation_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICatInformation_Impl::IsClassOfCategories(this, core::mem::transmute_copy(&rclsid), core::mem::transmute_copy(&cimplemented), core::mem::transmute_copy(&rgcatidimpl), core::mem::transmute_copy(&crequired), core::mem::transmute_copy(&rgcatidreq)).into()
        }
        unsafe extern "system" fn EnumImplCategoriesOfClass<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, rclsid: *const windows_core::GUID, ppenumcatid: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ICatInformation_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ICatInformation_Impl::EnumImplCategoriesOfClass(this, core::mem::transmute_copy(&rclsid)) {
                Ok(ok__) => {
                    ppenumcatid.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnumReqCategoriesOfClass<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, rclsid: *const windows_core::GUID, ppenumcatid: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ICatInformation_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ICatInformation_Impl::EnumReqCategoriesOfClass(this, core::mem::transmute_copy(&rclsid)) {
                Ok(ok__) => {
                    ppenumcatid.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            EnumCategories: EnumCategories::<Identity, OFFSET>,
            GetCategoryDesc: GetCategoryDesc::<Identity, OFFSET>,
            EnumClassesOfCategories: EnumClassesOfCategories::<Identity, OFFSET>,
            IsClassOfCategories: IsClassOfCategories::<Identity, OFFSET>,
            EnumImplCategoriesOfClass: EnumImplCategoriesOfClass::<Identity, OFFSET>,
            EnumReqCategoriesOfClass: EnumReqCategoriesOfClass::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ICatInformation as windows_core::Interface>::IID
    }
}
pub trait ICatRegister_Impl: Sized {
    fn RegisterCategories(&self, ccategories: u32, rgcategoryinfo: *const CATEGORYINFO) -> windows_core::Result<()>;
    fn UnRegisterCategories(&self, ccategories: u32, rgcatid: *const windows_core::GUID) -> windows_core::Result<()>;
    fn RegisterClassImplCategories(&self, rclsid: *const windows_core::GUID, ccategories: u32, rgcatid: *const windows_core::GUID) -> windows_core::Result<()>;
    fn UnRegisterClassImplCategories(&self, rclsid: *const windows_core::GUID, ccategories: u32, rgcatid: *const windows_core::GUID) -> windows_core::Result<()>;
    fn RegisterClassReqCategories(&self, rclsid: *const windows_core::GUID, ccategories: u32, rgcatid: *const windows_core::GUID) -> windows_core::Result<()>;
    fn UnRegisterClassReqCategories(&self, rclsid: *const windows_core::GUID, ccategories: u32, rgcatid: *const windows_core::GUID) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for ICatRegister {}
impl ICatRegister_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ICatRegister_Vtbl
    where
        Identity: ICatRegister_Impl,
    {
        unsafe extern "system" fn RegisterCategories<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ccategories: u32, rgcategoryinfo: *const CATEGORYINFO) -> windows_core::HRESULT
        where
            Identity: ICatRegister_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICatRegister_Impl::RegisterCategories(this, core::mem::transmute_copy(&ccategories), core::mem::transmute_copy(&rgcategoryinfo)).into()
        }
        unsafe extern "system" fn UnRegisterCategories<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ccategories: u32, rgcatid: *const windows_core::GUID) -> windows_core::HRESULT
        where
            Identity: ICatRegister_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICatRegister_Impl::UnRegisterCategories(this, core::mem::transmute_copy(&ccategories), core::mem::transmute_copy(&rgcatid)).into()
        }
        unsafe extern "system" fn RegisterClassImplCategories<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, rclsid: *const windows_core::GUID, ccategories: u32, rgcatid: *const windows_core::GUID) -> windows_core::HRESULT
        where
            Identity: ICatRegister_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICatRegister_Impl::RegisterClassImplCategories(this, core::mem::transmute_copy(&rclsid), core::mem::transmute_copy(&ccategories), core::mem::transmute_copy(&rgcatid)).into()
        }
        unsafe extern "system" fn UnRegisterClassImplCategories<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, rclsid: *const windows_core::GUID, ccategories: u32, rgcatid: *const windows_core::GUID) -> windows_core::HRESULT
        where
            Identity: ICatRegister_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICatRegister_Impl::UnRegisterClassImplCategories(this, core::mem::transmute_copy(&rclsid), core::mem::transmute_copy(&ccategories), core::mem::transmute_copy(&rgcatid)).into()
        }
        unsafe extern "system" fn RegisterClassReqCategories<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, rclsid: *const windows_core::GUID, ccategories: u32, rgcatid: *const windows_core::GUID) -> windows_core::HRESULT
        where
            Identity: ICatRegister_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICatRegister_Impl::RegisterClassReqCategories(this, core::mem::transmute_copy(&rclsid), core::mem::transmute_copy(&ccategories), core::mem::transmute_copy(&rgcatid)).into()
        }
        unsafe extern "system" fn UnRegisterClassReqCategories<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, rclsid: *const windows_core::GUID, ccategories: u32, rgcatid: *const windows_core::GUID) -> windows_core::HRESULT
        where
            Identity: ICatRegister_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICatRegister_Impl::UnRegisterClassReqCategories(this, core::mem::transmute_copy(&rclsid), core::mem::transmute_copy(&ccategories), core::mem::transmute_copy(&rgcatid)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            RegisterCategories: RegisterCategories::<Identity, OFFSET>,
            UnRegisterCategories: UnRegisterCategories::<Identity, OFFSET>,
            RegisterClassImplCategories: RegisterClassImplCategories::<Identity, OFFSET>,
            UnRegisterClassImplCategories: UnRegisterClassImplCategories::<Identity, OFFSET>,
            RegisterClassReqCategories: RegisterClassReqCategories::<Identity, OFFSET>,
            UnRegisterClassReqCategories: UnRegisterClassReqCategories::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ICatRegister as windows_core::Interface>::IID
    }
}
pub trait IChannelHook_Impl: Sized {
    fn ClientGetSize(&self, uextent: *const windows_core::GUID, riid: *const windows_core::GUID, pdatasize: *mut u32);
    fn ClientFillBuffer(&self, uextent: *const windows_core::GUID, riid: *const windows_core::GUID, pdatasize: *mut u32, pdatabuffer: *const core::ffi::c_void);
    fn ClientNotify(&self, uextent: *const windows_core::GUID, riid: *const windows_core::GUID, cbdatasize: u32, pdatabuffer: *const core::ffi::c_void, ldatarep: u32, hrfault: windows_core::HRESULT);
    fn ServerNotify(&self, uextent: *const windows_core::GUID, riid: *const windows_core::GUID, cbdatasize: u32, pdatabuffer: *const core::ffi::c_void, ldatarep: u32);
    fn ServerGetSize(&self, uextent: *const windows_core::GUID, riid: *const windows_core::GUID, hrfault: windows_core::HRESULT, pdatasize: *mut u32);
    fn ServerFillBuffer(&self, uextent: *const windows_core::GUID, riid: *const windows_core::GUID, pdatasize: *mut u32, pdatabuffer: *const core::ffi::c_void, hrfault: windows_core::HRESULT);
}
impl windows_core::RuntimeName for IChannelHook {}
impl IChannelHook_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IChannelHook_Vtbl
    where
        Identity: IChannelHook_Impl,
    {
        unsafe extern "system" fn ClientGetSize<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, uextent: *const windows_core::GUID, riid: *const windows_core::GUID, pdatasize: *mut u32)
        where
            Identity: IChannelHook_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IChannelHook_Impl::ClientGetSize(this, core::mem::transmute_copy(&uextent), core::mem::transmute_copy(&riid), core::mem::transmute_copy(&pdatasize))
        }
        unsafe extern "system" fn ClientFillBuffer<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, uextent: *const windows_core::GUID, riid: *const windows_core::GUID, pdatasize: *mut u32, pdatabuffer: *const core::ffi::c_void)
        where
            Identity: IChannelHook_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IChannelHook_Impl::ClientFillBuffer(this, core::mem::transmute_copy(&uextent), core::mem::transmute_copy(&riid), core::mem::transmute_copy(&pdatasize), core::mem::transmute_copy(&pdatabuffer))
        }
        unsafe extern "system" fn ClientNotify<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, uextent: *const windows_core::GUID, riid: *const windows_core::GUID, cbdatasize: u32, pdatabuffer: *const core::ffi::c_void, ldatarep: u32, hrfault: windows_core::HRESULT)
        where
            Identity: IChannelHook_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IChannelHook_Impl::ClientNotify(this, core::mem::transmute_copy(&uextent), core::mem::transmute_copy(&riid), core::mem::transmute_copy(&cbdatasize), core::mem::transmute_copy(&pdatabuffer), core::mem::transmute_copy(&ldatarep), core::mem::transmute_copy(&hrfault))
        }
        unsafe extern "system" fn ServerNotify<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, uextent: *const windows_core::GUID, riid: *const windows_core::GUID, cbdatasize: u32, pdatabuffer: *const core::ffi::c_void, ldatarep: u32)
        where
            Identity: IChannelHook_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IChannelHook_Impl::ServerNotify(this, core::mem::transmute_copy(&uextent), core::mem::transmute_copy(&riid), core::mem::transmute_copy(&cbdatasize), core::mem::transmute_copy(&pdatabuffer), core::mem::transmute_copy(&ldatarep))
        }
        unsafe extern "system" fn ServerGetSize<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, uextent: *const windows_core::GUID, riid: *const windows_core::GUID, hrfault: windows_core::HRESULT, pdatasize: *mut u32)
        where
            Identity: IChannelHook_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IChannelHook_Impl::ServerGetSize(this, core::mem::transmute_copy(&uextent), core::mem::transmute_copy(&riid), core::mem::transmute_copy(&hrfault), core::mem::transmute_copy(&pdatasize))
        }
        unsafe extern "system" fn ServerFillBuffer<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, uextent: *const windows_core::GUID, riid: *const windows_core::GUID, pdatasize: *mut u32, pdatabuffer: *const core::ffi::c_void, hrfault: windows_core::HRESULT)
        where
            Identity: IChannelHook_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IChannelHook_Impl::ServerFillBuffer(this, core::mem::transmute_copy(&uextent), core::mem::transmute_copy(&riid), core::mem::transmute_copy(&pdatasize), core::mem::transmute_copy(&pdatabuffer), core::mem::transmute_copy(&hrfault))
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            ClientGetSize: ClientGetSize::<Identity, OFFSET>,
            ClientFillBuffer: ClientFillBuffer::<Identity, OFFSET>,
            ClientNotify: ClientNotify::<Identity, OFFSET>,
            ServerNotify: ServerNotify::<Identity, OFFSET>,
            ServerGetSize: ServerGetSize::<Identity, OFFSET>,
            ServerFillBuffer: ServerFillBuffer::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IChannelHook as windows_core::Interface>::IID
    }
}
pub trait IClassActivator_Impl: Sized {
    fn GetClassObject(&self, rclsid: *const windows_core::GUID, dwclasscontext: u32, locale: u32, riid: *const windows_core::GUID, ppv: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IClassActivator {}
impl IClassActivator_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IClassActivator_Vtbl
    where
        Identity: IClassActivator_Impl,
    {
        unsafe extern "system" fn GetClassObject<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, rclsid: *const windows_core::GUID, dwclasscontext: u32, locale: u32, riid: *const windows_core::GUID, ppv: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IClassActivator_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IClassActivator_Impl::GetClassObject(this, core::mem::transmute_copy(&rclsid), core::mem::transmute_copy(&dwclasscontext), core::mem::transmute_copy(&locale), core::mem::transmute_copy(&riid), core::mem::transmute_copy(&ppv)).into()
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), GetClassObject: GetClassObject::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IClassActivator as windows_core::Interface>::IID
    }
}
pub trait IClassFactory_Impl: Sized {
    fn CreateInstance(&self, punkouter: Option<&windows_core::IUnknown>, riid: *const windows_core::GUID, ppvobject: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
    fn LockServer(&self, flock: super::super::Foundation::BOOL) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IClassFactory {}
impl IClassFactory_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IClassFactory_Vtbl
    where
        Identity: IClassFactory_Impl,
    {
        unsafe extern "system" fn CreateInstance<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, punkouter: *mut core::ffi::c_void, riid: *const windows_core::GUID, ppvobject: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IClassFactory_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IClassFactory_Impl::CreateInstance(this, windows_core::from_raw_borrowed(&punkouter), core::mem::transmute_copy(&riid), core::mem::transmute_copy(&ppvobject)).into()
        }
        unsafe extern "system" fn LockServer<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, flock: super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: IClassFactory_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IClassFactory_Impl::LockServer(this, core::mem::transmute_copy(&flock)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            CreateInstance: CreateInstance::<Identity, OFFSET>,
            LockServer: LockServer::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IClassFactory as windows_core::Interface>::IID
    }
}
pub trait IClientSecurity_Impl: Sized {
    fn QueryBlanket(&self, pproxy: Option<&windows_core::IUnknown>, pauthnsvc: *mut u32, pauthzsvc: *mut u32, pserverprincname: *mut *mut u16, pauthnlevel: *mut RPC_C_AUTHN_LEVEL, pimplevel: *mut RPC_C_IMP_LEVEL, pauthinfo: *mut *mut core::ffi::c_void, pcapabilites: *mut u32) -> windows_core::Result<()>;
    fn SetBlanket(&self, pproxy: Option<&windows_core::IUnknown>, dwauthnsvc: u32, dwauthzsvc: u32, pserverprincname: &windows_core::PCWSTR, dwauthnlevel: RPC_C_AUTHN_LEVEL, dwimplevel: RPC_C_IMP_LEVEL, pauthinfo: *const core::ffi::c_void, dwcapabilities: &EOLE_AUTHENTICATION_CAPABILITIES) -> windows_core::Result<()>;
    fn CopyProxy(&self, pproxy: Option<&windows_core::IUnknown>) -> windows_core::Result<windows_core::IUnknown>;
}
impl windows_core::RuntimeName for IClientSecurity {}
impl IClientSecurity_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IClientSecurity_Vtbl
    where
        Identity: IClientSecurity_Impl,
    {
        unsafe extern "system" fn QueryBlanket<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pproxy: *mut core::ffi::c_void, pauthnsvc: *mut u32, pauthzsvc: *mut u32, pserverprincname: *mut *mut u16, pauthnlevel: *mut RPC_C_AUTHN_LEVEL, pimplevel: *mut RPC_C_IMP_LEVEL, pauthinfo: *mut *mut core::ffi::c_void, pcapabilites: *mut u32) -> windows_core::HRESULT
        where
            Identity: IClientSecurity_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IClientSecurity_Impl::QueryBlanket(this, windows_core::from_raw_borrowed(&pproxy), core::mem::transmute_copy(&pauthnsvc), core::mem::transmute_copy(&pauthzsvc), core::mem::transmute_copy(&pserverprincname), core::mem::transmute_copy(&pauthnlevel), core::mem::transmute_copy(&pimplevel), core::mem::transmute_copy(&pauthinfo), core::mem::transmute_copy(&pcapabilites)).into()
        }
        unsafe extern "system" fn SetBlanket<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pproxy: *mut core::ffi::c_void, dwauthnsvc: u32, dwauthzsvc: u32, pserverprincname: windows_core::PCWSTR, dwauthnlevel: RPC_C_AUTHN_LEVEL, dwimplevel: RPC_C_IMP_LEVEL, pauthinfo: *const core::ffi::c_void, dwcapabilities: u32) -> windows_core::HRESULT
        where
            Identity: IClientSecurity_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IClientSecurity_Impl::SetBlanket(this, windows_core::from_raw_borrowed(&pproxy), core::mem::transmute_copy(&dwauthnsvc), core::mem::transmute_copy(&dwauthzsvc), core::mem::transmute(&pserverprincname), core::mem::transmute_copy(&dwauthnlevel), core::mem::transmute_copy(&dwimplevel), core::mem::transmute_copy(&pauthinfo), core::mem::transmute(&dwcapabilities)).into()
        }
        unsafe extern "system" fn CopyProxy<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pproxy: *mut core::ffi::c_void, ppcopy: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IClientSecurity_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IClientSecurity_Impl::CopyProxy(this, windows_core::from_raw_borrowed(&pproxy)) {
                Ok(ok__) => {
                    ppcopy.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            QueryBlanket: QueryBlanket::<Identity, OFFSET>,
            SetBlanket: SetBlanket::<Identity, OFFSET>,
            CopyProxy: CopyProxy::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IClientSecurity as windows_core::Interface>::IID
    }
}
pub trait IComThreadingInfo_Impl: Sized {
    fn GetCurrentApartmentType(&self) -> windows_core::Result<APTTYPE>;
    fn GetCurrentThreadType(&self) -> windows_core::Result<THDTYPE>;
    fn GetCurrentLogicalThreadId(&self) -> windows_core::Result<windows_core::GUID>;
    fn SetCurrentLogicalThreadId(&self, rguid: *const windows_core::GUID) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IComThreadingInfo {}
impl IComThreadingInfo_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IComThreadingInfo_Vtbl
    where
        Identity: IComThreadingInfo_Impl,
    {
        unsafe extern "system" fn GetCurrentApartmentType<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, papttype: *mut APTTYPE) -> windows_core::HRESULT
        where
            Identity: IComThreadingInfo_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IComThreadingInfo_Impl::GetCurrentApartmentType(this) {
                Ok(ok__) => {
                    papttype.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCurrentThreadType<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pthreadtype: *mut THDTYPE) -> windows_core::HRESULT
        where
            Identity: IComThreadingInfo_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IComThreadingInfo_Impl::GetCurrentThreadType(this) {
                Ok(ok__) => {
                    pthreadtype.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCurrentLogicalThreadId<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pguidlogicalthreadid: *mut windows_core::GUID) -> windows_core::HRESULT
        where
            Identity: IComThreadingInfo_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IComThreadingInfo_Impl::GetCurrentLogicalThreadId(this) {
                Ok(ok__) => {
                    pguidlogicalthreadid.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCurrentLogicalThreadId<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, rguid: *const windows_core::GUID) -> windows_core::HRESULT
        where
            Identity: IComThreadingInfo_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IComThreadingInfo_Impl::SetCurrentLogicalThreadId(this, core::mem::transmute_copy(&rguid)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetCurrentApartmentType: GetCurrentApartmentType::<Identity, OFFSET>,
            GetCurrentThreadType: GetCurrentThreadType::<Identity, OFFSET>,
            GetCurrentLogicalThreadId: GetCurrentLogicalThreadId::<Identity, OFFSET>,
            SetCurrentLogicalThreadId: SetCurrentLogicalThreadId::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IComThreadingInfo as windows_core::Interface>::IID
    }
}
pub trait IConnectionPoint_Impl: Sized {
    fn GetConnectionInterface(&self) -> windows_core::Result<windows_core::GUID>;
    fn GetConnectionPointContainer(&self) -> windows_core::Result<IConnectionPointContainer>;
    fn Advise(&self, punksink: Option<&windows_core::IUnknown>) -> windows_core::Result<u32>;
    fn Unadvise(&self, dwcookie: u32) -> windows_core::Result<()>;
    fn EnumConnections(&self) -> windows_core::Result<IEnumConnections>;
}
impl windows_core::RuntimeName for IConnectionPoint {}
impl IConnectionPoint_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IConnectionPoint_Vtbl
    where
        Identity: IConnectionPoint_Impl,
    {
        unsafe extern "system" fn GetConnectionInterface<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, piid: *mut windows_core::GUID) -> windows_core::HRESULT
        where
            Identity: IConnectionPoint_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IConnectionPoint_Impl::GetConnectionInterface(this) {
                Ok(ok__) => {
                    piid.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetConnectionPointContainer<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppcpc: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IConnectionPoint_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IConnectionPoint_Impl::GetConnectionPointContainer(this) {
                Ok(ok__) => {
                    ppcpc.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Advise<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, punksink: *mut core::ffi::c_void, pdwcookie: *mut u32) -> windows_core::HRESULT
        where
            Identity: IConnectionPoint_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IConnectionPoint_Impl::Advise(this, windows_core::from_raw_borrowed(&punksink)) {
                Ok(ok__) => {
                    pdwcookie.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Unadvise<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwcookie: u32) -> windows_core::HRESULT
        where
            Identity: IConnectionPoint_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IConnectionPoint_Impl::Unadvise(this, core::mem::transmute_copy(&dwcookie)).into()
        }
        unsafe extern "system" fn EnumConnections<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppenum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IConnectionPoint_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IConnectionPoint_Impl::EnumConnections(this) {
                Ok(ok__) => {
                    ppenum.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetConnectionInterface: GetConnectionInterface::<Identity, OFFSET>,
            GetConnectionPointContainer: GetConnectionPointContainer::<Identity, OFFSET>,
            Advise: Advise::<Identity, OFFSET>,
            Unadvise: Unadvise::<Identity, OFFSET>,
            EnumConnections: EnumConnections::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IConnectionPoint as windows_core::Interface>::IID
    }
}
pub trait IConnectionPointContainer_Impl: Sized {
    fn EnumConnectionPoints(&self) -> windows_core::Result<IEnumConnectionPoints>;
    fn FindConnectionPoint(&self, riid: *const windows_core::GUID) -> windows_core::Result<IConnectionPoint>;
}
impl windows_core::RuntimeName for IConnectionPointContainer {}
impl IConnectionPointContainer_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IConnectionPointContainer_Vtbl
    where
        Identity: IConnectionPointContainer_Impl,
    {
        unsafe extern "system" fn EnumConnectionPoints<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppenum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IConnectionPointContainer_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IConnectionPointContainer_Impl::EnumConnectionPoints(this) {
                Ok(ok__) => {
                    ppenum.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FindConnectionPoint<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, riid: *const windows_core::GUID, ppcp: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IConnectionPointContainer_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IConnectionPointContainer_Impl::FindConnectionPoint(this, core::mem::transmute_copy(&riid)) {
                Ok(ok__) => {
                    ppcp.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            EnumConnectionPoints: EnumConnectionPoints::<Identity, OFFSET>,
            FindConnectionPoint: FindConnectionPoint::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IConnectionPointContainer as windows_core::Interface>::IID
    }
}
pub trait IContext_Impl: Sized {
    fn SetProperty(&self, rpolicyid: *const windows_core::GUID, flags: u32, punk: Option<&windows_core::IUnknown>) -> windows_core::Result<()>;
    fn RemoveProperty(&self, rpolicyid: *const windows_core::GUID) -> windows_core::Result<()>;
    fn GetProperty(&self, rguid: *const windows_core::GUID, pflags: *mut u32, ppunk: *mut Option<windows_core::IUnknown>) -> windows_core::Result<()>;
    fn EnumContextProps(&self) -> windows_core::Result<IEnumContextProps>;
}
impl windows_core::RuntimeName for IContext {}
impl IContext_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IContext_Vtbl
    where
        Identity: IContext_Impl,
    {
        unsafe extern "system" fn SetProperty<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, rpolicyid: *const windows_core::GUID, flags: u32, punk: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IContext_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IContext_Impl::SetProperty(this, core::mem::transmute_copy(&rpolicyid), core::mem::transmute_copy(&flags), windows_core::from_raw_borrowed(&punk)).into()
        }
        unsafe extern "system" fn RemoveProperty<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, rpolicyid: *const windows_core::GUID) -> windows_core::HRESULT
        where
            Identity: IContext_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IContext_Impl::RemoveProperty(this, core::mem::transmute_copy(&rpolicyid)).into()
        }
        unsafe extern "system" fn GetProperty<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, rguid: *const windows_core::GUID, pflags: *mut u32, ppunk: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IContext_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IContext_Impl::GetProperty(this, core::mem::transmute_copy(&rguid), core::mem::transmute_copy(&pflags), core::mem::transmute_copy(&ppunk)).into()
        }
        unsafe extern "system" fn EnumContextProps<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppenumcontextprops: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IContext_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IContext_Impl::EnumContextProps(this) {
                Ok(ok__) => {
                    ppenumcontextprops.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            SetProperty: SetProperty::<Identity, OFFSET>,
            RemoveProperty: RemoveProperty::<Identity, OFFSET>,
            GetProperty: GetProperty::<Identity, OFFSET>,
            EnumContextProps: EnumContextProps::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IContext as windows_core::Interface>::IID
    }
}
pub trait IContextCallback_Impl: Sized {
    fn ContextCallback(&self, pfncallback: PFNCONTEXTCALL, pparam: *const ComCallData, riid: *const windows_core::GUID, imethod: i32, punk: Option<&windows_core::IUnknown>) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IContextCallback {}
impl IContextCallback_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IContextCallback_Vtbl
    where
        Identity: IContextCallback_Impl,
    {
        unsafe extern "system" fn ContextCallback<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pfncallback: PFNCONTEXTCALL, pparam: *const ComCallData, riid: *const windows_core::GUID, imethod: i32, punk: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IContextCallback_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IContextCallback_Impl::ContextCallback(this, core::mem::transmute_copy(&pfncallback), core::mem::transmute_copy(&pparam), core::mem::transmute_copy(&riid), core::mem::transmute_copy(&imethod), windows_core::from_raw_borrowed(&punk)).into()
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), ContextCallback: ContextCallback::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IContextCallback as windows_core::Interface>::IID
    }
}
pub trait IDataAdviseHolder_Impl: Sized {
    fn Advise(&self, pdataobject: Option<&IDataObject>, pfetc: *const FORMATETC, advf: u32, padvise: Option<&IAdviseSink>) -> windows_core::Result<u32>;
    fn Unadvise(&self, dwconnection: u32) -> windows_core::Result<()>;
    fn EnumAdvise(&self) -> windows_core::Result<IEnumSTATDATA>;
    fn SendOnDataChange(&self, pdataobject: Option<&IDataObject>, dwreserved: u32, advf: u32) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IDataAdviseHolder {}
impl IDataAdviseHolder_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IDataAdviseHolder_Vtbl
    where
        Identity: IDataAdviseHolder_Impl,
    {
        unsafe extern "system" fn Advise<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdataobject: *mut core::ffi::c_void, pfetc: *const FORMATETC, advf: u32, padvise: *mut core::ffi::c_void, pdwconnection: *mut u32) -> windows_core::HRESULT
        where
            Identity: IDataAdviseHolder_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IDataAdviseHolder_Impl::Advise(this, windows_core::from_raw_borrowed(&pdataobject), core::mem::transmute_copy(&pfetc), core::mem::transmute_copy(&advf), windows_core::from_raw_borrowed(&padvise)) {
                Ok(ok__) => {
                    pdwconnection.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Unadvise<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwconnection: u32) -> windows_core::HRESULT
        where
            Identity: IDataAdviseHolder_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDataAdviseHolder_Impl::Unadvise(this, core::mem::transmute_copy(&dwconnection)).into()
        }
        unsafe extern "system" fn EnumAdvise<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppenumadvise: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IDataAdviseHolder_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IDataAdviseHolder_Impl::EnumAdvise(this) {
                Ok(ok__) => {
                    ppenumadvise.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SendOnDataChange<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdataobject: *mut core::ffi::c_void, dwreserved: u32, advf: u32) -> windows_core::HRESULT
        where
            Identity: IDataAdviseHolder_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDataAdviseHolder_Impl::SendOnDataChange(this, windows_core::from_raw_borrowed(&pdataobject), core::mem::transmute_copy(&dwreserved), core::mem::transmute_copy(&advf)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Advise: Advise::<Identity, OFFSET>,
            Unadvise: Unadvise::<Identity, OFFSET>,
            EnumAdvise: EnumAdvise::<Identity, OFFSET>,
            SendOnDataChange: SendOnDataChange::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDataAdviseHolder as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com_StructuredStorage"))]
pub trait IDataObject_Impl: Sized {
    fn GetData(&self, pformatetcin: *const FORMATETC) -> windows_core::Result<STGMEDIUM>;
    fn GetDataHere(&self, pformatetc: *const FORMATETC, pmedium: *mut STGMEDIUM) -> windows_core::Result<()>;
    fn QueryGetData(&self, pformatetc: *const FORMATETC) -> windows_core::HRESULT;
    fn GetCanonicalFormatEtc(&self, pformatectin: *const FORMATETC, pformatetcout: *mut FORMATETC) -> windows_core::HRESULT;
    fn SetData(&self, pformatetc: *const FORMATETC, pmedium: *const STGMEDIUM, frelease: super::super::Foundation::BOOL) -> windows_core::Result<()>;
    fn EnumFormatEtc(&self, dwdirection: u32) -> windows_core::Result<IEnumFORMATETC>;
    fn DAdvise(&self, pformatetc: *const FORMATETC, advf: u32, padvsink: Option<&IAdviseSink>) -> windows_core::Result<u32>;
    fn DUnadvise(&self, dwconnection: u32) -> windows_core::Result<()>;
    fn EnumDAdvise(&self) -> windows_core::Result<IEnumSTATDATA>;
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com_StructuredStorage"))]
impl windows_core::RuntimeName for IDataObject {}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com_StructuredStorage"))]
impl IDataObject_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IDataObject_Vtbl
    where
        Identity: IDataObject_Impl,
    {
        unsafe extern "system" fn GetData<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pformatetcin: *const FORMATETC, pmedium: *mut STGMEDIUM) -> windows_core::HRESULT
        where
            Identity: IDataObject_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IDataObject_Impl::GetData(this, core::mem::transmute_copy(&pformatetcin)) {
                Ok(ok__) => {
                    pmedium.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDataHere<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pformatetc: *const FORMATETC, pmedium: *mut STGMEDIUM) -> windows_core::HRESULT
        where
            Identity: IDataObject_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDataObject_Impl::GetDataHere(this, core::mem::transmute_copy(&pformatetc), core::mem::transmute_copy(&pmedium)).into()
        }
        unsafe extern "system" fn QueryGetData<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pformatetc: *const FORMATETC) -> windows_core::HRESULT
        where
            Identity: IDataObject_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDataObject_Impl::QueryGetData(this, core::mem::transmute_copy(&pformatetc))
        }
        unsafe extern "system" fn GetCanonicalFormatEtc<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pformatectin: *const FORMATETC, pformatetcout: *mut FORMATETC) -> windows_core::HRESULT
        where
            Identity: IDataObject_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDataObject_Impl::GetCanonicalFormatEtc(this, core::mem::transmute_copy(&pformatectin), core::mem::transmute_copy(&pformatetcout))
        }
        unsafe extern "system" fn SetData<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pformatetc: *const FORMATETC, pmedium: *const STGMEDIUM, frelease: super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: IDataObject_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDataObject_Impl::SetData(this, core::mem::transmute_copy(&pformatetc), core::mem::transmute_copy(&pmedium), core::mem::transmute_copy(&frelease)).into()
        }
        unsafe extern "system" fn EnumFormatEtc<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwdirection: u32, ppenumformatetc: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IDataObject_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IDataObject_Impl::EnumFormatEtc(this, core::mem::transmute_copy(&dwdirection)) {
                Ok(ok__) => {
                    ppenumformatetc.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DAdvise<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pformatetc: *const FORMATETC, advf: u32, padvsink: *mut core::ffi::c_void, pdwconnection: *mut u32) -> windows_core::HRESULT
        where
            Identity: IDataObject_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IDataObject_Impl::DAdvise(this, core::mem::transmute_copy(&pformatetc), core::mem::transmute_copy(&advf), windows_core::from_raw_borrowed(&padvsink)) {
                Ok(ok__) => {
                    pdwconnection.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DUnadvise<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwconnection: u32) -> windows_core::HRESULT
        where
            Identity: IDataObject_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDataObject_Impl::DUnadvise(this, core::mem::transmute_copy(&dwconnection)).into()
        }
        unsafe extern "system" fn EnumDAdvise<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppenumadvise: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IDataObject_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IDataObject_Impl::EnumDAdvise(this) {
                Ok(ok__) => {
                    ppenumadvise.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetData: GetData::<Identity, OFFSET>,
            GetDataHere: GetDataHere::<Identity, OFFSET>,
            QueryGetData: QueryGetData::<Identity, OFFSET>,
            GetCanonicalFormatEtc: GetCanonicalFormatEtc::<Identity, OFFSET>,
            SetData: SetData::<Identity, OFFSET>,
            EnumFormatEtc: EnumFormatEtc::<Identity, OFFSET>,
            DAdvise: DAdvise::<Identity, OFFSET>,
            DUnadvise: DUnadvise::<Identity, OFFSET>,
            EnumDAdvise: EnumDAdvise::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDataObject as windows_core::Interface>::IID
    }
}
pub trait IDispatch_Impl: Sized {
    fn GetTypeInfoCount(&self) -> windows_core::Result<u32>;
    fn GetTypeInfo(&self, itinfo: u32, lcid: u32) -> windows_core::Result<ITypeInfo>;
    fn GetIDsOfNames(&self, riid: *const windows_core::GUID, rgsznames: *const windows_core::PCWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> windows_core::Result<()>;
    fn Invoke(&self, dispidmember: i32, riid: *const windows_core::GUID, lcid: u32, wflags: DISPATCH_FLAGS, pdispparams: *const DISPPARAMS, pvarresult: *mut windows_core::VARIANT, pexcepinfo: *mut EXCEPINFO, puargerr: *mut u32) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IDispatch {}
impl IDispatch_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IDispatch_Vtbl
    where
        Identity: IDispatch_Impl,
    {
        unsafe extern "system" fn GetTypeInfoCount<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pctinfo: *mut u32) -> windows_core::HRESULT
        where
            Identity: IDispatch_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IDispatch_Impl::GetTypeInfoCount(this) {
                Ok(ok__) => {
                    pctinfo.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetTypeInfo<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, itinfo: u32, lcid: u32, pptinfo: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IDispatch_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IDispatch_Impl::GetTypeInfo(this, core::mem::transmute_copy(&itinfo), core::mem::transmute_copy(&lcid)) {
                Ok(ok__) => {
                    pptinfo.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetIDsOfNames<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, riid: *const windows_core::GUID, rgsznames: *const windows_core::PCWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> windows_core::HRESULT
        where
            Identity: IDispatch_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDispatch_Impl::GetIDsOfNames(this, core::mem::transmute_copy(&riid), core::mem::transmute_copy(&rgsznames), core::mem::transmute_copy(&cnames), core::mem::transmute_copy(&lcid), core::mem::transmute_copy(&rgdispid)).into()
        }
        unsafe extern "system" fn Invoke<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, dispidmember: i32, riid: *const windows_core::GUID, lcid: u32, wflags: DISPATCH_FLAGS, pdispparams: *const DISPPARAMS, pvarresult: *mut core::mem::MaybeUninit<windows_core::VARIANT>, pexcepinfo: *mut EXCEPINFO, puargerr: *mut u32) -> windows_core::HRESULT
        where
            Identity: IDispatch_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDispatch_Impl::Invoke(this, core::mem::transmute_copy(&dispidmember), core::mem::transmute_copy(&riid), core::mem::transmute_copy(&lcid), core::mem::transmute_copy(&wflags), core::mem::transmute_copy(&pdispparams), core::mem::transmute_copy(&pvarresult), core::mem::transmute_copy(&pexcepinfo), core::mem::transmute_copy(&puargerr)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetTypeInfoCount: GetTypeInfoCount::<Identity, OFFSET>,
            GetTypeInfo: GetTypeInfo::<Identity, OFFSET>,
            GetIDsOfNames: GetIDsOfNames::<Identity, OFFSET>,
            Invoke: Invoke::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDispatch as windows_core::Interface>::IID
    }
}
pub trait IEnumCATEGORYINFO_Impl: Sized {
    fn Next(&self, celt: u32, rgelt: *mut CATEGORYINFO, pceltfetched: *mut u32) -> windows_core::Result<()>;
    fn Skip(&self, celt: u32) -> windows_core::Result<()>;
    fn Reset(&self) -> windows_core::Result<()>;
    fn Clone(&self) -> windows_core::Result<IEnumCATEGORYINFO>;
}
impl windows_core::RuntimeName for IEnumCATEGORYINFO {}
impl IEnumCATEGORYINFO_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IEnumCATEGORYINFO_Vtbl
    where
        Identity: IEnumCATEGORYINFO_Impl,
    {
        unsafe extern "system" fn Next<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, celt: u32, rgelt: *mut CATEGORYINFO, pceltfetched: *mut u32) -> windows_core::HRESULT
        where
            Identity: IEnumCATEGORYINFO_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEnumCATEGORYINFO_Impl::Next(this, core::mem::transmute_copy(&celt), core::mem::transmute_copy(&rgelt), core::mem::transmute_copy(&pceltfetched)).into()
        }
        unsafe extern "system" fn Skip<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, celt: u32) -> windows_core::HRESULT
        where
            Identity: IEnumCATEGORYINFO_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEnumCATEGORYINFO_Impl::Skip(this, core::mem::transmute_copy(&celt)).into()
        }
        unsafe extern "system" fn Reset<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IEnumCATEGORYINFO_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEnumCATEGORYINFO_Impl::Reset(this).into()
        }
        unsafe extern "system" fn Clone<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppenum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IEnumCATEGORYINFO_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IEnumCATEGORYINFO_Impl::Clone(this) {
                Ok(ok__) => {
                    ppenum.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Next: Next::<Identity, OFFSET>,
            Skip: Skip::<Identity, OFFSET>,
            Reset: Reset::<Identity, OFFSET>,
            Clone: Clone::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IEnumCATEGORYINFO as windows_core::Interface>::IID
    }
}
pub trait IEnumConnectionPoints_Impl: Sized {
    fn Next(&self, cconnections: u32, ppcp: *mut Option<IConnectionPoint>, pcfetched: *mut u32) -> windows_core::HRESULT;
    fn Skip(&self, cconnections: u32) -> windows_core::Result<()>;
    fn Reset(&self) -> windows_core::Result<()>;
    fn Clone(&self) -> windows_core::Result<IEnumConnectionPoints>;
}
impl windows_core::RuntimeName for IEnumConnectionPoints {}
impl IEnumConnectionPoints_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IEnumConnectionPoints_Vtbl
    where
        Identity: IEnumConnectionPoints_Impl,
    {
        unsafe extern "system" fn Next<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, cconnections: u32, ppcp: *mut *mut core::ffi::c_void, pcfetched: *mut u32) -> windows_core::HRESULT
        where
            Identity: IEnumConnectionPoints_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEnumConnectionPoints_Impl::Next(this, core::mem::transmute_copy(&cconnections), core::mem::transmute_copy(&ppcp), core::mem::transmute_copy(&pcfetched))
        }
        unsafe extern "system" fn Skip<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, cconnections: u32) -> windows_core::HRESULT
        where
            Identity: IEnumConnectionPoints_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEnumConnectionPoints_Impl::Skip(this, core::mem::transmute_copy(&cconnections)).into()
        }
        unsafe extern "system" fn Reset<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IEnumConnectionPoints_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEnumConnectionPoints_Impl::Reset(this).into()
        }
        unsafe extern "system" fn Clone<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppenum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IEnumConnectionPoints_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IEnumConnectionPoints_Impl::Clone(this) {
                Ok(ok__) => {
                    ppenum.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Next: Next::<Identity, OFFSET>,
            Skip: Skip::<Identity, OFFSET>,
            Reset: Reset::<Identity, OFFSET>,
            Clone: Clone::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IEnumConnectionPoints as windows_core::Interface>::IID
    }
}
pub trait IEnumConnections_Impl: Sized {
    fn Next(&self, cconnections: u32, rgcd: *mut CONNECTDATA, pcfetched: *mut u32) -> windows_core::HRESULT;
    fn Skip(&self, cconnections: u32) -> windows_core::Result<()>;
    fn Reset(&self) -> windows_core::Result<()>;
    fn Clone(&self) -> windows_core::Result<IEnumConnections>;
}
impl windows_core::RuntimeName for IEnumConnections {}
impl IEnumConnections_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IEnumConnections_Vtbl
    where
        Identity: IEnumConnections_Impl,
    {
        unsafe extern "system" fn Next<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, cconnections: u32, rgcd: *mut CONNECTDATA, pcfetched: *mut u32) -> windows_core::HRESULT
        where
            Identity: IEnumConnections_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEnumConnections_Impl::Next(this, core::mem::transmute_copy(&cconnections), core::mem::transmute_copy(&rgcd), core::mem::transmute_copy(&pcfetched))
        }
        unsafe extern "system" fn Skip<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, cconnections: u32) -> windows_core::HRESULT
        where
            Identity: IEnumConnections_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEnumConnections_Impl::Skip(this, core::mem::transmute_copy(&cconnections)).into()
        }
        unsafe extern "system" fn Reset<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IEnumConnections_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEnumConnections_Impl::Reset(this).into()
        }
        unsafe extern "system" fn Clone<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppenum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IEnumConnections_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IEnumConnections_Impl::Clone(this) {
                Ok(ok__) => {
                    ppenum.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Next: Next::<Identity, OFFSET>,
            Skip: Skip::<Identity, OFFSET>,
            Reset: Reset::<Identity, OFFSET>,
            Clone: Clone::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IEnumConnections as windows_core::Interface>::IID
    }
}
pub trait IEnumContextProps_Impl: Sized {
    fn Next(&self, celt: u32, pcontextproperties: *mut ContextProperty, pceltfetched: *mut u32) -> windows_core::Result<()>;
    fn Skip(&self, celt: u32) -> windows_core::Result<()>;
    fn Reset(&self) -> windows_core::Result<()>;
    fn Clone(&self) -> windows_core::Result<IEnumContextProps>;
    fn Count(&self) -> windows_core::Result<u32>;
}
impl windows_core::RuntimeName for IEnumContextProps {}
impl IEnumContextProps_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IEnumContextProps_Vtbl
    where
        Identity: IEnumContextProps_Impl,
    {
        unsafe extern "system" fn Next<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, celt: u32, pcontextproperties: *mut ContextProperty, pceltfetched: *mut u32) -> windows_core::HRESULT
        where
            Identity: IEnumContextProps_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEnumContextProps_Impl::Next(this, core::mem::transmute_copy(&celt), core::mem::transmute_copy(&pcontextproperties), core::mem::transmute_copy(&pceltfetched)).into()
        }
        unsafe extern "system" fn Skip<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, celt: u32) -> windows_core::HRESULT
        where
            Identity: IEnumContextProps_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEnumContextProps_Impl::Skip(this, core::mem::transmute_copy(&celt)).into()
        }
        unsafe extern "system" fn Reset<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IEnumContextProps_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEnumContextProps_Impl::Reset(this).into()
        }
        unsafe extern "system" fn Clone<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppenumcontextprops: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IEnumContextProps_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IEnumContextProps_Impl::Clone(this) {
                Ok(ok__) => {
                    ppenumcontextprops.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Count<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcelt: *mut u32) -> windows_core::HRESULT
        where
            Identity: IEnumContextProps_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IEnumContextProps_Impl::Count(this) {
                Ok(ok__) => {
                    pcelt.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Next: Next::<Identity, OFFSET>,
            Skip: Skip::<Identity, OFFSET>,
            Reset: Reset::<Identity, OFFSET>,
            Clone: Clone::<Identity, OFFSET>,
            Count: Count::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IEnumContextProps as windows_core::Interface>::IID
    }
}
pub trait IEnumFORMATETC_Impl: Sized {
    fn Next(&self, celt: u32, rgelt: *mut FORMATETC, pceltfetched: *mut u32) -> windows_core::HRESULT;
    fn Skip(&self, celt: u32) -> windows_core::Result<()>;
    fn Reset(&self) -> windows_core::Result<()>;
    fn Clone(&self) -> windows_core::Result<IEnumFORMATETC>;
}
impl windows_core::RuntimeName for IEnumFORMATETC {}
impl IEnumFORMATETC_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IEnumFORMATETC_Vtbl
    where
        Identity: IEnumFORMATETC_Impl,
    {
        unsafe extern "system" fn Next<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, celt: u32, rgelt: *mut FORMATETC, pceltfetched: *mut u32) -> windows_core::HRESULT
        where
            Identity: IEnumFORMATETC_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEnumFORMATETC_Impl::Next(this, core::mem::transmute_copy(&celt), core::mem::transmute_copy(&rgelt), core::mem::transmute_copy(&pceltfetched))
        }
        unsafe extern "system" fn Skip<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, celt: u32) -> windows_core::HRESULT
        where
            Identity: IEnumFORMATETC_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEnumFORMATETC_Impl::Skip(this, core::mem::transmute_copy(&celt)).into()
        }
        unsafe extern "system" fn Reset<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IEnumFORMATETC_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEnumFORMATETC_Impl::Reset(this).into()
        }
        unsafe extern "system" fn Clone<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppenum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IEnumFORMATETC_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IEnumFORMATETC_Impl::Clone(this) {
                Ok(ok__) => {
                    ppenum.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Next: Next::<Identity, OFFSET>,
            Skip: Skip::<Identity, OFFSET>,
            Reset: Reset::<Identity, OFFSET>,
            Clone: Clone::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IEnumFORMATETC as windows_core::Interface>::IID
    }
}
pub trait IEnumGUID_Impl: Sized {
    fn Next(&self, celt: u32, rgelt: *mut windows_core::GUID, pceltfetched: *mut u32) -> windows_core::HRESULT;
    fn Skip(&self, celt: u32) -> windows_core::HRESULT;
    fn Reset(&self) -> windows_core::Result<()>;
    fn Clone(&self) -> windows_core::Result<IEnumGUID>;
}
impl windows_core::RuntimeName for IEnumGUID {}
impl IEnumGUID_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IEnumGUID_Vtbl
    where
        Identity: IEnumGUID_Impl,
    {
        unsafe extern "system" fn Next<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, celt: u32, rgelt: *mut windows_core::GUID, pceltfetched: *mut u32) -> windows_core::HRESULT
        where
            Identity: IEnumGUID_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEnumGUID_Impl::Next(this, core::mem::transmute_copy(&celt), core::mem::transmute_copy(&rgelt), core::mem::transmute_copy(&pceltfetched))
        }
        unsafe extern "system" fn Skip<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, celt: u32) -> windows_core::HRESULT
        where
            Identity: IEnumGUID_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEnumGUID_Impl::Skip(this, core::mem::transmute_copy(&celt))
        }
        unsafe extern "system" fn Reset<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IEnumGUID_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEnumGUID_Impl::Reset(this).into()
        }
        unsafe extern "system" fn Clone<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppenum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IEnumGUID_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IEnumGUID_Impl::Clone(this) {
                Ok(ok__) => {
                    ppenum.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Next: Next::<Identity, OFFSET>,
            Skip: Skip::<Identity, OFFSET>,
            Reset: Reset::<Identity, OFFSET>,
            Clone: Clone::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IEnumGUID as windows_core::Interface>::IID
    }
}
pub trait IEnumMoniker_Impl: Sized {
    fn Next(&self, celt: u32, rgelt: *mut Option<IMoniker>, pceltfetched: *mut u32) -> windows_core::HRESULT;
    fn Skip(&self, celt: u32) -> windows_core::HRESULT;
    fn Reset(&self) -> windows_core::Result<()>;
    fn Clone(&self) -> windows_core::Result<IEnumMoniker>;
}
impl windows_core::RuntimeName for IEnumMoniker {}
impl IEnumMoniker_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IEnumMoniker_Vtbl
    where
        Identity: IEnumMoniker_Impl,
    {
        unsafe extern "system" fn Next<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, celt: u32, rgelt: *mut *mut core::ffi::c_void, pceltfetched: *mut u32) -> windows_core::HRESULT
        where
            Identity: IEnumMoniker_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEnumMoniker_Impl::Next(this, core::mem::transmute_copy(&celt), core::mem::transmute_copy(&rgelt), core::mem::transmute_copy(&pceltfetched))
        }
        unsafe extern "system" fn Skip<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, celt: u32) -> windows_core::HRESULT
        where
            Identity: IEnumMoniker_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEnumMoniker_Impl::Skip(this, core::mem::transmute_copy(&celt))
        }
        unsafe extern "system" fn Reset<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IEnumMoniker_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEnumMoniker_Impl::Reset(this).into()
        }
        unsafe extern "system" fn Clone<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppenum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IEnumMoniker_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IEnumMoniker_Impl::Clone(this) {
                Ok(ok__) => {
                    ppenum.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Next: Next::<Identity, OFFSET>,
            Skip: Skip::<Identity, OFFSET>,
            Reset: Reset::<Identity, OFFSET>,
            Clone: Clone::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IEnumMoniker as windows_core::Interface>::IID
    }
}
pub trait IEnumSTATDATA_Impl: Sized {
    fn Next(&self, celt: u32, rgelt: *mut STATDATA, pceltfetched: *mut u32) -> windows_core::Result<()>;
    fn Skip(&self, celt: u32) -> windows_core::Result<()>;
    fn Reset(&self) -> windows_core::Result<()>;
    fn Clone(&self) -> windows_core::Result<IEnumSTATDATA>;
}
impl windows_core::RuntimeName for IEnumSTATDATA {}
impl IEnumSTATDATA_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IEnumSTATDATA_Vtbl
    where
        Identity: IEnumSTATDATA_Impl,
    {
        unsafe extern "system" fn Next<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, celt: u32, rgelt: *mut STATDATA, pceltfetched: *mut u32) -> windows_core::HRESULT
        where
            Identity: IEnumSTATDATA_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEnumSTATDATA_Impl::Next(this, core::mem::transmute_copy(&celt), core::mem::transmute_copy(&rgelt), core::mem::transmute_copy(&pceltfetched)).into()
        }
        unsafe extern "system" fn Skip<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, celt: u32) -> windows_core::HRESULT
        where
            Identity: IEnumSTATDATA_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEnumSTATDATA_Impl::Skip(this, core::mem::transmute_copy(&celt)).into()
        }
        unsafe extern "system" fn Reset<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IEnumSTATDATA_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEnumSTATDATA_Impl::Reset(this).into()
        }
        unsafe extern "system" fn Clone<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppenum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IEnumSTATDATA_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IEnumSTATDATA_Impl::Clone(this) {
                Ok(ok__) => {
                    ppenum.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Next: Next::<Identity, OFFSET>,
            Skip: Skip::<Identity, OFFSET>,
            Reset: Reset::<Identity, OFFSET>,
            Clone: Clone::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IEnumSTATDATA as windows_core::Interface>::IID
    }
}
pub trait IEnumString_Impl: Sized {
    fn Next(&self, celt: u32, rgelt: *mut windows_core::PWSTR, pceltfetched: *mut u32) -> windows_core::HRESULT;
    fn Skip(&self, celt: u32) -> windows_core::HRESULT;
    fn Reset(&self) -> windows_core::Result<()>;
    fn Clone(&self) -> windows_core::Result<IEnumString>;
}
impl windows_core::RuntimeName for IEnumString {}
impl IEnumString_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IEnumString_Vtbl
    where
        Identity: IEnumString_Impl,
    {
        unsafe extern "system" fn Next<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, celt: u32, rgelt: *mut windows_core::PWSTR, pceltfetched: *mut u32) -> windows_core::HRESULT
        where
            Identity: IEnumString_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEnumString_Impl::Next(this, core::mem::transmute_copy(&celt), core::mem::transmute_copy(&rgelt), core::mem::transmute_copy(&pceltfetched))
        }
        unsafe extern "system" fn Skip<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, celt: u32) -> windows_core::HRESULT
        where
            Identity: IEnumString_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEnumString_Impl::Skip(this, core::mem::transmute_copy(&celt))
        }
        unsafe extern "system" fn Reset<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IEnumString_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEnumString_Impl::Reset(this).into()
        }
        unsafe extern "system" fn Clone<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppenum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IEnumString_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IEnumString_Impl::Clone(this) {
                Ok(ok__) => {
                    ppenum.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Next: Next::<Identity, OFFSET>,
            Skip: Skip::<Identity, OFFSET>,
            Reset: Reset::<Identity, OFFSET>,
            Clone: Clone::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IEnumString as windows_core::Interface>::IID
    }
}
pub trait IEnumUnknown_Impl: Sized {
    fn Next(&self, celt: u32, rgelt: *mut Option<windows_core::IUnknown>, pceltfetched: *mut u32) -> windows_core::HRESULT;
    fn Skip(&self, celt: u32) -> windows_core::Result<()>;
    fn Reset(&self) -> windows_core::Result<()>;
    fn Clone(&self) -> windows_core::Result<IEnumUnknown>;
}
impl windows_core::RuntimeName for IEnumUnknown {}
impl IEnumUnknown_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IEnumUnknown_Vtbl
    where
        Identity: IEnumUnknown_Impl,
    {
        unsafe extern "system" fn Next<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, celt: u32, rgelt: *mut *mut core::ffi::c_void, pceltfetched: *mut u32) -> windows_core::HRESULT
        where
            Identity: IEnumUnknown_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEnumUnknown_Impl::Next(this, core::mem::transmute_copy(&celt), core::mem::transmute_copy(&rgelt), core::mem::transmute_copy(&pceltfetched))
        }
        unsafe extern "system" fn Skip<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, celt: u32) -> windows_core::HRESULT
        where
            Identity: IEnumUnknown_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEnumUnknown_Impl::Skip(this, core::mem::transmute_copy(&celt)).into()
        }
        unsafe extern "system" fn Reset<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IEnumUnknown_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEnumUnknown_Impl::Reset(this).into()
        }
        unsafe extern "system" fn Clone<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppenum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IEnumUnknown_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IEnumUnknown_Impl::Clone(this) {
                Ok(ok__) => {
                    ppenum.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Next: Next::<Identity, OFFSET>,
            Skip: Skip::<Identity, OFFSET>,
            Reset: Reset::<Identity, OFFSET>,
            Clone: Clone::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IEnumUnknown as windows_core::Interface>::IID
    }
}
pub trait IErrorInfo_Impl: Sized {
    fn GetGUID(&self) -> windows_core::Result<windows_core::GUID>;
    fn GetSource(&self) -> windows_core::Result<windows_core::BSTR>;
    fn GetDescription(&self) -> windows_core::Result<windows_core::BSTR>;
    fn GetHelpFile(&self) -> windows_core::Result<windows_core::BSTR>;
    fn GetHelpContext(&self) -> windows_core::Result<u32>;
}
impl windows_core::RuntimeName for IErrorInfo {}
impl IErrorInfo_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IErrorInfo_Vtbl
    where
        Identity: IErrorInfo_Impl,
    {
        unsafe extern "system" fn GetGUID<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pguid: *mut windows_core::GUID) -> windows_core::HRESULT
        where
            Identity: IErrorInfo_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IErrorInfo_Impl::GetGUID(this) {
                Ok(ok__) => {
                    pguid.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSource<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrsource: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IErrorInfo_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IErrorInfo_Impl::GetSource(this) {
                Ok(ok__) => {
                    pbstrsource.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDescription<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrdescription: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IErrorInfo_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IErrorInfo_Impl::GetDescription(this) {
                Ok(ok__) => {
                    pbstrdescription.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetHelpFile<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrhelpfile: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IErrorInfo_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IErrorInfo_Impl::GetHelpFile(this) {
                Ok(ok__) => {
                    pbstrhelpfile.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetHelpContext<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwhelpcontext: *mut u32) -> windows_core::HRESULT
        where
            Identity: IErrorInfo_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IErrorInfo_Impl::GetHelpContext(this) {
                Ok(ok__) => {
                    pdwhelpcontext.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetGUID: GetGUID::<Identity, OFFSET>,
            GetSource: GetSource::<Identity, OFFSET>,
            GetDescription: GetDescription::<Identity, OFFSET>,
            GetHelpFile: GetHelpFile::<Identity, OFFSET>,
            GetHelpContext: GetHelpContext::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IErrorInfo as windows_core::Interface>::IID
    }
}
pub trait IErrorLog_Impl: Sized {
    fn AddError(&self, pszpropname: &windows_core::PCWSTR, pexcepinfo: *const EXCEPINFO) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IErrorLog {}
impl IErrorLog_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IErrorLog_Vtbl
    where
        Identity: IErrorLog_Impl,
    {
        unsafe extern "system" fn AddError<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszpropname: windows_core::PCWSTR, pexcepinfo: *const EXCEPINFO) -> windows_core::HRESULT
        where
            Identity: IErrorLog_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IErrorLog_Impl::AddError(this, core::mem::transmute(&pszpropname), core::mem::transmute_copy(&pexcepinfo)).into()
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), AddError: AddError::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IErrorLog as windows_core::Interface>::IID
    }
}
pub trait IExternalConnection_Impl: Sized {
    fn AddConnection(&self, extconn: u32, reserved: u32) -> u32;
    fn ReleaseConnection(&self, extconn: u32, reserved: u32, flastreleasecloses: super::super::Foundation::BOOL) -> u32;
}
impl windows_core::RuntimeName for IExternalConnection {}
impl IExternalConnection_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IExternalConnection_Vtbl
    where
        Identity: IExternalConnection_Impl,
    {
        unsafe extern "system" fn AddConnection<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, extconn: u32, reserved: u32) -> u32
        where
            Identity: IExternalConnection_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IExternalConnection_Impl::AddConnection(this, core::mem::transmute_copy(&extconn), core::mem::transmute_copy(&reserved))
        }
        unsafe extern "system" fn ReleaseConnection<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, extconn: u32, reserved: u32, flastreleasecloses: super::super::Foundation::BOOL) -> u32
        where
            Identity: IExternalConnection_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IExternalConnection_Impl::ReleaseConnection(this, core::mem::transmute_copy(&extconn), core::mem::transmute_copy(&reserved), core::mem::transmute_copy(&flastreleasecloses))
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            AddConnection: AddConnection::<Identity, OFFSET>,
            ReleaseConnection: ReleaseConnection::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IExternalConnection as windows_core::Interface>::IID
    }
}
pub trait IFastRundown_Impl: Sized {}
impl windows_core::RuntimeName for IFastRundown {}
impl IFastRundown_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IFastRundown_Vtbl
    where
        Identity: IFastRundown_Impl,
    {
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>() }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IFastRundown as windows_core::Interface>::IID
    }
}
pub trait IForegroundTransfer_Impl: Sized {
    fn AllowForegroundTransfer(&self, lpvreserved: *const core::ffi::c_void) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IForegroundTransfer {}
impl IForegroundTransfer_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IForegroundTransfer_Vtbl
    where
        Identity: IForegroundTransfer_Impl,
    {
        unsafe extern "system" fn AllowForegroundTransfer<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, lpvreserved: *const core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IForegroundTransfer_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IForegroundTransfer_Impl::AllowForegroundTransfer(this, core::mem::transmute_copy(&lpvreserved)).into()
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), AllowForegroundTransfer: AllowForegroundTransfer::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IForegroundTransfer as windows_core::Interface>::IID
    }
}
pub trait IGlobalInterfaceTable_Impl: Sized {
    fn RegisterInterfaceInGlobal(&self, punk: Option<&windows_core::IUnknown>, riid: *const windows_core::GUID) -> windows_core::Result<u32>;
    fn RevokeInterfaceFromGlobal(&self, dwcookie: u32) -> windows_core::Result<()>;
    fn GetInterfaceFromGlobal(&self, dwcookie: u32, riid: *const windows_core::GUID, ppv: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IGlobalInterfaceTable {}
impl IGlobalInterfaceTable_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IGlobalInterfaceTable_Vtbl
    where
        Identity: IGlobalInterfaceTable_Impl,
    {
        unsafe extern "system" fn RegisterInterfaceInGlobal<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, punk: *mut core::ffi::c_void, riid: *const windows_core::GUID, pdwcookie: *mut u32) -> windows_core::HRESULT
        where
            Identity: IGlobalInterfaceTable_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IGlobalInterfaceTable_Impl::RegisterInterfaceInGlobal(this, windows_core::from_raw_borrowed(&punk), core::mem::transmute_copy(&riid)) {
                Ok(ok__) => {
                    pdwcookie.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RevokeInterfaceFromGlobal<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwcookie: u32) -> windows_core::HRESULT
        where
            Identity: IGlobalInterfaceTable_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IGlobalInterfaceTable_Impl::RevokeInterfaceFromGlobal(this, core::mem::transmute_copy(&dwcookie)).into()
        }
        unsafe extern "system" fn GetInterfaceFromGlobal<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwcookie: u32, riid: *const windows_core::GUID, ppv: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IGlobalInterfaceTable_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IGlobalInterfaceTable_Impl::GetInterfaceFromGlobal(this, core::mem::transmute_copy(&dwcookie), core::mem::transmute_copy(&riid), core::mem::transmute_copy(&ppv)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            RegisterInterfaceInGlobal: RegisterInterfaceInGlobal::<Identity, OFFSET>,
            RevokeInterfaceFromGlobal: RevokeInterfaceFromGlobal::<Identity, OFFSET>,
            GetInterfaceFromGlobal: GetInterfaceFromGlobal::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IGlobalInterfaceTable as windows_core::Interface>::IID
    }
}
pub trait IGlobalOptions_Impl: Sized {
    fn Set(&self, dwproperty: GLOBALOPT_PROPERTIES, dwvalue: usize) -> windows_core::Result<()>;
    fn Query(&self, dwproperty: GLOBALOPT_PROPERTIES) -> windows_core::Result<usize>;
}
impl windows_core::RuntimeName for IGlobalOptions {}
impl IGlobalOptions_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IGlobalOptions_Vtbl
    where
        Identity: IGlobalOptions_Impl,
    {
        unsafe extern "system" fn Set<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwproperty: GLOBALOPT_PROPERTIES, dwvalue: usize) -> windows_core::HRESULT
        where
            Identity: IGlobalOptions_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IGlobalOptions_Impl::Set(this, core::mem::transmute_copy(&dwproperty), core::mem::transmute_copy(&dwvalue)).into()
        }
        unsafe extern "system" fn Query<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwproperty: GLOBALOPT_PROPERTIES, pdwvalue: *mut usize) -> windows_core::HRESULT
        where
            Identity: IGlobalOptions_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IGlobalOptions_Impl::Query(this, core::mem::transmute_copy(&dwproperty)) {
                Ok(ok__) => {
                    pdwvalue.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), Set: Set::<Identity, OFFSET>, Query: Query::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IGlobalOptions as windows_core::Interface>::IID
    }
}
pub trait IInitializeSpy_Impl: Sized {
    fn PreInitialize(&self, dwcoinit: u32, dwcurthreadaptrefs: u32) -> windows_core::Result<()>;
    fn PostInitialize(&self, hrcoinit: windows_core::HRESULT, dwcoinit: u32, dwnewthreadaptrefs: u32) -> windows_core::Result<()>;
    fn PreUninitialize(&self, dwcurthreadaptrefs: u32) -> windows_core::Result<()>;
    fn PostUninitialize(&self, dwnewthreadaptrefs: u32) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IInitializeSpy {}
impl IInitializeSpy_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IInitializeSpy_Vtbl
    where
        Identity: IInitializeSpy_Impl,
    {
        unsafe extern "system" fn PreInitialize<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwcoinit: u32, dwcurthreadaptrefs: u32) -> windows_core::HRESULT
        where
            Identity: IInitializeSpy_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IInitializeSpy_Impl::PreInitialize(this, core::mem::transmute_copy(&dwcoinit), core::mem::transmute_copy(&dwcurthreadaptrefs)).into()
        }
        unsafe extern "system" fn PostInitialize<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, hrcoinit: windows_core::HRESULT, dwcoinit: u32, dwnewthreadaptrefs: u32) -> windows_core::HRESULT
        where
            Identity: IInitializeSpy_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IInitializeSpy_Impl::PostInitialize(this, core::mem::transmute_copy(&hrcoinit), core::mem::transmute_copy(&dwcoinit), core::mem::transmute_copy(&dwnewthreadaptrefs)).into()
        }
        unsafe extern "system" fn PreUninitialize<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwcurthreadaptrefs: u32) -> windows_core::HRESULT
        where
            Identity: IInitializeSpy_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IInitializeSpy_Impl::PreUninitialize(this, core::mem::transmute_copy(&dwcurthreadaptrefs)).into()
        }
        unsafe extern "system" fn PostUninitialize<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwnewthreadaptrefs: u32) -> windows_core::HRESULT
        where
            Identity: IInitializeSpy_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IInitializeSpy_Impl::PostUninitialize(this, core::mem::transmute_copy(&dwnewthreadaptrefs)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            PreInitialize: PreInitialize::<Identity, OFFSET>,
            PostInitialize: PostInitialize::<Identity, OFFSET>,
            PreUninitialize: PreUninitialize::<Identity, OFFSET>,
            PostUninitialize: PostUninitialize::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IInitializeSpy as windows_core::Interface>::IID
    }
}
pub trait IInternalUnknown_Impl: Sized {
    fn QueryInternalInterface(&self, riid: *const windows_core::GUID, ppv: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IInternalUnknown {}
impl IInternalUnknown_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IInternalUnknown_Vtbl
    where
        Identity: IInternalUnknown_Impl,
    {
        unsafe extern "system" fn QueryInternalInterface<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, riid: *const windows_core::GUID, ppv: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IInternalUnknown_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IInternalUnknown_Impl::QueryInternalInterface(this, core::mem::transmute_copy(&riid), core::mem::transmute_copy(&ppv)).into()
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), QueryInternalInterface: QueryInternalInterface::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IInternalUnknown as windows_core::Interface>::IID
    }
}
pub trait IMachineGlobalObjectTable_Impl: Sized {
    fn RegisterObject(&self, clsid: *const windows_core::GUID, identifier: &windows_core::PCWSTR, object: Option<&windows_core::IUnknown>) -> windows_core::Result<MachineGlobalObjectTableRegistrationToken>;
    fn GetObject(&self, clsid: *const windows_core::GUID, identifier: &windows_core::PCWSTR, riid: *const windows_core::GUID, ppv: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
    fn RevokeObject(&self, token: MachineGlobalObjectTableRegistrationToken) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IMachineGlobalObjectTable {}
impl IMachineGlobalObjectTable_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IMachineGlobalObjectTable_Vtbl
    where
        Identity: IMachineGlobalObjectTable_Impl,
    {
        unsafe extern "system" fn RegisterObject<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, clsid: *const windows_core::GUID, identifier: windows_core::PCWSTR, object: *mut core::ffi::c_void, token: *mut MachineGlobalObjectTableRegistrationToken) -> windows_core::HRESULT
        where
            Identity: IMachineGlobalObjectTable_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IMachineGlobalObjectTable_Impl::RegisterObject(this, core::mem::transmute_copy(&clsid), core::mem::transmute(&identifier), windows_core::from_raw_borrowed(&object)) {
                Ok(ok__) => {
                    token.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetObject<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, clsid: *const windows_core::GUID, identifier: windows_core::PCWSTR, riid: *const windows_core::GUID, ppv: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IMachineGlobalObjectTable_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMachineGlobalObjectTable_Impl::GetObject(this, core::mem::transmute_copy(&clsid), core::mem::transmute(&identifier), core::mem::transmute_copy(&riid), core::mem::transmute_copy(&ppv)).into()
        }
        unsafe extern "system" fn RevokeObject<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, token: MachineGlobalObjectTableRegistrationToken) -> windows_core::HRESULT
        where
            Identity: IMachineGlobalObjectTable_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMachineGlobalObjectTable_Impl::RevokeObject(this, core::mem::transmute_copy(&token)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            RegisterObject: RegisterObject::<Identity, OFFSET>,
            GetObject: GetObject::<Identity, OFFSET>,
            RevokeObject: RevokeObject::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMachineGlobalObjectTable as windows_core::Interface>::IID
    }
}
pub trait IMalloc_Impl: Sized {
    fn Alloc(&self, cb: usize) -> *mut core::ffi::c_void;
    fn Realloc(&self, pv: *const core::ffi::c_void, cb: usize) -> *mut core::ffi::c_void;
    fn Free(&self, pv: *const core::ffi::c_void);
    fn GetSize(&self, pv: *const core::ffi::c_void) -> usize;
    fn DidAlloc(&self, pv: *const core::ffi::c_void) -> i32;
    fn HeapMinimize(&self);
}
impl windows_core::RuntimeName for IMalloc {}
impl IMalloc_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IMalloc_Vtbl
    where
        Identity: IMalloc_Impl,
    {
        unsafe extern "system" fn Alloc<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, cb: usize) -> *mut core::ffi::c_void
        where
            Identity: IMalloc_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMalloc_Impl::Alloc(this, core::mem::transmute_copy(&cb))
        }
        unsafe extern "system" fn Realloc<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pv: *const core::ffi::c_void, cb: usize) -> *mut core::ffi::c_void
        where
            Identity: IMalloc_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMalloc_Impl::Realloc(this, core::mem::transmute_copy(&pv), core::mem::transmute_copy(&cb))
        }
        unsafe extern "system" fn Free<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pv: *const core::ffi::c_void)
        where
            Identity: IMalloc_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMalloc_Impl::Free(this, core::mem::transmute_copy(&pv))
        }
        unsafe extern "system" fn GetSize<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pv: *const core::ffi::c_void) -> usize
        where
            Identity: IMalloc_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMalloc_Impl::GetSize(this, core::mem::transmute_copy(&pv))
        }
        unsafe extern "system" fn DidAlloc<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pv: *const core::ffi::c_void) -> i32
        where
            Identity: IMalloc_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMalloc_Impl::DidAlloc(this, core::mem::transmute_copy(&pv))
        }
        unsafe extern "system" fn HeapMinimize<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void)
        where
            Identity: IMalloc_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMalloc_Impl::HeapMinimize(this)
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Alloc: Alloc::<Identity, OFFSET>,
            Realloc: Realloc::<Identity, OFFSET>,
            Free: Free::<Identity, OFFSET>,
            GetSize: GetSize::<Identity, OFFSET>,
            DidAlloc: DidAlloc::<Identity, OFFSET>,
            HeapMinimize: HeapMinimize::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMalloc as windows_core::Interface>::IID
    }
}
pub trait IMallocSpy_Impl: Sized {
    fn PreAlloc(&self, cbrequest: usize) -> usize;
    fn PostAlloc(&self, pactual: *const core::ffi::c_void) -> *mut core::ffi::c_void;
    fn PreFree(&self, prequest: *const core::ffi::c_void, fspyed: super::super::Foundation::BOOL) -> *mut core::ffi::c_void;
    fn PostFree(&self, fspyed: super::super::Foundation::BOOL);
    fn PreRealloc(&self, prequest: *const core::ffi::c_void, cbrequest: usize, ppnewrequest: *mut *mut core::ffi::c_void, fspyed: super::super::Foundation::BOOL) -> usize;
    fn PostRealloc(&self, pactual: *const core::ffi::c_void, fspyed: super::super::Foundation::BOOL) -> *mut core::ffi::c_void;
    fn PreGetSize(&self, prequest: *const core::ffi::c_void, fspyed: super::super::Foundation::BOOL) -> *mut core::ffi::c_void;
    fn PostGetSize(&self, cbactual: usize, fspyed: super::super::Foundation::BOOL) -> usize;
    fn PreDidAlloc(&self, prequest: *const core::ffi::c_void, fspyed: super::super::Foundation::BOOL) -> *mut core::ffi::c_void;
    fn PostDidAlloc(&self, prequest: *const core::ffi::c_void, fspyed: super::super::Foundation::BOOL, factual: i32) -> i32;
    fn PreHeapMinimize(&self);
    fn PostHeapMinimize(&self);
}
impl windows_core::RuntimeName for IMallocSpy {}
impl IMallocSpy_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IMallocSpy_Vtbl
    where
        Identity: IMallocSpy_Impl,
    {
        unsafe extern "system" fn PreAlloc<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, cbrequest: usize) -> usize
        where
            Identity: IMallocSpy_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMallocSpy_Impl::PreAlloc(this, core::mem::transmute_copy(&cbrequest))
        }
        unsafe extern "system" fn PostAlloc<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pactual: *const core::ffi::c_void) -> *mut core::ffi::c_void
        where
            Identity: IMallocSpy_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMallocSpy_Impl::PostAlloc(this, core::mem::transmute_copy(&pactual))
        }
        unsafe extern "system" fn PreFree<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, prequest: *const core::ffi::c_void, fspyed: super::super::Foundation::BOOL) -> *mut core::ffi::c_void
        where
            Identity: IMallocSpy_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMallocSpy_Impl::PreFree(this, core::mem::transmute_copy(&prequest), core::mem::transmute_copy(&fspyed))
        }
        unsafe extern "system" fn PostFree<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, fspyed: super::super::Foundation::BOOL)
        where
            Identity: IMallocSpy_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMallocSpy_Impl::PostFree(this, core::mem::transmute_copy(&fspyed))
        }
        unsafe extern "system" fn PreRealloc<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, prequest: *const core::ffi::c_void, cbrequest: usize, ppnewrequest: *mut *mut core::ffi::c_void, fspyed: super::super::Foundation::BOOL) -> usize
        where
            Identity: IMallocSpy_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMallocSpy_Impl::PreRealloc(this, core::mem::transmute_copy(&prequest), core::mem::transmute_copy(&cbrequest), core::mem::transmute_copy(&ppnewrequest), core::mem::transmute_copy(&fspyed))
        }
        unsafe extern "system" fn PostRealloc<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pactual: *const core::ffi::c_void, fspyed: super::super::Foundation::BOOL) -> *mut core::ffi::c_void
        where
            Identity: IMallocSpy_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMallocSpy_Impl::PostRealloc(this, core::mem::transmute_copy(&pactual), core::mem::transmute_copy(&fspyed))
        }
        unsafe extern "system" fn PreGetSize<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, prequest: *const core::ffi::c_void, fspyed: super::super::Foundation::BOOL) -> *mut core::ffi::c_void
        where
            Identity: IMallocSpy_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMallocSpy_Impl::PreGetSize(this, core::mem::transmute_copy(&prequest), core::mem::transmute_copy(&fspyed))
        }
        unsafe extern "system" fn PostGetSize<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, cbactual: usize, fspyed: super::super::Foundation::BOOL) -> usize
        where
            Identity: IMallocSpy_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMallocSpy_Impl::PostGetSize(this, core::mem::transmute_copy(&cbactual), core::mem::transmute_copy(&fspyed))
        }
        unsafe extern "system" fn PreDidAlloc<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, prequest: *const core::ffi::c_void, fspyed: super::super::Foundation::BOOL) -> *mut core::ffi::c_void
        where
            Identity: IMallocSpy_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMallocSpy_Impl::PreDidAlloc(this, core::mem::transmute_copy(&prequest), core::mem::transmute_copy(&fspyed))
        }
        unsafe extern "system" fn PostDidAlloc<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, prequest: *const core::ffi::c_void, fspyed: super::super::Foundation::BOOL, factual: i32) -> i32
        where
            Identity: IMallocSpy_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMallocSpy_Impl::PostDidAlloc(this, core::mem::transmute_copy(&prequest), core::mem::transmute_copy(&fspyed), core::mem::transmute_copy(&factual))
        }
        unsafe extern "system" fn PreHeapMinimize<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void)
        where
            Identity: IMallocSpy_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMallocSpy_Impl::PreHeapMinimize(this)
        }
        unsafe extern "system" fn PostHeapMinimize<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void)
        where
            Identity: IMallocSpy_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMallocSpy_Impl::PostHeapMinimize(this)
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            PreAlloc: PreAlloc::<Identity, OFFSET>,
            PostAlloc: PostAlloc::<Identity, OFFSET>,
            PreFree: PreFree::<Identity, OFFSET>,
            PostFree: PostFree::<Identity, OFFSET>,
            PreRealloc: PreRealloc::<Identity, OFFSET>,
            PostRealloc: PostRealloc::<Identity, OFFSET>,
            PreGetSize: PreGetSize::<Identity, OFFSET>,
            PostGetSize: PostGetSize::<Identity, OFFSET>,
            PreDidAlloc: PreDidAlloc::<Identity, OFFSET>,
            PostDidAlloc: PostDidAlloc::<Identity, OFFSET>,
            PreHeapMinimize: PreHeapMinimize::<Identity, OFFSET>,
            PostHeapMinimize: PostHeapMinimize::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMallocSpy as windows_core::Interface>::IID
    }
}
pub trait IMoniker_Impl: Sized + IPersistStream_Impl {
    fn BindToObject(&self, pbc: Option<&IBindCtx>, pmktoleft: Option<&IMoniker>, riidresult: *const windows_core::GUID, ppvresult: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
    fn BindToStorage(&self, pbc: Option<&IBindCtx>, pmktoleft: Option<&IMoniker>, riid: *const windows_core::GUID, ppvobj: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
    fn Reduce(&self, pbc: Option<&IBindCtx>, dwreducehowfar: u32, ppmktoleft: *mut Option<IMoniker>, ppmkreduced: *mut Option<IMoniker>) -> windows_core::Result<()>;
    fn ComposeWith(&self, pmkright: Option<&IMoniker>, fonlyifnotgeneric: super::super::Foundation::BOOL) -> windows_core::Result<IMoniker>;
    fn Enum(&self, fforward: super::super::Foundation::BOOL) -> windows_core::Result<IEnumMoniker>;
    fn IsEqual(&self, pmkothermoniker: Option<&IMoniker>) -> windows_core::HRESULT;
    fn Hash(&self) -> windows_core::Result<u32>;
    fn IsRunning(&self, pbc: Option<&IBindCtx>, pmktoleft: Option<&IMoniker>, pmknewlyrunning: Option<&IMoniker>) -> windows_core::Result<()>;
    fn GetTimeOfLastChange(&self, pbc: Option<&IBindCtx>, pmktoleft: Option<&IMoniker>) -> windows_core::Result<super::super::Foundation::FILETIME>;
    fn Inverse(&self) -> windows_core::Result<IMoniker>;
    fn CommonPrefixWith(&self, pmkother: Option<&IMoniker>) -> windows_core::Result<IMoniker>;
    fn RelativePathTo(&self, pmkother: Option<&IMoniker>) -> windows_core::Result<IMoniker>;
    fn GetDisplayName(&self, pbc: Option<&IBindCtx>, pmktoleft: Option<&IMoniker>) -> windows_core::Result<windows_core::PWSTR>;
    fn ParseDisplayName(&self, pbc: Option<&IBindCtx>, pmktoleft: Option<&IMoniker>, pszdisplayname: &windows_core::PCWSTR, pcheaten: *mut u32, ppmkout: *mut Option<IMoniker>) -> windows_core::Result<()>;
    fn IsSystemMoniker(&self) -> windows_core::Result<u32>;
}
impl windows_core::RuntimeName for IMoniker {}
impl IMoniker_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IMoniker_Vtbl
    where
        Identity: IMoniker_Impl,
    {
        unsafe extern "system" fn BindToObject<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbc: *mut core::ffi::c_void, pmktoleft: *mut core::ffi::c_void, riidresult: *const windows_core::GUID, ppvresult: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IMoniker_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMoniker_Impl::BindToObject(this, windows_core::from_raw_borrowed(&pbc), windows_core::from_raw_borrowed(&pmktoleft), core::mem::transmute_copy(&riidresult), core::mem::transmute_copy(&ppvresult)).into()
        }
        unsafe extern "system" fn BindToStorage<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbc: *mut core::ffi::c_void, pmktoleft: *mut core::ffi::c_void, riid: *const windows_core::GUID, ppvobj: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IMoniker_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMoniker_Impl::BindToStorage(this, windows_core::from_raw_borrowed(&pbc), windows_core::from_raw_borrowed(&pmktoleft), core::mem::transmute_copy(&riid), core::mem::transmute_copy(&ppvobj)).into()
        }
        unsafe extern "system" fn Reduce<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbc: *mut core::ffi::c_void, dwreducehowfar: u32, ppmktoleft: *mut *mut core::ffi::c_void, ppmkreduced: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IMoniker_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMoniker_Impl::Reduce(this, windows_core::from_raw_borrowed(&pbc), core::mem::transmute_copy(&dwreducehowfar), core::mem::transmute_copy(&ppmktoleft), core::mem::transmute_copy(&ppmkreduced)).into()
        }
        unsafe extern "system" fn ComposeWith<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pmkright: *mut core::ffi::c_void, fonlyifnotgeneric: super::super::Foundation::BOOL, ppmkcomposite: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IMoniker_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IMoniker_Impl::ComposeWith(this, windows_core::from_raw_borrowed(&pmkright), core::mem::transmute_copy(&fonlyifnotgeneric)) {
                Ok(ok__) => {
                    ppmkcomposite.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Enum<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, fforward: super::super::Foundation::BOOL, ppenummoniker: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IMoniker_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IMoniker_Impl::Enum(this, core::mem::transmute_copy(&fforward)) {
                Ok(ok__) => {
                    ppenummoniker.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsEqual<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pmkothermoniker: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IMoniker_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMoniker_Impl::IsEqual(this, windows_core::from_raw_borrowed(&pmkothermoniker))
        }
        unsafe extern "system" fn Hash<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwhash: *mut u32) -> windows_core::HRESULT
        where
            Identity: IMoniker_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IMoniker_Impl::Hash(this) {
                Ok(ok__) => {
                    pdwhash.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsRunning<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbc: *mut core::ffi::c_void, pmktoleft: *mut core::ffi::c_void, pmknewlyrunning: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IMoniker_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMoniker_Impl::IsRunning(this, windows_core::from_raw_borrowed(&pbc), windows_core::from_raw_borrowed(&pmktoleft), windows_core::from_raw_borrowed(&pmknewlyrunning)).into()
        }
        unsafe extern "system" fn GetTimeOfLastChange<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbc: *mut core::ffi::c_void, pmktoleft: *mut core::ffi::c_void, pfiletime: *mut super::super::Foundation::FILETIME) -> windows_core::HRESULT
        where
            Identity: IMoniker_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IMoniker_Impl::GetTimeOfLastChange(this, windows_core::from_raw_borrowed(&pbc), windows_core::from_raw_borrowed(&pmktoleft)) {
                Ok(ok__) => {
                    pfiletime.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Inverse<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppmk: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IMoniker_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IMoniker_Impl::Inverse(this) {
                Ok(ok__) => {
                    ppmk.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CommonPrefixWith<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pmkother: *mut core::ffi::c_void, ppmkprefix: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IMoniker_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IMoniker_Impl::CommonPrefixWith(this, windows_core::from_raw_borrowed(&pmkother)) {
                Ok(ok__) => {
                    ppmkprefix.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RelativePathTo<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pmkother: *mut core::ffi::c_void, ppmkrelpath: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IMoniker_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IMoniker_Impl::RelativePathTo(this, windows_core::from_raw_borrowed(&pmkother)) {
                Ok(ok__) => {
                    ppmkrelpath.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDisplayName<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbc: *mut core::ffi::c_void, pmktoleft: *mut core::ffi::c_void, ppszdisplayname: *mut windows_core::PWSTR) -> windows_core::HRESULT
        where
            Identity: IMoniker_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IMoniker_Impl::GetDisplayName(this, windows_core::from_raw_borrowed(&pbc), windows_core::from_raw_borrowed(&pmktoleft)) {
                Ok(ok__) => {
                    ppszdisplayname.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ParseDisplayName<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbc: *mut core::ffi::c_void, pmktoleft: *mut core::ffi::c_void, pszdisplayname: windows_core::PCWSTR, pcheaten: *mut u32, ppmkout: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IMoniker_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMoniker_Impl::ParseDisplayName(this, windows_core::from_raw_borrowed(&pbc), windows_core::from_raw_borrowed(&pmktoleft), core::mem::transmute(&pszdisplayname), core::mem::transmute_copy(&pcheaten), core::mem::transmute_copy(&ppmkout)).into()
        }
        unsafe extern "system" fn IsSystemMoniker<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwmksys: *mut u32) -> windows_core::HRESULT
        where
            Identity: IMoniker_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IMoniker_Impl::IsSystemMoniker(this) {
                Ok(ok__) => {
                    pdwmksys.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: IPersistStream_Vtbl::new::<Identity, OFFSET>(),
            BindToObject: BindToObject::<Identity, OFFSET>,
            BindToStorage: BindToStorage::<Identity, OFFSET>,
            Reduce: Reduce::<Identity, OFFSET>,
            ComposeWith: ComposeWith::<Identity, OFFSET>,
            Enum: Enum::<Identity, OFFSET>,
            IsEqual: IsEqual::<Identity, OFFSET>,
            Hash: Hash::<Identity, OFFSET>,
            IsRunning: IsRunning::<Identity, OFFSET>,
            GetTimeOfLastChange: GetTimeOfLastChange::<Identity, OFFSET>,
            Inverse: Inverse::<Identity, OFFSET>,
            CommonPrefixWith: CommonPrefixWith::<Identity, OFFSET>,
            RelativePathTo: RelativePathTo::<Identity, OFFSET>,
            GetDisplayName: GetDisplayName::<Identity, OFFSET>,
            ParseDisplayName: ParseDisplayName::<Identity, OFFSET>,
            IsSystemMoniker: IsSystemMoniker::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMoniker as windows_core::Interface>::IID || iid == &<IPersist as windows_core::Interface>::IID || iid == &<IPersistStream as windows_core::Interface>::IID
    }
}
pub trait IMultiQI_Impl: Sized {
    fn QueryMultipleInterfaces(&self, cmqis: u32, pmqis: *mut MULTI_QI) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IMultiQI {}
impl IMultiQI_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IMultiQI_Vtbl
    where
        Identity: IMultiQI_Impl,
    {
        unsafe extern "system" fn QueryMultipleInterfaces<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, cmqis: u32, pmqis: *mut MULTI_QI) -> windows_core::HRESULT
        where
            Identity: IMultiQI_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMultiQI_Impl::QueryMultipleInterfaces(this, core::mem::transmute_copy(&cmqis), core::mem::transmute_copy(&pmqis)).into()
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), QueryMultipleInterfaces: QueryMultipleInterfaces::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMultiQI as windows_core::Interface>::IID
    }
}
pub trait INoMarshal_Impl: Sized {}
impl windows_core::RuntimeName for INoMarshal {}
impl INoMarshal_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> INoMarshal_Vtbl
    where
        Identity: INoMarshal_Impl,
    {
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>() }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<INoMarshal as windows_core::Interface>::IID
    }
}
pub trait IOplockStorage_Impl: Sized {
    fn CreateStorageEx(&self, pwcsname: &windows_core::PCWSTR, grfmode: u32, stgfmt: u32, grfattrs: u32, riid: *const windows_core::GUID, ppstgopen: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
    fn OpenStorageEx(&self, pwcsname: &windows_core::PCWSTR, grfmode: u32, stgfmt: u32, grfattrs: u32, riid: *const windows_core::GUID, ppstgopen: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IOplockStorage {}
impl IOplockStorage_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IOplockStorage_Vtbl
    where
        Identity: IOplockStorage_Impl,
    {
        unsafe extern "system" fn CreateStorageEx<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwcsname: windows_core::PCWSTR, grfmode: u32, stgfmt: u32, grfattrs: u32, riid: *const windows_core::GUID, ppstgopen: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IOplockStorage_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IOplockStorage_Impl::CreateStorageEx(this, core::mem::transmute(&pwcsname), core::mem::transmute_copy(&grfmode), core::mem::transmute_copy(&stgfmt), core::mem::transmute_copy(&grfattrs), core::mem::transmute_copy(&riid), core::mem::transmute_copy(&ppstgopen)).into()
        }
        unsafe extern "system" fn OpenStorageEx<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwcsname: windows_core::PCWSTR, grfmode: u32, stgfmt: u32, grfattrs: u32, riid: *const windows_core::GUID, ppstgopen: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IOplockStorage_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IOplockStorage_Impl::OpenStorageEx(this, core::mem::transmute(&pwcsname), core::mem::transmute_copy(&grfmode), core::mem::transmute_copy(&stgfmt), core::mem::transmute_copy(&grfattrs), core::mem::transmute_copy(&riid), core::mem::transmute_copy(&ppstgopen)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            CreateStorageEx: CreateStorageEx::<Identity, OFFSET>,
            OpenStorageEx: OpenStorageEx::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IOplockStorage as windows_core::Interface>::IID
    }
}
pub trait IPSFactoryBuffer_Impl: Sized {
    fn CreateProxy(&self, punkouter: Option<&windows_core::IUnknown>, riid: *const windows_core::GUID, ppproxy: *mut Option<IRpcProxyBuffer>, ppv: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
    fn CreateStub(&self, riid: *const windows_core::GUID, punkserver: Option<&windows_core::IUnknown>) -> windows_core::Result<IRpcStubBuffer>;
}
impl windows_core::RuntimeName for IPSFactoryBuffer {}
impl IPSFactoryBuffer_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IPSFactoryBuffer_Vtbl
    where
        Identity: IPSFactoryBuffer_Impl,
    {
        unsafe extern "system" fn CreateProxy<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, punkouter: *mut core::ffi::c_void, riid: *const windows_core::GUID, ppproxy: *mut *mut core::ffi::c_void, ppv: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IPSFactoryBuffer_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IPSFactoryBuffer_Impl::CreateProxy(this, windows_core::from_raw_borrowed(&punkouter), core::mem::transmute_copy(&riid), core::mem::transmute_copy(&ppproxy), core::mem::transmute_copy(&ppv)).into()
        }
        unsafe extern "system" fn CreateStub<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, riid: *const windows_core::GUID, punkserver: *mut core::ffi::c_void, ppstub: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IPSFactoryBuffer_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IPSFactoryBuffer_Impl::CreateStub(this, core::mem::transmute_copy(&riid), windows_core::from_raw_borrowed(&punkserver)) {
                Ok(ok__) => {
                    ppstub.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            CreateProxy: CreateProxy::<Identity, OFFSET>,
            CreateStub: CreateStub::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IPSFactoryBuffer as windows_core::Interface>::IID
    }
}
pub trait IPersist_Impl: Sized {
    fn GetClassID(&self) -> windows_core::Result<windows_core::GUID>;
}
impl windows_core::RuntimeName for IPersist {}
impl IPersist_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IPersist_Vtbl
    where
        Identity: IPersist_Impl,
    {
        unsafe extern "system" fn GetClassID<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pclassid: *mut windows_core::GUID) -> windows_core::HRESULT
        where
            Identity: IPersist_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IPersist_Impl::GetClassID(this) {
                Ok(ok__) => {
                    pclassid.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), GetClassID: GetClassID::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IPersist as windows_core::Interface>::IID
    }
}
pub trait IPersistFile_Impl: Sized + IPersist_Impl {
    fn IsDirty(&self) -> windows_core::HRESULT;
    fn Load(&self, pszfilename: &windows_core::PCWSTR, dwmode: STGM) -> windows_core::Result<()>;
    fn Save(&self, pszfilename: &windows_core::PCWSTR, fremember: super::super::Foundation::BOOL) -> windows_core::Result<()>;
    fn SaveCompleted(&self, pszfilename: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn GetCurFile(&self) -> windows_core::Result<windows_core::PWSTR>;
}
impl windows_core::RuntimeName for IPersistFile {}
impl IPersistFile_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IPersistFile_Vtbl
    where
        Identity: IPersistFile_Impl,
    {
        unsafe extern "system" fn IsDirty<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IPersistFile_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IPersistFile_Impl::IsDirty(this)
        }
        unsafe extern "system" fn Load<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszfilename: windows_core::PCWSTR, dwmode: STGM) -> windows_core::HRESULT
        where
            Identity: IPersistFile_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IPersistFile_Impl::Load(this, core::mem::transmute(&pszfilename), core::mem::transmute_copy(&dwmode)).into()
        }
        unsafe extern "system" fn Save<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszfilename: windows_core::PCWSTR, fremember: super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: IPersistFile_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IPersistFile_Impl::Save(this, core::mem::transmute(&pszfilename), core::mem::transmute_copy(&fremember)).into()
        }
        unsafe extern "system" fn SaveCompleted<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszfilename: windows_core::PCWSTR) -> windows_core::HRESULT
        where
            Identity: IPersistFile_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IPersistFile_Impl::SaveCompleted(this, core::mem::transmute(&pszfilename)).into()
        }
        unsafe extern "system" fn GetCurFile<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppszfilename: *mut windows_core::PWSTR) -> windows_core::HRESULT
        where
            Identity: IPersistFile_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IPersistFile_Impl::GetCurFile(this) {
                Ok(ok__) => {
                    ppszfilename.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: IPersist_Vtbl::new::<Identity, OFFSET>(),
            IsDirty: IsDirty::<Identity, OFFSET>,
            Load: Load::<Identity, OFFSET>,
            Save: Save::<Identity, OFFSET>,
            SaveCompleted: SaveCompleted::<Identity, OFFSET>,
            GetCurFile: GetCurFile::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IPersistFile as windows_core::Interface>::IID || iid == &<IPersist as windows_core::Interface>::IID
    }
}
pub trait IPersistMemory_Impl: Sized + IPersist_Impl {
    fn IsDirty(&self) -> windows_core::HRESULT;
    fn Load(&self, pmem: *const core::ffi::c_void, cbsize: u32) -> windows_core::Result<()>;
    fn Save(&self, pmem: *mut core::ffi::c_void, fcleardirty: super::super::Foundation::BOOL, cbsize: u32) -> windows_core::Result<()>;
    fn GetSizeMax(&self) -> windows_core::Result<u32>;
    fn InitNew(&self) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IPersistMemory {}
impl IPersistMemory_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IPersistMemory_Vtbl
    where
        Identity: IPersistMemory_Impl,
    {
        unsafe extern "system" fn IsDirty<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IPersistMemory_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IPersistMemory_Impl::IsDirty(this)
        }
        unsafe extern "system" fn Load<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pmem: *const core::ffi::c_void, cbsize: u32) -> windows_core::HRESULT
        where
            Identity: IPersistMemory_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IPersistMemory_Impl::Load(this, core::mem::transmute_copy(&pmem), core::mem::transmute_copy(&cbsize)).into()
        }
        unsafe extern "system" fn Save<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pmem: *mut core::ffi::c_void, fcleardirty: super::super::Foundation::BOOL, cbsize: u32) -> windows_core::HRESULT
        where
            Identity: IPersistMemory_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IPersistMemory_Impl::Save(this, core::mem::transmute_copy(&pmem), core::mem::transmute_copy(&fcleardirty), core::mem::transmute_copy(&cbsize)).into()
        }
        unsafe extern "system" fn GetSizeMax<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcbsize: *mut u32) -> windows_core::HRESULT
        where
            Identity: IPersistMemory_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IPersistMemory_Impl::GetSizeMax(this) {
                Ok(ok__) => {
                    pcbsize.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn InitNew<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IPersistMemory_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IPersistMemory_Impl::InitNew(this).into()
        }
        Self {
            base__: IPersist_Vtbl::new::<Identity, OFFSET>(),
            IsDirty: IsDirty::<Identity, OFFSET>,
            Load: Load::<Identity, OFFSET>,
            Save: Save::<Identity, OFFSET>,
            GetSizeMax: GetSizeMax::<Identity, OFFSET>,
            InitNew: InitNew::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IPersistMemory as windows_core::Interface>::IID || iid == &<IPersist as windows_core::Interface>::IID
    }
}
pub trait IPersistStream_Impl: Sized + IPersist_Impl {
    fn IsDirty(&self) -> windows_core::HRESULT;
    fn Load(&self, pstm: Option<&IStream>) -> windows_core::Result<()>;
    fn Save(&self, pstm: Option<&IStream>, fcleardirty: super::super::Foundation::BOOL) -> windows_core::Result<()>;
    fn GetSizeMax(&self) -> windows_core::Result<u64>;
}
impl windows_core::RuntimeName for IPersistStream {}
impl IPersistStream_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IPersistStream_Vtbl
    where
        Identity: IPersistStream_Impl,
    {
        unsafe extern "system" fn IsDirty<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IPersistStream_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IPersistStream_Impl::IsDirty(this)
        }
        unsafe extern "system" fn Load<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pstm: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IPersistStream_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IPersistStream_Impl::Load(this, windows_core::from_raw_borrowed(&pstm)).into()
        }
        unsafe extern "system" fn Save<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pstm: *mut core::ffi::c_void, fcleardirty: super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: IPersistStream_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IPersistStream_Impl::Save(this, windows_core::from_raw_borrowed(&pstm), core::mem::transmute_copy(&fcleardirty)).into()
        }
        unsafe extern "system" fn GetSizeMax<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcbsize: *mut u64) -> windows_core::HRESULT
        where
            Identity: IPersistStream_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IPersistStream_Impl::GetSizeMax(this) {
                Ok(ok__) => {
                    pcbsize.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: IPersist_Vtbl::new::<Identity, OFFSET>(),
            IsDirty: IsDirty::<Identity, OFFSET>,
            Load: Load::<Identity, OFFSET>,
            Save: Save::<Identity, OFFSET>,
            GetSizeMax: GetSizeMax::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IPersistStream as windows_core::Interface>::IID || iid == &<IPersist as windows_core::Interface>::IID
    }
}
pub trait IPersistStreamInit_Impl: Sized + IPersist_Impl {
    fn IsDirty(&self) -> windows_core::HRESULT;
    fn Load(&self, pstm: Option<&IStream>) -> windows_core::Result<()>;
    fn Save(&self, pstm: Option<&IStream>, fcleardirty: super::super::Foundation::BOOL) -> windows_core::Result<()>;
    fn GetSizeMax(&self) -> windows_core::Result<u64>;
    fn InitNew(&self) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IPersistStreamInit {}
impl IPersistStreamInit_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IPersistStreamInit_Vtbl
    where
        Identity: IPersistStreamInit_Impl,
    {
        unsafe extern "system" fn IsDirty<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IPersistStreamInit_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IPersistStreamInit_Impl::IsDirty(this)
        }
        unsafe extern "system" fn Load<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pstm: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IPersistStreamInit_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IPersistStreamInit_Impl::Load(this, windows_core::from_raw_borrowed(&pstm)).into()
        }
        unsafe extern "system" fn Save<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pstm: *mut core::ffi::c_void, fcleardirty: super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: IPersistStreamInit_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IPersistStreamInit_Impl::Save(this, windows_core::from_raw_borrowed(&pstm), core::mem::transmute_copy(&fcleardirty)).into()
        }
        unsafe extern "system" fn GetSizeMax<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcbsize: *mut u64) -> windows_core::HRESULT
        where
            Identity: IPersistStreamInit_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IPersistStreamInit_Impl::GetSizeMax(this) {
                Ok(ok__) => {
                    pcbsize.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn InitNew<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IPersistStreamInit_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IPersistStreamInit_Impl::InitNew(this).into()
        }
        Self {
            base__: IPersist_Vtbl::new::<Identity, OFFSET>(),
            IsDirty: IsDirty::<Identity, OFFSET>,
            Load: Load::<Identity, OFFSET>,
            Save: Save::<Identity, OFFSET>,
            GetSizeMax: GetSizeMax::<Identity, OFFSET>,
            InitNew: InitNew::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IPersistStreamInit as windows_core::Interface>::IID || iid == &<IPersist as windows_core::Interface>::IID
    }
}
pub trait IPipeByte_Impl: Sized {
    fn Pull(&self, buf: *mut u8, crequest: u32, pcreturned: *mut u32) -> windows_core::Result<()>;
    fn Push(&self, buf: *const u8, csent: u32) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IPipeByte {}
impl IPipeByte_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IPipeByte_Vtbl
    where
        Identity: IPipeByte_Impl,
    {
        unsafe extern "system" fn Pull<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, buf: *mut u8, crequest: u32, pcreturned: *mut u32) -> windows_core::HRESULT
        where
            Identity: IPipeByte_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IPipeByte_Impl::Pull(this, core::mem::transmute_copy(&buf), core::mem::transmute_copy(&crequest), core::mem::transmute_copy(&pcreturned)).into()
        }
        unsafe extern "system" fn Push<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, buf: *const u8, csent: u32) -> windows_core::HRESULT
        where
            Identity: IPipeByte_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IPipeByte_Impl::Push(this, core::mem::transmute_copy(&buf), core::mem::transmute_copy(&csent)).into()
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), Pull: Pull::<Identity, OFFSET>, Push: Push::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IPipeByte as windows_core::Interface>::IID
    }
}
pub trait IPipeDouble_Impl: Sized {
    fn Pull(&self, buf: *mut f64, crequest: u32, pcreturned: *mut u32) -> windows_core::Result<()>;
    fn Push(&self, buf: *const f64, csent: u32) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IPipeDouble {}
impl IPipeDouble_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IPipeDouble_Vtbl
    where
        Identity: IPipeDouble_Impl,
    {
        unsafe extern "system" fn Pull<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, buf: *mut f64, crequest: u32, pcreturned: *mut u32) -> windows_core::HRESULT
        where
            Identity: IPipeDouble_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IPipeDouble_Impl::Pull(this, core::mem::transmute_copy(&buf), core::mem::transmute_copy(&crequest), core::mem::transmute_copy(&pcreturned)).into()
        }
        unsafe extern "system" fn Push<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, buf: *const f64, csent: u32) -> windows_core::HRESULT
        where
            Identity: IPipeDouble_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IPipeDouble_Impl::Push(this, core::mem::transmute_copy(&buf), core::mem::transmute_copy(&csent)).into()
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), Pull: Pull::<Identity, OFFSET>, Push: Push::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IPipeDouble as windows_core::Interface>::IID
    }
}
pub trait IPipeLong_Impl: Sized {
    fn Pull(&self, buf: *mut i32, crequest: u32, pcreturned: *mut u32) -> windows_core::Result<()>;
    fn Push(&self, buf: *const i32, csent: u32) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IPipeLong {}
impl IPipeLong_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IPipeLong_Vtbl
    where
        Identity: IPipeLong_Impl,
    {
        unsafe extern "system" fn Pull<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, buf: *mut i32, crequest: u32, pcreturned: *mut u32) -> windows_core::HRESULT
        where
            Identity: IPipeLong_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IPipeLong_Impl::Pull(this, core::mem::transmute_copy(&buf), core::mem::transmute_copy(&crequest), core::mem::transmute_copy(&pcreturned)).into()
        }
        unsafe extern "system" fn Push<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, buf: *const i32, csent: u32) -> windows_core::HRESULT
        where
            Identity: IPipeLong_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IPipeLong_Impl::Push(this, core::mem::transmute_copy(&buf), core::mem::transmute_copy(&csent)).into()
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), Pull: Pull::<Identity, OFFSET>, Push: Push::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IPipeLong as windows_core::Interface>::IID
    }
}
pub trait IProcessInitControl_Impl: Sized {
    fn ResetInitializerTimeout(&self, dwsecondsremaining: u32) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IProcessInitControl {}
impl IProcessInitControl_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IProcessInitControl_Vtbl
    where
        Identity: IProcessInitControl_Impl,
    {
        unsafe extern "system" fn ResetInitializerTimeout<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwsecondsremaining: u32) -> windows_core::HRESULT
        where
            Identity: IProcessInitControl_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IProcessInitControl_Impl::ResetInitializerTimeout(this, core::mem::transmute_copy(&dwsecondsremaining)).into()
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), ResetInitializerTimeout: ResetInitializerTimeout::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IProcessInitControl as windows_core::Interface>::IID
    }
}
pub trait IProcessLock_Impl: Sized {
    fn AddRefOnProcess(&self) -> u32;
    fn ReleaseRefOnProcess(&self) -> u32;
}
impl windows_core::RuntimeName for IProcessLock {}
impl IProcessLock_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IProcessLock_Vtbl
    where
        Identity: IProcessLock_Impl,
    {
        unsafe extern "system" fn AddRefOnProcess<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> u32
        where
            Identity: IProcessLock_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IProcessLock_Impl::AddRefOnProcess(this)
        }
        unsafe extern "system" fn ReleaseRefOnProcess<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> u32
        where
            Identity: IProcessLock_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IProcessLock_Impl::ReleaseRefOnProcess(this)
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            AddRefOnProcess: AddRefOnProcess::<Identity, OFFSET>,
            ReleaseRefOnProcess: ReleaseRefOnProcess::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IProcessLock as windows_core::Interface>::IID
    }
}
pub trait IProgressNotify_Impl: Sized {
    fn OnProgress(&self, dwprogresscurrent: u32, dwprogressmaximum: u32, faccurate: super::super::Foundation::BOOL, fowner: super::super::Foundation::BOOL) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IProgressNotify {}
impl IProgressNotify_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IProgressNotify_Vtbl
    where
        Identity: IProgressNotify_Impl,
    {
        unsafe extern "system" fn OnProgress<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwprogresscurrent: u32, dwprogressmaximum: u32, faccurate: super::super::Foundation::BOOL, fowner: super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: IProgressNotify_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IProgressNotify_Impl::OnProgress(this, core::mem::transmute_copy(&dwprogresscurrent), core::mem::transmute_copy(&dwprogressmaximum), core::mem::transmute_copy(&faccurate), core::mem::transmute_copy(&fowner)).into()
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), OnProgress: OnProgress::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IProgressNotify as windows_core::Interface>::IID
    }
}
pub trait IROTData_Impl: Sized {
    fn GetComparisonData(&self, pbdata: *mut u8, cbmax: u32, pcbdata: *mut u32) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IROTData {}
impl IROTData_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IROTData_Vtbl
    where
        Identity: IROTData_Impl,
    {
        unsafe extern "system" fn GetComparisonData<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbdata: *mut u8, cbmax: u32, pcbdata: *mut u32) -> windows_core::HRESULT
        where
            Identity: IROTData_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IROTData_Impl::GetComparisonData(this, core::mem::transmute_copy(&pbdata), core::mem::transmute_copy(&cbmax), core::mem::transmute_copy(&pcbdata)).into()
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), GetComparisonData: GetComparisonData::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IROTData as windows_core::Interface>::IID
    }
}
pub trait IReleaseMarshalBuffers_Impl: Sized {
    fn ReleaseMarshalBuffer(&self, pmsg: *mut RPCOLEMESSAGE, dwflags: u32, pchnl: Option<&windows_core::IUnknown>) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IReleaseMarshalBuffers {}
impl IReleaseMarshalBuffers_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IReleaseMarshalBuffers_Vtbl
    where
        Identity: IReleaseMarshalBuffers_Impl,
    {
        unsafe extern "system" fn ReleaseMarshalBuffer<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pmsg: *mut RPCOLEMESSAGE, dwflags: u32, pchnl: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IReleaseMarshalBuffers_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IReleaseMarshalBuffers_Impl::ReleaseMarshalBuffer(this, core::mem::transmute_copy(&pmsg), core::mem::transmute_copy(&dwflags), windows_core::from_raw_borrowed(&pchnl)).into()
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), ReleaseMarshalBuffer: ReleaseMarshalBuffer::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IReleaseMarshalBuffers as windows_core::Interface>::IID
    }
}
pub trait IRpcChannelBuffer_Impl: Sized {
    fn GetBuffer(&self, pmessage: *mut RPCOLEMESSAGE, riid: *const windows_core::GUID) -> windows_core::Result<()>;
    fn SendReceive(&self, pmessage: *mut RPCOLEMESSAGE, pstatus: *mut u32) -> windows_core::Result<()>;
    fn FreeBuffer(&self, pmessage: *mut RPCOLEMESSAGE) -> windows_core::Result<()>;
    fn GetDestCtx(&self, pdwdestcontext: *mut u32, ppvdestcontext: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
    fn IsConnected(&self) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IRpcChannelBuffer {}
impl IRpcChannelBuffer_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IRpcChannelBuffer_Vtbl
    where
        Identity: IRpcChannelBuffer_Impl,
    {
        unsafe extern "system" fn GetBuffer<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pmessage: *mut RPCOLEMESSAGE, riid: *const windows_core::GUID) -> windows_core::HRESULT
        where
            Identity: IRpcChannelBuffer_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IRpcChannelBuffer_Impl::GetBuffer(this, core::mem::transmute_copy(&pmessage), core::mem::transmute_copy(&riid)).into()
        }
        unsafe extern "system" fn SendReceive<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pmessage: *mut RPCOLEMESSAGE, pstatus: *mut u32) -> windows_core::HRESULT
        where
            Identity: IRpcChannelBuffer_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IRpcChannelBuffer_Impl::SendReceive(this, core::mem::transmute_copy(&pmessage), core::mem::transmute_copy(&pstatus)).into()
        }
        unsafe extern "system" fn FreeBuffer<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pmessage: *mut RPCOLEMESSAGE) -> windows_core::HRESULT
        where
            Identity: IRpcChannelBuffer_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IRpcChannelBuffer_Impl::FreeBuffer(this, core::mem::transmute_copy(&pmessage)).into()
        }
        unsafe extern "system" fn GetDestCtx<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwdestcontext: *mut u32, ppvdestcontext: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IRpcChannelBuffer_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IRpcChannelBuffer_Impl::GetDestCtx(this, core::mem::transmute_copy(&pdwdestcontext), core::mem::transmute_copy(&ppvdestcontext)).into()
        }
        unsafe extern "system" fn IsConnected<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IRpcChannelBuffer_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IRpcChannelBuffer_Impl::IsConnected(this).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetBuffer: GetBuffer::<Identity, OFFSET>,
            SendReceive: SendReceive::<Identity, OFFSET>,
            FreeBuffer: FreeBuffer::<Identity, OFFSET>,
            GetDestCtx: GetDestCtx::<Identity, OFFSET>,
            IsConnected: IsConnected::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IRpcChannelBuffer as windows_core::Interface>::IID
    }
}
pub trait IRpcChannelBuffer2_Impl: Sized + IRpcChannelBuffer_Impl {
    fn GetProtocolVersion(&self) -> windows_core::Result<u32>;
}
impl windows_core::RuntimeName for IRpcChannelBuffer2 {}
impl IRpcChannelBuffer2_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IRpcChannelBuffer2_Vtbl
    where
        Identity: IRpcChannelBuffer2_Impl,
    {
        unsafe extern "system" fn GetProtocolVersion<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwversion: *mut u32) -> windows_core::HRESULT
        where
            Identity: IRpcChannelBuffer2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IRpcChannelBuffer2_Impl::GetProtocolVersion(this) {
                Ok(ok__) => {
                    pdwversion.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self { base__: IRpcChannelBuffer_Vtbl::new::<Identity, OFFSET>(), GetProtocolVersion: GetProtocolVersion::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IRpcChannelBuffer2 as windows_core::Interface>::IID || iid == &<IRpcChannelBuffer as windows_core::Interface>::IID
    }
}
pub trait IRpcChannelBuffer3_Impl: Sized + IRpcChannelBuffer2_Impl {
    fn Send(&self, pmsg: *mut RPCOLEMESSAGE, pulstatus: *mut u32) -> windows_core::Result<()>;
    fn Receive(&self, pmsg: *mut RPCOLEMESSAGE, ulsize: u32, pulstatus: *mut u32) -> windows_core::Result<()>;
    fn Cancel(&self, pmsg: *mut RPCOLEMESSAGE) -> windows_core::Result<()>;
    fn GetCallContext(&self, pmsg: *const RPCOLEMESSAGE, riid: *const windows_core::GUID, pinterface: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
    fn GetDestCtxEx(&self, pmsg: *const RPCOLEMESSAGE, pdwdestcontext: *mut u32, ppvdestcontext: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
    fn GetState(&self, pmsg: *const RPCOLEMESSAGE) -> windows_core::Result<u32>;
    fn RegisterAsync(&self, pmsg: *mut RPCOLEMESSAGE, pasyncmgr: Option<&IAsyncManager>) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IRpcChannelBuffer3 {}
impl IRpcChannelBuffer3_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IRpcChannelBuffer3_Vtbl
    where
        Identity: IRpcChannelBuffer3_Impl,
    {
        unsafe extern "system" fn Send<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pmsg: *mut RPCOLEMESSAGE, pulstatus: *mut u32) -> windows_core::HRESULT
        where
            Identity: IRpcChannelBuffer3_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IRpcChannelBuffer3_Impl::Send(this, core::mem::transmute_copy(&pmsg), core::mem::transmute_copy(&pulstatus)).into()
        }
        unsafe extern "system" fn Receive<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pmsg: *mut RPCOLEMESSAGE, ulsize: u32, pulstatus: *mut u32) -> windows_core::HRESULT
        where
            Identity: IRpcChannelBuffer3_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IRpcChannelBuffer3_Impl::Receive(this, core::mem::transmute_copy(&pmsg), core::mem::transmute_copy(&ulsize), core::mem::transmute_copy(&pulstatus)).into()
        }
        unsafe extern "system" fn Cancel<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pmsg: *mut RPCOLEMESSAGE) -> windows_core::HRESULT
        where
            Identity: IRpcChannelBuffer3_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IRpcChannelBuffer3_Impl::Cancel(this, core::mem::transmute_copy(&pmsg)).into()
        }
        unsafe extern "system" fn GetCallContext<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pmsg: *const RPCOLEMESSAGE, riid: *const windows_core::GUID, pinterface: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IRpcChannelBuffer3_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IRpcChannelBuffer3_Impl::GetCallContext(this, core::mem::transmute_copy(&pmsg), core::mem::transmute_copy(&riid), core::mem::transmute_copy(&pinterface)).into()
        }
        unsafe extern "system" fn GetDestCtxEx<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pmsg: *const RPCOLEMESSAGE, pdwdestcontext: *mut u32, ppvdestcontext: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IRpcChannelBuffer3_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IRpcChannelBuffer3_Impl::GetDestCtxEx(this, core::mem::transmute_copy(&pmsg), core::mem::transmute_copy(&pdwdestcontext), core::mem::transmute_copy(&ppvdestcontext)).into()
        }
        unsafe extern "system" fn GetState<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pmsg: *const RPCOLEMESSAGE, pstate: *mut u32) -> windows_core::HRESULT
        where
            Identity: IRpcChannelBuffer3_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IRpcChannelBuffer3_Impl::GetState(this, core::mem::transmute_copy(&pmsg)) {
                Ok(ok__) => {
                    pstate.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RegisterAsync<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pmsg: *mut RPCOLEMESSAGE, pasyncmgr: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IRpcChannelBuffer3_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IRpcChannelBuffer3_Impl::RegisterAsync(this, core::mem::transmute_copy(&pmsg), windows_core::from_raw_borrowed(&pasyncmgr)).into()
        }
        Self {
            base__: IRpcChannelBuffer2_Vtbl::new::<Identity, OFFSET>(),
            Send: Send::<Identity, OFFSET>,
            Receive: Receive::<Identity, OFFSET>,
            Cancel: Cancel::<Identity, OFFSET>,
            GetCallContext: GetCallContext::<Identity, OFFSET>,
            GetDestCtxEx: GetDestCtxEx::<Identity, OFFSET>,
            GetState: GetState::<Identity, OFFSET>,
            RegisterAsync: RegisterAsync::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IRpcChannelBuffer3 as windows_core::Interface>::IID || iid == &<IRpcChannelBuffer as windows_core::Interface>::IID || iid == &<IRpcChannelBuffer2 as windows_core::Interface>::IID
    }
}
pub trait IRpcHelper_Impl: Sized {
    fn GetDCOMProtocolVersion(&self) -> windows_core::Result<u32>;
    fn GetIIDFromOBJREF(&self, pobjref: *const core::ffi::c_void) -> windows_core::Result<*mut windows_core::GUID>;
}
impl windows_core::RuntimeName for IRpcHelper {}
impl IRpcHelper_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IRpcHelper_Vtbl
    where
        Identity: IRpcHelper_Impl,
    {
        unsafe extern "system" fn GetDCOMProtocolVersion<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcomversion: *mut u32) -> windows_core::HRESULT
        where
            Identity: IRpcHelper_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IRpcHelper_Impl::GetDCOMProtocolVersion(this) {
                Ok(ok__) => {
                    pcomversion.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetIIDFromOBJREF<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pobjref: *const core::ffi::c_void, piid: *mut *mut windows_core::GUID) -> windows_core::HRESULT
        where
            Identity: IRpcHelper_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IRpcHelper_Impl::GetIIDFromOBJREF(this, core::mem::transmute_copy(&pobjref)) {
                Ok(ok__) => {
                    piid.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetDCOMProtocolVersion: GetDCOMProtocolVersion::<Identity, OFFSET>,
            GetIIDFromOBJREF: GetIIDFromOBJREF::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IRpcHelper as windows_core::Interface>::IID
    }
}
pub trait IRpcOptions_Impl: Sized {
    fn Set(&self, pprx: Option<&windows_core::IUnknown>, dwproperty: RPCOPT_PROPERTIES, dwvalue: usize) -> windows_core::Result<()>;
    fn Query(&self, pprx: Option<&windows_core::IUnknown>, dwproperty: RPCOPT_PROPERTIES) -> windows_core::Result<usize>;
}
impl windows_core::RuntimeName for IRpcOptions {}
impl IRpcOptions_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IRpcOptions_Vtbl
    where
        Identity: IRpcOptions_Impl,
    {
        unsafe extern "system" fn Set<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pprx: *mut core::ffi::c_void, dwproperty: RPCOPT_PROPERTIES, dwvalue: usize) -> windows_core::HRESULT
        where
            Identity: IRpcOptions_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IRpcOptions_Impl::Set(this, windows_core::from_raw_borrowed(&pprx), core::mem::transmute_copy(&dwproperty), core::mem::transmute_copy(&dwvalue)).into()
        }
        unsafe extern "system" fn Query<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pprx: *mut core::ffi::c_void, dwproperty: RPCOPT_PROPERTIES, pdwvalue: *mut usize) -> windows_core::HRESULT
        where
            Identity: IRpcOptions_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IRpcOptions_Impl::Query(this, windows_core::from_raw_borrowed(&pprx), core::mem::transmute_copy(&dwproperty)) {
                Ok(ok__) => {
                    pdwvalue.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), Set: Set::<Identity, OFFSET>, Query: Query::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IRpcOptions as windows_core::Interface>::IID
    }
}
pub trait IRpcProxyBuffer_Impl: Sized {
    fn Connect(&self, prpcchannelbuffer: Option<&IRpcChannelBuffer>) -> windows_core::Result<()>;
    fn Disconnect(&self);
}
impl windows_core::RuntimeName for IRpcProxyBuffer {}
impl IRpcProxyBuffer_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IRpcProxyBuffer_Vtbl
    where
        Identity: IRpcProxyBuffer_Impl,
    {
        unsafe extern "system" fn Connect<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, prpcchannelbuffer: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IRpcProxyBuffer_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IRpcProxyBuffer_Impl::Connect(this, windows_core::from_raw_borrowed(&prpcchannelbuffer)).into()
        }
        unsafe extern "system" fn Disconnect<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void)
        where
            Identity: IRpcProxyBuffer_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IRpcProxyBuffer_Impl::Disconnect(this)
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), Connect: Connect::<Identity, OFFSET>, Disconnect: Disconnect::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IRpcProxyBuffer as windows_core::Interface>::IID
    }
}
pub trait IRpcStubBuffer_Impl: Sized {
    fn Connect(&self, punkserver: Option<&windows_core::IUnknown>) -> windows_core::Result<()>;
    fn Disconnect(&self);
    fn Invoke(&self, _prpcmsg: *mut RPCOLEMESSAGE, _prpcchannelbuffer: Option<&IRpcChannelBuffer>) -> windows_core::Result<()>;
    fn IsIIDSupported(&self, riid: *const windows_core::GUID) -> Option<IRpcStubBuffer>;
    fn CountRefs(&self) -> u32;
    fn DebugServerQueryInterface(&self, ppv: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
    fn DebugServerRelease(&self, pv: *const core::ffi::c_void);
}
impl windows_core::RuntimeName for IRpcStubBuffer {}
impl IRpcStubBuffer_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IRpcStubBuffer_Vtbl
    where
        Identity: IRpcStubBuffer_Impl,
    {
        unsafe extern "system" fn Connect<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, punkserver: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IRpcStubBuffer_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IRpcStubBuffer_Impl::Connect(this, windows_core::from_raw_borrowed(&punkserver)).into()
        }
        unsafe extern "system" fn Disconnect<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void)
        where
            Identity: IRpcStubBuffer_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IRpcStubBuffer_Impl::Disconnect(this)
        }
        unsafe extern "system" fn Invoke<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, _prpcmsg: *mut RPCOLEMESSAGE, _prpcchannelbuffer: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IRpcStubBuffer_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IRpcStubBuffer_Impl::Invoke(this, core::mem::transmute_copy(&_prpcmsg), windows_core::from_raw_borrowed(&_prpcchannelbuffer)).into()
        }
        unsafe extern "system" fn IsIIDSupported<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, riid: *const windows_core::GUID) -> Option<IRpcStubBuffer>
        where
            Identity: IRpcStubBuffer_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IRpcStubBuffer_Impl::IsIIDSupported(this, core::mem::transmute_copy(&riid))
        }
        unsafe extern "system" fn CountRefs<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> u32
        where
            Identity: IRpcStubBuffer_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IRpcStubBuffer_Impl::CountRefs(this)
        }
        unsafe extern "system" fn DebugServerQueryInterface<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppv: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IRpcStubBuffer_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IRpcStubBuffer_Impl::DebugServerQueryInterface(this, core::mem::transmute_copy(&ppv)).into()
        }
        unsafe extern "system" fn DebugServerRelease<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pv: *const core::ffi::c_void)
        where
            Identity: IRpcStubBuffer_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IRpcStubBuffer_Impl::DebugServerRelease(this, core::mem::transmute_copy(&pv))
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Connect: Connect::<Identity, OFFSET>,
            Disconnect: Disconnect::<Identity, OFFSET>,
            Invoke: Invoke::<Identity, OFFSET>,
            IsIIDSupported: IsIIDSupported::<Identity, OFFSET>,
            CountRefs: CountRefs::<Identity, OFFSET>,
            DebugServerQueryInterface: DebugServerQueryInterface::<Identity, OFFSET>,
            DebugServerRelease: DebugServerRelease::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IRpcStubBuffer as windows_core::Interface>::IID
    }
}
pub trait IRpcSyntaxNegotiate_Impl: Sized {
    fn NegotiateSyntax(&self, pmsg: *mut RPCOLEMESSAGE) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IRpcSyntaxNegotiate {}
impl IRpcSyntaxNegotiate_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IRpcSyntaxNegotiate_Vtbl
    where
        Identity: IRpcSyntaxNegotiate_Impl,
    {
        unsafe extern "system" fn NegotiateSyntax<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pmsg: *mut RPCOLEMESSAGE) -> windows_core::HRESULT
        where
            Identity: IRpcSyntaxNegotiate_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IRpcSyntaxNegotiate_Impl::NegotiateSyntax(this, core::mem::transmute_copy(&pmsg)).into()
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), NegotiateSyntax: NegotiateSyntax::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IRpcSyntaxNegotiate as windows_core::Interface>::IID
    }
}
pub trait IRunnableObject_Impl: Sized {
    fn GetRunningClass(&self) -> windows_core::Result<windows_core::GUID>;
    fn Run(&self, pbc: Option<&IBindCtx>) -> windows_core::Result<()>;
    fn IsRunning(&self) -> super::super::Foundation::BOOL;
    fn LockRunning(&self, flock: super::super::Foundation::BOOL, flastunlockcloses: super::super::Foundation::BOOL) -> windows_core::Result<()>;
    fn SetContainedObject(&self, fcontained: super::super::Foundation::BOOL) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IRunnableObject {}
impl IRunnableObject_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IRunnableObject_Vtbl
    where
        Identity: IRunnableObject_Impl,
    {
        unsafe extern "system" fn GetRunningClass<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, lpclsid: *mut windows_core::GUID) -> windows_core::HRESULT
        where
            Identity: IRunnableObject_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IRunnableObject_Impl::GetRunningClass(this) {
                Ok(ok__) => {
                    lpclsid.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Run<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbc: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IRunnableObject_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IRunnableObject_Impl::Run(this, windows_core::from_raw_borrowed(&pbc)).into()
        }
        unsafe extern "system" fn IsRunning<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> super::super::Foundation::BOOL
        where
            Identity: IRunnableObject_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IRunnableObject_Impl::IsRunning(this)
        }
        unsafe extern "system" fn LockRunning<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, flock: super::super::Foundation::BOOL, flastunlockcloses: super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: IRunnableObject_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IRunnableObject_Impl::LockRunning(this, core::mem::transmute_copy(&flock), core::mem::transmute_copy(&flastunlockcloses)).into()
        }
        unsafe extern "system" fn SetContainedObject<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, fcontained: super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: IRunnableObject_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IRunnableObject_Impl::SetContainedObject(this, core::mem::transmute_copy(&fcontained)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetRunningClass: GetRunningClass::<Identity, OFFSET>,
            Run: Run::<Identity, OFFSET>,
            IsRunning: IsRunning::<Identity, OFFSET>,
            LockRunning: LockRunning::<Identity, OFFSET>,
            SetContainedObject: SetContainedObject::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IRunnableObject as windows_core::Interface>::IID
    }
}
pub trait IRunningObjectTable_Impl: Sized {
    fn Register(&self, grfflags: ROT_FLAGS, punkobject: Option<&windows_core::IUnknown>, pmkobjectname: Option<&IMoniker>) -> windows_core::Result<u32>;
    fn Revoke(&self, dwregister: u32) -> windows_core::Result<()>;
    fn IsRunning(&self, pmkobjectname: Option<&IMoniker>) -> windows_core::Result<()>;
    fn GetObject(&self, pmkobjectname: Option<&IMoniker>) -> windows_core::Result<windows_core::IUnknown>;
    fn NoteChangeTime(&self, dwregister: u32, pfiletime: *const super::super::Foundation::FILETIME) -> windows_core::Result<()>;
    fn GetTimeOfLastChange(&self, pmkobjectname: Option<&IMoniker>) -> windows_core::Result<super::super::Foundation::FILETIME>;
    fn EnumRunning(&self) -> windows_core::Result<IEnumMoniker>;
}
impl windows_core::RuntimeName for IRunningObjectTable {}
impl IRunningObjectTable_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IRunningObjectTable_Vtbl
    where
        Identity: IRunningObjectTable_Impl,
    {
        unsafe extern "system" fn Register<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, grfflags: ROT_FLAGS, punkobject: *mut core::ffi::c_void, pmkobjectname: *mut core::ffi::c_void, pdwregister: *mut u32) -> windows_core::HRESULT
        where
            Identity: IRunningObjectTable_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IRunningObjectTable_Impl::Register(this, core::mem::transmute_copy(&grfflags), windows_core::from_raw_borrowed(&punkobject), windows_core::from_raw_borrowed(&pmkobjectname)) {
                Ok(ok__) => {
                    pdwregister.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Revoke<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwregister: u32) -> windows_core::HRESULT
        where
            Identity: IRunningObjectTable_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IRunningObjectTable_Impl::Revoke(this, core::mem::transmute_copy(&dwregister)).into()
        }
        unsafe extern "system" fn IsRunning<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pmkobjectname: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IRunningObjectTable_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IRunningObjectTable_Impl::IsRunning(this, windows_core::from_raw_borrowed(&pmkobjectname)).into()
        }
        unsafe extern "system" fn GetObject<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pmkobjectname: *mut core::ffi::c_void, ppunkobject: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IRunningObjectTable_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IRunningObjectTable_Impl::GetObject(this, windows_core::from_raw_borrowed(&pmkobjectname)) {
                Ok(ok__) => {
                    ppunkobject.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn NoteChangeTime<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwregister: u32, pfiletime: *const super::super::Foundation::FILETIME) -> windows_core::HRESULT
        where
            Identity: IRunningObjectTable_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IRunningObjectTable_Impl::NoteChangeTime(this, core::mem::transmute_copy(&dwregister), core::mem::transmute_copy(&pfiletime)).into()
        }
        unsafe extern "system" fn GetTimeOfLastChange<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pmkobjectname: *mut core::ffi::c_void, pfiletime: *mut super::super::Foundation::FILETIME) -> windows_core::HRESULT
        where
            Identity: IRunningObjectTable_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IRunningObjectTable_Impl::GetTimeOfLastChange(this, windows_core::from_raw_borrowed(&pmkobjectname)) {
                Ok(ok__) => {
                    pfiletime.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnumRunning<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppenummoniker: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IRunningObjectTable_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IRunningObjectTable_Impl::EnumRunning(this) {
                Ok(ok__) => {
                    ppenummoniker.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Register: Register::<Identity, OFFSET>,
            Revoke: Revoke::<Identity, OFFSET>,
            IsRunning: IsRunning::<Identity, OFFSET>,
            GetObject: GetObject::<Identity, OFFSET>,
            NoteChangeTime: NoteChangeTime::<Identity, OFFSET>,
            GetTimeOfLastChange: GetTimeOfLastChange::<Identity, OFFSET>,
            EnumRunning: EnumRunning::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IRunningObjectTable as windows_core::Interface>::IID
    }
}
pub trait ISequentialStream_Impl: Sized {
    fn Read(&self, pv: *mut core::ffi::c_void, cb: u32, pcbread: *mut u32) -> windows_core::HRESULT;
    fn Write(&self, pv: *const core::ffi::c_void, cb: u32, pcbwritten: *mut u32) -> windows_core::HRESULT;
}
impl windows_core::RuntimeName for ISequentialStream {}
impl ISequentialStream_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ISequentialStream_Vtbl
    where
        Identity: ISequentialStream_Impl,
    {
        unsafe extern "system" fn Read<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pv: *mut core::ffi::c_void, cb: u32, pcbread: *mut u32) -> windows_core::HRESULT
        where
            Identity: ISequentialStream_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISequentialStream_Impl::Read(this, core::mem::transmute_copy(&pv), core::mem::transmute_copy(&cb), core::mem::transmute_copy(&pcbread))
        }
        unsafe extern "system" fn Write<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pv: *const core::ffi::c_void, cb: u32, pcbwritten: *mut u32) -> windows_core::HRESULT
        where
            Identity: ISequentialStream_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISequentialStream_Impl::Write(this, core::mem::transmute_copy(&pv), core::mem::transmute_copy(&cb), core::mem::transmute_copy(&pcbwritten))
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), Read: Read::<Identity, OFFSET>, Write: Write::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISequentialStream as windows_core::Interface>::IID
    }
}
pub trait IServerSecurity_Impl: Sized {
    fn QueryBlanket(&self, pauthnsvc: *mut u32, pauthzsvc: *mut u32, pserverprincname: *mut *mut u16, pauthnlevel: *mut u32, pimplevel: *mut u32, pprivs: *mut *mut core::ffi::c_void, pcapabilities: *mut u32) -> windows_core::Result<()>;
    fn ImpersonateClient(&self) -> windows_core::Result<()>;
    fn RevertToSelf(&self) -> windows_core::Result<()>;
    fn IsImpersonating(&self) -> super::super::Foundation::BOOL;
}
impl windows_core::RuntimeName for IServerSecurity {}
impl IServerSecurity_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IServerSecurity_Vtbl
    where
        Identity: IServerSecurity_Impl,
    {
        unsafe extern "system" fn QueryBlanket<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pauthnsvc: *mut u32, pauthzsvc: *mut u32, pserverprincname: *mut *mut u16, pauthnlevel: *mut u32, pimplevel: *mut u32, pprivs: *mut *mut core::ffi::c_void, pcapabilities: *mut u32) -> windows_core::HRESULT
        where
            Identity: IServerSecurity_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IServerSecurity_Impl::QueryBlanket(this, core::mem::transmute_copy(&pauthnsvc), core::mem::transmute_copy(&pauthzsvc), core::mem::transmute_copy(&pserverprincname), core::mem::transmute_copy(&pauthnlevel), core::mem::transmute_copy(&pimplevel), core::mem::transmute_copy(&pprivs), core::mem::transmute_copy(&pcapabilities)).into()
        }
        unsafe extern "system" fn ImpersonateClient<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IServerSecurity_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IServerSecurity_Impl::ImpersonateClient(this).into()
        }
        unsafe extern "system" fn RevertToSelf<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IServerSecurity_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IServerSecurity_Impl::RevertToSelf(this).into()
        }
        unsafe extern "system" fn IsImpersonating<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> super::super::Foundation::BOOL
        where
            Identity: IServerSecurity_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IServerSecurity_Impl::IsImpersonating(this)
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            QueryBlanket: QueryBlanket::<Identity, OFFSET>,
            ImpersonateClient: ImpersonateClient::<Identity, OFFSET>,
            RevertToSelf: RevertToSelf::<Identity, OFFSET>,
            IsImpersonating: IsImpersonating::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IServerSecurity as windows_core::Interface>::IID
    }
}
pub trait IServiceProvider_Impl: Sized {
    fn QueryService(&self, guidservice: *const windows_core::GUID, riid: *const windows_core::GUID, ppvobject: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IServiceProvider {}
impl IServiceProvider_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IServiceProvider_Vtbl
    where
        Identity: IServiceProvider_Impl,
    {
        unsafe extern "system" fn QueryService<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, guidservice: *const windows_core::GUID, riid: *const windows_core::GUID, ppvobject: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IServiceProvider_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IServiceProvider_Impl::QueryService(this, core::mem::transmute_copy(&guidservice), core::mem::transmute_copy(&riid), core::mem::transmute_copy(&ppvobject)).into()
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), QueryService: QueryService::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IServiceProvider as windows_core::Interface>::IID
    }
}
pub trait IStdMarshalInfo_Impl: Sized {
    fn GetClassForHandler(&self, dwdestcontext: u32, pvdestcontext: *const core::ffi::c_void) -> windows_core::Result<windows_core::GUID>;
}
impl windows_core::RuntimeName for IStdMarshalInfo {}
impl IStdMarshalInfo_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IStdMarshalInfo_Vtbl
    where
        Identity: IStdMarshalInfo_Impl,
    {
        unsafe extern "system" fn GetClassForHandler<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwdestcontext: u32, pvdestcontext: *const core::ffi::c_void, pclsid: *mut windows_core::GUID) -> windows_core::HRESULT
        where
            Identity: IStdMarshalInfo_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IStdMarshalInfo_Impl::GetClassForHandler(this, core::mem::transmute_copy(&dwdestcontext), core::mem::transmute_copy(&pvdestcontext)) {
                Ok(ok__) => {
                    pclsid.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), GetClassForHandler: GetClassForHandler::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IStdMarshalInfo as windows_core::Interface>::IID
    }
}
pub trait IStream_Impl: Sized + ISequentialStream_Impl {
    fn Seek(&self, dlibmove: i64, dworigin: STREAM_SEEK, plibnewposition: *mut u64) -> windows_core::Result<()>;
    fn SetSize(&self, libnewsize: u64) -> windows_core::Result<()>;
    fn CopyTo(&self, pstm: Option<&IStream>, cb: u64, pcbread: *mut u64, pcbwritten: *mut u64) -> windows_core::Result<()>;
    fn Commit(&self, grfcommitflags: &STGC) -> windows_core::Result<()>;
    fn Revert(&self) -> windows_core::Result<()>;
    fn LockRegion(&self, liboffset: u64, cb: u64, dwlocktype: &LOCKTYPE) -> windows_core::Result<()>;
    fn UnlockRegion(&self, liboffset: u64, cb: u64, dwlocktype: u32) -> windows_core::Result<()>;
    fn Stat(&self, pstatstg: *mut STATSTG, grfstatflag: &STATFLAG) -> windows_core::Result<()>;
    fn Clone(&self) -> windows_core::Result<IStream>;
}
impl windows_core::RuntimeName for IStream {}
impl IStream_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IStream_Vtbl
    where
        Identity: IStream_Impl,
    {
        unsafe extern "system" fn Seek<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, dlibmove: i64, dworigin: STREAM_SEEK, plibnewposition: *mut u64) -> windows_core::HRESULT
        where
            Identity: IStream_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IStream_Impl::Seek(this, core::mem::transmute_copy(&dlibmove), core::mem::transmute_copy(&dworigin), core::mem::transmute_copy(&plibnewposition)).into()
        }
        unsafe extern "system" fn SetSize<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, libnewsize: u64) -> windows_core::HRESULT
        where
            Identity: IStream_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IStream_Impl::SetSize(this, core::mem::transmute_copy(&libnewsize)).into()
        }
        unsafe extern "system" fn CopyTo<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pstm: *mut core::ffi::c_void, cb: u64, pcbread: *mut u64, pcbwritten: *mut u64) -> windows_core::HRESULT
        where
            Identity: IStream_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IStream_Impl::CopyTo(this, windows_core::from_raw_borrowed(&pstm), core::mem::transmute_copy(&cb), core::mem::transmute_copy(&pcbread), core::mem::transmute_copy(&pcbwritten)).into()
        }
        unsafe extern "system" fn Commit<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, grfcommitflags: u32) -> windows_core::HRESULT
        where
            Identity: IStream_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IStream_Impl::Commit(this, core::mem::transmute(&grfcommitflags)).into()
        }
        unsafe extern "system" fn Revert<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IStream_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IStream_Impl::Revert(this).into()
        }
        unsafe extern "system" fn LockRegion<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, liboffset: u64, cb: u64, dwlocktype: u32) -> windows_core::HRESULT
        where
            Identity: IStream_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IStream_Impl::LockRegion(this, core::mem::transmute_copy(&liboffset), core::mem::transmute_copy(&cb), core::mem::transmute(&dwlocktype)).into()
        }
        unsafe extern "system" fn UnlockRegion<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, liboffset: u64, cb: u64, dwlocktype: u32) -> windows_core::HRESULT
        where
            Identity: IStream_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IStream_Impl::UnlockRegion(this, core::mem::transmute_copy(&liboffset), core::mem::transmute_copy(&cb), core::mem::transmute_copy(&dwlocktype)).into()
        }
        unsafe extern "system" fn Stat<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pstatstg: *mut STATSTG, grfstatflag: u32) -> windows_core::HRESULT
        where
            Identity: IStream_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IStream_Impl::Stat(this, core::mem::transmute_copy(&pstatstg), core::mem::transmute(&grfstatflag)).into()
        }
        unsafe extern "system" fn Clone<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppstm: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IStream_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IStream_Impl::Clone(this) {
                Ok(ok__) => {
                    ppstm.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: ISequentialStream_Vtbl::new::<Identity, OFFSET>(),
            Seek: Seek::<Identity, OFFSET>,
            SetSize: SetSize::<Identity, OFFSET>,
            CopyTo: CopyTo::<Identity, OFFSET>,
            Commit: Commit::<Identity, OFFSET>,
            Revert: Revert::<Identity, OFFSET>,
            LockRegion: LockRegion::<Identity, OFFSET>,
            UnlockRegion: UnlockRegion::<Identity, OFFSET>,
            Stat: Stat::<Identity, OFFSET>,
            Clone: Clone::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IStream as windows_core::Interface>::IID || iid == &<ISequentialStream as windows_core::Interface>::IID
    }
}
pub trait ISupportAllowLowerTrustActivation_Impl: Sized {}
impl windows_core::RuntimeName for ISupportAllowLowerTrustActivation {}
impl ISupportAllowLowerTrustActivation_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ISupportAllowLowerTrustActivation_Vtbl
    where
        Identity: ISupportAllowLowerTrustActivation_Impl,
    {
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>() }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISupportAllowLowerTrustActivation as windows_core::Interface>::IID
    }
}
pub trait ISupportErrorInfo_Impl: Sized {
    fn InterfaceSupportsErrorInfo(&self, riid: *const windows_core::GUID) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for ISupportErrorInfo {}
impl ISupportErrorInfo_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ISupportErrorInfo_Vtbl
    where
        Identity: ISupportErrorInfo_Impl,
    {
        unsafe extern "system" fn InterfaceSupportsErrorInfo<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, riid: *const windows_core::GUID) -> windows_core::HRESULT
        where
            Identity: ISupportErrorInfo_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISupportErrorInfo_Impl::InterfaceSupportsErrorInfo(this, core::mem::transmute_copy(&riid)).into()
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), InterfaceSupportsErrorInfo: InterfaceSupportsErrorInfo::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISupportErrorInfo as windows_core::Interface>::IID
    }
}
pub trait ISurrogate_Impl: Sized {
    fn LoadDllServer(&self, clsid: *const windows_core::GUID) -> windows_core::Result<()>;
    fn FreeSurrogate(&self) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for ISurrogate {}
impl ISurrogate_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ISurrogate_Vtbl
    where
        Identity: ISurrogate_Impl,
    {
        unsafe extern "system" fn LoadDllServer<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, clsid: *const windows_core::GUID) -> windows_core::HRESULT
        where
            Identity: ISurrogate_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISurrogate_Impl::LoadDllServer(this, core::mem::transmute_copy(&clsid)).into()
        }
        unsafe extern "system" fn FreeSurrogate<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ISurrogate_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISurrogate_Impl::FreeSurrogate(this).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            LoadDllServer: LoadDllServer::<Identity, OFFSET>,
            FreeSurrogate: FreeSurrogate::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISurrogate as windows_core::Interface>::IID
    }
}
pub trait ISurrogateService_Impl: Sized {
    fn Init(&self, rguidprocessid: *const windows_core::GUID, pprocesslock: Option<&IProcessLock>) -> windows_core::Result<super::super::Foundation::BOOL>;
    fn ApplicationLaunch(&self, rguidapplid: *const windows_core::GUID, apptype: ApplicationType) -> windows_core::Result<()>;
    fn ApplicationFree(&self, rguidapplid: *const windows_core::GUID) -> windows_core::Result<()>;
    fn CatalogRefresh(&self, ulreserved: u32) -> windows_core::Result<()>;
    fn ProcessShutdown(&self, shutdowntype: ShutdownType) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for ISurrogateService {}
impl ISurrogateService_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ISurrogateService_Vtbl
    where
        Identity: ISurrogateService_Impl,
    {
        unsafe extern "system" fn Init<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, rguidprocessid: *const windows_core::GUID, pprocesslock: *mut core::ffi::c_void, pfapplicationaware: *mut super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: ISurrogateService_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISurrogateService_Impl::Init(this, core::mem::transmute_copy(&rguidprocessid), windows_core::from_raw_borrowed(&pprocesslock)) {
                Ok(ok__) => {
                    pfapplicationaware.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ApplicationLaunch<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, rguidapplid: *const windows_core::GUID, apptype: ApplicationType) -> windows_core::HRESULT
        where
            Identity: ISurrogateService_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISurrogateService_Impl::ApplicationLaunch(this, core::mem::transmute_copy(&rguidapplid), core::mem::transmute_copy(&apptype)).into()
        }
        unsafe extern "system" fn ApplicationFree<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, rguidapplid: *const windows_core::GUID) -> windows_core::HRESULT
        where
            Identity: ISurrogateService_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISurrogateService_Impl::ApplicationFree(this, core::mem::transmute_copy(&rguidapplid)).into()
        }
        unsafe extern "system" fn CatalogRefresh<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ulreserved: u32) -> windows_core::HRESULT
        where
            Identity: ISurrogateService_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISurrogateService_Impl::CatalogRefresh(this, core::mem::transmute_copy(&ulreserved)).into()
        }
        unsafe extern "system" fn ProcessShutdown<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, shutdowntype: ShutdownType) -> windows_core::HRESULT
        where
            Identity: ISurrogateService_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISurrogateService_Impl::ProcessShutdown(this, core::mem::transmute_copy(&shutdowntype)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Init: Init::<Identity, OFFSET>,
            ApplicationLaunch: ApplicationLaunch::<Identity, OFFSET>,
            ApplicationFree: ApplicationFree::<Identity, OFFSET>,
            CatalogRefresh: CatalogRefresh::<Identity, OFFSET>,
            ProcessShutdown: ProcessShutdown::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISurrogateService as windows_core::Interface>::IID
    }
}
pub trait ISynchronize_Impl: Sized {
    fn Wait(&self, dwflags: u32, dwmilliseconds: u32) -> windows_core::Result<()>;
    fn Signal(&self) -> windows_core::Result<()>;
    fn Reset(&self) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for ISynchronize {}
impl ISynchronize_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ISynchronize_Vtbl
    where
        Identity: ISynchronize_Impl,
    {
        unsafe extern "system" fn Wait<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwflags: u32, dwmilliseconds: u32) -> windows_core::HRESULT
        where
            Identity: ISynchronize_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISynchronize_Impl::Wait(this, core::mem::transmute_copy(&dwflags), core::mem::transmute_copy(&dwmilliseconds)).into()
        }
        unsafe extern "system" fn Signal<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ISynchronize_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISynchronize_Impl::Signal(this).into()
        }
        unsafe extern "system" fn Reset<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ISynchronize_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISynchronize_Impl::Reset(this).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Wait: Wait::<Identity, OFFSET>,
            Signal: Signal::<Identity, OFFSET>,
            Reset: Reset::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISynchronize as windows_core::Interface>::IID
    }
}
pub trait ISynchronizeContainer_Impl: Sized {
    fn AddSynchronize(&self, psync: Option<&ISynchronize>) -> windows_core::Result<()>;
    fn WaitMultiple(&self, dwflags: u32, dwtimeout: u32) -> windows_core::Result<ISynchronize>;
}
impl windows_core::RuntimeName for ISynchronizeContainer {}
impl ISynchronizeContainer_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ISynchronizeContainer_Vtbl
    where
        Identity: ISynchronizeContainer_Impl,
    {
        unsafe extern "system" fn AddSynchronize<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, psync: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ISynchronizeContainer_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISynchronizeContainer_Impl::AddSynchronize(this, windows_core::from_raw_borrowed(&psync)).into()
        }
        unsafe extern "system" fn WaitMultiple<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwflags: u32, dwtimeout: u32, ppsync: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ISynchronizeContainer_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISynchronizeContainer_Impl::WaitMultiple(this, core::mem::transmute_copy(&dwflags), core::mem::transmute_copy(&dwtimeout)) {
                Ok(ok__) => {
                    ppsync.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            AddSynchronize: AddSynchronize::<Identity, OFFSET>,
            WaitMultiple: WaitMultiple::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISynchronizeContainer as windows_core::Interface>::IID
    }
}
pub trait ISynchronizeEvent_Impl: Sized + ISynchronizeHandle_Impl {
    fn SetEventHandle(&self, ph: *const super::super::Foundation::HANDLE) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for ISynchronizeEvent {}
impl ISynchronizeEvent_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ISynchronizeEvent_Vtbl
    where
        Identity: ISynchronizeEvent_Impl,
    {
        unsafe extern "system" fn SetEventHandle<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ph: *const super::super::Foundation::HANDLE) -> windows_core::HRESULT
        where
            Identity: ISynchronizeEvent_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISynchronizeEvent_Impl::SetEventHandle(this, core::mem::transmute_copy(&ph)).into()
        }
        Self { base__: ISynchronizeHandle_Vtbl::new::<Identity, OFFSET>(), SetEventHandle: SetEventHandle::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISynchronizeEvent as windows_core::Interface>::IID || iid == &<ISynchronizeHandle as windows_core::Interface>::IID
    }
}
pub trait ISynchronizeHandle_Impl: Sized {
    fn GetHandle(&self) -> windows_core::Result<super::super::Foundation::HANDLE>;
}
impl windows_core::RuntimeName for ISynchronizeHandle {}
impl ISynchronizeHandle_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ISynchronizeHandle_Vtbl
    where
        Identity: ISynchronizeHandle_Impl,
    {
        unsafe extern "system" fn GetHandle<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ph: *mut super::super::Foundation::HANDLE) -> windows_core::HRESULT
        where
            Identity: ISynchronizeHandle_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISynchronizeHandle_Impl::GetHandle(this) {
                Ok(ok__) => {
                    ph.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), GetHandle: GetHandle::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISynchronizeHandle as windows_core::Interface>::IID
    }
}
pub trait ISynchronizeMutex_Impl: Sized + ISynchronize_Impl {
    fn ReleaseMutex(&self) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for ISynchronizeMutex {}
impl ISynchronizeMutex_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ISynchronizeMutex_Vtbl
    where
        Identity: ISynchronizeMutex_Impl,
    {
        unsafe extern "system" fn ReleaseMutex<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ISynchronizeMutex_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISynchronizeMutex_Impl::ReleaseMutex(this).into()
        }
        Self { base__: ISynchronize_Vtbl::new::<Identity, OFFSET>(), ReleaseMutex: ReleaseMutex::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISynchronizeMutex as windows_core::Interface>::IID || iid == &<ISynchronize as windows_core::Interface>::IID
    }
}
pub trait ITimeAndNoticeControl_Impl: Sized {
    fn SuppressChanges(&self, res1: u32, res2: u32) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for ITimeAndNoticeControl {}
impl ITimeAndNoticeControl_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ITimeAndNoticeControl_Vtbl
    where
        Identity: ITimeAndNoticeControl_Impl,
    {
        unsafe extern "system" fn SuppressChanges<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, res1: u32, res2: u32) -> windows_core::HRESULT
        where
            Identity: ITimeAndNoticeControl_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITimeAndNoticeControl_Impl::SuppressChanges(this, core::mem::transmute_copy(&res1), core::mem::transmute_copy(&res2)).into()
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), SuppressChanges: SuppressChanges::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITimeAndNoticeControl as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ITypeComp_Impl: Sized {
    fn Bind(&self, szname: &windows_core::PCWSTR, lhashval: u32, wflags: u16, pptinfo: *mut Option<ITypeInfo>, pdesckind: *mut DESCKIND, pbindptr: *mut BINDPTR) -> windows_core::Result<()>;
    fn BindType(&self, szname: &windows_core::PCWSTR, lhashval: u32, pptinfo: *mut Option<ITypeInfo>, pptcomp: *mut Option<ITypeComp>) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for ITypeComp {}
#[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ITypeComp_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ITypeComp_Vtbl
    where
        Identity: ITypeComp_Impl,
    {
        unsafe extern "system" fn Bind<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, szname: windows_core::PCWSTR, lhashval: u32, wflags: u16, pptinfo: *mut *mut core::ffi::c_void, pdesckind: *mut DESCKIND, pbindptr: *mut BINDPTR) -> windows_core::HRESULT
        where
            Identity: ITypeComp_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITypeComp_Impl::Bind(this, core::mem::transmute(&szname), core::mem::transmute_copy(&lhashval), core::mem::transmute_copy(&wflags), core::mem::transmute_copy(&pptinfo), core::mem::transmute_copy(&pdesckind), core::mem::transmute_copy(&pbindptr)).into()
        }
        unsafe extern "system" fn BindType<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, szname: windows_core::PCWSTR, lhashval: u32, pptinfo: *mut *mut core::ffi::c_void, pptcomp: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ITypeComp_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITypeComp_Impl::BindType(this, core::mem::transmute(&szname), core::mem::transmute_copy(&lhashval), core::mem::transmute_copy(&pptinfo), core::mem::transmute_copy(&pptcomp)).into()
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), Bind: Bind::<Identity, OFFSET>, BindType: BindType::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITypeComp as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ITypeInfo_Impl: Sized {
    fn GetTypeAttr(&self) -> windows_core::Result<*mut TYPEATTR>;
    fn GetTypeComp(&self) -> windows_core::Result<ITypeComp>;
    fn GetFuncDesc(&self, index: u32) -> windows_core::Result<*mut FUNCDESC>;
    fn GetVarDesc(&self, index: u32) -> windows_core::Result<*mut VARDESC>;
    fn GetNames(&self, memid: i32, rgbstrnames: *mut windows_core::BSTR, cmaxnames: u32, pcnames: *mut u32) -> windows_core::Result<()>;
    fn GetRefTypeOfImplType(&self, index: u32) -> windows_core::Result<u32>;
    fn GetImplTypeFlags(&self, index: u32) -> windows_core::Result<IMPLTYPEFLAGS>;
    fn GetIDsOfNames(&self, rgsznames: *const windows_core::PCWSTR, cnames: u32, pmemid: *mut i32) -> windows_core::Result<()>;
    fn Invoke(&self, pvinstance: *const core::ffi::c_void, memid: i32, wflags: DISPATCH_FLAGS, pdispparams: *mut DISPPARAMS, pvarresult: *mut windows_core::VARIANT, pexcepinfo: *mut EXCEPINFO, puargerr: *mut u32) -> windows_core::Result<()>;
    fn GetDocumentation(&self, memid: i32, pbstrname: *mut windows_core::BSTR, pbstrdocstring: *mut windows_core::BSTR, pdwhelpcontext: *mut u32, pbstrhelpfile: *mut windows_core::BSTR) -> windows_core::Result<()>;
    fn GetDllEntry(&self, memid: i32, invkind: INVOKEKIND, pbstrdllname: *mut windows_core::BSTR, pbstrname: *mut windows_core::BSTR, pwordinal: *mut u16) -> windows_core::Result<()>;
    fn GetRefTypeInfo(&self, hreftype: u32) -> windows_core::Result<ITypeInfo>;
    fn AddressOfMember(&self, memid: i32, invkind: INVOKEKIND, ppv: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
    fn CreateInstance(&self, punkouter: Option<&windows_core::IUnknown>, riid: *const windows_core::GUID, ppvobj: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
    fn GetMops(&self, memid: i32) -> windows_core::Result<windows_core::BSTR>;
    fn GetContainingTypeLib(&self, pptlib: *mut Option<ITypeLib>, pindex: *mut u32) -> windows_core::Result<()>;
    fn ReleaseTypeAttr(&self, ptypeattr: *const TYPEATTR);
    fn ReleaseFuncDesc(&self, pfuncdesc: *const FUNCDESC);
    fn ReleaseVarDesc(&self, pvardesc: *const VARDESC);
}
#[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for ITypeInfo {}
#[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ITypeInfo_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ITypeInfo_Vtbl
    where
        Identity: ITypeInfo_Impl,
    {
        unsafe extern "system" fn GetTypeAttr<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pptypeattr: *mut *mut TYPEATTR) -> windows_core::HRESULT
        where
            Identity: ITypeInfo_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITypeInfo_Impl::GetTypeAttr(this) {
                Ok(ok__) => {
                    pptypeattr.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetTypeComp<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pptcomp: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ITypeInfo_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITypeInfo_Impl::GetTypeComp(this) {
                Ok(ok__) => {
                    pptcomp.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFuncDesc<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: u32, ppfuncdesc: *mut *mut FUNCDESC) -> windows_core::HRESULT
        where
            Identity: ITypeInfo_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITypeInfo_Impl::GetFuncDesc(this, core::mem::transmute_copy(&index)) {
                Ok(ok__) => {
                    ppfuncdesc.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetVarDesc<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: u32, ppvardesc: *mut *mut VARDESC) -> windows_core::HRESULT
        where
            Identity: ITypeInfo_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITypeInfo_Impl::GetVarDesc(this, core::mem::transmute_copy(&index)) {
                Ok(ok__) => {
                    ppvardesc.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetNames<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, memid: i32, rgbstrnames: *mut core::mem::MaybeUninit<windows_core::BSTR>, cmaxnames: u32, pcnames: *mut u32) -> windows_core::HRESULT
        where
            Identity: ITypeInfo_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITypeInfo_Impl::GetNames(this, core::mem::transmute_copy(&memid), core::mem::transmute_copy(&rgbstrnames), core::mem::transmute_copy(&cmaxnames), core::mem::transmute_copy(&pcnames)).into()
        }
        unsafe extern "system" fn GetRefTypeOfImplType<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: u32, preftype: *mut u32) -> windows_core::HRESULT
        where
            Identity: ITypeInfo_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITypeInfo_Impl::GetRefTypeOfImplType(this, core::mem::transmute_copy(&index)) {
                Ok(ok__) => {
                    preftype.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetImplTypeFlags<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: u32, pimpltypeflags: *mut IMPLTYPEFLAGS) -> windows_core::HRESULT
        where
            Identity: ITypeInfo_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITypeInfo_Impl::GetImplTypeFlags(this, core::mem::transmute_copy(&index)) {
                Ok(ok__) => {
                    pimpltypeflags.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetIDsOfNames<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, rgsznames: *const windows_core::PCWSTR, cnames: u32, pmemid: *mut i32) -> windows_core::HRESULT
        where
            Identity: ITypeInfo_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITypeInfo_Impl::GetIDsOfNames(this, core::mem::transmute_copy(&rgsznames), core::mem::transmute_copy(&cnames), core::mem::transmute_copy(&pmemid)).into()
        }
        unsafe extern "system" fn Invoke<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvinstance: *const core::ffi::c_void, memid: i32, wflags: DISPATCH_FLAGS, pdispparams: *mut DISPPARAMS, pvarresult: *mut core::mem::MaybeUninit<windows_core::VARIANT>, pexcepinfo: *mut EXCEPINFO, puargerr: *mut u32) -> windows_core::HRESULT
        where
            Identity: ITypeInfo_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITypeInfo_Impl::Invoke(this, core::mem::transmute_copy(&pvinstance), core::mem::transmute_copy(&memid), core::mem::transmute_copy(&wflags), core::mem::transmute_copy(&pdispparams), core::mem::transmute_copy(&pvarresult), core::mem::transmute_copy(&pexcepinfo), core::mem::transmute_copy(&puargerr)).into()
        }
        unsafe extern "system" fn GetDocumentation<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, memid: i32, pbstrname: *mut core::mem::MaybeUninit<windows_core::BSTR>, pbstrdocstring: *mut core::mem::MaybeUninit<windows_core::BSTR>, pdwhelpcontext: *mut u32, pbstrhelpfile: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: ITypeInfo_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITypeInfo_Impl::GetDocumentation(this, core::mem::transmute_copy(&memid), core::mem::transmute_copy(&pbstrname), core::mem::transmute_copy(&pbstrdocstring), core::mem::transmute_copy(&pdwhelpcontext), core::mem::transmute_copy(&pbstrhelpfile)).into()
        }
        unsafe extern "system" fn GetDllEntry<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, memid: i32, invkind: INVOKEKIND, pbstrdllname: *mut core::mem::MaybeUninit<windows_core::BSTR>, pbstrname: *mut core::mem::MaybeUninit<windows_core::BSTR>, pwordinal: *mut u16) -> windows_core::HRESULT
        where
            Identity: ITypeInfo_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITypeInfo_Impl::GetDllEntry(this, core::mem::transmute_copy(&memid), core::mem::transmute_copy(&invkind), core::mem::transmute_copy(&pbstrdllname), core::mem::transmute_copy(&pbstrname), core::mem::transmute_copy(&pwordinal)).into()
        }
        unsafe extern "system" fn GetRefTypeInfo<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, hreftype: u32, pptinfo: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ITypeInfo_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITypeInfo_Impl::GetRefTypeInfo(this, core::mem::transmute_copy(&hreftype)) {
                Ok(ok__) => {
                    pptinfo.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AddressOfMember<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, memid: i32, invkind: INVOKEKIND, ppv: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ITypeInfo_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITypeInfo_Impl::AddressOfMember(this, core::mem::transmute_copy(&memid), core::mem::transmute_copy(&invkind), core::mem::transmute_copy(&ppv)).into()
        }
        unsafe extern "system" fn CreateInstance<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, punkouter: *mut core::ffi::c_void, riid: *const windows_core::GUID, ppvobj: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ITypeInfo_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITypeInfo_Impl::CreateInstance(this, windows_core::from_raw_borrowed(&punkouter), core::mem::transmute_copy(&riid), core::mem::transmute_copy(&ppvobj)).into()
        }
        unsafe extern "system" fn GetMops<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, memid: i32, pbstrmops: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: ITypeInfo_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITypeInfo_Impl::GetMops(this, core::mem::transmute_copy(&memid)) {
                Ok(ok__) => {
                    pbstrmops.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetContainingTypeLib<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pptlib: *mut *mut core::ffi::c_void, pindex: *mut u32) -> windows_core::HRESULT
        where
            Identity: ITypeInfo_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITypeInfo_Impl::GetContainingTypeLib(this, core::mem::transmute_copy(&pptlib), core::mem::transmute_copy(&pindex)).into()
        }
        unsafe extern "system" fn ReleaseTypeAttr<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ptypeattr: *const TYPEATTR)
        where
            Identity: ITypeInfo_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITypeInfo_Impl::ReleaseTypeAttr(this, core::mem::transmute_copy(&ptypeattr))
        }
        unsafe extern "system" fn ReleaseFuncDesc<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pfuncdesc: *const FUNCDESC)
        where
            Identity: ITypeInfo_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITypeInfo_Impl::ReleaseFuncDesc(this, core::mem::transmute_copy(&pfuncdesc))
        }
        unsafe extern "system" fn ReleaseVarDesc<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvardesc: *const VARDESC)
        where
            Identity: ITypeInfo_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITypeInfo_Impl::ReleaseVarDesc(this, core::mem::transmute_copy(&pvardesc))
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetTypeAttr: GetTypeAttr::<Identity, OFFSET>,
            GetTypeComp: GetTypeComp::<Identity, OFFSET>,
            GetFuncDesc: GetFuncDesc::<Identity, OFFSET>,
            GetVarDesc: GetVarDesc::<Identity, OFFSET>,
            GetNames: GetNames::<Identity, OFFSET>,
            GetRefTypeOfImplType: GetRefTypeOfImplType::<Identity, OFFSET>,
            GetImplTypeFlags: GetImplTypeFlags::<Identity, OFFSET>,
            GetIDsOfNames: GetIDsOfNames::<Identity, OFFSET>,
            Invoke: Invoke::<Identity, OFFSET>,
            GetDocumentation: GetDocumentation::<Identity, OFFSET>,
            GetDllEntry: GetDllEntry::<Identity, OFFSET>,
            GetRefTypeInfo: GetRefTypeInfo::<Identity, OFFSET>,
            AddressOfMember: AddressOfMember::<Identity, OFFSET>,
            CreateInstance: CreateInstance::<Identity, OFFSET>,
            GetMops: GetMops::<Identity, OFFSET>,
            GetContainingTypeLib: GetContainingTypeLib::<Identity, OFFSET>,
            ReleaseTypeAttr: ReleaseTypeAttr::<Identity, OFFSET>,
            ReleaseFuncDesc: ReleaseFuncDesc::<Identity, OFFSET>,
            ReleaseVarDesc: ReleaseVarDesc::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITypeInfo as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ITypeInfo2_Impl: Sized + ITypeInfo_Impl {
    fn GetTypeKind(&self) -> windows_core::Result<TYPEKIND>;
    fn GetTypeFlags(&self) -> windows_core::Result<u32>;
    fn GetFuncIndexOfMemId(&self, memid: i32, invkind: INVOKEKIND) -> windows_core::Result<u32>;
    fn GetVarIndexOfMemId(&self, memid: i32) -> windows_core::Result<u32>;
    fn GetCustData(&self, guid: *const windows_core::GUID) -> windows_core::Result<windows_core::VARIANT>;
    fn GetFuncCustData(&self, index: u32, guid: *const windows_core::GUID) -> windows_core::Result<windows_core::VARIANT>;
    fn GetParamCustData(&self, indexfunc: u32, indexparam: u32, guid: *const windows_core::GUID) -> windows_core::Result<windows_core::VARIANT>;
    fn GetVarCustData(&self, index: u32, guid: *const windows_core::GUID) -> windows_core::Result<windows_core::VARIANT>;
    fn GetImplTypeCustData(&self, index: u32, guid: *const windows_core::GUID) -> windows_core::Result<windows_core::VARIANT>;
    fn GetDocumentation2(&self, memid: i32, lcid: u32, pbstrhelpstring: *mut windows_core::BSTR, pdwhelpstringcontext: *mut u32, pbstrhelpstringdll: *mut windows_core::BSTR) -> windows_core::Result<()>;
    fn GetAllCustData(&self) -> windows_core::Result<CUSTDATA>;
    fn GetAllFuncCustData(&self, index: u32) -> windows_core::Result<CUSTDATA>;
    fn GetAllParamCustData(&self, indexfunc: u32, indexparam: u32) -> windows_core::Result<CUSTDATA>;
    fn GetAllVarCustData(&self, index: u32) -> windows_core::Result<CUSTDATA>;
    fn GetAllImplTypeCustData(&self, index: u32) -> windows_core::Result<CUSTDATA>;
}
#[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for ITypeInfo2 {}
#[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ITypeInfo2_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ITypeInfo2_Vtbl
    where
        Identity: ITypeInfo2_Impl,
    {
        unsafe extern "system" fn GetTypeKind<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ptypekind: *mut TYPEKIND) -> windows_core::HRESULT
        where
            Identity: ITypeInfo2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITypeInfo2_Impl::GetTypeKind(this) {
                Ok(ok__) => {
                    ptypekind.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetTypeFlags<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ptypeflags: *mut u32) -> windows_core::HRESULT
        where
            Identity: ITypeInfo2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITypeInfo2_Impl::GetTypeFlags(this) {
                Ok(ok__) => {
                    ptypeflags.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFuncIndexOfMemId<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, memid: i32, invkind: INVOKEKIND, pfuncindex: *mut u32) -> windows_core::HRESULT
        where
            Identity: ITypeInfo2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITypeInfo2_Impl::GetFuncIndexOfMemId(this, core::mem::transmute_copy(&memid), core::mem::transmute_copy(&invkind)) {
                Ok(ok__) => {
                    pfuncindex.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetVarIndexOfMemId<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, memid: i32, pvarindex: *mut u32) -> windows_core::HRESULT
        where
            Identity: ITypeInfo2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITypeInfo2_Impl::GetVarIndexOfMemId(this, core::mem::transmute_copy(&memid)) {
                Ok(ok__) => {
                    pvarindex.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCustData<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, guid: *const windows_core::GUID, pvarval: *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT
        where
            Identity: ITypeInfo2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITypeInfo2_Impl::GetCustData(this, core::mem::transmute_copy(&guid)) {
                Ok(ok__) => {
                    pvarval.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFuncCustData<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: u32, guid: *const windows_core::GUID, pvarval: *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT
        where
            Identity: ITypeInfo2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITypeInfo2_Impl::GetFuncCustData(this, core::mem::transmute_copy(&index), core::mem::transmute_copy(&guid)) {
                Ok(ok__) => {
                    pvarval.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetParamCustData<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, indexfunc: u32, indexparam: u32, guid: *const windows_core::GUID, pvarval: *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT
        where
            Identity: ITypeInfo2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITypeInfo2_Impl::GetParamCustData(this, core::mem::transmute_copy(&indexfunc), core::mem::transmute_copy(&indexparam), core::mem::transmute_copy(&guid)) {
                Ok(ok__) => {
                    pvarval.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetVarCustData<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: u32, guid: *const windows_core::GUID, pvarval: *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT
        where
            Identity: ITypeInfo2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITypeInfo2_Impl::GetVarCustData(this, core::mem::transmute_copy(&index), core::mem::transmute_copy(&guid)) {
                Ok(ok__) => {
                    pvarval.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetImplTypeCustData<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: u32, guid: *const windows_core::GUID, pvarval: *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT
        where
            Identity: ITypeInfo2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITypeInfo2_Impl::GetImplTypeCustData(this, core::mem::transmute_copy(&index), core::mem::transmute_copy(&guid)) {
                Ok(ok__) => {
                    pvarval.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDocumentation2<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, memid: i32, lcid: u32, pbstrhelpstring: *mut core::mem::MaybeUninit<windows_core::BSTR>, pdwhelpstringcontext: *mut u32, pbstrhelpstringdll: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: ITypeInfo2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITypeInfo2_Impl::GetDocumentation2(this, core::mem::transmute_copy(&memid), core::mem::transmute_copy(&lcid), core::mem::transmute_copy(&pbstrhelpstring), core::mem::transmute_copy(&pdwhelpstringcontext), core::mem::transmute_copy(&pbstrhelpstringdll)).into()
        }
        unsafe extern "system" fn GetAllCustData<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcustdata: *mut CUSTDATA) -> windows_core::HRESULT
        where
            Identity: ITypeInfo2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITypeInfo2_Impl::GetAllCustData(this) {
                Ok(ok__) => {
                    pcustdata.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetAllFuncCustData<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: u32, pcustdata: *mut CUSTDATA) -> windows_core::HRESULT
        where
            Identity: ITypeInfo2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITypeInfo2_Impl::GetAllFuncCustData(this, core::mem::transmute_copy(&index)) {
                Ok(ok__) => {
                    pcustdata.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetAllParamCustData<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, indexfunc: u32, indexparam: u32, pcustdata: *mut CUSTDATA) -> windows_core::HRESULT
        where
            Identity: ITypeInfo2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITypeInfo2_Impl::GetAllParamCustData(this, core::mem::transmute_copy(&indexfunc), core::mem::transmute_copy(&indexparam)) {
                Ok(ok__) => {
                    pcustdata.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetAllVarCustData<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: u32, pcustdata: *mut CUSTDATA) -> windows_core::HRESULT
        where
            Identity: ITypeInfo2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITypeInfo2_Impl::GetAllVarCustData(this, core::mem::transmute_copy(&index)) {
                Ok(ok__) => {
                    pcustdata.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetAllImplTypeCustData<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: u32, pcustdata: *mut CUSTDATA) -> windows_core::HRESULT
        where
            Identity: ITypeInfo2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITypeInfo2_Impl::GetAllImplTypeCustData(this, core::mem::transmute_copy(&index)) {
                Ok(ok__) => {
                    pcustdata.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: ITypeInfo_Vtbl::new::<Identity, OFFSET>(),
            GetTypeKind: GetTypeKind::<Identity, OFFSET>,
            GetTypeFlags: GetTypeFlags::<Identity, OFFSET>,
            GetFuncIndexOfMemId: GetFuncIndexOfMemId::<Identity, OFFSET>,
            GetVarIndexOfMemId: GetVarIndexOfMemId::<Identity, OFFSET>,
            GetCustData: GetCustData::<Identity, OFFSET>,
            GetFuncCustData: GetFuncCustData::<Identity, OFFSET>,
            GetParamCustData: GetParamCustData::<Identity, OFFSET>,
            GetVarCustData: GetVarCustData::<Identity, OFFSET>,
            GetImplTypeCustData: GetImplTypeCustData::<Identity, OFFSET>,
            GetDocumentation2: GetDocumentation2::<Identity, OFFSET>,
            GetAllCustData: GetAllCustData::<Identity, OFFSET>,
            GetAllFuncCustData: GetAllFuncCustData::<Identity, OFFSET>,
            GetAllParamCustData: GetAllParamCustData::<Identity, OFFSET>,
            GetAllVarCustData: GetAllVarCustData::<Identity, OFFSET>,
            GetAllImplTypeCustData: GetAllImplTypeCustData::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITypeInfo2 as windows_core::Interface>::IID || iid == &<ITypeInfo as windows_core::Interface>::IID
    }
}
pub trait ITypeLib_Impl: Sized {
    fn GetTypeInfoCount(&self) -> u32;
    fn GetTypeInfo(&self, index: u32) -> windows_core::Result<ITypeInfo>;
    fn GetTypeInfoType(&self, index: u32) -> windows_core::Result<TYPEKIND>;
    fn GetTypeInfoOfGuid(&self, guid: *const windows_core::GUID) -> windows_core::Result<ITypeInfo>;
    fn GetLibAttr(&self) -> windows_core::Result<*mut TLIBATTR>;
    fn GetTypeComp(&self) -> windows_core::Result<ITypeComp>;
    fn GetDocumentation(&self, index: i32, pbstrname: *mut windows_core::BSTR, pbstrdocstring: *mut windows_core::BSTR, pdwhelpcontext: *mut u32, pbstrhelpfile: *mut windows_core::BSTR) -> windows_core::Result<()>;
    fn IsName(&self, sznamebuf: &windows_core::PWSTR, lhashval: u32, pfname: *mut super::super::Foundation::BOOL) -> windows_core::Result<()>;
    fn FindName(&self, sznamebuf: &windows_core::PWSTR, lhashval: u32, pptinfo: *mut Option<ITypeInfo>, rgmemid: *mut i32, pcfound: *mut u16) -> windows_core::Result<()>;
    fn ReleaseTLibAttr(&self, ptlibattr: *const TLIBATTR);
}
impl windows_core::RuntimeName for ITypeLib {}
impl ITypeLib_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ITypeLib_Vtbl
    where
        Identity: ITypeLib_Impl,
    {
        unsafe extern "system" fn GetTypeInfoCount<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> u32
        where
            Identity: ITypeLib_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITypeLib_Impl::GetTypeInfoCount(this)
        }
        unsafe extern "system" fn GetTypeInfo<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: u32, pptinfo: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ITypeLib_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITypeLib_Impl::GetTypeInfo(this, core::mem::transmute_copy(&index)) {
                Ok(ok__) => {
                    pptinfo.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetTypeInfoType<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: u32, ptkind: *mut TYPEKIND) -> windows_core::HRESULT
        where
            Identity: ITypeLib_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITypeLib_Impl::GetTypeInfoType(this, core::mem::transmute_copy(&index)) {
                Ok(ok__) => {
                    ptkind.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetTypeInfoOfGuid<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, guid: *const windows_core::GUID, pptinfo: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ITypeLib_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITypeLib_Impl::GetTypeInfoOfGuid(this, core::mem::transmute_copy(&guid)) {
                Ok(ok__) => {
                    pptinfo.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetLibAttr<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pptlibattr: *mut *mut TLIBATTR) -> windows_core::HRESULT
        where
            Identity: ITypeLib_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITypeLib_Impl::GetLibAttr(this) {
                Ok(ok__) => {
                    pptlibattr.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetTypeComp<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pptcomp: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ITypeLib_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITypeLib_Impl::GetTypeComp(this) {
                Ok(ok__) => {
                    pptcomp.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDocumentation<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: i32, pbstrname: *mut core::mem::MaybeUninit<windows_core::BSTR>, pbstrdocstring: *mut core::mem::MaybeUninit<windows_core::BSTR>, pdwhelpcontext: *mut u32, pbstrhelpfile: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: ITypeLib_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITypeLib_Impl::GetDocumentation(this, core::mem::transmute_copy(&index), core::mem::transmute_copy(&pbstrname), core::mem::transmute_copy(&pbstrdocstring), core::mem::transmute_copy(&pdwhelpcontext), core::mem::transmute_copy(&pbstrhelpfile)).into()
        }
        unsafe extern "system" fn IsName<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, sznamebuf: windows_core::PWSTR, lhashval: u32, pfname: *mut super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: ITypeLib_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITypeLib_Impl::IsName(this, core::mem::transmute(&sznamebuf), core::mem::transmute_copy(&lhashval), core::mem::transmute_copy(&pfname)).into()
        }
        unsafe extern "system" fn FindName<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, sznamebuf: windows_core::PWSTR, lhashval: u32, pptinfo: *mut *mut core::ffi::c_void, rgmemid: *mut i32, pcfound: *mut u16) -> windows_core::HRESULT
        where
            Identity: ITypeLib_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITypeLib_Impl::FindName(this, core::mem::transmute(&sznamebuf), core::mem::transmute_copy(&lhashval), core::mem::transmute_copy(&pptinfo), core::mem::transmute_copy(&rgmemid), core::mem::transmute_copy(&pcfound)).into()
        }
        unsafe extern "system" fn ReleaseTLibAttr<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ptlibattr: *const TLIBATTR)
        where
            Identity: ITypeLib_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITypeLib_Impl::ReleaseTLibAttr(this, core::mem::transmute_copy(&ptlibattr))
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetTypeInfoCount: GetTypeInfoCount::<Identity, OFFSET>,
            GetTypeInfo: GetTypeInfo::<Identity, OFFSET>,
            GetTypeInfoType: GetTypeInfoType::<Identity, OFFSET>,
            GetTypeInfoOfGuid: GetTypeInfoOfGuid::<Identity, OFFSET>,
            GetLibAttr: GetLibAttr::<Identity, OFFSET>,
            GetTypeComp: GetTypeComp::<Identity, OFFSET>,
            GetDocumentation: GetDocumentation::<Identity, OFFSET>,
            IsName: IsName::<Identity, OFFSET>,
            FindName: FindName::<Identity, OFFSET>,
            ReleaseTLibAttr: ReleaseTLibAttr::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITypeLib as windows_core::Interface>::IID
    }
}
pub trait ITypeLib2_Impl: Sized + ITypeLib_Impl {
    fn GetCustData(&self, guid: *const windows_core::GUID) -> windows_core::Result<windows_core::VARIANT>;
    fn GetLibStatistics(&self, pcuniquenames: *mut u32, pcchuniquenames: *mut u32) -> windows_core::Result<()>;
    fn GetDocumentation2(&self, index: i32, lcid: u32, pbstrhelpstring: *mut windows_core::BSTR, pdwhelpstringcontext: *mut u32, pbstrhelpstringdll: *mut windows_core::BSTR) -> windows_core::Result<()>;
    fn GetAllCustData(&self) -> windows_core::Result<CUSTDATA>;
}
impl windows_core::RuntimeName for ITypeLib2 {}
impl ITypeLib2_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ITypeLib2_Vtbl
    where
        Identity: ITypeLib2_Impl,
    {
        unsafe extern "system" fn GetCustData<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, guid: *const windows_core::GUID, pvarval: *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT
        where
            Identity: ITypeLib2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITypeLib2_Impl::GetCustData(this, core::mem::transmute_copy(&guid)) {
                Ok(ok__) => {
                    pvarval.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetLibStatistics<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcuniquenames: *mut u32, pcchuniquenames: *mut u32) -> windows_core::HRESULT
        where
            Identity: ITypeLib2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITypeLib2_Impl::GetLibStatistics(this, core::mem::transmute_copy(&pcuniquenames), core::mem::transmute_copy(&pcchuniquenames)).into()
        }
        unsafe extern "system" fn GetDocumentation2<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: i32, lcid: u32, pbstrhelpstring: *mut core::mem::MaybeUninit<windows_core::BSTR>, pdwhelpstringcontext: *mut u32, pbstrhelpstringdll: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: ITypeLib2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITypeLib2_Impl::GetDocumentation2(this, core::mem::transmute_copy(&index), core::mem::transmute_copy(&lcid), core::mem::transmute_copy(&pbstrhelpstring), core::mem::transmute_copy(&pdwhelpstringcontext), core::mem::transmute_copy(&pbstrhelpstringdll)).into()
        }
        unsafe extern "system" fn GetAllCustData<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcustdata: *mut CUSTDATA) -> windows_core::HRESULT
        where
            Identity: ITypeLib2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITypeLib2_Impl::GetAllCustData(this) {
                Ok(ok__) => {
                    pcustdata.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: ITypeLib_Vtbl::new::<Identity, OFFSET>(),
            GetCustData: GetCustData::<Identity, OFFSET>,
            GetLibStatistics: GetLibStatistics::<Identity, OFFSET>,
            GetDocumentation2: GetDocumentation2::<Identity, OFFSET>,
            GetAllCustData: GetAllCustData::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITypeLib2 as windows_core::Interface>::IID || iid == &<ITypeLib as windows_core::Interface>::IID
    }
}
pub trait ITypeLibRegistration_Impl: Sized {
    fn GetGuid(&self) -> windows_core::Result<windows_core::GUID>;
    fn GetVersion(&self) -> windows_core::Result<windows_core::BSTR>;
    fn GetLcid(&self) -> windows_core::Result<u32>;
    fn GetWin32Path(&self) -> windows_core::Result<windows_core::BSTR>;
    fn GetWin64Path(&self) -> windows_core::Result<windows_core::BSTR>;
    fn GetDisplayName(&self) -> windows_core::Result<windows_core::BSTR>;
    fn GetFlags(&self) -> windows_core::Result<u32>;
    fn GetHelpDir(&self) -> windows_core::Result<windows_core::BSTR>;
}
impl windows_core::RuntimeName for ITypeLibRegistration {}
impl ITypeLibRegistration_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ITypeLibRegistration_Vtbl
    where
        Identity: ITypeLibRegistration_Impl,
    {
        unsafe extern "system" fn GetGuid<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pguid: *mut windows_core::GUID) -> windows_core::HRESULT
        where
            Identity: ITypeLibRegistration_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITypeLibRegistration_Impl::GetGuid(this) {
                Ok(ok__) => {
                    pguid.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetVersion<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pversion: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: ITypeLibRegistration_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITypeLibRegistration_Impl::GetVersion(this) {
                Ok(ok__) => {
                    pversion.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetLcid<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, plcid: *mut u32) -> windows_core::HRESULT
        where
            Identity: ITypeLibRegistration_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITypeLibRegistration_Impl::GetLcid(this) {
                Ok(ok__) => {
                    plcid.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetWin32Path<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwin32path: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: ITypeLibRegistration_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITypeLibRegistration_Impl::GetWin32Path(this) {
                Ok(ok__) => {
                    pwin32path.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetWin64Path<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwin64path: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: ITypeLibRegistration_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITypeLibRegistration_Impl::GetWin64Path(this) {
                Ok(ok__) => {
                    pwin64path.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDisplayName<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdisplayname: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: ITypeLibRegistration_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITypeLibRegistration_Impl::GetDisplayName(this) {
                Ok(ok__) => {
                    pdisplayname.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFlags<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pflags: *mut u32) -> windows_core::HRESULT
        where
            Identity: ITypeLibRegistration_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITypeLibRegistration_Impl::GetFlags(this) {
                Ok(ok__) => {
                    pflags.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetHelpDir<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, phelpdir: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: ITypeLibRegistration_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITypeLibRegistration_Impl::GetHelpDir(this) {
                Ok(ok__) => {
                    phelpdir.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetGuid: GetGuid::<Identity, OFFSET>,
            GetVersion: GetVersion::<Identity, OFFSET>,
            GetLcid: GetLcid::<Identity, OFFSET>,
            GetWin32Path: GetWin32Path::<Identity, OFFSET>,
            GetWin64Path: GetWin64Path::<Identity, OFFSET>,
            GetDisplayName: GetDisplayName::<Identity, OFFSET>,
            GetFlags: GetFlags::<Identity, OFFSET>,
            GetHelpDir: GetHelpDir::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITypeLibRegistration as windows_core::Interface>::IID
    }
}
pub trait ITypeLibRegistrationReader_Impl: Sized {
    fn EnumTypeLibRegistrations(&self) -> windows_core::Result<IEnumUnknown>;
}
impl windows_core::RuntimeName for ITypeLibRegistrationReader {}
impl ITypeLibRegistrationReader_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ITypeLibRegistrationReader_Vtbl
    where
        Identity: ITypeLibRegistrationReader_Impl,
    {
        unsafe extern "system" fn EnumTypeLibRegistrations<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppenumunknown: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ITypeLibRegistrationReader_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITypeLibRegistrationReader_Impl::EnumTypeLibRegistrations(this) {
                Ok(ok__) => {
                    ppenumunknown.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), EnumTypeLibRegistrations: EnumTypeLibRegistrations::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITypeLibRegistrationReader as windows_core::Interface>::IID
    }
}
pub trait IUri_Impl: Sized {
    fn GetPropertyBSTR(&self, uriprop: Uri_PROPERTY, pbstrproperty: *mut windows_core::BSTR, dwflags: u32) -> windows_core::Result<()>;
    fn GetPropertyLength(&self, uriprop: Uri_PROPERTY, pcchproperty: *mut u32, dwflags: u32) -> windows_core::Result<()>;
    fn GetPropertyDWORD(&self, uriprop: Uri_PROPERTY, pdwproperty: *mut u32, dwflags: u32) -> windows_core::Result<()>;
    fn HasProperty(&self, uriprop: Uri_PROPERTY) -> windows_core::Result<super::super::Foundation::BOOL>;
    fn GetAbsoluteUri(&self) -> windows_core::Result<windows_core::BSTR>;
    fn GetAuthority(&self) -> windows_core::Result<windows_core::BSTR>;
    fn GetDisplayUri(&self) -> windows_core::Result<windows_core::BSTR>;
    fn GetDomain(&self) -> windows_core::Result<windows_core::BSTR>;
    fn GetExtension(&self) -> windows_core::Result<windows_core::BSTR>;
    fn GetFragment(&self) -> windows_core::Result<windows_core::BSTR>;
    fn GetHost(&self) -> windows_core::Result<windows_core::BSTR>;
    fn GetPassword(&self) -> windows_core::Result<windows_core::BSTR>;
    fn GetPath(&self) -> windows_core::Result<windows_core::BSTR>;
    fn GetPathAndQuery(&self) -> windows_core::Result<windows_core::BSTR>;
    fn GetQuery(&self) -> windows_core::Result<windows_core::BSTR>;
    fn GetRawUri(&self) -> windows_core::Result<windows_core::BSTR>;
    fn GetSchemeName(&self) -> windows_core::Result<windows_core::BSTR>;
    fn GetUserInfo(&self) -> windows_core::Result<windows_core::BSTR>;
    fn GetUserName(&self) -> windows_core::Result<windows_core::BSTR>;
    fn GetHostType(&self) -> windows_core::Result<u32>;
    fn GetPort(&self) -> windows_core::Result<u32>;
    fn GetScheme(&self) -> windows_core::Result<u32>;
    fn GetZone(&self) -> windows_core::Result<u32>;
    fn GetProperties(&self) -> windows_core::Result<u32>;
    fn IsEqual(&self, puri: Option<&IUri>) -> windows_core::Result<super::super::Foundation::BOOL>;
}
impl windows_core::RuntimeName for IUri {}
impl IUri_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IUri_Vtbl
    where
        Identity: IUri_Impl,
    {
        unsafe extern "system" fn GetPropertyBSTR<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, uriprop: Uri_PROPERTY, pbstrproperty: *mut core::mem::MaybeUninit<windows_core::BSTR>, dwflags: u32) -> windows_core::HRESULT
        where
            Identity: IUri_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IUri_Impl::GetPropertyBSTR(this, core::mem::transmute_copy(&uriprop), core::mem::transmute_copy(&pbstrproperty), core::mem::transmute_copy(&dwflags)).into()
        }
        unsafe extern "system" fn GetPropertyLength<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, uriprop: Uri_PROPERTY, pcchproperty: *mut u32, dwflags: u32) -> windows_core::HRESULT
        where
            Identity: IUri_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IUri_Impl::GetPropertyLength(this, core::mem::transmute_copy(&uriprop), core::mem::transmute_copy(&pcchproperty), core::mem::transmute_copy(&dwflags)).into()
        }
        unsafe extern "system" fn GetPropertyDWORD<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, uriprop: Uri_PROPERTY, pdwproperty: *mut u32, dwflags: u32) -> windows_core::HRESULT
        where
            Identity: IUri_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IUri_Impl::GetPropertyDWORD(this, core::mem::transmute_copy(&uriprop), core::mem::transmute_copy(&pdwproperty), core::mem::transmute_copy(&dwflags)).into()
        }
        unsafe extern "system" fn HasProperty<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, uriprop: Uri_PROPERTY, pfhasproperty: *mut super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: IUri_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IUri_Impl::HasProperty(this, core::mem::transmute_copy(&uriprop)) {
                Ok(ok__) => {
                    pfhasproperty.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetAbsoluteUri<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrabsoluteuri: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IUri_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IUri_Impl::GetAbsoluteUri(this) {
                Ok(ok__) => {
                    pbstrabsoluteuri.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetAuthority<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrauthority: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IUri_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IUri_Impl::GetAuthority(this) {
                Ok(ok__) => {
                    pbstrauthority.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDisplayUri<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrdisplaystring: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IUri_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IUri_Impl::GetDisplayUri(this) {
                Ok(ok__) => {
                    pbstrdisplaystring.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDomain<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrdomain: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IUri_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IUri_Impl::GetDomain(this) {
                Ok(ok__) => {
                    pbstrdomain.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetExtension<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrextension: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IUri_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IUri_Impl::GetExtension(this) {
                Ok(ok__) => {
                    pbstrextension.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFragment<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrfragment: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IUri_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IUri_Impl::GetFragment(this) {
                Ok(ok__) => {
                    pbstrfragment.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetHost<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrhost: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IUri_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IUri_Impl::GetHost(this) {
                Ok(ok__) => {
                    pbstrhost.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPassword<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrpassword: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IUri_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IUri_Impl::GetPassword(this) {
                Ok(ok__) => {
                    pbstrpassword.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPath<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrpath: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IUri_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IUri_Impl::GetPath(this) {
                Ok(ok__) => {
                    pbstrpath.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPathAndQuery<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrpathandquery: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IUri_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IUri_Impl::GetPathAndQuery(this) {
                Ok(ok__) => {
                    pbstrpathandquery.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetQuery<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrquery: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IUri_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IUri_Impl::GetQuery(this) {
                Ok(ok__) => {
                    pbstrquery.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetRawUri<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrrawuri: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IUri_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IUri_Impl::GetRawUri(this) {
                Ok(ok__) => {
                    pbstrrawuri.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSchemeName<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrschemename: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IUri_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IUri_Impl::GetSchemeName(this) {
                Ok(ok__) => {
                    pbstrschemename.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetUserInfo<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstruserinfo: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IUri_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IUri_Impl::GetUserInfo(this) {
                Ok(ok__) => {
                    pbstruserinfo.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetUserName<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrusername: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IUri_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IUri_Impl::GetUserName(this) {
                Ok(ok__) => {
                    pbstrusername.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetHostType<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwhosttype: *mut u32) -> windows_core::HRESULT
        where
            Identity: IUri_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IUri_Impl::GetHostType(this) {
                Ok(ok__) => {
                    pdwhosttype.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPort<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwport: *mut u32) -> windows_core::HRESULT
        where
            Identity: IUri_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IUri_Impl::GetPort(this) {
                Ok(ok__) => {
                    pdwport.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetScheme<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwscheme: *mut u32) -> windows_core::HRESULT
        where
            Identity: IUri_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IUri_Impl::GetScheme(this) {
                Ok(ok__) => {
                    pdwscheme.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetZone<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwzone: *mut u32) -> windows_core::HRESULT
        where
            Identity: IUri_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IUri_Impl::GetZone(this) {
                Ok(ok__) => {
                    pdwzone.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetProperties<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwflags: *mut u32) -> windows_core::HRESULT
        where
            Identity: IUri_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IUri_Impl::GetProperties(this) {
                Ok(ok__) => {
                    pdwflags.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsEqual<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, puri: *mut core::ffi::c_void, pfequal: *mut super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: IUri_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IUri_Impl::IsEqual(this, windows_core::from_raw_borrowed(&puri)) {
                Ok(ok__) => {
                    pfequal.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetPropertyBSTR: GetPropertyBSTR::<Identity, OFFSET>,
            GetPropertyLength: GetPropertyLength::<Identity, OFFSET>,
            GetPropertyDWORD: GetPropertyDWORD::<Identity, OFFSET>,
            HasProperty: HasProperty::<Identity, OFFSET>,
            GetAbsoluteUri: GetAbsoluteUri::<Identity, OFFSET>,
            GetAuthority: GetAuthority::<Identity, OFFSET>,
            GetDisplayUri: GetDisplayUri::<Identity, OFFSET>,
            GetDomain: GetDomain::<Identity, OFFSET>,
            GetExtension: GetExtension::<Identity, OFFSET>,
            GetFragment: GetFragment::<Identity, OFFSET>,
            GetHost: GetHost::<Identity, OFFSET>,
            GetPassword: GetPassword::<Identity, OFFSET>,
            GetPath: GetPath::<Identity, OFFSET>,
            GetPathAndQuery: GetPathAndQuery::<Identity, OFFSET>,
            GetQuery: GetQuery::<Identity, OFFSET>,
            GetRawUri: GetRawUri::<Identity, OFFSET>,
            GetSchemeName: GetSchemeName::<Identity, OFFSET>,
            GetUserInfo: GetUserInfo::<Identity, OFFSET>,
            GetUserName: GetUserName::<Identity, OFFSET>,
            GetHostType: GetHostType::<Identity, OFFSET>,
            GetPort: GetPort::<Identity, OFFSET>,
            GetScheme: GetScheme::<Identity, OFFSET>,
            GetZone: GetZone::<Identity, OFFSET>,
            GetProperties: GetProperties::<Identity, OFFSET>,
            IsEqual: IsEqual::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IUri as windows_core::Interface>::IID
    }
}
pub trait IUriBuilder_Impl: Sized {
    fn CreateUriSimple(&self, dwallowencodingpropertymask: u32, dwreserved: usize) -> windows_core::Result<IUri>;
    fn CreateUri(&self, dwcreateflags: u32, dwallowencodingpropertymask: u32, dwreserved: usize) -> windows_core::Result<IUri>;
    fn CreateUriWithFlags(&self, dwcreateflags: u32, dwuribuilderflags: u32, dwallowencodingpropertymask: u32, dwreserved: usize) -> windows_core::Result<IUri>;
    fn GetIUri(&self) -> windows_core::Result<IUri>;
    fn SetIUri(&self, piuri: Option<&IUri>) -> windows_core::Result<()>;
    fn GetFragment(&self, pcchfragment: *mut u32, ppwzfragment: *mut windows_core::PCWSTR) -> windows_core::Result<()>;
    fn GetHost(&self, pcchhost: *mut u32, ppwzhost: *mut windows_core::PCWSTR) -> windows_core::Result<()>;
    fn GetPassword(&self, pcchpassword: *mut u32, ppwzpassword: *mut windows_core::PCWSTR) -> windows_core::Result<()>;
    fn GetPath(&self, pcchpath: *mut u32, ppwzpath: *mut windows_core::PCWSTR) -> windows_core::Result<()>;
    fn GetPort(&self, pfhasport: *mut super::super::Foundation::BOOL, pdwport: *mut u32) -> windows_core::Result<()>;
    fn GetQuery(&self, pcchquery: *mut u32, ppwzquery: *mut windows_core::PCWSTR) -> windows_core::Result<()>;
    fn GetSchemeName(&self, pcchschemename: *mut u32, ppwzschemename: *mut windows_core::PCWSTR) -> windows_core::Result<()>;
    fn GetUserName(&self, pcchusername: *mut u32, ppwzusername: *mut windows_core::PCWSTR) -> windows_core::Result<()>;
    fn SetFragment(&self, pwznewvalue: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn SetHost(&self, pwznewvalue: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn SetPassword(&self, pwznewvalue: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn SetPath(&self, pwznewvalue: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn SetPort(&self, fhasport: super::super::Foundation::BOOL, dwnewvalue: u32) -> windows_core::Result<()>;
    fn SetQuery(&self, pwznewvalue: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn SetSchemeName(&self, pwznewvalue: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn SetUserName(&self, pwznewvalue: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn RemoveProperties(&self, dwpropertymask: u32) -> windows_core::Result<()>;
    fn HasBeenModified(&self) -> windows_core::Result<super::super::Foundation::BOOL>;
}
impl windows_core::RuntimeName for IUriBuilder {}
impl IUriBuilder_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IUriBuilder_Vtbl
    where
        Identity: IUriBuilder_Impl,
    {
        unsafe extern "system" fn CreateUriSimple<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwallowencodingpropertymask: u32, dwreserved: usize, ppiuri: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IUriBuilder_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IUriBuilder_Impl::CreateUriSimple(this, core::mem::transmute_copy(&dwallowencodingpropertymask), core::mem::transmute_copy(&dwreserved)) {
                Ok(ok__) => {
                    ppiuri.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateUri<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwcreateflags: u32, dwallowencodingpropertymask: u32, dwreserved: usize, ppiuri: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IUriBuilder_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IUriBuilder_Impl::CreateUri(this, core::mem::transmute_copy(&dwcreateflags), core::mem::transmute_copy(&dwallowencodingpropertymask), core::mem::transmute_copy(&dwreserved)) {
                Ok(ok__) => {
                    ppiuri.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateUriWithFlags<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwcreateflags: u32, dwuribuilderflags: u32, dwallowencodingpropertymask: u32, dwreserved: usize, ppiuri: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IUriBuilder_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IUriBuilder_Impl::CreateUriWithFlags(this, core::mem::transmute_copy(&dwcreateflags), core::mem::transmute_copy(&dwuribuilderflags), core::mem::transmute_copy(&dwallowencodingpropertymask), core::mem::transmute_copy(&dwreserved)) {
                Ok(ok__) => {
                    ppiuri.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetIUri<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppiuri: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IUriBuilder_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IUriBuilder_Impl::GetIUri(this) {
                Ok(ok__) => {
                    ppiuri.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetIUri<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, piuri: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IUriBuilder_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IUriBuilder_Impl::SetIUri(this, windows_core::from_raw_borrowed(&piuri)).into()
        }
        unsafe extern "system" fn GetFragment<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcchfragment: *mut u32, ppwzfragment: *mut windows_core::PCWSTR) -> windows_core::HRESULT
        where
            Identity: IUriBuilder_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IUriBuilder_Impl::GetFragment(this, core::mem::transmute_copy(&pcchfragment), core::mem::transmute_copy(&ppwzfragment)).into()
        }
        unsafe extern "system" fn GetHost<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcchhost: *mut u32, ppwzhost: *mut windows_core::PCWSTR) -> windows_core::HRESULT
        where
            Identity: IUriBuilder_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IUriBuilder_Impl::GetHost(this, core::mem::transmute_copy(&pcchhost), core::mem::transmute_copy(&ppwzhost)).into()
        }
        unsafe extern "system" fn GetPassword<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcchpassword: *mut u32, ppwzpassword: *mut windows_core::PCWSTR) -> windows_core::HRESULT
        where
            Identity: IUriBuilder_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IUriBuilder_Impl::GetPassword(this, core::mem::transmute_copy(&pcchpassword), core::mem::transmute_copy(&ppwzpassword)).into()
        }
        unsafe extern "system" fn GetPath<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcchpath: *mut u32, ppwzpath: *mut windows_core::PCWSTR) -> windows_core::HRESULT
        where
            Identity: IUriBuilder_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IUriBuilder_Impl::GetPath(this, core::mem::transmute_copy(&pcchpath), core::mem::transmute_copy(&ppwzpath)).into()
        }
        unsafe extern "system" fn GetPort<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pfhasport: *mut super::super::Foundation::BOOL, pdwport: *mut u32) -> windows_core::HRESULT
        where
            Identity: IUriBuilder_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IUriBuilder_Impl::GetPort(this, core::mem::transmute_copy(&pfhasport), core::mem::transmute_copy(&pdwport)).into()
        }
        unsafe extern "system" fn GetQuery<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcchquery: *mut u32, ppwzquery: *mut windows_core::PCWSTR) -> windows_core::HRESULT
        where
            Identity: IUriBuilder_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IUriBuilder_Impl::GetQuery(this, core::mem::transmute_copy(&pcchquery), core::mem::transmute_copy(&ppwzquery)).into()
        }
        unsafe extern "system" fn GetSchemeName<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcchschemename: *mut u32, ppwzschemename: *mut windows_core::PCWSTR) -> windows_core::HRESULT
        where
            Identity: IUriBuilder_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IUriBuilder_Impl::GetSchemeName(this, core::mem::transmute_copy(&pcchschemename), core::mem::transmute_copy(&ppwzschemename)).into()
        }
        unsafe extern "system" fn GetUserName<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcchusername: *mut u32, ppwzusername: *mut windows_core::PCWSTR) -> windows_core::HRESULT
        where
            Identity: IUriBuilder_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IUriBuilder_Impl::GetUserName(this, core::mem::transmute_copy(&pcchusername), core::mem::transmute_copy(&ppwzusername)).into()
        }
        unsafe extern "system" fn SetFragment<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwznewvalue: windows_core::PCWSTR) -> windows_core::HRESULT
        where
            Identity: IUriBuilder_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IUriBuilder_Impl::SetFragment(this, core::mem::transmute(&pwznewvalue)).into()
        }
        unsafe extern "system" fn SetHost<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwznewvalue: windows_core::PCWSTR) -> windows_core::HRESULT
        where
            Identity: IUriBuilder_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IUriBuilder_Impl::SetHost(this, core::mem::transmute(&pwznewvalue)).into()
        }
        unsafe extern "system" fn SetPassword<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwznewvalue: windows_core::PCWSTR) -> windows_core::HRESULT
        where
            Identity: IUriBuilder_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IUriBuilder_Impl::SetPassword(this, core::mem::transmute(&pwznewvalue)).into()
        }
        unsafe extern "system" fn SetPath<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwznewvalue: windows_core::PCWSTR) -> windows_core::HRESULT
        where
            Identity: IUriBuilder_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IUriBuilder_Impl::SetPath(this, core::mem::transmute(&pwznewvalue)).into()
        }
        unsafe extern "system" fn SetPort<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, fhasport: super::super::Foundation::BOOL, dwnewvalue: u32) -> windows_core::HRESULT
        where
            Identity: IUriBuilder_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IUriBuilder_Impl::SetPort(this, core::mem::transmute_copy(&fhasport), core::mem::transmute_copy(&dwnewvalue)).into()
        }
        unsafe extern "system" fn SetQuery<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwznewvalue: windows_core::PCWSTR) -> windows_core::HRESULT
        where
            Identity: IUriBuilder_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IUriBuilder_Impl::SetQuery(this, core::mem::transmute(&pwznewvalue)).into()
        }
        unsafe extern "system" fn SetSchemeName<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwznewvalue: windows_core::PCWSTR) -> windows_core::HRESULT
        where
            Identity: IUriBuilder_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IUriBuilder_Impl::SetSchemeName(this, core::mem::transmute(&pwznewvalue)).into()
        }
        unsafe extern "system" fn SetUserName<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwznewvalue: windows_core::PCWSTR) -> windows_core::HRESULT
        where
            Identity: IUriBuilder_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IUriBuilder_Impl::SetUserName(this, core::mem::transmute(&pwznewvalue)).into()
        }
        unsafe extern "system" fn RemoveProperties<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwpropertymask: u32) -> windows_core::HRESULT
        where
            Identity: IUriBuilder_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IUriBuilder_Impl::RemoveProperties(this, core::mem::transmute_copy(&dwpropertymask)).into()
        }
        unsafe extern "system" fn HasBeenModified<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pfmodified: *mut super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: IUriBuilder_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IUriBuilder_Impl::HasBeenModified(this) {
                Ok(ok__) => {
                    pfmodified.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            CreateUriSimple: CreateUriSimple::<Identity, OFFSET>,
            CreateUri: CreateUri::<Identity, OFFSET>,
            CreateUriWithFlags: CreateUriWithFlags::<Identity, OFFSET>,
            GetIUri: GetIUri::<Identity, OFFSET>,
            SetIUri: SetIUri::<Identity, OFFSET>,
            GetFragment: GetFragment::<Identity, OFFSET>,
            GetHost: GetHost::<Identity, OFFSET>,
            GetPassword: GetPassword::<Identity, OFFSET>,
            GetPath: GetPath::<Identity, OFFSET>,
            GetPort: GetPort::<Identity, OFFSET>,
            GetQuery: GetQuery::<Identity, OFFSET>,
            GetSchemeName: GetSchemeName::<Identity, OFFSET>,
            GetUserName: GetUserName::<Identity, OFFSET>,
            SetFragment: SetFragment::<Identity, OFFSET>,
            SetHost: SetHost::<Identity, OFFSET>,
            SetPassword: SetPassword::<Identity, OFFSET>,
            SetPath: SetPath::<Identity, OFFSET>,
            SetPort: SetPort::<Identity, OFFSET>,
            SetQuery: SetQuery::<Identity, OFFSET>,
            SetSchemeName: SetSchemeName::<Identity, OFFSET>,
            SetUserName: SetUserName::<Identity, OFFSET>,
            RemoveProperties: RemoveProperties::<Identity, OFFSET>,
            HasBeenModified: HasBeenModified::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IUriBuilder as windows_core::Interface>::IID
    }
}
pub trait IUrlMon_Impl: Sized {
    fn AsyncGetClassBits(&self, rclsid: *const windows_core::GUID, psztype: &windows_core::PCWSTR, pszext: &windows_core::PCWSTR, dwfileversionms: u32, dwfileversionls: u32, pszcodebase: &windows_core::PCWSTR, pbc: Option<&IBindCtx>, dwclasscontext: u32, riid: *const windows_core::GUID, flags: u32) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IUrlMon {}
impl IUrlMon_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IUrlMon_Vtbl
    where
        Identity: IUrlMon_Impl,
    {
        unsafe extern "system" fn AsyncGetClassBits<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, rclsid: *const windows_core::GUID, psztype: windows_core::PCWSTR, pszext: windows_core::PCWSTR, dwfileversionms: u32, dwfileversionls: u32, pszcodebase: windows_core::PCWSTR, pbc: *mut core::ffi::c_void, dwclasscontext: u32, riid: *const windows_core::GUID, flags: u32) -> windows_core::HRESULT
        where
            Identity: IUrlMon_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IUrlMon_Impl::AsyncGetClassBits(this, core::mem::transmute_copy(&rclsid), core::mem::transmute(&psztype), core::mem::transmute(&pszext), core::mem::transmute_copy(&dwfileversionms), core::mem::transmute_copy(&dwfileversionls), core::mem::transmute(&pszcodebase), windows_core::from_raw_borrowed(&pbc), core::mem::transmute_copy(&dwclasscontext), core::mem::transmute_copy(&riid), core::mem::transmute_copy(&flags)).into()
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), AsyncGetClassBits: AsyncGetClassBits::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IUrlMon as windows_core::Interface>::IID
    }
}
pub trait IWaitMultiple_Impl: Sized {
    fn WaitMultiple(&self, timeout: u32) -> windows_core::Result<ISynchronize>;
    fn AddSynchronize(&self, psync: Option<&ISynchronize>) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IWaitMultiple {}
impl IWaitMultiple_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IWaitMultiple_Vtbl
    where
        Identity: IWaitMultiple_Impl,
    {
        unsafe extern "system" fn WaitMultiple<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, timeout: u32, psync: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IWaitMultiple_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWaitMultiple_Impl::WaitMultiple(this, core::mem::transmute_copy(&timeout)) {
                Ok(ok__) => {
                    psync.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AddSynchronize<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, psync: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IWaitMultiple_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWaitMultiple_Impl::AddSynchronize(this, windows_core::from_raw_borrowed(&psync)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            WaitMultiple: WaitMultiple::<Identity, OFFSET>,
            AddSynchronize: AddSynchronize::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWaitMultiple as windows_core::Interface>::IID
    }
}
