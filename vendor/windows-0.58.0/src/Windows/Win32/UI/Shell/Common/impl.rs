pub trait IObjectArray_Impl: Sized {
    fn GetCount(&self) -> windows_core::Result<u32>;
    fn GetAt(&self, uiindex: u32, riid: *const windows_core::GUID, ppv: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IObjectArray {}
impl IObjectArray_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IObjectArray_Vtbl
    where
        Identity: IObjectArray_Impl,
    {
        unsafe extern "system" fn GetCount<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcobjects: *mut u32) -> windows_core::HRESULT
        where
            Identity: IObjectArray_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IObjectArray_Impl::GetCount(this) {
                Ok(ok__) => {
                    pcobjects.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetAt<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, uiindex: u32, riid: *const windows_core::GUID, ppv: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IObjectArray_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IObjectArray_Impl::GetAt(this, core::mem::transmute_copy(&uiindex), core::mem::transmute_copy(&riid), core::mem::transmute_copy(&ppv)).into()
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), GetCount: GetCount::<Identity, OFFSET>, GetAt: GetAt::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IObjectArray as windows_core::Interface>::IID
    }
}
pub trait IObjectCollection_Impl: Sized + IObjectArray_Impl {
    fn AddObject(&self, punk: Option<&windows_core::IUnknown>) -> windows_core::Result<()>;
    fn AddFromArray(&self, poasource: Option<&IObjectArray>) -> windows_core::Result<()>;
    fn RemoveObjectAt(&self, uiindex: u32) -> windows_core::Result<()>;
    fn Clear(&self) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IObjectCollection {}
impl IObjectCollection_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IObjectCollection_Vtbl
    where
        Identity: IObjectCollection_Impl,
    {
        unsafe extern "system" fn AddObject<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, punk: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IObjectCollection_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IObjectCollection_Impl::AddObject(this, windows_core::from_raw_borrowed(&punk)).into()
        }
        unsafe extern "system" fn AddFromArray<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, poasource: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IObjectCollection_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IObjectCollection_Impl::AddFromArray(this, windows_core::from_raw_borrowed(&poasource)).into()
        }
        unsafe extern "system" fn RemoveObjectAt<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, uiindex: u32) -> windows_core::HRESULT
        where
            Identity: IObjectCollection_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IObjectCollection_Impl::RemoveObjectAt(this, core::mem::transmute_copy(&uiindex)).into()
        }
        unsafe extern "system" fn Clear<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IObjectCollection_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IObjectCollection_Impl::Clear(this).into()
        }
        Self {
            base__: IObjectArray_Vtbl::new::<Identity, OFFSET>(),
            AddObject: AddObject::<Identity, OFFSET>,
            AddFromArray: AddFromArray::<Identity, OFFSET>,
            RemoveObjectAt: RemoveObjectAt::<Identity, OFFSET>,
            Clear: Clear::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IObjectCollection as windows_core::Interface>::IID || iid == &<IObjectArray as windows_core::Interface>::IID
    }
}
