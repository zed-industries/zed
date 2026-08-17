#[cfg(feature = "Win32_System_Com")]
pub trait IEnumACDGroup_Impl: Sized {
    fn Next(&self, celt: u32, ppelements: *mut Option<ITACDGroup>, pceltfetched: *mut u32) -> windows_core::Result<()>;
    fn Reset(&self) -> windows_core::Result<()>;
    fn Skip(&self, celt: u32) -> windows_core::Result<()>;
    fn Clone(&self) -> windows_core::Result<IEnumACDGroup>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IEnumACDGroup {}
#[cfg(feature = "Win32_System_Com")]
impl IEnumACDGroup_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IEnumACDGroup_Impl, const OFFSET: isize>() -> IEnumACDGroup_Vtbl {
        unsafe extern "system" fn Next<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IEnumACDGroup_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, celt: u32, ppelements: *mut *mut core::ffi::c_void, pceltfetched: *mut u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IEnumACDGroup_Impl::Next(this, core::mem::transmute_copy(&celt), core::mem::transmute_copy(&ppelements), core::mem::transmute_copy(&pceltfetched)).into()
        }
        unsafe extern "system" fn Reset<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IEnumACDGroup_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IEnumACDGroup_Impl::Reset(this).into()
        }
        unsafe extern "system" fn Skip<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IEnumACDGroup_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, celt: u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IEnumACDGroup_Impl::Skip(this, core::mem::transmute_copy(&celt)).into()
        }
        unsafe extern "system" fn Clone<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IEnumACDGroup_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppenum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IEnumACDGroup_Impl::Clone(this) {
                Ok(ok__) => {
                    core::ptr::write(ppenum, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Next: Next::<Identity, Impl, OFFSET>,
            Reset: Reset::<Identity, Impl, OFFSET>,
            Skip: Skip::<Identity, Impl, OFFSET>,
            Clone: Clone::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IEnumACDGroup as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IEnumAddress_Impl: Sized {
    fn Next(&self, celt: u32, ppelements: *mut Option<ITAddress>, pceltfetched: *mut u32) -> windows_core::Result<()>;
    fn Reset(&self) -> windows_core::Result<()>;
    fn Skip(&self, celt: u32) -> windows_core::Result<()>;
    fn Clone(&self) -> windows_core::Result<IEnumAddress>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IEnumAddress {}
#[cfg(feature = "Win32_System_Com")]
impl IEnumAddress_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IEnumAddress_Impl, const OFFSET: isize>() -> IEnumAddress_Vtbl {
        unsafe extern "system" fn Next<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IEnumAddress_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, celt: u32, ppelements: *mut *mut core::ffi::c_void, pceltfetched: *mut u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IEnumAddress_Impl::Next(this, core::mem::transmute_copy(&celt), core::mem::transmute_copy(&ppelements), core::mem::transmute_copy(&pceltfetched)).into()
        }
        unsafe extern "system" fn Reset<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IEnumAddress_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IEnumAddress_Impl::Reset(this).into()
        }
        unsafe extern "system" fn Skip<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IEnumAddress_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, celt: u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IEnumAddress_Impl::Skip(this, core::mem::transmute_copy(&celt)).into()
        }
        unsafe extern "system" fn Clone<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IEnumAddress_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppenum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IEnumAddress_Impl::Clone(this) {
                Ok(ok__) => {
                    core::ptr::write(ppenum, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Next: Next::<Identity, Impl, OFFSET>,
            Reset: Reset::<Identity, Impl, OFFSET>,
            Skip: Skip::<Identity, Impl, OFFSET>,
            Clone: Clone::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IEnumAddress as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IEnumAgent_Impl: Sized {
    fn Next(&self, celt: u32, ppelements: *mut Option<ITAgent>, pceltfetched: *mut u32) -> windows_core::Result<()>;
    fn Reset(&self) -> windows_core::Result<()>;
    fn Skip(&self, celt: u32) -> windows_core::Result<()>;
    fn Clone(&self) -> windows_core::Result<IEnumAgent>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IEnumAgent {}
#[cfg(feature = "Win32_System_Com")]
impl IEnumAgent_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IEnumAgent_Impl, const OFFSET: isize>() -> IEnumAgent_Vtbl {
        unsafe extern "system" fn Next<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IEnumAgent_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, celt: u32, ppelements: *mut *mut core::ffi::c_void, pceltfetched: *mut u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IEnumAgent_Impl::Next(this, core::mem::transmute_copy(&celt), core::mem::transmute_copy(&ppelements), core::mem::transmute_copy(&pceltfetched)).into()
        }
        unsafe extern "system" fn Reset<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IEnumAgent_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IEnumAgent_Impl::Reset(this).into()
        }
        unsafe extern "system" fn Skip<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IEnumAgent_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, celt: u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IEnumAgent_Impl::Skip(this, core::mem::transmute_copy(&celt)).into()
        }
        unsafe extern "system" fn Clone<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IEnumAgent_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppenum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IEnumAgent_Impl::Clone(this) {
                Ok(ok__) => {
                    core::ptr::write(ppenum, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Next: Next::<Identity, Impl, OFFSET>,
            Reset: Reset::<Identity, Impl, OFFSET>,
            Skip: Skip::<Identity, Impl, OFFSET>,
            Clone: Clone::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IEnumAgent as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IEnumAgentHandler_Impl: Sized {
    fn Next(&self, celt: u32, ppelements: *mut Option<ITAgentHandler>, pceltfetched: *mut u32) -> windows_core::Result<()>;
    fn Reset(&self) -> windows_core::Result<()>;
    fn Skip(&self, celt: u32) -> windows_core::Result<()>;
    fn Clone(&self) -> windows_core::Result<IEnumAgentHandler>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IEnumAgentHandler {}
#[cfg(feature = "Win32_System_Com")]
impl IEnumAgentHandler_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IEnumAgentHandler_Impl, const OFFSET: isize>() -> IEnumAgentHandler_Vtbl {
        unsafe extern "system" fn Next<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IEnumAgentHandler_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, celt: u32, ppelements: *mut *mut core::ffi::c_void, pceltfetched: *mut u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IEnumAgentHandler_Impl::Next(this, core::mem::transmute_copy(&celt), core::mem::transmute_copy(&ppelements), core::mem::transmute_copy(&pceltfetched)).into()
        }
        unsafe extern "system" fn Reset<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IEnumAgentHandler_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IEnumAgentHandler_Impl::Reset(this).into()
        }
        unsafe extern "system" fn Skip<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IEnumAgentHandler_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, celt: u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IEnumAgentHandler_Impl::Skip(this, core::mem::transmute_copy(&celt)).into()
        }
        unsafe extern "system" fn Clone<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IEnumAgentHandler_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppenum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IEnumAgentHandler_Impl::Clone(this) {
                Ok(ok__) => {
                    core::ptr::write(ppenum, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Next: Next::<Identity, Impl, OFFSET>,
            Reset: Reset::<Identity, Impl, OFFSET>,
            Skip: Skip::<Identity, Impl, OFFSET>,
            Clone: Clone::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IEnumAgentHandler as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IEnumAgentSession_Impl: Sized {
    fn Next(&self, celt: u32, ppelements: *mut Option<ITAgentSession>, pceltfetched: *mut u32) -> windows_core::Result<()>;
    fn Reset(&self) -> windows_core::Result<()>;
    fn Skip(&self, celt: u32) -> windows_core::Result<()>;
    fn Clone(&self) -> windows_core::Result<IEnumAgentSession>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IEnumAgentSession {}
#[cfg(feature = "Win32_System_Com")]
impl IEnumAgentSession_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IEnumAgentSession_Impl, const OFFSET: isize>() -> IEnumAgentSession_Vtbl {
        unsafe extern "system" fn Next<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IEnumAgentSession_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, celt: u32, ppelements: *mut *mut core::ffi::c_void, pceltfetched: *mut u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IEnumAgentSession_Impl::Next(this, core::mem::transmute_copy(&celt), core::mem::transmute_copy(&ppelements), core::mem::transmute_copy(&pceltfetched)).into()
        }
        unsafe extern "system" fn Reset<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IEnumAgentSession_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IEnumAgentSession_Impl::Reset(this).into()
        }
        unsafe extern "system" fn Skip<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IEnumAgentSession_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, celt: u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IEnumAgentSession_Impl::Skip(this, core::mem::transmute_copy(&celt)).into()
        }
        unsafe extern "system" fn Clone<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IEnumAgentSession_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppenum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IEnumAgentSession_Impl::Clone(this) {
                Ok(ok__) => {
                    core::ptr::write(ppenum, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Next: Next::<Identity, Impl, OFFSET>,
            Reset: Reset::<Identity, Impl, OFFSET>,
            Skip: Skip::<Identity, Impl, OFFSET>,
            Clone: Clone::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IEnumAgentSession as windows_core::Interface>::IID
    }
}
pub trait IEnumBstr_Impl: Sized {
    fn Next(&self, celt: u32, ppstrings: *mut windows_core::BSTR, pceltfetched: *mut u32) -> windows_core::Result<()>;
    fn Reset(&self) -> windows_core::Result<()>;
    fn Skip(&self, celt: u32) -> windows_core::Result<()>;
    fn Clone(&self) -> windows_core::Result<IEnumBstr>;
}
impl windows_core::RuntimeName for IEnumBstr {}
impl IEnumBstr_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IEnumBstr_Impl, const OFFSET: isize>() -> IEnumBstr_Vtbl {
        unsafe extern "system" fn Next<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IEnumBstr_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, celt: u32, ppstrings: *mut core::mem::MaybeUninit<windows_core::BSTR>, pceltfetched: *mut u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IEnumBstr_Impl::Next(this, core::mem::transmute_copy(&celt), core::mem::transmute_copy(&ppstrings), core::mem::transmute_copy(&pceltfetched)).into()
        }
        unsafe extern "system" fn Reset<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IEnumBstr_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IEnumBstr_Impl::Reset(this).into()
        }
        unsafe extern "system" fn Skip<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IEnumBstr_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, celt: u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IEnumBstr_Impl::Skip(this, core::mem::transmute_copy(&celt)).into()
        }
        unsafe extern "system" fn Clone<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IEnumBstr_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppenum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IEnumBstr_Impl::Clone(this) {
                Ok(ok__) => {
                    core::ptr::write(ppenum, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Next: Next::<Identity, Impl, OFFSET>,
            Reset: Reset::<Identity, Impl, OFFSET>,
            Skip: Skip::<Identity, Impl, OFFSET>,
            Clone: Clone::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IEnumBstr as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IEnumCall_Impl: Sized {
    fn Next(&self, celt: u32, ppelements: *mut Option<ITCallInfo>, pceltfetched: *mut u32) -> windows_core::Result<()>;
    fn Reset(&self) -> windows_core::Result<()>;
    fn Skip(&self, celt: u32) -> windows_core::Result<()>;
    fn Clone(&self) -> windows_core::Result<IEnumCall>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IEnumCall {}
#[cfg(feature = "Win32_System_Com")]
impl IEnumCall_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IEnumCall_Impl, const OFFSET: isize>() -> IEnumCall_Vtbl {
        unsafe extern "system" fn Next<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IEnumCall_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, celt: u32, ppelements: *mut *mut core::ffi::c_void, pceltfetched: *mut u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IEnumCall_Impl::Next(this, core::mem::transmute_copy(&celt), core::mem::transmute_copy(&ppelements), core::mem::transmute_copy(&pceltfetched)).into()
        }
        unsafe extern "system" fn Reset<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IEnumCall_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IEnumCall_Impl::Reset(this).into()
        }
        unsafe extern "system" fn Skip<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IEnumCall_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, celt: u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IEnumCall_Impl::Skip(this, core::mem::transmute_copy(&celt)).into()
        }
        unsafe extern "system" fn Clone<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IEnumCall_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppenum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IEnumCall_Impl::Clone(this) {
                Ok(ok__) => {
                    core::ptr::write(ppenum, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Next: Next::<Identity, Impl, OFFSET>,
            Reset: Reset::<Identity, Impl, OFFSET>,
            Skip: Skip::<Identity, Impl, OFFSET>,
            Clone: Clone::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IEnumCall as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IEnumCallHub_Impl: Sized {
    fn Next(&self, celt: u32, ppelements: *mut Option<ITCallHub>, pceltfetched: *mut u32) -> windows_core::Result<()>;
    fn Reset(&self) -> windows_core::Result<()>;
    fn Skip(&self, celt: u32) -> windows_core::Result<()>;
    fn Clone(&self) -> windows_core::Result<IEnumCallHub>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IEnumCallHub {}
#[cfg(feature = "Win32_System_Com")]
impl IEnumCallHub_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IEnumCallHub_Impl, const OFFSET: isize>() -> IEnumCallHub_Vtbl {
        unsafe extern "system" fn Next<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IEnumCallHub_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, celt: u32, ppelements: *mut *mut core::ffi::c_void, pceltfetched: *mut u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IEnumCallHub_Impl::Next(this, core::mem::transmute_copy(&celt), core::mem::transmute_copy(&ppelements), core::mem::transmute_copy(&pceltfetched)).into()
        }
        unsafe extern "system" fn Reset<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IEnumCallHub_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IEnumCallHub_Impl::Reset(this).into()
        }
        unsafe extern "system" fn Skip<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IEnumCallHub_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, celt: u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IEnumCallHub_Impl::Skip(this, core::mem::transmute_copy(&celt)).into()
        }
        unsafe extern "system" fn Clone<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IEnumCallHub_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppenum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IEnumCallHub_Impl::Clone(this) {
                Ok(ok__) => {
                    core::ptr::write(ppenum, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Next: Next::<Identity, Impl, OFFSET>,
            Reset: Reset::<Identity, Impl, OFFSET>,
            Skip: Skip::<Identity, Impl, OFFSET>,
            Clone: Clone::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IEnumCallHub as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IEnumCallingCard_Impl: Sized {
    fn Next(&self, celt: u32, ppelements: *mut Option<ITCallingCard>, pceltfetched: *mut u32) -> windows_core::Result<()>;
    fn Reset(&self) -> windows_core::Result<()>;
    fn Skip(&self, celt: u32) -> windows_core::Result<()>;
    fn Clone(&self) -> windows_core::Result<IEnumCallingCard>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IEnumCallingCard {}
#[cfg(feature = "Win32_System_Com")]
impl IEnumCallingCard_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IEnumCallingCard_Impl, const OFFSET: isize>() -> IEnumCallingCard_Vtbl {
        unsafe extern "system" fn Next<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IEnumCallingCard_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, celt: u32, ppelements: *mut *mut core::ffi::c_void, pceltfetched: *mut u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IEnumCallingCard_Impl::Next(this, core::mem::transmute_copy(&celt), core::mem::transmute_copy(&ppelements), core::mem::transmute_copy(&pceltfetched)).into()
        }
        unsafe extern "system" fn Reset<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IEnumCallingCard_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IEnumCallingCard_Impl::Reset(this).into()
        }
        unsafe extern "system" fn Skip<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IEnumCallingCard_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, celt: u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IEnumCallingCard_Impl::Skip(this, core::mem::transmute_copy(&celt)).into()
        }
        unsafe extern "system" fn Clone<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IEnumCallingCard_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppenum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IEnumCallingCard_Impl::Clone(this) {
                Ok(ok__) => {
                    core::ptr::write(ppenum, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Next: Next::<Identity, Impl, OFFSET>,
            Reset: Reset::<Identity, Impl, OFFSET>,
            Skip: Skip::<Identity, Impl, OFFSET>,
            Clone: Clone::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IEnumCallingCard as windows_core::Interface>::IID
    }
}
pub trait IEnumDialableAddrs_Impl: Sized {
    fn Next(&self, celt: u32, ppelements: *mut windows_core::BSTR, pcfetched: *mut u32) -> windows_core::Result<()>;
    fn Reset(&self) -> windows_core::Result<()>;
    fn Skip(&self, celt: u32) -> windows_core::Result<()>;
    fn Clone(&self) -> windows_core::Result<IEnumDialableAddrs>;
}
impl windows_core::RuntimeName for IEnumDialableAddrs {}
impl IEnumDialableAddrs_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IEnumDialableAddrs_Impl, const OFFSET: isize>() -> IEnumDialableAddrs_Vtbl {
        unsafe extern "system" fn Next<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IEnumDialableAddrs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, celt: u32, ppelements: *mut core::mem::MaybeUninit<windows_core::BSTR>, pcfetched: *mut u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IEnumDialableAddrs_Impl::Next(this, core::mem::transmute_copy(&celt), core::mem::transmute_copy(&ppelements), core::mem::transmute_copy(&pcfetched)).into()
        }
        unsafe extern "system" fn Reset<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IEnumDialableAddrs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IEnumDialableAddrs_Impl::Reset(this).into()
        }
        unsafe extern "system" fn Skip<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IEnumDialableAddrs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, celt: u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IEnumDialableAddrs_Impl::Skip(this, core::mem::transmute_copy(&celt)).into()
        }
        unsafe extern "system" fn Clone<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IEnumDialableAddrs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppenum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IEnumDialableAddrs_Impl::Clone(this) {
                Ok(ok__) => {
                    core::ptr::write(ppenum, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Next: Next::<Identity, Impl, OFFSET>,
            Reset: Reset::<Identity, Impl, OFFSET>,
            Skip: Skip::<Identity, Impl, OFFSET>,
            Clone: Clone::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IEnumDialableAddrs as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IEnumDirectory_Impl: Sized {
    fn Next(&self, celt: u32, ppelements: *mut Option<ITDirectory>, pcfetched: *mut u32) -> windows_core::Result<()>;
    fn Reset(&self) -> windows_core::Result<()>;
    fn Skip(&self, celt: u32) -> windows_core::Result<()>;
    fn Clone(&self) -> windows_core::Result<IEnumDirectory>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IEnumDirectory {}
#[cfg(feature = "Win32_System_Com")]
impl IEnumDirectory_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IEnumDirectory_Impl, const OFFSET: isize>() -> IEnumDirectory_Vtbl {
        unsafe extern "system" fn Next<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IEnumDirectory_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, celt: u32, ppelements: *mut *mut core::ffi::c_void, pcfetched: *mut u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IEnumDirectory_Impl::Next(this, core::mem::transmute_copy(&celt), core::mem::transmute_copy(&ppelements), core::mem::transmute_copy(&pcfetched)).into()
        }
        unsafe extern "system" fn Reset<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IEnumDirectory_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IEnumDirectory_Impl::Reset(this).into()
        }
        unsafe extern "system" fn Skip<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IEnumDirectory_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, celt: u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IEnumDirectory_Impl::Skip(this, core::mem::transmute_copy(&celt)).into()
        }
        unsafe extern "system" fn Clone<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IEnumDirectory_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppenum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IEnumDirectory_Impl::Clone(this) {
                Ok(ok__) => {
                    core::ptr::write(ppenum, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Next: Next::<Identity, Impl, OFFSET>,
            Reset: Reset::<Identity, Impl, OFFSET>,
            Skip: Skip::<Identity, Impl, OFFSET>,
            Clone: Clone::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IEnumDirectory as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IEnumDirectoryObject_Impl: Sized {
    fn Next(&self, celt: u32, pval: *mut Option<ITDirectoryObject>, pcfetched: *mut u32) -> windows_core::Result<()>;
    fn Reset(&self) -> windows_core::Result<()>;
    fn Skip(&self, celt: u32) -> windows_core::Result<()>;
    fn Clone(&self) -> windows_core::Result<IEnumDirectoryObject>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IEnumDirectoryObject {}
#[cfg(feature = "Win32_System_Com")]
impl IEnumDirectoryObject_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IEnumDirectoryObject_Impl, const OFFSET: isize>() -> IEnumDirectoryObject_Vtbl {
        unsafe extern "system" fn Next<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IEnumDirectoryObject_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, celt: u32, pval: *mut *mut core::ffi::c_void, pcfetched: *mut u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IEnumDirectoryObject_Impl::Next(this, core::mem::transmute_copy(&celt), core::mem::transmute_copy(&pval), core::mem::transmute_copy(&pcfetched)).into()
        }
        unsafe extern "system" fn Reset<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IEnumDirectoryObject_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IEnumDirectoryObject_Impl::Reset(this).into()
        }
        unsafe extern "system" fn Skip<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IEnumDirectoryObject_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, celt: u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IEnumDirectoryObject_Impl::Skip(this, core::mem::transmute_copy(&celt)).into()
        }
        unsafe extern "system" fn Clone<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IEnumDirectoryObject_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppenum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IEnumDirectoryObject_Impl::Clone(this) {
                Ok(ok__) => {
                    core::ptr::write(ppenum, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Next: Next::<Identity, Impl, OFFSET>,
            Reset: Reset::<Identity, Impl, OFFSET>,
            Skip: Skip::<Identity, Impl, OFFSET>,
            Clone: Clone::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IEnumDirectoryObject as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IEnumLocation_Impl: Sized {
    fn Next(&self, celt: u32, ppelements: *mut Option<ITLocationInfo>, pceltfetched: *mut u32) -> windows_core::Result<()>;
    fn Reset(&self) -> windows_core::Result<()>;
    fn Skip(&self, celt: u32) -> windows_core::Result<()>;
    fn Clone(&self) -> windows_core::Result<IEnumLocation>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IEnumLocation {}
#[cfg(feature = "Win32_System_Com")]
impl IEnumLocation_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IEnumLocation_Impl, const OFFSET: isize>() -> IEnumLocation_Vtbl {
        unsafe extern "system" fn Next<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IEnumLocation_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, celt: u32, ppelements: *mut *mut core::ffi::c_void, pceltfetched: *mut u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IEnumLocation_Impl::Next(this, core::mem::transmute_copy(&celt), core::mem::transmute_copy(&ppelements), core::mem::transmute_copy(&pceltfetched)).into()
        }
        unsafe extern "system" fn Reset<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IEnumLocation_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IEnumLocation_Impl::Reset(this).into()
        }
        unsafe extern "system" fn Skip<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IEnumLocation_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, celt: u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IEnumLocation_Impl::Skip(this, core::mem::transmute_copy(&celt)).into()
        }
        unsafe extern "system" fn Clone<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IEnumLocation_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppenum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IEnumLocation_Impl::Clone(this) {
                Ok(ok__) => {
                    core::ptr::write(ppenum, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Next: Next::<Identity, Impl, OFFSET>,
            Reset: Reset::<Identity, Impl, OFFSET>,
            Skip: Skip::<Identity, Impl, OFFSET>,
            Clone: Clone::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IEnumLocation as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IEnumMcastScope_Impl: Sized {
    fn Next(&self, celt: u32, ppscopes: *mut Option<IMcastScope>, pceltfetched: *mut u32) -> windows_core::Result<()>;
    fn Reset(&self) -> windows_core::Result<()>;
    fn Skip(&self, celt: u32) -> windows_core::Result<()>;
    fn Clone(&self) -> windows_core::Result<IEnumMcastScope>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IEnumMcastScope {}
#[cfg(feature = "Win32_System_Com")]
impl IEnumMcastScope_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IEnumMcastScope_Impl, const OFFSET: isize>() -> IEnumMcastScope_Vtbl {
        unsafe extern "system" fn Next<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IEnumMcastScope_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, celt: u32, ppscopes: *mut *mut core::ffi::c_void, pceltfetched: *mut u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IEnumMcastScope_Impl::Next(this, core::mem::transmute_copy(&celt), core::mem::transmute_copy(&ppscopes), core::mem::transmute_copy(&pceltfetched)).into()
        }
        unsafe extern "system" fn Reset<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IEnumMcastScope_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IEnumMcastScope_Impl::Reset(this).into()
        }
        unsafe extern "system" fn Skip<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IEnumMcastScope_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, celt: u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IEnumMcastScope_Impl::Skip(this, core::mem::transmute_copy(&celt)).into()
        }
        unsafe extern "system" fn Clone<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IEnumMcastScope_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppenum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IEnumMcastScope_Impl::Clone(this) {
                Ok(ok__) => {
                    core::ptr::write(ppenum, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Next: Next::<Identity, Impl, OFFSET>,
            Reset: Reset::<Identity, Impl, OFFSET>,
            Skip: Skip::<Identity, Impl, OFFSET>,
            Clone: Clone::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IEnumMcastScope as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IEnumPhone_Impl: Sized {
    fn Next(&self, celt: u32, ppelements: *mut Option<ITPhone>, pceltfetched: *mut u32) -> windows_core::Result<()>;
    fn Reset(&self) -> windows_core::Result<()>;
    fn Skip(&self, celt: u32) -> windows_core::Result<()>;
    fn Clone(&self) -> windows_core::Result<IEnumPhone>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IEnumPhone {}
#[cfg(feature = "Win32_System_Com")]
impl IEnumPhone_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IEnumPhone_Impl, const OFFSET: isize>() -> IEnumPhone_Vtbl {
        unsafe extern "system" fn Next<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IEnumPhone_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, celt: u32, ppelements: *mut *mut core::ffi::c_void, pceltfetched: *mut u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IEnumPhone_Impl::Next(this, core::mem::transmute_copy(&celt), core::mem::transmute_copy(&ppelements), core::mem::transmute_copy(&pceltfetched)).into()
        }
        unsafe extern "system" fn Reset<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IEnumPhone_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IEnumPhone_Impl::Reset(this).into()
        }
        unsafe extern "system" fn Skip<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IEnumPhone_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, celt: u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IEnumPhone_Impl::Skip(this, core::mem::transmute_copy(&celt)).into()
        }
        unsafe extern "system" fn Clone<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IEnumPhone_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppenum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IEnumPhone_Impl::Clone(this) {
                Ok(ok__) => {
                    core::ptr::write(ppenum, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Next: Next::<Identity, Impl, OFFSET>,
            Reset: Reset::<Identity, Impl, OFFSET>,
            Skip: Skip::<Identity, Impl, OFFSET>,
            Clone: Clone::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IEnumPhone as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IEnumPluggableSuperclassInfo_Impl: Sized {
    fn Next(&self, celt: u32, ppelements: *mut Option<ITPluggableTerminalSuperclassInfo>, pceltfetched: *mut u32) -> windows_core::Result<()>;
    fn Reset(&self) -> windows_core::Result<()>;
    fn Skip(&self, celt: u32) -> windows_core::Result<()>;
    fn Clone(&self) -> windows_core::Result<IEnumPluggableSuperclassInfo>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IEnumPluggableSuperclassInfo {}
#[cfg(feature = "Win32_System_Com")]
impl IEnumPluggableSuperclassInfo_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IEnumPluggableSuperclassInfo_Impl, const OFFSET: isize>() -> IEnumPluggableSuperclassInfo_Vtbl {
        unsafe extern "system" fn Next<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IEnumPluggableSuperclassInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, celt: u32, ppelements: *mut *mut core::ffi::c_void, pceltfetched: *mut u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IEnumPluggableSuperclassInfo_Impl::Next(this, core::mem::transmute_copy(&celt), core::mem::transmute_copy(&ppelements), core::mem::transmute_copy(&pceltfetched)).into()
        }
        unsafe extern "system" fn Reset<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IEnumPluggableSuperclassInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IEnumPluggableSuperclassInfo_Impl::Reset(this).into()
        }
        unsafe extern "system" fn Skip<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IEnumPluggableSuperclassInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, celt: u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IEnumPluggableSuperclassInfo_Impl::Skip(this, core::mem::transmute_copy(&celt)).into()
        }
        unsafe extern "system" fn Clone<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IEnumPluggableSuperclassInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppenum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IEnumPluggableSuperclassInfo_Impl::Clone(this) {
                Ok(ok__) => {
                    core::ptr::write(ppenum, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Next: Next::<Identity, Impl, OFFSET>,
            Reset: Reset::<Identity, Impl, OFFSET>,
            Skip: Skip::<Identity, Impl, OFFSET>,
            Clone: Clone::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IEnumPluggableSuperclassInfo as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IEnumPluggableTerminalClassInfo_Impl: Sized {
    fn Next(&self, celt: u32, ppelements: *mut Option<ITPluggableTerminalClassInfo>, pceltfetched: *mut u32) -> windows_core::Result<()>;
    fn Reset(&self) -> windows_core::Result<()>;
    fn Skip(&self, celt: u32) -> windows_core::Result<()>;
    fn Clone(&self) -> windows_core::Result<IEnumPluggableTerminalClassInfo>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IEnumPluggableTerminalClassInfo {}
#[cfg(feature = "Win32_System_Com")]
impl IEnumPluggableTerminalClassInfo_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IEnumPluggableTerminalClassInfo_Impl, const OFFSET: isize>() -> IEnumPluggableTerminalClassInfo_Vtbl {
        unsafe extern "system" fn Next<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IEnumPluggableTerminalClassInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, celt: u32, ppelements: *mut *mut core::ffi::c_void, pceltfetched: *mut u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IEnumPluggableTerminalClassInfo_Impl::Next(this, core::mem::transmute_copy(&celt), core::mem::transmute_copy(&ppelements), core::mem::transmute_copy(&pceltfetched)).into()
        }
        unsafe extern "system" fn Reset<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IEnumPluggableTerminalClassInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IEnumPluggableTerminalClassInfo_Impl::Reset(this).into()
        }
        unsafe extern "system" fn Skip<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IEnumPluggableTerminalClassInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, celt: u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IEnumPluggableTerminalClassInfo_Impl::Skip(this, core::mem::transmute_copy(&celt)).into()
        }
        unsafe extern "system" fn Clone<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IEnumPluggableTerminalClassInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppenum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IEnumPluggableTerminalClassInfo_Impl::Clone(this) {
                Ok(ok__) => {
                    core::ptr::write(ppenum, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Next: Next::<Identity, Impl, OFFSET>,
            Reset: Reset::<Identity, Impl, OFFSET>,
            Skip: Skip::<Identity, Impl, OFFSET>,
            Clone: Clone::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IEnumPluggableTerminalClassInfo as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IEnumQueue_Impl: Sized {
    fn Next(&self, celt: u32, ppelements: *mut Option<ITQueue>, pceltfetched: *mut u32) -> windows_core::Result<()>;
    fn Reset(&self) -> windows_core::Result<()>;
    fn Skip(&self, celt: u32) -> windows_core::Result<()>;
    fn Clone(&self) -> windows_core::Result<IEnumQueue>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IEnumQueue {}
#[cfg(feature = "Win32_System_Com")]
impl IEnumQueue_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IEnumQueue_Impl, const OFFSET: isize>() -> IEnumQueue_Vtbl {
        unsafe extern "system" fn Next<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IEnumQueue_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, celt: u32, ppelements: *mut *mut core::ffi::c_void, pceltfetched: *mut u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IEnumQueue_Impl::Next(this, core::mem::transmute_copy(&celt), core::mem::transmute_copy(&ppelements), core::mem::transmute_copy(&pceltfetched)).into()
        }
        unsafe extern "system" fn Reset<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IEnumQueue_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IEnumQueue_Impl::Reset(this).into()
        }
        unsafe extern "system" fn Skip<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IEnumQueue_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, celt: u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IEnumQueue_Impl::Skip(this, core::mem::transmute_copy(&celt)).into()
        }
        unsafe extern "system" fn Clone<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IEnumQueue_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppenum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IEnumQueue_Impl::Clone(this) {
                Ok(ok__) => {
                    core::ptr::write(ppenum, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Next: Next::<Identity, Impl, OFFSET>,
            Reset: Reset::<Identity, Impl, OFFSET>,
            Skip: Skip::<Identity, Impl, OFFSET>,
            Clone: Clone::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IEnumQueue as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IEnumStream_Impl: Sized {
    fn Next(&self, celt: u32, ppelements: *mut Option<ITStream>, pceltfetched: *mut u32) -> windows_core::Result<()>;
    fn Reset(&self) -> windows_core::Result<()>;
    fn Skip(&self, celt: u32) -> windows_core::Result<()>;
    fn Clone(&self) -> windows_core::Result<IEnumStream>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IEnumStream {}
#[cfg(feature = "Win32_System_Com")]
impl IEnumStream_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IEnumStream_Impl, const OFFSET: isize>() -> IEnumStream_Vtbl {
        unsafe extern "system" fn Next<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IEnumStream_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, celt: u32, ppelements: *mut *mut core::ffi::c_void, pceltfetched: *mut u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IEnumStream_Impl::Next(this, core::mem::transmute_copy(&celt), core::mem::transmute_copy(&ppelements), core::mem::transmute_copy(&pceltfetched)).into()
        }
        unsafe extern "system" fn Reset<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IEnumStream_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IEnumStream_Impl::Reset(this).into()
        }
        unsafe extern "system" fn Skip<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IEnumStream_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, celt: u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IEnumStream_Impl::Skip(this, core::mem::transmute_copy(&celt)).into()
        }
        unsafe extern "system" fn Clone<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IEnumStream_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppenum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IEnumStream_Impl::Clone(this) {
                Ok(ok__) => {
                    core::ptr::write(ppenum, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Next: Next::<Identity, Impl, OFFSET>,
            Reset: Reset::<Identity, Impl, OFFSET>,
            Skip: Skip::<Identity, Impl, OFFSET>,
            Clone: Clone::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IEnumStream as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IEnumSubStream_Impl: Sized {
    fn Next(&self, celt: u32, ppelements: *mut Option<ITSubStream>, pceltfetched: *mut u32) -> windows_core::Result<()>;
    fn Reset(&self) -> windows_core::Result<()>;
    fn Skip(&self, celt: u32) -> windows_core::Result<()>;
    fn Clone(&self) -> windows_core::Result<IEnumSubStream>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IEnumSubStream {}
#[cfg(feature = "Win32_System_Com")]
impl IEnumSubStream_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IEnumSubStream_Impl, const OFFSET: isize>() -> IEnumSubStream_Vtbl {
        unsafe extern "system" fn Next<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IEnumSubStream_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, celt: u32, ppelements: *mut *mut core::ffi::c_void, pceltfetched: *mut u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IEnumSubStream_Impl::Next(this, core::mem::transmute_copy(&celt), core::mem::transmute_copy(&ppelements), core::mem::transmute_copy(&pceltfetched)).into()
        }
        unsafe extern "system" fn Reset<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IEnumSubStream_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IEnumSubStream_Impl::Reset(this).into()
        }
        unsafe extern "system" fn Skip<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IEnumSubStream_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, celt: u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IEnumSubStream_Impl::Skip(this, core::mem::transmute_copy(&celt)).into()
        }
        unsafe extern "system" fn Clone<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IEnumSubStream_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppenum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IEnumSubStream_Impl::Clone(this) {
                Ok(ok__) => {
                    core::ptr::write(ppenum, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Next: Next::<Identity, Impl, OFFSET>,
            Reset: Reset::<Identity, Impl, OFFSET>,
            Skip: Skip::<Identity, Impl, OFFSET>,
            Clone: Clone::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IEnumSubStream as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IEnumTerminal_Impl: Sized {
    fn Next(&self, celt: u32, ppelements: *mut Option<ITTerminal>, pceltfetched: *mut u32) -> windows_core::Result<()>;
    fn Reset(&self) -> windows_core::Result<()>;
    fn Skip(&self, celt: u32) -> windows_core::Result<()>;
    fn Clone(&self) -> windows_core::Result<IEnumTerminal>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IEnumTerminal {}
#[cfg(feature = "Win32_System_Com")]
impl IEnumTerminal_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IEnumTerminal_Impl, const OFFSET: isize>() -> IEnumTerminal_Vtbl {
        unsafe extern "system" fn Next<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IEnumTerminal_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, celt: u32, ppelements: *mut *mut core::ffi::c_void, pceltfetched: *mut u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IEnumTerminal_Impl::Next(this, core::mem::transmute_copy(&celt), core::mem::transmute_copy(&ppelements), core::mem::transmute_copy(&pceltfetched)).into()
        }
        unsafe extern "system" fn Reset<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IEnumTerminal_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IEnumTerminal_Impl::Reset(this).into()
        }
        unsafe extern "system" fn Skip<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IEnumTerminal_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, celt: u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IEnumTerminal_Impl::Skip(this, core::mem::transmute_copy(&celt)).into()
        }
        unsafe extern "system" fn Clone<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IEnumTerminal_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppenum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IEnumTerminal_Impl::Clone(this) {
                Ok(ok__) => {
                    core::ptr::write(ppenum, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Next: Next::<Identity, Impl, OFFSET>,
            Reset: Reset::<Identity, Impl, OFFSET>,
            Skip: Skip::<Identity, Impl, OFFSET>,
            Clone: Clone::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IEnumTerminal as windows_core::Interface>::IID
    }
}
pub trait IEnumTerminalClass_Impl: Sized {
    fn Next(&self, celt: u32, pelements: *mut windows_core::GUID, pceltfetched: *mut u32) -> windows_core::Result<()>;
    fn Reset(&self) -> windows_core::Result<()>;
    fn Skip(&self, celt: u32) -> windows_core::Result<()>;
    fn Clone(&self) -> windows_core::Result<IEnumTerminalClass>;
}
impl windows_core::RuntimeName for IEnumTerminalClass {}
impl IEnumTerminalClass_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IEnumTerminalClass_Impl, const OFFSET: isize>() -> IEnumTerminalClass_Vtbl {
        unsafe extern "system" fn Next<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IEnumTerminalClass_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, celt: u32, pelements: *mut windows_core::GUID, pceltfetched: *mut u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IEnumTerminalClass_Impl::Next(this, core::mem::transmute_copy(&celt), core::mem::transmute_copy(&pelements), core::mem::transmute_copy(&pceltfetched)).into()
        }
        unsafe extern "system" fn Reset<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IEnumTerminalClass_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IEnumTerminalClass_Impl::Reset(this).into()
        }
        unsafe extern "system" fn Skip<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IEnumTerminalClass_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, celt: u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IEnumTerminalClass_Impl::Skip(this, core::mem::transmute_copy(&celt)).into()
        }
        unsafe extern "system" fn Clone<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IEnumTerminalClass_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppenum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IEnumTerminalClass_Impl::Clone(this) {
                Ok(ok__) => {
                    core::ptr::write(ppenum, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Next: Next::<Identity, Impl, OFFSET>,
            Reset: Reset::<Identity, Impl, OFFSET>,
            Skip: Skip::<Identity, Impl, OFFSET>,
            Clone: Clone::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IEnumTerminalClass as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IMcastAddressAllocation_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn Scopes(&self) -> windows_core::Result<windows_core::VARIANT>;
    fn EnumerateScopes(&self) -> windows_core::Result<IEnumMcastScope>;
    fn RequestAddress(&self, pscope: Option<&IMcastScope>, leasestarttime: f64, leasestoptime: f64, numaddresses: i32) -> windows_core::Result<IMcastLeaseInfo>;
    fn RenewAddress(&self, lreserved: i32, prenewrequest: Option<&IMcastLeaseInfo>) -> windows_core::Result<IMcastLeaseInfo>;
    fn ReleaseAddress(&self, preleaserequest: Option<&IMcastLeaseInfo>) -> windows_core::Result<()>;
    fn CreateLeaseInfo(&self, leasestarttime: f64, leasestoptime: f64, dwnumaddresses: u32, ppaddresses: *const windows_core::PCWSTR, prequestid: &windows_core::PCWSTR, pserveraddress: &windows_core::PCWSTR) -> windows_core::Result<IMcastLeaseInfo>;
    fn CreateLeaseInfoFromVariant(&self, leasestarttime: f64, leasestoptime: f64, vaddresses: &windows_core::VARIANT, prequestid: &windows_core::BSTR, pserveraddress: &windows_core::BSTR) -> windows_core::Result<IMcastLeaseInfo>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IMcastAddressAllocation {}
#[cfg(feature = "Win32_System_Com")]
impl IMcastAddressAllocation_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IMcastAddressAllocation_Impl, const OFFSET: isize>() -> IMcastAddressAllocation_Vtbl {
        unsafe extern "system" fn Scopes<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IMcastAddressAllocation_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvariant: *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IMcastAddressAllocation_Impl::Scopes(this) {
                Ok(ok__) => {
                    core::ptr::write(pvariant, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnumerateScopes<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IMcastAddressAllocation_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppenummcastscope: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IMcastAddressAllocation_Impl::EnumerateScopes(this) {
                Ok(ok__) => {
                    core::ptr::write(ppenummcastscope, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RequestAddress<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IMcastAddressAllocation_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pscope: *mut core::ffi::c_void, leasestarttime: f64, leasestoptime: f64, numaddresses: i32, ppleaseresponse: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IMcastAddressAllocation_Impl::RequestAddress(this, windows_core::from_raw_borrowed(&pscope), core::mem::transmute_copy(&leasestarttime), core::mem::transmute_copy(&leasestoptime), core::mem::transmute_copy(&numaddresses)) {
                Ok(ok__) => {
                    core::ptr::write(ppleaseresponse, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RenewAddress<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IMcastAddressAllocation_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lreserved: i32, prenewrequest: *mut core::ffi::c_void, pprenewresponse: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IMcastAddressAllocation_Impl::RenewAddress(this, core::mem::transmute_copy(&lreserved), windows_core::from_raw_borrowed(&prenewrequest)) {
                Ok(ok__) => {
                    core::ptr::write(pprenewresponse, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ReleaseAddress<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IMcastAddressAllocation_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, preleaserequest: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IMcastAddressAllocation_Impl::ReleaseAddress(this, windows_core::from_raw_borrowed(&preleaserequest)).into()
        }
        unsafe extern "system" fn CreateLeaseInfo<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IMcastAddressAllocation_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, leasestarttime: f64, leasestoptime: f64, dwnumaddresses: u32, ppaddresses: *const windows_core::PCWSTR, prequestid: windows_core::PCWSTR, pserveraddress: windows_core::PCWSTR, ppreleaserequest: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IMcastAddressAllocation_Impl::CreateLeaseInfo(this, core::mem::transmute_copy(&leasestarttime), core::mem::transmute_copy(&leasestoptime), core::mem::transmute_copy(&dwnumaddresses), core::mem::transmute_copy(&ppaddresses), core::mem::transmute(&prequestid), core::mem::transmute(&pserveraddress)) {
                Ok(ok__) => {
                    core::ptr::write(ppreleaserequest, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateLeaseInfoFromVariant<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IMcastAddressAllocation_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, leasestarttime: f64, leasestoptime: f64, vaddresses: core::mem::MaybeUninit<windows_core::VARIANT>, prequestid: core::mem::MaybeUninit<windows_core::BSTR>, pserveraddress: core::mem::MaybeUninit<windows_core::BSTR>, ppreleaserequest: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IMcastAddressAllocation_Impl::CreateLeaseInfoFromVariant(this, core::mem::transmute_copy(&leasestarttime), core::mem::transmute_copy(&leasestoptime), core::mem::transmute(&vaddresses), core::mem::transmute(&prequestid), core::mem::transmute(&pserveraddress)) {
                Ok(ok__) => {
                    core::ptr::write(ppreleaserequest, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            Scopes: Scopes::<Identity, Impl, OFFSET>,
            EnumerateScopes: EnumerateScopes::<Identity, Impl, OFFSET>,
            RequestAddress: RequestAddress::<Identity, Impl, OFFSET>,
            RenewAddress: RenewAddress::<Identity, Impl, OFFSET>,
            ReleaseAddress: ReleaseAddress::<Identity, Impl, OFFSET>,
            CreateLeaseInfo: CreateLeaseInfo::<Identity, Impl, OFFSET>,
            CreateLeaseInfoFromVariant: CreateLeaseInfoFromVariant::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMcastAddressAllocation as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IMcastLeaseInfo_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn RequestID(&self) -> windows_core::Result<windows_core::BSTR>;
    fn LeaseStartTime(&self) -> windows_core::Result<f64>;
    fn SetLeaseStartTime(&self, time: f64) -> windows_core::Result<()>;
    fn LeaseStopTime(&self) -> windows_core::Result<f64>;
    fn SetLeaseStopTime(&self, time: f64) -> windows_core::Result<()>;
    fn AddressCount(&self) -> windows_core::Result<i32>;
    fn ServerAddress(&self) -> windows_core::Result<windows_core::BSTR>;
    fn TTL(&self) -> windows_core::Result<i32>;
    fn Addresses(&self) -> windows_core::Result<windows_core::VARIANT>;
    fn EnumerateAddresses(&self) -> windows_core::Result<IEnumBstr>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IMcastLeaseInfo {}
#[cfg(feature = "Win32_System_Com")]
impl IMcastLeaseInfo_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IMcastLeaseInfo_Impl, const OFFSET: isize>() -> IMcastLeaseInfo_Vtbl {
        unsafe extern "system" fn RequestID<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IMcastLeaseInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pprequestid: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IMcastLeaseInfo_Impl::RequestID(this) {
                Ok(ok__) => {
                    core::ptr::write(pprequestid, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LeaseStartTime<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IMcastLeaseInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ptime: *mut f64) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IMcastLeaseInfo_Impl::LeaseStartTime(this) {
                Ok(ok__) => {
                    core::ptr::write(ptime, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetLeaseStartTime<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IMcastLeaseInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, time: f64) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IMcastLeaseInfo_Impl::SetLeaseStartTime(this, core::mem::transmute_copy(&time)).into()
        }
        unsafe extern "system" fn LeaseStopTime<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IMcastLeaseInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ptime: *mut f64) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IMcastLeaseInfo_Impl::LeaseStopTime(this) {
                Ok(ok__) => {
                    core::ptr::write(ptime, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetLeaseStopTime<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IMcastLeaseInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, time: f64) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IMcastLeaseInfo_Impl::SetLeaseStopTime(this, core::mem::transmute_copy(&time)).into()
        }
        unsafe extern "system" fn AddressCount<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IMcastLeaseInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcount: *mut i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IMcastLeaseInfo_Impl::AddressCount(this) {
                Ok(ok__) => {
                    core::ptr::write(pcount, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ServerAddress<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IMcastLeaseInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppaddress: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IMcastLeaseInfo_Impl::ServerAddress(this) {
                Ok(ok__) => {
                    core::ptr::write(ppaddress, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TTL<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IMcastLeaseInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pttl: *mut i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IMcastLeaseInfo_Impl::TTL(this) {
                Ok(ok__) => {
                    core::ptr::write(pttl, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Addresses<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IMcastLeaseInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvariant: *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IMcastLeaseInfo_Impl::Addresses(this) {
                Ok(ok__) => {
                    core::ptr::write(pvariant, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnumerateAddresses<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IMcastLeaseInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppenumaddresses: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IMcastLeaseInfo_Impl::EnumerateAddresses(this) {
                Ok(ok__) => {
                    core::ptr::write(ppenumaddresses, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            RequestID: RequestID::<Identity, Impl, OFFSET>,
            LeaseStartTime: LeaseStartTime::<Identity, Impl, OFFSET>,
            SetLeaseStartTime: SetLeaseStartTime::<Identity, Impl, OFFSET>,
            LeaseStopTime: LeaseStopTime::<Identity, Impl, OFFSET>,
            SetLeaseStopTime: SetLeaseStopTime::<Identity, Impl, OFFSET>,
            AddressCount: AddressCount::<Identity, Impl, OFFSET>,
            ServerAddress: ServerAddress::<Identity, Impl, OFFSET>,
            TTL: TTL::<Identity, Impl, OFFSET>,
            Addresses: Addresses::<Identity, Impl, OFFSET>,
            EnumerateAddresses: EnumerateAddresses::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMcastLeaseInfo as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IMcastScope_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn ScopeID(&self) -> windows_core::Result<i32>;
    fn ServerID(&self) -> windows_core::Result<i32>;
    fn InterfaceID(&self) -> windows_core::Result<i32>;
    fn ScopeDescription(&self) -> windows_core::Result<windows_core::BSTR>;
    fn TTL(&self) -> windows_core::Result<i32>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IMcastScope {}
#[cfg(feature = "Win32_System_Com")]
impl IMcastScope_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IMcastScope_Impl, const OFFSET: isize>() -> IMcastScope_Vtbl {
        unsafe extern "system" fn ScopeID<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IMcastScope_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pid: *mut i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IMcastScope_Impl::ScopeID(this) {
                Ok(ok__) => {
                    core::ptr::write(pid, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ServerID<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IMcastScope_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pid: *mut i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IMcastScope_Impl::ServerID(this) {
                Ok(ok__) => {
                    core::ptr::write(pid, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn InterfaceID<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IMcastScope_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pid: *mut i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IMcastScope_Impl::InterfaceID(this) {
                Ok(ok__) => {
                    core::ptr::write(pid, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ScopeDescription<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IMcastScope_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppdescription: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IMcastScope_Impl::ScopeDescription(this) {
                Ok(ok__) => {
                    core::ptr::write(ppdescription, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TTL<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IMcastScope_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pttl: *mut i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IMcastScope_Impl::TTL(this) {
                Ok(ok__) => {
                    core::ptr::write(pttl, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            ScopeID: ScopeID::<Identity, Impl, OFFSET>,
            ServerID: ServerID::<Identity, Impl, OFFSET>,
            InterfaceID: InterfaceID::<Identity, Impl, OFFSET>,
            ScopeDescription: ScopeDescription::<Identity, Impl, OFFSET>,
            TTL: TTL::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMcastScope as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait ITACDGroup_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn Name(&self) -> windows_core::Result<windows_core::BSTR>;
    fn EnumerateQueues(&self) -> windows_core::Result<IEnumQueue>;
    fn Queues(&self) -> windows_core::Result<windows_core::VARIANT>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for ITACDGroup {}
#[cfg(feature = "Win32_System_Com")]
impl ITACDGroup_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITACDGroup_Impl, const OFFSET: isize>() -> ITACDGroup_Vtbl {
        unsafe extern "system" fn Name<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITACDGroup_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppname: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ITACDGroup_Impl::Name(this) {
                Ok(ok__) => {
                    core::ptr::write(ppname, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnumerateQueues<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITACDGroup_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppenumqueue: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ITACDGroup_Impl::EnumerateQueues(this) {
                Ok(ok__) => {
                    core::ptr::write(ppenumqueue, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Queues<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITACDGroup_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvariant: *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ITACDGroup_Impl::Queues(this) {
                Ok(ok__) => {
                    core::ptr::write(pvariant, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            Name: Name::<Identity, Impl, OFFSET>,
            EnumerateQueues: EnumerateQueues::<Identity, Impl, OFFSET>,
            Queues: Queues::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITACDGroup as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait ITACDGroupEvent_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn Group(&self) -> windows_core::Result<ITACDGroup>;
    fn Event(&self) -> windows_core::Result<ACDGROUP_EVENT>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for ITACDGroupEvent {}
#[cfg(feature = "Win32_System_Com")]
impl ITACDGroupEvent_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITACDGroupEvent_Impl, const OFFSET: isize>() -> ITACDGroupEvent_Vtbl {
        unsafe extern "system" fn Group<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITACDGroupEvent_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppgroup: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ITACDGroupEvent_Impl::Group(this) {
                Ok(ok__) => {
                    core::ptr::write(ppgroup, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Event<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITACDGroupEvent_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pevent: *mut ACDGROUP_EVENT) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ITACDGroupEvent_Impl::Event(this) {
                Ok(ok__) => {
                    core::ptr::write(pevent, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            Group: Group::<Identity, Impl, OFFSET>,
            Event: Event::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITACDGroupEvent as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Media_MediaFoundation")]
pub trait ITAMMediaFormat_Impl: Sized {
    fn MediaFormat(&self) -> windows_core::Result<*mut super::super::Media::MediaFoundation::AM_MEDIA_TYPE>;
    fn SetMediaFormat(&self, pmt: *const super::super::Media::MediaFoundation::AM_MEDIA_TYPE) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_Media_MediaFoundation")]
impl windows_core::RuntimeName for ITAMMediaFormat {}
#[cfg(feature = "Win32_Media_MediaFoundation")]
impl ITAMMediaFormat_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITAMMediaFormat_Impl, const OFFSET: isize>() -> ITAMMediaFormat_Vtbl {
        unsafe extern "system" fn MediaFormat<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITAMMediaFormat_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppmt: *mut *mut super::super::Media::MediaFoundation::AM_MEDIA_TYPE) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ITAMMediaFormat_Impl::MediaFormat(this) {
                Ok(ok__) => {
                    core::ptr::write(ppmt, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMediaFormat<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITAMMediaFormat_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pmt: *const super::super::Media::MediaFoundation::AM_MEDIA_TYPE) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ITAMMediaFormat_Impl::SetMediaFormat(this, core::mem::transmute_copy(&pmt)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            MediaFormat: MediaFormat::<Identity, Impl, OFFSET>,
            SetMediaFormat: SetMediaFormat::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITAMMediaFormat as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait ITASRTerminalEvent_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn Terminal(&self) -> windows_core::Result<ITTerminal>;
    fn Call(&self) -> windows_core::Result<ITCallInfo>;
    fn Error(&self) -> windows_core::Result<windows_core::HRESULT>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for ITASRTerminalEvent {}
#[cfg(feature = "Win32_System_Com")]
impl ITASRTerminalEvent_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITASRTerminalEvent_Impl, const OFFSET: isize>() -> ITASRTerminalEvent_Vtbl {
        unsafe extern "system" fn Terminal<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITASRTerminalEvent_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppterminal: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ITASRTerminalEvent_Impl::Terminal(this) {
                Ok(ok__) => {
                    core::ptr::write(ppterminal, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Call<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITASRTerminalEvent_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppcall: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ITASRTerminalEvent_Impl::Call(this) {
                Ok(ok__) => {
                    core::ptr::write(ppcall, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Error<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITASRTerminalEvent_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, phrerrorcode: *mut windows_core::HRESULT) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ITASRTerminalEvent_Impl::Error(this) {
                Ok(ok__) => {
                    core::ptr::write(phrerrorcode, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            Terminal: Terminal::<Identity, Impl, OFFSET>,
            Call: Call::<Identity, Impl, OFFSET>,
            Error: Error::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITASRTerminalEvent as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait ITAddress_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn State(&self) -> windows_core::Result<ADDRESS_STATE>;
    fn AddressName(&self) -> windows_core::Result<windows_core::BSTR>;
    fn ServiceProviderName(&self) -> windows_core::Result<windows_core::BSTR>;
    fn TAPIObject(&self) -> windows_core::Result<ITTAPI>;
    fn CreateCall(&self, pdestaddress: &windows_core::BSTR, laddresstype: i32, lmediatypes: i32) -> windows_core::Result<ITBasicCallControl>;
    fn Calls(&self) -> windows_core::Result<windows_core::VARIANT>;
    fn EnumerateCalls(&self) -> windows_core::Result<IEnumCall>;
    fn DialableAddress(&self) -> windows_core::Result<windows_core::BSTR>;
    fn CreateForwardInfoObject(&self) -> windows_core::Result<ITForwardInformation>;
    fn Forward(&self, pforwardinfo: Option<&ITForwardInformation>, pcall: Option<&ITBasicCallControl>) -> windows_core::Result<()>;
    fn CurrentForwardInfo(&self) -> windows_core::Result<ITForwardInformation>;
    fn SetMessageWaiting(&self, fmessagewaiting: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
    fn MessageWaiting(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn SetDoNotDisturb(&self, fdonotdisturb: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
    fn DoNotDisturb(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for ITAddress {}
#[cfg(feature = "Win32_System_Com")]
impl ITAddress_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITAddress_Impl, const OFFSET: isize>() -> ITAddress_Vtbl {
        unsafe extern "system" fn State<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITAddress_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, paddressstate: *mut ADDRESS_STATE) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ITAddress_Impl::State(this) {
                Ok(ok__) => {
                    core::ptr::write(paddressstate, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AddressName<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITAddress_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppname: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ITAddress_Impl::AddressName(this) {
                Ok(ok__) => {
                    core::ptr::write(ppname, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ServiceProviderName<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITAddress_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppname: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ITAddress_Impl::ServiceProviderName(this) {
                Ok(ok__) => {
                    core::ptr::write(ppname, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TAPIObject<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITAddress_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pptapiobject: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ITAddress_Impl::TAPIObject(this) {
                Ok(ok__) => {
                    core::ptr::write(pptapiobject, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateCall<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITAddress_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdestaddress: core::mem::MaybeUninit<windows_core::BSTR>, laddresstype: i32, lmediatypes: i32, ppcall: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ITAddress_Impl::CreateCall(this, core::mem::transmute(&pdestaddress), core::mem::transmute_copy(&laddresstype), core::mem::transmute_copy(&lmediatypes)) {
                Ok(ok__) => {
                    core::ptr::write(ppcall, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Calls<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITAddress_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvariant: *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ITAddress_Impl::Calls(this) {
                Ok(ok__) => {
                    core::ptr::write(pvariant, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnumerateCalls<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITAddress_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppcallenum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ITAddress_Impl::EnumerateCalls(this) {
                Ok(ok__) => {
                    core::ptr::write(ppcallenum, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DialableAddress<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITAddress_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdialableaddress: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ITAddress_Impl::DialableAddress(this) {
                Ok(ok__) => {
                    core::ptr::write(pdialableaddress, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateForwardInfoObject<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITAddress_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppforwardinfo: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ITAddress_Impl::CreateForwardInfoObject(this) {
                Ok(ok__) => {
                    core::ptr::write(ppforwardinfo, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Forward<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITAddress_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pforwardinfo: *mut core::ffi::c_void, pcall: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ITAddress_Impl::Forward(this, windows_core::from_raw_borrowed(&pforwardinfo), windows_core::from_raw_borrowed(&pcall)).into()
        }
        unsafe extern "system" fn CurrentForwardInfo<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITAddress_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppforwardinfo: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ITAddress_Impl::CurrentForwardInfo(this) {
                Ok(ok__) => {
                    core::ptr::write(ppforwardinfo, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMessageWaiting<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITAddress_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, fmessagewaiting: super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ITAddress_Impl::SetMessageWaiting(this, core::mem::transmute_copy(&fmessagewaiting)).into()
        }
        unsafe extern "system" fn MessageWaiting<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITAddress_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pfmessagewaiting: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ITAddress_Impl::MessageWaiting(this) {
                Ok(ok__) => {
                    core::ptr::write(pfmessagewaiting, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDoNotDisturb<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITAddress_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, fdonotdisturb: super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ITAddress_Impl::SetDoNotDisturb(this, core::mem::transmute_copy(&fdonotdisturb)).into()
        }
        unsafe extern "system" fn DoNotDisturb<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITAddress_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pfdonotdisturb: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ITAddress_Impl::DoNotDisturb(this) {
                Ok(ok__) => {
                    core::ptr::write(pfdonotdisturb, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            State: State::<Identity, Impl, OFFSET>,
            AddressName: AddressName::<Identity, Impl, OFFSET>,
            ServiceProviderName: ServiceProviderName::<Identity, Impl, OFFSET>,
            TAPIObject: TAPIObject::<Identity, Impl, OFFSET>,
            CreateCall: CreateCall::<Identity, Impl, OFFSET>,
            Calls: Calls::<Identity, Impl, OFFSET>,
            EnumerateCalls: EnumerateCalls::<Identity, Impl, OFFSET>,
            DialableAddress: DialableAddress::<Identity, Impl, OFFSET>,
            CreateForwardInfoObject: CreateForwardInfoObject::<Identity, Impl, OFFSET>,
            Forward: Forward::<Identity, Impl, OFFSET>,
            CurrentForwardInfo: CurrentForwardInfo::<Identity, Impl, OFFSET>,
            SetMessageWaiting: SetMessageWaiting::<Identity, Impl, OFFSET>,
            MessageWaiting: MessageWaiting::<Identity, Impl, OFFSET>,
            SetDoNotDisturb: SetDoNotDisturb::<Identity, Impl, OFFSET>,
            DoNotDisturb: DoNotDisturb::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITAddress as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait ITAddress2_Impl: Sized + ITAddress_Impl {
    fn Phones(&self) -> windows_core::Result<windows_core::VARIANT>;
    fn EnumeratePhones(&self) -> windows_core::Result<IEnumPhone>;
    fn GetPhoneFromTerminal(&self, pterminal: Option<&ITTerminal>) -> windows_core::Result<ITPhone>;
    fn PreferredPhones(&self) -> windows_core::Result<windows_core::VARIANT>;
    fn EnumeratePreferredPhones(&self) -> windows_core::Result<IEnumPhone>;
    fn get_EventFilter(&self, tapievent: TAPI_EVENT, lsubevent: i32) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn put_EventFilter(&self, tapievent: TAPI_EVENT, lsubevent: i32, benable: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
    fn DeviceSpecific(&self, pcall: Option<&ITCallInfo>, pparams: *const u8, dwsize: u32) -> windows_core::Result<()>;
    fn DeviceSpecificVariant(&self, pcall: Option<&ITCallInfo>, vardevspecificbytearray: &windows_core::VARIANT) -> windows_core::Result<()>;
    fn NegotiateExtVersion(&self, llowversion: i32, lhighversion: i32) -> windows_core::Result<i32>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for ITAddress2 {}
#[cfg(feature = "Win32_System_Com")]
impl ITAddress2_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITAddress2_Impl, const OFFSET: isize>() -> ITAddress2_Vtbl {
        unsafe extern "system" fn Phones<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITAddress2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pphones: *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ITAddress2_Impl::Phones(this) {
                Ok(ok__) => {
                    core::ptr::write(pphones, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnumeratePhones<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITAddress2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppenumphone: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ITAddress2_Impl::EnumeratePhones(this) {
                Ok(ok__) => {
                    core::ptr::write(ppenumphone, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPhoneFromTerminal<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITAddress2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pterminal: *mut core::ffi::c_void, ppphone: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ITAddress2_Impl::GetPhoneFromTerminal(this, windows_core::from_raw_borrowed(&pterminal)) {
                Ok(ok__) => {
                    core::ptr::write(ppphone, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PreferredPhones<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITAddress2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pphones: *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ITAddress2_Impl::PreferredPhones(this) {
                Ok(ok__) => {
                    core::ptr::write(pphones, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnumeratePreferredPhones<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITAddress2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppenumphone: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ITAddress2_Impl::EnumeratePreferredPhones(this) {
                Ok(ok__) => {
                    core::ptr::write(ppenumphone, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn get_EventFilter<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITAddress2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, tapievent: TAPI_EVENT, lsubevent: i32, penable: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ITAddress2_Impl::get_EventFilter(this, core::mem::transmute_copy(&tapievent), core::mem::transmute_copy(&lsubevent)) {
                Ok(ok__) => {
                    core::ptr::write(penable, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn put_EventFilter<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITAddress2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, tapievent: TAPI_EVENT, lsubevent: i32, benable: super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ITAddress2_Impl::put_EventFilter(this, core::mem::transmute_copy(&tapievent), core::mem::transmute_copy(&lsubevent), core::mem::transmute_copy(&benable)).into()
        }
        unsafe extern "system" fn DeviceSpecific<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITAddress2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcall: *mut core::ffi::c_void, pparams: *const u8, dwsize: u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ITAddress2_Impl::DeviceSpecific(this, windows_core::from_raw_borrowed(&pcall), core::mem::transmute_copy(&pparams), core::mem::transmute_copy(&dwsize)).into()
        }
        unsafe extern "system" fn DeviceSpecificVariant<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITAddress2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcall: *mut core::ffi::c_void, vardevspecificbytearray: core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ITAddress2_Impl::DeviceSpecificVariant(this, windows_core::from_raw_borrowed(&pcall), core::mem::transmute(&vardevspecificbytearray)).into()
        }
        unsafe extern "system" fn NegotiateExtVersion<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITAddress2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, llowversion: i32, lhighversion: i32, plextversion: *mut i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ITAddress2_Impl::NegotiateExtVersion(this, core::mem::transmute_copy(&llowversion), core::mem::transmute_copy(&lhighversion)) {
                Ok(ok__) => {
                    core::ptr::write(plextversion, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: ITAddress_Vtbl::new::<Identity, Impl, OFFSET>(),
            Phones: Phones::<Identity, Impl, OFFSET>,
            EnumeratePhones: EnumeratePhones::<Identity, Impl, OFFSET>,
            GetPhoneFromTerminal: GetPhoneFromTerminal::<Identity, Impl, OFFSET>,
            PreferredPhones: PreferredPhones::<Identity, Impl, OFFSET>,
            EnumeratePreferredPhones: EnumeratePreferredPhones::<Identity, Impl, OFFSET>,
            get_EventFilter: get_EventFilter::<Identity, Impl, OFFSET>,
            put_EventFilter: put_EventFilter::<Identity, Impl, OFFSET>,
            DeviceSpecific: DeviceSpecific::<Identity, Impl, OFFSET>,
            DeviceSpecificVariant: DeviceSpecificVariant::<Identity, Impl, OFFSET>,
            NegotiateExtVersion: NegotiateExtVersion::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITAddress2 as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID || iid == &<ITAddress as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait ITAddressCapabilities_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn get_AddressCapability(&self, addresscap: ADDRESS_CAPABILITY) -> windows_core::Result<i32>;
    fn get_AddressCapabilityString(&self, addresscapstring: ADDRESS_CAPABILITY_STRING) -> windows_core::Result<windows_core::BSTR>;
    fn CallTreatments(&self) -> windows_core::Result<windows_core::VARIANT>;
    fn EnumerateCallTreatments(&self) -> windows_core::Result<IEnumBstr>;
    fn CompletionMessages(&self) -> windows_core::Result<windows_core::VARIANT>;
    fn EnumerateCompletionMessages(&self) -> windows_core::Result<IEnumBstr>;
    fn DeviceClasses(&self) -> windows_core::Result<windows_core::VARIANT>;
    fn EnumerateDeviceClasses(&self) -> windows_core::Result<IEnumBstr>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for ITAddressCapabilities {}
#[cfg(feature = "Win32_System_Com")]
impl ITAddressCapabilities_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITAddressCapabilities_Impl, const OFFSET: isize>() -> ITAddressCapabilities_Vtbl {
        unsafe extern "system" fn get_AddressCapability<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITAddressCapabilities_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, addresscap: ADDRESS_CAPABILITY, plcapability: *mut i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ITAddressCapabilities_Impl::get_AddressCapability(this, core::mem::transmute_copy(&addresscap)) {
                Ok(ok__) => {
                    core::ptr::write(plcapability, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn get_AddressCapabilityString<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITAddressCapabilities_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, addresscapstring: ADDRESS_CAPABILITY_STRING, ppcapabilitystring: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ITAddressCapabilities_Impl::get_AddressCapabilityString(this, core::mem::transmute_copy(&addresscapstring)) {
                Ok(ok__) => {
                    core::ptr::write(ppcapabilitystring, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CallTreatments<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITAddressCapabilities_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvariant: *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ITAddressCapabilities_Impl::CallTreatments(this) {
                Ok(ok__) => {
                    core::ptr::write(pvariant, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnumerateCallTreatments<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITAddressCapabilities_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppenumcalltreatment: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ITAddressCapabilities_Impl::EnumerateCallTreatments(this) {
                Ok(ok__) => {
                    core::ptr::write(ppenumcalltreatment, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CompletionMessages<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITAddressCapabilities_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvariant: *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ITAddressCapabilities_Impl::CompletionMessages(this) {
                Ok(ok__) => {
                    core::ptr::write(pvariant, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnumerateCompletionMessages<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITAddressCapabilities_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppenumcompletionmessage: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ITAddressCapabilities_Impl::EnumerateCompletionMessages(this) {
                Ok(ok__) => {
                    core::ptr::write(ppenumcompletionmessage, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DeviceClasses<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITAddressCapabilities_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvariant: *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ITAddressCapabilities_Impl::DeviceClasses(this) {
                Ok(ok__) => {
                    core::ptr::write(pvariant, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnumerateDeviceClasses<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITAddressCapabilities_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppenumdeviceclass: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ITAddressCapabilities_Impl::EnumerateDeviceClasses(this) {
                Ok(ok__) => {
                    core::ptr::write(ppenumdeviceclass, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            get_AddressCapability: get_AddressCapability::<Identity, Impl, OFFSET>,
            get_AddressCapabilityString: get_AddressCapabilityString::<Identity, Impl, OFFSET>,
            CallTreatments: CallTreatments::<Identity, Impl, OFFSET>,
            EnumerateCallTreatments: EnumerateCallTreatments::<Identity, Impl, OFFSET>,
            CompletionMessages: CompletionMessages::<Identity, Impl, OFFSET>,
            EnumerateCompletionMessages: EnumerateCompletionMessages::<Identity, Impl, OFFSET>,
            DeviceClasses: DeviceClasses::<Identity, Impl, OFFSET>,
            EnumerateDeviceClasses: EnumerateDeviceClasses::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITAddressCapabilities as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait ITAddressDeviceSpecificEvent_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn Address(&self) -> windows_core::Result<ITAddress>;
    fn Call(&self) -> windows_core::Result<ITCallInfo>;
    fn lParam1(&self) -> windows_core::Result<i32>;
    fn lParam2(&self) -> windows_core::Result<i32>;
    fn lParam3(&self) -> windows_core::Result<i32>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for ITAddressDeviceSpecificEvent {}
#[cfg(feature = "Win32_System_Com")]
impl ITAddressDeviceSpecificEvent_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITAddressDeviceSpecificEvent_Impl, const OFFSET: isize>() -> ITAddressDeviceSpecificEvent_Vtbl {
        unsafe extern "system" fn Address<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITAddressDeviceSpecificEvent_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppaddress: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ITAddressDeviceSpecificEvent_Impl::Address(this) {
                Ok(ok__) => {
                    core::ptr::write(ppaddress, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Call<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITAddressDeviceSpecificEvent_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppcall: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ITAddressDeviceSpecificEvent_Impl::Call(this) {
                Ok(ok__) => {
                    core::ptr::write(ppcall, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn lParam1<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITAddressDeviceSpecificEvent_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pparam1: *mut i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ITAddressDeviceSpecificEvent_Impl::lParam1(this) {
                Ok(ok__) => {
                    core::ptr::write(pparam1, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn lParam2<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITAddressDeviceSpecificEvent_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pparam2: *mut i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ITAddressDeviceSpecificEvent_Impl::lParam2(this) {
                Ok(ok__) => {
                    core::ptr::write(pparam2, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn lParam3<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITAddressDeviceSpecificEvent_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pparam3: *mut i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ITAddressDeviceSpecificEvent_Impl::lParam3(this) {
                Ok(ok__) => {
                    core::ptr::write(pparam3, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            Address: Address::<Identity, Impl, OFFSET>,
            Call: Call::<Identity, Impl, OFFSET>,
            lParam1: lParam1::<Identity, Impl, OFFSET>,
            lParam2: lParam2::<Identity, Impl, OFFSET>,
            lParam3: lParam3::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITAddressDeviceSpecificEvent as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait ITAddressEvent_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn Address(&self) -> windows_core::Result<ITAddress>;
    fn Event(&self) -> windows_core::Result<ADDRESS_EVENT>;
    fn Terminal(&self) -> windows_core::Result<ITTerminal>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for ITAddressEvent {}
#[cfg(feature = "Win32_System_Com")]
impl ITAddressEvent_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITAddressEvent_Impl, const OFFSET: isize>() -> ITAddressEvent_Vtbl {
        unsafe extern "system" fn Address<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITAddressEvent_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppaddress: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ITAddressEvent_Impl::Address(this) {
                Ok(ok__) => {
                    core::ptr::write(ppaddress, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Event<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITAddressEvent_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pevent: *mut ADDRESS_EVENT) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ITAddressEvent_Impl::Event(this) {
                Ok(ok__) => {
                    core::ptr::write(pevent, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Terminal<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITAddressEvent_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppterminal: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ITAddressEvent_Impl::Terminal(this) {
                Ok(ok__) => {
                    core::ptr::write(ppterminal, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            Address: Address::<Identity, Impl, OFFSET>,
            Event: Event::<Identity, Impl, OFFSET>,
            Terminal: Terminal::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITAddressEvent as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait ITAddressTranslation_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn TranslateAddress(&self, paddresstotranslate: &windows_core::BSTR, lcard: i32, ltranslateoptions: i32) -> windows_core::Result<ITAddressTranslationInfo>;
    fn TranslateDialog(&self, hwndowner: isize, paddressin: &windows_core::BSTR) -> windows_core::Result<()>;
    fn EnumerateLocations(&self) -> windows_core::Result<IEnumLocation>;
    fn Locations(&self) -> windows_core::Result<windows_core::VARIANT>;
    fn EnumerateCallingCards(&self) -> windows_core::Result<IEnumCallingCard>;
    fn CallingCards(&self) -> windows_core::Result<windows_core::VARIANT>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for ITAddressTranslation {}
#[cfg(feature = "Win32_System_Com")]
impl ITAddressTranslation_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITAddressTranslation_Impl, const OFFSET: isize>() -> ITAddressTranslation_Vtbl {
        unsafe extern "system" fn TranslateAddress<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITAddressTranslation_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, paddresstotranslate: core::mem::MaybeUninit<windows_core::BSTR>, lcard: i32, ltranslateoptions: i32, pptranslated: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ITAddressTranslation_Impl::TranslateAddress(this, core::mem::transmute(&paddresstotranslate), core::mem::transmute_copy(&lcard), core::mem::transmute_copy(&ltranslateoptions)) {
                Ok(ok__) => {
                    core::ptr::write(pptranslated, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TranslateDialog<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITAddressTranslation_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hwndowner: isize, paddressin: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ITAddressTranslation_Impl::TranslateDialog(this, core::mem::transmute_copy(&hwndowner), core::mem::transmute(&paddressin)).into()
        }
        unsafe extern "system" fn EnumerateLocations<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITAddressTranslation_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppenumlocation: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ITAddressTranslation_Impl::EnumerateLocations(this) {
                Ok(ok__) => {
                    core::ptr::write(ppenumlocation, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Locations<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITAddressTranslation_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvariant: *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ITAddressTranslation_Impl::Locations(this) {
                Ok(ok__) => {
                    core::ptr::write(pvariant, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnumerateCallingCards<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITAddressTranslation_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppenumcallingcard: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ITAddressTranslation_Impl::EnumerateCallingCards(this) {
                Ok(ok__) => {
                    core::ptr::write(ppenumcallingcard, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CallingCards<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITAddressTranslation_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvariant: *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ITAddressTranslation_Impl::CallingCards(this) {
                Ok(ok__) => {
                    core::ptr::write(pvariant, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            TranslateAddress: TranslateAddress::<Identity, Impl, OFFSET>,
            TranslateDialog: TranslateDialog::<Identity, Impl, OFFSET>,
            EnumerateLocations: EnumerateLocations::<Identity, Impl, OFFSET>,
            Locations: Locations::<Identity, Impl, OFFSET>,
            EnumerateCallingCards: EnumerateCallingCards::<Identity, Impl, OFFSET>,
            CallingCards: CallingCards::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITAddressTranslation as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait ITAddressTranslationInfo_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn DialableString(&self) -> windows_core::Result<windows_core::BSTR>;
    fn DisplayableString(&self) -> windows_core::Result<windows_core::BSTR>;
    fn CurrentCountryCode(&self) -> windows_core::Result<i32>;
    fn DestinationCountryCode(&self) -> windows_core::Result<i32>;
    fn TranslationResults(&self) -> windows_core::Result<i32>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for ITAddressTranslationInfo {}
#[cfg(feature = "Win32_System_Com")]
impl ITAddressTranslationInfo_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITAddressTranslationInfo_Impl, const OFFSET: isize>() -> ITAddressTranslationInfo_Vtbl {
        unsafe extern "system" fn DialableString<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITAddressTranslationInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppdialablestring: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ITAddressTranslationInfo_Impl::DialableString(this) {
                Ok(ok__) => {
                    core::ptr::write(ppdialablestring, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DisplayableString<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITAddressTranslationInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppdisplayablestring: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ITAddressTranslationInfo_Impl::DisplayableString(this) {
                Ok(ok__) => {
                    core::ptr::write(ppdisplayablestring, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CurrentCountryCode<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITAddressTranslationInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, countrycode: *mut i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ITAddressTranslationInfo_Impl::CurrentCountryCode(this) {
                Ok(ok__) => {
                    core::ptr::write(countrycode, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DestinationCountryCode<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITAddressTranslationInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, countrycode: *mut i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ITAddressTranslationInfo_Impl::DestinationCountryCode(this) {
                Ok(ok__) => {
                    core::ptr::write(countrycode, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TranslationResults<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITAddressTranslationInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, plresults: *mut i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ITAddressTranslationInfo_Impl::TranslationResults(this) {
                Ok(ok__) => {
                    core::ptr::write(plresults, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            DialableString: DialableString::<Identity, Impl, OFFSET>,
            DisplayableString: DisplayableString::<Identity, Impl, OFFSET>,
            CurrentCountryCode: CurrentCountryCode::<Identity, Impl, OFFSET>,
            DestinationCountryCode: DestinationCountryCode::<Identity, Impl, OFFSET>,
            TranslationResults: TranslationResults::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITAddressTranslationInfo as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait ITAgent_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn EnumerateAgentSessions(&self) -> windows_core::Result<IEnumAgentSession>;
    fn CreateSession(&self, pacdgroup: Option<&ITACDGroup>, paddress: Option<&ITAddress>) -> windows_core::Result<ITAgentSession>;
    fn CreateSessionWithPIN(&self, pacdgroup: Option<&ITACDGroup>, paddress: Option<&ITAddress>, ppin: &windows_core::BSTR) -> windows_core::Result<ITAgentSession>;
    fn ID(&self) -> windows_core::Result<windows_core::BSTR>;
    fn User(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetState(&self, agentstate: AGENT_STATE) -> windows_core::Result<()>;
    fn State(&self) -> windows_core::Result<AGENT_STATE>;
    fn SetMeasurementPeriod(&self, lperiod: i32) -> windows_core::Result<()>;
    fn MeasurementPeriod(&self) -> windows_core::Result<i32>;
    fn OverallCallRate(&self) -> windows_core::Result<super::super::System::Com::CY>;
    fn NumberOfACDCalls(&self) -> windows_core::Result<i32>;
    fn NumberOfIncomingCalls(&self) -> windows_core::Result<i32>;
    fn NumberOfOutgoingCalls(&self) -> windows_core::Result<i32>;
    fn TotalACDTalkTime(&self) -> windows_core::Result<i32>;
    fn TotalACDCallTime(&self) -> windows_core::Result<i32>;
    fn TotalWrapUpTime(&self) -> windows_core::Result<i32>;
    fn AgentSessions(&self) -> windows_core::Result<windows_core::VARIANT>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for ITAgent {}
#[cfg(feature = "Win32_System_Com")]
impl ITAgent_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITAgent_Impl, const OFFSET: isize>() -> ITAgent_Vtbl {
        unsafe extern "system" fn EnumerateAgentSessions<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITAgent_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppenumagentsession: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ITAgent_Impl::EnumerateAgentSessions(this) {
                Ok(ok__) => {
                    core::ptr::write(ppenumagentsession, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateSession<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITAgent_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pacdgroup: *mut core::ffi::c_void, paddress: *mut core::ffi::c_void, ppagentsession: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ITAgent_Impl::CreateSession(this, windows_core::from_raw_borrowed(&pacdgroup), windows_core::from_raw_borrowed(&paddress)) {
                Ok(ok__) => {
                    core::ptr::write(ppagentsession, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateSessionWithPIN<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITAgent_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pacdgroup: *mut core::ffi::c_void, paddress: *mut core::ffi::c_void, ppin: core::mem::MaybeUninit<windows_core::BSTR>, ppagentsession: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ITAgent_Impl::CreateSessionWithPIN(this, windows_core::from_raw_borrowed(&pacdgroup), windows_core::from_raw_borrowed(&paddress), core::mem::transmute(&ppin)) {
                Ok(ok__) => {
                    core::ptr::write(ppagentsession, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ID<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITAgent_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppid: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ITAgent_Impl::ID(this) {
                Ok(ok__) => {
                    core::ptr::write(ppid, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn User<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITAgent_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppuser: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ITAgent_Impl::User(this) {
                Ok(ok__) => {
                    core::ptr::write(ppuser, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetState<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITAgent_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, agentstate: AGENT_STATE) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ITAgent_Impl::SetState(this, core::mem::transmute_copy(&agentstate)).into()
        }
        unsafe extern "system" fn State<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITAgent_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pagentstate: *mut AGENT_STATE) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ITAgent_Impl::State(this) {
                Ok(ok__) => {
                    core::ptr::write(pagentstate, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMeasurementPeriod<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITAgent_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lperiod: i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ITAgent_Impl::SetMeasurementPeriod(this, core::mem::transmute_copy(&lperiod)).into()
        }
        unsafe extern "system" fn MeasurementPeriod<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITAgent_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, plperiod: *mut i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ITAgent_Impl::MeasurementPeriod(this) {
                Ok(ok__) => {
                    core::ptr::write(plperiod, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OverallCallRate<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITAgent_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcycallrate: *mut super::super::System::Com::CY) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ITAgent_Impl::OverallCallRate(this) {
                Ok(ok__) => {
                    core::ptr::write(pcycallrate, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn NumberOfACDCalls<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITAgent_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, plcalls: *mut i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ITAgent_Impl::NumberOfACDCalls(this) {
                Ok(ok__) => {
                    core::ptr::write(plcalls, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn NumberOfIncomingCalls<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITAgent_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, plcalls: *mut i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ITAgent_Impl::NumberOfIncomingCalls(this) {
                Ok(ok__) => {
                    core::ptr::write(plcalls, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn NumberOfOutgoingCalls<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITAgent_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, plcalls: *mut i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ITAgent_Impl::NumberOfOutgoingCalls(this) {
                Ok(ok__) => {
                    core::ptr::write(plcalls, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TotalACDTalkTime<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITAgent_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pltalktime: *mut i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ITAgent_Impl::TotalACDTalkTime(this) {
                Ok(ok__) => {
                    core::ptr::write(pltalktime, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TotalACDCallTime<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITAgent_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, plcalltime: *mut i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ITAgent_Impl::TotalACDCallTime(this) {
                Ok(ok__) => {
                    core::ptr::write(plcalltime, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TotalWrapUpTime<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITAgent_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, plwrapuptime: *mut i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ITAgent_Impl::TotalWrapUpTime(this) {
                Ok(ok__) => {
                    core::ptr::write(plwrapuptime, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AgentSessions<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITAgent_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvariant: *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ITAgent_Impl::AgentSessions(this) {
                Ok(ok__) => {
                    core::ptr::write(pvariant, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            EnumerateAgentSessions: EnumerateAgentSessions::<Identity, Impl, OFFSET>,
            CreateSession: CreateSession::<Identity, Impl, OFFSET>,
            CreateSessionWithPIN: CreateSessionWithPIN::<Identity, Impl, OFFSET>,
            ID: ID::<Identity, Impl, OFFSET>,
            User: User::<Identity, Impl, OFFSET>,
            SetState: SetState::<Identity, Impl, OFFSET>,
            State: State::<Identity, Impl, OFFSET>,
            SetMeasurementPeriod: SetMeasurementPeriod::<Identity, Impl, OFFSET>,
            MeasurementPeriod: MeasurementPeriod::<Identity, Impl, OFFSET>,
            OverallCallRate: OverallCallRate::<Identity, Impl, OFFSET>,
            NumberOfACDCalls: NumberOfACDCalls::<Identity, Impl, OFFSET>,
            NumberOfIncomingCalls: NumberOfIncomingCalls::<Identity, Impl, OFFSET>,
            NumberOfOutgoingCalls: NumberOfOutgoingCalls::<Identity, Impl, OFFSET>,
            TotalACDTalkTime: TotalACDTalkTime::<Identity, Impl, OFFSET>,
            TotalACDCallTime: TotalACDCallTime::<Identity, Impl, OFFSET>,
            TotalWrapUpTime: TotalWrapUpTime::<Identity, Impl, OFFSET>,
            AgentSessions: AgentSessions::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITAgent as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait ITAgentEvent_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn Agent(&self) -> windows_core::Result<ITAgent>;
    fn Event(&self) -> windows_core::Result<AGENT_EVENT>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for ITAgentEvent {}
#[cfg(feature = "Win32_System_Com")]
impl ITAgentEvent_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITAgentEvent_Impl, const OFFSET: isize>() -> ITAgentEvent_Vtbl {
        unsafe extern "system" fn Agent<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITAgentEvent_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppagent: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ITAgentEvent_Impl::Agent(this) {
                Ok(ok__) => {
                    core::ptr::write(ppagent, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Event<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITAgentEvent_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pevent: *mut AGENT_EVENT) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ITAgentEvent_Impl::Event(this) {
                Ok(ok__) => {
                    core::ptr::write(pevent, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            Agent: Agent::<Identity, Impl, OFFSET>,
            Event: Event::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITAgentEvent as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait ITAgentHandler_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn Name(&self) -> windows_core::Result<windows_core::BSTR>;
    fn CreateAgent(&self) -> windows_core::Result<ITAgent>;
    fn CreateAgentWithID(&self, pid: &windows_core::BSTR, ppin: &windows_core::BSTR) -> windows_core::Result<ITAgent>;
    fn EnumerateACDGroups(&self) -> windows_core::Result<IEnumACDGroup>;
    fn EnumerateUsableAddresses(&self) -> windows_core::Result<IEnumAddress>;
    fn ACDGroups(&self) -> windows_core::Result<windows_core::VARIANT>;
    fn UsableAddresses(&self) -> windows_core::Result<windows_core::VARIANT>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for ITAgentHandler {}
#[cfg(feature = "Win32_System_Com")]
impl ITAgentHandler_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITAgentHandler_Impl, const OFFSET: isize>() -> ITAgentHandler_Vtbl {
        unsafe extern "system" fn Name<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITAgentHandler_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppname: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ITAgentHandler_Impl::Name(this) {
                Ok(ok__) => {
                    core::ptr::write(ppname, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateAgent<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITAgentHandler_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppagent: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ITAgentHandler_Impl::CreateAgent(this) {
                Ok(ok__) => {
                    core::ptr::write(ppagent, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateAgentWithID<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITAgentHandler_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pid: core::mem::MaybeUninit<windows_core::BSTR>, ppin: core::mem::MaybeUninit<windows_core::BSTR>, ppagent: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ITAgentHandler_Impl::CreateAgentWithID(this, core::mem::transmute(&pid), core::mem::transmute(&ppin)) {
                Ok(ok__) => {
                    core::ptr::write(ppagent, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnumerateACDGroups<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITAgentHandler_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppenumacdgroup: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ITAgentHandler_Impl::EnumerateACDGroups(this) {
                Ok(ok__) => {
                    core::ptr::write(ppenumacdgroup, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnumerateUsableAddresses<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITAgentHandler_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppenumaddress: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ITAgentHandler_Impl::EnumerateUsableAddresses(this) {
                Ok(ok__) => {
                    core::ptr::write(ppenumaddress, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ACDGroups<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITAgentHandler_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvariant: *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ITAgentHandler_Impl::ACDGroups(this) {
                Ok(ok__) => {
                    core::ptr::write(pvariant, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UsableAddresses<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITAgentHandler_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvariant: *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ITAgentHandler_Impl::UsableAddresses(this) {
                Ok(ok__) => {
                    core::ptr::write(pvariant, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            Name: Name::<Identity, Impl, OFFSET>,
            CreateAgent: CreateAgent::<Identity, Impl, OFFSET>,
            CreateAgentWithID: CreateAgentWithID::<Identity, Impl, OFFSET>,
            EnumerateACDGroups: EnumerateACDGroups::<Identity, Impl, OFFSET>,
            EnumerateUsableAddresses: EnumerateUsableAddresses::<Identity, Impl, OFFSET>,
            ACDGroups: ACDGroups::<Identity, Impl, OFFSET>,
            UsableAddresses: UsableAddresses::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITAgentHandler as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait ITAgentHandlerEvent_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn AgentHandler(&self) -> windows_core::Result<ITAgentHandler>;
    fn Event(&self) -> windows_core::Result<AGENTHANDLER_EVENT>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for ITAgentHandlerEvent {}
#[cfg(feature = "Win32_System_Com")]
impl ITAgentHandlerEvent_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITAgentHandlerEvent_Impl, const OFFSET: isize>() -> ITAgentHandlerEvent_Vtbl {
        unsafe extern "system" fn AgentHandler<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITAgentHandlerEvent_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppagenthandler: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ITAgentHandlerEvent_Impl::AgentHandler(this) {
                Ok(ok__) => {
                    core::ptr::write(ppagenthandler, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Event<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITAgentHandlerEvent_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pevent: *mut AGENTHANDLER_EVENT) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ITAgentHandlerEvent_Impl::Event(this) {
                Ok(ok__) => {
                    core::ptr::write(pevent, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            AgentHandler: AgentHandler::<Identity, Impl, OFFSET>,
            Event: Event::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITAgentHandlerEvent as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait ITAgentSession_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn Agent(&self) -> windows_core::Result<ITAgent>;
    fn Address(&self) -> windows_core::Result<ITAddress>;
    fn ACDGroup(&self) -> windows_core::Result<ITACDGroup>;
    fn SetState(&self, sessionstate: AGENT_SESSION_STATE) -> windows_core::Result<()>;
    fn State(&self) -> windows_core::Result<AGENT_SESSION_STATE>;
    fn SessionStartTime(&self) -> windows_core::Result<f64>;
    fn SessionDuration(&self) -> windows_core::Result<i32>;
    fn NumberOfCalls(&self) -> windows_core::Result<i32>;
    fn TotalTalkTime(&self) -> windows_core::Result<i32>;
    fn AverageTalkTime(&self) -> windows_core::Result<i32>;
    fn TotalCallTime(&self) -> windows_core::Result<i32>;
    fn AverageCallTime(&self) -> windows_core::Result<i32>;
    fn TotalWrapUpTime(&self) -> windows_core::Result<i32>;
    fn AverageWrapUpTime(&self) -> windows_core::Result<i32>;
    fn ACDCallRate(&self) -> windows_core::Result<super::super::System::Com::CY>;
    fn LongestTimeToAnswer(&self) -> windows_core::Result<i32>;
    fn AverageTimeToAnswer(&self) -> windows_core::Result<i32>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for ITAgentSession {}
#[cfg(feature = "Win32_System_Com")]
impl ITAgentSession_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITAgentSession_Impl, const OFFSET: isize>() -> ITAgentSession_Vtbl {
        unsafe extern "system" fn Agent<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITAgentSession_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppagent: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ITAgentSession_Impl::Agent(this) {
                Ok(ok__) => {
                    core::ptr::write(ppagent, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Address<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITAgentSession_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppaddress: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ITAgentSession_Impl::Address(this) {
                Ok(ok__) => {
                    core::ptr::write(ppaddress, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ACDGroup<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITAgentSession_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppacdgroup: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ITAgentSession_Impl::ACDGroup(this) {
                Ok(ok__) => {
                    core::ptr::write(ppacdgroup, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetState<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITAgentSession_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, sessionstate: AGENT_SESSION_STATE) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ITAgentSession_Impl::SetState(this, core::mem::transmute_copy(&sessionstate)).into()
        }
        unsafe extern "system" fn State<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITAgentSession_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, psessionstate: *mut AGENT_SESSION_STATE) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ITAgentSession_Impl::State(this) {
                Ok(ok__) => {
                    core::ptr::write(psessionstate, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SessionStartTime<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITAgentSession_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdatesessionstart: *mut f64) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ITAgentSession_Impl::SessionStartTime(this) {
                Ok(ok__) => {
                    core::ptr::write(pdatesessionstart, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SessionDuration<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITAgentSession_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, plduration: *mut i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ITAgentSession_Impl::SessionDuration(this) {
                Ok(ok__) => {
                    core::ptr::write(plduration, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn NumberOfCalls<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITAgentSession_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, plcalls: *mut i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ITAgentSession_Impl::NumberOfCalls(this) {
                Ok(ok__) => {
                    core::ptr::write(plcalls, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TotalTalkTime<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITAgentSession_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pltalktime: *mut i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ITAgentSession_Impl::TotalTalkTime(this) {
                Ok(ok__) => {
                    core::ptr::write(pltalktime, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AverageTalkTime<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITAgentSession_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pltalktime: *mut i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ITAgentSession_Impl::AverageTalkTime(this) {
                Ok(ok__) => {
                    core::ptr::write(pltalktime, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TotalCallTime<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITAgentSession_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, plcalltime: *mut i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ITAgentSession_Impl::TotalCallTime(this) {
                Ok(ok__) => {
                    core::ptr::write(plcalltime, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AverageCallTime<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITAgentSession_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, plcalltime: *mut i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ITAgentSession_Impl::AverageCallTime(this) {
                Ok(ok__) => {
                    core::ptr::write(plcalltime, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TotalWrapUpTime<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITAgentSession_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, plwrapuptime: *mut i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ITAgentSession_Impl::TotalWrapUpTime(this) {
                Ok(ok__) => {
                    core::ptr::write(plwrapuptime, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AverageWrapUpTime<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITAgentSession_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, plwrapuptime: *mut i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ITAgentSession_Impl::AverageWrapUpTime(this) {
                Ok(ok__) => {
                    core::ptr::write(plwrapuptime, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ACDCallRate<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITAgentSession_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcycallrate: *mut super::super::System::Com::CY) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ITAgentSession_Impl::ACDCallRate(this) {
                Ok(ok__) => {
                    core::ptr::write(pcycallrate, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LongestTimeToAnswer<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITAgentSession_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, planswertime: *mut i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ITAgentSession_Impl::LongestTimeToAnswer(this) {
                Ok(ok__) => {
                    core::ptr::write(planswertime, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AverageTimeToAnswer<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITAgentSession_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, planswertime: *mut i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ITAgentSession_Impl::AverageTimeToAnswer(this) {
                Ok(ok__) => {
                    core::ptr::write(planswertime, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            Agent: Agent::<Identity, Impl, OFFSET>,
            Address: Address::<Identity, Impl, OFFSET>,
            ACDGroup: ACDGroup::<Identity, Impl, OFFSET>,
            SetState: SetState::<Identity, Impl, OFFSET>,
            State: State::<Identity, Impl, OFFSET>,
            SessionStartTime: SessionStartTime::<Identity, Impl, OFFSET>,
            SessionDuration: SessionDuration::<Identity, Impl, OFFSET>,
            NumberOfCalls: NumberOfCalls::<Identity, Impl, OFFSET>,
            TotalTalkTime: TotalTalkTime::<Identity, Impl, OFFSET>,
            AverageTalkTime: AverageTalkTime::<Identity, Impl, OFFSET>,
            TotalCallTime: TotalCallTime::<Identity, Impl, OFFSET>,
            AverageCallTime: AverageCallTime::<Identity, Impl, OFFSET>,
            TotalWrapUpTime: TotalWrapUpTime::<Identity, Impl, OFFSET>,
            AverageWrapUpTime: AverageWrapUpTime::<Identity, Impl, OFFSET>,
            ACDCallRate: ACDCallRate::<Identity, Impl, OFFSET>,
            LongestTimeToAnswer: LongestTimeToAnswer::<Identity, Impl, OFFSET>,
            AverageTimeToAnswer: AverageTimeToAnswer::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITAgentSession as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait ITAgentSessionEvent_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn Session(&self) -> windows_core::Result<ITAgentSession>;
    fn Event(&self) -> windows_core::Result<AGENT_SESSION_EVENT>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for ITAgentSessionEvent {}
#[cfg(feature = "Win32_System_Com")]
impl ITAgentSessionEvent_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITAgentSessionEvent_Impl, const OFFSET: isize>() -> ITAgentSessionEvent_Vtbl {
        unsafe extern "system" fn Session<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITAgentSessionEvent_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppsession: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ITAgentSessionEvent_Impl::Session(this) {
                Ok(ok__) => {
                    core::ptr::write(ppsession, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Event<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITAgentSessionEvent_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pevent: *mut AGENT_SESSION_EVENT) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ITAgentSessionEvent_Impl::Event(this) {
                Ok(ok__) => {
                    core::ptr::write(pevent, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            Session: Session::<Identity, Impl, OFFSET>,
            Event: Event::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITAgentSessionEvent as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Media_DirectShow")]
pub trait ITAllocatorProperties_Impl: Sized {
    fn SetAllocatorProperties(&self, pallocproperties: *const super::super::Media::DirectShow::ALLOCATOR_PROPERTIES) -> windows_core::Result<()>;
    fn GetAllocatorProperties(&self) -> windows_core::Result<super::super::Media::DirectShow::ALLOCATOR_PROPERTIES>;
    fn SetAllocateBuffers(&self, ballocbuffers: super::super::Foundation::BOOL) -> windows_core::Result<()>;
    fn GetAllocateBuffers(&self) -> windows_core::Result<super::super::Foundation::BOOL>;
    fn SetBufferSize(&self, buffersize: u32) -> windows_core::Result<()>;
    fn GetBufferSize(&self) -> windows_core::Result<u32>;
}
#[cfg(feature = "Win32_Media_DirectShow")]
impl windows_core::RuntimeName for ITAllocatorProperties {}
#[cfg(feature = "Win32_Media_DirectShow")]
impl ITAllocatorProperties_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITAllocatorProperties_Impl, const OFFSET: isize>() -> ITAllocatorProperties_Vtbl {
        unsafe extern "system" fn SetAllocatorProperties<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITAllocatorProperties_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pallocproperties: *const super::super::Media::DirectShow::ALLOCATOR_PROPERTIES) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ITAllocatorProperties_Impl::SetAllocatorProperties(this, core::mem::transmute_copy(&pallocproperties)).into()
        }
        unsafe extern "system" fn GetAllocatorProperties<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITAllocatorProperties_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pallocproperties: *mut super::super::Media::DirectShow::ALLOCATOR_PROPERTIES) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ITAllocatorProperties_Impl::GetAllocatorProperties(this) {
                Ok(ok__) => {
                    core::ptr::write(pallocproperties, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAllocateBuffers<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITAllocatorProperties_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ballocbuffers: super::super::Foundation::BOOL) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ITAllocatorProperties_Impl::SetAllocateBuffers(this, core::mem::transmute_copy(&ballocbuffers)).into()
        }
        unsafe extern "system" fn GetAllocateBuffers<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITAllocatorProperties_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pballocbuffers: *mut super::super::Foundation::BOOL) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ITAllocatorProperties_Impl::GetAllocateBuffers(this) {
                Ok(ok__) => {
                    core::ptr::write(pballocbuffers, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetBufferSize<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITAllocatorProperties_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, buffersize: u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ITAllocatorProperties_Impl::SetBufferSize(this, core::mem::transmute_copy(&buffersize)).into()
        }
        unsafe extern "system" fn GetBufferSize<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITAllocatorProperties_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbuffersize: *mut u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ITAllocatorProperties_Impl::GetBufferSize(this) {
                Ok(ok__) => {
                    core::ptr::write(pbuffersize, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            SetAllocatorProperties: SetAllocatorProperties::<Identity, Impl, OFFSET>,
            GetAllocatorProperties: GetAllocatorProperties::<Identity, Impl, OFFSET>,
            SetAllocateBuffers: SetAllocateBuffers::<Identity, Impl, OFFSET>,
            GetAllocateBuffers: GetAllocateBuffers::<Identity, Impl, OFFSET>,
            SetBufferSize: SetBufferSize::<Identity, Impl, OFFSET>,
            GetBufferSize: GetBufferSize::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITAllocatorProperties as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait ITAutomatedPhoneControl_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn StartTone(&self, tone: PHONE_TONE, lduration: i32) -> windows_core::Result<()>;
    fn StopTone(&self) -> windows_core::Result<()>;
    fn Tone(&self) -> windows_core::Result<PHONE_TONE>;
    fn StartRinger(&self, lringmode: i32, lduration: i32) -> windows_core::Result<()>;
    fn StopRinger(&self) -> windows_core::Result<()>;
    fn Ringer(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn SetPhoneHandlingEnabled(&self, fenabled: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
    fn PhoneHandlingEnabled(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn SetAutoEndOfNumberTimeout(&self, ltimeout: i32) -> windows_core::Result<()>;
    fn AutoEndOfNumberTimeout(&self) -> windows_core::Result<i32>;
    fn SetAutoDialtone(&self, fenabled: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
    fn AutoDialtone(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn SetAutoStopTonesOnOnHook(&self, fenabled: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
    fn AutoStopTonesOnOnHook(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn SetAutoStopRingOnOffHook(&self, fenabled: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
    fn AutoStopRingOnOffHook(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn SetAutoKeypadTones(&self, fenabled: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
    fn AutoKeypadTones(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn SetAutoKeypadTonesMinimumDuration(&self, lduration: i32) -> windows_core::Result<()>;
    fn AutoKeypadTonesMinimumDuration(&self) -> windows_core::Result<i32>;
    fn SetAutoVolumeControl(&self, fenabled: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
    fn AutoVolumeControl(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn SetAutoVolumeControlStep(&self, lstepsize: i32) -> windows_core::Result<()>;
    fn AutoVolumeControlStep(&self) -> windows_core::Result<i32>;
    fn SetAutoVolumeControlRepeatDelay(&self, ldelay: i32) -> windows_core::Result<()>;
    fn AutoVolumeControlRepeatDelay(&self) -> windows_core::Result<i32>;
    fn SetAutoVolumeControlRepeatPeriod(&self, lperiod: i32) -> windows_core::Result<()>;
    fn AutoVolumeControlRepeatPeriod(&self) -> windows_core::Result<i32>;
    fn SelectCall(&self, pcall: Option<&ITCallInfo>, fselectdefaultterminals: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
    fn UnselectCall(&self, pcall: Option<&ITCallInfo>) -> windows_core::Result<()>;
    fn EnumerateSelectedCalls(&self) -> windows_core::Result<IEnumCall>;
    fn SelectedCalls(&self) -> windows_core::Result<windows_core::VARIANT>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for ITAutomatedPhoneControl {}
#[cfg(feature = "Win32_System_Com")]
impl ITAutomatedPhoneControl_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITAutomatedPhoneControl_Impl, const OFFSET: isize>() -> ITAutomatedPhoneControl_Vtbl {
        unsafe extern "system" fn StartTone<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITAutomatedPhoneControl_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, tone: PHONE_TONE, lduration: i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ITAutomatedPhoneControl_Impl::StartTone(this, core::mem::transmute_copy(&tone), core::mem::transmute_copy(&lduration)).into()
        }
        unsafe extern "system" fn StopTone<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITAutomatedPhoneControl_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ITAutomatedPhoneControl_Impl::StopTone(this).into()
        }
        unsafe extern "system" fn Tone<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITAutomatedPhoneControl_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ptone: *mut PHONE_TONE) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ITAutomatedPhoneControl_Impl::Tone(this) {
                Ok(ok__) => {
                    core::ptr::write(ptone, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn StartRinger<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITAutomatedPhoneControl_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lringmode: i32, lduration: i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ITAutomatedPhoneControl_Impl::StartRinger(this, core::mem::transmute_copy(&lringmode), core::mem::transmute_copy(&lduration)).into()
        }
        unsafe extern "system" fn StopRinger<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITAutomatedPhoneControl_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ITAutomatedPhoneControl_Impl::StopRinger(this).into()
        }
        unsafe extern "system" fn Ringer<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITAutomatedPhoneControl_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pfringing: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ITAutomatedPhoneControl_Impl::Ringer(this) {
                Ok(ok__) => {
                    core::ptr::write(pfringing, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPhoneHandlingEnabled<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITAutomatedPhoneControl_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, fenabled: super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ITAutomatedPhoneControl_Impl::SetPhoneHandlingEnabled(this, core::mem::transmute_copy(&fenabled)).into()
        }
        unsafe extern "system" fn PhoneHandlingEnabled<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITAutomatedPhoneControl_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pfenabled: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ITAutomatedPhoneControl_Impl::PhoneHandlingEnabled(this) {
                Ok(ok__) => {
                    core::ptr::write(pfenabled, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAutoEndOfNumberTimeout<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITAutomatedPhoneControl_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ltimeout: i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ITAutomatedPhoneControl_Impl::SetAutoEndOfNumberTimeout(this, core::mem::transmute_copy(&ltimeout)).into()
        }
        unsafe extern "system" fn AutoEndOfNumberTimeout<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITAutomatedPhoneControl_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pltimeout: *mut i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ITAutomatedPhoneControl_Impl::AutoEndOfNumberTimeout(this) {
                Ok(ok__) => {
                    core::ptr::write(pltimeout, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAutoDialtone<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITAutomatedPhoneControl_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, fenabled: super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ITAutomatedPhoneControl_Impl::SetAutoDialtone(this, core::mem::transmute_copy(&fenabled)).into()
        }
        unsafe extern "system" fn AutoDialtone<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITAutomatedPhoneControl_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pfenabled: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ITAutomatedPhoneControl_Impl::AutoDialtone(this) {
                Ok(ok__) => {
                    core::ptr::write(pfenabled, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAutoStopTonesOnOnHook<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITAutomatedPhoneControl_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, fenabled: super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ITAutomatedPhoneControl_Impl::SetAutoStopTonesOnOnHook(this, core::mem::transmute_copy(&fenabled)).into()
        }
        unsafe extern "system" fn AutoStopTonesOnOnHook<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITAutomatedPhoneControl_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pfenabled: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ITAutomatedPhoneControl_Impl::AutoStopTonesOnOnHook(this) {
                Ok(ok__) => {
                    core::ptr::write(pfenabled, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAutoStopRingOnOffHook<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITAutomatedPhoneControl_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, fenabled: super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ITAutomatedPhoneControl_Impl::SetAutoStopRingOnOffHook(this, core::mem::transmute_copy(&fenabled)).into()
        }
        unsafe extern "system" fn AutoStopRingOnOffHook<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITAutomatedPhoneControl_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pfenabled: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ITAutomatedPhoneControl_Impl::AutoStopRingOnOffHook(this) {
                Ok(ok__) => {
                    core::ptr::write(pfenabled, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAutoKeypadTones<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITAutomatedPhoneControl_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, fenabled: super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ITAutomatedPhoneControl_Impl::SetAutoKeypadTones(this, core::mem::transmute_copy(&fenabled)).into()
        }
        unsafe extern "system" fn AutoKeypadTones<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITAutomatedPhoneControl_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pfenabled: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ITAutomatedPhoneControl_Impl::AutoKeypadTones(this) {
                Ok(ok__) => {
                    core::ptr::write(pfenabled, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAutoKeypadTonesMinimumDuration<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITAutomatedPhoneControl_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lduration: i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ITAutomatedPhoneControl_Impl::SetAutoKeypadTonesMinimumDuration(this, core::mem::transmute_copy(&lduration)).into()
        }
        unsafe extern "system" fn AutoKeypadTonesMinimumDuration<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITAutomatedPhoneControl_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, plduration: *mut i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ITAutomatedPhoneControl_Impl::AutoKeypadTonesMinimumDuration(this) {
                Ok(ok__) => {
                    core::ptr::write(plduration, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAutoVolumeControl<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITAutomatedPhoneControl_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, fenabled: super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ITAutomatedPhoneControl_Impl::SetAutoVolumeControl(this, core::mem::transmute_copy(&fenabled)).into()
        }
        unsafe extern "system" fn AutoVolumeControl<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITAutomatedPhoneControl_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, fenabled: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ITAutomatedPhoneControl_Impl::AutoVolumeControl(this) {
                Ok(ok__) => {
                    core::ptr::write(fenabled, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAutoVolumeControlStep<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITAutomatedPhoneControl_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lstepsize: i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ITAutomatedPhoneControl_Impl::SetAutoVolumeControlStep(this, core::mem::transmute_copy(&lstepsize)).into()
        }
        unsafe extern "system" fn AutoVolumeControlStep<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITAutomatedPhoneControl_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, plstepsize: *mut i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ITAutomatedPhoneControl_Impl::AutoVolumeControlStep(this) {
                Ok(ok__) => {
                    core::ptr::write(plstepsize, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAutoVolumeControlRepeatDelay<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITAutomatedPhoneControl_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ldelay: i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ITAutomatedPhoneControl_Impl::SetAutoVolumeControlRepeatDelay(this, core::mem::transmute_copy(&ldelay)).into()
        }
        unsafe extern "system" fn AutoVolumeControlRepeatDelay<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITAutomatedPhoneControl_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pldelay: *mut i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ITAutomatedPhoneControl_Impl::AutoVolumeControlRepeatDelay(this) {
                Ok(ok__) => {
                    core::ptr::write(pldelay, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAutoVolumeControlRepeatPeriod<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITAutomatedPhoneControl_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lperiod: i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ITAutomatedPhoneControl_Impl::SetAutoVolumeControlRepeatPeriod(this, core::mem::transmute_copy(&lperiod)).into()
        }
        unsafe extern "system" fn AutoVolumeControlRepeatPeriod<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITAutomatedPhoneControl_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, plperiod: *mut i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ITAutomatedPhoneControl_Impl::AutoVolumeControlRepeatPeriod(this) {
                Ok(ok__) => {
                    core::ptr::write(plperiod, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SelectCall<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITAutomatedPhoneControl_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcall: *mut core::ffi::c_void, fselectdefaultterminals: super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ITAutomatedPhoneControl_Impl::SelectCall(this, windows_core::from_raw_borrowed(&pcall), core::mem::transmute_copy(&fselectdefaultterminals)).into()
        }
        unsafe extern "system" fn UnselectCall<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITAutomatedPhoneControl_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcall: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ITAutomatedPhoneControl_Impl::UnselectCall(this, windows_core::from_raw_borrowed(&pcall)).into()
        }
        unsafe extern "system" fn EnumerateSelectedCalls<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITAutomatedPhoneControl_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppcallenum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ITAutomatedPhoneControl_Impl::EnumerateSelectedCalls(this) {
                Ok(ok__) => {
                    core::ptr::write(ppcallenum, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SelectedCalls<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITAutomatedPhoneControl_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvariant: *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ITAutomatedPhoneControl_Impl::SelectedCalls(this) {
                Ok(ok__) => {
                    core::ptr::write(pvariant, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            StartTone: StartTone::<Identity, Impl, OFFSET>,
            StopTone: StopTone::<Identity, Impl, OFFSET>,
            Tone: Tone::<Identity, Impl, OFFSET>,
            StartRinger: StartRinger::<Identity, Impl, OFFSET>,
            StopRinger: StopRinger::<Identity, Impl, OFFSET>,
            Ringer: Ringer::<Identity, Impl, OFFSET>,
            SetPhoneHandlingEnabled: SetPhoneHandlingEnabled::<Identity, Impl, OFFSET>,
            PhoneHandlingEnabled: PhoneHandlingEnabled::<Identity, Impl, OFFSET>,
            SetAutoEndOfNumberTimeout: SetAutoEndOfNumberTimeout::<Identity, Impl, OFFSET>,
            AutoEndOfNumberTimeout: AutoEndOfNumberTimeout::<Identity, Impl, OFFSET>,
            SetAutoDialtone: SetAutoDialtone::<Identity, Impl, OFFSET>,
            AutoDialtone: AutoDialtone::<Identity, Impl, OFFSET>,
            SetAutoStopTonesOnOnHook: SetAutoStopTonesOnOnHook::<Identity, Impl, OFFSET>,
            AutoStopTonesOnOnHook: AutoStopTonesOnOnHook::<Identity, Impl, OFFSET>,
            SetAutoStopRingOnOffHook: SetAutoStopRingOnOffHook::<Identity, Impl, OFFSET>,
            AutoStopRingOnOffHook: AutoStopRingOnOffHook::<Identity, Impl, OFFSET>,
            SetAutoKeypadTones: SetAutoKeypadTones::<Identity, Impl, OFFSET>,
            AutoKeypadTones: AutoKeypadTones::<Identity, Impl, OFFSET>,
            SetAutoKeypadTonesMinimumDuration: SetAutoKeypadTonesMinimumDuration::<Identity, Impl, OFFSET>,
            AutoKeypadTonesMinimumDuration: AutoKeypadTonesMinimumDuration::<Identity, Impl, OFFSET>,
            SetAutoVolumeControl: SetAutoVolumeControl::<Identity, Impl, OFFSET>,
            AutoVolumeControl: AutoVolumeControl::<Identity, Impl, OFFSET>,
            SetAutoVolumeControlStep: SetAutoVolumeControlStep::<Identity, Impl, OFFSET>,
            AutoVolumeControlStep: AutoVolumeControlStep::<Identity, Impl, OFFSET>,
            SetAutoVolumeControlRepeatDelay: SetAutoVolumeControlRepeatDelay::<Identity, Impl, OFFSET>,
            AutoVolumeControlRepeatDelay: AutoVolumeControlRepeatDelay::<Identity, Impl, OFFSET>,
            SetAutoVolumeControlRepeatPeriod: SetAutoVolumeControlRepeatPeriod::<Identity, Impl, OFFSET>,
            AutoVolumeControlRepeatPeriod: AutoVolumeControlRepeatPeriod::<Identity, Impl, OFFSET>,
            SelectCall: SelectCall::<Identity, Impl, OFFSET>,
            UnselectCall: UnselectCall::<Identity, Impl, OFFSET>,
            EnumerateSelectedCalls: EnumerateSelectedCalls::<Identity, Impl, OFFSET>,
            SelectedCalls: SelectedCalls::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITAutomatedPhoneControl as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait ITBasicAudioTerminal_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn SetVolume(&self, lvolume: i32) -> windows_core::Result<()>;
    fn Volume(&self) -> windows_core::Result<i32>;
    fn SetBalance(&self, lbalance: i32) -> windows_core::Result<()>;
    fn Balance(&self) -> windows_core::Result<i32>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for ITBasicAudioTerminal {}
#[cfg(feature = "Win32_System_Com")]
impl ITBasicAudioTerminal_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITBasicAudioTerminal_Impl, const OFFSET: isize>() -> ITBasicAudioTerminal_Vtbl {
        unsafe extern "system" fn SetVolume<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITBasicAudioTerminal_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lvolume: i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ITBasicAudioTerminal_Impl::SetVolume(this, core::mem::transmute_copy(&lvolume)).into()
        }
        unsafe extern "system" fn Volume<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITBasicAudioTerminal_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, plvolume: *mut i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ITBasicAudioTerminal_Impl::Volume(this) {
                Ok(ok__) => {
                    core::ptr::write(plvolume, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetBalance<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITBasicAudioTerminal_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lbalance: i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ITBasicAudioTerminal_Impl::SetBalance(this, core::mem::transmute_copy(&lbalance)).into()
        }
        unsafe extern "system" fn Balance<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITBasicAudioTerminal_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, plbalance: *mut i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ITBasicAudioTerminal_Impl::Balance(this) {
                Ok(ok__) => {
                    core::ptr::write(plbalance, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            SetVolume: SetVolume::<Identity, Impl, OFFSET>,
            Volume: Volume::<Identity, Impl, OFFSET>,
            SetBalance: SetBalance::<Identity, Impl, OFFSET>,
            Balance: Balance::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITBasicAudioTerminal as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait ITBasicCallControl_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn Connect(&self, fsync: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
    fn Answer(&self) -> windows_core::Result<()>;
    fn Disconnect(&self, code: DISCONNECT_CODE) -> windows_core::Result<()>;
    fn Hold(&self, fhold: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
    fn HandoffDirect(&self, papplicationname: &windows_core::BSTR) -> windows_core::Result<()>;
    fn HandoffIndirect(&self, lmediatype: i32) -> windows_core::Result<()>;
    fn Conference(&self, pcall: Option<&ITBasicCallControl>, fsync: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
    fn Transfer(&self, pcall: Option<&ITBasicCallControl>, fsync: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
    fn BlindTransfer(&self, pdestaddress: &windows_core::BSTR) -> windows_core::Result<()>;
    fn SwapHold(&self, pcall: Option<&ITBasicCallControl>) -> windows_core::Result<()>;
    fn ParkDirect(&self, pparkaddress: &windows_core::BSTR) -> windows_core::Result<()>;
    fn ParkIndirect(&self) -> windows_core::Result<windows_core::BSTR>;
    fn Unpark(&self) -> windows_core::Result<()>;
    fn SetQOS(&self, lmediatype: i32, servicelevel: QOS_SERVICE_LEVEL) -> windows_core::Result<()>;
    fn Pickup(&self, pgroupid: &windows_core::BSTR) -> windows_core::Result<()>;
    fn Dial(&self, pdestaddress: &windows_core::BSTR) -> windows_core::Result<()>;
    fn Finish(&self, finishmode: FINISH_MODE) -> windows_core::Result<()>;
    fn RemoveFromConference(&self) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for ITBasicCallControl {}
#[cfg(feature = "Win32_System_Com")]
impl ITBasicCallControl_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITBasicCallControl_Impl, const OFFSET: isize>() -> ITBasicCallControl_Vtbl {
        unsafe extern "system" fn Connect<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITBasicCallControl_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, fsync: super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ITBasicCallControl_Impl::Connect(this, core::mem::transmute_copy(&fsync)).into()
        }
        unsafe extern "system" fn Answer<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITBasicCallControl_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ITBasicCallControl_Impl::Answer(this).into()
        }
        unsafe extern "system" fn Disconnect<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITBasicCallControl_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, code: DISCONNECT_CODE) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ITBasicCallControl_Impl::Disconnect(this, core::mem::transmute_copy(&code)).into()
        }
        unsafe extern "system" fn Hold<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITBasicCallControl_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, fhold: super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ITBasicCallControl_Impl::Hold(this, core::mem::transmute_copy(&fhold)).into()
        }
        unsafe extern "system" fn HandoffDirect<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITBasicCallControl_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, papplicationname: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ITBasicCallControl_Impl::HandoffDirect(this, core::mem::transmute(&papplicationname)).into()
        }
        unsafe extern "system" fn HandoffIndirect<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITBasicCallControl_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lmediatype: i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ITBasicCallControl_Impl::HandoffIndirect(this, core::mem::transmute_copy(&lmediatype)).into()
        }
        unsafe extern "system" fn Conference<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITBasicCallControl_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcall: *mut core::ffi::c_void, fsync: super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ITBasicCallControl_Impl::Conference(this, windows_core::from_raw_borrowed(&pcall), core::mem::transmute_copy(&fsync)).into()
        }
        unsafe extern "system" fn Transfer<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITBasicCallControl_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcall: *mut core::ffi::c_void, fsync: super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ITBasicCallControl_Impl::Transfer(this, windows_core::from_raw_borrowed(&pcall), core::mem::transmute_copy(&fsync)).into()
        }
        unsafe extern "system" fn BlindTransfer<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITBasicCallControl_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdestaddress: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ITBasicCallControl_Impl::BlindTransfer(this, core::mem::transmute(&pdestaddress)).into()
        }
        unsafe extern "system" fn SwapHold<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITBasicCallControl_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcall: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ITBasicCallControl_Impl::SwapHold(this, windows_core::from_raw_borrowed(&pcall)).into()
        }
        unsafe extern "system" fn ParkDirect<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITBasicCallControl_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pparkaddress: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ITBasicCallControl_Impl::ParkDirect(this, core::mem::transmute(&pparkaddress)).into()
        }
        unsafe extern "system" fn ParkIndirect<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITBasicCallControl_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppnondiraddress: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ITBasicCallControl_Impl::ParkIndirect(this) {
                Ok(ok__) => {
                    core::ptr::write(ppnondiraddress, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Unpark<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITBasicCallControl_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ITBasicCallControl_Impl::Unpark(this).into()
        }
        unsafe extern "system" fn SetQOS<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITBasicCallControl_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lmediatype: i32, servicelevel: QOS_SERVICE_LEVEL) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ITBasicCallControl_Impl::SetQOS(this, core::mem::transmute_copy(&lmediatype), core::mem::transmute_copy(&servicelevel)).into()
        }
        unsafe extern "system" fn Pickup<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITBasicCallControl_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pgroupid: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ITBasicCallControl_Impl::Pickup(this, core::mem::transmute(&pgroupid)).into()
        }
        unsafe extern "system" fn Dial<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITBasicCallControl_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdestaddress: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ITBasicCallControl_Impl::Dial(this, core::mem::transmute(&pdestaddress)).into()
        }
        unsafe extern "system" fn Finish<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITBasicCallControl_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, finishmode: FINISH_MODE) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ITBasicCallControl_Impl::Finish(this, core::mem::transmute_copy(&finishmode)).into()
        }
        unsafe extern "system" fn RemoveFromConference<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITBasicCallControl_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ITBasicCallControl_Impl::RemoveFromConference(this).into()
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            Connect: Connect::<Identity, Impl, OFFSET>,
            Answer: Answer::<Identity, Impl, OFFSET>,
            Disconnect: Disconnect::<Identity, Impl, OFFSET>,
            Hold: Hold::<Identity, Impl, OFFSET>,
            HandoffDirect: HandoffDirect::<Identity, Impl, OFFSET>,
            HandoffIndirect: HandoffIndirect::<Identity, Impl, OFFSET>,
            Conference: Conference::<Identity, Impl, OFFSET>,
            Transfer: Transfer::<Identity, Impl, OFFSET>,
            BlindTransfer: BlindTransfer::<Identity, Impl, OFFSET>,
            SwapHold: SwapHold::<Identity, Impl, OFFSET>,
            ParkDirect: ParkDirect::<Identity, Impl, OFFSET>,
            ParkIndirect: ParkIndirect::<Identity, Impl, OFFSET>,
            Unpark: Unpark::<Identity, Impl, OFFSET>,
            SetQOS: SetQOS::<Identity, Impl, OFFSET>,
            Pickup: Pickup::<Identity, Impl, OFFSET>,
            Dial: Dial::<Identity, Impl, OFFSET>,
            Finish: Finish::<Identity, Impl, OFFSET>,
            RemoveFromConference: RemoveFromConference::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITBasicCallControl as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait ITBasicCallControl2_Impl: Sized + ITBasicCallControl_Impl {
    fn RequestTerminal(&self, bstrterminalclassguid: &windows_core::BSTR, lmediatype: i32, direction: TERMINAL_DIRECTION) -> windows_core::Result<ITTerminal>;
    fn SelectTerminalOnCall(&self, pterminal: Option<&ITTerminal>) -> windows_core::Result<()>;
    fn UnselectTerminalOnCall(&self, pterminal: Option<&ITTerminal>) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for ITBasicCallControl2 {}
#[cfg(feature = "Win32_System_Com")]
impl ITBasicCallControl2_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITBasicCallControl2_Impl, const OFFSET: isize>() -> ITBasicCallControl2_Vtbl {
        unsafe extern "system" fn RequestTerminal<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITBasicCallControl2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrterminalclassguid: core::mem::MaybeUninit<windows_core::BSTR>, lmediatype: i32, direction: TERMINAL_DIRECTION, ppterminal: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ITBasicCallControl2_Impl::RequestTerminal(this, core::mem::transmute(&bstrterminalclassguid), core::mem::transmute_copy(&lmediatype), core::mem::transmute_copy(&direction)) {
                Ok(ok__) => {
                    core::ptr::write(ppterminal, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SelectTerminalOnCall<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITBasicCallControl2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pterminal: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ITBasicCallControl2_Impl::SelectTerminalOnCall(this, windows_core::from_raw_borrowed(&pterminal)).into()
        }
        unsafe extern "system" fn UnselectTerminalOnCall<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITBasicCallControl2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pterminal: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ITBasicCallControl2_Impl::UnselectTerminalOnCall(this, windows_core::from_raw_borrowed(&pterminal)).into()
        }
        Self {
            base__: ITBasicCallControl_Vtbl::new::<Identity, Impl, OFFSET>(),
            RequestTerminal: RequestTerminal::<Identity, Impl, OFFSET>,
            SelectTerminalOnCall: SelectTerminalOnCall::<Identity, Impl, OFFSET>,
            UnselectTerminalOnCall: UnselectTerminalOnCall::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITBasicCallControl2 as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID || iid == &<ITBasicCallControl as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait ITCallHub_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn Clear(&self) -> windows_core::Result<()>;
    fn EnumerateCalls(&self) -> windows_core::Result<IEnumCall>;
    fn Calls(&self) -> windows_core::Result<windows_core::VARIANT>;
    fn NumCalls(&self) -> windows_core::Result<i32>;
    fn State(&self) -> windows_core::Result<CALLHUB_STATE>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for ITCallHub {}
#[cfg(feature = "Win32_System_Com")]
impl ITCallHub_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITCallHub_Impl, const OFFSET: isize>() -> ITCallHub_Vtbl {
        unsafe extern "system" fn Clear<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITCallHub_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ITCallHub_Impl::Clear(this).into()
        }
        unsafe extern "system" fn EnumerateCalls<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITCallHub_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppenumcall: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ITCallHub_Impl::EnumerateCalls(this) {
                Ok(ok__) => {
                    core::ptr::write(ppenumcall, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Calls<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITCallHub_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcalls: *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ITCallHub_Impl::Calls(this) {
                Ok(ok__) => {
                    core::ptr::write(pcalls, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn NumCalls<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITCallHub_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, plcalls: *mut i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ITCallHub_Impl::NumCalls(this) {
                Ok(ok__) => {
                    core::ptr::write(plcalls, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn State<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITCallHub_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pstate: *mut CALLHUB_STATE) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ITCallHub_Impl::State(this) {
                Ok(ok__) => {
                    core::ptr::write(pstate, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            Clear: Clear::<Identity, Impl, OFFSET>,
            EnumerateCalls: EnumerateCalls::<Identity, Impl, OFFSET>,
            Calls: Calls::<Identity, Impl, OFFSET>,
            NumCalls: NumCalls::<Identity, Impl, OFFSET>,
            State: State::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITCallHub as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait ITCallHubEvent_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn Event(&self) -> windows_core::Result<CALLHUB_EVENT>;
    fn CallHub(&self) -> windows_core::Result<ITCallHub>;
    fn Call(&self) -> windows_core::Result<ITCallInfo>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for ITCallHubEvent {}
#[cfg(feature = "Win32_System_Com")]
impl ITCallHubEvent_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITCallHubEvent_Impl, const OFFSET: isize>() -> ITCallHubEvent_Vtbl {
        unsafe extern "system" fn Event<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITCallHubEvent_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pevent: *mut CALLHUB_EVENT) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ITCallHubEvent_Impl::Event(this) {
                Ok(ok__) => {
                    core::ptr::write(pevent, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CallHub<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITCallHubEvent_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppcallhub: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ITCallHubEvent_Impl::CallHub(this) {
                Ok(ok__) => {
                    core::ptr::write(ppcallhub, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Call<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITCallHubEvent_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppcall: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ITCallHubEvent_Impl::Call(this) {
                Ok(ok__) => {
                    core::ptr::write(ppcall, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            Event: Event::<Identity, Impl, OFFSET>,
            CallHub: CallHub::<Identity, Impl, OFFSET>,
            Call: Call::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITCallHubEvent as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait ITCallInfo_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn Address(&self) -> windows_core::Result<ITAddress>;
    fn CallState(&self) -> windows_core::Result<CALL_STATE>;
    fn Privilege(&self) -> windows_core::Result<CALL_PRIVILEGE>;
    fn CallHub(&self) -> windows_core::Result<ITCallHub>;
    fn get_CallInfoLong(&self, callinfolong: CALLINFO_LONG) -> windows_core::Result<i32>;
    fn put_CallInfoLong(&self, callinfolong: CALLINFO_LONG, lcallinfolongval: i32) -> windows_core::Result<()>;
    fn get_CallInfoString(&self, callinfostring: CALLINFO_STRING) -> windows_core::Result<windows_core::BSTR>;
    fn put_CallInfoString(&self, callinfostring: CALLINFO_STRING, pcallinfostring: &windows_core::BSTR) -> windows_core::Result<()>;
    fn get_CallInfoBuffer(&self, callinfobuffer: CALLINFO_BUFFER) -> windows_core::Result<windows_core::VARIANT>;
    fn put_CallInfoBuffer(&self, callinfobuffer: CALLINFO_BUFFER, pcallinfobuffer: &windows_core::VARIANT) -> windows_core::Result<()>;
    fn GetCallInfoBuffer(&self, callinfobuffer: CALLINFO_BUFFER, pdwsize: *mut u32, ppcallinfobuffer: *mut *mut u8) -> windows_core::Result<()>;
    fn SetCallInfoBuffer(&self, callinfobuffer: CALLINFO_BUFFER, dwsize: u32, pcallinfobuffer: *const u8) -> windows_core::Result<()>;
    fn ReleaseUserUserInfo(&self) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for ITCallInfo {}
#[cfg(feature = "Win32_System_Com")]
impl ITCallInfo_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITCallInfo_Impl, const OFFSET: isize>() -> ITCallInfo_Vtbl {
        unsafe extern "system" fn Address<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITCallInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppaddress: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ITCallInfo_Impl::Address(this) {
                Ok(ok__) => {
                    core::ptr::write(ppaddress, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CallState<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITCallInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcallstate: *mut CALL_STATE) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ITCallInfo_Impl::CallState(this) {
                Ok(ok__) => {
                    core::ptr::write(pcallstate, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Privilege<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITCallInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pprivilege: *mut CALL_PRIVILEGE) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ITCallInfo_Impl::Privilege(this) {
                Ok(ok__) => {
                    core::ptr::write(pprivilege, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CallHub<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITCallInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppcallhub: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ITCallInfo_Impl::CallHub(this) {
                Ok(ok__) => {
                    core::ptr::write(ppcallhub, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn get_CallInfoLong<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITCallInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, callinfolong: CALLINFO_LONG, plcallinfolongval: *mut i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ITCallInfo_Impl::get_CallInfoLong(this, core::mem::transmute_copy(&callinfolong)) {
                Ok(ok__) => {
                    core::ptr::write(plcallinfolongval, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn put_CallInfoLong<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITCallInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, callinfolong: CALLINFO_LONG, lcallinfolongval: i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ITCallInfo_Impl::put_CallInfoLong(this, core::mem::transmute_copy(&callinfolong), core::mem::transmute_copy(&lcallinfolongval)).into()
        }
        unsafe extern "system" fn get_CallInfoString<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITCallInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, callinfostring: CALLINFO_STRING, ppcallinfostring: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ITCallInfo_Impl::get_CallInfoString(this, core::mem::transmute_copy(&callinfostring)) {
                Ok(ok__) => {
                    core::ptr::write(ppcallinfostring, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn put_CallInfoString<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITCallInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, callinfostring: CALLINFO_STRING, pcallinfostring: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ITCallInfo_Impl::put_CallInfoString(this, core::mem::transmute_copy(&callinfostring), core::mem::transmute(&pcallinfostring)).into()
        }
        unsafe extern "system" fn get_CallInfoBuffer<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITCallInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, callinfobuffer: CALLINFO_BUFFER, ppcallinfobuffer: *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ITCallInfo_Impl::get_CallInfoBuffer(this, core::mem::transmute_copy(&callinfobuffer)) {
                Ok(ok__) => {
                    core::ptr::write(ppcallinfobuffer, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn put_CallInfoBuffer<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITCallInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, callinfobuffer: CALLINFO_BUFFER, pcallinfobuffer: core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ITCallInfo_Impl::put_CallInfoBuffer(this, core::mem::transmute_copy(&callinfobuffer), core::mem::transmute(&pcallinfobuffer)).into()
        }
        unsafe extern "system" fn GetCallInfoBuffer<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITCallInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, callinfobuffer: CALLINFO_BUFFER, pdwsize: *mut u32, ppcallinfobuffer: *mut *mut u8) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ITCallInfo_Impl::GetCallInfoBuffer(this, core::mem::transmute_copy(&callinfobuffer), core::mem::transmute_copy(&pdwsize), core::mem::transmute_copy(&ppcallinfobuffer)).into()
        }
        unsafe extern "system" fn SetCallInfoBuffer<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITCallInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, callinfobuffer: CALLINFO_BUFFER, dwsize: u32, pcallinfobuffer: *const u8) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ITCallInfo_Impl::SetCallInfoBuffer(this, core::mem::transmute_copy(&callinfobuffer), core::mem::transmute_copy(&dwsize), core::mem::transmute_copy(&pcallinfobuffer)).into()
        }
        unsafe extern "system" fn ReleaseUserUserInfo<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITCallInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ITCallInfo_Impl::ReleaseUserUserInfo(this).into()
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            Address: Address::<Identity, Impl, OFFSET>,
            CallState: CallState::<Identity, Impl, OFFSET>,
            Privilege: Privilege::<Identity, Impl, OFFSET>,
            CallHub: CallHub::<Identity, Impl, OFFSET>,
            get_CallInfoLong: get_CallInfoLong::<Identity, Impl, OFFSET>,
            put_CallInfoLong: put_CallInfoLong::<Identity, Impl, OFFSET>,
            get_CallInfoString: get_CallInfoString::<Identity, Impl, OFFSET>,
            put_CallInfoString: put_CallInfoString::<Identity, Impl, OFFSET>,
            get_CallInfoBuffer: get_CallInfoBuffer::<Identity, Impl, OFFSET>,
            put_CallInfoBuffer: put_CallInfoBuffer::<Identity, Impl, OFFSET>,
            GetCallInfoBuffer: GetCallInfoBuffer::<Identity, Impl, OFFSET>,
            SetCallInfoBuffer: SetCallInfoBuffer::<Identity, Impl, OFFSET>,
            ReleaseUserUserInfo: ReleaseUserUserInfo::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITCallInfo as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait ITCallInfo2_Impl: Sized + ITCallInfo_Impl {
    fn get_EventFilter(&self, tapievent: TAPI_EVENT, lsubevent: i32) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn put_EventFilter(&self, tapievent: TAPI_EVENT, lsubevent: i32, benable: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for ITCallInfo2 {}
#[cfg(feature = "Win32_System_Com")]
impl ITCallInfo2_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITCallInfo2_Impl, const OFFSET: isize>() -> ITCallInfo2_Vtbl {
        unsafe extern "system" fn get_EventFilter<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITCallInfo2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, tapievent: TAPI_EVENT, lsubevent: i32, penable: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ITCallInfo2_Impl::get_EventFilter(this, core::mem::transmute_copy(&tapievent), core::mem::transmute_copy(&lsubevent)) {
                Ok(ok__) => {
                    core::ptr::write(penable, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn put_EventFilter<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITCallInfo2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, tapievent: TAPI_EVENT, lsubevent: i32, benable: super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ITCallInfo2_Impl::put_EventFilter(this, core::mem::transmute_copy(&tapievent), core::mem::transmute_copy(&lsubevent), core::mem::transmute_copy(&benable)).into()
        }
        Self {
            base__: ITCallInfo_Vtbl::new::<Identity, Impl, OFFSET>(),
            get_EventFilter: get_EventFilter::<Identity, Impl, OFFSET>,
            put_EventFilter: put_EventFilter::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITCallInfo2 as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID || iid == &<ITCallInfo as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait ITCallInfoChangeEvent_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn Call(&self) -> windows_core::Result<ITCallInfo>;
    fn Cause(&self) -> windows_core::Result<CALLINFOCHANGE_CAUSE>;
    fn CallbackInstance(&self) -> windows_core::Result<i32>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for ITCallInfoChangeEvent {}
#[cfg(feature = "Win32_System_Com")]
impl ITCallInfoChangeEvent_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITCallInfoChangeEvent_Impl, const OFFSET: isize>() -> ITCallInfoChangeEvent_Vtbl {
        unsafe extern "system" fn Call<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITCallInfoChangeEvent_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppcall: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ITCallInfoChangeEvent_Impl::Call(this) {
                Ok(ok__) => {
                    core::ptr::write(ppcall, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Cause<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITCallInfoChangeEvent_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcic: *mut CALLINFOCHANGE_CAUSE) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ITCallInfoChangeEvent_Impl::Cause(this) {
                Ok(ok__) => {
                    core::ptr::write(pcic, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CallbackInstance<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITCallInfoChangeEvent_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, plcallbackinstance: *mut i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ITCallInfoChangeEvent_Impl::CallbackInstance(this) {
                Ok(ok__) => {
                    core::ptr::write(plcallbackinstance, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            Call: Call::<Identity, Impl, OFFSET>,
            Cause: Cause::<Identity, Impl, OFFSET>,
            CallbackInstance: CallbackInstance::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITCallInfoChangeEvent as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait ITCallMediaEvent_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn Call(&self) -> windows_core::Result<ITCallInfo>;
    fn Event(&self) -> windows_core::Result<CALL_MEDIA_EVENT>;
    fn Error(&self) -> windows_core::Result<windows_core::HRESULT>;
    fn Terminal(&self) -> windows_core::Result<ITTerminal>;
    fn Stream(&self) -> windows_core::Result<ITStream>;
    fn Cause(&self) -> windows_core::Result<CALL_MEDIA_EVENT_CAUSE>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for ITCallMediaEvent {}
#[cfg(feature = "Win32_System_Com")]
impl ITCallMediaEvent_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITCallMediaEvent_Impl, const OFFSET: isize>() -> ITCallMediaEvent_Vtbl {
        unsafe extern "system" fn Call<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITCallMediaEvent_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppcallinfo: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ITCallMediaEvent_Impl::Call(this) {
                Ok(ok__) => {
                    core::ptr::write(ppcallinfo, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Event<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITCallMediaEvent_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcallmediaevent: *mut CALL_MEDIA_EVENT) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ITCallMediaEvent_Impl::Event(this) {
                Ok(ok__) => {
                    core::ptr::write(pcallmediaevent, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Error<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITCallMediaEvent_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, phrerror: *mut windows_core::HRESULT) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ITCallMediaEvent_Impl::Error(this) {
                Ok(ok__) => {
                    core::ptr::write(phrerror, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Terminal<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITCallMediaEvent_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppterminal: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ITCallMediaEvent_Impl::Terminal(this) {
                Ok(ok__) => {
                    core::ptr::write(ppterminal, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Stream<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITCallMediaEvent_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppstream: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ITCallMediaEvent_Impl::Stream(this) {
                Ok(ok__) => {
                    core::ptr::write(ppstream, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Cause<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITCallMediaEvent_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcause: *mut CALL_MEDIA_EVENT_CAUSE) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ITCallMediaEvent_Impl::Cause(this) {
                Ok(ok__) => {
                    core::ptr::write(pcause, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            Call: Call::<Identity, Impl, OFFSET>,
            Event: Event::<Identity, Impl, OFFSET>,
            Error: Error::<Identity, Impl, OFFSET>,
            Terminal: Terminal::<Identity, Impl, OFFSET>,
            Stream: Stream::<Identity, Impl, OFFSET>,
            Cause: Cause::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITCallMediaEvent as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait ITCallNotificationEvent_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn Call(&self) -> windows_core::Result<ITCallInfo>;
    fn Event(&self) -> windows_core::Result<CALL_NOTIFICATION_EVENT>;
    fn CallbackInstance(&self) -> windows_core::Result<i32>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for ITCallNotificationEvent {}
#[cfg(feature = "Win32_System_Com")]
impl ITCallNotificationEvent_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITCallNotificationEvent_Impl, const OFFSET: isize>() -> ITCallNotificationEvent_Vtbl {
        unsafe extern "system" fn Call<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITCallNotificationEvent_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppcall: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ITCallNotificationEvent_Impl::Call(this) {
                Ok(ok__) => {
                    core::ptr::write(ppcall, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Event<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITCallNotificationEvent_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcallnotificationevent: *mut CALL_NOTIFICATION_EVENT) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ITCallNotificationEvent_Impl::Event(this) {
                Ok(ok__) => {
                    core::ptr::write(pcallnotificationevent, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CallbackInstance<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITCallNotificationEvent_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, plcallbackinstance: *mut i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ITCallNotificationEvent_Impl::CallbackInstance(this) {
                Ok(ok__) => {
                    core::ptr::write(plcallbackinstance, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            Call: Call::<Identity, Impl, OFFSET>,
            Event: Event::<Identity, Impl, OFFSET>,
            CallbackInstance: CallbackInstance::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITCallNotificationEvent as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait ITCallStateEvent_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn Call(&self) -> windows_core::Result<ITCallInfo>;
    fn State(&self) -> windows_core::Result<CALL_STATE>;
    fn Cause(&self) -> windows_core::Result<CALL_STATE_EVENT_CAUSE>;
    fn CallbackInstance(&self) -> windows_core::Result<i32>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for ITCallStateEvent {}
#[cfg(feature = "Win32_System_Com")]
impl ITCallStateEvent_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITCallStateEvent_Impl, const OFFSET: isize>() -> ITCallStateEvent_Vtbl {
        unsafe extern "system" fn Call<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITCallStateEvent_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppcallinfo: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ITCallStateEvent_Impl::Call(this) {
                Ok(ok__) => {
                    core::ptr::write(ppcallinfo, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn State<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITCallStateEvent_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcallstate: *mut CALL_STATE) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ITCallStateEvent_Impl::State(this) {
                Ok(ok__) => {
                    core::ptr::write(pcallstate, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Cause<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITCallStateEvent_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcec: *mut CALL_STATE_EVENT_CAUSE) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ITCallStateEvent_Impl::Cause(this) {
                Ok(ok__) => {
                    core::ptr::write(pcec, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CallbackInstance<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITCallStateEvent_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, plcallbackinstance: *mut i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ITCallStateEvent_Impl::CallbackInstance(this) {
                Ok(ok__) => {
                    core::ptr::write(plcallbackinstance, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            Call: Call::<Identity, Impl, OFFSET>,
            State: State::<Identity, Impl, OFFSET>,
            Cause: Cause::<Identity, Impl, OFFSET>,
            CallbackInstance: CallbackInstance::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITCallStateEvent as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait ITCallingCard_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn PermanentCardID(&self) -> windows_core::Result<i32>;
    fn NumberOfDigits(&self) -> windows_core::Result<i32>;
    fn Options(&self) -> windows_core::Result<i32>;
    fn CardName(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SameAreaDialingRule(&self) -> windows_core::Result<windows_core::BSTR>;
    fn LongDistanceDialingRule(&self) -> windows_core::Result<windows_core::BSTR>;
    fn InternationalDialingRule(&self) -> windows_core::Result<windows_core::BSTR>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for ITCallingCard {}
#[cfg(feature = "Win32_System_Com")]
impl ITCallingCard_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITCallingCard_Impl, const OFFSET: isize>() -> ITCallingCard_Vtbl {
        unsafe extern "system" fn PermanentCardID<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITCallingCard_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, plcardid: *mut i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ITCallingCard_Impl::PermanentCardID(this) {
                Ok(ok__) => {
                    core::ptr::write(plcardid, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn NumberOfDigits<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITCallingCard_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pldigits: *mut i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ITCallingCard_Impl::NumberOfDigits(this) {
                Ok(ok__) => {
                    core::ptr::write(pldigits, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Options<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITCallingCard_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ploptions: *mut i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ITCallingCard_Impl::Options(this) {
                Ok(ok__) => {
                    core::ptr::write(ploptions, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CardName<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITCallingCard_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppcardname: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ITCallingCard_Impl::CardName(this) {
                Ok(ok__) => {
                    core::ptr::write(ppcardname, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SameAreaDialingRule<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITCallingCard_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pprule: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ITCallingCard_Impl::SameAreaDialingRule(this) {
                Ok(ok__) => {
                    core::ptr::write(pprule, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LongDistanceDialingRule<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITCallingCard_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pprule: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ITCallingCard_Impl::LongDistanceDialingRule(this) {
                Ok(ok__) => {
                    core::ptr::write(pprule, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn InternationalDialingRule<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITCallingCard_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pprule: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ITCallingCard_Impl::InternationalDialingRule(this) {
                Ok(ok__) => {
                    core::ptr::write(pprule, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            PermanentCardID: PermanentCardID::<Identity, Impl, OFFSET>,
            NumberOfDigits: NumberOfDigits::<Identity, Impl, OFFSET>,
            Options: Options::<Identity, Impl, OFFSET>,
            CardName: CardName::<Identity, Impl, OFFSET>,
            SameAreaDialingRule: SameAreaDialingRule::<Identity, Impl, OFFSET>,
            LongDistanceDialingRule: LongDistanceDialingRule::<Identity, Impl, OFFSET>,
            InternationalDialingRule: InternationalDialingRule::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITCallingCard as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait ITCollection_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn Count(&self) -> windows_core::Result<i32>;
    fn get_Item(&self, index: i32) -> windows_core::Result<windows_core::VARIANT>;
    fn _NewEnum(&self) -> windows_core::Result<windows_core::IUnknown>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for ITCollection {}
#[cfg(feature = "Win32_System_Com")]
impl ITCollection_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITCollection_Impl, const OFFSET: isize>() -> ITCollection_Vtbl {
        unsafe extern "system" fn Count<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lcount: *mut i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ITCollection_Impl::Count(this) {
                Ok(ok__) => {
                    core::ptr::write(lcount, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn get_Item<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: i32, pvariant: *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ITCollection_Impl::get_Item(this, core::mem::transmute_copy(&index)) {
                Ok(ok__) => {
                    core::ptr::write(pvariant, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn _NewEnum<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppnewenum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ITCollection_Impl::_NewEnum(this) {
                Ok(ok__) => {
                    core::ptr::write(ppnewenum, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            Count: Count::<Identity, Impl, OFFSET>,
            get_Item: get_Item::<Identity, Impl, OFFSET>,
            _NewEnum: _NewEnum::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITCollection as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait ITCollection2_Impl: Sized + ITCollection_Impl {
    fn Add(&self, index: i32, pvariant: *const windows_core::VARIANT) -> windows_core::Result<()>;
    fn Remove(&self, index: i32) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for ITCollection2 {}
#[cfg(feature = "Win32_System_Com")]
impl ITCollection2_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITCollection2_Impl, const OFFSET: isize>() -> ITCollection2_Vtbl {
        unsafe extern "system" fn Add<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITCollection2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: i32, pvariant: *const core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ITCollection2_Impl::Add(this, core::mem::transmute_copy(&index), core::mem::transmute_copy(&pvariant)).into()
        }
        unsafe extern "system" fn Remove<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITCollection2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ITCollection2_Impl::Remove(this, core::mem::transmute_copy(&index)).into()
        }
        Self { base__: ITCollection_Vtbl::new::<Identity, Impl, OFFSET>(), Add: Add::<Identity, Impl, OFFSET>, Remove: Remove::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITCollection2 as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID || iid == &<ITCollection as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait ITCustomTone_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn Frequency(&self) -> windows_core::Result<i32>;
    fn SetFrequency(&self, lfrequency: i32) -> windows_core::Result<()>;
    fn CadenceOn(&self) -> windows_core::Result<i32>;
    fn SetCadenceOn(&self, cadenceon: i32) -> windows_core::Result<()>;
    fn CadenceOff(&self) -> windows_core::Result<i32>;
    fn SetCadenceOff(&self, lcadenceoff: i32) -> windows_core::Result<()>;
    fn Volume(&self) -> windows_core::Result<i32>;
    fn SetVolume(&self, lvolume: i32) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for ITCustomTone {}
#[cfg(feature = "Win32_System_Com")]
impl ITCustomTone_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITCustomTone_Impl, const OFFSET: isize>() -> ITCustomTone_Vtbl {
        unsafe extern "system" fn Frequency<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITCustomTone_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, plfrequency: *mut i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ITCustomTone_Impl::Frequency(this) {
                Ok(ok__) => {
                    core::ptr::write(plfrequency, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetFrequency<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITCustomTone_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lfrequency: i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ITCustomTone_Impl::SetFrequency(this, core::mem::transmute_copy(&lfrequency)).into()
        }
        unsafe extern "system" fn CadenceOn<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITCustomTone_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, plcadenceon: *mut i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ITCustomTone_Impl::CadenceOn(this) {
                Ok(ok__) => {
                    core::ptr::write(plcadenceon, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCadenceOn<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITCustomTone_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, cadenceon: i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ITCustomTone_Impl::SetCadenceOn(this, core::mem::transmute_copy(&cadenceon)).into()
        }
        unsafe extern "system" fn CadenceOff<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITCustomTone_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, plcadenceoff: *mut i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ITCustomTone_Impl::CadenceOff(this) {
                Ok(ok__) => {
                    core::ptr::write(plcadenceoff, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCadenceOff<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITCustomTone_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lcadenceoff: i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ITCustomTone_Impl::SetCadenceOff(this, core::mem::transmute_copy(&lcadenceoff)).into()
        }
        unsafe extern "system" fn Volume<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITCustomTone_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, plvolume: *mut i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ITCustomTone_Impl::Volume(this) {
                Ok(ok__) => {
                    core::ptr::write(plvolume, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetVolume<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITCustomTone_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lvolume: i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ITCustomTone_Impl::SetVolume(this, core::mem::transmute_copy(&lvolume)).into()
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            Frequency: Frequency::<Identity, Impl, OFFSET>,
            SetFrequency: SetFrequency::<Identity, Impl, OFFSET>,
            CadenceOn: CadenceOn::<Identity, Impl, OFFSET>,
            SetCadenceOn: SetCadenceOn::<Identity, Impl, OFFSET>,
            CadenceOff: CadenceOff::<Identity, Impl, OFFSET>,
            SetCadenceOff: SetCadenceOff::<Identity, Impl, OFFSET>,
            Volume: Volume::<Identity, Impl, OFFSET>,
            SetVolume: SetVolume::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITCustomTone as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait ITDetectTone_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn AppSpecific(&self) -> windows_core::Result<i32>;
    fn SetAppSpecific(&self, lappspecific: i32) -> windows_core::Result<()>;
    fn Duration(&self) -> windows_core::Result<i32>;
    fn SetDuration(&self, lduration: i32) -> windows_core::Result<()>;
    fn get_Frequency(&self, index: i32) -> windows_core::Result<i32>;
    fn put_Frequency(&self, index: i32, lfrequency: i32) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for ITDetectTone {}
#[cfg(feature = "Win32_System_Com")]
impl ITDetectTone_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITDetectTone_Impl, const OFFSET: isize>() -> ITDetectTone_Vtbl {
        unsafe extern "system" fn AppSpecific<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITDetectTone_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, plappspecific: *mut i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ITDetectTone_Impl::AppSpecific(this) {
                Ok(ok__) => {
                    core::ptr::write(plappspecific, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAppSpecific<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITDetectTone_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lappspecific: i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ITDetectTone_Impl::SetAppSpecific(this, core::mem::transmute_copy(&lappspecific)).into()
        }
        unsafe extern "system" fn Duration<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITDetectTone_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, plduration: *mut i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ITDetectTone_Impl::Duration(this) {
                Ok(ok__) => {
                    core::ptr::write(plduration, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDuration<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITDetectTone_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lduration: i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ITDetectTone_Impl::SetDuration(this, core::mem::transmute_copy(&lduration)).into()
        }
        unsafe extern "system" fn get_Frequency<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITDetectTone_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: i32, plfrequency: *mut i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ITDetectTone_Impl::get_Frequency(this, core::mem::transmute_copy(&index)) {
                Ok(ok__) => {
                    core::ptr::write(plfrequency, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn put_Frequency<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITDetectTone_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: i32, lfrequency: i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ITDetectTone_Impl::put_Frequency(this, core::mem::transmute_copy(&index), core::mem::transmute_copy(&lfrequency)).into()
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            AppSpecific: AppSpecific::<Identity, Impl, OFFSET>,
            SetAppSpecific: SetAppSpecific::<Identity, Impl, OFFSET>,
            Duration: Duration::<Identity, Impl, OFFSET>,
            SetDuration: SetDuration::<Identity, Impl, OFFSET>,
            get_Frequency: get_Frequency::<Identity, Impl, OFFSET>,
            put_Frequency: put_Frequency::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITDetectTone as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait ITDigitDetectionEvent_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn Call(&self) -> windows_core::Result<ITCallInfo>;
    fn Digit(&self) -> windows_core::Result<u8>;
    fn DigitMode(&self) -> windows_core::Result<i32>;
    fn TickCount(&self) -> windows_core::Result<i32>;
    fn CallbackInstance(&self) -> windows_core::Result<i32>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for ITDigitDetectionEvent {}
#[cfg(feature = "Win32_System_Com")]
impl ITDigitDetectionEvent_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITDigitDetectionEvent_Impl, const OFFSET: isize>() -> ITDigitDetectionEvent_Vtbl {
        unsafe extern "system" fn Call<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITDigitDetectionEvent_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppcallinfo: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ITDigitDetectionEvent_Impl::Call(this) {
                Ok(ok__) => {
                    core::ptr::write(ppcallinfo, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Digit<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITDigitDetectionEvent_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pucdigit: *mut u8) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ITDigitDetectionEvent_Impl::Digit(this) {
                Ok(ok__) => {
                    core::ptr::write(pucdigit, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DigitMode<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITDigitDetectionEvent_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdigitmode: *mut i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ITDigitDetectionEvent_Impl::DigitMode(this) {
                Ok(ok__) => {
                    core::ptr::write(pdigitmode, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TickCount<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITDigitDetectionEvent_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pltickcount: *mut i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ITDigitDetectionEvent_Impl::TickCount(this) {
                Ok(ok__) => {
                    core::ptr::write(pltickcount, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CallbackInstance<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITDigitDetectionEvent_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, plcallbackinstance: *mut i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ITDigitDetectionEvent_Impl::CallbackInstance(this) {
                Ok(ok__) => {
                    core::ptr::write(plcallbackinstance, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            Call: Call::<Identity, Impl, OFFSET>,
            Digit: Digit::<Identity, Impl, OFFSET>,
            DigitMode: DigitMode::<Identity, Impl, OFFSET>,
            TickCount: TickCount::<Identity, Impl, OFFSET>,
            CallbackInstance: CallbackInstance::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITDigitDetectionEvent as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait ITDigitGenerationEvent_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn Call(&self) -> windows_core::Result<ITCallInfo>;
    fn GenerationTermination(&self) -> windows_core::Result<i32>;
    fn TickCount(&self) -> windows_core::Result<i32>;
    fn CallbackInstance(&self) -> windows_core::Result<i32>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for ITDigitGenerationEvent {}
#[cfg(feature = "Win32_System_Com")]
impl ITDigitGenerationEvent_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITDigitGenerationEvent_Impl, const OFFSET: isize>() -> ITDigitGenerationEvent_Vtbl {
        unsafe extern "system" fn Call<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITDigitGenerationEvent_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppcallinfo: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ITDigitGenerationEvent_Impl::Call(this) {
                Ok(ok__) => {
                    core::ptr::write(ppcallinfo, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GenerationTermination<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITDigitGenerationEvent_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, plgenerationtermination: *mut i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ITDigitGenerationEvent_Impl::GenerationTermination(this) {
                Ok(ok__) => {
                    core::ptr::write(plgenerationtermination, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TickCount<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITDigitGenerationEvent_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pltickcount: *mut i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ITDigitGenerationEvent_Impl::TickCount(this) {
                Ok(ok__) => {
                    core::ptr::write(pltickcount, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CallbackInstance<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITDigitGenerationEvent_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, plcallbackinstance: *mut i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ITDigitGenerationEvent_Impl::CallbackInstance(this) {
                Ok(ok__) => {
                    core::ptr::write(plcallbackinstance, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            Call: Call::<Identity, Impl, OFFSET>,
            GenerationTermination: GenerationTermination::<Identity, Impl, OFFSET>,
            TickCount: TickCount::<Identity, Impl, OFFSET>,
            CallbackInstance: CallbackInstance::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITDigitGenerationEvent as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait ITDigitsGatheredEvent_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn Call(&self) -> windows_core::Result<ITCallInfo>;
    fn Digits(&self) -> windows_core::Result<windows_core::BSTR>;
    fn GatherTermination(&self) -> windows_core::Result<TAPI_GATHERTERM>;
    fn TickCount(&self) -> windows_core::Result<i32>;
    fn CallbackInstance(&self) -> windows_core::Result<i32>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for ITDigitsGatheredEvent {}
#[cfg(feature = "Win32_System_Com")]
impl ITDigitsGatheredEvent_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITDigitsGatheredEvent_Impl, const OFFSET: isize>() -> ITDigitsGatheredEvent_Vtbl {
        unsafe extern "system" fn Call<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITDigitsGatheredEvent_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppcallinfo: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ITDigitsGatheredEvent_Impl::Call(this) {
                Ok(ok__) => {
                    core::ptr::write(ppcallinfo, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Digits<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITDigitsGatheredEvent_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppdigits: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ITDigitsGatheredEvent_Impl::Digits(this) {
                Ok(ok__) => {
                    core::ptr::write(ppdigits, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GatherTermination<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITDigitsGatheredEvent_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pgathertermination: *mut TAPI_GATHERTERM) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ITDigitsGatheredEvent_Impl::GatherTermination(this) {
                Ok(ok__) => {
                    core::ptr::write(pgathertermination, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TickCount<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITDigitsGatheredEvent_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pltickcount: *mut i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ITDigitsGatheredEvent_Impl::TickCount(this) {
                Ok(ok__) => {
                    core::ptr::write(pltickcount, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CallbackInstance<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITDigitsGatheredEvent_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, plcallbackinstance: *mut i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ITDigitsGatheredEvent_Impl::CallbackInstance(this) {
                Ok(ok__) => {
                    core::ptr::write(plcallbackinstance, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            Call: Call::<Identity, Impl, OFFSET>,
            Digits: Digits::<Identity, Impl, OFFSET>,
            GatherTermination: GatherTermination::<Identity, Impl, OFFSET>,
            TickCount: TickCount::<Identity, Impl, OFFSET>,
            CallbackInstance: CallbackInstance::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITDigitsGatheredEvent as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait ITDirectory_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn DirectoryType(&self) -> windows_core::Result<DIRECTORY_TYPE>;
    fn DisplayName(&self) -> windows_core::Result<windows_core::BSTR>;
    fn IsDynamic(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn DefaultObjectTTL(&self) -> windows_core::Result<i32>;
    fn SetDefaultObjectTTL(&self, ttl: i32) -> windows_core::Result<()>;
    fn EnableAutoRefresh(&self, fenable: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
    fn Connect(&self, fsecure: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
    fn Bind(&self, pdomainname: &windows_core::BSTR, pusername: &windows_core::BSTR, ppassword: &windows_core::BSTR, lflags: i32) -> windows_core::Result<()>;
    fn AddDirectoryObject(&self, pdirectoryobject: Option<&ITDirectoryObject>) -> windows_core::Result<()>;
    fn ModifyDirectoryObject(&self, pdirectoryobject: Option<&ITDirectoryObject>) -> windows_core::Result<()>;
    fn RefreshDirectoryObject(&self, pdirectoryobject: Option<&ITDirectoryObject>) -> windows_core::Result<()>;
    fn DeleteDirectoryObject(&self, pdirectoryobject: Option<&ITDirectoryObject>) -> windows_core::Result<()>;
    fn get_DirectoryObjects(&self, directoryobjecttype: DIRECTORY_OBJECT_TYPE, pname: &windows_core::BSTR) -> windows_core::Result<windows_core::VARIANT>;
    fn EnumerateDirectoryObjects(&self, directoryobjecttype: DIRECTORY_OBJECT_TYPE, pname: &windows_core::BSTR) -> windows_core::Result<IEnumDirectoryObject>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for ITDirectory {}
#[cfg(feature = "Win32_System_Com")]
impl ITDirectory_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITDirectory_Impl, const OFFSET: isize>() -> ITDirectory_Vtbl {
        unsafe extern "system" fn DirectoryType<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITDirectory_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdirectorytype: *mut DIRECTORY_TYPE) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ITDirectory_Impl::DirectoryType(this) {
                Ok(ok__) => {
                    core::ptr::write(pdirectorytype, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DisplayName<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITDirectory_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pname: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ITDirectory_Impl::DisplayName(this) {
                Ok(ok__) => {
                    core::ptr::write(pname, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsDynamic<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITDirectory_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pfdynamic: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ITDirectory_Impl::IsDynamic(this) {
                Ok(ok__) => {
                    core::ptr::write(pfdynamic, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DefaultObjectTTL<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITDirectory_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pttl: *mut i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ITDirectory_Impl::DefaultObjectTTL(this) {
                Ok(ok__) => {
                    core::ptr::write(pttl, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDefaultObjectTTL<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITDirectory_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ttl: i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ITDirectory_Impl::SetDefaultObjectTTL(this, core::mem::transmute_copy(&ttl)).into()
        }
        unsafe extern "system" fn EnableAutoRefresh<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITDirectory_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, fenable: super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ITDirectory_Impl::EnableAutoRefresh(this, core::mem::transmute_copy(&fenable)).into()
        }
        unsafe extern "system" fn Connect<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITDirectory_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, fsecure: super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ITDirectory_Impl::Connect(this, core::mem::transmute_copy(&fsecure)).into()
        }
        unsafe extern "system" fn Bind<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITDirectory_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdomainname: core::mem::MaybeUninit<windows_core::BSTR>, pusername: core::mem::MaybeUninit<windows_core::BSTR>, ppassword: core::mem::MaybeUninit<windows_core::BSTR>, lflags: i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ITDirectory_Impl::Bind(this, core::mem::transmute(&pdomainname), core::mem::transmute(&pusername), core::mem::transmute(&ppassword), core::mem::transmute_copy(&lflags)).into()
        }
        unsafe extern "system" fn AddDirectoryObject<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITDirectory_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdirectoryobject: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ITDirectory_Impl::AddDirectoryObject(this, windows_core::from_raw_borrowed(&pdirectoryobject)).into()
        }
        unsafe extern "system" fn ModifyDirectoryObject<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITDirectory_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdirectoryobject: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ITDirectory_Impl::ModifyDirectoryObject(this, windows_core::from_raw_borrowed(&pdirectoryobject)).into()
        }
        unsafe extern "system" fn RefreshDirectoryObject<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITDirectory_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdirectoryobject: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ITDirectory_Impl::RefreshDirectoryObject(this, windows_core::from_raw_borrowed(&pdirectoryobject)).into()
        }
        unsafe extern "system" fn DeleteDirectoryObject<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITDirectory_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdirectoryobject: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ITDirectory_Impl::DeleteDirectoryObject(this, windows_core::from_raw_borrowed(&pdirectoryobject)).into()
        }
        unsafe extern "system" fn get_DirectoryObjects<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITDirectory_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, directoryobjecttype: DIRECTORY_OBJECT_TYPE, pname: core::mem::MaybeUninit<windows_core::BSTR>, pvariant: *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ITDirectory_Impl::get_DirectoryObjects(this, core::mem::transmute_copy(&directoryobjecttype), core::mem::transmute(&pname)) {
                Ok(ok__) => {
                    core::ptr::write(pvariant, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnumerateDirectoryObjects<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITDirectory_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, directoryobjecttype: DIRECTORY_OBJECT_TYPE, pname: core::mem::MaybeUninit<windows_core::BSTR>, ppenumobject: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ITDirectory_Impl::EnumerateDirectoryObjects(this, core::mem::transmute_copy(&directoryobjecttype), core::mem::transmute(&pname)) {
                Ok(ok__) => {
                    core::ptr::write(ppenumobject, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            DirectoryType: DirectoryType::<Identity, Impl, OFFSET>,
            DisplayName: DisplayName::<Identity, Impl, OFFSET>,
            IsDynamic: IsDynamic::<Identity, Impl, OFFSET>,
            DefaultObjectTTL: DefaultObjectTTL::<Identity, Impl, OFFSET>,
            SetDefaultObjectTTL: SetDefaultObjectTTL::<Identity, Impl, OFFSET>,
            EnableAutoRefresh: EnableAutoRefresh::<Identity, Impl, OFFSET>,
            Connect: Connect::<Identity, Impl, OFFSET>,
            Bind: Bind::<Identity, Impl, OFFSET>,
            AddDirectoryObject: AddDirectoryObject::<Identity, Impl, OFFSET>,
            ModifyDirectoryObject: ModifyDirectoryObject::<Identity, Impl, OFFSET>,
            RefreshDirectoryObject: RefreshDirectoryObject::<Identity, Impl, OFFSET>,
            DeleteDirectoryObject: DeleteDirectoryObject::<Identity, Impl, OFFSET>,
            get_DirectoryObjects: get_DirectoryObjects::<Identity, Impl, OFFSET>,
            EnumerateDirectoryObjects: EnumerateDirectoryObjects::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITDirectory as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait ITDirectoryObject_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn ObjectType(&self) -> windows_core::Result<DIRECTORY_OBJECT_TYPE>;
    fn Name(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetName(&self, pname: &windows_core::BSTR) -> windows_core::Result<()>;
    fn get_DialableAddrs(&self, dwaddresstype: i32) -> windows_core::Result<windows_core::VARIANT>;
    fn EnumerateDialableAddrs(&self, dwaddresstype: u32) -> windows_core::Result<IEnumDialableAddrs>;
    fn SecurityDescriptor(&self) -> windows_core::Result<super::super::System::Com::IDispatch>;
    fn SetSecurityDescriptor(&self, psecdes: Option<&super::super::System::Com::IDispatch>) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for ITDirectoryObject {}
#[cfg(feature = "Win32_System_Com")]
impl ITDirectoryObject_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITDirectoryObject_Impl, const OFFSET: isize>() -> ITDirectoryObject_Vtbl {
        unsafe extern "system" fn ObjectType<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITDirectoryObject_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pobjecttype: *mut DIRECTORY_OBJECT_TYPE) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ITDirectoryObject_Impl::ObjectType(this) {
                Ok(ok__) => {
                    core::ptr::write(pobjecttype, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Name<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITDirectoryObject_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppname: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ITDirectoryObject_Impl::Name(this) {
                Ok(ok__) => {
                    core::ptr::write(ppname, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetName<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITDirectoryObject_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pname: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ITDirectoryObject_Impl::SetName(this, core::mem::transmute(&pname)).into()
        }
        unsafe extern "system" fn get_DialableAddrs<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITDirectoryObject_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwaddresstype: i32, pvariant: *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ITDirectoryObject_Impl::get_DialableAddrs(this, core::mem::transmute_copy(&dwaddresstype)) {
                Ok(ok__) => {
                    core::ptr::write(pvariant, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnumerateDialableAddrs<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITDirectoryObject_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwaddresstype: u32, ppenumdialableaddrs: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ITDirectoryObject_Impl::EnumerateDialableAddrs(this, core::mem::transmute_copy(&dwaddresstype)) {
                Ok(ok__) => {
                    core::ptr::write(ppenumdialableaddrs, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SecurityDescriptor<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITDirectoryObject_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppsecdes: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ITDirectoryObject_Impl::SecurityDescriptor(this) {
                Ok(ok__) => {
                    core::ptr::write(ppsecdes, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSecurityDescriptor<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITDirectoryObject_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, psecdes: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ITDirectoryObject_Impl::SetSecurityDescriptor(this, windows_core::from_raw_borrowed(&psecdes)).into()
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            ObjectType: ObjectType::<Identity, Impl, OFFSET>,
            Name: Name::<Identity, Impl, OFFSET>,
            SetName: SetName::<Identity, Impl, OFFSET>,
            get_DialableAddrs: get_DialableAddrs::<Identity, Impl, OFFSET>,
            EnumerateDialableAddrs: EnumerateDialableAddrs::<Identity, Impl, OFFSET>,
            SecurityDescriptor: SecurityDescriptor::<Identity, Impl, OFFSET>,
            SetSecurityDescriptor: SetSecurityDescriptor::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITDirectoryObject as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait ITDirectoryObjectConference_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn Protocol(&self) -> windows_core::Result<windows_core::BSTR>;
    fn Originator(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetOriginator(&self, poriginator: &windows_core::BSTR) -> windows_core::Result<()>;
    fn AdvertisingScope(&self) -> windows_core::Result<RND_ADVERTISING_SCOPE>;
    fn SetAdvertisingScope(&self, advertisingscope: RND_ADVERTISING_SCOPE) -> windows_core::Result<()>;
    fn Url(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetUrl(&self, purl: &windows_core::BSTR) -> windows_core::Result<()>;
    fn Description(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetDescription(&self, pdescription: &windows_core::BSTR) -> windows_core::Result<()>;
    fn IsEncrypted(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn SetIsEncrypted(&self, fencrypted: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
    fn StartTime(&self) -> windows_core::Result<f64>;
    fn SetStartTime(&self, date: f64) -> windows_core::Result<()>;
    fn StopTime(&self) -> windows_core::Result<f64>;
    fn SetStopTime(&self, date: f64) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for ITDirectoryObjectConference {}
#[cfg(feature = "Win32_System_Com")]
impl ITDirectoryObjectConference_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITDirectoryObjectConference_Impl, const OFFSET: isize>() -> ITDirectoryObjectConference_Vtbl {
        unsafe extern "system" fn Protocol<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITDirectoryObjectConference_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppprotocol: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ITDirectoryObjectConference_Impl::Protocol(this) {
                Ok(ok__) => {
                    core::ptr::write(ppprotocol, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Originator<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITDirectoryObjectConference_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pporiginator: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ITDirectoryObjectConference_Impl::Originator(this) {
                Ok(ok__) => {
                    core::ptr::write(pporiginator, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetOriginator<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITDirectoryObjectConference_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, poriginator: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ITDirectoryObjectConference_Impl::SetOriginator(this, core::mem::transmute(&poriginator)).into()
        }
        unsafe extern "system" fn AdvertisingScope<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITDirectoryObjectConference_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, padvertisingscope: *mut RND_ADVERTISING_SCOPE) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ITDirectoryObjectConference_Impl::AdvertisingScope(this) {
                Ok(ok__) => {
                    core::ptr::write(padvertisingscope, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAdvertisingScope<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITDirectoryObjectConference_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, advertisingscope: RND_ADVERTISING_SCOPE) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ITDirectoryObjectConference_Impl::SetAdvertisingScope(this, core::mem::transmute_copy(&advertisingscope)).into()
        }
        unsafe extern "system" fn Url<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITDirectoryObjectConference_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppurl: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ITDirectoryObjectConference_Impl::Url(this) {
                Ok(ok__) => {
                    core::ptr::write(ppurl, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetUrl<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITDirectoryObjectConference_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, purl: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ITDirectoryObjectConference_Impl::SetUrl(this, core::mem::transmute(&purl)).into()
        }
        unsafe extern "system" fn Description<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITDirectoryObjectConference_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppdescription: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ITDirectoryObjectConference_Impl::Description(this) {
                Ok(ok__) => {
                    core::ptr::write(ppdescription, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDescription<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITDirectoryObjectConference_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdescription: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ITDirectoryObjectConference_Impl::SetDescription(this, core::mem::transmute(&pdescription)).into()
        }
        unsafe extern "system" fn IsEncrypted<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITDirectoryObjectConference_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pfencrypted: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ITDirectoryObjectConference_Impl::IsEncrypted(this) {
                Ok(ok__) => {
                    core::ptr::write(pfencrypted, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetIsEncrypted<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITDirectoryObjectConference_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, fencrypted: super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ITDirectoryObjectConference_Impl::SetIsEncrypted(this, core::mem::transmute_copy(&fencrypted)).into()
        }
        unsafe extern "system" fn StartTime<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITDirectoryObjectConference_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdate: *mut f64) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ITDirectoryObjectConference_Impl::StartTime(this) {
                Ok(ok__) => {
                    core::ptr::write(pdate, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetStartTime<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITDirectoryObjectConference_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, date: f64) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ITDirectoryObjectConference_Impl::SetStartTime(this, core::mem::transmute_copy(&date)).into()
        }
        unsafe extern "system" fn StopTime<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITDirectoryObjectConference_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdate: *mut f64) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ITDirectoryObjectConference_Impl::StopTime(this) {
                Ok(ok__) => {
                    core::ptr::write(pdate, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetStopTime<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITDirectoryObjectConference_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, date: f64) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ITDirectoryObjectConference_Impl::SetStopTime(this, core::mem::transmute_copy(&date)).into()
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            Protocol: Protocol::<Identity, Impl, OFFSET>,
            Originator: Originator::<Identity, Impl, OFFSET>,
            SetOriginator: SetOriginator::<Identity, Impl, OFFSET>,
            AdvertisingScope: AdvertisingScope::<Identity, Impl, OFFSET>,
            SetAdvertisingScope: SetAdvertisingScope::<Identity, Impl, OFFSET>,
            Url: Url::<Identity, Impl, OFFSET>,
            SetUrl: SetUrl::<Identity, Impl, OFFSET>,
            Description: Description::<Identity, Impl, OFFSET>,
            SetDescription: SetDescription::<Identity, Impl, OFFSET>,
            IsEncrypted: IsEncrypted::<Identity, Impl, OFFSET>,
            SetIsEncrypted: SetIsEncrypted::<Identity, Impl, OFFSET>,
            StartTime: StartTime::<Identity, Impl, OFFSET>,
            SetStartTime: SetStartTime::<Identity, Impl, OFFSET>,
            StopTime: StopTime::<Identity, Impl, OFFSET>,
            SetStopTime: SetStopTime::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITDirectoryObjectConference as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait ITDirectoryObjectUser_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn IPPhonePrimary(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetIPPhonePrimary(&self, pname: &windows_core::BSTR) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for ITDirectoryObjectUser {}
#[cfg(feature = "Win32_System_Com")]
impl ITDirectoryObjectUser_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITDirectoryObjectUser_Impl, const OFFSET: isize>() -> ITDirectoryObjectUser_Vtbl {
        unsafe extern "system" fn IPPhonePrimary<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITDirectoryObjectUser_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppname: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ITDirectoryObjectUser_Impl::IPPhonePrimary(this) {
                Ok(ok__) => {
                    core::ptr::write(ppname, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetIPPhonePrimary<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITDirectoryObjectUser_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pname: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ITDirectoryObjectUser_Impl::SetIPPhonePrimary(this, core::mem::transmute(&pname)).into()
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            IPPhonePrimary: IPPhonePrimary::<Identity, Impl, OFFSET>,
            SetIPPhonePrimary: SetIPPhonePrimary::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITDirectoryObjectUser as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait ITDispatchMapper_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn QueryDispatchInterface(&self, piid: &windows_core::BSTR, pinterfacetomap: Option<&super::super::System::Com::IDispatch>) -> windows_core::Result<super::super::System::Com::IDispatch>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for ITDispatchMapper {}
#[cfg(feature = "Win32_System_Com")]
impl ITDispatchMapper_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITDispatchMapper_Impl, const OFFSET: isize>() -> ITDispatchMapper_Vtbl {
        unsafe extern "system" fn QueryDispatchInterface<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITDispatchMapper_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, piid: core::mem::MaybeUninit<windows_core::BSTR>, pinterfacetomap: *mut core::ffi::c_void, ppreturnedinterface: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ITDispatchMapper_Impl::QueryDispatchInterface(this, core::mem::transmute(&piid), windows_core::from_raw_borrowed(&pinterfacetomap)) {
                Ok(ok__) => {
                    core::ptr::write(ppreturnedinterface, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            QueryDispatchInterface: QueryDispatchInterface::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITDispatchMapper as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait ITFileTerminalEvent_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn Terminal(&self) -> windows_core::Result<ITTerminal>;
    fn Track(&self) -> windows_core::Result<ITFileTrack>;
    fn Call(&self) -> windows_core::Result<ITCallInfo>;
    fn State(&self) -> windows_core::Result<TERMINAL_MEDIA_STATE>;
    fn Cause(&self) -> windows_core::Result<FT_STATE_EVENT_CAUSE>;
    fn Error(&self) -> windows_core::Result<windows_core::HRESULT>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for ITFileTerminalEvent {}
#[cfg(feature = "Win32_System_Com")]
impl ITFileTerminalEvent_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITFileTerminalEvent_Impl, const OFFSET: isize>() -> ITFileTerminalEvent_Vtbl {
        unsafe extern "system" fn Terminal<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITFileTerminalEvent_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppterminal: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ITFileTerminalEvent_Impl::Terminal(this) {
                Ok(ok__) => {
                    core::ptr::write(ppterminal, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Track<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITFileTerminalEvent_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pptrackterminal: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ITFileTerminalEvent_Impl::Track(this) {
                Ok(ok__) => {
                    core::ptr::write(pptrackterminal, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Call<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITFileTerminalEvent_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppcall: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ITFileTerminalEvent_Impl::Call(this) {
                Ok(ok__) => {
                    core::ptr::write(ppcall, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn State<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITFileTerminalEvent_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pstate: *mut TERMINAL_MEDIA_STATE) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ITFileTerminalEvent_Impl::State(this) {
                Ok(ok__) => {
                    core::ptr::write(pstate, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Cause<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITFileTerminalEvent_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcause: *mut FT_STATE_EVENT_CAUSE) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ITFileTerminalEvent_Impl::Cause(this) {
                Ok(ok__) => {
                    core::ptr::write(pcause, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Error<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITFileTerminalEvent_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, phrerrorcode: *mut windows_core::HRESULT) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ITFileTerminalEvent_Impl::Error(this) {
                Ok(ok__) => {
                    core::ptr::write(phrerrorcode, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            Terminal: Terminal::<Identity, Impl, OFFSET>,
            Track: Track::<Identity, Impl, OFFSET>,
            Call: Call::<Identity, Impl, OFFSET>,
            State: State::<Identity, Impl, OFFSET>,
            Cause: Cause::<Identity, Impl, OFFSET>,
            Error: Error::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITFileTerminalEvent as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Media_MediaFoundation", feature = "Win32_System_Com"))]
pub trait ITFileTrack_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn Format(&self) -> windows_core::Result<*mut super::super::Media::MediaFoundation::AM_MEDIA_TYPE>;
    fn SetFormat(&self, pmt: *const super::super::Media::MediaFoundation::AM_MEDIA_TYPE) -> windows_core::Result<()>;
    fn ControllingTerminal(&self) -> windows_core::Result<ITTerminal>;
    fn AudioFormatForScripting(&self) -> windows_core::Result<ITScriptableAudioFormat>;
    fn SetAudioFormatForScripting(&self, paudioformat: Option<&ITScriptableAudioFormat>) -> windows_core::Result<()>;
    fn EmptyAudioFormatForScripting(&self) -> windows_core::Result<ITScriptableAudioFormat>;
}
#[cfg(all(feature = "Win32_Media_MediaFoundation", feature = "Win32_System_Com"))]
impl windows_core::RuntimeName for ITFileTrack {}
#[cfg(all(feature = "Win32_Media_MediaFoundation", feature = "Win32_System_Com"))]
impl ITFileTrack_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITFileTrack_Impl, const OFFSET: isize>() -> ITFileTrack_Vtbl {
        unsafe extern "system" fn Format<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITFileTrack_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppmt: *mut *mut super::super::Media::MediaFoundation::AM_MEDIA_TYPE) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ITFileTrack_Impl::Format(this) {
                Ok(ok__) => {
                    core::ptr::write(ppmt, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetFormat<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITFileTrack_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pmt: *const super::super::Media::MediaFoundation::AM_MEDIA_TYPE) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ITFileTrack_Impl::SetFormat(this, core::mem::transmute_copy(&pmt)).into()
        }
        unsafe extern "system" fn ControllingTerminal<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITFileTrack_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppcontrollingterminal: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ITFileTrack_Impl::ControllingTerminal(this) {
                Ok(ok__) => {
                    core::ptr::write(ppcontrollingterminal, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AudioFormatForScripting<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITFileTrack_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppaudioformat: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ITFileTrack_Impl::AudioFormatForScripting(this) {
                Ok(ok__) => {
                    core::ptr::write(ppaudioformat, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAudioFormatForScripting<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITFileTrack_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, paudioformat: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ITFileTrack_Impl::SetAudioFormatForScripting(this, windows_core::from_raw_borrowed(&paudioformat)).into()
        }
        unsafe extern "system" fn EmptyAudioFormatForScripting<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITFileTrack_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppaudioformat: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ITFileTrack_Impl::EmptyAudioFormatForScripting(this) {
                Ok(ok__) => {
                    core::ptr::write(ppaudioformat, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            Format: Format::<Identity, Impl, OFFSET>,
            SetFormat: SetFormat::<Identity, Impl, OFFSET>,
            ControllingTerminal: ControllingTerminal::<Identity, Impl, OFFSET>,
            AudioFormatForScripting: AudioFormatForScripting::<Identity, Impl, OFFSET>,
            SetAudioFormatForScripting: SetAudioFormatForScripting::<Identity, Impl, OFFSET>,
            EmptyAudioFormatForScripting: EmptyAudioFormatForScripting::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITFileTrack as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait ITForwardInformation_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn SetNumRingsNoAnswer(&self, lnumrings: i32) -> windows_core::Result<()>;
    fn NumRingsNoAnswer(&self) -> windows_core::Result<i32>;
    fn SetForwardType(&self, forwardtype: i32, pdestaddress: &windows_core::BSTR, pcalleraddress: &windows_core::BSTR) -> windows_core::Result<()>;
    fn get_ForwardTypeDestination(&self, forwardtype: i32) -> windows_core::Result<windows_core::BSTR>;
    fn get_ForwardTypeCaller(&self, forwardtype: i32) -> windows_core::Result<windows_core::BSTR>;
    fn GetForwardType(&self, forwardtype: i32, ppdestinationaddress: *mut windows_core::BSTR, ppcalleraddress: *mut windows_core::BSTR) -> windows_core::Result<()>;
    fn Clear(&self) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for ITForwardInformation {}
#[cfg(feature = "Win32_System_Com")]
impl ITForwardInformation_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITForwardInformation_Impl, const OFFSET: isize>() -> ITForwardInformation_Vtbl {
        unsafe extern "system" fn SetNumRingsNoAnswer<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITForwardInformation_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lnumrings: i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ITForwardInformation_Impl::SetNumRingsNoAnswer(this, core::mem::transmute_copy(&lnumrings)).into()
        }
        unsafe extern "system" fn NumRingsNoAnswer<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITForwardInformation_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, plnumrings: *mut i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ITForwardInformation_Impl::NumRingsNoAnswer(this) {
                Ok(ok__) => {
                    core::ptr::write(plnumrings, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetForwardType<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITForwardInformation_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, forwardtype: i32, pdestaddress: core::mem::MaybeUninit<windows_core::BSTR>, pcalleraddress: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ITForwardInformation_Impl::SetForwardType(this, core::mem::transmute_copy(&forwardtype), core::mem::transmute(&pdestaddress), core::mem::transmute(&pcalleraddress)).into()
        }
        unsafe extern "system" fn get_ForwardTypeDestination<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITForwardInformation_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, forwardtype: i32, ppdestaddress: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ITForwardInformation_Impl::get_ForwardTypeDestination(this, core::mem::transmute_copy(&forwardtype)) {
                Ok(ok__) => {
                    core::ptr::write(ppdestaddress, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn get_ForwardTypeCaller<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITForwardInformation_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, forwardtype: i32, ppcalleraddress: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ITForwardInformation_Impl::get_ForwardTypeCaller(this, core::mem::transmute_copy(&forwardtype)) {
                Ok(ok__) => {
                    core::ptr::write(ppcalleraddress, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetForwardType<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITForwardInformation_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, forwardtype: i32, ppdestinationaddress: *mut core::mem::MaybeUninit<windows_core::BSTR>, ppcalleraddress: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ITForwardInformation_Impl::GetForwardType(this, core::mem::transmute_copy(&forwardtype), core::mem::transmute_copy(&ppdestinationaddress), core::mem::transmute_copy(&ppcalleraddress)).into()
        }
        unsafe extern "system" fn Clear<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITForwardInformation_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ITForwardInformation_Impl::Clear(this).into()
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            SetNumRingsNoAnswer: SetNumRingsNoAnswer::<Identity, Impl, OFFSET>,
            NumRingsNoAnswer: NumRingsNoAnswer::<Identity, Impl, OFFSET>,
            SetForwardType: SetForwardType::<Identity, Impl, OFFSET>,
            get_ForwardTypeDestination: get_ForwardTypeDestination::<Identity, Impl, OFFSET>,
            get_ForwardTypeCaller: get_ForwardTypeCaller::<Identity, Impl, OFFSET>,
            GetForwardType: GetForwardType::<Identity, Impl, OFFSET>,
            Clear: Clear::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITForwardInformation as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait ITForwardInformation2_Impl: Sized + ITForwardInformation_Impl {
    fn SetForwardType2(&self, forwardtype: i32, pdestaddress: &windows_core::BSTR, destaddresstype: i32, pcalleraddress: &windows_core::BSTR, calleraddresstype: i32) -> windows_core::Result<()>;
    fn GetForwardType2(&self, forwardtype: i32, ppdestinationaddress: *mut windows_core::BSTR, pdestaddresstype: *mut i32, ppcalleraddress: *mut windows_core::BSTR, pcalleraddresstype: *mut i32) -> windows_core::Result<()>;
    fn get_ForwardTypeDestinationAddressType(&self, forwardtype: i32) -> windows_core::Result<i32>;
    fn get_ForwardTypeCallerAddressType(&self, forwardtype: i32) -> windows_core::Result<i32>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for ITForwardInformation2 {}
#[cfg(feature = "Win32_System_Com")]
impl ITForwardInformation2_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITForwardInformation2_Impl, const OFFSET: isize>() -> ITForwardInformation2_Vtbl {
        unsafe extern "system" fn SetForwardType2<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITForwardInformation2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, forwardtype: i32, pdestaddress: core::mem::MaybeUninit<windows_core::BSTR>, destaddresstype: i32, pcalleraddress: core::mem::MaybeUninit<windows_core::BSTR>, calleraddresstype: i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ITForwardInformation2_Impl::SetForwardType2(this, core::mem::transmute_copy(&forwardtype), core::mem::transmute(&pdestaddress), core::mem::transmute_copy(&destaddresstype), core::mem::transmute(&pcalleraddress), core::mem::transmute_copy(&calleraddresstype)).into()
        }
        unsafe extern "system" fn GetForwardType2<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITForwardInformation2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, forwardtype: i32, ppdestinationaddress: *mut core::mem::MaybeUninit<windows_core::BSTR>, pdestaddresstype: *mut i32, ppcalleraddress: *mut core::mem::MaybeUninit<windows_core::BSTR>, pcalleraddresstype: *mut i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ITForwardInformation2_Impl::GetForwardType2(this, core::mem::transmute_copy(&forwardtype), core::mem::transmute_copy(&ppdestinationaddress), core::mem::transmute_copy(&pdestaddresstype), core::mem::transmute_copy(&ppcalleraddress), core::mem::transmute_copy(&pcalleraddresstype)).into()
        }
        unsafe extern "system" fn get_ForwardTypeDestinationAddressType<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITForwardInformation2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, forwardtype: i32, pdestaddresstype: *mut i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ITForwardInformation2_Impl::get_ForwardTypeDestinationAddressType(this, core::mem::transmute_copy(&forwardtype)) {
                Ok(ok__) => {
                    core::ptr::write(pdestaddresstype, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn get_ForwardTypeCallerAddressType<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITForwardInformation2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, forwardtype: i32, pcalleraddresstype: *mut i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ITForwardInformation2_Impl::get_ForwardTypeCallerAddressType(this, core::mem::transmute_copy(&forwardtype)) {
                Ok(ok__) => {
                    core::ptr::write(pcalleraddresstype, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: ITForwardInformation_Vtbl::new::<Identity, Impl, OFFSET>(),
            SetForwardType2: SetForwardType2::<Identity, Impl, OFFSET>,
            GetForwardType2: GetForwardType2::<Identity, Impl, OFFSET>,
            get_ForwardTypeDestinationAddressType: get_ForwardTypeDestinationAddressType::<Identity, Impl, OFFSET>,
            get_ForwardTypeCallerAddressType: get_ForwardTypeCallerAddressType::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITForwardInformation2 as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID || iid == &<ITForwardInformation as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait ITILSConfig_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn Port(&self) -> windows_core::Result<i32>;
    fn SetPort(&self, port: i32) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for ITILSConfig {}
#[cfg(feature = "Win32_System_Com")]
impl ITILSConfig_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITILSConfig_Impl, const OFFSET: isize>() -> ITILSConfig_Vtbl {
        unsafe extern "system" fn Port<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITILSConfig_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pport: *mut i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ITILSConfig_Impl::Port(this) {
                Ok(ok__) => {
                    core::ptr::write(pport, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPort<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITILSConfig_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, port: i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ITILSConfig_Impl::SetPort(this, core::mem::transmute_copy(&port)).into()
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            Port: Port::<Identity, Impl, OFFSET>,
            SetPort: SetPort::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITILSConfig as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
pub trait ITLegacyAddressMediaControl_Impl: Sized {
    fn GetID(&self, pdeviceclass: &windows_core::BSTR, pdwsize: *mut u32, ppdeviceid: *mut *mut u8) -> windows_core::Result<()>;
    fn GetDevConfig(&self, pdeviceclass: &windows_core::BSTR, pdwsize: *mut u32, ppdeviceconfig: *mut *mut u8) -> windows_core::Result<()>;
    fn SetDevConfig(&self, pdeviceclass: &windows_core::BSTR, dwsize: u32, pdeviceconfig: *const u8) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for ITLegacyAddressMediaControl {}
impl ITLegacyAddressMediaControl_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITLegacyAddressMediaControl_Impl, const OFFSET: isize>() -> ITLegacyAddressMediaControl_Vtbl {
        unsafe extern "system" fn GetID<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITLegacyAddressMediaControl_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdeviceclass: core::mem::MaybeUninit<windows_core::BSTR>, pdwsize: *mut u32, ppdeviceid: *mut *mut u8) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ITLegacyAddressMediaControl_Impl::GetID(this, core::mem::transmute(&pdeviceclass), core::mem::transmute_copy(&pdwsize), core::mem::transmute_copy(&ppdeviceid)).into()
        }
        unsafe extern "system" fn GetDevConfig<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITLegacyAddressMediaControl_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdeviceclass: core::mem::MaybeUninit<windows_core::BSTR>, pdwsize: *mut u32, ppdeviceconfig: *mut *mut u8) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ITLegacyAddressMediaControl_Impl::GetDevConfig(this, core::mem::transmute(&pdeviceclass), core::mem::transmute_copy(&pdwsize), core::mem::transmute_copy(&ppdeviceconfig)).into()
        }
        unsafe extern "system" fn SetDevConfig<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITLegacyAddressMediaControl_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdeviceclass: core::mem::MaybeUninit<windows_core::BSTR>, dwsize: u32, pdeviceconfig: *const u8) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ITLegacyAddressMediaControl_Impl::SetDevConfig(this, core::mem::transmute(&pdeviceclass), core::mem::transmute_copy(&dwsize), core::mem::transmute_copy(&pdeviceconfig)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetID: GetID::<Identity, Impl, OFFSET>,
            GetDevConfig: GetDevConfig::<Identity, Impl, OFFSET>,
            SetDevConfig: SetDevConfig::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITLegacyAddressMediaControl as windows_core::Interface>::IID
    }
}
pub trait ITLegacyAddressMediaControl2_Impl: Sized + ITLegacyAddressMediaControl_Impl {
    fn ConfigDialog(&self, hwndowner: super::super::Foundation::HWND, pdeviceclass: &windows_core::BSTR) -> windows_core::Result<()>;
    fn ConfigDialogEdit(&self, hwndowner: super::super::Foundation::HWND, pdeviceclass: &windows_core::BSTR, dwsizein: u32, pdeviceconfigin: *const u8, pdwsizeout: *mut u32, ppdeviceconfigout: *mut *mut u8) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for ITLegacyAddressMediaControl2 {}
impl ITLegacyAddressMediaControl2_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITLegacyAddressMediaControl2_Impl, const OFFSET: isize>() -> ITLegacyAddressMediaControl2_Vtbl {
        unsafe extern "system" fn ConfigDialog<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITLegacyAddressMediaControl2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hwndowner: super::super::Foundation::HWND, pdeviceclass: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ITLegacyAddressMediaControl2_Impl::ConfigDialog(this, core::mem::transmute_copy(&hwndowner), core::mem::transmute(&pdeviceclass)).into()
        }
        unsafe extern "system" fn ConfigDialogEdit<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITLegacyAddressMediaControl2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hwndowner: super::super::Foundation::HWND, pdeviceclass: core::mem::MaybeUninit<windows_core::BSTR>, dwsizein: u32, pdeviceconfigin: *const u8, pdwsizeout: *mut u32, ppdeviceconfigout: *mut *mut u8) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ITLegacyAddressMediaControl2_Impl::ConfigDialogEdit(this, core::mem::transmute_copy(&hwndowner), core::mem::transmute(&pdeviceclass), core::mem::transmute_copy(&dwsizein), core::mem::transmute_copy(&pdeviceconfigin), core::mem::transmute_copy(&pdwsizeout), core::mem::transmute_copy(&ppdeviceconfigout)).into()
        }
        Self {
            base__: ITLegacyAddressMediaControl_Vtbl::new::<Identity, Impl, OFFSET>(),
            ConfigDialog: ConfigDialog::<Identity, Impl, OFFSET>,
            ConfigDialogEdit: ConfigDialogEdit::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITLegacyAddressMediaControl2 as windows_core::Interface>::IID || iid == &<ITLegacyAddressMediaControl as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait ITLegacyCallMediaControl_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn DetectDigits(&self, digitmode: i32) -> windows_core::Result<()>;
    fn GenerateDigits(&self, pdigits: &windows_core::BSTR, digitmode: i32) -> windows_core::Result<()>;
    fn GetID(&self, pdeviceclass: &windows_core::BSTR, pdwsize: *mut u32, ppdeviceid: *mut *mut u8) -> windows_core::Result<()>;
    fn SetMediaType(&self, lmediatype: i32) -> windows_core::Result<()>;
    fn MonitorMedia(&self, lmediatype: i32) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for ITLegacyCallMediaControl {}
#[cfg(feature = "Win32_System_Com")]
impl ITLegacyCallMediaControl_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITLegacyCallMediaControl_Impl, const OFFSET: isize>() -> ITLegacyCallMediaControl_Vtbl {
        unsafe extern "system" fn DetectDigits<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITLegacyCallMediaControl_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, digitmode: i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ITLegacyCallMediaControl_Impl::DetectDigits(this, core::mem::transmute_copy(&digitmode)).into()
        }
        unsafe extern "system" fn GenerateDigits<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITLegacyCallMediaControl_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdigits: core::mem::MaybeUninit<windows_core::BSTR>, digitmode: i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ITLegacyCallMediaControl_Impl::GenerateDigits(this, core::mem::transmute(&pdigits), core::mem::transmute_copy(&digitmode)).into()
        }
        unsafe extern "system" fn GetID<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITLegacyCallMediaControl_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdeviceclass: core::mem::MaybeUninit<windows_core::BSTR>, pdwsize: *mut u32, ppdeviceid: *mut *mut u8) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ITLegacyCallMediaControl_Impl::GetID(this, core::mem::transmute(&pdeviceclass), core::mem::transmute_copy(&pdwsize), core::mem::transmute_copy(&ppdeviceid)).into()
        }
        unsafe extern "system" fn SetMediaType<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITLegacyCallMediaControl_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lmediatype: i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ITLegacyCallMediaControl_Impl::SetMediaType(this, core::mem::transmute_copy(&lmediatype)).into()
        }
        unsafe extern "system" fn MonitorMedia<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITLegacyCallMediaControl_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lmediatype: i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ITLegacyCallMediaControl_Impl::MonitorMedia(this, core::mem::transmute_copy(&lmediatype)).into()
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            DetectDigits: DetectDigits::<Identity, Impl, OFFSET>,
            GenerateDigits: GenerateDigits::<Identity, Impl, OFFSET>,
            GetID: GetID::<Identity, Impl, OFFSET>,
            SetMediaType: SetMediaType::<Identity, Impl, OFFSET>,
            MonitorMedia: MonitorMedia::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITLegacyCallMediaControl as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait ITLegacyCallMediaControl2_Impl: Sized + ITLegacyCallMediaControl_Impl {
    fn GenerateDigits2(&self, pdigits: &windows_core::BSTR, digitmode: i32, lduration: i32) -> windows_core::Result<()>;
    fn GatherDigits(&self, digitmode: i32, lnumdigits: i32, pterminationdigits: &windows_core::BSTR, lfirstdigittimeout: i32, linterdigittimeout: i32) -> windows_core::Result<()>;
    fn DetectTones(&self, ptonelist: *const TAPI_DETECTTONE, lnumtones: i32) -> windows_core::Result<()>;
    fn DetectTonesByCollection(&self, pdetecttonecollection: Option<&ITCollection2>) -> windows_core::Result<()>;
    fn GenerateTone(&self, tonemode: TAPI_TONEMODE, lduration: i32) -> windows_core::Result<()>;
    fn GenerateCustomTones(&self, ptonelist: *const TAPI_CUSTOMTONE, lnumtones: i32, lduration: i32) -> windows_core::Result<()>;
    fn GenerateCustomTonesByCollection(&self, pcustomtonecollection: Option<&ITCollection2>, lduration: i32) -> windows_core::Result<()>;
    fn CreateDetectToneObject(&self) -> windows_core::Result<ITDetectTone>;
    fn CreateCustomToneObject(&self) -> windows_core::Result<ITCustomTone>;
    fn GetIDAsVariant(&self, bstrdeviceclass: &windows_core::BSTR) -> windows_core::Result<windows_core::VARIANT>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for ITLegacyCallMediaControl2 {}
#[cfg(feature = "Win32_System_Com")]
impl ITLegacyCallMediaControl2_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITLegacyCallMediaControl2_Impl, const OFFSET: isize>() -> ITLegacyCallMediaControl2_Vtbl {
        unsafe extern "system" fn GenerateDigits2<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITLegacyCallMediaControl2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdigits: core::mem::MaybeUninit<windows_core::BSTR>, digitmode: i32, lduration: i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ITLegacyCallMediaControl2_Impl::GenerateDigits2(this, core::mem::transmute(&pdigits), core::mem::transmute_copy(&digitmode), core::mem::transmute_copy(&lduration)).into()
        }
        unsafe extern "system" fn GatherDigits<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITLegacyCallMediaControl2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, digitmode: i32, lnumdigits: i32, pterminationdigits: core::mem::MaybeUninit<windows_core::BSTR>, lfirstdigittimeout: i32, linterdigittimeout: i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ITLegacyCallMediaControl2_Impl::GatherDigits(this, core::mem::transmute_copy(&digitmode), core::mem::transmute_copy(&lnumdigits), core::mem::transmute(&pterminationdigits), core::mem::transmute_copy(&lfirstdigittimeout), core::mem::transmute_copy(&linterdigittimeout)).into()
        }
        unsafe extern "system" fn DetectTones<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITLegacyCallMediaControl2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ptonelist: *const TAPI_DETECTTONE, lnumtones: i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ITLegacyCallMediaControl2_Impl::DetectTones(this, core::mem::transmute_copy(&ptonelist), core::mem::transmute_copy(&lnumtones)).into()
        }
        unsafe extern "system" fn DetectTonesByCollection<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITLegacyCallMediaControl2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdetecttonecollection: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ITLegacyCallMediaControl2_Impl::DetectTonesByCollection(this, windows_core::from_raw_borrowed(&pdetecttonecollection)).into()
        }
        unsafe extern "system" fn GenerateTone<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITLegacyCallMediaControl2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, tonemode: TAPI_TONEMODE, lduration: i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ITLegacyCallMediaControl2_Impl::GenerateTone(this, core::mem::transmute_copy(&tonemode), core::mem::transmute_copy(&lduration)).into()
        }
        unsafe extern "system" fn GenerateCustomTones<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITLegacyCallMediaControl2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ptonelist: *const TAPI_CUSTOMTONE, lnumtones: i32, lduration: i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ITLegacyCallMediaControl2_Impl::GenerateCustomTones(this, core::mem::transmute_copy(&ptonelist), core::mem::transmute_copy(&lnumtones), core::mem::transmute_copy(&lduration)).into()
        }
        unsafe extern "system" fn GenerateCustomTonesByCollection<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITLegacyCallMediaControl2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcustomtonecollection: *mut core::ffi::c_void, lduration: i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ITLegacyCallMediaControl2_Impl::GenerateCustomTonesByCollection(this, windows_core::from_raw_borrowed(&pcustomtonecollection), core::mem::transmute_copy(&lduration)).into()
        }
        unsafe extern "system" fn CreateDetectToneObject<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITLegacyCallMediaControl2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppdetecttone: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ITLegacyCallMediaControl2_Impl::CreateDetectToneObject(this) {
                Ok(ok__) => {
                    core::ptr::write(ppdetecttone, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateCustomToneObject<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITLegacyCallMediaControl2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppcustomtone: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ITLegacyCallMediaControl2_Impl::CreateCustomToneObject(this) {
                Ok(ok__) => {
                    core::ptr::write(ppcustomtone, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetIDAsVariant<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITLegacyCallMediaControl2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrdeviceclass: core::mem::MaybeUninit<windows_core::BSTR>, pvardeviceid: *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ITLegacyCallMediaControl2_Impl::GetIDAsVariant(this, core::mem::transmute(&bstrdeviceclass)) {
                Ok(ok__) => {
                    core::ptr::write(pvardeviceid, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: ITLegacyCallMediaControl_Vtbl::new::<Identity, Impl, OFFSET>(),
            GenerateDigits2: GenerateDigits2::<Identity, Impl, OFFSET>,
            GatherDigits: GatherDigits::<Identity, Impl, OFFSET>,
            DetectTones: DetectTones::<Identity, Impl, OFFSET>,
            DetectTonesByCollection: DetectTonesByCollection::<Identity, Impl, OFFSET>,
            GenerateTone: GenerateTone::<Identity, Impl, OFFSET>,
            GenerateCustomTones: GenerateCustomTones::<Identity, Impl, OFFSET>,
            GenerateCustomTonesByCollection: GenerateCustomTonesByCollection::<Identity, Impl, OFFSET>,
            CreateDetectToneObject: CreateDetectToneObject::<Identity, Impl, OFFSET>,
            CreateCustomToneObject: CreateCustomToneObject::<Identity, Impl, OFFSET>,
            GetIDAsVariant: GetIDAsVariant::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITLegacyCallMediaControl2 as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID || iid == &<ITLegacyCallMediaControl as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait ITLegacyWaveSupport_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn IsFullDuplex(&self) -> windows_core::Result<FULLDUPLEX_SUPPORT>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for ITLegacyWaveSupport {}
#[cfg(feature = "Win32_System_Com")]
impl ITLegacyWaveSupport_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITLegacyWaveSupport_Impl, const OFFSET: isize>() -> ITLegacyWaveSupport_Vtbl {
        unsafe extern "system" fn IsFullDuplex<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITLegacyWaveSupport_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, psupport: *mut FULLDUPLEX_SUPPORT) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ITLegacyWaveSupport_Impl::IsFullDuplex(this) {
                Ok(ok__) => {
                    core::ptr::write(psupport, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self { base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(), IsFullDuplex: IsFullDuplex::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITLegacyWaveSupport as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait ITLocationInfo_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn PermanentLocationID(&self) -> windows_core::Result<i32>;
    fn CountryCode(&self) -> windows_core::Result<i32>;
    fn CountryID(&self) -> windows_core::Result<i32>;
    fn Options(&self) -> windows_core::Result<i32>;
    fn PreferredCardID(&self) -> windows_core::Result<i32>;
    fn LocationName(&self) -> windows_core::Result<windows_core::BSTR>;
    fn CityCode(&self) -> windows_core::Result<windows_core::BSTR>;
    fn LocalAccessCode(&self) -> windows_core::Result<windows_core::BSTR>;
    fn LongDistanceAccessCode(&self) -> windows_core::Result<windows_core::BSTR>;
    fn TollPrefixList(&self) -> windows_core::Result<windows_core::BSTR>;
    fn CancelCallWaitingCode(&self) -> windows_core::Result<windows_core::BSTR>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for ITLocationInfo {}
#[cfg(feature = "Win32_System_Com")]
impl ITLocationInfo_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITLocationInfo_Impl, const OFFSET: isize>() -> ITLocationInfo_Vtbl {
        unsafe extern "system" fn PermanentLocationID<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITLocationInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pllocationid: *mut i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ITLocationInfo_Impl::PermanentLocationID(this) {
                Ok(ok__) => {
                    core::ptr::write(pllocationid, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CountryCode<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITLocationInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, plcountrycode: *mut i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ITLocationInfo_Impl::CountryCode(this) {
                Ok(ok__) => {
                    core::ptr::write(plcountrycode, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CountryID<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITLocationInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, plcountryid: *mut i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ITLocationInfo_Impl::CountryID(this) {
                Ok(ok__) => {
                    core::ptr::write(plcountryid, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Options<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITLocationInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ploptions: *mut i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ITLocationInfo_Impl::Options(this) {
                Ok(ok__) => {
                    core::ptr::write(ploptions, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PreferredCardID<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITLocationInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, plcardid: *mut i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ITLocationInfo_Impl::PreferredCardID(this) {
                Ok(ok__) => {
                    core::ptr::write(plcardid, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LocationName<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITLocationInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pplocationname: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ITLocationInfo_Impl::LocationName(this) {
                Ok(ok__) => {
                    core::ptr::write(pplocationname, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CityCode<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITLocationInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppcode: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ITLocationInfo_Impl::CityCode(this) {
                Ok(ok__) => {
                    core::ptr::write(ppcode, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LocalAccessCode<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITLocationInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppcode: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ITLocationInfo_Impl::LocalAccessCode(this) {
                Ok(ok__) => {
                    core::ptr::write(ppcode, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LongDistanceAccessCode<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITLocationInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppcode: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ITLocationInfo_Impl::LongDistanceAccessCode(this) {
                Ok(ok__) => {
                    core::ptr::write(ppcode, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TollPrefixList<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITLocationInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pptolllist: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ITLocationInfo_Impl::TollPrefixList(this) {
                Ok(ok__) => {
                    core::ptr::write(pptolllist, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CancelCallWaitingCode<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITLocationInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppcode: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ITLocationInfo_Impl::CancelCallWaitingCode(this) {
                Ok(ok__) => {
                    core::ptr::write(ppcode, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            PermanentLocationID: PermanentLocationID::<Identity, Impl, OFFSET>,
            CountryCode: CountryCode::<Identity, Impl, OFFSET>,
            CountryID: CountryID::<Identity, Impl, OFFSET>,
            Options: Options::<Identity, Impl, OFFSET>,
            PreferredCardID: PreferredCardID::<Identity, Impl, OFFSET>,
            LocationName: LocationName::<Identity, Impl, OFFSET>,
            CityCode: CityCode::<Identity, Impl, OFFSET>,
            LocalAccessCode: LocalAccessCode::<Identity, Impl, OFFSET>,
            LongDistanceAccessCode: LongDistanceAccessCode::<Identity, Impl, OFFSET>,
            TollPrefixList: TollPrefixList::<Identity, Impl, OFFSET>,
            CancelCallWaitingCode: CancelCallWaitingCode::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITLocationInfo as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
pub trait ITMSPAddress_Impl: Sized {
    fn Initialize(&self, hevent: *const i32) -> windows_core::Result<()>;
    fn Shutdown(&self) -> windows_core::Result<()>;
    fn CreateMSPCall(&self, hcall: *const i32, dwreserved: u32, dwmediatype: u32, pouterunknown: Option<&windows_core::IUnknown>) -> windows_core::Result<windows_core::IUnknown>;
    fn ShutdownMSPCall(&self, pstreamcontrol: Option<&windows_core::IUnknown>) -> windows_core::Result<()>;
    fn ReceiveTSPData(&self, pmspcall: Option<&windows_core::IUnknown>, pbuffer: *const u8, dwsize: u32) -> windows_core::Result<()>;
    fn GetEvent(&self, pdwsize: *mut u32, peventbuffer: *mut u8) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for ITMSPAddress {}
impl ITMSPAddress_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITMSPAddress_Impl, const OFFSET: isize>() -> ITMSPAddress_Vtbl {
        unsafe extern "system" fn Initialize<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITMSPAddress_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hevent: *const i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ITMSPAddress_Impl::Initialize(this, core::mem::transmute_copy(&hevent)).into()
        }
        unsafe extern "system" fn Shutdown<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITMSPAddress_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ITMSPAddress_Impl::Shutdown(this).into()
        }
        unsafe extern "system" fn CreateMSPCall<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITMSPAddress_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hcall: *const i32, dwreserved: u32, dwmediatype: u32, pouterunknown: *mut core::ffi::c_void, ppstreamcontrol: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ITMSPAddress_Impl::CreateMSPCall(this, core::mem::transmute_copy(&hcall), core::mem::transmute_copy(&dwreserved), core::mem::transmute_copy(&dwmediatype), windows_core::from_raw_borrowed(&pouterunknown)) {
                Ok(ok__) => {
                    core::ptr::write(ppstreamcontrol, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ShutdownMSPCall<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITMSPAddress_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pstreamcontrol: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ITMSPAddress_Impl::ShutdownMSPCall(this, windows_core::from_raw_borrowed(&pstreamcontrol)).into()
        }
        unsafe extern "system" fn ReceiveTSPData<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITMSPAddress_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pmspcall: *mut core::ffi::c_void, pbuffer: *const u8, dwsize: u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ITMSPAddress_Impl::ReceiveTSPData(this, windows_core::from_raw_borrowed(&pmspcall), core::mem::transmute_copy(&pbuffer), core::mem::transmute_copy(&dwsize)).into()
        }
        unsafe extern "system" fn GetEvent<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITMSPAddress_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwsize: *mut u32, peventbuffer: *mut u8) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ITMSPAddress_Impl::GetEvent(this, core::mem::transmute_copy(&pdwsize), core::mem::transmute_copy(&peventbuffer)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Initialize: Initialize::<Identity, Impl, OFFSET>,
            Shutdown: Shutdown::<Identity, Impl, OFFSET>,
            CreateMSPCall: CreateMSPCall::<Identity, Impl, OFFSET>,
            ShutdownMSPCall: ShutdownMSPCall::<Identity, Impl, OFFSET>,
            ReceiveTSPData: ReceiveTSPData::<Identity, Impl, OFFSET>,
            GetEvent: GetEvent::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITMSPAddress as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait ITMediaControl_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn Start(&self) -> windows_core::Result<()>;
    fn Stop(&self) -> windows_core::Result<()>;
    fn Pause(&self) -> windows_core::Result<()>;
    fn MediaState(&self) -> windows_core::Result<TERMINAL_MEDIA_STATE>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for ITMediaControl {}
#[cfg(feature = "Win32_System_Com")]
impl ITMediaControl_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITMediaControl_Impl, const OFFSET: isize>() -> ITMediaControl_Vtbl {
        unsafe extern "system" fn Start<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITMediaControl_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ITMediaControl_Impl::Start(this).into()
        }
        unsafe extern "system" fn Stop<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITMediaControl_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ITMediaControl_Impl::Stop(this).into()
        }
        unsafe extern "system" fn Pause<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITMediaControl_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ITMediaControl_Impl::Pause(this).into()
        }
        unsafe extern "system" fn MediaState<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITMediaControl_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pterminalmediastate: *mut TERMINAL_MEDIA_STATE) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ITMediaControl_Impl::MediaState(this) {
                Ok(ok__) => {
                    core::ptr::write(pterminalmediastate, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            Start: Start::<Identity, Impl, OFFSET>,
            Stop: Stop::<Identity, Impl, OFFSET>,
            Pause: Pause::<Identity, Impl, OFFSET>,
            MediaState: MediaState::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITMediaControl as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait ITMediaPlayback_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn SetPlayList(&self, playlistvariant: &windows_core::VARIANT) -> windows_core::Result<()>;
    fn PlayList(&self) -> windows_core::Result<windows_core::VARIANT>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for ITMediaPlayback {}
#[cfg(feature = "Win32_System_Com")]
impl ITMediaPlayback_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITMediaPlayback_Impl, const OFFSET: isize>() -> ITMediaPlayback_Vtbl {
        unsafe extern "system" fn SetPlayList<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITMediaPlayback_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, playlistvariant: core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ITMediaPlayback_Impl::SetPlayList(this, core::mem::transmute(&playlistvariant)).into()
        }
        unsafe extern "system" fn PlayList<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITMediaPlayback_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pplaylistvariant: *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ITMediaPlayback_Impl::PlayList(this) {
                Ok(ok__) => {
                    core::ptr::write(pplaylistvariant, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            SetPlayList: SetPlayList::<Identity, Impl, OFFSET>,
            PlayList: PlayList::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITMediaPlayback as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait ITMediaRecord_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn SetFileName(&self, bstrfilename: &windows_core::BSTR) -> windows_core::Result<()>;
    fn FileName(&self) -> windows_core::Result<windows_core::BSTR>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for ITMediaRecord {}
#[cfg(feature = "Win32_System_Com")]
impl ITMediaRecord_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITMediaRecord_Impl, const OFFSET: isize>() -> ITMediaRecord_Vtbl {
        unsafe extern "system" fn SetFileName<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITMediaRecord_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrfilename: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ITMediaRecord_Impl::SetFileName(this, core::mem::transmute(&bstrfilename)).into()
        }
        unsafe extern "system" fn FileName<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITMediaRecord_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrfilename: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ITMediaRecord_Impl::FileName(this) {
                Ok(ok__) => {
                    core::ptr::write(pbstrfilename, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            SetFileName: SetFileName::<Identity, Impl, OFFSET>,
            FileName: FileName::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITMediaRecord as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait ITMediaSupport_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn MediaTypes(&self) -> windows_core::Result<i32>;
    fn QueryMediaType(&self, lmediatype: i32) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for ITMediaSupport {}
#[cfg(feature = "Win32_System_Com")]
impl ITMediaSupport_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITMediaSupport_Impl, const OFFSET: isize>() -> ITMediaSupport_Vtbl {
        unsafe extern "system" fn MediaTypes<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITMediaSupport_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, plmediatypes: *mut i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ITMediaSupport_Impl::MediaTypes(this) {
                Ok(ok__) => {
                    core::ptr::write(plmediatypes, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn QueryMediaType<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITMediaSupport_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lmediatype: i32, pfsupport: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ITMediaSupport_Impl::QueryMediaType(this, core::mem::transmute_copy(&lmediatype)) {
                Ok(ok__) => {
                    core::ptr::write(pfsupport, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            MediaTypes: MediaTypes::<Identity, Impl, OFFSET>,
            QueryMediaType: QueryMediaType::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITMediaSupport as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait ITMultiTrackTerminal_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn TrackTerminals(&self) -> windows_core::Result<windows_core::VARIANT>;
    fn EnumerateTrackTerminals(&self) -> windows_core::Result<IEnumTerminal>;
    fn CreateTrackTerminal(&self, mediatype: i32, terminaldirection: TERMINAL_DIRECTION) -> windows_core::Result<ITTerminal>;
    fn MediaTypesInUse(&self) -> windows_core::Result<i32>;
    fn DirectionsInUse(&self) -> windows_core::Result<TERMINAL_DIRECTION>;
    fn RemoveTrackTerminal(&self, ptrackterminaltoremove: Option<&ITTerminal>) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for ITMultiTrackTerminal {}
#[cfg(feature = "Win32_System_Com")]
impl ITMultiTrackTerminal_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITMultiTrackTerminal_Impl, const OFFSET: isize>() -> ITMultiTrackTerminal_Vtbl {
        unsafe extern "system" fn TrackTerminals<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITMultiTrackTerminal_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvariant: *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ITMultiTrackTerminal_Impl::TrackTerminals(this) {
                Ok(ok__) => {
                    core::ptr::write(pvariant, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnumerateTrackTerminals<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITMultiTrackTerminal_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppenumterminal: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ITMultiTrackTerminal_Impl::EnumerateTrackTerminals(this) {
                Ok(ok__) => {
                    core::ptr::write(ppenumterminal, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateTrackTerminal<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITMultiTrackTerminal_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, mediatype: i32, terminaldirection: TERMINAL_DIRECTION, ppterminal: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ITMultiTrackTerminal_Impl::CreateTrackTerminal(this, core::mem::transmute_copy(&mediatype), core::mem::transmute_copy(&terminaldirection)) {
                Ok(ok__) => {
                    core::ptr::write(ppterminal, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MediaTypesInUse<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITMultiTrackTerminal_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, plmediatypesinuse: *mut i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ITMultiTrackTerminal_Impl::MediaTypesInUse(this) {
                Ok(ok__) => {
                    core::ptr::write(plmediatypesinuse, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DirectionsInUse<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITMultiTrackTerminal_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pldirectionsinused: *mut TERMINAL_DIRECTION) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ITMultiTrackTerminal_Impl::DirectionsInUse(this) {
                Ok(ok__) => {
                    core::ptr::write(pldirectionsinused, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveTrackTerminal<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITMultiTrackTerminal_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ptrackterminaltoremove: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ITMultiTrackTerminal_Impl::RemoveTrackTerminal(this, windows_core::from_raw_borrowed(&ptrackterminaltoremove)).into()
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            TrackTerminals: TrackTerminals::<Identity, Impl, OFFSET>,
            EnumerateTrackTerminals: EnumerateTrackTerminals::<Identity, Impl, OFFSET>,
            CreateTrackTerminal: CreateTrackTerminal::<Identity, Impl, OFFSET>,
            MediaTypesInUse: MediaTypesInUse::<Identity, Impl, OFFSET>,
            DirectionsInUse: DirectionsInUse::<Identity, Impl, OFFSET>,
            RemoveTrackTerminal: RemoveTrackTerminal::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITMultiTrackTerminal as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait ITPhone_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn Open(&self, privilege: PHONE_PRIVILEGE) -> windows_core::Result<()>;
    fn Close(&self) -> windows_core::Result<()>;
    fn Addresses(&self) -> windows_core::Result<windows_core::VARIANT>;
    fn EnumerateAddresses(&self) -> windows_core::Result<IEnumAddress>;
    fn get_PhoneCapsLong(&self, pclcap: PHONECAPS_LONG) -> windows_core::Result<i32>;
    fn get_PhoneCapsString(&self, pcscap: PHONECAPS_STRING) -> windows_core::Result<windows_core::BSTR>;
    fn get_Terminals(&self, paddress: Option<&ITAddress>) -> windows_core::Result<windows_core::VARIANT>;
    fn EnumerateTerminals(&self, paddress: Option<&ITAddress>) -> windows_core::Result<IEnumTerminal>;
    fn get_ButtonMode(&self, lbuttonid: i32) -> windows_core::Result<PHONE_BUTTON_MODE>;
    fn put_ButtonMode(&self, lbuttonid: i32, buttonmode: PHONE_BUTTON_MODE) -> windows_core::Result<()>;
    fn get_ButtonFunction(&self, lbuttonid: i32) -> windows_core::Result<PHONE_BUTTON_FUNCTION>;
    fn put_ButtonFunction(&self, lbuttonid: i32, buttonfunction: PHONE_BUTTON_FUNCTION) -> windows_core::Result<()>;
    fn get_ButtonText(&self, lbuttonid: i32) -> windows_core::Result<windows_core::BSTR>;
    fn put_ButtonText(&self, lbuttonid: i32, bstrbuttontext: &windows_core::BSTR) -> windows_core::Result<()>;
    fn get_ButtonState(&self, lbuttonid: i32) -> windows_core::Result<PHONE_BUTTON_STATE>;
    fn get_HookSwitchState(&self, hookswitchdevice: PHONE_HOOK_SWITCH_DEVICE) -> windows_core::Result<PHONE_HOOK_SWITCH_STATE>;
    fn put_HookSwitchState(&self, hookswitchdevice: PHONE_HOOK_SWITCH_DEVICE, hookswitchstate: PHONE_HOOK_SWITCH_STATE) -> windows_core::Result<()>;
    fn SetRingMode(&self, lringmode: i32) -> windows_core::Result<()>;
    fn RingMode(&self) -> windows_core::Result<i32>;
    fn SetRingVolume(&self, lringvolume: i32) -> windows_core::Result<()>;
    fn RingVolume(&self) -> windows_core::Result<i32>;
    fn Privilege(&self) -> windows_core::Result<PHONE_PRIVILEGE>;
    fn GetPhoneCapsBuffer(&self, pcbcaps: PHONECAPS_BUFFER, pdwsize: *mut u32, ppphonecapsbuffer: *mut *mut u8) -> windows_core::Result<()>;
    fn get_PhoneCapsBuffer(&self, pcbcaps: PHONECAPS_BUFFER) -> windows_core::Result<windows_core::VARIANT>;
    fn get_LampMode(&self, llampid: i32) -> windows_core::Result<PHONE_LAMP_MODE>;
    fn put_LampMode(&self, llampid: i32, lampmode: PHONE_LAMP_MODE) -> windows_core::Result<()>;
    fn Display(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetDisplay(&self, lrow: i32, lcolumn: i32, bstrdisplay: &windows_core::BSTR) -> windows_core::Result<()>;
    fn PreferredAddresses(&self) -> windows_core::Result<windows_core::VARIANT>;
    fn EnumeratePreferredAddresses(&self) -> windows_core::Result<IEnumAddress>;
    fn DeviceSpecific(&self, pparams: *const u8, dwsize: u32) -> windows_core::Result<()>;
    fn DeviceSpecificVariant(&self, vardevspecificbytearray: &windows_core::VARIANT) -> windows_core::Result<()>;
    fn NegotiateExtVersion(&self, llowversion: i32, lhighversion: i32) -> windows_core::Result<i32>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for ITPhone {}
#[cfg(feature = "Win32_System_Com")]
impl ITPhone_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITPhone_Impl, const OFFSET: isize>() -> ITPhone_Vtbl {
        unsafe extern "system" fn Open<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITPhone_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, privilege: PHONE_PRIVILEGE) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ITPhone_Impl::Open(this, core::mem::transmute_copy(&privilege)).into()
        }
        unsafe extern "system" fn Close<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITPhone_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ITPhone_Impl::Close(this).into()
        }
        unsafe extern "system" fn Addresses<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITPhone_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, paddresses: *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ITPhone_Impl::Addresses(this) {
                Ok(ok__) => {
                    core::ptr::write(paddresses, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnumerateAddresses<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITPhone_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppenumaddress: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ITPhone_Impl::EnumerateAddresses(this) {
                Ok(ok__) => {
                    core::ptr::write(ppenumaddress, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn get_PhoneCapsLong<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITPhone_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pclcap: PHONECAPS_LONG, plcapability: *mut i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ITPhone_Impl::get_PhoneCapsLong(this, core::mem::transmute_copy(&pclcap)) {
                Ok(ok__) => {
                    core::ptr::write(plcapability, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn get_PhoneCapsString<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITPhone_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcscap: PHONECAPS_STRING, ppcapability: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ITPhone_Impl::get_PhoneCapsString(this, core::mem::transmute_copy(&pcscap)) {
                Ok(ok__) => {
                    core::ptr::write(ppcapability, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn get_Terminals<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITPhone_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, paddress: *mut core::ffi::c_void, pterminals: *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ITPhone_Impl::get_Terminals(this, windows_core::from_raw_borrowed(&paddress)) {
                Ok(ok__) => {
                    core::ptr::write(pterminals, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnumerateTerminals<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITPhone_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, paddress: *mut core::ffi::c_void, ppenumterminal: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ITPhone_Impl::EnumerateTerminals(this, windows_core::from_raw_borrowed(&paddress)) {
                Ok(ok__) => {
                    core::ptr::write(ppenumterminal, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn get_ButtonMode<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITPhone_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lbuttonid: i32, pbuttonmode: *mut PHONE_BUTTON_MODE) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ITPhone_Impl::get_ButtonMode(this, core::mem::transmute_copy(&lbuttonid)) {
                Ok(ok__) => {
                    core::ptr::write(pbuttonmode, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn put_ButtonMode<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITPhone_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lbuttonid: i32, buttonmode: PHONE_BUTTON_MODE) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ITPhone_Impl::put_ButtonMode(this, core::mem::transmute_copy(&lbuttonid), core::mem::transmute_copy(&buttonmode)).into()
        }
        unsafe extern "system" fn get_ButtonFunction<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITPhone_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lbuttonid: i32, pbuttonfunction: *mut PHONE_BUTTON_FUNCTION) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ITPhone_Impl::get_ButtonFunction(this, core::mem::transmute_copy(&lbuttonid)) {
                Ok(ok__) => {
                    core::ptr::write(pbuttonfunction, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn put_ButtonFunction<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITPhone_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lbuttonid: i32, buttonfunction: PHONE_BUTTON_FUNCTION) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ITPhone_Impl::put_ButtonFunction(this, core::mem::transmute_copy(&lbuttonid), core::mem::transmute_copy(&buttonfunction)).into()
        }
        unsafe extern "system" fn get_ButtonText<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITPhone_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lbuttonid: i32, ppbuttontext: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ITPhone_Impl::get_ButtonText(this, core::mem::transmute_copy(&lbuttonid)) {
                Ok(ok__) => {
                    core::ptr::write(ppbuttontext, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn put_ButtonText<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITPhone_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lbuttonid: i32, bstrbuttontext: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ITPhone_Impl::put_ButtonText(this, core::mem::transmute_copy(&lbuttonid), core::mem::transmute(&bstrbuttontext)).into()
        }
        unsafe extern "system" fn get_ButtonState<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITPhone_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lbuttonid: i32, pbuttonstate: *mut PHONE_BUTTON_STATE) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ITPhone_Impl::get_ButtonState(this, core::mem::transmute_copy(&lbuttonid)) {
                Ok(ok__) => {
                    core::ptr::write(pbuttonstate, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn get_HookSwitchState<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITPhone_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hookswitchdevice: PHONE_HOOK_SWITCH_DEVICE, phookswitchstate: *mut PHONE_HOOK_SWITCH_STATE) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ITPhone_Impl::get_HookSwitchState(this, core::mem::transmute_copy(&hookswitchdevice)) {
                Ok(ok__) => {
                    core::ptr::write(phookswitchstate, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn put_HookSwitchState<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITPhone_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hookswitchdevice: PHONE_HOOK_SWITCH_DEVICE, hookswitchstate: PHONE_HOOK_SWITCH_STATE) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ITPhone_Impl::put_HookSwitchState(this, core::mem::transmute_copy(&hookswitchdevice), core::mem::transmute_copy(&hookswitchstate)).into()
        }
        unsafe extern "system" fn SetRingMode<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITPhone_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lringmode: i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ITPhone_Impl::SetRingMode(this, core::mem::transmute_copy(&lringmode)).into()
        }
        unsafe extern "system" fn RingMode<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITPhone_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, plringmode: *mut i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ITPhone_Impl::RingMode(this) {
                Ok(ok__) => {
                    core::ptr::write(plringmode, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetRingVolume<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITPhone_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lringvolume: i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ITPhone_Impl::SetRingVolume(this, core::mem::transmute_copy(&lringvolume)).into()
        }
        unsafe extern "system" fn RingVolume<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITPhone_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, plringvolume: *mut i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ITPhone_Impl::RingVolume(this) {
                Ok(ok__) => {
                    core::ptr::write(plringvolume, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Privilege<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITPhone_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pprivilege: *mut PHONE_PRIVILEGE) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ITPhone_Impl::Privilege(this) {
                Ok(ok__) => {
                    core::ptr::write(pprivilege, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPhoneCapsBuffer<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITPhone_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcbcaps: PHONECAPS_BUFFER, pdwsize: *mut u32, ppphonecapsbuffer: *mut *mut u8) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ITPhone_Impl::GetPhoneCapsBuffer(this, core::mem::transmute_copy(&pcbcaps), core::mem::transmute_copy(&pdwsize), core::mem::transmute_copy(&ppphonecapsbuffer)).into()
        }
        unsafe extern "system" fn get_PhoneCapsBuffer<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITPhone_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcbcaps: PHONECAPS_BUFFER, pvarbuffer: *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ITPhone_Impl::get_PhoneCapsBuffer(this, core::mem::transmute_copy(&pcbcaps)) {
                Ok(ok__) => {
                    core::ptr::write(pvarbuffer, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn get_LampMode<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITPhone_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, llampid: i32, plampmode: *mut PHONE_LAMP_MODE) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ITPhone_Impl::get_LampMode(this, core::mem::transmute_copy(&llampid)) {
                Ok(ok__) => {
                    core::ptr::write(plampmode, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn put_LampMode<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITPhone_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, llampid: i32, lampmode: PHONE_LAMP_MODE) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ITPhone_Impl::put_LampMode(this, core::mem::transmute_copy(&llampid), core::mem::transmute_copy(&lampmode)).into()
        }
        unsafe extern "system" fn Display<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITPhone_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrdisplay: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ITPhone_Impl::Display(this) {
                Ok(ok__) => {
                    core::ptr::write(pbstrdisplay, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDisplay<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITPhone_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lrow: i32, lcolumn: i32, bstrdisplay: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ITPhone_Impl::SetDisplay(this, core::mem::transmute_copy(&lrow), core::mem::transmute_copy(&lcolumn), core::mem::transmute(&bstrdisplay)).into()
        }
        unsafe extern "system" fn PreferredAddresses<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITPhone_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, paddresses: *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ITPhone_Impl::PreferredAddresses(this) {
                Ok(ok__) => {
                    core::ptr::write(paddresses, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnumeratePreferredAddresses<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITPhone_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppenumaddress: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ITPhone_Impl::EnumeratePreferredAddresses(this) {
                Ok(ok__) => {
                    core::ptr::write(ppenumaddress, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DeviceSpecific<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITPhone_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pparams: *const u8, dwsize: u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ITPhone_Impl::DeviceSpecific(this, core::mem::transmute_copy(&pparams), core::mem::transmute_copy(&dwsize)).into()
        }
        unsafe extern "system" fn DeviceSpecificVariant<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITPhone_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, vardevspecificbytearray: core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ITPhone_Impl::DeviceSpecificVariant(this, core::mem::transmute(&vardevspecificbytearray)).into()
        }
        unsafe extern "system" fn NegotiateExtVersion<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITPhone_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, llowversion: i32, lhighversion: i32, plextversion: *mut i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ITPhone_Impl::NegotiateExtVersion(this, core::mem::transmute_copy(&llowversion), core::mem::transmute_copy(&lhighversion)) {
                Ok(ok__) => {
                    core::ptr::write(plextversion, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            Open: Open::<Identity, Impl, OFFSET>,
            Close: Close::<Identity, Impl, OFFSET>,
            Addresses: Addresses::<Identity, Impl, OFFSET>,
            EnumerateAddresses: EnumerateAddresses::<Identity, Impl, OFFSET>,
            get_PhoneCapsLong: get_PhoneCapsLong::<Identity, Impl, OFFSET>,
            get_PhoneCapsString: get_PhoneCapsString::<Identity, Impl, OFFSET>,
            get_Terminals: get_Terminals::<Identity, Impl, OFFSET>,
            EnumerateTerminals: EnumerateTerminals::<Identity, Impl, OFFSET>,
            get_ButtonMode: get_ButtonMode::<Identity, Impl, OFFSET>,
            put_ButtonMode: put_ButtonMode::<Identity, Impl, OFFSET>,
            get_ButtonFunction: get_ButtonFunction::<Identity, Impl, OFFSET>,
            put_ButtonFunction: put_ButtonFunction::<Identity, Impl, OFFSET>,
            get_ButtonText: get_ButtonText::<Identity, Impl, OFFSET>,
            put_ButtonText: put_ButtonText::<Identity, Impl, OFFSET>,
            get_ButtonState: get_ButtonState::<Identity, Impl, OFFSET>,
            get_HookSwitchState: get_HookSwitchState::<Identity, Impl, OFFSET>,
            put_HookSwitchState: put_HookSwitchState::<Identity, Impl, OFFSET>,
            SetRingMode: SetRingMode::<Identity, Impl, OFFSET>,
            RingMode: RingMode::<Identity, Impl, OFFSET>,
            SetRingVolume: SetRingVolume::<Identity, Impl, OFFSET>,
            RingVolume: RingVolume::<Identity, Impl, OFFSET>,
            Privilege: Privilege::<Identity, Impl, OFFSET>,
            GetPhoneCapsBuffer: GetPhoneCapsBuffer::<Identity, Impl, OFFSET>,
            get_PhoneCapsBuffer: get_PhoneCapsBuffer::<Identity, Impl, OFFSET>,
            get_LampMode: get_LampMode::<Identity, Impl, OFFSET>,
            put_LampMode: put_LampMode::<Identity, Impl, OFFSET>,
            Display: Display::<Identity, Impl, OFFSET>,
            SetDisplay: SetDisplay::<Identity, Impl, OFFSET>,
            PreferredAddresses: PreferredAddresses::<Identity, Impl, OFFSET>,
            EnumeratePreferredAddresses: EnumeratePreferredAddresses::<Identity, Impl, OFFSET>,
            DeviceSpecific: DeviceSpecific::<Identity, Impl, OFFSET>,
            DeviceSpecificVariant: DeviceSpecificVariant::<Identity, Impl, OFFSET>,
            NegotiateExtVersion: NegotiateExtVersion::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITPhone as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait ITPhoneDeviceSpecificEvent_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn Phone(&self) -> windows_core::Result<ITPhone>;
    fn lParam1(&self) -> windows_core::Result<i32>;
    fn lParam2(&self) -> windows_core::Result<i32>;
    fn lParam3(&self) -> windows_core::Result<i32>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for ITPhoneDeviceSpecificEvent {}
#[cfg(feature = "Win32_System_Com")]
impl ITPhoneDeviceSpecificEvent_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITPhoneDeviceSpecificEvent_Impl, const OFFSET: isize>() -> ITPhoneDeviceSpecificEvent_Vtbl {
        unsafe extern "system" fn Phone<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITPhoneDeviceSpecificEvent_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppphone: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ITPhoneDeviceSpecificEvent_Impl::Phone(this) {
                Ok(ok__) => {
                    core::ptr::write(ppphone, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn lParam1<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITPhoneDeviceSpecificEvent_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pparam1: *mut i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ITPhoneDeviceSpecificEvent_Impl::lParam1(this) {
                Ok(ok__) => {
                    core::ptr::write(pparam1, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn lParam2<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITPhoneDeviceSpecificEvent_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pparam2: *mut i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ITPhoneDeviceSpecificEvent_Impl::lParam2(this) {
                Ok(ok__) => {
                    core::ptr::write(pparam2, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn lParam3<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITPhoneDeviceSpecificEvent_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pparam3: *mut i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ITPhoneDeviceSpecificEvent_Impl::lParam3(this) {
                Ok(ok__) => {
                    core::ptr::write(pparam3, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            Phone: Phone::<Identity, Impl, OFFSET>,
            lParam1: lParam1::<Identity, Impl, OFFSET>,
            lParam2: lParam2::<Identity, Impl, OFFSET>,
            lParam3: lParam3::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITPhoneDeviceSpecificEvent as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait ITPhoneEvent_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn Phone(&self) -> windows_core::Result<ITPhone>;
    fn Event(&self) -> windows_core::Result<PHONE_EVENT>;
    fn ButtonState(&self) -> windows_core::Result<PHONE_BUTTON_STATE>;
    fn HookSwitchState(&self) -> windows_core::Result<PHONE_HOOK_SWITCH_STATE>;
    fn HookSwitchDevice(&self) -> windows_core::Result<PHONE_HOOK_SWITCH_DEVICE>;
    fn RingMode(&self) -> windows_core::Result<i32>;
    fn ButtonLampId(&self) -> windows_core::Result<i32>;
    fn NumberGathered(&self) -> windows_core::Result<windows_core::BSTR>;
    fn Call(&self) -> windows_core::Result<ITCallInfo>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for ITPhoneEvent {}
#[cfg(feature = "Win32_System_Com")]
impl ITPhoneEvent_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITPhoneEvent_Impl, const OFFSET: isize>() -> ITPhoneEvent_Vtbl {
        unsafe extern "system" fn Phone<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITPhoneEvent_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppphone: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ITPhoneEvent_Impl::Phone(this) {
                Ok(ok__) => {
                    core::ptr::write(ppphone, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Event<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITPhoneEvent_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pevent: *mut PHONE_EVENT) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ITPhoneEvent_Impl::Event(this) {
                Ok(ok__) => {
                    core::ptr::write(pevent, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ButtonState<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITPhoneEvent_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pstate: *mut PHONE_BUTTON_STATE) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ITPhoneEvent_Impl::ButtonState(this) {
                Ok(ok__) => {
                    core::ptr::write(pstate, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn HookSwitchState<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITPhoneEvent_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pstate: *mut PHONE_HOOK_SWITCH_STATE) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ITPhoneEvent_Impl::HookSwitchState(this) {
                Ok(ok__) => {
                    core::ptr::write(pstate, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn HookSwitchDevice<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITPhoneEvent_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdevice: *mut PHONE_HOOK_SWITCH_DEVICE) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ITPhoneEvent_Impl::HookSwitchDevice(this) {
                Ok(ok__) => {
                    core::ptr::write(pdevice, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RingMode<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITPhoneEvent_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, plringmode: *mut i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ITPhoneEvent_Impl::RingMode(this) {
                Ok(ok__) => {
                    core::ptr::write(plringmode, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ButtonLampId<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITPhoneEvent_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, plbuttonlampid: *mut i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ITPhoneEvent_Impl::ButtonLampId(this) {
                Ok(ok__) => {
                    core::ptr::write(plbuttonlampid, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn NumberGathered<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITPhoneEvent_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppnumber: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ITPhoneEvent_Impl::NumberGathered(this) {
                Ok(ok__) => {
                    core::ptr::write(ppnumber, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Call<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITPhoneEvent_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppcallinfo: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ITPhoneEvent_Impl::Call(this) {
                Ok(ok__) => {
                    core::ptr::write(ppcallinfo, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            Phone: Phone::<Identity, Impl, OFFSET>,
            Event: Event::<Identity, Impl, OFFSET>,
            ButtonState: ButtonState::<Identity, Impl, OFFSET>,
            HookSwitchState: HookSwitchState::<Identity, Impl, OFFSET>,
            HookSwitchDevice: HookSwitchDevice::<Identity, Impl, OFFSET>,
            RingMode: RingMode::<Identity, Impl, OFFSET>,
            ButtonLampId: ButtonLampId::<Identity, Impl, OFFSET>,
            NumberGathered: NumberGathered::<Identity, Impl, OFFSET>,
            Call: Call::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITPhoneEvent as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait ITPluggableTerminalClassInfo_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn Name(&self) -> windows_core::Result<windows_core::BSTR>;
    fn Company(&self) -> windows_core::Result<windows_core::BSTR>;
    fn Version(&self) -> windows_core::Result<windows_core::BSTR>;
    fn TerminalClass(&self) -> windows_core::Result<windows_core::BSTR>;
    fn CLSID(&self) -> windows_core::Result<windows_core::BSTR>;
    fn Direction(&self) -> windows_core::Result<TERMINAL_DIRECTION>;
    fn MediaTypes(&self) -> windows_core::Result<i32>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for ITPluggableTerminalClassInfo {}
#[cfg(feature = "Win32_System_Com")]
impl ITPluggableTerminalClassInfo_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITPluggableTerminalClassInfo_Impl, const OFFSET: isize>() -> ITPluggableTerminalClassInfo_Vtbl {
        unsafe extern "system" fn Name<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITPluggableTerminalClassInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pname: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ITPluggableTerminalClassInfo_Impl::Name(this) {
                Ok(ok__) => {
                    core::ptr::write(pname, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Company<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITPluggableTerminalClassInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcompany: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ITPluggableTerminalClassInfo_Impl::Company(this) {
                Ok(ok__) => {
                    core::ptr::write(pcompany, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Version<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITPluggableTerminalClassInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pversion: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ITPluggableTerminalClassInfo_Impl::Version(this) {
                Ok(ok__) => {
                    core::ptr::write(pversion, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TerminalClass<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITPluggableTerminalClassInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pterminalclass: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ITPluggableTerminalClassInfo_Impl::TerminalClass(this) {
                Ok(ok__) => {
                    core::ptr::write(pterminalclass, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CLSID<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITPluggableTerminalClassInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pclsid: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ITPluggableTerminalClassInfo_Impl::CLSID(this) {
                Ok(ok__) => {
                    core::ptr::write(pclsid, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Direction<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITPluggableTerminalClassInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdirection: *mut TERMINAL_DIRECTION) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ITPluggableTerminalClassInfo_Impl::Direction(this) {
                Ok(ok__) => {
                    core::ptr::write(pdirection, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MediaTypes<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITPluggableTerminalClassInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pmediatypes: *mut i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ITPluggableTerminalClassInfo_Impl::MediaTypes(this) {
                Ok(ok__) => {
                    core::ptr::write(pmediatypes, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            Name: Name::<Identity, Impl, OFFSET>,
            Company: Company::<Identity, Impl, OFFSET>,
            Version: Version::<Identity, Impl, OFFSET>,
            TerminalClass: TerminalClass::<Identity, Impl, OFFSET>,
            CLSID: CLSID::<Identity, Impl, OFFSET>,
            Direction: Direction::<Identity, Impl, OFFSET>,
            MediaTypes: MediaTypes::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITPluggableTerminalClassInfo as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait ITPluggableTerminalEventSink_Impl: Sized {
    fn FireEvent(&self, pmspeventinfo: *const MSP_EVENT_INFO) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for ITPluggableTerminalEventSink {}
#[cfg(feature = "Win32_System_Com")]
impl ITPluggableTerminalEventSink_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITPluggableTerminalEventSink_Impl, const OFFSET: isize>() -> ITPluggableTerminalEventSink_Vtbl {
        unsafe extern "system" fn FireEvent<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITPluggableTerminalEventSink_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pmspeventinfo: *const MSP_EVENT_INFO) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ITPluggableTerminalEventSink_Impl::FireEvent(this, core::mem::transmute_copy(&pmspeventinfo)).into()
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), FireEvent: FireEvent::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITPluggableTerminalEventSink as windows_core::Interface>::IID
    }
}
pub trait ITPluggableTerminalEventSinkRegistration_Impl: Sized {
    fn RegisterSink(&self, peventsink: Option<&ITPluggableTerminalEventSink>) -> windows_core::Result<()>;
    fn UnregisterSink(&self) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for ITPluggableTerminalEventSinkRegistration {}
impl ITPluggableTerminalEventSinkRegistration_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITPluggableTerminalEventSinkRegistration_Impl, const OFFSET: isize>() -> ITPluggableTerminalEventSinkRegistration_Vtbl {
        unsafe extern "system" fn RegisterSink<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITPluggableTerminalEventSinkRegistration_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, peventsink: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ITPluggableTerminalEventSinkRegistration_Impl::RegisterSink(this, windows_core::from_raw_borrowed(&peventsink)).into()
        }
        unsafe extern "system" fn UnregisterSink<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITPluggableTerminalEventSinkRegistration_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ITPluggableTerminalEventSinkRegistration_Impl::UnregisterSink(this).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            RegisterSink: RegisterSink::<Identity, Impl, OFFSET>,
            UnregisterSink: UnregisterSink::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITPluggableTerminalEventSinkRegistration as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait ITPluggableTerminalSuperclassInfo_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn Name(&self) -> windows_core::Result<windows_core::BSTR>;
    fn CLSID(&self) -> windows_core::Result<windows_core::BSTR>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for ITPluggableTerminalSuperclassInfo {}
#[cfg(feature = "Win32_System_Com")]
impl ITPluggableTerminalSuperclassInfo_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITPluggableTerminalSuperclassInfo_Impl, const OFFSET: isize>() -> ITPluggableTerminalSuperclassInfo_Vtbl {
        unsafe extern "system" fn Name<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITPluggableTerminalSuperclassInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pname: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ITPluggableTerminalSuperclassInfo_Impl::Name(this) {
                Ok(ok__) => {
                    core::ptr::write(pname, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CLSID<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITPluggableTerminalSuperclassInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pclsid: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ITPluggableTerminalSuperclassInfo_Impl::CLSID(this) {
                Ok(ok__) => {
                    core::ptr::write(pclsid, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            Name: Name::<Identity, Impl, OFFSET>,
            CLSID: CLSID::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITPluggableTerminalSuperclassInfo as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait ITPrivateEvent_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn Address(&self) -> windows_core::Result<ITAddress>;
    fn Call(&self) -> windows_core::Result<ITCallInfo>;
    fn CallHub(&self) -> windows_core::Result<ITCallHub>;
    fn EventCode(&self) -> windows_core::Result<i32>;
    fn EventInterface(&self) -> windows_core::Result<super::super::System::Com::IDispatch>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for ITPrivateEvent {}
#[cfg(feature = "Win32_System_Com")]
impl ITPrivateEvent_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITPrivateEvent_Impl, const OFFSET: isize>() -> ITPrivateEvent_Vtbl {
        unsafe extern "system" fn Address<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITPrivateEvent_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppaddress: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ITPrivateEvent_Impl::Address(this) {
                Ok(ok__) => {
                    core::ptr::write(ppaddress, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Call<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITPrivateEvent_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppcallinfo: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ITPrivateEvent_Impl::Call(this) {
                Ok(ok__) => {
                    core::ptr::write(ppcallinfo, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CallHub<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITPrivateEvent_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppcallhub: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ITPrivateEvent_Impl::CallHub(this) {
                Ok(ok__) => {
                    core::ptr::write(ppcallhub, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EventCode<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITPrivateEvent_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pleventcode: *mut i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ITPrivateEvent_Impl::EventCode(this) {
                Ok(ok__) => {
                    core::ptr::write(pleventcode, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EventInterface<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITPrivateEvent_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, peventinterface: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ITPrivateEvent_Impl::EventInterface(this) {
                Ok(ok__) => {
                    core::ptr::write(peventinterface, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            Address: Address::<Identity, Impl, OFFSET>,
            Call: Call::<Identity, Impl, OFFSET>,
            CallHub: CallHub::<Identity, Impl, OFFSET>,
            EventCode: EventCode::<Identity, Impl, OFFSET>,
            EventInterface: EventInterface::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITPrivateEvent as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait ITQOSEvent_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn Call(&self) -> windows_core::Result<ITCallInfo>;
    fn Event(&self) -> windows_core::Result<QOS_EVENT>;
    fn MediaType(&self) -> windows_core::Result<i32>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for ITQOSEvent {}
#[cfg(feature = "Win32_System_Com")]
impl ITQOSEvent_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITQOSEvent_Impl, const OFFSET: isize>() -> ITQOSEvent_Vtbl {
        unsafe extern "system" fn Call<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITQOSEvent_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppcall: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ITQOSEvent_Impl::Call(this) {
                Ok(ok__) => {
                    core::ptr::write(ppcall, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Event<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITQOSEvent_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pqosevent: *mut QOS_EVENT) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ITQOSEvent_Impl::Event(this) {
                Ok(ok__) => {
                    core::ptr::write(pqosevent, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MediaType<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITQOSEvent_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, plmediatype: *mut i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ITQOSEvent_Impl::MediaType(this) {
                Ok(ok__) => {
                    core::ptr::write(plmediatype, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            Call: Call::<Identity, Impl, OFFSET>,
            Event: Event::<Identity, Impl, OFFSET>,
            MediaType: MediaType::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITQOSEvent as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait ITQueue_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn SetMeasurementPeriod(&self, lperiod: i32) -> windows_core::Result<()>;
    fn MeasurementPeriod(&self) -> windows_core::Result<i32>;
    fn TotalCallsQueued(&self) -> windows_core::Result<i32>;
    fn CurrentCallsQueued(&self) -> windows_core::Result<i32>;
    fn TotalCallsAbandoned(&self) -> windows_core::Result<i32>;
    fn TotalCallsFlowedIn(&self) -> windows_core::Result<i32>;
    fn TotalCallsFlowedOut(&self) -> windows_core::Result<i32>;
    fn LongestEverWaitTime(&self) -> windows_core::Result<i32>;
    fn CurrentLongestWaitTime(&self) -> windows_core::Result<i32>;
    fn AverageWaitTime(&self) -> windows_core::Result<i32>;
    fn FinalDisposition(&self) -> windows_core::Result<i32>;
    fn Name(&self) -> windows_core::Result<windows_core::BSTR>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for ITQueue {}
#[cfg(feature = "Win32_System_Com")]
impl ITQueue_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITQueue_Impl, const OFFSET: isize>() -> ITQueue_Vtbl {
        unsafe extern "system" fn SetMeasurementPeriod<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITQueue_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lperiod: i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ITQueue_Impl::SetMeasurementPeriod(this, core::mem::transmute_copy(&lperiod)).into()
        }
        unsafe extern "system" fn MeasurementPeriod<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITQueue_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, plperiod: *mut i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ITQueue_Impl::MeasurementPeriod(this) {
                Ok(ok__) => {
                    core::ptr::write(plperiod, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TotalCallsQueued<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITQueue_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, plcalls: *mut i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ITQueue_Impl::TotalCallsQueued(this) {
                Ok(ok__) => {
                    core::ptr::write(plcalls, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CurrentCallsQueued<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITQueue_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, plcalls: *mut i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ITQueue_Impl::CurrentCallsQueued(this) {
                Ok(ok__) => {
                    core::ptr::write(plcalls, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TotalCallsAbandoned<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITQueue_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, plcalls: *mut i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ITQueue_Impl::TotalCallsAbandoned(this) {
                Ok(ok__) => {
                    core::ptr::write(plcalls, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TotalCallsFlowedIn<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITQueue_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, plcalls: *mut i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ITQueue_Impl::TotalCallsFlowedIn(this) {
                Ok(ok__) => {
                    core::ptr::write(plcalls, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TotalCallsFlowedOut<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITQueue_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, plcalls: *mut i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ITQueue_Impl::TotalCallsFlowedOut(this) {
                Ok(ok__) => {
                    core::ptr::write(plcalls, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LongestEverWaitTime<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITQueue_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, plwaittime: *mut i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ITQueue_Impl::LongestEverWaitTime(this) {
                Ok(ok__) => {
                    core::ptr::write(plwaittime, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CurrentLongestWaitTime<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITQueue_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, plwaittime: *mut i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ITQueue_Impl::CurrentLongestWaitTime(this) {
                Ok(ok__) => {
                    core::ptr::write(plwaittime, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AverageWaitTime<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITQueue_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, plwaittime: *mut i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ITQueue_Impl::AverageWaitTime(this) {
                Ok(ok__) => {
                    core::ptr::write(plwaittime, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FinalDisposition<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITQueue_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, plcalls: *mut i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ITQueue_Impl::FinalDisposition(this) {
                Ok(ok__) => {
                    core::ptr::write(plcalls, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Name<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITQueue_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppname: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ITQueue_Impl::Name(this) {
                Ok(ok__) => {
                    core::ptr::write(ppname, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            SetMeasurementPeriod: SetMeasurementPeriod::<Identity, Impl, OFFSET>,
            MeasurementPeriod: MeasurementPeriod::<Identity, Impl, OFFSET>,
            TotalCallsQueued: TotalCallsQueued::<Identity, Impl, OFFSET>,
            CurrentCallsQueued: CurrentCallsQueued::<Identity, Impl, OFFSET>,
            TotalCallsAbandoned: TotalCallsAbandoned::<Identity, Impl, OFFSET>,
            TotalCallsFlowedIn: TotalCallsFlowedIn::<Identity, Impl, OFFSET>,
            TotalCallsFlowedOut: TotalCallsFlowedOut::<Identity, Impl, OFFSET>,
            LongestEverWaitTime: LongestEverWaitTime::<Identity, Impl, OFFSET>,
            CurrentLongestWaitTime: CurrentLongestWaitTime::<Identity, Impl, OFFSET>,
            AverageWaitTime: AverageWaitTime::<Identity, Impl, OFFSET>,
            FinalDisposition: FinalDisposition::<Identity, Impl, OFFSET>,
            Name: Name::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITQueue as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait ITQueueEvent_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn Queue(&self) -> windows_core::Result<ITQueue>;
    fn Event(&self) -> windows_core::Result<ACDQUEUE_EVENT>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for ITQueueEvent {}
#[cfg(feature = "Win32_System_Com")]
impl ITQueueEvent_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITQueueEvent_Impl, const OFFSET: isize>() -> ITQueueEvent_Vtbl {
        unsafe extern "system" fn Queue<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITQueueEvent_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppqueue: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ITQueueEvent_Impl::Queue(this) {
                Ok(ok__) => {
                    core::ptr::write(ppqueue, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Event<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITQueueEvent_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pevent: *mut ACDQUEUE_EVENT) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ITQueueEvent_Impl::Event(this) {
                Ok(ok__) => {
                    core::ptr::write(pevent, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            Queue: Queue::<Identity, Impl, OFFSET>,
            Event: Event::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITQueueEvent as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait ITRendezvous_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn DefaultDirectories(&self) -> windows_core::Result<windows_core::VARIANT>;
    fn EnumerateDefaultDirectories(&self) -> windows_core::Result<IEnumDirectory>;
    fn CreateDirectory(&self, directorytype: DIRECTORY_TYPE, pname: &windows_core::BSTR) -> windows_core::Result<ITDirectory>;
    fn CreateDirectoryObject(&self, directoryobjecttype: DIRECTORY_OBJECT_TYPE, pname: &windows_core::BSTR) -> windows_core::Result<ITDirectoryObject>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for ITRendezvous {}
#[cfg(feature = "Win32_System_Com")]
impl ITRendezvous_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITRendezvous_Impl, const OFFSET: isize>() -> ITRendezvous_Vtbl {
        unsafe extern "system" fn DefaultDirectories<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITRendezvous_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvariant: *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ITRendezvous_Impl::DefaultDirectories(this) {
                Ok(ok__) => {
                    core::ptr::write(pvariant, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnumerateDefaultDirectories<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITRendezvous_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppenumdirectory: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ITRendezvous_Impl::EnumerateDefaultDirectories(this) {
                Ok(ok__) => {
                    core::ptr::write(ppenumdirectory, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateDirectory<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITRendezvous_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, directorytype: DIRECTORY_TYPE, pname: core::mem::MaybeUninit<windows_core::BSTR>, ppdir: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ITRendezvous_Impl::CreateDirectory(this, core::mem::transmute_copy(&directorytype), core::mem::transmute(&pname)) {
                Ok(ok__) => {
                    core::ptr::write(ppdir, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateDirectoryObject<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITRendezvous_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, directoryobjecttype: DIRECTORY_OBJECT_TYPE, pname: core::mem::MaybeUninit<windows_core::BSTR>, ppdirectoryobject: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ITRendezvous_Impl::CreateDirectoryObject(this, core::mem::transmute_copy(&directoryobjecttype), core::mem::transmute(&pname)) {
                Ok(ok__) => {
                    core::ptr::write(ppdirectoryobject, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            DefaultDirectories: DefaultDirectories::<Identity, Impl, OFFSET>,
            EnumerateDefaultDirectories: EnumerateDefaultDirectories::<Identity, Impl, OFFSET>,
            CreateDirectory: CreateDirectory::<Identity, Impl, OFFSET>,
            CreateDirectoryObject: CreateDirectoryObject::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITRendezvous as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait ITRequest_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn MakeCall(&self, pdestaddress: &windows_core::BSTR, pappname: &windows_core::BSTR, pcalledparty: &windows_core::BSTR, pcomment: &windows_core::BSTR) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for ITRequest {}
#[cfg(feature = "Win32_System_Com")]
impl ITRequest_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITRequest_Impl, const OFFSET: isize>() -> ITRequest_Vtbl {
        unsafe extern "system" fn MakeCall<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITRequest_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdestaddress: core::mem::MaybeUninit<windows_core::BSTR>, pappname: core::mem::MaybeUninit<windows_core::BSTR>, pcalledparty: core::mem::MaybeUninit<windows_core::BSTR>, pcomment: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ITRequest_Impl::MakeCall(this, core::mem::transmute(&pdestaddress), core::mem::transmute(&pappname), core::mem::transmute(&pcalledparty), core::mem::transmute(&pcomment)).into()
        }
        Self { base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(), MakeCall: MakeCall::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITRequest as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait ITRequestEvent_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn RegistrationInstance(&self) -> windows_core::Result<i32>;
    fn RequestMode(&self) -> windows_core::Result<i32>;
    fn DestAddress(&self) -> windows_core::Result<windows_core::BSTR>;
    fn AppName(&self) -> windows_core::Result<windows_core::BSTR>;
    fn CalledParty(&self) -> windows_core::Result<windows_core::BSTR>;
    fn Comment(&self) -> windows_core::Result<windows_core::BSTR>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for ITRequestEvent {}
#[cfg(feature = "Win32_System_Com")]
impl ITRequestEvent_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITRequestEvent_Impl, const OFFSET: isize>() -> ITRequestEvent_Vtbl {
        unsafe extern "system" fn RegistrationInstance<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITRequestEvent_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, plregistrationinstance: *mut i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ITRequestEvent_Impl::RegistrationInstance(this) {
                Ok(ok__) => {
                    core::ptr::write(plregistrationinstance, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RequestMode<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITRequestEvent_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, plrequestmode: *mut i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ITRequestEvent_Impl::RequestMode(this) {
                Ok(ok__) => {
                    core::ptr::write(plrequestmode, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DestAddress<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITRequestEvent_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppdestaddress: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ITRequestEvent_Impl::DestAddress(this) {
                Ok(ok__) => {
                    core::ptr::write(ppdestaddress, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AppName<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITRequestEvent_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppappname: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ITRequestEvent_Impl::AppName(this) {
                Ok(ok__) => {
                    core::ptr::write(ppappname, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CalledParty<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITRequestEvent_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppcalledparty: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ITRequestEvent_Impl::CalledParty(this) {
                Ok(ok__) => {
                    core::ptr::write(ppcalledparty, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Comment<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITRequestEvent_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppcomment: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ITRequestEvent_Impl::Comment(this) {
                Ok(ok__) => {
                    core::ptr::write(ppcomment, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            RegistrationInstance: RegistrationInstance::<Identity, Impl, OFFSET>,
            RequestMode: RequestMode::<Identity, Impl, OFFSET>,
            DestAddress: DestAddress::<Identity, Impl, OFFSET>,
            AppName: AppName::<Identity, Impl, OFFSET>,
            CalledParty: CalledParty::<Identity, Impl, OFFSET>,
            Comment: Comment::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITRequestEvent as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait ITScriptableAudioFormat_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn Channels(&self) -> windows_core::Result<i32>;
    fn SetChannels(&self, nnewval: i32) -> windows_core::Result<()>;
    fn SamplesPerSec(&self) -> windows_core::Result<i32>;
    fn SetSamplesPerSec(&self, nnewval: i32) -> windows_core::Result<()>;
    fn AvgBytesPerSec(&self) -> windows_core::Result<i32>;
    fn SetAvgBytesPerSec(&self, nnewval: i32) -> windows_core::Result<()>;
    fn BlockAlign(&self) -> windows_core::Result<i32>;
    fn SetBlockAlign(&self, nnewval: i32) -> windows_core::Result<()>;
    fn BitsPerSample(&self) -> windows_core::Result<i32>;
    fn SetBitsPerSample(&self, nnewval: i32) -> windows_core::Result<()>;
    fn FormatTag(&self) -> windows_core::Result<i32>;
    fn SetFormatTag(&self, nnewval: i32) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for ITScriptableAudioFormat {}
#[cfg(feature = "Win32_System_Com")]
impl ITScriptableAudioFormat_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITScriptableAudioFormat_Impl, const OFFSET: isize>() -> ITScriptableAudioFormat_Vtbl {
        unsafe extern "system" fn Channels<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITScriptableAudioFormat_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ITScriptableAudioFormat_Impl::Channels(this) {
                Ok(ok__) => {
                    core::ptr::write(pval, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetChannels<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITScriptableAudioFormat_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, nnewval: i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ITScriptableAudioFormat_Impl::SetChannels(this, core::mem::transmute_copy(&nnewval)).into()
        }
        unsafe extern "system" fn SamplesPerSec<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITScriptableAudioFormat_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ITScriptableAudioFormat_Impl::SamplesPerSec(this) {
                Ok(ok__) => {
                    core::ptr::write(pval, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSamplesPerSec<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITScriptableAudioFormat_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, nnewval: i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ITScriptableAudioFormat_Impl::SetSamplesPerSec(this, core::mem::transmute_copy(&nnewval)).into()
        }
        unsafe extern "system" fn AvgBytesPerSec<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITScriptableAudioFormat_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ITScriptableAudioFormat_Impl::AvgBytesPerSec(this) {
                Ok(ok__) => {
                    core::ptr::write(pval, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAvgBytesPerSec<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITScriptableAudioFormat_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, nnewval: i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ITScriptableAudioFormat_Impl::SetAvgBytesPerSec(this, core::mem::transmute_copy(&nnewval)).into()
        }
        unsafe extern "system" fn BlockAlign<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITScriptableAudioFormat_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ITScriptableAudioFormat_Impl::BlockAlign(this) {
                Ok(ok__) => {
                    core::ptr::write(pval, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetBlockAlign<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITScriptableAudioFormat_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, nnewval: i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ITScriptableAudioFormat_Impl::SetBlockAlign(this, core::mem::transmute_copy(&nnewval)).into()
        }
        unsafe extern "system" fn BitsPerSample<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITScriptableAudioFormat_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ITScriptableAudioFormat_Impl::BitsPerSample(this) {
                Ok(ok__) => {
                    core::ptr::write(pval, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetBitsPerSample<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITScriptableAudioFormat_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, nnewval: i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ITScriptableAudioFormat_Impl::SetBitsPerSample(this, core::mem::transmute_copy(&nnewval)).into()
        }
        unsafe extern "system" fn FormatTag<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITScriptableAudioFormat_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ITScriptableAudioFormat_Impl::FormatTag(this) {
                Ok(ok__) => {
                    core::ptr::write(pval, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetFormatTag<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITScriptableAudioFormat_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, nnewval: i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ITScriptableAudioFormat_Impl::SetFormatTag(this, core::mem::transmute_copy(&nnewval)).into()
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            Channels: Channels::<Identity, Impl, OFFSET>,
            SetChannels: SetChannels::<Identity, Impl, OFFSET>,
            SamplesPerSec: SamplesPerSec::<Identity, Impl, OFFSET>,
            SetSamplesPerSec: SetSamplesPerSec::<Identity, Impl, OFFSET>,
            AvgBytesPerSec: AvgBytesPerSec::<Identity, Impl, OFFSET>,
            SetAvgBytesPerSec: SetAvgBytesPerSec::<Identity, Impl, OFFSET>,
            BlockAlign: BlockAlign::<Identity, Impl, OFFSET>,
            SetBlockAlign: SetBlockAlign::<Identity, Impl, OFFSET>,
            BitsPerSample: BitsPerSample::<Identity, Impl, OFFSET>,
            SetBitsPerSample: SetBitsPerSample::<Identity, Impl, OFFSET>,
            FormatTag: FormatTag::<Identity, Impl, OFFSET>,
            SetFormatTag: SetFormatTag::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITScriptableAudioFormat as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait ITStaticAudioTerminal_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn WaveId(&self) -> windows_core::Result<i32>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for ITStaticAudioTerminal {}
#[cfg(feature = "Win32_System_Com")]
impl ITStaticAudioTerminal_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITStaticAudioTerminal_Impl, const OFFSET: isize>() -> ITStaticAudioTerminal_Vtbl {
        unsafe extern "system" fn WaveId<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITStaticAudioTerminal_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, plwaveid: *mut i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ITStaticAudioTerminal_Impl::WaveId(this) {
                Ok(ok__) => {
                    core::ptr::write(plwaveid, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self { base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(), WaveId: WaveId::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITStaticAudioTerminal as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait ITStream_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn MediaType(&self) -> windows_core::Result<i32>;
    fn Direction(&self) -> windows_core::Result<TERMINAL_DIRECTION>;
    fn Name(&self) -> windows_core::Result<windows_core::BSTR>;
    fn StartStream(&self) -> windows_core::Result<()>;
    fn PauseStream(&self) -> windows_core::Result<()>;
    fn StopStream(&self) -> windows_core::Result<()>;
    fn SelectTerminal(&self, pterminal: Option<&ITTerminal>) -> windows_core::Result<()>;
    fn UnselectTerminal(&self, pterminal: Option<&ITTerminal>) -> windows_core::Result<()>;
    fn EnumerateTerminals(&self) -> windows_core::Result<IEnumTerminal>;
    fn Terminals(&self) -> windows_core::Result<windows_core::VARIANT>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for ITStream {}
#[cfg(feature = "Win32_System_Com")]
impl ITStream_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITStream_Impl, const OFFSET: isize>() -> ITStream_Vtbl {
        unsafe extern "system" fn MediaType<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITStream_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, plmediatype: *mut i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ITStream_Impl::MediaType(this) {
                Ok(ok__) => {
                    core::ptr::write(plmediatype, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Direction<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITStream_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ptd: *mut TERMINAL_DIRECTION) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ITStream_Impl::Direction(this) {
                Ok(ok__) => {
                    core::ptr::write(ptd, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Name<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITStream_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppname: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ITStream_Impl::Name(this) {
                Ok(ok__) => {
                    core::ptr::write(ppname, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn StartStream<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITStream_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ITStream_Impl::StartStream(this).into()
        }
        unsafe extern "system" fn PauseStream<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITStream_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ITStream_Impl::PauseStream(this).into()
        }
        unsafe extern "system" fn StopStream<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITStream_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ITStream_Impl::StopStream(this).into()
        }
        unsafe extern "system" fn SelectTerminal<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITStream_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pterminal: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ITStream_Impl::SelectTerminal(this, windows_core::from_raw_borrowed(&pterminal)).into()
        }
        unsafe extern "system" fn UnselectTerminal<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITStream_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pterminal: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ITStream_Impl::UnselectTerminal(this, windows_core::from_raw_borrowed(&pterminal)).into()
        }
        unsafe extern "system" fn EnumerateTerminals<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITStream_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppenumterminal: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ITStream_Impl::EnumerateTerminals(this) {
                Ok(ok__) => {
                    core::ptr::write(ppenumterminal, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Terminals<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITStream_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pterminals: *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ITStream_Impl::Terminals(this) {
                Ok(ok__) => {
                    core::ptr::write(pterminals, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            MediaType: MediaType::<Identity, Impl, OFFSET>,
            Direction: Direction::<Identity, Impl, OFFSET>,
            Name: Name::<Identity, Impl, OFFSET>,
            StartStream: StartStream::<Identity, Impl, OFFSET>,
            PauseStream: PauseStream::<Identity, Impl, OFFSET>,
            StopStream: StopStream::<Identity, Impl, OFFSET>,
            SelectTerminal: SelectTerminal::<Identity, Impl, OFFSET>,
            UnselectTerminal: UnselectTerminal::<Identity, Impl, OFFSET>,
            EnumerateTerminals: EnumerateTerminals::<Identity, Impl, OFFSET>,
            Terminals: Terminals::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITStream as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait ITStreamControl_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn CreateStream(&self, lmediatype: i32, td: TERMINAL_DIRECTION) -> windows_core::Result<ITStream>;
    fn RemoveStream(&self, pstream: Option<&ITStream>) -> windows_core::Result<()>;
    fn EnumerateStreams(&self) -> windows_core::Result<IEnumStream>;
    fn Streams(&self) -> windows_core::Result<windows_core::VARIANT>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for ITStreamControl {}
#[cfg(feature = "Win32_System_Com")]
impl ITStreamControl_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITStreamControl_Impl, const OFFSET: isize>() -> ITStreamControl_Vtbl {
        unsafe extern "system" fn CreateStream<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITStreamControl_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lmediatype: i32, td: TERMINAL_DIRECTION, ppstream: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ITStreamControl_Impl::CreateStream(this, core::mem::transmute_copy(&lmediatype), core::mem::transmute_copy(&td)) {
                Ok(ok__) => {
                    core::ptr::write(ppstream, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveStream<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITStreamControl_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pstream: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ITStreamControl_Impl::RemoveStream(this, windows_core::from_raw_borrowed(&pstream)).into()
        }
        unsafe extern "system" fn EnumerateStreams<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITStreamControl_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppenumstream: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ITStreamControl_Impl::EnumerateStreams(this) {
                Ok(ok__) => {
                    core::ptr::write(ppenumstream, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Streams<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITStreamControl_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvariant: *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ITStreamControl_Impl::Streams(this) {
                Ok(ok__) => {
                    core::ptr::write(pvariant, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            CreateStream: CreateStream::<Identity, Impl, OFFSET>,
            RemoveStream: RemoveStream::<Identity, Impl, OFFSET>,
            EnumerateStreams: EnumerateStreams::<Identity, Impl, OFFSET>,
            Streams: Streams::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITStreamControl as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait ITSubStream_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn StartSubStream(&self) -> windows_core::Result<()>;
    fn PauseSubStream(&self) -> windows_core::Result<()>;
    fn StopSubStream(&self) -> windows_core::Result<()>;
    fn SelectTerminal(&self, pterminal: Option<&ITTerminal>) -> windows_core::Result<()>;
    fn UnselectTerminal(&self, pterminal: Option<&ITTerminal>) -> windows_core::Result<()>;
    fn EnumerateTerminals(&self) -> windows_core::Result<IEnumTerminal>;
    fn Terminals(&self) -> windows_core::Result<windows_core::VARIANT>;
    fn Stream(&self) -> windows_core::Result<ITStream>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for ITSubStream {}
#[cfg(feature = "Win32_System_Com")]
impl ITSubStream_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITSubStream_Impl, const OFFSET: isize>() -> ITSubStream_Vtbl {
        unsafe extern "system" fn StartSubStream<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITSubStream_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ITSubStream_Impl::StartSubStream(this).into()
        }
        unsafe extern "system" fn PauseSubStream<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITSubStream_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ITSubStream_Impl::PauseSubStream(this).into()
        }
        unsafe extern "system" fn StopSubStream<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITSubStream_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ITSubStream_Impl::StopSubStream(this).into()
        }
        unsafe extern "system" fn SelectTerminal<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITSubStream_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pterminal: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ITSubStream_Impl::SelectTerminal(this, windows_core::from_raw_borrowed(&pterminal)).into()
        }
        unsafe extern "system" fn UnselectTerminal<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITSubStream_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pterminal: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ITSubStream_Impl::UnselectTerminal(this, windows_core::from_raw_borrowed(&pterminal)).into()
        }
        unsafe extern "system" fn EnumerateTerminals<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITSubStream_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppenumterminal: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ITSubStream_Impl::EnumerateTerminals(this) {
                Ok(ok__) => {
                    core::ptr::write(ppenumterminal, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Terminals<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITSubStream_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pterminals: *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ITSubStream_Impl::Terminals(this) {
                Ok(ok__) => {
                    core::ptr::write(pterminals, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Stream<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITSubStream_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppitstream: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ITSubStream_Impl::Stream(this) {
                Ok(ok__) => {
                    core::ptr::write(ppitstream, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            StartSubStream: StartSubStream::<Identity, Impl, OFFSET>,
            PauseSubStream: PauseSubStream::<Identity, Impl, OFFSET>,
            StopSubStream: StopSubStream::<Identity, Impl, OFFSET>,
            SelectTerminal: SelectTerminal::<Identity, Impl, OFFSET>,
            UnselectTerminal: UnselectTerminal::<Identity, Impl, OFFSET>,
            EnumerateTerminals: EnumerateTerminals::<Identity, Impl, OFFSET>,
            Terminals: Terminals::<Identity, Impl, OFFSET>,
            Stream: Stream::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITSubStream as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait ITSubStreamControl_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn CreateSubStream(&self) -> windows_core::Result<ITSubStream>;
    fn RemoveSubStream(&self, psubstream: Option<&ITSubStream>) -> windows_core::Result<()>;
    fn EnumerateSubStreams(&self) -> windows_core::Result<IEnumSubStream>;
    fn SubStreams(&self) -> windows_core::Result<windows_core::VARIANT>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for ITSubStreamControl {}
#[cfg(feature = "Win32_System_Com")]
impl ITSubStreamControl_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITSubStreamControl_Impl, const OFFSET: isize>() -> ITSubStreamControl_Vtbl {
        unsafe extern "system" fn CreateSubStream<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITSubStreamControl_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppsubstream: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ITSubStreamControl_Impl::CreateSubStream(this) {
                Ok(ok__) => {
                    core::ptr::write(ppsubstream, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveSubStream<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITSubStreamControl_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, psubstream: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ITSubStreamControl_Impl::RemoveSubStream(this, windows_core::from_raw_borrowed(&psubstream)).into()
        }
        unsafe extern "system" fn EnumerateSubStreams<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITSubStreamControl_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppenumsubstream: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ITSubStreamControl_Impl::EnumerateSubStreams(this) {
                Ok(ok__) => {
                    core::ptr::write(ppenumsubstream, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SubStreams<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITSubStreamControl_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvariant: *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ITSubStreamControl_Impl::SubStreams(this) {
                Ok(ok__) => {
                    core::ptr::write(pvariant, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            CreateSubStream: CreateSubStream::<Identity, Impl, OFFSET>,
            RemoveSubStream: RemoveSubStream::<Identity, Impl, OFFSET>,
            EnumerateSubStreams: EnumerateSubStreams::<Identity, Impl, OFFSET>,
            SubStreams: SubStreams::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITSubStreamControl as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait ITTAPI_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn Initialize(&self) -> windows_core::Result<()>;
    fn Shutdown(&self) -> windows_core::Result<()>;
    fn Addresses(&self) -> windows_core::Result<windows_core::VARIANT>;
    fn EnumerateAddresses(&self) -> windows_core::Result<IEnumAddress>;
    fn RegisterCallNotifications(&self, paddress: Option<&ITAddress>, fmonitor: super::super::Foundation::VARIANT_BOOL, fowner: super::super::Foundation::VARIANT_BOOL, lmediatypes: i32, lcallbackinstance: i32) -> windows_core::Result<i32>;
    fn UnregisterNotifications(&self, lregister: i32) -> windows_core::Result<()>;
    fn CallHubs(&self) -> windows_core::Result<windows_core::VARIANT>;
    fn EnumerateCallHubs(&self) -> windows_core::Result<IEnumCallHub>;
    fn SetCallHubTracking(&self, paddresses: &windows_core::VARIANT, btracking: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
    fn EnumeratePrivateTAPIObjects(&self) -> windows_core::Result<super::super::System::Com::IEnumUnknown>;
    fn PrivateTAPIObjects(&self) -> windows_core::Result<windows_core::VARIANT>;
    fn RegisterRequestRecipient(&self, lregistrationinstance: i32, lrequestmode: i32, fenable: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
    fn SetAssistedTelephonyPriority(&self, pappfilename: &windows_core::BSTR, fpriority: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
    fn SetApplicationPriority(&self, pappfilename: &windows_core::BSTR, lmediatype: i32, fpriority: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
    fn SetEventFilter(&self, lfiltermask: i32) -> windows_core::Result<()>;
    fn EventFilter(&self) -> windows_core::Result<i32>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for ITTAPI {}
#[cfg(feature = "Win32_System_Com")]
impl ITTAPI_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITTAPI_Impl, const OFFSET: isize>() -> ITTAPI_Vtbl {
        unsafe extern "system" fn Initialize<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITTAPI_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ITTAPI_Impl::Initialize(this).into()
        }
        unsafe extern "system" fn Shutdown<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITTAPI_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ITTAPI_Impl::Shutdown(this).into()
        }
        unsafe extern "system" fn Addresses<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITTAPI_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvariant: *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ITTAPI_Impl::Addresses(this) {
                Ok(ok__) => {
                    core::ptr::write(pvariant, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnumerateAddresses<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITTAPI_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppenumaddress: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ITTAPI_Impl::EnumerateAddresses(this) {
                Ok(ok__) => {
                    core::ptr::write(ppenumaddress, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RegisterCallNotifications<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITTAPI_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, paddress: *mut core::ffi::c_void, fmonitor: super::super::Foundation::VARIANT_BOOL, fowner: super::super::Foundation::VARIANT_BOOL, lmediatypes: i32, lcallbackinstance: i32, plregister: *mut i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ITTAPI_Impl::RegisterCallNotifications(this, windows_core::from_raw_borrowed(&paddress), core::mem::transmute_copy(&fmonitor), core::mem::transmute_copy(&fowner), core::mem::transmute_copy(&lmediatypes), core::mem::transmute_copy(&lcallbackinstance)) {
                Ok(ok__) => {
                    core::ptr::write(plregister, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UnregisterNotifications<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITTAPI_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lregister: i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ITTAPI_Impl::UnregisterNotifications(this, core::mem::transmute_copy(&lregister)).into()
        }
        unsafe extern "system" fn CallHubs<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITTAPI_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvariant: *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ITTAPI_Impl::CallHubs(this) {
                Ok(ok__) => {
                    core::ptr::write(pvariant, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnumerateCallHubs<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITTAPI_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppenumcallhub: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ITTAPI_Impl::EnumerateCallHubs(this) {
                Ok(ok__) => {
                    core::ptr::write(ppenumcallhub, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCallHubTracking<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITTAPI_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, paddresses: core::mem::MaybeUninit<windows_core::VARIANT>, btracking: super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ITTAPI_Impl::SetCallHubTracking(this, core::mem::transmute(&paddresses), core::mem::transmute_copy(&btracking)).into()
        }
        unsafe extern "system" fn EnumeratePrivateTAPIObjects<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITTAPI_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppenumunknown: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ITTAPI_Impl::EnumeratePrivateTAPIObjects(this) {
                Ok(ok__) => {
                    core::ptr::write(ppenumunknown, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PrivateTAPIObjects<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITTAPI_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvariant: *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ITTAPI_Impl::PrivateTAPIObjects(this) {
                Ok(ok__) => {
                    core::ptr::write(pvariant, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RegisterRequestRecipient<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITTAPI_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lregistrationinstance: i32, lrequestmode: i32, fenable: super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ITTAPI_Impl::RegisterRequestRecipient(this, core::mem::transmute_copy(&lregistrationinstance), core::mem::transmute_copy(&lrequestmode), core::mem::transmute_copy(&fenable)).into()
        }
        unsafe extern "system" fn SetAssistedTelephonyPriority<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITTAPI_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pappfilename: core::mem::MaybeUninit<windows_core::BSTR>, fpriority: super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ITTAPI_Impl::SetAssistedTelephonyPriority(this, core::mem::transmute(&pappfilename), core::mem::transmute_copy(&fpriority)).into()
        }
        unsafe extern "system" fn SetApplicationPriority<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITTAPI_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pappfilename: core::mem::MaybeUninit<windows_core::BSTR>, lmediatype: i32, fpriority: super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ITTAPI_Impl::SetApplicationPriority(this, core::mem::transmute(&pappfilename), core::mem::transmute_copy(&lmediatype), core::mem::transmute_copy(&fpriority)).into()
        }
        unsafe extern "system" fn SetEventFilter<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITTAPI_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lfiltermask: i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ITTAPI_Impl::SetEventFilter(this, core::mem::transmute_copy(&lfiltermask)).into()
        }
        unsafe extern "system" fn EventFilter<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITTAPI_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, plfiltermask: *mut i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ITTAPI_Impl::EventFilter(this) {
                Ok(ok__) => {
                    core::ptr::write(plfiltermask, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            Initialize: Initialize::<Identity, Impl, OFFSET>,
            Shutdown: Shutdown::<Identity, Impl, OFFSET>,
            Addresses: Addresses::<Identity, Impl, OFFSET>,
            EnumerateAddresses: EnumerateAddresses::<Identity, Impl, OFFSET>,
            RegisterCallNotifications: RegisterCallNotifications::<Identity, Impl, OFFSET>,
            UnregisterNotifications: UnregisterNotifications::<Identity, Impl, OFFSET>,
            CallHubs: CallHubs::<Identity, Impl, OFFSET>,
            EnumerateCallHubs: EnumerateCallHubs::<Identity, Impl, OFFSET>,
            SetCallHubTracking: SetCallHubTracking::<Identity, Impl, OFFSET>,
            EnumeratePrivateTAPIObjects: EnumeratePrivateTAPIObjects::<Identity, Impl, OFFSET>,
            PrivateTAPIObjects: PrivateTAPIObjects::<Identity, Impl, OFFSET>,
            RegisterRequestRecipient: RegisterRequestRecipient::<Identity, Impl, OFFSET>,
            SetAssistedTelephonyPriority: SetAssistedTelephonyPriority::<Identity, Impl, OFFSET>,
            SetApplicationPriority: SetApplicationPriority::<Identity, Impl, OFFSET>,
            SetEventFilter: SetEventFilter::<Identity, Impl, OFFSET>,
            EventFilter: EventFilter::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITTAPI as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait ITTAPI2_Impl: Sized + ITTAPI_Impl {
    fn Phones(&self) -> windows_core::Result<windows_core::VARIANT>;
    fn EnumeratePhones(&self) -> windows_core::Result<IEnumPhone>;
    fn CreateEmptyCollectionObject(&self) -> windows_core::Result<ITCollection2>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for ITTAPI2 {}
#[cfg(feature = "Win32_System_Com")]
impl ITTAPI2_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITTAPI2_Impl, const OFFSET: isize>() -> ITTAPI2_Vtbl {
        unsafe extern "system" fn Phones<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITTAPI2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pphones: *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ITTAPI2_Impl::Phones(this) {
                Ok(ok__) => {
                    core::ptr::write(pphones, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnumeratePhones<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITTAPI2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppenumphone: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ITTAPI2_Impl::EnumeratePhones(this) {
                Ok(ok__) => {
                    core::ptr::write(ppenumphone, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateEmptyCollectionObject<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITTAPI2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppcollection: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ITTAPI2_Impl::CreateEmptyCollectionObject(this) {
                Ok(ok__) => {
                    core::ptr::write(ppcollection, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: ITTAPI_Vtbl::new::<Identity, Impl, OFFSET>(),
            Phones: Phones::<Identity, Impl, OFFSET>,
            EnumeratePhones: EnumeratePhones::<Identity, Impl, OFFSET>,
            CreateEmptyCollectionObject: CreateEmptyCollectionObject::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITTAPI2 as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID || iid == &<ITTAPI as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait ITTAPICallCenter_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn EnumerateAgentHandlers(&self) -> windows_core::Result<IEnumAgentHandler>;
    fn AgentHandlers(&self) -> windows_core::Result<windows_core::VARIANT>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for ITTAPICallCenter {}
#[cfg(feature = "Win32_System_Com")]
impl ITTAPICallCenter_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITTAPICallCenter_Impl, const OFFSET: isize>() -> ITTAPICallCenter_Vtbl {
        unsafe extern "system" fn EnumerateAgentHandlers<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITTAPICallCenter_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppenumhandler: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ITTAPICallCenter_Impl::EnumerateAgentHandlers(this) {
                Ok(ok__) => {
                    core::ptr::write(ppenumhandler, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AgentHandlers<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITTAPICallCenter_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvariant: *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ITTAPICallCenter_Impl::AgentHandlers(this) {
                Ok(ok__) => {
                    core::ptr::write(pvariant, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            EnumerateAgentHandlers: EnumerateAgentHandlers::<Identity, Impl, OFFSET>,
            AgentHandlers: AgentHandlers::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITTAPICallCenter as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait ITTAPIDispatchEventNotification_Impl: Sized + super::super::System::Com::IDispatch_Impl {}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for ITTAPIDispatchEventNotification {}
#[cfg(feature = "Win32_System_Com")]
impl ITTAPIDispatchEventNotification_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITTAPIDispatchEventNotification_Impl, const OFFSET: isize>() -> ITTAPIDispatchEventNotification_Vtbl {
        Self { base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>() }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITTAPIDispatchEventNotification as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait ITTAPIEventNotification_Impl: Sized {
    fn Event(&self, tapievent: TAPI_EVENT, pevent: Option<&super::super::System::Com::IDispatch>) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for ITTAPIEventNotification {}
#[cfg(feature = "Win32_System_Com")]
impl ITTAPIEventNotification_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITTAPIEventNotification_Impl, const OFFSET: isize>() -> ITTAPIEventNotification_Vtbl {
        unsafe extern "system" fn Event<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITTAPIEventNotification_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, tapievent: TAPI_EVENT, pevent: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ITTAPIEventNotification_Impl::Event(this, core::mem::transmute_copy(&tapievent), windows_core::from_raw_borrowed(&pevent)).into()
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), Event: Event::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITTAPIEventNotification as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait ITTAPIObjectEvent_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn TAPIObject(&self) -> windows_core::Result<ITTAPI>;
    fn Event(&self) -> windows_core::Result<TAPIOBJECT_EVENT>;
    fn Address(&self) -> windows_core::Result<ITAddress>;
    fn CallbackInstance(&self) -> windows_core::Result<i32>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for ITTAPIObjectEvent {}
#[cfg(feature = "Win32_System_Com")]
impl ITTAPIObjectEvent_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITTAPIObjectEvent_Impl, const OFFSET: isize>() -> ITTAPIObjectEvent_Vtbl {
        unsafe extern "system" fn TAPIObject<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITTAPIObjectEvent_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pptapiobject: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ITTAPIObjectEvent_Impl::TAPIObject(this) {
                Ok(ok__) => {
                    core::ptr::write(pptapiobject, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Event<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITTAPIObjectEvent_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pevent: *mut TAPIOBJECT_EVENT) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ITTAPIObjectEvent_Impl::Event(this) {
                Ok(ok__) => {
                    core::ptr::write(pevent, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Address<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITTAPIObjectEvent_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppaddress: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ITTAPIObjectEvent_Impl::Address(this) {
                Ok(ok__) => {
                    core::ptr::write(ppaddress, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CallbackInstance<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITTAPIObjectEvent_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, plcallbackinstance: *mut i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ITTAPIObjectEvent_Impl::CallbackInstance(this) {
                Ok(ok__) => {
                    core::ptr::write(plcallbackinstance, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            TAPIObject: TAPIObject::<Identity, Impl, OFFSET>,
            Event: Event::<Identity, Impl, OFFSET>,
            Address: Address::<Identity, Impl, OFFSET>,
            CallbackInstance: CallbackInstance::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITTAPIObjectEvent as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait ITTAPIObjectEvent2_Impl: Sized + ITTAPIObjectEvent_Impl {
    fn Phone(&self) -> windows_core::Result<ITPhone>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for ITTAPIObjectEvent2 {}
#[cfg(feature = "Win32_System_Com")]
impl ITTAPIObjectEvent2_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITTAPIObjectEvent2_Impl, const OFFSET: isize>() -> ITTAPIObjectEvent2_Vtbl {
        unsafe extern "system" fn Phone<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITTAPIObjectEvent2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppphone: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ITTAPIObjectEvent2_Impl::Phone(this) {
                Ok(ok__) => {
                    core::ptr::write(ppphone, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self { base__: ITTAPIObjectEvent_Vtbl::new::<Identity, Impl, OFFSET>(), Phone: Phone::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITTAPIObjectEvent2 as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID || iid == &<ITTAPIObjectEvent as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait ITTTSTerminalEvent_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn Terminal(&self) -> windows_core::Result<ITTerminal>;
    fn Call(&self) -> windows_core::Result<ITCallInfo>;
    fn Error(&self) -> windows_core::Result<windows_core::HRESULT>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for ITTTSTerminalEvent {}
#[cfg(feature = "Win32_System_Com")]
impl ITTTSTerminalEvent_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITTTSTerminalEvent_Impl, const OFFSET: isize>() -> ITTTSTerminalEvent_Vtbl {
        unsafe extern "system" fn Terminal<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITTTSTerminalEvent_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppterminal: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ITTTSTerminalEvent_Impl::Terminal(this) {
                Ok(ok__) => {
                    core::ptr::write(ppterminal, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Call<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITTTSTerminalEvent_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppcall: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ITTTSTerminalEvent_Impl::Call(this) {
                Ok(ok__) => {
                    core::ptr::write(ppcall, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Error<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITTTSTerminalEvent_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, phrerrorcode: *mut windows_core::HRESULT) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ITTTSTerminalEvent_Impl::Error(this) {
                Ok(ok__) => {
                    core::ptr::write(phrerrorcode, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            Terminal: Terminal::<Identity, Impl, OFFSET>,
            Call: Call::<Identity, Impl, OFFSET>,
            Error: Error::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITTTSTerminalEvent as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait ITTerminal_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn Name(&self) -> windows_core::Result<windows_core::BSTR>;
    fn State(&self) -> windows_core::Result<TERMINAL_STATE>;
    fn TerminalType(&self) -> windows_core::Result<TERMINAL_TYPE>;
    fn TerminalClass(&self) -> windows_core::Result<windows_core::BSTR>;
    fn MediaType(&self) -> windows_core::Result<i32>;
    fn Direction(&self) -> windows_core::Result<TERMINAL_DIRECTION>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for ITTerminal {}
#[cfg(feature = "Win32_System_Com")]
impl ITTerminal_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITTerminal_Impl, const OFFSET: isize>() -> ITTerminal_Vtbl {
        unsafe extern "system" fn Name<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITTerminal_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppname: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ITTerminal_Impl::Name(this) {
                Ok(ok__) => {
                    core::ptr::write(ppname, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn State<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITTerminal_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pterminalstate: *mut TERMINAL_STATE) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ITTerminal_Impl::State(this) {
                Ok(ok__) => {
                    core::ptr::write(pterminalstate, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TerminalType<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITTerminal_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ptype: *mut TERMINAL_TYPE) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ITTerminal_Impl::TerminalType(this) {
                Ok(ok__) => {
                    core::ptr::write(ptype, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TerminalClass<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITTerminal_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppterminalclass: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ITTerminal_Impl::TerminalClass(this) {
                Ok(ok__) => {
                    core::ptr::write(ppterminalclass, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MediaType<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITTerminal_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, plmediatype: *mut i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ITTerminal_Impl::MediaType(this) {
                Ok(ok__) => {
                    core::ptr::write(plmediatype, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Direction<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITTerminal_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdirection: *mut TERMINAL_DIRECTION) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ITTerminal_Impl::Direction(this) {
                Ok(ok__) => {
                    core::ptr::write(pdirection, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            Name: Name::<Identity, Impl, OFFSET>,
            State: State::<Identity, Impl, OFFSET>,
            TerminalType: TerminalType::<Identity, Impl, OFFSET>,
            TerminalClass: TerminalClass::<Identity, Impl, OFFSET>,
            MediaType: MediaType::<Identity, Impl, OFFSET>,
            Direction: Direction::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITTerminal as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait ITTerminalSupport_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn StaticTerminals(&self) -> windows_core::Result<windows_core::VARIANT>;
    fn EnumerateStaticTerminals(&self) -> windows_core::Result<IEnumTerminal>;
    fn DynamicTerminalClasses(&self) -> windows_core::Result<windows_core::VARIANT>;
    fn EnumerateDynamicTerminalClasses(&self) -> windows_core::Result<IEnumTerminalClass>;
    fn CreateTerminal(&self, pterminalclass: &windows_core::BSTR, lmediatype: i32, direction: TERMINAL_DIRECTION) -> windows_core::Result<ITTerminal>;
    fn GetDefaultStaticTerminal(&self, lmediatype: i32, direction: TERMINAL_DIRECTION) -> windows_core::Result<ITTerminal>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for ITTerminalSupport {}
#[cfg(feature = "Win32_System_Com")]
impl ITTerminalSupport_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITTerminalSupport_Impl, const OFFSET: isize>() -> ITTerminalSupport_Vtbl {
        unsafe extern "system" fn StaticTerminals<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITTerminalSupport_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvariant: *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ITTerminalSupport_Impl::StaticTerminals(this) {
                Ok(ok__) => {
                    core::ptr::write(pvariant, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnumerateStaticTerminals<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITTerminalSupport_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppterminalenumerator: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ITTerminalSupport_Impl::EnumerateStaticTerminals(this) {
                Ok(ok__) => {
                    core::ptr::write(ppterminalenumerator, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DynamicTerminalClasses<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITTerminalSupport_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvariant: *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ITTerminalSupport_Impl::DynamicTerminalClasses(this) {
                Ok(ok__) => {
                    core::ptr::write(pvariant, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnumerateDynamicTerminalClasses<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITTerminalSupport_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppterminalclassenumerator: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ITTerminalSupport_Impl::EnumerateDynamicTerminalClasses(this) {
                Ok(ok__) => {
                    core::ptr::write(ppterminalclassenumerator, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateTerminal<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITTerminalSupport_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pterminalclass: core::mem::MaybeUninit<windows_core::BSTR>, lmediatype: i32, direction: TERMINAL_DIRECTION, ppterminal: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ITTerminalSupport_Impl::CreateTerminal(this, core::mem::transmute(&pterminalclass), core::mem::transmute_copy(&lmediatype), core::mem::transmute_copy(&direction)) {
                Ok(ok__) => {
                    core::ptr::write(ppterminal, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDefaultStaticTerminal<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITTerminalSupport_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lmediatype: i32, direction: TERMINAL_DIRECTION, ppterminal: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ITTerminalSupport_Impl::GetDefaultStaticTerminal(this, core::mem::transmute_copy(&lmediatype), core::mem::transmute_copy(&direction)) {
                Ok(ok__) => {
                    core::ptr::write(ppterminal, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            StaticTerminals: StaticTerminals::<Identity, Impl, OFFSET>,
            EnumerateStaticTerminals: EnumerateStaticTerminals::<Identity, Impl, OFFSET>,
            DynamicTerminalClasses: DynamicTerminalClasses::<Identity, Impl, OFFSET>,
            EnumerateDynamicTerminalClasses: EnumerateDynamicTerminalClasses::<Identity, Impl, OFFSET>,
            CreateTerminal: CreateTerminal::<Identity, Impl, OFFSET>,
            GetDefaultStaticTerminal: GetDefaultStaticTerminal::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITTerminalSupport as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait ITTerminalSupport2_Impl: Sized + ITTerminalSupport_Impl {
    fn PluggableSuperclasses(&self) -> windows_core::Result<windows_core::VARIANT>;
    fn EnumeratePluggableSuperclasses(&self) -> windows_core::Result<IEnumPluggableSuperclassInfo>;
    fn get_PluggableTerminalClasses(&self, bstrterminalsuperclass: &windows_core::BSTR, lmediatype: i32) -> windows_core::Result<windows_core::VARIANT>;
    fn EnumeratePluggableTerminalClasses(&self, iidterminalsuperclass: &windows_core::GUID, lmediatype: i32) -> windows_core::Result<IEnumPluggableTerminalClassInfo>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for ITTerminalSupport2 {}
#[cfg(feature = "Win32_System_Com")]
impl ITTerminalSupport2_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITTerminalSupport2_Impl, const OFFSET: isize>() -> ITTerminalSupport2_Vtbl {
        unsafe extern "system" fn PluggableSuperclasses<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITTerminalSupport2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvariant: *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ITTerminalSupport2_Impl::PluggableSuperclasses(this) {
                Ok(ok__) => {
                    core::ptr::write(pvariant, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnumeratePluggableSuperclasses<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITTerminalSupport2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppsuperclassenumerator: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ITTerminalSupport2_Impl::EnumeratePluggableSuperclasses(this) {
                Ok(ok__) => {
                    core::ptr::write(ppsuperclassenumerator, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn get_PluggableTerminalClasses<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITTerminalSupport2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrterminalsuperclass: core::mem::MaybeUninit<windows_core::BSTR>, lmediatype: i32, pvariant: *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ITTerminalSupport2_Impl::get_PluggableTerminalClasses(this, core::mem::transmute(&bstrterminalsuperclass), core::mem::transmute_copy(&lmediatype)) {
                Ok(ok__) => {
                    core::ptr::write(pvariant, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnumeratePluggableTerminalClasses<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITTerminalSupport2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, iidterminalsuperclass: windows_core::GUID, lmediatype: i32, ppclassenumerator: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ITTerminalSupport2_Impl::EnumeratePluggableTerminalClasses(this, core::mem::transmute(&iidterminalsuperclass), core::mem::transmute_copy(&lmediatype)) {
                Ok(ok__) => {
                    core::ptr::write(ppclassenumerator, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: ITTerminalSupport_Vtbl::new::<Identity, Impl, OFFSET>(),
            PluggableSuperclasses: PluggableSuperclasses::<Identity, Impl, OFFSET>,
            EnumeratePluggableSuperclasses: EnumeratePluggableSuperclasses::<Identity, Impl, OFFSET>,
            get_PluggableTerminalClasses: get_PluggableTerminalClasses::<Identity, Impl, OFFSET>,
            EnumeratePluggableTerminalClasses: EnumeratePluggableTerminalClasses::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITTerminalSupport2 as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID || iid == &<ITTerminalSupport as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait ITToneDetectionEvent_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn Call(&self) -> windows_core::Result<ITCallInfo>;
    fn AppSpecific(&self) -> windows_core::Result<i32>;
    fn TickCount(&self) -> windows_core::Result<i32>;
    fn CallbackInstance(&self) -> windows_core::Result<i32>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for ITToneDetectionEvent {}
#[cfg(feature = "Win32_System_Com")]
impl ITToneDetectionEvent_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITToneDetectionEvent_Impl, const OFFSET: isize>() -> ITToneDetectionEvent_Vtbl {
        unsafe extern "system" fn Call<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITToneDetectionEvent_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppcallinfo: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ITToneDetectionEvent_Impl::Call(this) {
                Ok(ok__) => {
                    core::ptr::write(ppcallinfo, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AppSpecific<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITToneDetectionEvent_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, plappspecific: *mut i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ITToneDetectionEvent_Impl::AppSpecific(this) {
                Ok(ok__) => {
                    core::ptr::write(plappspecific, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TickCount<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITToneDetectionEvent_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pltickcount: *mut i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ITToneDetectionEvent_Impl::TickCount(this) {
                Ok(ok__) => {
                    core::ptr::write(pltickcount, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CallbackInstance<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITToneDetectionEvent_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, plcallbackinstance: *mut i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ITToneDetectionEvent_Impl::CallbackInstance(this) {
                Ok(ok__) => {
                    core::ptr::write(plcallbackinstance, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            Call: Call::<Identity, Impl, OFFSET>,
            AppSpecific: AppSpecific::<Identity, Impl, OFFSET>,
            TickCount: TickCount::<Identity, Impl, OFFSET>,
            CallbackInstance: CallbackInstance::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITToneDetectionEvent as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait ITToneTerminalEvent_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn Terminal(&self) -> windows_core::Result<ITTerminal>;
    fn Call(&self) -> windows_core::Result<ITCallInfo>;
    fn Error(&self) -> windows_core::Result<windows_core::HRESULT>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for ITToneTerminalEvent {}
#[cfg(feature = "Win32_System_Com")]
impl ITToneTerminalEvent_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITToneTerminalEvent_Impl, const OFFSET: isize>() -> ITToneTerminalEvent_Vtbl {
        unsafe extern "system" fn Terminal<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITToneTerminalEvent_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppterminal: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ITToneTerminalEvent_Impl::Terminal(this) {
                Ok(ok__) => {
                    core::ptr::write(ppterminal, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Call<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITToneTerminalEvent_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppcall: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ITToneTerminalEvent_Impl::Call(this) {
                Ok(ok__) => {
                    core::ptr::write(ppcall, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Error<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITToneTerminalEvent_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, phrerrorcode: *mut windows_core::HRESULT) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ITToneTerminalEvent_Impl::Error(this) {
                Ok(ok__) => {
                    core::ptr::write(phrerrorcode, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            Terminal: Terminal::<Identity, Impl, OFFSET>,
            Call: Call::<Identity, Impl, OFFSET>,
            Error: Error::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITToneTerminalEvent as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_AddressBook", feature = "Win32_System_Com"))]
pub trait ITnef_Impl: Sized {
    fn AddProps(&self, ulflags: u32, ulelemid: u32, lpvdata: *mut core::ffi::c_void, lpproplist: *mut super::super::System::AddressBook::SPropTagArray) -> windows_core::Result<()>;
    fn ExtractProps(&self, ulflags: u32, lpproplist: *mut super::super::System::AddressBook::SPropTagArray, lpproblems: *mut *mut STnefProblemArray) -> windows_core::Result<()>;
    fn Finish(&self, ulflags: u32, lpkey: *mut u16, lpproblems: *mut *mut STnefProblemArray) -> windows_core::Result<()>;
    fn OpenTaggedBody(&self, lpmessage: Option<&super::super::System::AddressBook::IMessage>, ulflags: u32) -> windows_core::Result<super::super::System::Com::IStream>;
    fn SetProps(&self, ulflags: u32, ulelemid: u32, cvalues: u32, lpprops: *mut super::super::System::AddressBook::SPropValue) -> windows_core::Result<()>;
    fn EncodeRecips(&self, ulflags: u32, lprecipienttable: Option<&super::super::System::AddressBook::IMAPITable>) -> windows_core::Result<()>;
    fn FinishComponent(&self, ulflags: u32, ulcomponentid: u32, lpcustomproplist: *mut super::super::System::AddressBook::SPropTagArray, lpcustomprops: *mut super::super::System::AddressBook::SPropValue, lpproplist: *mut super::super::System::AddressBook::SPropTagArray, lpproblems: *mut *mut STnefProblemArray) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_System_AddressBook", feature = "Win32_System_Com"))]
impl windows_core::RuntimeName for ITnef {}
#[cfg(all(feature = "Win32_System_AddressBook", feature = "Win32_System_Com"))]
impl ITnef_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITnef_Impl, const OFFSET: isize>() -> ITnef_Vtbl {
        unsafe extern "system" fn AddProps<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITnef_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ulflags: u32, ulelemid: u32, lpvdata: *mut core::ffi::c_void, lpproplist: *mut super::super::System::AddressBook::SPropTagArray) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ITnef_Impl::AddProps(this, core::mem::transmute_copy(&ulflags), core::mem::transmute_copy(&ulelemid), core::mem::transmute_copy(&lpvdata), core::mem::transmute_copy(&lpproplist)).into()
        }
        unsafe extern "system" fn ExtractProps<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITnef_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ulflags: u32, lpproplist: *mut super::super::System::AddressBook::SPropTagArray, lpproblems: *mut *mut STnefProblemArray) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ITnef_Impl::ExtractProps(this, core::mem::transmute_copy(&ulflags), core::mem::transmute_copy(&lpproplist), core::mem::transmute_copy(&lpproblems)).into()
        }
        unsafe extern "system" fn Finish<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITnef_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ulflags: u32, lpkey: *mut u16, lpproblems: *mut *mut STnefProblemArray) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ITnef_Impl::Finish(this, core::mem::transmute_copy(&ulflags), core::mem::transmute_copy(&lpkey), core::mem::transmute_copy(&lpproblems)).into()
        }
        unsafe extern "system" fn OpenTaggedBody<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITnef_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lpmessage: *mut core::ffi::c_void, ulflags: u32, lppstream: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ITnef_Impl::OpenTaggedBody(this, windows_core::from_raw_borrowed(&lpmessage), core::mem::transmute_copy(&ulflags)) {
                Ok(ok__) => {
                    core::ptr::write(lppstream, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetProps<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITnef_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ulflags: u32, ulelemid: u32, cvalues: u32, lpprops: *mut super::super::System::AddressBook::SPropValue) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ITnef_Impl::SetProps(this, core::mem::transmute_copy(&ulflags), core::mem::transmute_copy(&ulelemid), core::mem::transmute_copy(&cvalues), core::mem::transmute_copy(&lpprops)).into()
        }
        unsafe extern "system" fn EncodeRecips<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITnef_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ulflags: u32, lprecipienttable: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ITnef_Impl::EncodeRecips(this, core::mem::transmute_copy(&ulflags), windows_core::from_raw_borrowed(&lprecipienttable)).into()
        }
        unsafe extern "system" fn FinishComponent<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITnef_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ulflags: u32, ulcomponentid: u32, lpcustomproplist: *mut super::super::System::AddressBook::SPropTagArray, lpcustomprops: *mut super::super::System::AddressBook::SPropValue, lpproplist: *mut super::super::System::AddressBook::SPropTagArray, lpproblems: *mut *mut STnefProblemArray) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ITnef_Impl::FinishComponent(this, core::mem::transmute_copy(&ulflags), core::mem::transmute_copy(&ulcomponentid), core::mem::transmute_copy(&lpcustomproplist), core::mem::transmute_copy(&lpcustomprops), core::mem::transmute_copy(&lpproplist), core::mem::transmute_copy(&lpproblems)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            AddProps: AddProps::<Identity, Impl, OFFSET>,
            ExtractProps: ExtractProps::<Identity, Impl, OFFSET>,
            Finish: Finish::<Identity, Impl, OFFSET>,
            OpenTaggedBody: OpenTaggedBody::<Identity, Impl, OFFSET>,
            SetProps: SetProps::<Identity, Impl, OFFSET>,
            EncodeRecips: EncodeRecips::<Identity, Impl, OFFSET>,
            FinishComponent: FinishComponent::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITnef as windows_core::Interface>::IID
    }
}
