pub trait IWindowsDevicesAllJoynBusAttachmentFactoryInterop_Impl: Sized {
    fn CreateFromWin32Handle(&self, win32handle: u64, enableaboutdata: u8, riid: *const windows_core::GUID, ppv: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IWindowsDevicesAllJoynBusAttachmentFactoryInterop {}
impl IWindowsDevicesAllJoynBusAttachmentFactoryInterop_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IWindowsDevicesAllJoynBusAttachmentFactoryInterop_Vtbl
    where
        Identity: IWindowsDevicesAllJoynBusAttachmentFactoryInterop_Impl,
    {
        unsafe extern "system" fn CreateFromWin32Handle<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, win32handle: u64, enableaboutdata: u8, riid: *const windows_core::GUID, ppv: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IWindowsDevicesAllJoynBusAttachmentFactoryInterop_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWindowsDevicesAllJoynBusAttachmentFactoryInterop_Impl::CreateFromWin32Handle(this, core::mem::transmute_copy(&win32handle), core::mem::transmute_copy(&enableaboutdata), core::mem::transmute_copy(&riid), core::mem::transmute_copy(&ppv)).into()
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, IWindowsDevicesAllJoynBusAttachmentFactoryInterop, OFFSET>(),
            CreateFromWin32Handle: CreateFromWin32Handle::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWindowsDevicesAllJoynBusAttachmentFactoryInterop as windows_core::Interface>::IID
    }
}
pub trait IWindowsDevicesAllJoynBusAttachmentInterop_Impl: Sized {
    fn Win32Handle(&self) -> windows_core::Result<u64>;
}
impl windows_core::RuntimeName for IWindowsDevicesAllJoynBusAttachmentInterop {}
impl IWindowsDevicesAllJoynBusAttachmentInterop_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IWindowsDevicesAllJoynBusAttachmentInterop_Vtbl
    where
        Identity: IWindowsDevicesAllJoynBusAttachmentInterop_Impl,
    {
        unsafe extern "system" fn Win32Handle<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut u64) -> windows_core::HRESULT
        where
            Identity: IWindowsDevicesAllJoynBusAttachmentInterop_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWindowsDevicesAllJoynBusAttachmentInterop_Impl::Win32Handle(this) {
                Ok(ok__) => {
                    value.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, IWindowsDevicesAllJoynBusAttachmentInterop, OFFSET>(),
            Win32Handle: Win32Handle::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWindowsDevicesAllJoynBusAttachmentInterop as windows_core::Interface>::IID
    }
}
pub trait IWindowsDevicesAllJoynBusObjectFactoryInterop_Impl: Sized {
    fn CreateFromWin32Handle(&self, win32handle: u64, riid: *const windows_core::GUID, ppv: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IWindowsDevicesAllJoynBusObjectFactoryInterop {}
impl IWindowsDevicesAllJoynBusObjectFactoryInterop_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IWindowsDevicesAllJoynBusObjectFactoryInterop_Vtbl
    where
        Identity: IWindowsDevicesAllJoynBusObjectFactoryInterop_Impl,
    {
        unsafe extern "system" fn CreateFromWin32Handle<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, win32handle: u64, riid: *const windows_core::GUID, ppv: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IWindowsDevicesAllJoynBusObjectFactoryInterop_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWindowsDevicesAllJoynBusObjectFactoryInterop_Impl::CreateFromWin32Handle(this, core::mem::transmute_copy(&win32handle), core::mem::transmute_copy(&riid), core::mem::transmute_copy(&ppv)).into()
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, IWindowsDevicesAllJoynBusObjectFactoryInterop, OFFSET>(),
            CreateFromWin32Handle: CreateFromWin32Handle::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWindowsDevicesAllJoynBusObjectFactoryInterop as windows_core::Interface>::IID
    }
}
pub trait IWindowsDevicesAllJoynBusObjectInterop_Impl: Sized {
    fn AddPropertyGetHandler(&self, context: *const core::ffi::c_void, interfacename: &windows_core::HSTRING, callback: isize) -> windows_core::Result<()>;
    fn AddPropertySetHandler(&self, context: *const core::ffi::c_void, interfacename: &windows_core::HSTRING, callback: isize) -> windows_core::Result<()>;
    fn Win32Handle(&self) -> windows_core::Result<u64>;
}
impl windows_core::RuntimeName for IWindowsDevicesAllJoynBusObjectInterop {}
impl IWindowsDevicesAllJoynBusObjectInterop_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IWindowsDevicesAllJoynBusObjectInterop_Vtbl
    where
        Identity: IWindowsDevicesAllJoynBusObjectInterop_Impl,
    {
        unsafe extern "system" fn AddPropertyGetHandler<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, context: *const core::ffi::c_void, interfacename: core::mem::MaybeUninit<windows_core::HSTRING>, callback: isize) -> windows_core::HRESULT
        where
            Identity: IWindowsDevicesAllJoynBusObjectInterop_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWindowsDevicesAllJoynBusObjectInterop_Impl::AddPropertyGetHandler(this, core::mem::transmute_copy(&context), core::mem::transmute(&interfacename), core::mem::transmute_copy(&callback)).into()
        }
        unsafe extern "system" fn AddPropertySetHandler<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, context: *const core::ffi::c_void, interfacename: core::mem::MaybeUninit<windows_core::HSTRING>, callback: isize) -> windows_core::HRESULT
        where
            Identity: IWindowsDevicesAllJoynBusObjectInterop_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWindowsDevicesAllJoynBusObjectInterop_Impl::AddPropertySetHandler(this, core::mem::transmute_copy(&context), core::mem::transmute(&interfacename), core::mem::transmute_copy(&callback)).into()
        }
        unsafe extern "system" fn Win32Handle<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut u64) -> windows_core::HRESULT
        where
            Identity: IWindowsDevicesAllJoynBusObjectInterop_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWindowsDevicesAllJoynBusObjectInterop_Impl::Win32Handle(this) {
                Ok(ok__) => {
                    value.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, IWindowsDevicesAllJoynBusObjectInterop, OFFSET>(),
            AddPropertyGetHandler: AddPropertyGetHandler::<Identity, OFFSET>,
            AddPropertySetHandler: AddPropertySetHandler::<Identity, OFFSET>,
            Win32Handle: Win32Handle::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWindowsDevicesAllJoynBusObjectInterop as windows_core::Interface>::IID
    }
}
