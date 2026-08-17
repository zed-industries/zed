pub trait ICreateDeviceAccessAsync_Impl: Sized {
    fn Cancel(&self) -> windows_core::Result<()>;
    fn Wait(&self, timeout: u32) -> windows_core::Result<()>;
    fn Close(&self) -> windows_core::Result<()>;
    fn GetResult(&self, riid: *const windows_core::GUID, deviceaccess: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for ICreateDeviceAccessAsync {}
impl ICreateDeviceAccessAsync_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ICreateDeviceAccessAsync_Vtbl
    where
        Identity: ICreateDeviceAccessAsync_Impl,
    {
        unsafe extern "system" fn Cancel<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ICreateDeviceAccessAsync_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICreateDeviceAccessAsync_Impl::Cancel(this).into()
        }
        unsafe extern "system" fn Wait<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, timeout: u32) -> windows_core::HRESULT
        where
            Identity: ICreateDeviceAccessAsync_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICreateDeviceAccessAsync_Impl::Wait(this, core::mem::transmute_copy(&timeout)).into()
        }
        unsafe extern "system" fn Close<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ICreateDeviceAccessAsync_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICreateDeviceAccessAsync_Impl::Close(this).into()
        }
        unsafe extern "system" fn GetResult<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, riid: *const windows_core::GUID, deviceaccess: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ICreateDeviceAccessAsync_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICreateDeviceAccessAsync_Impl::GetResult(this, core::mem::transmute_copy(&riid), core::mem::transmute_copy(&deviceaccess)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Cancel: Cancel::<Identity, OFFSET>,
            Wait: Wait::<Identity, OFFSET>,
            Close: Close::<Identity, OFFSET>,
            GetResult: GetResult::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ICreateDeviceAccessAsync as windows_core::Interface>::IID
    }
}
pub trait IDeviceIoControl_Impl: Sized {
    fn DeviceIoControlSync(&self, iocontrolcode: u32, inputbuffer: *const u8, inputbuffersize: u32, outputbuffer: *mut u8, outputbuffersize: u32, bytesreturned: *mut u32) -> windows_core::Result<()>;
    fn DeviceIoControlAsync(&self, iocontrolcode: u32, inputbuffer: *const u8, inputbuffersize: u32, outputbuffer: *mut u8, outputbuffersize: u32, requestcompletioncallback: Option<&IDeviceRequestCompletionCallback>, cancelcontext: *mut usize) -> windows_core::Result<()>;
    fn CancelOperation(&self, cancelcontext: usize) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IDeviceIoControl {}
impl IDeviceIoControl_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IDeviceIoControl_Vtbl
    where
        Identity: IDeviceIoControl_Impl,
    {
        unsafe extern "system" fn DeviceIoControlSync<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, iocontrolcode: u32, inputbuffer: *const u8, inputbuffersize: u32, outputbuffer: *mut u8, outputbuffersize: u32, bytesreturned: *mut u32) -> windows_core::HRESULT
        where
            Identity: IDeviceIoControl_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDeviceIoControl_Impl::DeviceIoControlSync(this, core::mem::transmute_copy(&iocontrolcode), core::mem::transmute_copy(&inputbuffer), core::mem::transmute_copy(&inputbuffersize), core::mem::transmute_copy(&outputbuffer), core::mem::transmute_copy(&outputbuffersize), core::mem::transmute_copy(&bytesreturned)).into()
        }
        unsafe extern "system" fn DeviceIoControlAsync<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, iocontrolcode: u32, inputbuffer: *const u8, inputbuffersize: u32, outputbuffer: *mut u8, outputbuffersize: u32, requestcompletioncallback: *mut core::ffi::c_void, cancelcontext: *mut usize) -> windows_core::HRESULT
        where
            Identity: IDeviceIoControl_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDeviceIoControl_Impl::DeviceIoControlAsync(this, core::mem::transmute_copy(&iocontrolcode), core::mem::transmute_copy(&inputbuffer), core::mem::transmute_copy(&inputbuffersize), core::mem::transmute_copy(&outputbuffer), core::mem::transmute_copy(&outputbuffersize), windows_core::from_raw_borrowed(&requestcompletioncallback), core::mem::transmute_copy(&cancelcontext)).into()
        }
        unsafe extern "system" fn CancelOperation<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, cancelcontext: usize) -> windows_core::HRESULT
        where
            Identity: IDeviceIoControl_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDeviceIoControl_Impl::CancelOperation(this, core::mem::transmute_copy(&cancelcontext)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            DeviceIoControlSync: DeviceIoControlSync::<Identity, OFFSET>,
            DeviceIoControlAsync: DeviceIoControlAsync::<Identity, OFFSET>,
            CancelOperation: CancelOperation::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDeviceIoControl as windows_core::Interface>::IID
    }
}
pub trait IDeviceRequestCompletionCallback_Impl: Sized {
    fn Invoke(&self, requestresult: windows_core::HRESULT, bytesreturned: u32) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IDeviceRequestCompletionCallback {}
impl IDeviceRequestCompletionCallback_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IDeviceRequestCompletionCallback_Vtbl
    where
        Identity: IDeviceRequestCompletionCallback_Impl,
    {
        unsafe extern "system" fn Invoke<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, requestresult: windows_core::HRESULT, bytesreturned: u32) -> windows_core::HRESULT
        where
            Identity: IDeviceRequestCompletionCallback_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDeviceRequestCompletionCallback_Impl::Invoke(this, core::mem::transmute_copy(&requestresult), core::mem::transmute_copy(&bytesreturned)).into()
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), Invoke: Invoke::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDeviceRequestCompletionCallback as windows_core::Interface>::IID
    }
}
