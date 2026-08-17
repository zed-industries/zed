pub trait IRtwqAsyncCallback_Impl: Sized {
    fn GetParameters(&self, pdwflags: *mut u32, pdwqueue: *mut u32) -> windows_core::Result<()>;
    fn Invoke(&self, pasyncresult: Option<&IRtwqAsyncResult>) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IRtwqAsyncCallback {}
impl IRtwqAsyncCallback_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IRtwqAsyncCallback_Impl, const OFFSET: isize>() -> IRtwqAsyncCallback_Vtbl {
        unsafe extern "system" fn GetParameters<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IRtwqAsyncCallback_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwflags: *mut u32, pdwqueue: *mut u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IRtwqAsyncCallback_Impl::GetParameters(this, core::mem::transmute_copy(&pdwflags), core::mem::transmute_copy(&pdwqueue)).into()
        }
        unsafe extern "system" fn Invoke<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IRtwqAsyncCallback_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pasyncresult: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IRtwqAsyncCallback_Impl::Invoke(this, windows_core::from_raw_borrowed(&pasyncresult)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetParameters: GetParameters::<Identity, Impl, OFFSET>,
            Invoke: Invoke::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IRtwqAsyncCallback as windows_core::Interface>::IID
    }
}
pub trait IRtwqAsyncResult_Impl: Sized {
    fn GetState(&self) -> windows_core::Result<windows_core::IUnknown>;
    fn GetStatus(&self) -> windows_core::Result<()>;
    fn SetStatus(&self, hrstatus: windows_core::HRESULT) -> windows_core::Result<()>;
    fn GetObject(&self) -> windows_core::Result<windows_core::IUnknown>;
    fn GetStateNoAddRef(&self) -> Option<windows_core::IUnknown>;
}
impl windows_core::RuntimeName for IRtwqAsyncResult {}
impl IRtwqAsyncResult_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IRtwqAsyncResult_Impl, const OFFSET: isize>() -> IRtwqAsyncResult_Vtbl {
        unsafe extern "system" fn GetState<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IRtwqAsyncResult_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppunkstate: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IRtwqAsyncResult_Impl::GetState(this) {
                Ok(ok__) => {
                    core::ptr::write(ppunkstate, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetStatus<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IRtwqAsyncResult_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IRtwqAsyncResult_Impl::GetStatus(this).into()
        }
        unsafe extern "system" fn SetStatus<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IRtwqAsyncResult_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hrstatus: windows_core::HRESULT) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IRtwqAsyncResult_Impl::SetStatus(this, core::mem::transmute_copy(&hrstatus)).into()
        }
        unsafe extern "system" fn GetObject<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IRtwqAsyncResult_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppobject: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IRtwqAsyncResult_Impl::GetObject(this) {
                Ok(ok__) => {
                    core::ptr::write(ppobject, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetStateNoAddRef<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IRtwqAsyncResult_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> Option<windows_core::IUnknown> {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IRtwqAsyncResult_Impl::GetStateNoAddRef(this)
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetState: GetState::<Identity, Impl, OFFSET>,
            GetStatus: GetStatus::<Identity, Impl, OFFSET>,
            SetStatus: SetStatus::<Identity, Impl, OFFSET>,
            GetObject: GetObject::<Identity, Impl, OFFSET>,
            GetStateNoAddRef: GetStateNoAddRef::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IRtwqAsyncResult as windows_core::Interface>::IID
    }
}
pub trait IRtwqPlatformEvents_Impl: Sized {
    fn InitializationComplete(&self) -> windows_core::Result<()>;
    fn ShutdownStart(&self) -> windows_core::Result<()>;
    fn ShutdownComplete(&self) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IRtwqPlatformEvents {}
impl IRtwqPlatformEvents_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IRtwqPlatformEvents_Impl, const OFFSET: isize>() -> IRtwqPlatformEvents_Vtbl {
        unsafe extern "system" fn InitializationComplete<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IRtwqPlatformEvents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IRtwqPlatformEvents_Impl::InitializationComplete(this).into()
        }
        unsafe extern "system" fn ShutdownStart<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IRtwqPlatformEvents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IRtwqPlatformEvents_Impl::ShutdownStart(this).into()
        }
        unsafe extern "system" fn ShutdownComplete<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IRtwqPlatformEvents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IRtwqPlatformEvents_Impl::ShutdownComplete(this).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            InitializationComplete: InitializationComplete::<Identity, Impl, OFFSET>,
            ShutdownStart: ShutdownStart::<Identity, Impl, OFFSET>,
            ShutdownComplete: ShutdownComplete::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IRtwqPlatformEvents as windows_core::Interface>::IID
    }
}
pub trait RTWQASYNCRESULT_Impl: Sized + IRtwqAsyncResult_Impl {}
impl windows_core::RuntimeName for RTWQASYNCRESULT {}
impl RTWQASYNCRESULT_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: RTWQASYNCRESULT_Impl, const OFFSET: isize>() -> RTWQASYNCRESULT_Vtbl {
        Self { base__: IRtwqAsyncResult_Vtbl::new::<Identity, Impl, OFFSET>() }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<RTWQASYNCRESULT as windows_core::Interface>::IID || iid == &<IRtwqAsyncResult as windows_core::Interface>::IID
    }
}
