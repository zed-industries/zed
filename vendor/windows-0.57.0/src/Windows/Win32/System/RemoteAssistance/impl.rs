#[cfg(feature = "Win32_System_Com")]
pub trait DRendezvousSessionEvents_Impl: Sized + super::Com::IDispatch_Impl {}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for DRendezvousSessionEvents {}
#[cfg(feature = "Win32_System_Com")]
impl DRendezvousSessionEvents_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: DRendezvousSessionEvents_Impl, const OFFSET: isize>() -> DRendezvousSessionEvents_Vtbl {
        Self { base__: super::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>() }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<DRendezvousSessionEvents as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID
    }
}
pub trait IRendezvousApplication_Impl: Sized {
    fn SetRendezvousSession(&self, prendezvoussession: Option<&windows_core::IUnknown>) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IRendezvousApplication {}
impl IRendezvousApplication_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IRendezvousApplication_Impl, const OFFSET: isize>() -> IRendezvousApplication_Vtbl {
        unsafe extern "system" fn SetRendezvousSession<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IRendezvousApplication_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, prendezvoussession: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IRendezvousApplication_Impl::SetRendezvousSession(this, windows_core::from_raw_borrowed(&prendezvoussession)).into()
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), SetRendezvousSession: SetRendezvousSession::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IRendezvousApplication as windows_core::Interface>::IID
    }
}
pub trait IRendezvousSession_Impl: Sized {
    fn State(&self) -> windows_core::Result<RENDEZVOUS_SESSION_STATE>;
    fn RemoteUser(&self) -> windows_core::Result<windows_core::BSTR>;
    fn Flags(&self) -> windows_core::Result<i32>;
    fn SendContextData(&self, bstrdata: &windows_core::BSTR) -> windows_core::Result<()>;
    fn Terminate(&self, hr: windows_core::HRESULT, bstrappdata: &windows_core::BSTR) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IRendezvousSession {}
impl IRendezvousSession_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IRendezvousSession_Impl, const OFFSET: isize>() -> IRendezvousSession_Vtbl {
        unsafe extern "system" fn State<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IRendezvousSession_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, psessionstate: *mut RENDEZVOUS_SESSION_STATE) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IRendezvousSession_Impl::State(this) {
                Ok(ok__) => {
                    core::ptr::write(psessionstate, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoteUser<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IRendezvousSession_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrusername: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IRendezvousSession_Impl::RemoteUser(this) {
                Ok(ok__) => {
                    core::ptr::write(bstrusername, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Flags<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IRendezvousSession_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pflags: *mut i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IRendezvousSession_Impl::Flags(this) {
                Ok(ok__) => {
                    core::ptr::write(pflags, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SendContextData<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IRendezvousSession_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrdata: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IRendezvousSession_Impl::SendContextData(this, core::mem::transmute(&bstrdata)).into()
        }
        unsafe extern "system" fn Terminate<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IRendezvousSession_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hr: windows_core::HRESULT, bstrappdata: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IRendezvousSession_Impl::Terminate(this, core::mem::transmute_copy(&hr), core::mem::transmute(&bstrappdata)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            State: State::<Identity, Impl, OFFSET>,
            RemoteUser: RemoteUser::<Identity, Impl, OFFSET>,
            Flags: Flags::<Identity, Impl, OFFSET>,
            SendContextData: SendContextData::<Identity, Impl, OFFSET>,
            Terminate: Terminate::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IRendezvousSession as windows_core::Interface>::IID
    }
}
