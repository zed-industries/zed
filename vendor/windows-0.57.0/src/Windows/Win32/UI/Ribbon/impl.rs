pub trait IUIApplication_Impl: Sized {
    fn OnViewChanged(&self, viewid: u32, typeid: UI_VIEWTYPE, view: Option<&windows_core::IUnknown>, verb: UI_VIEWVERB, ureasoncode: i32) -> windows_core::Result<()>;
    fn OnCreateUICommand(&self, commandid: u32, typeid: UI_COMMANDTYPE) -> windows_core::Result<IUICommandHandler>;
    fn OnDestroyUICommand(&self, commandid: u32, typeid: UI_COMMANDTYPE, commandhandler: Option<&IUICommandHandler>) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IUIApplication {}
impl IUIApplication_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIApplication_Impl, const OFFSET: isize>() -> IUIApplication_Vtbl {
        unsafe extern "system" fn OnViewChanged<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIApplication_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, viewid: u32, typeid: UI_VIEWTYPE, view: *mut core::ffi::c_void, verb: UI_VIEWVERB, ureasoncode: i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IUIApplication_Impl::OnViewChanged(this, core::mem::transmute_copy(&viewid), core::mem::transmute_copy(&typeid), windows_core::from_raw_borrowed(&view), core::mem::transmute_copy(&verb), core::mem::transmute_copy(&ureasoncode)).into()
        }
        unsafe extern "system" fn OnCreateUICommand<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIApplication_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, commandid: u32, typeid: UI_COMMANDTYPE, commandhandler: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IUIApplication_Impl::OnCreateUICommand(this, core::mem::transmute_copy(&commandid), core::mem::transmute_copy(&typeid)) {
                Ok(ok__) => {
                    core::ptr::write(commandhandler, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OnDestroyUICommand<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIApplication_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, commandid: u32, typeid: UI_COMMANDTYPE, commandhandler: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IUIApplication_Impl::OnDestroyUICommand(this, core::mem::transmute_copy(&commandid), core::mem::transmute_copy(&typeid), windows_core::from_raw_borrowed(&commandhandler)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            OnViewChanged: OnViewChanged::<Identity, Impl, OFFSET>,
            OnCreateUICommand: OnCreateUICommand::<Identity, Impl, OFFSET>,
            OnDestroyUICommand: OnDestroyUICommand::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IUIApplication as windows_core::Interface>::IID
    }
}
pub trait IUICollection_Impl: Sized {
    fn GetCount(&self) -> windows_core::Result<u32>;
    fn GetItem(&self, index: u32) -> windows_core::Result<windows_core::IUnknown>;
    fn Add(&self, item: Option<&windows_core::IUnknown>) -> windows_core::Result<()>;
    fn Insert(&self, index: u32, item: Option<&windows_core::IUnknown>) -> windows_core::Result<()>;
    fn RemoveAt(&self, index: u32) -> windows_core::Result<()>;
    fn Replace(&self, indexreplaced: u32, itemreplacewith: Option<&windows_core::IUnknown>) -> windows_core::Result<()>;
    fn Clear(&self) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IUICollection {}
impl IUICollection_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IUICollection_Impl, const OFFSET: isize>() -> IUICollection_Vtbl {
        unsafe extern "system" fn GetCount<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IUICollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, count: *mut u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IUICollection_Impl::GetCount(this) {
                Ok(ok__) => {
                    core::ptr::write(count, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetItem<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IUICollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: u32, item: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IUICollection_Impl::GetItem(this, core::mem::transmute_copy(&index)) {
                Ok(ok__) => {
                    core::ptr::write(item, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Add<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IUICollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, item: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IUICollection_Impl::Add(this, windows_core::from_raw_borrowed(&item)).into()
        }
        unsafe extern "system" fn Insert<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IUICollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: u32, item: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IUICollection_Impl::Insert(this, core::mem::transmute_copy(&index), windows_core::from_raw_borrowed(&item)).into()
        }
        unsafe extern "system" fn RemoveAt<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IUICollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IUICollection_Impl::RemoveAt(this, core::mem::transmute_copy(&index)).into()
        }
        unsafe extern "system" fn Replace<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IUICollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, indexreplaced: u32, itemreplacewith: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IUICollection_Impl::Replace(this, core::mem::transmute_copy(&indexreplaced), windows_core::from_raw_borrowed(&itemreplacewith)).into()
        }
        unsafe extern "system" fn Clear<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IUICollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IUICollection_Impl::Clear(this).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetCount: GetCount::<Identity, Impl, OFFSET>,
            GetItem: GetItem::<Identity, Impl, OFFSET>,
            Add: Add::<Identity, Impl, OFFSET>,
            Insert: Insert::<Identity, Impl, OFFSET>,
            RemoveAt: RemoveAt::<Identity, Impl, OFFSET>,
            Replace: Replace::<Identity, Impl, OFFSET>,
            Clear: Clear::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IUICollection as windows_core::Interface>::IID
    }
}
pub trait IUICollectionChangedEvent_Impl: Sized {
    fn OnChanged(&self, action: UI_COLLECTIONCHANGE, oldindex: u32, olditem: Option<&windows_core::IUnknown>, newindex: u32, newitem: Option<&windows_core::IUnknown>) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IUICollectionChangedEvent {}
impl IUICollectionChangedEvent_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IUICollectionChangedEvent_Impl, const OFFSET: isize>() -> IUICollectionChangedEvent_Vtbl {
        unsafe extern "system" fn OnChanged<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IUICollectionChangedEvent_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, action: UI_COLLECTIONCHANGE, oldindex: u32, olditem: *mut core::ffi::c_void, newindex: u32, newitem: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IUICollectionChangedEvent_Impl::OnChanged(this, core::mem::transmute_copy(&action), core::mem::transmute_copy(&oldindex), windows_core::from_raw_borrowed(&olditem), core::mem::transmute_copy(&newindex), windows_core::from_raw_borrowed(&newitem)).into()
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), OnChanged: OnChanged::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IUICollectionChangedEvent as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub trait IUICommandHandler_Impl: Sized {
    fn Execute(&self, commandid: u32, verb: UI_EXECUTIONVERB, key: *const super::Shell::PropertiesSystem::PROPERTYKEY, currentvalue: *const windows_core::PROPVARIANT, commandexecutionproperties: Option<&IUISimplePropertySet>) -> windows_core::Result<()>;
    fn UpdateProperty(&self, commandid: u32, key: *const super::Shell::PropertiesSystem::PROPERTYKEY, currentvalue: *const windows_core::PROPVARIANT) -> windows_core::Result<windows_core::PROPVARIANT>;
}
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
impl windows_core::RuntimeName for IUICommandHandler {}
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
impl IUICommandHandler_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IUICommandHandler_Impl, const OFFSET: isize>() -> IUICommandHandler_Vtbl {
        unsafe extern "system" fn Execute<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IUICommandHandler_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, commandid: u32, verb: UI_EXECUTIONVERB, key: *const super::Shell::PropertiesSystem::PROPERTYKEY, currentvalue: *const core::mem::MaybeUninit<windows_core::PROPVARIANT>, commandexecutionproperties: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IUICommandHandler_Impl::Execute(this, core::mem::transmute_copy(&commandid), core::mem::transmute_copy(&verb), core::mem::transmute_copy(&key), core::mem::transmute_copy(&currentvalue), windows_core::from_raw_borrowed(&commandexecutionproperties)).into()
        }
        unsafe extern "system" fn UpdateProperty<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IUICommandHandler_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, commandid: u32, key: *const super::Shell::PropertiesSystem::PROPERTYKEY, currentvalue: *const core::mem::MaybeUninit<windows_core::PROPVARIANT>, newvalue: *mut core::mem::MaybeUninit<windows_core::PROPVARIANT>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IUICommandHandler_Impl::UpdateProperty(this, core::mem::transmute_copy(&commandid), core::mem::transmute_copy(&key), core::mem::transmute_copy(&currentvalue)) {
                Ok(ok__) => {
                    core::ptr::write(newvalue, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Execute: Execute::<Identity, Impl, OFFSET>,
            UpdateProperty: UpdateProperty::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IUICommandHandler as windows_core::Interface>::IID
    }
}
pub trait IUIContextualUI_Impl: Sized {
    fn ShowAtLocation(&self, x: i32, y: i32) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IUIContextualUI {}
impl IUIContextualUI_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIContextualUI_Impl, const OFFSET: isize>() -> IUIContextualUI_Vtbl {
        unsafe extern "system" fn ShowAtLocation<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIContextualUI_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, x: i32, y: i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IUIContextualUI_Impl::ShowAtLocation(this, core::mem::transmute_copy(&x), core::mem::transmute_copy(&y)).into()
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), ShowAtLocation: ShowAtLocation::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IUIContextualUI as windows_core::Interface>::IID
    }
}
pub trait IUIEventLogger_Impl: Sized {
    fn OnUIEvent(&self, peventparams: *const UI_EVENTPARAMS);
}
impl windows_core::RuntimeName for IUIEventLogger {}
impl IUIEventLogger_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIEventLogger_Impl, const OFFSET: isize>() -> IUIEventLogger_Vtbl {
        unsafe extern "system" fn OnUIEvent<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIEventLogger_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, peventparams: *const UI_EVENTPARAMS) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IUIEventLogger_Impl::OnUIEvent(this, core::mem::transmute_copy(&peventparams))
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), OnUIEvent: OnUIEvent::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IUIEventLogger as windows_core::Interface>::IID
    }
}
pub trait IUIEventingManager_Impl: Sized {
    fn SetEventLogger(&self, eventlogger: Option<&IUIEventLogger>) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IUIEventingManager {}
impl IUIEventingManager_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIEventingManager_Impl, const OFFSET: isize>() -> IUIEventingManager_Vtbl {
        unsafe extern "system" fn SetEventLogger<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIEventingManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, eventlogger: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IUIEventingManager_Impl::SetEventLogger(this, windows_core::from_raw_borrowed(&eventlogger)).into()
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), SetEventLogger: SetEventLogger::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IUIEventingManager as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub trait IUIFramework_Impl: Sized {
    fn Initialize(&self, framewnd: super::super::Foundation::HWND, application: Option<&IUIApplication>) -> windows_core::Result<()>;
    fn Destroy(&self) -> windows_core::Result<()>;
    fn LoadUI(&self, instance: super::super::Foundation::HINSTANCE, resourcename: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn GetView(&self, viewid: u32, riid: *const windows_core::GUID, ppv: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
    fn GetUICommandProperty(&self, commandid: u32, key: *const super::Shell::PropertiesSystem::PROPERTYKEY) -> windows_core::Result<windows_core::PROPVARIANT>;
    fn SetUICommandProperty(&self, commandid: u32, key: *const super::Shell::PropertiesSystem::PROPERTYKEY, value: *const windows_core::PROPVARIANT) -> windows_core::Result<()>;
    fn InvalidateUICommand(&self, commandid: u32, flags: UI_INVALIDATIONS, key: *const super::Shell::PropertiesSystem::PROPERTYKEY) -> windows_core::Result<()>;
    fn FlushPendingInvalidations(&self) -> windows_core::Result<()>;
    fn SetModes(&self, imodes: i32) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
impl windows_core::RuntimeName for IUIFramework {}
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
impl IUIFramework_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIFramework_Impl, const OFFSET: isize>() -> IUIFramework_Vtbl {
        unsafe extern "system" fn Initialize<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIFramework_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, framewnd: super::super::Foundation::HWND, application: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IUIFramework_Impl::Initialize(this, core::mem::transmute_copy(&framewnd), windows_core::from_raw_borrowed(&application)).into()
        }
        unsafe extern "system" fn Destroy<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIFramework_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IUIFramework_Impl::Destroy(this).into()
        }
        unsafe extern "system" fn LoadUI<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIFramework_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, instance: super::super::Foundation::HINSTANCE, resourcename: windows_core::PCWSTR) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IUIFramework_Impl::LoadUI(this, core::mem::transmute_copy(&instance), core::mem::transmute(&resourcename)).into()
        }
        unsafe extern "system" fn GetView<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIFramework_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, viewid: u32, riid: *const windows_core::GUID, ppv: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IUIFramework_Impl::GetView(this, core::mem::transmute_copy(&viewid), core::mem::transmute_copy(&riid), core::mem::transmute_copy(&ppv)).into()
        }
        unsafe extern "system" fn GetUICommandProperty<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIFramework_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, commandid: u32, key: *const super::Shell::PropertiesSystem::PROPERTYKEY, value: *mut core::mem::MaybeUninit<windows_core::PROPVARIANT>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IUIFramework_Impl::GetUICommandProperty(this, core::mem::transmute_copy(&commandid), core::mem::transmute_copy(&key)) {
                Ok(ok__) => {
                    core::ptr::write(value, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetUICommandProperty<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIFramework_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, commandid: u32, key: *const super::Shell::PropertiesSystem::PROPERTYKEY, value: *const core::mem::MaybeUninit<windows_core::PROPVARIANT>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IUIFramework_Impl::SetUICommandProperty(this, core::mem::transmute_copy(&commandid), core::mem::transmute_copy(&key), core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn InvalidateUICommand<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIFramework_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, commandid: u32, flags: UI_INVALIDATIONS, key: *const super::Shell::PropertiesSystem::PROPERTYKEY) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IUIFramework_Impl::InvalidateUICommand(this, core::mem::transmute_copy(&commandid), core::mem::transmute_copy(&flags), core::mem::transmute_copy(&key)).into()
        }
        unsafe extern "system" fn FlushPendingInvalidations<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIFramework_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IUIFramework_Impl::FlushPendingInvalidations(this).into()
        }
        unsafe extern "system" fn SetModes<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIFramework_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, imodes: i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IUIFramework_Impl::SetModes(this, core::mem::transmute_copy(&imodes)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Initialize: Initialize::<Identity, Impl, OFFSET>,
            Destroy: Destroy::<Identity, Impl, OFFSET>,
            LoadUI: LoadUI::<Identity, Impl, OFFSET>,
            GetView: GetView::<Identity, Impl, OFFSET>,
            GetUICommandProperty: GetUICommandProperty::<Identity, Impl, OFFSET>,
            SetUICommandProperty: SetUICommandProperty::<Identity, Impl, OFFSET>,
            InvalidateUICommand: InvalidateUICommand::<Identity, Impl, OFFSET>,
            FlushPendingInvalidations: FlushPendingInvalidations::<Identity, Impl, OFFSET>,
            SetModes: SetModes::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IUIFramework as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Graphics_Gdi")]
pub trait IUIImage_Impl: Sized {
    fn GetBitmap(&self) -> windows_core::Result<super::super::Graphics::Gdi::HBITMAP>;
}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl windows_core::RuntimeName for IUIImage {}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl IUIImage_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIImage_Impl, const OFFSET: isize>() -> IUIImage_Vtbl {
        unsafe extern "system" fn GetBitmap<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIImage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bitmap: *mut super::super::Graphics::Gdi::HBITMAP) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IUIImage_Impl::GetBitmap(this) {
                Ok(ok__) => {
                    core::ptr::write(bitmap, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), GetBitmap: GetBitmap::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IUIImage as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Graphics_Gdi")]
pub trait IUIImageFromBitmap_Impl: Sized {
    fn CreateImage(&self, bitmap: super::super::Graphics::Gdi::HBITMAP, options: UI_OWNERSHIP) -> windows_core::Result<IUIImage>;
}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl windows_core::RuntimeName for IUIImageFromBitmap {}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl IUIImageFromBitmap_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIImageFromBitmap_Impl, const OFFSET: isize>() -> IUIImageFromBitmap_Vtbl {
        unsafe extern "system" fn CreateImage<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIImageFromBitmap_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bitmap: super::super::Graphics::Gdi::HBITMAP, options: UI_OWNERSHIP, image: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IUIImageFromBitmap_Impl::CreateImage(this, core::mem::transmute_copy(&bitmap), core::mem::transmute_copy(&options)) {
                Ok(ok__) => {
                    core::ptr::write(image, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), CreateImage: CreateImage::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IUIImageFromBitmap as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IUIRibbon_Impl: Sized {
    fn GetHeight(&self) -> windows_core::Result<u32>;
    fn LoadSettingsFromStream(&self, pstream: Option<&super::super::System::Com::IStream>) -> windows_core::Result<()>;
    fn SaveSettingsToStream(&self, pstream: Option<&super::super::System::Com::IStream>) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IUIRibbon {}
#[cfg(feature = "Win32_System_Com")]
impl IUIRibbon_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIRibbon_Impl, const OFFSET: isize>() -> IUIRibbon_Vtbl {
        unsafe extern "system" fn GetHeight<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIRibbon_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, cy: *mut u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IUIRibbon_Impl::GetHeight(this) {
                Ok(ok__) => {
                    core::ptr::write(cy, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LoadSettingsFromStream<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIRibbon_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pstream: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IUIRibbon_Impl::LoadSettingsFromStream(this, windows_core::from_raw_borrowed(&pstream)).into()
        }
        unsafe extern "system" fn SaveSettingsToStream<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIRibbon_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pstream: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IUIRibbon_Impl::SaveSettingsToStream(this, windows_core::from_raw_borrowed(&pstream)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetHeight: GetHeight::<Identity, Impl, OFFSET>,
            LoadSettingsFromStream: LoadSettingsFromStream::<Identity, Impl, OFFSET>,
            SaveSettingsToStream: SaveSettingsToStream::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IUIRibbon as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub trait IUISimplePropertySet_Impl: Sized {
    fn GetValue(&self, key: *const super::Shell::PropertiesSystem::PROPERTYKEY) -> windows_core::Result<windows_core::PROPVARIANT>;
}
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
impl windows_core::RuntimeName for IUISimplePropertySet {}
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
impl IUISimplePropertySet_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IUISimplePropertySet_Impl, const OFFSET: isize>() -> IUISimplePropertySet_Vtbl {
        unsafe extern "system" fn GetValue<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IUISimplePropertySet_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, key: *const super::Shell::PropertiesSystem::PROPERTYKEY, value: *mut core::mem::MaybeUninit<windows_core::PROPVARIANT>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IUISimplePropertySet_Impl::GetValue(this, core::mem::transmute_copy(&key)) {
                Ok(ok__) => {
                    core::ptr::write(value, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), GetValue: GetValue::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IUISimplePropertySet as windows_core::Interface>::IID
    }
}
