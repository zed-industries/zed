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
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IEnumACDGroup_Vtbl
    where
        Identity: IEnumACDGroup_Impl,
    {
        unsafe extern "system" fn Next<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, celt: u32, ppelements: *mut *mut core::ffi::c_void, pceltfetched: *mut u32) -> windows_core::HRESULT
        where
            Identity: IEnumACDGroup_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEnumACDGroup_Impl::Next(this, core::mem::transmute_copy(&celt), core::mem::transmute_copy(&ppelements), core::mem::transmute_copy(&pceltfetched)).into()
        }
        unsafe extern "system" fn Reset<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IEnumACDGroup_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEnumACDGroup_Impl::Reset(this).into()
        }
        unsafe extern "system" fn Skip<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, celt: u32) -> windows_core::HRESULT
        where
            Identity: IEnumACDGroup_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEnumACDGroup_Impl::Skip(this, core::mem::transmute_copy(&celt)).into()
        }
        unsafe extern "system" fn Clone<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppenum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IEnumACDGroup_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IEnumACDGroup_Impl::Clone(this) {
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
            Reset: Reset::<Identity, OFFSET>,
            Skip: Skip::<Identity, OFFSET>,
            Clone: Clone::<Identity, OFFSET>,
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
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IEnumAddress_Vtbl
    where
        Identity: IEnumAddress_Impl,
    {
        unsafe extern "system" fn Next<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, celt: u32, ppelements: *mut *mut core::ffi::c_void, pceltfetched: *mut u32) -> windows_core::HRESULT
        where
            Identity: IEnumAddress_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEnumAddress_Impl::Next(this, core::mem::transmute_copy(&celt), core::mem::transmute_copy(&ppelements), core::mem::transmute_copy(&pceltfetched)).into()
        }
        unsafe extern "system" fn Reset<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IEnumAddress_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEnumAddress_Impl::Reset(this).into()
        }
        unsafe extern "system" fn Skip<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, celt: u32) -> windows_core::HRESULT
        where
            Identity: IEnumAddress_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEnumAddress_Impl::Skip(this, core::mem::transmute_copy(&celt)).into()
        }
        unsafe extern "system" fn Clone<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppenum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IEnumAddress_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IEnumAddress_Impl::Clone(this) {
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
            Reset: Reset::<Identity, OFFSET>,
            Skip: Skip::<Identity, OFFSET>,
            Clone: Clone::<Identity, OFFSET>,
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
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IEnumAgent_Vtbl
    where
        Identity: IEnumAgent_Impl,
    {
        unsafe extern "system" fn Next<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, celt: u32, ppelements: *mut *mut core::ffi::c_void, pceltfetched: *mut u32) -> windows_core::HRESULT
        where
            Identity: IEnumAgent_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEnumAgent_Impl::Next(this, core::mem::transmute_copy(&celt), core::mem::transmute_copy(&ppelements), core::mem::transmute_copy(&pceltfetched)).into()
        }
        unsafe extern "system" fn Reset<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IEnumAgent_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEnumAgent_Impl::Reset(this).into()
        }
        unsafe extern "system" fn Skip<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, celt: u32) -> windows_core::HRESULT
        where
            Identity: IEnumAgent_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEnumAgent_Impl::Skip(this, core::mem::transmute_copy(&celt)).into()
        }
        unsafe extern "system" fn Clone<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppenum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IEnumAgent_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IEnumAgent_Impl::Clone(this) {
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
            Reset: Reset::<Identity, OFFSET>,
            Skip: Skip::<Identity, OFFSET>,
            Clone: Clone::<Identity, OFFSET>,
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
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IEnumAgentHandler_Vtbl
    where
        Identity: IEnumAgentHandler_Impl,
    {
        unsafe extern "system" fn Next<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, celt: u32, ppelements: *mut *mut core::ffi::c_void, pceltfetched: *mut u32) -> windows_core::HRESULT
        where
            Identity: IEnumAgentHandler_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEnumAgentHandler_Impl::Next(this, core::mem::transmute_copy(&celt), core::mem::transmute_copy(&ppelements), core::mem::transmute_copy(&pceltfetched)).into()
        }
        unsafe extern "system" fn Reset<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IEnumAgentHandler_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEnumAgentHandler_Impl::Reset(this).into()
        }
        unsafe extern "system" fn Skip<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, celt: u32) -> windows_core::HRESULT
        where
            Identity: IEnumAgentHandler_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEnumAgentHandler_Impl::Skip(this, core::mem::transmute_copy(&celt)).into()
        }
        unsafe extern "system" fn Clone<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppenum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IEnumAgentHandler_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IEnumAgentHandler_Impl::Clone(this) {
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
            Reset: Reset::<Identity, OFFSET>,
            Skip: Skip::<Identity, OFFSET>,
            Clone: Clone::<Identity, OFFSET>,
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
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IEnumAgentSession_Vtbl
    where
        Identity: IEnumAgentSession_Impl,
    {
        unsafe extern "system" fn Next<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, celt: u32, ppelements: *mut *mut core::ffi::c_void, pceltfetched: *mut u32) -> windows_core::HRESULT
        where
            Identity: IEnumAgentSession_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEnumAgentSession_Impl::Next(this, core::mem::transmute_copy(&celt), core::mem::transmute_copy(&ppelements), core::mem::transmute_copy(&pceltfetched)).into()
        }
        unsafe extern "system" fn Reset<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IEnumAgentSession_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEnumAgentSession_Impl::Reset(this).into()
        }
        unsafe extern "system" fn Skip<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, celt: u32) -> windows_core::HRESULT
        where
            Identity: IEnumAgentSession_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEnumAgentSession_Impl::Skip(this, core::mem::transmute_copy(&celt)).into()
        }
        unsafe extern "system" fn Clone<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppenum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IEnumAgentSession_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IEnumAgentSession_Impl::Clone(this) {
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
            Reset: Reset::<Identity, OFFSET>,
            Skip: Skip::<Identity, OFFSET>,
            Clone: Clone::<Identity, OFFSET>,
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
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IEnumBstr_Vtbl
    where
        Identity: IEnumBstr_Impl,
    {
        unsafe extern "system" fn Next<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, celt: u32, ppstrings: *mut core::mem::MaybeUninit<windows_core::BSTR>, pceltfetched: *mut u32) -> windows_core::HRESULT
        where
            Identity: IEnumBstr_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEnumBstr_Impl::Next(this, core::mem::transmute_copy(&celt), core::mem::transmute_copy(&ppstrings), core::mem::transmute_copy(&pceltfetched)).into()
        }
        unsafe extern "system" fn Reset<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IEnumBstr_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEnumBstr_Impl::Reset(this).into()
        }
        unsafe extern "system" fn Skip<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, celt: u32) -> windows_core::HRESULT
        where
            Identity: IEnumBstr_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEnumBstr_Impl::Skip(this, core::mem::transmute_copy(&celt)).into()
        }
        unsafe extern "system" fn Clone<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppenum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IEnumBstr_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IEnumBstr_Impl::Clone(this) {
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
            Reset: Reset::<Identity, OFFSET>,
            Skip: Skip::<Identity, OFFSET>,
            Clone: Clone::<Identity, OFFSET>,
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
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IEnumCall_Vtbl
    where
        Identity: IEnumCall_Impl,
    {
        unsafe extern "system" fn Next<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, celt: u32, ppelements: *mut *mut core::ffi::c_void, pceltfetched: *mut u32) -> windows_core::HRESULT
        where
            Identity: IEnumCall_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEnumCall_Impl::Next(this, core::mem::transmute_copy(&celt), core::mem::transmute_copy(&ppelements), core::mem::transmute_copy(&pceltfetched)).into()
        }
        unsafe extern "system" fn Reset<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IEnumCall_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEnumCall_Impl::Reset(this).into()
        }
        unsafe extern "system" fn Skip<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, celt: u32) -> windows_core::HRESULT
        where
            Identity: IEnumCall_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEnumCall_Impl::Skip(this, core::mem::transmute_copy(&celt)).into()
        }
        unsafe extern "system" fn Clone<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppenum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IEnumCall_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IEnumCall_Impl::Clone(this) {
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
            Reset: Reset::<Identity, OFFSET>,
            Skip: Skip::<Identity, OFFSET>,
            Clone: Clone::<Identity, OFFSET>,
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
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IEnumCallHub_Vtbl
    where
        Identity: IEnumCallHub_Impl,
    {
        unsafe extern "system" fn Next<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, celt: u32, ppelements: *mut *mut core::ffi::c_void, pceltfetched: *mut u32) -> windows_core::HRESULT
        where
            Identity: IEnumCallHub_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEnumCallHub_Impl::Next(this, core::mem::transmute_copy(&celt), core::mem::transmute_copy(&ppelements), core::mem::transmute_copy(&pceltfetched)).into()
        }
        unsafe extern "system" fn Reset<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IEnumCallHub_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEnumCallHub_Impl::Reset(this).into()
        }
        unsafe extern "system" fn Skip<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, celt: u32) -> windows_core::HRESULT
        where
            Identity: IEnumCallHub_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEnumCallHub_Impl::Skip(this, core::mem::transmute_copy(&celt)).into()
        }
        unsafe extern "system" fn Clone<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppenum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IEnumCallHub_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IEnumCallHub_Impl::Clone(this) {
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
            Reset: Reset::<Identity, OFFSET>,
            Skip: Skip::<Identity, OFFSET>,
            Clone: Clone::<Identity, OFFSET>,
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
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IEnumCallingCard_Vtbl
    where
        Identity: IEnumCallingCard_Impl,
    {
        unsafe extern "system" fn Next<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, celt: u32, ppelements: *mut *mut core::ffi::c_void, pceltfetched: *mut u32) -> windows_core::HRESULT
        where
            Identity: IEnumCallingCard_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEnumCallingCard_Impl::Next(this, core::mem::transmute_copy(&celt), core::mem::transmute_copy(&ppelements), core::mem::transmute_copy(&pceltfetched)).into()
        }
        unsafe extern "system" fn Reset<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IEnumCallingCard_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEnumCallingCard_Impl::Reset(this).into()
        }
        unsafe extern "system" fn Skip<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, celt: u32) -> windows_core::HRESULT
        where
            Identity: IEnumCallingCard_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEnumCallingCard_Impl::Skip(this, core::mem::transmute_copy(&celt)).into()
        }
        unsafe extern "system" fn Clone<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppenum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IEnumCallingCard_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IEnumCallingCard_Impl::Clone(this) {
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
            Reset: Reset::<Identity, OFFSET>,
            Skip: Skip::<Identity, OFFSET>,
            Clone: Clone::<Identity, OFFSET>,
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
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IEnumDialableAddrs_Vtbl
    where
        Identity: IEnumDialableAddrs_Impl,
    {
        unsafe extern "system" fn Next<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, celt: u32, ppelements: *mut core::mem::MaybeUninit<windows_core::BSTR>, pcfetched: *mut u32) -> windows_core::HRESULT
        where
            Identity: IEnumDialableAddrs_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEnumDialableAddrs_Impl::Next(this, core::mem::transmute_copy(&celt), core::mem::transmute_copy(&ppelements), core::mem::transmute_copy(&pcfetched)).into()
        }
        unsafe extern "system" fn Reset<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IEnumDialableAddrs_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEnumDialableAddrs_Impl::Reset(this).into()
        }
        unsafe extern "system" fn Skip<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, celt: u32) -> windows_core::HRESULT
        where
            Identity: IEnumDialableAddrs_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEnumDialableAddrs_Impl::Skip(this, core::mem::transmute_copy(&celt)).into()
        }
        unsafe extern "system" fn Clone<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppenum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IEnumDialableAddrs_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IEnumDialableAddrs_Impl::Clone(this) {
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
            Reset: Reset::<Identity, OFFSET>,
            Skip: Skip::<Identity, OFFSET>,
            Clone: Clone::<Identity, OFFSET>,
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
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IEnumDirectory_Vtbl
    where
        Identity: IEnumDirectory_Impl,
    {
        unsafe extern "system" fn Next<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, celt: u32, ppelements: *mut *mut core::ffi::c_void, pcfetched: *mut u32) -> windows_core::HRESULT
        where
            Identity: IEnumDirectory_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEnumDirectory_Impl::Next(this, core::mem::transmute_copy(&celt), core::mem::transmute_copy(&ppelements), core::mem::transmute_copy(&pcfetched)).into()
        }
        unsafe extern "system" fn Reset<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IEnumDirectory_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEnumDirectory_Impl::Reset(this).into()
        }
        unsafe extern "system" fn Skip<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, celt: u32) -> windows_core::HRESULT
        where
            Identity: IEnumDirectory_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEnumDirectory_Impl::Skip(this, core::mem::transmute_copy(&celt)).into()
        }
        unsafe extern "system" fn Clone<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppenum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IEnumDirectory_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IEnumDirectory_Impl::Clone(this) {
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
            Reset: Reset::<Identity, OFFSET>,
            Skip: Skip::<Identity, OFFSET>,
            Clone: Clone::<Identity, OFFSET>,
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
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IEnumDirectoryObject_Vtbl
    where
        Identity: IEnumDirectoryObject_Impl,
    {
        unsafe extern "system" fn Next<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, celt: u32, pval: *mut *mut core::ffi::c_void, pcfetched: *mut u32) -> windows_core::HRESULT
        where
            Identity: IEnumDirectoryObject_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEnumDirectoryObject_Impl::Next(this, core::mem::transmute_copy(&celt), core::mem::transmute_copy(&pval), core::mem::transmute_copy(&pcfetched)).into()
        }
        unsafe extern "system" fn Reset<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IEnumDirectoryObject_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEnumDirectoryObject_Impl::Reset(this).into()
        }
        unsafe extern "system" fn Skip<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, celt: u32) -> windows_core::HRESULT
        where
            Identity: IEnumDirectoryObject_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEnumDirectoryObject_Impl::Skip(this, core::mem::transmute_copy(&celt)).into()
        }
        unsafe extern "system" fn Clone<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppenum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IEnumDirectoryObject_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IEnumDirectoryObject_Impl::Clone(this) {
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
            Reset: Reset::<Identity, OFFSET>,
            Skip: Skip::<Identity, OFFSET>,
            Clone: Clone::<Identity, OFFSET>,
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
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IEnumLocation_Vtbl
    where
        Identity: IEnumLocation_Impl,
    {
        unsafe extern "system" fn Next<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, celt: u32, ppelements: *mut *mut core::ffi::c_void, pceltfetched: *mut u32) -> windows_core::HRESULT
        where
            Identity: IEnumLocation_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEnumLocation_Impl::Next(this, core::mem::transmute_copy(&celt), core::mem::transmute_copy(&ppelements), core::mem::transmute_copy(&pceltfetched)).into()
        }
        unsafe extern "system" fn Reset<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IEnumLocation_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEnumLocation_Impl::Reset(this).into()
        }
        unsafe extern "system" fn Skip<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, celt: u32) -> windows_core::HRESULT
        where
            Identity: IEnumLocation_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEnumLocation_Impl::Skip(this, core::mem::transmute_copy(&celt)).into()
        }
        unsafe extern "system" fn Clone<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppenum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IEnumLocation_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IEnumLocation_Impl::Clone(this) {
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
            Reset: Reset::<Identity, OFFSET>,
            Skip: Skip::<Identity, OFFSET>,
            Clone: Clone::<Identity, OFFSET>,
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
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IEnumMcastScope_Vtbl
    where
        Identity: IEnumMcastScope_Impl,
    {
        unsafe extern "system" fn Next<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, celt: u32, ppscopes: *mut *mut core::ffi::c_void, pceltfetched: *mut u32) -> windows_core::HRESULT
        where
            Identity: IEnumMcastScope_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEnumMcastScope_Impl::Next(this, core::mem::transmute_copy(&celt), core::mem::transmute_copy(&ppscopes), core::mem::transmute_copy(&pceltfetched)).into()
        }
        unsafe extern "system" fn Reset<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IEnumMcastScope_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEnumMcastScope_Impl::Reset(this).into()
        }
        unsafe extern "system" fn Skip<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, celt: u32) -> windows_core::HRESULT
        where
            Identity: IEnumMcastScope_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEnumMcastScope_Impl::Skip(this, core::mem::transmute_copy(&celt)).into()
        }
        unsafe extern "system" fn Clone<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppenum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IEnumMcastScope_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IEnumMcastScope_Impl::Clone(this) {
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
            Reset: Reset::<Identity, OFFSET>,
            Skip: Skip::<Identity, OFFSET>,
            Clone: Clone::<Identity, OFFSET>,
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
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IEnumPhone_Vtbl
    where
        Identity: IEnumPhone_Impl,
    {
        unsafe extern "system" fn Next<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, celt: u32, ppelements: *mut *mut core::ffi::c_void, pceltfetched: *mut u32) -> windows_core::HRESULT
        where
            Identity: IEnumPhone_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEnumPhone_Impl::Next(this, core::mem::transmute_copy(&celt), core::mem::transmute_copy(&ppelements), core::mem::transmute_copy(&pceltfetched)).into()
        }
        unsafe extern "system" fn Reset<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IEnumPhone_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEnumPhone_Impl::Reset(this).into()
        }
        unsafe extern "system" fn Skip<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, celt: u32) -> windows_core::HRESULT
        where
            Identity: IEnumPhone_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEnumPhone_Impl::Skip(this, core::mem::transmute_copy(&celt)).into()
        }
        unsafe extern "system" fn Clone<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppenum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IEnumPhone_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IEnumPhone_Impl::Clone(this) {
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
            Reset: Reset::<Identity, OFFSET>,
            Skip: Skip::<Identity, OFFSET>,
            Clone: Clone::<Identity, OFFSET>,
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
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IEnumPluggableSuperclassInfo_Vtbl
    where
        Identity: IEnumPluggableSuperclassInfo_Impl,
    {
        unsafe extern "system" fn Next<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, celt: u32, ppelements: *mut *mut core::ffi::c_void, pceltfetched: *mut u32) -> windows_core::HRESULT
        where
            Identity: IEnumPluggableSuperclassInfo_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEnumPluggableSuperclassInfo_Impl::Next(this, core::mem::transmute_copy(&celt), core::mem::transmute_copy(&ppelements), core::mem::transmute_copy(&pceltfetched)).into()
        }
        unsafe extern "system" fn Reset<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IEnumPluggableSuperclassInfo_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEnumPluggableSuperclassInfo_Impl::Reset(this).into()
        }
        unsafe extern "system" fn Skip<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, celt: u32) -> windows_core::HRESULT
        where
            Identity: IEnumPluggableSuperclassInfo_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEnumPluggableSuperclassInfo_Impl::Skip(this, core::mem::transmute_copy(&celt)).into()
        }
        unsafe extern "system" fn Clone<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppenum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IEnumPluggableSuperclassInfo_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IEnumPluggableSuperclassInfo_Impl::Clone(this) {
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
            Reset: Reset::<Identity, OFFSET>,
            Skip: Skip::<Identity, OFFSET>,
            Clone: Clone::<Identity, OFFSET>,
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
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IEnumPluggableTerminalClassInfo_Vtbl
    where
        Identity: IEnumPluggableTerminalClassInfo_Impl,
    {
        unsafe extern "system" fn Next<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, celt: u32, ppelements: *mut *mut core::ffi::c_void, pceltfetched: *mut u32) -> windows_core::HRESULT
        where
            Identity: IEnumPluggableTerminalClassInfo_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEnumPluggableTerminalClassInfo_Impl::Next(this, core::mem::transmute_copy(&celt), core::mem::transmute_copy(&ppelements), core::mem::transmute_copy(&pceltfetched)).into()
        }
        unsafe extern "system" fn Reset<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IEnumPluggableTerminalClassInfo_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEnumPluggableTerminalClassInfo_Impl::Reset(this).into()
        }
        unsafe extern "system" fn Skip<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, celt: u32) -> windows_core::HRESULT
        where
            Identity: IEnumPluggableTerminalClassInfo_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEnumPluggableTerminalClassInfo_Impl::Skip(this, core::mem::transmute_copy(&celt)).into()
        }
        unsafe extern "system" fn Clone<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppenum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IEnumPluggableTerminalClassInfo_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IEnumPluggableTerminalClassInfo_Impl::Clone(this) {
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
            Reset: Reset::<Identity, OFFSET>,
            Skip: Skip::<Identity, OFFSET>,
            Clone: Clone::<Identity, OFFSET>,
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
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IEnumQueue_Vtbl
    where
        Identity: IEnumQueue_Impl,
    {
        unsafe extern "system" fn Next<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, celt: u32, ppelements: *mut *mut core::ffi::c_void, pceltfetched: *mut u32) -> windows_core::HRESULT
        where
            Identity: IEnumQueue_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEnumQueue_Impl::Next(this, core::mem::transmute_copy(&celt), core::mem::transmute_copy(&ppelements), core::mem::transmute_copy(&pceltfetched)).into()
        }
        unsafe extern "system" fn Reset<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IEnumQueue_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEnumQueue_Impl::Reset(this).into()
        }
        unsafe extern "system" fn Skip<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, celt: u32) -> windows_core::HRESULT
        where
            Identity: IEnumQueue_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEnumQueue_Impl::Skip(this, core::mem::transmute_copy(&celt)).into()
        }
        unsafe extern "system" fn Clone<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppenum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IEnumQueue_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IEnumQueue_Impl::Clone(this) {
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
            Reset: Reset::<Identity, OFFSET>,
            Skip: Skip::<Identity, OFFSET>,
            Clone: Clone::<Identity, OFFSET>,
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
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IEnumStream_Vtbl
    where
        Identity: IEnumStream_Impl,
    {
        unsafe extern "system" fn Next<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, celt: u32, ppelements: *mut *mut core::ffi::c_void, pceltfetched: *mut u32) -> windows_core::HRESULT
        where
            Identity: IEnumStream_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEnumStream_Impl::Next(this, core::mem::transmute_copy(&celt), core::mem::transmute_copy(&ppelements), core::mem::transmute_copy(&pceltfetched)).into()
        }
        unsafe extern "system" fn Reset<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IEnumStream_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEnumStream_Impl::Reset(this).into()
        }
        unsafe extern "system" fn Skip<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, celt: u32) -> windows_core::HRESULT
        where
            Identity: IEnumStream_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEnumStream_Impl::Skip(this, core::mem::transmute_copy(&celt)).into()
        }
        unsafe extern "system" fn Clone<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppenum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IEnumStream_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IEnumStream_Impl::Clone(this) {
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
            Reset: Reset::<Identity, OFFSET>,
            Skip: Skip::<Identity, OFFSET>,
            Clone: Clone::<Identity, OFFSET>,
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
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IEnumSubStream_Vtbl
    where
        Identity: IEnumSubStream_Impl,
    {
        unsafe extern "system" fn Next<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, celt: u32, ppelements: *mut *mut core::ffi::c_void, pceltfetched: *mut u32) -> windows_core::HRESULT
        where
            Identity: IEnumSubStream_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEnumSubStream_Impl::Next(this, core::mem::transmute_copy(&celt), core::mem::transmute_copy(&ppelements), core::mem::transmute_copy(&pceltfetched)).into()
        }
        unsafe extern "system" fn Reset<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IEnumSubStream_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEnumSubStream_Impl::Reset(this).into()
        }
        unsafe extern "system" fn Skip<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, celt: u32) -> windows_core::HRESULT
        where
            Identity: IEnumSubStream_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEnumSubStream_Impl::Skip(this, core::mem::transmute_copy(&celt)).into()
        }
        unsafe extern "system" fn Clone<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppenum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IEnumSubStream_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IEnumSubStream_Impl::Clone(this) {
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
            Reset: Reset::<Identity, OFFSET>,
            Skip: Skip::<Identity, OFFSET>,
            Clone: Clone::<Identity, OFFSET>,
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
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IEnumTerminal_Vtbl
    where
        Identity: IEnumTerminal_Impl,
    {
        unsafe extern "system" fn Next<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, celt: u32, ppelements: *mut *mut core::ffi::c_void, pceltfetched: *mut u32) -> windows_core::HRESULT
        where
            Identity: IEnumTerminal_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEnumTerminal_Impl::Next(this, core::mem::transmute_copy(&celt), core::mem::transmute_copy(&ppelements), core::mem::transmute_copy(&pceltfetched)).into()
        }
        unsafe extern "system" fn Reset<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IEnumTerminal_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEnumTerminal_Impl::Reset(this).into()
        }
        unsafe extern "system" fn Skip<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, celt: u32) -> windows_core::HRESULT
        where
            Identity: IEnumTerminal_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEnumTerminal_Impl::Skip(this, core::mem::transmute_copy(&celt)).into()
        }
        unsafe extern "system" fn Clone<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppenum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IEnumTerminal_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IEnumTerminal_Impl::Clone(this) {
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
            Reset: Reset::<Identity, OFFSET>,
            Skip: Skip::<Identity, OFFSET>,
            Clone: Clone::<Identity, OFFSET>,
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
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IEnumTerminalClass_Vtbl
    where
        Identity: IEnumTerminalClass_Impl,
    {
        unsafe extern "system" fn Next<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, celt: u32, pelements: *mut windows_core::GUID, pceltfetched: *mut u32) -> windows_core::HRESULT
        where
            Identity: IEnumTerminalClass_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEnumTerminalClass_Impl::Next(this, core::mem::transmute_copy(&celt), core::mem::transmute_copy(&pelements), core::mem::transmute_copy(&pceltfetched)).into()
        }
        unsafe extern "system" fn Reset<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IEnumTerminalClass_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEnumTerminalClass_Impl::Reset(this).into()
        }
        unsafe extern "system" fn Skip<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, celt: u32) -> windows_core::HRESULT
        where
            Identity: IEnumTerminalClass_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEnumTerminalClass_Impl::Skip(this, core::mem::transmute_copy(&celt)).into()
        }
        unsafe extern "system" fn Clone<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppenum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IEnumTerminalClass_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IEnumTerminalClass_Impl::Clone(this) {
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
            Reset: Reset::<Identity, OFFSET>,
            Skip: Skip::<Identity, OFFSET>,
            Clone: Clone::<Identity, OFFSET>,
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
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IMcastAddressAllocation_Vtbl
    where
        Identity: IMcastAddressAllocation_Impl,
    {
        unsafe extern "system" fn Scopes<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvariant: *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT
        where
            Identity: IMcastAddressAllocation_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IMcastAddressAllocation_Impl::Scopes(this) {
                Ok(ok__) => {
                    pvariant.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnumerateScopes<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppenummcastscope: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IMcastAddressAllocation_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IMcastAddressAllocation_Impl::EnumerateScopes(this) {
                Ok(ok__) => {
                    ppenummcastscope.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RequestAddress<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pscope: *mut core::ffi::c_void, leasestarttime: f64, leasestoptime: f64, numaddresses: i32, ppleaseresponse: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IMcastAddressAllocation_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IMcastAddressAllocation_Impl::RequestAddress(this, windows_core::from_raw_borrowed(&pscope), core::mem::transmute_copy(&leasestarttime), core::mem::transmute_copy(&leasestoptime), core::mem::transmute_copy(&numaddresses)) {
                Ok(ok__) => {
                    ppleaseresponse.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RenewAddress<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, lreserved: i32, prenewrequest: *mut core::ffi::c_void, pprenewresponse: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IMcastAddressAllocation_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IMcastAddressAllocation_Impl::RenewAddress(this, core::mem::transmute_copy(&lreserved), windows_core::from_raw_borrowed(&prenewrequest)) {
                Ok(ok__) => {
                    pprenewresponse.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ReleaseAddress<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, preleaserequest: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IMcastAddressAllocation_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMcastAddressAllocation_Impl::ReleaseAddress(this, windows_core::from_raw_borrowed(&preleaserequest)).into()
        }
        unsafe extern "system" fn CreateLeaseInfo<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, leasestarttime: f64, leasestoptime: f64, dwnumaddresses: u32, ppaddresses: *const windows_core::PCWSTR, prequestid: windows_core::PCWSTR, pserveraddress: windows_core::PCWSTR, ppreleaserequest: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IMcastAddressAllocation_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IMcastAddressAllocation_Impl::CreateLeaseInfo(this, core::mem::transmute_copy(&leasestarttime), core::mem::transmute_copy(&leasestoptime), core::mem::transmute_copy(&dwnumaddresses), core::mem::transmute_copy(&ppaddresses), core::mem::transmute(&prequestid), core::mem::transmute(&pserveraddress)) {
                Ok(ok__) => {
                    ppreleaserequest.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateLeaseInfoFromVariant<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, leasestarttime: f64, leasestoptime: f64, vaddresses: core::mem::MaybeUninit<windows_core::VARIANT>, prequestid: core::mem::MaybeUninit<windows_core::BSTR>, pserveraddress: core::mem::MaybeUninit<windows_core::BSTR>, ppreleaserequest: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IMcastAddressAllocation_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IMcastAddressAllocation_Impl::CreateLeaseInfoFromVariant(this, core::mem::transmute_copy(&leasestarttime), core::mem::transmute_copy(&leasestoptime), core::mem::transmute(&vaddresses), core::mem::transmute(&prequestid), core::mem::transmute(&pserveraddress)) {
                Ok(ok__) => {
                    ppreleaserequest.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            Scopes: Scopes::<Identity, OFFSET>,
            EnumerateScopes: EnumerateScopes::<Identity, OFFSET>,
            RequestAddress: RequestAddress::<Identity, OFFSET>,
            RenewAddress: RenewAddress::<Identity, OFFSET>,
            ReleaseAddress: ReleaseAddress::<Identity, OFFSET>,
            CreateLeaseInfo: CreateLeaseInfo::<Identity, OFFSET>,
            CreateLeaseInfoFromVariant: CreateLeaseInfoFromVariant::<Identity, OFFSET>,
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
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IMcastLeaseInfo_Vtbl
    where
        Identity: IMcastLeaseInfo_Impl,
    {
        unsafe extern "system" fn RequestID<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pprequestid: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IMcastLeaseInfo_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IMcastLeaseInfo_Impl::RequestID(this) {
                Ok(ok__) => {
                    pprequestid.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LeaseStartTime<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ptime: *mut f64) -> windows_core::HRESULT
        where
            Identity: IMcastLeaseInfo_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IMcastLeaseInfo_Impl::LeaseStartTime(this) {
                Ok(ok__) => {
                    ptime.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetLeaseStartTime<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, time: f64) -> windows_core::HRESULT
        where
            Identity: IMcastLeaseInfo_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMcastLeaseInfo_Impl::SetLeaseStartTime(this, core::mem::transmute_copy(&time)).into()
        }
        unsafe extern "system" fn LeaseStopTime<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ptime: *mut f64) -> windows_core::HRESULT
        where
            Identity: IMcastLeaseInfo_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IMcastLeaseInfo_Impl::LeaseStopTime(this) {
                Ok(ok__) => {
                    ptime.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetLeaseStopTime<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, time: f64) -> windows_core::HRESULT
        where
            Identity: IMcastLeaseInfo_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMcastLeaseInfo_Impl::SetLeaseStopTime(this, core::mem::transmute_copy(&time)).into()
        }
        unsafe extern "system" fn AddressCount<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcount: *mut i32) -> windows_core::HRESULT
        where
            Identity: IMcastLeaseInfo_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IMcastLeaseInfo_Impl::AddressCount(this) {
                Ok(ok__) => {
                    pcount.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ServerAddress<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppaddress: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IMcastLeaseInfo_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IMcastLeaseInfo_Impl::ServerAddress(this) {
                Ok(ok__) => {
                    ppaddress.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TTL<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pttl: *mut i32) -> windows_core::HRESULT
        where
            Identity: IMcastLeaseInfo_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IMcastLeaseInfo_Impl::TTL(this) {
                Ok(ok__) => {
                    pttl.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Addresses<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvariant: *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT
        where
            Identity: IMcastLeaseInfo_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IMcastLeaseInfo_Impl::Addresses(this) {
                Ok(ok__) => {
                    pvariant.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnumerateAddresses<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppenumaddresses: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IMcastLeaseInfo_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IMcastLeaseInfo_Impl::EnumerateAddresses(this) {
                Ok(ok__) => {
                    ppenumaddresses.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            RequestID: RequestID::<Identity, OFFSET>,
            LeaseStartTime: LeaseStartTime::<Identity, OFFSET>,
            SetLeaseStartTime: SetLeaseStartTime::<Identity, OFFSET>,
            LeaseStopTime: LeaseStopTime::<Identity, OFFSET>,
            SetLeaseStopTime: SetLeaseStopTime::<Identity, OFFSET>,
            AddressCount: AddressCount::<Identity, OFFSET>,
            ServerAddress: ServerAddress::<Identity, OFFSET>,
            TTL: TTL::<Identity, OFFSET>,
            Addresses: Addresses::<Identity, OFFSET>,
            EnumerateAddresses: EnumerateAddresses::<Identity, OFFSET>,
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
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IMcastScope_Vtbl
    where
        Identity: IMcastScope_Impl,
    {
        unsafe extern "system" fn ScopeID<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pid: *mut i32) -> windows_core::HRESULT
        where
            Identity: IMcastScope_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IMcastScope_Impl::ScopeID(this) {
                Ok(ok__) => {
                    pid.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ServerID<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pid: *mut i32) -> windows_core::HRESULT
        where
            Identity: IMcastScope_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IMcastScope_Impl::ServerID(this) {
                Ok(ok__) => {
                    pid.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn InterfaceID<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pid: *mut i32) -> windows_core::HRESULT
        where
            Identity: IMcastScope_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IMcastScope_Impl::InterfaceID(this) {
                Ok(ok__) => {
                    pid.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ScopeDescription<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppdescription: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IMcastScope_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IMcastScope_Impl::ScopeDescription(this) {
                Ok(ok__) => {
                    ppdescription.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TTL<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pttl: *mut i32) -> windows_core::HRESULT
        where
            Identity: IMcastScope_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IMcastScope_Impl::TTL(this) {
                Ok(ok__) => {
                    pttl.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            ScopeID: ScopeID::<Identity, OFFSET>,
            ServerID: ServerID::<Identity, OFFSET>,
            InterfaceID: InterfaceID::<Identity, OFFSET>,
            ScopeDescription: ScopeDescription::<Identity, OFFSET>,
            TTL: TTL::<Identity, OFFSET>,
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
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ITACDGroup_Vtbl
    where
        Identity: ITACDGroup_Impl,
    {
        unsafe extern "system" fn Name<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppname: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: ITACDGroup_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITACDGroup_Impl::Name(this) {
                Ok(ok__) => {
                    ppname.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnumerateQueues<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppenumqueue: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ITACDGroup_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITACDGroup_Impl::EnumerateQueues(this) {
                Ok(ok__) => {
                    ppenumqueue.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Queues<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvariant: *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT
        where
            Identity: ITACDGroup_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITACDGroup_Impl::Queues(this) {
                Ok(ok__) => {
                    pvariant.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            Name: Name::<Identity, OFFSET>,
            EnumerateQueues: EnumerateQueues::<Identity, OFFSET>,
            Queues: Queues::<Identity, OFFSET>,
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
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ITACDGroupEvent_Vtbl
    where
        Identity: ITACDGroupEvent_Impl,
    {
        unsafe extern "system" fn Group<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppgroup: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ITACDGroupEvent_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITACDGroupEvent_Impl::Group(this) {
                Ok(ok__) => {
                    ppgroup.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Event<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pevent: *mut ACDGROUP_EVENT) -> windows_core::HRESULT
        where
            Identity: ITACDGroupEvent_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITACDGroupEvent_Impl::Event(this) {
                Ok(ok__) => {
                    pevent.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self { base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(), Group: Group::<Identity, OFFSET>, Event: Event::<Identity, OFFSET> }
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
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ITAMMediaFormat_Vtbl
    where
        Identity: ITAMMediaFormat_Impl,
    {
        unsafe extern "system" fn MediaFormat<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppmt: *mut *mut super::super::Media::MediaFoundation::AM_MEDIA_TYPE) -> windows_core::HRESULT
        where
            Identity: ITAMMediaFormat_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITAMMediaFormat_Impl::MediaFormat(this) {
                Ok(ok__) => {
                    ppmt.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMediaFormat<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pmt: *const super::super::Media::MediaFoundation::AM_MEDIA_TYPE) -> windows_core::HRESULT
        where
            Identity: ITAMMediaFormat_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITAMMediaFormat_Impl::SetMediaFormat(this, core::mem::transmute_copy(&pmt)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            MediaFormat: MediaFormat::<Identity, OFFSET>,
            SetMediaFormat: SetMediaFormat::<Identity, OFFSET>,
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
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ITASRTerminalEvent_Vtbl
    where
        Identity: ITASRTerminalEvent_Impl,
    {
        unsafe extern "system" fn Terminal<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppterminal: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ITASRTerminalEvent_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITASRTerminalEvent_Impl::Terminal(this) {
                Ok(ok__) => {
                    ppterminal.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Call<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppcall: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ITASRTerminalEvent_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITASRTerminalEvent_Impl::Call(this) {
                Ok(ok__) => {
                    ppcall.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Error<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, phrerrorcode: *mut windows_core::HRESULT) -> windows_core::HRESULT
        where
            Identity: ITASRTerminalEvent_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITASRTerminalEvent_Impl::Error(this) {
                Ok(ok__) => {
                    phrerrorcode.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            Terminal: Terminal::<Identity, OFFSET>,
            Call: Call::<Identity, OFFSET>,
            Error: Error::<Identity, OFFSET>,
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
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ITAddress_Vtbl
    where
        Identity: ITAddress_Impl,
    {
        unsafe extern "system" fn State<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, paddressstate: *mut ADDRESS_STATE) -> windows_core::HRESULT
        where
            Identity: ITAddress_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITAddress_Impl::State(this) {
                Ok(ok__) => {
                    paddressstate.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AddressName<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppname: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: ITAddress_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITAddress_Impl::AddressName(this) {
                Ok(ok__) => {
                    ppname.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ServiceProviderName<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppname: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: ITAddress_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITAddress_Impl::ServiceProviderName(this) {
                Ok(ok__) => {
                    ppname.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TAPIObject<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pptapiobject: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ITAddress_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITAddress_Impl::TAPIObject(this) {
                Ok(ok__) => {
                    pptapiobject.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateCall<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdestaddress: core::mem::MaybeUninit<windows_core::BSTR>, laddresstype: i32, lmediatypes: i32, ppcall: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ITAddress_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITAddress_Impl::CreateCall(this, core::mem::transmute(&pdestaddress), core::mem::transmute_copy(&laddresstype), core::mem::transmute_copy(&lmediatypes)) {
                Ok(ok__) => {
                    ppcall.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Calls<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvariant: *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT
        where
            Identity: ITAddress_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITAddress_Impl::Calls(this) {
                Ok(ok__) => {
                    pvariant.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnumerateCalls<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppcallenum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ITAddress_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITAddress_Impl::EnumerateCalls(this) {
                Ok(ok__) => {
                    ppcallenum.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DialableAddress<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdialableaddress: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: ITAddress_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITAddress_Impl::DialableAddress(this) {
                Ok(ok__) => {
                    pdialableaddress.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateForwardInfoObject<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppforwardinfo: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ITAddress_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITAddress_Impl::CreateForwardInfoObject(this) {
                Ok(ok__) => {
                    ppforwardinfo.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Forward<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pforwardinfo: *mut core::ffi::c_void, pcall: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ITAddress_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITAddress_Impl::Forward(this, windows_core::from_raw_borrowed(&pforwardinfo), windows_core::from_raw_borrowed(&pcall)).into()
        }
        unsafe extern "system" fn CurrentForwardInfo<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppforwardinfo: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ITAddress_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITAddress_Impl::CurrentForwardInfo(this) {
                Ok(ok__) => {
                    ppforwardinfo.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMessageWaiting<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, fmessagewaiting: super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT
        where
            Identity: ITAddress_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITAddress_Impl::SetMessageWaiting(this, core::mem::transmute_copy(&fmessagewaiting)).into()
        }
        unsafe extern "system" fn MessageWaiting<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pfmessagewaiting: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT
        where
            Identity: ITAddress_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITAddress_Impl::MessageWaiting(this) {
                Ok(ok__) => {
                    pfmessagewaiting.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDoNotDisturb<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, fdonotdisturb: super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT
        where
            Identity: ITAddress_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITAddress_Impl::SetDoNotDisturb(this, core::mem::transmute_copy(&fdonotdisturb)).into()
        }
        unsafe extern "system" fn DoNotDisturb<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pfdonotdisturb: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT
        where
            Identity: ITAddress_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITAddress_Impl::DoNotDisturb(this) {
                Ok(ok__) => {
                    pfdonotdisturb.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            State: State::<Identity, OFFSET>,
            AddressName: AddressName::<Identity, OFFSET>,
            ServiceProviderName: ServiceProviderName::<Identity, OFFSET>,
            TAPIObject: TAPIObject::<Identity, OFFSET>,
            CreateCall: CreateCall::<Identity, OFFSET>,
            Calls: Calls::<Identity, OFFSET>,
            EnumerateCalls: EnumerateCalls::<Identity, OFFSET>,
            DialableAddress: DialableAddress::<Identity, OFFSET>,
            CreateForwardInfoObject: CreateForwardInfoObject::<Identity, OFFSET>,
            Forward: Forward::<Identity, OFFSET>,
            CurrentForwardInfo: CurrentForwardInfo::<Identity, OFFSET>,
            SetMessageWaiting: SetMessageWaiting::<Identity, OFFSET>,
            MessageWaiting: MessageWaiting::<Identity, OFFSET>,
            SetDoNotDisturb: SetDoNotDisturb::<Identity, OFFSET>,
            DoNotDisturb: DoNotDisturb::<Identity, OFFSET>,
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
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ITAddress2_Vtbl
    where
        Identity: ITAddress2_Impl,
    {
        unsafe extern "system" fn Phones<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pphones: *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT
        where
            Identity: ITAddress2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITAddress2_Impl::Phones(this) {
                Ok(ok__) => {
                    pphones.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnumeratePhones<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppenumphone: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ITAddress2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITAddress2_Impl::EnumeratePhones(this) {
                Ok(ok__) => {
                    ppenumphone.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPhoneFromTerminal<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pterminal: *mut core::ffi::c_void, ppphone: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ITAddress2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITAddress2_Impl::GetPhoneFromTerminal(this, windows_core::from_raw_borrowed(&pterminal)) {
                Ok(ok__) => {
                    ppphone.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PreferredPhones<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pphones: *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT
        where
            Identity: ITAddress2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITAddress2_Impl::PreferredPhones(this) {
                Ok(ok__) => {
                    pphones.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnumeratePreferredPhones<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppenumphone: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ITAddress2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITAddress2_Impl::EnumeratePreferredPhones(this) {
                Ok(ok__) => {
                    ppenumphone.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn get_EventFilter<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, tapievent: TAPI_EVENT, lsubevent: i32, penable: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT
        where
            Identity: ITAddress2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITAddress2_Impl::get_EventFilter(this, core::mem::transmute_copy(&tapievent), core::mem::transmute_copy(&lsubevent)) {
                Ok(ok__) => {
                    penable.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn put_EventFilter<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, tapievent: TAPI_EVENT, lsubevent: i32, benable: super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT
        where
            Identity: ITAddress2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITAddress2_Impl::put_EventFilter(this, core::mem::transmute_copy(&tapievent), core::mem::transmute_copy(&lsubevent), core::mem::transmute_copy(&benable)).into()
        }
        unsafe extern "system" fn DeviceSpecific<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcall: *mut core::ffi::c_void, pparams: *const u8, dwsize: u32) -> windows_core::HRESULT
        where
            Identity: ITAddress2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITAddress2_Impl::DeviceSpecific(this, windows_core::from_raw_borrowed(&pcall), core::mem::transmute_copy(&pparams), core::mem::transmute_copy(&dwsize)).into()
        }
        unsafe extern "system" fn DeviceSpecificVariant<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcall: *mut core::ffi::c_void, vardevspecificbytearray: core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT
        where
            Identity: ITAddress2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITAddress2_Impl::DeviceSpecificVariant(this, windows_core::from_raw_borrowed(&pcall), core::mem::transmute(&vardevspecificbytearray)).into()
        }
        unsafe extern "system" fn NegotiateExtVersion<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, llowversion: i32, lhighversion: i32, plextversion: *mut i32) -> windows_core::HRESULT
        where
            Identity: ITAddress2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITAddress2_Impl::NegotiateExtVersion(this, core::mem::transmute_copy(&llowversion), core::mem::transmute_copy(&lhighversion)) {
                Ok(ok__) => {
                    plextversion.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: ITAddress_Vtbl::new::<Identity, OFFSET>(),
            Phones: Phones::<Identity, OFFSET>,
            EnumeratePhones: EnumeratePhones::<Identity, OFFSET>,
            GetPhoneFromTerminal: GetPhoneFromTerminal::<Identity, OFFSET>,
            PreferredPhones: PreferredPhones::<Identity, OFFSET>,
            EnumeratePreferredPhones: EnumeratePreferredPhones::<Identity, OFFSET>,
            get_EventFilter: get_EventFilter::<Identity, OFFSET>,
            put_EventFilter: put_EventFilter::<Identity, OFFSET>,
            DeviceSpecific: DeviceSpecific::<Identity, OFFSET>,
            DeviceSpecificVariant: DeviceSpecificVariant::<Identity, OFFSET>,
            NegotiateExtVersion: NegotiateExtVersion::<Identity, OFFSET>,
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
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ITAddressCapabilities_Vtbl
    where
        Identity: ITAddressCapabilities_Impl,
    {
        unsafe extern "system" fn get_AddressCapability<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, addresscap: ADDRESS_CAPABILITY, plcapability: *mut i32) -> windows_core::HRESULT
        where
            Identity: ITAddressCapabilities_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITAddressCapabilities_Impl::get_AddressCapability(this, core::mem::transmute_copy(&addresscap)) {
                Ok(ok__) => {
                    plcapability.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn get_AddressCapabilityString<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, addresscapstring: ADDRESS_CAPABILITY_STRING, ppcapabilitystring: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: ITAddressCapabilities_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITAddressCapabilities_Impl::get_AddressCapabilityString(this, core::mem::transmute_copy(&addresscapstring)) {
                Ok(ok__) => {
                    ppcapabilitystring.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CallTreatments<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvariant: *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT
        where
            Identity: ITAddressCapabilities_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITAddressCapabilities_Impl::CallTreatments(this) {
                Ok(ok__) => {
                    pvariant.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnumerateCallTreatments<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppenumcalltreatment: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ITAddressCapabilities_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITAddressCapabilities_Impl::EnumerateCallTreatments(this) {
                Ok(ok__) => {
                    ppenumcalltreatment.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CompletionMessages<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvariant: *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT
        where
            Identity: ITAddressCapabilities_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITAddressCapabilities_Impl::CompletionMessages(this) {
                Ok(ok__) => {
                    pvariant.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnumerateCompletionMessages<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppenumcompletionmessage: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ITAddressCapabilities_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITAddressCapabilities_Impl::EnumerateCompletionMessages(this) {
                Ok(ok__) => {
                    ppenumcompletionmessage.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DeviceClasses<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvariant: *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT
        where
            Identity: ITAddressCapabilities_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITAddressCapabilities_Impl::DeviceClasses(this) {
                Ok(ok__) => {
                    pvariant.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnumerateDeviceClasses<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppenumdeviceclass: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ITAddressCapabilities_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITAddressCapabilities_Impl::EnumerateDeviceClasses(this) {
                Ok(ok__) => {
                    ppenumdeviceclass.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            get_AddressCapability: get_AddressCapability::<Identity, OFFSET>,
            get_AddressCapabilityString: get_AddressCapabilityString::<Identity, OFFSET>,
            CallTreatments: CallTreatments::<Identity, OFFSET>,
            EnumerateCallTreatments: EnumerateCallTreatments::<Identity, OFFSET>,
            CompletionMessages: CompletionMessages::<Identity, OFFSET>,
            EnumerateCompletionMessages: EnumerateCompletionMessages::<Identity, OFFSET>,
            DeviceClasses: DeviceClasses::<Identity, OFFSET>,
            EnumerateDeviceClasses: EnumerateDeviceClasses::<Identity, OFFSET>,
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
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ITAddressDeviceSpecificEvent_Vtbl
    where
        Identity: ITAddressDeviceSpecificEvent_Impl,
    {
        unsafe extern "system" fn Address<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppaddress: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ITAddressDeviceSpecificEvent_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITAddressDeviceSpecificEvent_Impl::Address(this) {
                Ok(ok__) => {
                    ppaddress.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Call<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppcall: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ITAddressDeviceSpecificEvent_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITAddressDeviceSpecificEvent_Impl::Call(this) {
                Ok(ok__) => {
                    ppcall.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn lParam1<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pparam1: *mut i32) -> windows_core::HRESULT
        where
            Identity: ITAddressDeviceSpecificEvent_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITAddressDeviceSpecificEvent_Impl::lParam1(this) {
                Ok(ok__) => {
                    pparam1.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn lParam2<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pparam2: *mut i32) -> windows_core::HRESULT
        where
            Identity: ITAddressDeviceSpecificEvent_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITAddressDeviceSpecificEvent_Impl::lParam2(this) {
                Ok(ok__) => {
                    pparam2.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn lParam3<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pparam3: *mut i32) -> windows_core::HRESULT
        where
            Identity: ITAddressDeviceSpecificEvent_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITAddressDeviceSpecificEvent_Impl::lParam3(this) {
                Ok(ok__) => {
                    pparam3.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            Address: Address::<Identity, OFFSET>,
            Call: Call::<Identity, OFFSET>,
            lParam1: lParam1::<Identity, OFFSET>,
            lParam2: lParam2::<Identity, OFFSET>,
            lParam3: lParam3::<Identity, OFFSET>,
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
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ITAddressEvent_Vtbl
    where
        Identity: ITAddressEvent_Impl,
    {
        unsafe extern "system" fn Address<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppaddress: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ITAddressEvent_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITAddressEvent_Impl::Address(this) {
                Ok(ok__) => {
                    ppaddress.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Event<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pevent: *mut ADDRESS_EVENT) -> windows_core::HRESULT
        where
            Identity: ITAddressEvent_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITAddressEvent_Impl::Event(this) {
                Ok(ok__) => {
                    pevent.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Terminal<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppterminal: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ITAddressEvent_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITAddressEvent_Impl::Terminal(this) {
                Ok(ok__) => {
                    ppterminal.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            Address: Address::<Identity, OFFSET>,
            Event: Event::<Identity, OFFSET>,
            Terminal: Terminal::<Identity, OFFSET>,
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
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ITAddressTranslation_Vtbl
    where
        Identity: ITAddressTranslation_Impl,
    {
        unsafe extern "system" fn TranslateAddress<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, paddresstotranslate: core::mem::MaybeUninit<windows_core::BSTR>, lcard: i32, ltranslateoptions: i32, pptranslated: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ITAddressTranslation_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITAddressTranslation_Impl::TranslateAddress(this, core::mem::transmute(&paddresstotranslate), core::mem::transmute_copy(&lcard), core::mem::transmute_copy(&ltranslateoptions)) {
                Ok(ok__) => {
                    pptranslated.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TranslateDialog<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, hwndowner: isize, paddressin: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: ITAddressTranslation_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITAddressTranslation_Impl::TranslateDialog(this, core::mem::transmute_copy(&hwndowner), core::mem::transmute(&paddressin)).into()
        }
        unsafe extern "system" fn EnumerateLocations<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppenumlocation: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ITAddressTranslation_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITAddressTranslation_Impl::EnumerateLocations(this) {
                Ok(ok__) => {
                    ppenumlocation.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Locations<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvariant: *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT
        where
            Identity: ITAddressTranslation_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITAddressTranslation_Impl::Locations(this) {
                Ok(ok__) => {
                    pvariant.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnumerateCallingCards<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppenumcallingcard: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ITAddressTranslation_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITAddressTranslation_Impl::EnumerateCallingCards(this) {
                Ok(ok__) => {
                    ppenumcallingcard.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CallingCards<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvariant: *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT
        where
            Identity: ITAddressTranslation_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITAddressTranslation_Impl::CallingCards(this) {
                Ok(ok__) => {
                    pvariant.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            TranslateAddress: TranslateAddress::<Identity, OFFSET>,
            TranslateDialog: TranslateDialog::<Identity, OFFSET>,
            EnumerateLocations: EnumerateLocations::<Identity, OFFSET>,
            Locations: Locations::<Identity, OFFSET>,
            EnumerateCallingCards: EnumerateCallingCards::<Identity, OFFSET>,
            CallingCards: CallingCards::<Identity, OFFSET>,
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
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ITAddressTranslationInfo_Vtbl
    where
        Identity: ITAddressTranslationInfo_Impl,
    {
        unsafe extern "system" fn DialableString<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppdialablestring: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: ITAddressTranslationInfo_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITAddressTranslationInfo_Impl::DialableString(this) {
                Ok(ok__) => {
                    ppdialablestring.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DisplayableString<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppdisplayablestring: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: ITAddressTranslationInfo_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITAddressTranslationInfo_Impl::DisplayableString(this) {
                Ok(ok__) => {
                    ppdisplayablestring.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CurrentCountryCode<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, countrycode: *mut i32) -> windows_core::HRESULT
        where
            Identity: ITAddressTranslationInfo_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITAddressTranslationInfo_Impl::CurrentCountryCode(this) {
                Ok(ok__) => {
                    countrycode.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DestinationCountryCode<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, countrycode: *mut i32) -> windows_core::HRESULT
        where
            Identity: ITAddressTranslationInfo_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITAddressTranslationInfo_Impl::DestinationCountryCode(this) {
                Ok(ok__) => {
                    countrycode.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TranslationResults<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, plresults: *mut i32) -> windows_core::HRESULT
        where
            Identity: ITAddressTranslationInfo_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITAddressTranslationInfo_Impl::TranslationResults(this) {
                Ok(ok__) => {
                    plresults.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            DialableString: DialableString::<Identity, OFFSET>,
            DisplayableString: DisplayableString::<Identity, OFFSET>,
            CurrentCountryCode: CurrentCountryCode::<Identity, OFFSET>,
            DestinationCountryCode: DestinationCountryCode::<Identity, OFFSET>,
            TranslationResults: TranslationResults::<Identity, OFFSET>,
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
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ITAgent_Vtbl
    where
        Identity: ITAgent_Impl,
    {
        unsafe extern "system" fn EnumerateAgentSessions<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppenumagentsession: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ITAgent_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITAgent_Impl::EnumerateAgentSessions(this) {
                Ok(ok__) => {
                    ppenumagentsession.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateSession<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pacdgroup: *mut core::ffi::c_void, paddress: *mut core::ffi::c_void, ppagentsession: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ITAgent_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITAgent_Impl::CreateSession(this, windows_core::from_raw_borrowed(&pacdgroup), windows_core::from_raw_borrowed(&paddress)) {
                Ok(ok__) => {
                    ppagentsession.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateSessionWithPIN<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pacdgroup: *mut core::ffi::c_void, paddress: *mut core::ffi::c_void, ppin: core::mem::MaybeUninit<windows_core::BSTR>, ppagentsession: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ITAgent_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITAgent_Impl::CreateSessionWithPIN(this, windows_core::from_raw_borrowed(&pacdgroup), windows_core::from_raw_borrowed(&paddress), core::mem::transmute(&ppin)) {
                Ok(ok__) => {
                    ppagentsession.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ID<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppid: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: ITAgent_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITAgent_Impl::ID(this) {
                Ok(ok__) => {
                    ppid.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn User<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppuser: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: ITAgent_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITAgent_Impl::User(this) {
                Ok(ok__) => {
                    ppuser.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetState<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, agentstate: AGENT_STATE) -> windows_core::HRESULT
        where
            Identity: ITAgent_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITAgent_Impl::SetState(this, core::mem::transmute_copy(&agentstate)).into()
        }
        unsafe extern "system" fn State<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pagentstate: *mut AGENT_STATE) -> windows_core::HRESULT
        where
            Identity: ITAgent_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITAgent_Impl::State(this) {
                Ok(ok__) => {
                    pagentstate.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMeasurementPeriod<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, lperiod: i32) -> windows_core::HRESULT
        where
            Identity: ITAgent_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITAgent_Impl::SetMeasurementPeriod(this, core::mem::transmute_copy(&lperiod)).into()
        }
        unsafe extern "system" fn MeasurementPeriod<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, plperiod: *mut i32) -> windows_core::HRESULT
        where
            Identity: ITAgent_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITAgent_Impl::MeasurementPeriod(this) {
                Ok(ok__) => {
                    plperiod.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OverallCallRate<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcycallrate: *mut super::super::System::Com::CY) -> windows_core::HRESULT
        where
            Identity: ITAgent_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITAgent_Impl::OverallCallRate(this) {
                Ok(ok__) => {
                    pcycallrate.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn NumberOfACDCalls<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, plcalls: *mut i32) -> windows_core::HRESULT
        where
            Identity: ITAgent_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITAgent_Impl::NumberOfACDCalls(this) {
                Ok(ok__) => {
                    plcalls.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn NumberOfIncomingCalls<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, plcalls: *mut i32) -> windows_core::HRESULT
        where
            Identity: ITAgent_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITAgent_Impl::NumberOfIncomingCalls(this) {
                Ok(ok__) => {
                    plcalls.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn NumberOfOutgoingCalls<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, plcalls: *mut i32) -> windows_core::HRESULT
        where
            Identity: ITAgent_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITAgent_Impl::NumberOfOutgoingCalls(this) {
                Ok(ok__) => {
                    plcalls.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TotalACDTalkTime<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pltalktime: *mut i32) -> windows_core::HRESULT
        where
            Identity: ITAgent_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITAgent_Impl::TotalACDTalkTime(this) {
                Ok(ok__) => {
                    pltalktime.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TotalACDCallTime<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, plcalltime: *mut i32) -> windows_core::HRESULT
        where
            Identity: ITAgent_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITAgent_Impl::TotalACDCallTime(this) {
                Ok(ok__) => {
                    plcalltime.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TotalWrapUpTime<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, plwrapuptime: *mut i32) -> windows_core::HRESULT
        where
            Identity: ITAgent_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITAgent_Impl::TotalWrapUpTime(this) {
                Ok(ok__) => {
                    plwrapuptime.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AgentSessions<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvariant: *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT
        where
            Identity: ITAgent_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITAgent_Impl::AgentSessions(this) {
                Ok(ok__) => {
                    pvariant.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            EnumerateAgentSessions: EnumerateAgentSessions::<Identity, OFFSET>,
            CreateSession: CreateSession::<Identity, OFFSET>,
            CreateSessionWithPIN: CreateSessionWithPIN::<Identity, OFFSET>,
            ID: ID::<Identity, OFFSET>,
            User: User::<Identity, OFFSET>,
            SetState: SetState::<Identity, OFFSET>,
            State: State::<Identity, OFFSET>,
            SetMeasurementPeriod: SetMeasurementPeriod::<Identity, OFFSET>,
            MeasurementPeriod: MeasurementPeriod::<Identity, OFFSET>,
            OverallCallRate: OverallCallRate::<Identity, OFFSET>,
            NumberOfACDCalls: NumberOfACDCalls::<Identity, OFFSET>,
            NumberOfIncomingCalls: NumberOfIncomingCalls::<Identity, OFFSET>,
            NumberOfOutgoingCalls: NumberOfOutgoingCalls::<Identity, OFFSET>,
            TotalACDTalkTime: TotalACDTalkTime::<Identity, OFFSET>,
            TotalACDCallTime: TotalACDCallTime::<Identity, OFFSET>,
            TotalWrapUpTime: TotalWrapUpTime::<Identity, OFFSET>,
            AgentSessions: AgentSessions::<Identity, OFFSET>,
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
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ITAgentEvent_Vtbl
    where
        Identity: ITAgentEvent_Impl,
    {
        unsafe extern "system" fn Agent<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppagent: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ITAgentEvent_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITAgentEvent_Impl::Agent(this) {
                Ok(ok__) => {
                    ppagent.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Event<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pevent: *mut AGENT_EVENT) -> windows_core::HRESULT
        where
            Identity: ITAgentEvent_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITAgentEvent_Impl::Event(this) {
                Ok(ok__) => {
                    pevent.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self { base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(), Agent: Agent::<Identity, OFFSET>, Event: Event::<Identity, OFFSET> }
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
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ITAgentHandler_Vtbl
    where
        Identity: ITAgentHandler_Impl,
    {
        unsafe extern "system" fn Name<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppname: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: ITAgentHandler_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITAgentHandler_Impl::Name(this) {
                Ok(ok__) => {
                    ppname.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateAgent<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppagent: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ITAgentHandler_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITAgentHandler_Impl::CreateAgent(this) {
                Ok(ok__) => {
                    ppagent.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateAgentWithID<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pid: core::mem::MaybeUninit<windows_core::BSTR>, ppin: core::mem::MaybeUninit<windows_core::BSTR>, ppagent: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ITAgentHandler_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITAgentHandler_Impl::CreateAgentWithID(this, core::mem::transmute(&pid), core::mem::transmute(&ppin)) {
                Ok(ok__) => {
                    ppagent.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnumerateACDGroups<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppenumacdgroup: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ITAgentHandler_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITAgentHandler_Impl::EnumerateACDGroups(this) {
                Ok(ok__) => {
                    ppenumacdgroup.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnumerateUsableAddresses<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppenumaddress: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ITAgentHandler_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITAgentHandler_Impl::EnumerateUsableAddresses(this) {
                Ok(ok__) => {
                    ppenumaddress.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ACDGroups<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvariant: *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT
        where
            Identity: ITAgentHandler_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITAgentHandler_Impl::ACDGroups(this) {
                Ok(ok__) => {
                    pvariant.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UsableAddresses<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvariant: *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT
        where
            Identity: ITAgentHandler_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITAgentHandler_Impl::UsableAddresses(this) {
                Ok(ok__) => {
                    pvariant.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            Name: Name::<Identity, OFFSET>,
            CreateAgent: CreateAgent::<Identity, OFFSET>,
            CreateAgentWithID: CreateAgentWithID::<Identity, OFFSET>,
            EnumerateACDGroups: EnumerateACDGroups::<Identity, OFFSET>,
            EnumerateUsableAddresses: EnumerateUsableAddresses::<Identity, OFFSET>,
            ACDGroups: ACDGroups::<Identity, OFFSET>,
            UsableAddresses: UsableAddresses::<Identity, OFFSET>,
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
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ITAgentHandlerEvent_Vtbl
    where
        Identity: ITAgentHandlerEvent_Impl,
    {
        unsafe extern "system" fn AgentHandler<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppagenthandler: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ITAgentHandlerEvent_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITAgentHandlerEvent_Impl::AgentHandler(this) {
                Ok(ok__) => {
                    ppagenthandler.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Event<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pevent: *mut AGENTHANDLER_EVENT) -> windows_core::HRESULT
        where
            Identity: ITAgentHandlerEvent_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITAgentHandlerEvent_Impl::Event(this) {
                Ok(ok__) => {
                    pevent.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            AgentHandler: AgentHandler::<Identity, OFFSET>,
            Event: Event::<Identity, OFFSET>,
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
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ITAgentSession_Vtbl
    where
        Identity: ITAgentSession_Impl,
    {
        unsafe extern "system" fn Agent<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppagent: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ITAgentSession_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITAgentSession_Impl::Agent(this) {
                Ok(ok__) => {
                    ppagent.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Address<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppaddress: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ITAgentSession_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITAgentSession_Impl::Address(this) {
                Ok(ok__) => {
                    ppaddress.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ACDGroup<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppacdgroup: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ITAgentSession_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITAgentSession_Impl::ACDGroup(this) {
                Ok(ok__) => {
                    ppacdgroup.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetState<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, sessionstate: AGENT_SESSION_STATE) -> windows_core::HRESULT
        where
            Identity: ITAgentSession_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITAgentSession_Impl::SetState(this, core::mem::transmute_copy(&sessionstate)).into()
        }
        unsafe extern "system" fn State<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, psessionstate: *mut AGENT_SESSION_STATE) -> windows_core::HRESULT
        where
            Identity: ITAgentSession_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITAgentSession_Impl::State(this) {
                Ok(ok__) => {
                    psessionstate.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SessionStartTime<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdatesessionstart: *mut f64) -> windows_core::HRESULT
        where
            Identity: ITAgentSession_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITAgentSession_Impl::SessionStartTime(this) {
                Ok(ok__) => {
                    pdatesessionstart.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SessionDuration<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, plduration: *mut i32) -> windows_core::HRESULT
        where
            Identity: ITAgentSession_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITAgentSession_Impl::SessionDuration(this) {
                Ok(ok__) => {
                    plduration.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn NumberOfCalls<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, plcalls: *mut i32) -> windows_core::HRESULT
        where
            Identity: ITAgentSession_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITAgentSession_Impl::NumberOfCalls(this) {
                Ok(ok__) => {
                    plcalls.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TotalTalkTime<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pltalktime: *mut i32) -> windows_core::HRESULT
        where
            Identity: ITAgentSession_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITAgentSession_Impl::TotalTalkTime(this) {
                Ok(ok__) => {
                    pltalktime.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AverageTalkTime<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pltalktime: *mut i32) -> windows_core::HRESULT
        where
            Identity: ITAgentSession_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITAgentSession_Impl::AverageTalkTime(this) {
                Ok(ok__) => {
                    pltalktime.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TotalCallTime<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, plcalltime: *mut i32) -> windows_core::HRESULT
        where
            Identity: ITAgentSession_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITAgentSession_Impl::TotalCallTime(this) {
                Ok(ok__) => {
                    plcalltime.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AverageCallTime<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, plcalltime: *mut i32) -> windows_core::HRESULT
        where
            Identity: ITAgentSession_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITAgentSession_Impl::AverageCallTime(this) {
                Ok(ok__) => {
                    plcalltime.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TotalWrapUpTime<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, plwrapuptime: *mut i32) -> windows_core::HRESULT
        where
            Identity: ITAgentSession_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITAgentSession_Impl::TotalWrapUpTime(this) {
                Ok(ok__) => {
                    plwrapuptime.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AverageWrapUpTime<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, plwrapuptime: *mut i32) -> windows_core::HRESULT
        where
            Identity: ITAgentSession_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITAgentSession_Impl::AverageWrapUpTime(this) {
                Ok(ok__) => {
                    plwrapuptime.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ACDCallRate<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcycallrate: *mut super::super::System::Com::CY) -> windows_core::HRESULT
        where
            Identity: ITAgentSession_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITAgentSession_Impl::ACDCallRate(this) {
                Ok(ok__) => {
                    pcycallrate.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LongestTimeToAnswer<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, planswertime: *mut i32) -> windows_core::HRESULT
        where
            Identity: ITAgentSession_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITAgentSession_Impl::LongestTimeToAnswer(this) {
                Ok(ok__) => {
                    planswertime.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AverageTimeToAnswer<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, planswertime: *mut i32) -> windows_core::HRESULT
        where
            Identity: ITAgentSession_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITAgentSession_Impl::AverageTimeToAnswer(this) {
                Ok(ok__) => {
                    planswertime.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            Agent: Agent::<Identity, OFFSET>,
            Address: Address::<Identity, OFFSET>,
            ACDGroup: ACDGroup::<Identity, OFFSET>,
            SetState: SetState::<Identity, OFFSET>,
            State: State::<Identity, OFFSET>,
            SessionStartTime: SessionStartTime::<Identity, OFFSET>,
            SessionDuration: SessionDuration::<Identity, OFFSET>,
            NumberOfCalls: NumberOfCalls::<Identity, OFFSET>,
            TotalTalkTime: TotalTalkTime::<Identity, OFFSET>,
            AverageTalkTime: AverageTalkTime::<Identity, OFFSET>,
            TotalCallTime: TotalCallTime::<Identity, OFFSET>,
            AverageCallTime: AverageCallTime::<Identity, OFFSET>,
            TotalWrapUpTime: TotalWrapUpTime::<Identity, OFFSET>,
            AverageWrapUpTime: AverageWrapUpTime::<Identity, OFFSET>,
            ACDCallRate: ACDCallRate::<Identity, OFFSET>,
            LongestTimeToAnswer: LongestTimeToAnswer::<Identity, OFFSET>,
            AverageTimeToAnswer: AverageTimeToAnswer::<Identity, OFFSET>,
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
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ITAgentSessionEvent_Vtbl
    where
        Identity: ITAgentSessionEvent_Impl,
    {
        unsafe extern "system" fn Session<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppsession: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ITAgentSessionEvent_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITAgentSessionEvent_Impl::Session(this) {
                Ok(ok__) => {
                    ppsession.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Event<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pevent: *mut AGENT_SESSION_EVENT) -> windows_core::HRESULT
        where
            Identity: ITAgentSessionEvent_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITAgentSessionEvent_Impl::Event(this) {
                Ok(ok__) => {
                    pevent.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            Session: Session::<Identity, OFFSET>,
            Event: Event::<Identity, OFFSET>,
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
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ITAllocatorProperties_Vtbl
    where
        Identity: ITAllocatorProperties_Impl,
    {
        unsafe extern "system" fn SetAllocatorProperties<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pallocproperties: *const super::super::Media::DirectShow::ALLOCATOR_PROPERTIES) -> windows_core::HRESULT
        where
            Identity: ITAllocatorProperties_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITAllocatorProperties_Impl::SetAllocatorProperties(this, core::mem::transmute_copy(&pallocproperties)).into()
        }
        unsafe extern "system" fn GetAllocatorProperties<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pallocproperties: *mut super::super::Media::DirectShow::ALLOCATOR_PROPERTIES) -> windows_core::HRESULT
        where
            Identity: ITAllocatorProperties_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITAllocatorProperties_Impl::GetAllocatorProperties(this) {
                Ok(ok__) => {
                    pallocproperties.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAllocateBuffers<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ballocbuffers: super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: ITAllocatorProperties_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITAllocatorProperties_Impl::SetAllocateBuffers(this, core::mem::transmute_copy(&ballocbuffers)).into()
        }
        unsafe extern "system" fn GetAllocateBuffers<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pballocbuffers: *mut super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: ITAllocatorProperties_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITAllocatorProperties_Impl::GetAllocateBuffers(this) {
                Ok(ok__) => {
                    pballocbuffers.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetBufferSize<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, buffersize: u32) -> windows_core::HRESULT
        where
            Identity: ITAllocatorProperties_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITAllocatorProperties_Impl::SetBufferSize(this, core::mem::transmute_copy(&buffersize)).into()
        }
        unsafe extern "system" fn GetBufferSize<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbuffersize: *mut u32) -> windows_core::HRESULT
        where
            Identity: ITAllocatorProperties_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITAllocatorProperties_Impl::GetBufferSize(this) {
                Ok(ok__) => {
                    pbuffersize.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            SetAllocatorProperties: SetAllocatorProperties::<Identity, OFFSET>,
            GetAllocatorProperties: GetAllocatorProperties::<Identity, OFFSET>,
            SetAllocateBuffers: SetAllocateBuffers::<Identity, OFFSET>,
            GetAllocateBuffers: GetAllocateBuffers::<Identity, OFFSET>,
            SetBufferSize: SetBufferSize::<Identity, OFFSET>,
            GetBufferSize: GetBufferSize::<Identity, OFFSET>,
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
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ITAutomatedPhoneControl_Vtbl
    where
        Identity: ITAutomatedPhoneControl_Impl,
    {
        unsafe extern "system" fn StartTone<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, tone: PHONE_TONE, lduration: i32) -> windows_core::HRESULT
        where
            Identity: ITAutomatedPhoneControl_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITAutomatedPhoneControl_Impl::StartTone(this, core::mem::transmute_copy(&tone), core::mem::transmute_copy(&lduration)).into()
        }
        unsafe extern "system" fn StopTone<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ITAutomatedPhoneControl_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITAutomatedPhoneControl_Impl::StopTone(this).into()
        }
        unsafe extern "system" fn Tone<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ptone: *mut PHONE_TONE) -> windows_core::HRESULT
        where
            Identity: ITAutomatedPhoneControl_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITAutomatedPhoneControl_Impl::Tone(this) {
                Ok(ok__) => {
                    ptone.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn StartRinger<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, lringmode: i32, lduration: i32) -> windows_core::HRESULT
        where
            Identity: ITAutomatedPhoneControl_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITAutomatedPhoneControl_Impl::StartRinger(this, core::mem::transmute_copy(&lringmode), core::mem::transmute_copy(&lduration)).into()
        }
        unsafe extern "system" fn StopRinger<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ITAutomatedPhoneControl_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITAutomatedPhoneControl_Impl::StopRinger(this).into()
        }
        unsafe extern "system" fn Ringer<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pfringing: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT
        where
            Identity: ITAutomatedPhoneControl_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITAutomatedPhoneControl_Impl::Ringer(this) {
                Ok(ok__) => {
                    pfringing.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPhoneHandlingEnabled<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, fenabled: super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT
        where
            Identity: ITAutomatedPhoneControl_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITAutomatedPhoneControl_Impl::SetPhoneHandlingEnabled(this, core::mem::transmute_copy(&fenabled)).into()
        }
        unsafe extern "system" fn PhoneHandlingEnabled<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pfenabled: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT
        where
            Identity: ITAutomatedPhoneControl_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITAutomatedPhoneControl_Impl::PhoneHandlingEnabled(this) {
                Ok(ok__) => {
                    pfenabled.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAutoEndOfNumberTimeout<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ltimeout: i32) -> windows_core::HRESULT
        where
            Identity: ITAutomatedPhoneControl_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITAutomatedPhoneControl_Impl::SetAutoEndOfNumberTimeout(this, core::mem::transmute_copy(&ltimeout)).into()
        }
        unsafe extern "system" fn AutoEndOfNumberTimeout<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pltimeout: *mut i32) -> windows_core::HRESULT
        where
            Identity: ITAutomatedPhoneControl_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITAutomatedPhoneControl_Impl::AutoEndOfNumberTimeout(this) {
                Ok(ok__) => {
                    pltimeout.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAutoDialtone<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, fenabled: super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT
        where
            Identity: ITAutomatedPhoneControl_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITAutomatedPhoneControl_Impl::SetAutoDialtone(this, core::mem::transmute_copy(&fenabled)).into()
        }
        unsafe extern "system" fn AutoDialtone<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pfenabled: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT
        where
            Identity: ITAutomatedPhoneControl_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITAutomatedPhoneControl_Impl::AutoDialtone(this) {
                Ok(ok__) => {
                    pfenabled.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAutoStopTonesOnOnHook<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, fenabled: super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT
        where
            Identity: ITAutomatedPhoneControl_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITAutomatedPhoneControl_Impl::SetAutoStopTonesOnOnHook(this, core::mem::transmute_copy(&fenabled)).into()
        }
        unsafe extern "system" fn AutoStopTonesOnOnHook<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pfenabled: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT
        where
            Identity: ITAutomatedPhoneControl_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITAutomatedPhoneControl_Impl::AutoStopTonesOnOnHook(this) {
                Ok(ok__) => {
                    pfenabled.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAutoStopRingOnOffHook<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, fenabled: super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT
        where
            Identity: ITAutomatedPhoneControl_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITAutomatedPhoneControl_Impl::SetAutoStopRingOnOffHook(this, core::mem::transmute_copy(&fenabled)).into()
        }
        unsafe extern "system" fn AutoStopRingOnOffHook<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pfenabled: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT
        where
            Identity: ITAutomatedPhoneControl_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITAutomatedPhoneControl_Impl::AutoStopRingOnOffHook(this) {
                Ok(ok__) => {
                    pfenabled.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAutoKeypadTones<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, fenabled: super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT
        where
            Identity: ITAutomatedPhoneControl_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITAutomatedPhoneControl_Impl::SetAutoKeypadTones(this, core::mem::transmute_copy(&fenabled)).into()
        }
        unsafe extern "system" fn AutoKeypadTones<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pfenabled: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT
        where
            Identity: ITAutomatedPhoneControl_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITAutomatedPhoneControl_Impl::AutoKeypadTones(this) {
                Ok(ok__) => {
                    pfenabled.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAutoKeypadTonesMinimumDuration<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, lduration: i32) -> windows_core::HRESULT
        where
            Identity: ITAutomatedPhoneControl_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITAutomatedPhoneControl_Impl::SetAutoKeypadTonesMinimumDuration(this, core::mem::transmute_copy(&lduration)).into()
        }
        unsafe extern "system" fn AutoKeypadTonesMinimumDuration<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, plduration: *mut i32) -> windows_core::HRESULT
        where
            Identity: ITAutomatedPhoneControl_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITAutomatedPhoneControl_Impl::AutoKeypadTonesMinimumDuration(this) {
                Ok(ok__) => {
                    plduration.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAutoVolumeControl<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, fenabled: super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT
        where
            Identity: ITAutomatedPhoneControl_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITAutomatedPhoneControl_Impl::SetAutoVolumeControl(this, core::mem::transmute_copy(&fenabled)).into()
        }
        unsafe extern "system" fn AutoVolumeControl<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, fenabled: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT
        where
            Identity: ITAutomatedPhoneControl_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITAutomatedPhoneControl_Impl::AutoVolumeControl(this) {
                Ok(ok__) => {
                    fenabled.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAutoVolumeControlStep<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, lstepsize: i32) -> windows_core::HRESULT
        where
            Identity: ITAutomatedPhoneControl_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITAutomatedPhoneControl_Impl::SetAutoVolumeControlStep(this, core::mem::transmute_copy(&lstepsize)).into()
        }
        unsafe extern "system" fn AutoVolumeControlStep<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, plstepsize: *mut i32) -> windows_core::HRESULT
        where
            Identity: ITAutomatedPhoneControl_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITAutomatedPhoneControl_Impl::AutoVolumeControlStep(this) {
                Ok(ok__) => {
                    plstepsize.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAutoVolumeControlRepeatDelay<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ldelay: i32) -> windows_core::HRESULT
        where
            Identity: ITAutomatedPhoneControl_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITAutomatedPhoneControl_Impl::SetAutoVolumeControlRepeatDelay(this, core::mem::transmute_copy(&ldelay)).into()
        }
        unsafe extern "system" fn AutoVolumeControlRepeatDelay<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pldelay: *mut i32) -> windows_core::HRESULT
        where
            Identity: ITAutomatedPhoneControl_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITAutomatedPhoneControl_Impl::AutoVolumeControlRepeatDelay(this) {
                Ok(ok__) => {
                    pldelay.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAutoVolumeControlRepeatPeriod<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, lperiod: i32) -> windows_core::HRESULT
        where
            Identity: ITAutomatedPhoneControl_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITAutomatedPhoneControl_Impl::SetAutoVolumeControlRepeatPeriod(this, core::mem::transmute_copy(&lperiod)).into()
        }
        unsafe extern "system" fn AutoVolumeControlRepeatPeriod<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, plperiod: *mut i32) -> windows_core::HRESULT
        where
            Identity: ITAutomatedPhoneControl_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITAutomatedPhoneControl_Impl::AutoVolumeControlRepeatPeriod(this) {
                Ok(ok__) => {
                    plperiod.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SelectCall<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcall: *mut core::ffi::c_void, fselectdefaultterminals: super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT
        where
            Identity: ITAutomatedPhoneControl_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITAutomatedPhoneControl_Impl::SelectCall(this, windows_core::from_raw_borrowed(&pcall), core::mem::transmute_copy(&fselectdefaultterminals)).into()
        }
        unsafe extern "system" fn UnselectCall<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcall: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ITAutomatedPhoneControl_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITAutomatedPhoneControl_Impl::UnselectCall(this, windows_core::from_raw_borrowed(&pcall)).into()
        }
        unsafe extern "system" fn EnumerateSelectedCalls<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppcallenum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ITAutomatedPhoneControl_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITAutomatedPhoneControl_Impl::EnumerateSelectedCalls(this) {
                Ok(ok__) => {
                    ppcallenum.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SelectedCalls<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvariant: *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT
        where
            Identity: ITAutomatedPhoneControl_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITAutomatedPhoneControl_Impl::SelectedCalls(this) {
                Ok(ok__) => {
                    pvariant.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            StartTone: StartTone::<Identity, OFFSET>,
            StopTone: StopTone::<Identity, OFFSET>,
            Tone: Tone::<Identity, OFFSET>,
            StartRinger: StartRinger::<Identity, OFFSET>,
            StopRinger: StopRinger::<Identity, OFFSET>,
            Ringer: Ringer::<Identity, OFFSET>,
            SetPhoneHandlingEnabled: SetPhoneHandlingEnabled::<Identity, OFFSET>,
            PhoneHandlingEnabled: PhoneHandlingEnabled::<Identity, OFFSET>,
            SetAutoEndOfNumberTimeout: SetAutoEndOfNumberTimeout::<Identity, OFFSET>,
            AutoEndOfNumberTimeout: AutoEndOfNumberTimeout::<Identity, OFFSET>,
            SetAutoDialtone: SetAutoDialtone::<Identity, OFFSET>,
            AutoDialtone: AutoDialtone::<Identity, OFFSET>,
            SetAutoStopTonesOnOnHook: SetAutoStopTonesOnOnHook::<Identity, OFFSET>,
            AutoStopTonesOnOnHook: AutoStopTonesOnOnHook::<Identity, OFFSET>,
            SetAutoStopRingOnOffHook: SetAutoStopRingOnOffHook::<Identity, OFFSET>,
            AutoStopRingOnOffHook: AutoStopRingOnOffHook::<Identity, OFFSET>,
            SetAutoKeypadTones: SetAutoKeypadTones::<Identity, OFFSET>,
            AutoKeypadTones: AutoKeypadTones::<Identity, OFFSET>,
            SetAutoKeypadTonesMinimumDuration: SetAutoKeypadTonesMinimumDuration::<Identity, OFFSET>,
            AutoKeypadTonesMinimumDuration: AutoKeypadTonesMinimumDuration::<Identity, OFFSET>,
            SetAutoVolumeControl: SetAutoVolumeControl::<Identity, OFFSET>,
            AutoVolumeControl: AutoVolumeControl::<Identity, OFFSET>,
            SetAutoVolumeControlStep: SetAutoVolumeControlStep::<Identity, OFFSET>,
            AutoVolumeControlStep: AutoVolumeControlStep::<Identity, OFFSET>,
            SetAutoVolumeControlRepeatDelay: SetAutoVolumeControlRepeatDelay::<Identity, OFFSET>,
            AutoVolumeControlRepeatDelay: AutoVolumeControlRepeatDelay::<Identity, OFFSET>,
            SetAutoVolumeControlRepeatPeriod: SetAutoVolumeControlRepeatPeriod::<Identity, OFFSET>,
            AutoVolumeControlRepeatPeriod: AutoVolumeControlRepeatPeriod::<Identity, OFFSET>,
            SelectCall: SelectCall::<Identity, OFFSET>,
            UnselectCall: UnselectCall::<Identity, OFFSET>,
            EnumerateSelectedCalls: EnumerateSelectedCalls::<Identity, OFFSET>,
            SelectedCalls: SelectedCalls::<Identity, OFFSET>,
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
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ITBasicAudioTerminal_Vtbl
    where
        Identity: ITBasicAudioTerminal_Impl,
    {
        unsafe extern "system" fn SetVolume<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, lvolume: i32) -> windows_core::HRESULT
        where
            Identity: ITBasicAudioTerminal_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITBasicAudioTerminal_Impl::SetVolume(this, core::mem::transmute_copy(&lvolume)).into()
        }
        unsafe extern "system" fn Volume<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, plvolume: *mut i32) -> windows_core::HRESULT
        where
            Identity: ITBasicAudioTerminal_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITBasicAudioTerminal_Impl::Volume(this) {
                Ok(ok__) => {
                    plvolume.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetBalance<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, lbalance: i32) -> windows_core::HRESULT
        where
            Identity: ITBasicAudioTerminal_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITBasicAudioTerminal_Impl::SetBalance(this, core::mem::transmute_copy(&lbalance)).into()
        }
        unsafe extern "system" fn Balance<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, plbalance: *mut i32) -> windows_core::HRESULT
        where
            Identity: ITBasicAudioTerminal_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITBasicAudioTerminal_Impl::Balance(this) {
                Ok(ok__) => {
                    plbalance.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            SetVolume: SetVolume::<Identity, OFFSET>,
            Volume: Volume::<Identity, OFFSET>,
            SetBalance: SetBalance::<Identity, OFFSET>,
            Balance: Balance::<Identity, OFFSET>,
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
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ITBasicCallControl_Vtbl
    where
        Identity: ITBasicCallControl_Impl,
    {
        unsafe extern "system" fn Connect<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, fsync: super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT
        where
            Identity: ITBasicCallControl_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITBasicCallControl_Impl::Connect(this, core::mem::transmute_copy(&fsync)).into()
        }
        unsafe extern "system" fn Answer<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ITBasicCallControl_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITBasicCallControl_Impl::Answer(this).into()
        }
        unsafe extern "system" fn Disconnect<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, code: DISCONNECT_CODE) -> windows_core::HRESULT
        where
            Identity: ITBasicCallControl_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITBasicCallControl_Impl::Disconnect(this, core::mem::transmute_copy(&code)).into()
        }
        unsafe extern "system" fn Hold<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, fhold: super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT
        where
            Identity: ITBasicCallControl_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITBasicCallControl_Impl::Hold(this, core::mem::transmute_copy(&fhold)).into()
        }
        unsafe extern "system" fn HandoffDirect<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, papplicationname: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: ITBasicCallControl_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITBasicCallControl_Impl::HandoffDirect(this, core::mem::transmute(&papplicationname)).into()
        }
        unsafe extern "system" fn HandoffIndirect<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, lmediatype: i32) -> windows_core::HRESULT
        where
            Identity: ITBasicCallControl_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITBasicCallControl_Impl::HandoffIndirect(this, core::mem::transmute_copy(&lmediatype)).into()
        }
        unsafe extern "system" fn Conference<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcall: *mut core::ffi::c_void, fsync: super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT
        where
            Identity: ITBasicCallControl_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITBasicCallControl_Impl::Conference(this, windows_core::from_raw_borrowed(&pcall), core::mem::transmute_copy(&fsync)).into()
        }
        unsafe extern "system" fn Transfer<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcall: *mut core::ffi::c_void, fsync: super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT
        where
            Identity: ITBasicCallControl_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITBasicCallControl_Impl::Transfer(this, windows_core::from_raw_borrowed(&pcall), core::mem::transmute_copy(&fsync)).into()
        }
        unsafe extern "system" fn BlindTransfer<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdestaddress: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: ITBasicCallControl_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITBasicCallControl_Impl::BlindTransfer(this, core::mem::transmute(&pdestaddress)).into()
        }
        unsafe extern "system" fn SwapHold<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcall: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ITBasicCallControl_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITBasicCallControl_Impl::SwapHold(this, windows_core::from_raw_borrowed(&pcall)).into()
        }
        unsafe extern "system" fn ParkDirect<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pparkaddress: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: ITBasicCallControl_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITBasicCallControl_Impl::ParkDirect(this, core::mem::transmute(&pparkaddress)).into()
        }
        unsafe extern "system" fn ParkIndirect<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppnondiraddress: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: ITBasicCallControl_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITBasicCallControl_Impl::ParkIndirect(this) {
                Ok(ok__) => {
                    ppnondiraddress.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Unpark<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ITBasicCallControl_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITBasicCallControl_Impl::Unpark(this).into()
        }
        unsafe extern "system" fn SetQOS<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, lmediatype: i32, servicelevel: QOS_SERVICE_LEVEL) -> windows_core::HRESULT
        where
            Identity: ITBasicCallControl_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITBasicCallControl_Impl::SetQOS(this, core::mem::transmute_copy(&lmediatype), core::mem::transmute_copy(&servicelevel)).into()
        }
        unsafe extern "system" fn Pickup<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pgroupid: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: ITBasicCallControl_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITBasicCallControl_Impl::Pickup(this, core::mem::transmute(&pgroupid)).into()
        }
        unsafe extern "system" fn Dial<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdestaddress: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: ITBasicCallControl_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITBasicCallControl_Impl::Dial(this, core::mem::transmute(&pdestaddress)).into()
        }
        unsafe extern "system" fn Finish<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, finishmode: FINISH_MODE) -> windows_core::HRESULT
        where
            Identity: ITBasicCallControl_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITBasicCallControl_Impl::Finish(this, core::mem::transmute_copy(&finishmode)).into()
        }
        unsafe extern "system" fn RemoveFromConference<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ITBasicCallControl_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITBasicCallControl_Impl::RemoveFromConference(this).into()
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            Connect: Connect::<Identity, OFFSET>,
            Answer: Answer::<Identity, OFFSET>,
            Disconnect: Disconnect::<Identity, OFFSET>,
            Hold: Hold::<Identity, OFFSET>,
            HandoffDirect: HandoffDirect::<Identity, OFFSET>,
            HandoffIndirect: HandoffIndirect::<Identity, OFFSET>,
            Conference: Conference::<Identity, OFFSET>,
            Transfer: Transfer::<Identity, OFFSET>,
            BlindTransfer: BlindTransfer::<Identity, OFFSET>,
            SwapHold: SwapHold::<Identity, OFFSET>,
            ParkDirect: ParkDirect::<Identity, OFFSET>,
            ParkIndirect: ParkIndirect::<Identity, OFFSET>,
            Unpark: Unpark::<Identity, OFFSET>,
            SetQOS: SetQOS::<Identity, OFFSET>,
            Pickup: Pickup::<Identity, OFFSET>,
            Dial: Dial::<Identity, OFFSET>,
            Finish: Finish::<Identity, OFFSET>,
            RemoveFromConference: RemoveFromConference::<Identity, OFFSET>,
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
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ITBasicCallControl2_Vtbl
    where
        Identity: ITBasicCallControl2_Impl,
    {
        unsafe extern "system" fn RequestTerminal<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrterminalclassguid: core::mem::MaybeUninit<windows_core::BSTR>, lmediatype: i32, direction: TERMINAL_DIRECTION, ppterminal: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ITBasicCallControl2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITBasicCallControl2_Impl::RequestTerminal(this, core::mem::transmute(&bstrterminalclassguid), core::mem::transmute_copy(&lmediatype), core::mem::transmute_copy(&direction)) {
                Ok(ok__) => {
                    ppterminal.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SelectTerminalOnCall<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pterminal: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ITBasicCallControl2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITBasicCallControl2_Impl::SelectTerminalOnCall(this, windows_core::from_raw_borrowed(&pterminal)).into()
        }
        unsafe extern "system" fn UnselectTerminalOnCall<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pterminal: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ITBasicCallControl2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITBasicCallControl2_Impl::UnselectTerminalOnCall(this, windows_core::from_raw_borrowed(&pterminal)).into()
        }
        Self {
            base__: ITBasicCallControl_Vtbl::new::<Identity, OFFSET>(),
            RequestTerminal: RequestTerminal::<Identity, OFFSET>,
            SelectTerminalOnCall: SelectTerminalOnCall::<Identity, OFFSET>,
            UnselectTerminalOnCall: UnselectTerminalOnCall::<Identity, OFFSET>,
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
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ITCallHub_Vtbl
    where
        Identity: ITCallHub_Impl,
    {
        unsafe extern "system" fn Clear<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ITCallHub_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITCallHub_Impl::Clear(this).into()
        }
        unsafe extern "system" fn EnumerateCalls<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppenumcall: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ITCallHub_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITCallHub_Impl::EnumerateCalls(this) {
                Ok(ok__) => {
                    ppenumcall.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Calls<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcalls: *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT
        where
            Identity: ITCallHub_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITCallHub_Impl::Calls(this) {
                Ok(ok__) => {
                    pcalls.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn NumCalls<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, plcalls: *mut i32) -> windows_core::HRESULT
        where
            Identity: ITCallHub_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITCallHub_Impl::NumCalls(this) {
                Ok(ok__) => {
                    plcalls.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn State<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pstate: *mut CALLHUB_STATE) -> windows_core::HRESULT
        where
            Identity: ITCallHub_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITCallHub_Impl::State(this) {
                Ok(ok__) => {
                    pstate.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            Clear: Clear::<Identity, OFFSET>,
            EnumerateCalls: EnumerateCalls::<Identity, OFFSET>,
            Calls: Calls::<Identity, OFFSET>,
            NumCalls: NumCalls::<Identity, OFFSET>,
            State: State::<Identity, OFFSET>,
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
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ITCallHubEvent_Vtbl
    where
        Identity: ITCallHubEvent_Impl,
    {
        unsafe extern "system" fn Event<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pevent: *mut CALLHUB_EVENT) -> windows_core::HRESULT
        where
            Identity: ITCallHubEvent_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITCallHubEvent_Impl::Event(this) {
                Ok(ok__) => {
                    pevent.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CallHub<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppcallhub: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ITCallHubEvent_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITCallHubEvent_Impl::CallHub(this) {
                Ok(ok__) => {
                    ppcallhub.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Call<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppcall: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ITCallHubEvent_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITCallHubEvent_Impl::Call(this) {
                Ok(ok__) => {
                    ppcall.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            Event: Event::<Identity, OFFSET>,
            CallHub: CallHub::<Identity, OFFSET>,
            Call: Call::<Identity, OFFSET>,
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
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ITCallInfo_Vtbl
    where
        Identity: ITCallInfo_Impl,
    {
        unsafe extern "system" fn Address<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppaddress: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ITCallInfo_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITCallInfo_Impl::Address(this) {
                Ok(ok__) => {
                    ppaddress.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CallState<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcallstate: *mut CALL_STATE) -> windows_core::HRESULT
        where
            Identity: ITCallInfo_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITCallInfo_Impl::CallState(this) {
                Ok(ok__) => {
                    pcallstate.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Privilege<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pprivilege: *mut CALL_PRIVILEGE) -> windows_core::HRESULT
        where
            Identity: ITCallInfo_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITCallInfo_Impl::Privilege(this) {
                Ok(ok__) => {
                    pprivilege.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CallHub<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppcallhub: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ITCallInfo_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITCallInfo_Impl::CallHub(this) {
                Ok(ok__) => {
                    ppcallhub.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn get_CallInfoLong<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, callinfolong: CALLINFO_LONG, plcallinfolongval: *mut i32) -> windows_core::HRESULT
        where
            Identity: ITCallInfo_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITCallInfo_Impl::get_CallInfoLong(this, core::mem::transmute_copy(&callinfolong)) {
                Ok(ok__) => {
                    plcallinfolongval.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn put_CallInfoLong<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, callinfolong: CALLINFO_LONG, lcallinfolongval: i32) -> windows_core::HRESULT
        where
            Identity: ITCallInfo_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITCallInfo_Impl::put_CallInfoLong(this, core::mem::transmute_copy(&callinfolong), core::mem::transmute_copy(&lcallinfolongval)).into()
        }
        unsafe extern "system" fn get_CallInfoString<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, callinfostring: CALLINFO_STRING, ppcallinfostring: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: ITCallInfo_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITCallInfo_Impl::get_CallInfoString(this, core::mem::transmute_copy(&callinfostring)) {
                Ok(ok__) => {
                    ppcallinfostring.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn put_CallInfoString<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, callinfostring: CALLINFO_STRING, pcallinfostring: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: ITCallInfo_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITCallInfo_Impl::put_CallInfoString(this, core::mem::transmute_copy(&callinfostring), core::mem::transmute(&pcallinfostring)).into()
        }
        unsafe extern "system" fn get_CallInfoBuffer<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, callinfobuffer: CALLINFO_BUFFER, ppcallinfobuffer: *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT
        where
            Identity: ITCallInfo_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITCallInfo_Impl::get_CallInfoBuffer(this, core::mem::transmute_copy(&callinfobuffer)) {
                Ok(ok__) => {
                    ppcallinfobuffer.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn put_CallInfoBuffer<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, callinfobuffer: CALLINFO_BUFFER, pcallinfobuffer: core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT
        where
            Identity: ITCallInfo_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITCallInfo_Impl::put_CallInfoBuffer(this, core::mem::transmute_copy(&callinfobuffer), core::mem::transmute(&pcallinfobuffer)).into()
        }
        unsafe extern "system" fn GetCallInfoBuffer<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, callinfobuffer: CALLINFO_BUFFER, pdwsize: *mut u32, ppcallinfobuffer: *mut *mut u8) -> windows_core::HRESULT
        where
            Identity: ITCallInfo_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITCallInfo_Impl::GetCallInfoBuffer(this, core::mem::transmute_copy(&callinfobuffer), core::mem::transmute_copy(&pdwsize), core::mem::transmute_copy(&ppcallinfobuffer)).into()
        }
        unsafe extern "system" fn SetCallInfoBuffer<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, callinfobuffer: CALLINFO_BUFFER, dwsize: u32, pcallinfobuffer: *const u8) -> windows_core::HRESULT
        where
            Identity: ITCallInfo_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITCallInfo_Impl::SetCallInfoBuffer(this, core::mem::transmute_copy(&callinfobuffer), core::mem::transmute_copy(&dwsize), core::mem::transmute_copy(&pcallinfobuffer)).into()
        }
        unsafe extern "system" fn ReleaseUserUserInfo<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ITCallInfo_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITCallInfo_Impl::ReleaseUserUserInfo(this).into()
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            Address: Address::<Identity, OFFSET>,
            CallState: CallState::<Identity, OFFSET>,
            Privilege: Privilege::<Identity, OFFSET>,
            CallHub: CallHub::<Identity, OFFSET>,
            get_CallInfoLong: get_CallInfoLong::<Identity, OFFSET>,
            put_CallInfoLong: put_CallInfoLong::<Identity, OFFSET>,
            get_CallInfoString: get_CallInfoString::<Identity, OFFSET>,
            put_CallInfoString: put_CallInfoString::<Identity, OFFSET>,
            get_CallInfoBuffer: get_CallInfoBuffer::<Identity, OFFSET>,
            put_CallInfoBuffer: put_CallInfoBuffer::<Identity, OFFSET>,
            GetCallInfoBuffer: GetCallInfoBuffer::<Identity, OFFSET>,
            SetCallInfoBuffer: SetCallInfoBuffer::<Identity, OFFSET>,
            ReleaseUserUserInfo: ReleaseUserUserInfo::<Identity, OFFSET>,
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
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ITCallInfo2_Vtbl
    where
        Identity: ITCallInfo2_Impl,
    {
        unsafe extern "system" fn get_EventFilter<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, tapievent: TAPI_EVENT, lsubevent: i32, penable: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT
        where
            Identity: ITCallInfo2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITCallInfo2_Impl::get_EventFilter(this, core::mem::transmute_copy(&tapievent), core::mem::transmute_copy(&lsubevent)) {
                Ok(ok__) => {
                    penable.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn put_EventFilter<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, tapievent: TAPI_EVENT, lsubevent: i32, benable: super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT
        where
            Identity: ITCallInfo2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITCallInfo2_Impl::put_EventFilter(this, core::mem::transmute_copy(&tapievent), core::mem::transmute_copy(&lsubevent), core::mem::transmute_copy(&benable)).into()
        }
        Self {
            base__: ITCallInfo_Vtbl::new::<Identity, OFFSET>(),
            get_EventFilter: get_EventFilter::<Identity, OFFSET>,
            put_EventFilter: put_EventFilter::<Identity, OFFSET>,
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
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ITCallInfoChangeEvent_Vtbl
    where
        Identity: ITCallInfoChangeEvent_Impl,
    {
        unsafe extern "system" fn Call<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppcall: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ITCallInfoChangeEvent_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITCallInfoChangeEvent_Impl::Call(this) {
                Ok(ok__) => {
                    ppcall.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Cause<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcic: *mut CALLINFOCHANGE_CAUSE) -> windows_core::HRESULT
        where
            Identity: ITCallInfoChangeEvent_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITCallInfoChangeEvent_Impl::Cause(this) {
                Ok(ok__) => {
                    pcic.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CallbackInstance<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, plcallbackinstance: *mut i32) -> windows_core::HRESULT
        where
            Identity: ITCallInfoChangeEvent_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITCallInfoChangeEvent_Impl::CallbackInstance(this) {
                Ok(ok__) => {
                    plcallbackinstance.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            Call: Call::<Identity, OFFSET>,
            Cause: Cause::<Identity, OFFSET>,
            CallbackInstance: CallbackInstance::<Identity, OFFSET>,
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
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ITCallMediaEvent_Vtbl
    where
        Identity: ITCallMediaEvent_Impl,
    {
        unsafe extern "system" fn Call<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppcallinfo: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ITCallMediaEvent_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITCallMediaEvent_Impl::Call(this) {
                Ok(ok__) => {
                    ppcallinfo.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Event<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcallmediaevent: *mut CALL_MEDIA_EVENT) -> windows_core::HRESULT
        where
            Identity: ITCallMediaEvent_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITCallMediaEvent_Impl::Event(this) {
                Ok(ok__) => {
                    pcallmediaevent.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Error<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, phrerror: *mut windows_core::HRESULT) -> windows_core::HRESULT
        where
            Identity: ITCallMediaEvent_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITCallMediaEvent_Impl::Error(this) {
                Ok(ok__) => {
                    phrerror.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Terminal<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppterminal: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ITCallMediaEvent_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITCallMediaEvent_Impl::Terminal(this) {
                Ok(ok__) => {
                    ppterminal.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Stream<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppstream: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ITCallMediaEvent_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITCallMediaEvent_Impl::Stream(this) {
                Ok(ok__) => {
                    ppstream.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Cause<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcause: *mut CALL_MEDIA_EVENT_CAUSE) -> windows_core::HRESULT
        where
            Identity: ITCallMediaEvent_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITCallMediaEvent_Impl::Cause(this) {
                Ok(ok__) => {
                    pcause.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            Call: Call::<Identity, OFFSET>,
            Event: Event::<Identity, OFFSET>,
            Error: Error::<Identity, OFFSET>,
            Terminal: Terminal::<Identity, OFFSET>,
            Stream: Stream::<Identity, OFFSET>,
            Cause: Cause::<Identity, OFFSET>,
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
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ITCallNotificationEvent_Vtbl
    where
        Identity: ITCallNotificationEvent_Impl,
    {
        unsafe extern "system" fn Call<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppcall: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ITCallNotificationEvent_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITCallNotificationEvent_Impl::Call(this) {
                Ok(ok__) => {
                    ppcall.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Event<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcallnotificationevent: *mut CALL_NOTIFICATION_EVENT) -> windows_core::HRESULT
        where
            Identity: ITCallNotificationEvent_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITCallNotificationEvent_Impl::Event(this) {
                Ok(ok__) => {
                    pcallnotificationevent.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CallbackInstance<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, plcallbackinstance: *mut i32) -> windows_core::HRESULT
        where
            Identity: ITCallNotificationEvent_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITCallNotificationEvent_Impl::CallbackInstance(this) {
                Ok(ok__) => {
                    plcallbackinstance.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            Call: Call::<Identity, OFFSET>,
            Event: Event::<Identity, OFFSET>,
            CallbackInstance: CallbackInstance::<Identity, OFFSET>,
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
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ITCallStateEvent_Vtbl
    where
        Identity: ITCallStateEvent_Impl,
    {
        unsafe extern "system" fn Call<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppcallinfo: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ITCallStateEvent_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITCallStateEvent_Impl::Call(this) {
                Ok(ok__) => {
                    ppcallinfo.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn State<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcallstate: *mut CALL_STATE) -> windows_core::HRESULT
        where
            Identity: ITCallStateEvent_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITCallStateEvent_Impl::State(this) {
                Ok(ok__) => {
                    pcallstate.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Cause<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcec: *mut CALL_STATE_EVENT_CAUSE) -> windows_core::HRESULT
        where
            Identity: ITCallStateEvent_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITCallStateEvent_Impl::Cause(this) {
                Ok(ok__) => {
                    pcec.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CallbackInstance<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, plcallbackinstance: *mut i32) -> windows_core::HRESULT
        where
            Identity: ITCallStateEvent_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITCallStateEvent_Impl::CallbackInstance(this) {
                Ok(ok__) => {
                    plcallbackinstance.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            Call: Call::<Identity, OFFSET>,
            State: State::<Identity, OFFSET>,
            Cause: Cause::<Identity, OFFSET>,
            CallbackInstance: CallbackInstance::<Identity, OFFSET>,
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
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ITCallingCard_Vtbl
    where
        Identity: ITCallingCard_Impl,
    {
        unsafe extern "system" fn PermanentCardID<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, plcardid: *mut i32) -> windows_core::HRESULT
        where
            Identity: ITCallingCard_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITCallingCard_Impl::PermanentCardID(this) {
                Ok(ok__) => {
                    plcardid.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn NumberOfDigits<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pldigits: *mut i32) -> windows_core::HRESULT
        where
            Identity: ITCallingCard_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITCallingCard_Impl::NumberOfDigits(this) {
                Ok(ok__) => {
                    pldigits.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Options<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ploptions: *mut i32) -> windows_core::HRESULT
        where
            Identity: ITCallingCard_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITCallingCard_Impl::Options(this) {
                Ok(ok__) => {
                    ploptions.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CardName<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppcardname: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: ITCallingCard_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITCallingCard_Impl::CardName(this) {
                Ok(ok__) => {
                    ppcardname.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SameAreaDialingRule<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pprule: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: ITCallingCard_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITCallingCard_Impl::SameAreaDialingRule(this) {
                Ok(ok__) => {
                    pprule.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LongDistanceDialingRule<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pprule: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: ITCallingCard_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITCallingCard_Impl::LongDistanceDialingRule(this) {
                Ok(ok__) => {
                    pprule.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn InternationalDialingRule<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pprule: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: ITCallingCard_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITCallingCard_Impl::InternationalDialingRule(this) {
                Ok(ok__) => {
                    pprule.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            PermanentCardID: PermanentCardID::<Identity, OFFSET>,
            NumberOfDigits: NumberOfDigits::<Identity, OFFSET>,
            Options: Options::<Identity, OFFSET>,
            CardName: CardName::<Identity, OFFSET>,
            SameAreaDialingRule: SameAreaDialingRule::<Identity, OFFSET>,
            LongDistanceDialingRule: LongDistanceDialingRule::<Identity, OFFSET>,
            InternationalDialingRule: InternationalDialingRule::<Identity, OFFSET>,
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
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ITCollection_Vtbl
    where
        Identity: ITCollection_Impl,
    {
        unsafe extern "system" fn Count<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, lcount: *mut i32) -> windows_core::HRESULT
        where
            Identity: ITCollection_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITCollection_Impl::Count(this) {
                Ok(ok__) => {
                    lcount.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn get_Item<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: i32, pvariant: *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT
        where
            Identity: ITCollection_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITCollection_Impl::get_Item(this, core::mem::transmute_copy(&index)) {
                Ok(ok__) => {
                    pvariant.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn _NewEnum<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppnewenum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ITCollection_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITCollection_Impl::_NewEnum(this) {
                Ok(ok__) => {
                    ppnewenum.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            Count: Count::<Identity, OFFSET>,
            get_Item: get_Item::<Identity, OFFSET>,
            _NewEnum: _NewEnum::<Identity, OFFSET>,
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
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ITCollection2_Vtbl
    where
        Identity: ITCollection2_Impl,
    {
        unsafe extern "system" fn Add<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: i32, pvariant: *const core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT
        where
            Identity: ITCollection2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITCollection2_Impl::Add(this, core::mem::transmute_copy(&index), core::mem::transmute_copy(&pvariant)).into()
        }
        unsafe extern "system" fn Remove<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: i32) -> windows_core::HRESULT
        where
            Identity: ITCollection2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITCollection2_Impl::Remove(this, core::mem::transmute_copy(&index)).into()
        }
        Self { base__: ITCollection_Vtbl::new::<Identity, OFFSET>(), Add: Add::<Identity, OFFSET>, Remove: Remove::<Identity, OFFSET> }
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
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ITCustomTone_Vtbl
    where
        Identity: ITCustomTone_Impl,
    {
        unsafe extern "system" fn Frequency<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, plfrequency: *mut i32) -> windows_core::HRESULT
        where
            Identity: ITCustomTone_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITCustomTone_Impl::Frequency(this) {
                Ok(ok__) => {
                    plfrequency.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetFrequency<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, lfrequency: i32) -> windows_core::HRESULT
        where
            Identity: ITCustomTone_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITCustomTone_Impl::SetFrequency(this, core::mem::transmute_copy(&lfrequency)).into()
        }
        unsafe extern "system" fn CadenceOn<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, plcadenceon: *mut i32) -> windows_core::HRESULT
        where
            Identity: ITCustomTone_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITCustomTone_Impl::CadenceOn(this) {
                Ok(ok__) => {
                    plcadenceon.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCadenceOn<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, cadenceon: i32) -> windows_core::HRESULT
        where
            Identity: ITCustomTone_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITCustomTone_Impl::SetCadenceOn(this, core::mem::transmute_copy(&cadenceon)).into()
        }
        unsafe extern "system" fn CadenceOff<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, plcadenceoff: *mut i32) -> windows_core::HRESULT
        where
            Identity: ITCustomTone_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITCustomTone_Impl::CadenceOff(this) {
                Ok(ok__) => {
                    plcadenceoff.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCadenceOff<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, lcadenceoff: i32) -> windows_core::HRESULT
        where
            Identity: ITCustomTone_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITCustomTone_Impl::SetCadenceOff(this, core::mem::transmute_copy(&lcadenceoff)).into()
        }
        unsafe extern "system" fn Volume<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, plvolume: *mut i32) -> windows_core::HRESULT
        where
            Identity: ITCustomTone_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITCustomTone_Impl::Volume(this) {
                Ok(ok__) => {
                    plvolume.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetVolume<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, lvolume: i32) -> windows_core::HRESULT
        where
            Identity: ITCustomTone_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITCustomTone_Impl::SetVolume(this, core::mem::transmute_copy(&lvolume)).into()
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            Frequency: Frequency::<Identity, OFFSET>,
            SetFrequency: SetFrequency::<Identity, OFFSET>,
            CadenceOn: CadenceOn::<Identity, OFFSET>,
            SetCadenceOn: SetCadenceOn::<Identity, OFFSET>,
            CadenceOff: CadenceOff::<Identity, OFFSET>,
            SetCadenceOff: SetCadenceOff::<Identity, OFFSET>,
            Volume: Volume::<Identity, OFFSET>,
            SetVolume: SetVolume::<Identity, OFFSET>,
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
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ITDetectTone_Vtbl
    where
        Identity: ITDetectTone_Impl,
    {
        unsafe extern "system" fn AppSpecific<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, plappspecific: *mut i32) -> windows_core::HRESULT
        where
            Identity: ITDetectTone_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITDetectTone_Impl::AppSpecific(this) {
                Ok(ok__) => {
                    plappspecific.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAppSpecific<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, lappspecific: i32) -> windows_core::HRESULT
        where
            Identity: ITDetectTone_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITDetectTone_Impl::SetAppSpecific(this, core::mem::transmute_copy(&lappspecific)).into()
        }
        unsafe extern "system" fn Duration<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, plduration: *mut i32) -> windows_core::HRESULT
        where
            Identity: ITDetectTone_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITDetectTone_Impl::Duration(this) {
                Ok(ok__) => {
                    plduration.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDuration<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, lduration: i32) -> windows_core::HRESULT
        where
            Identity: ITDetectTone_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITDetectTone_Impl::SetDuration(this, core::mem::transmute_copy(&lduration)).into()
        }
        unsafe extern "system" fn get_Frequency<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: i32, plfrequency: *mut i32) -> windows_core::HRESULT
        where
            Identity: ITDetectTone_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITDetectTone_Impl::get_Frequency(this, core::mem::transmute_copy(&index)) {
                Ok(ok__) => {
                    plfrequency.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn put_Frequency<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: i32, lfrequency: i32) -> windows_core::HRESULT
        where
            Identity: ITDetectTone_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITDetectTone_Impl::put_Frequency(this, core::mem::transmute_copy(&index), core::mem::transmute_copy(&lfrequency)).into()
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            AppSpecific: AppSpecific::<Identity, OFFSET>,
            SetAppSpecific: SetAppSpecific::<Identity, OFFSET>,
            Duration: Duration::<Identity, OFFSET>,
            SetDuration: SetDuration::<Identity, OFFSET>,
            get_Frequency: get_Frequency::<Identity, OFFSET>,
            put_Frequency: put_Frequency::<Identity, OFFSET>,
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
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ITDigitDetectionEvent_Vtbl
    where
        Identity: ITDigitDetectionEvent_Impl,
    {
        unsafe extern "system" fn Call<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppcallinfo: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ITDigitDetectionEvent_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITDigitDetectionEvent_Impl::Call(this) {
                Ok(ok__) => {
                    ppcallinfo.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Digit<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pucdigit: *mut u8) -> windows_core::HRESULT
        where
            Identity: ITDigitDetectionEvent_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITDigitDetectionEvent_Impl::Digit(this) {
                Ok(ok__) => {
                    pucdigit.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DigitMode<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdigitmode: *mut i32) -> windows_core::HRESULT
        where
            Identity: ITDigitDetectionEvent_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITDigitDetectionEvent_Impl::DigitMode(this) {
                Ok(ok__) => {
                    pdigitmode.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TickCount<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pltickcount: *mut i32) -> windows_core::HRESULT
        where
            Identity: ITDigitDetectionEvent_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITDigitDetectionEvent_Impl::TickCount(this) {
                Ok(ok__) => {
                    pltickcount.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CallbackInstance<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, plcallbackinstance: *mut i32) -> windows_core::HRESULT
        where
            Identity: ITDigitDetectionEvent_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITDigitDetectionEvent_Impl::CallbackInstance(this) {
                Ok(ok__) => {
                    plcallbackinstance.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            Call: Call::<Identity, OFFSET>,
            Digit: Digit::<Identity, OFFSET>,
            DigitMode: DigitMode::<Identity, OFFSET>,
            TickCount: TickCount::<Identity, OFFSET>,
            CallbackInstance: CallbackInstance::<Identity, OFFSET>,
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
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ITDigitGenerationEvent_Vtbl
    where
        Identity: ITDigitGenerationEvent_Impl,
    {
        unsafe extern "system" fn Call<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppcallinfo: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ITDigitGenerationEvent_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITDigitGenerationEvent_Impl::Call(this) {
                Ok(ok__) => {
                    ppcallinfo.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GenerationTermination<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, plgenerationtermination: *mut i32) -> windows_core::HRESULT
        where
            Identity: ITDigitGenerationEvent_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITDigitGenerationEvent_Impl::GenerationTermination(this) {
                Ok(ok__) => {
                    plgenerationtermination.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TickCount<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pltickcount: *mut i32) -> windows_core::HRESULT
        where
            Identity: ITDigitGenerationEvent_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITDigitGenerationEvent_Impl::TickCount(this) {
                Ok(ok__) => {
                    pltickcount.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CallbackInstance<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, plcallbackinstance: *mut i32) -> windows_core::HRESULT
        where
            Identity: ITDigitGenerationEvent_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITDigitGenerationEvent_Impl::CallbackInstance(this) {
                Ok(ok__) => {
                    plcallbackinstance.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            Call: Call::<Identity, OFFSET>,
            GenerationTermination: GenerationTermination::<Identity, OFFSET>,
            TickCount: TickCount::<Identity, OFFSET>,
            CallbackInstance: CallbackInstance::<Identity, OFFSET>,
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
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ITDigitsGatheredEvent_Vtbl
    where
        Identity: ITDigitsGatheredEvent_Impl,
    {
        unsafe extern "system" fn Call<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppcallinfo: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ITDigitsGatheredEvent_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITDigitsGatheredEvent_Impl::Call(this) {
                Ok(ok__) => {
                    ppcallinfo.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Digits<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppdigits: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: ITDigitsGatheredEvent_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITDigitsGatheredEvent_Impl::Digits(this) {
                Ok(ok__) => {
                    ppdigits.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GatherTermination<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pgathertermination: *mut TAPI_GATHERTERM) -> windows_core::HRESULT
        where
            Identity: ITDigitsGatheredEvent_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITDigitsGatheredEvent_Impl::GatherTermination(this) {
                Ok(ok__) => {
                    pgathertermination.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TickCount<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pltickcount: *mut i32) -> windows_core::HRESULT
        where
            Identity: ITDigitsGatheredEvent_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITDigitsGatheredEvent_Impl::TickCount(this) {
                Ok(ok__) => {
                    pltickcount.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CallbackInstance<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, plcallbackinstance: *mut i32) -> windows_core::HRESULT
        where
            Identity: ITDigitsGatheredEvent_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITDigitsGatheredEvent_Impl::CallbackInstance(this) {
                Ok(ok__) => {
                    plcallbackinstance.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            Call: Call::<Identity, OFFSET>,
            Digits: Digits::<Identity, OFFSET>,
            GatherTermination: GatherTermination::<Identity, OFFSET>,
            TickCount: TickCount::<Identity, OFFSET>,
            CallbackInstance: CallbackInstance::<Identity, OFFSET>,
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
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ITDirectory_Vtbl
    where
        Identity: ITDirectory_Impl,
    {
        unsafe extern "system" fn DirectoryType<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdirectorytype: *mut DIRECTORY_TYPE) -> windows_core::HRESULT
        where
            Identity: ITDirectory_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITDirectory_Impl::DirectoryType(this) {
                Ok(ok__) => {
                    pdirectorytype.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DisplayName<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pname: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: ITDirectory_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITDirectory_Impl::DisplayName(this) {
                Ok(ok__) => {
                    pname.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsDynamic<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pfdynamic: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT
        where
            Identity: ITDirectory_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITDirectory_Impl::IsDynamic(this) {
                Ok(ok__) => {
                    pfdynamic.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DefaultObjectTTL<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pttl: *mut i32) -> windows_core::HRESULT
        where
            Identity: ITDirectory_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITDirectory_Impl::DefaultObjectTTL(this) {
                Ok(ok__) => {
                    pttl.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDefaultObjectTTL<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ttl: i32) -> windows_core::HRESULT
        where
            Identity: ITDirectory_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITDirectory_Impl::SetDefaultObjectTTL(this, core::mem::transmute_copy(&ttl)).into()
        }
        unsafe extern "system" fn EnableAutoRefresh<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, fenable: super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT
        where
            Identity: ITDirectory_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITDirectory_Impl::EnableAutoRefresh(this, core::mem::transmute_copy(&fenable)).into()
        }
        unsafe extern "system" fn Connect<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, fsecure: super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT
        where
            Identity: ITDirectory_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITDirectory_Impl::Connect(this, core::mem::transmute_copy(&fsecure)).into()
        }
        unsafe extern "system" fn Bind<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdomainname: core::mem::MaybeUninit<windows_core::BSTR>, pusername: core::mem::MaybeUninit<windows_core::BSTR>, ppassword: core::mem::MaybeUninit<windows_core::BSTR>, lflags: i32) -> windows_core::HRESULT
        where
            Identity: ITDirectory_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITDirectory_Impl::Bind(this, core::mem::transmute(&pdomainname), core::mem::transmute(&pusername), core::mem::transmute(&ppassword), core::mem::transmute_copy(&lflags)).into()
        }
        unsafe extern "system" fn AddDirectoryObject<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdirectoryobject: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ITDirectory_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITDirectory_Impl::AddDirectoryObject(this, windows_core::from_raw_borrowed(&pdirectoryobject)).into()
        }
        unsafe extern "system" fn ModifyDirectoryObject<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdirectoryobject: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ITDirectory_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITDirectory_Impl::ModifyDirectoryObject(this, windows_core::from_raw_borrowed(&pdirectoryobject)).into()
        }
        unsafe extern "system" fn RefreshDirectoryObject<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdirectoryobject: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ITDirectory_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITDirectory_Impl::RefreshDirectoryObject(this, windows_core::from_raw_borrowed(&pdirectoryobject)).into()
        }
        unsafe extern "system" fn DeleteDirectoryObject<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdirectoryobject: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ITDirectory_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITDirectory_Impl::DeleteDirectoryObject(this, windows_core::from_raw_borrowed(&pdirectoryobject)).into()
        }
        unsafe extern "system" fn get_DirectoryObjects<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, directoryobjecttype: DIRECTORY_OBJECT_TYPE, pname: core::mem::MaybeUninit<windows_core::BSTR>, pvariant: *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT
        where
            Identity: ITDirectory_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITDirectory_Impl::get_DirectoryObjects(this, core::mem::transmute_copy(&directoryobjecttype), core::mem::transmute(&pname)) {
                Ok(ok__) => {
                    pvariant.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnumerateDirectoryObjects<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, directoryobjecttype: DIRECTORY_OBJECT_TYPE, pname: core::mem::MaybeUninit<windows_core::BSTR>, ppenumobject: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ITDirectory_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITDirectory_Impl::EnumerateDirectoryObjects(this, core::mem::transmute_copy(&directoryobjecttype), core::mem::transmute(&pname)) {
                Ok(ok__) => {
                    ppenumobject.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            DirectoryType: DirectoryType::<Identity, OFFSET>,
            DisplayName: DisplayName::<Identity, OFFSET>,
            IsDynamic: IsDynamic::<Identity, OFFSET>,
            DefaultObjectTTL: DefaultObjectTTL::<Identity, OFFSET>,
            SetDefaultObjectTTL: SetDefaultObjectTTL::<Identity, OFFSET>,
            EnableAutoRefresh: EnableAutoRefresh::<Identity, OFFSET>,
            Connect: Connect::<Identity, OFFSET>,
            Bind: Bind::<Identity, OFFSET>,
            AddDirectoryObject: AddDirectoryObject::<Identity, OFFSET>,
            ModifyDirectoryObject: ModifyDirectoryObject::<Identity, OFFSET>,
            RefreshDirectoryObject: RefreshDirectoryObject::<Identity, OFFSET>,
            DeleteDirectoryObject: DeleteDirectoryObject::<Identity, OFFSET>,
            get_DirectoryObjects: get_DirectoryObjects::<Identity, OFFSET>,
            EnumerateDirectoryObjects: EnumerateDirectoryObjects::<Identity, OFFSET>,
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
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ITDirectoryObject_Vtbl
    where
        Identity: ITDirectoryObject_Impl,
    {
        unsafe extern "system" fn ObjectType<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pobjecttype: *mut DIRECTORY_OBJECT_TYPE) -> windows_core::HRESULT
        where
            Identity: ITDirectoryObject_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITDirectoryObject_Impl::ObjectType(this) {
                Ok(ok__) => {
                    pobjecttype.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Name<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppname: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: ITDirectoryObject_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITDirectoryObject_Impl::Name(this) {
                Ok(ok__) => {
                    ppname.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetName<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pname: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: ITDirectoryObject_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITDirectoryObject_Impl::SetName(this, core::mem::transmute(&pname)).into()
        }
        unsafe extern "system" fn get_DialableAddrs<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwaddresstype: i32, pvariant: *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT
        where
            Identity: ITDirectoryObject_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITDirectoryObject_Impl::get_DialableAddrs(this, core::mem::transmute_copy(&dwaddresstype)) {
                Ok(ok__) => {
                    pvariant.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnumerateDialableAddrs<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwaddresstype: u32, ppenumdialableaddrs: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ITDirectoryObject_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITDirectoryObject_Impl::EnumerateDialableAddrs(this, core::mem::transmute_copy(&dwaddresstype)) {
                Ok(ok__) => {
                    ppenumdialableaddrs.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SecurityDescriptor<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppsecdes: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ITDirectoryObject_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITDirectoryObject_Impl::SecurityDescriptor(this) {
                Ok(ok__) => {
                    ppsecdes.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSecurityDescriptor<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, psecdes: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ITDirectoryObject_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITDirectoryObject_Impl::SetSecurityDescriptor(this, windows_core::from_raw_borrowed(&psecdes)).into()
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            ObjectType: ObjectType::<Identity, OFFSET>,
            Name: Name::<Identity, OFFSET>,
            SetName: SetName::<Identity, OFFSET>,
            get_DialableAddrs: get_DialableAddrs::<Identity, OFFSET>,
            EnumerateDialableAddrs: EnumerateDialableAddrs::<Identity, OFFSET>,
            SecurityDescriptor: SecurityDescriptor::<Identity, OFFSET>,
            SetSecurityDescriptor: SetSecurityDescriptor::<Identity, OFFSET>,
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
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ITDirectoryObjectConference_Vtbl
    where
        Identity: ITDirectoryObjectConference_Impl,
    {
        unsafe extern "system" fn Protocol<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppprotocol: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: ITDirectoryObjectConference_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITDirectoryObjectConference_Impl::Protocol(this) {
                Ok(ok__) => {
                    ppprotocol.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Originator<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pporiginator: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: ITDirectoryObjectConference_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITDirectoryObjectConference_Impl::Originator(this) {
                Ok(ok__) => {
                    pporiginator.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetOriginator<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, poriginator: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: ITDirectoryObjectConference_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITDirectoryObjectConference_Impl::SetOriginator(this, core::mem::transmute(&poriginator)).into()
        }
        unsafe extern "system" fn AdvertisingScope<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, padvertisingscope: *mut RND_ADVERTISING_SCOPE) -> windows_core::HRESULT
        where
            Identity: ITDirectoryObjectConference_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITDirectoryObjectConference_Impl::AdvertisingScope(this) {
                Ok(ok__) => {
                    padvertisingscope.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAdvertisingScope<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, advertisingscope: RND_ADVERTISING_SCOPE) -> windows_core::HRESULT
        where
            Identity: ITDirectoryObjectConference_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITDirectoryObjectConference_Impl::SetAdvertisingScope(this, core::mem::transmute_copy(&advertisingscope)).into()
        }
        unsafe extern "system" fn Url<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppurl: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: ITDirectoryObjectConference_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITDirectoryObjectConference_Impl::Url(this) {
                Ok(ok__) => {
                    ppurl.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetUrl<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, purl: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: ITDirectoryObjectConference_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITDirectoryObjectConference_Impl::SetUrl(this, core::mem::transmute(&purl)).into()
        }
        unsafe extern "system" fn Description<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppdescription: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: ITDirectoryObjectConference_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITDirectoryObjectConference_Impl::Description(this) {
                Ok(ok__) => {
                    ppdescription.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDescription<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdescription: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: ITDirectoryObjectConference_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITDirectoryObjectConference_Impl::SetDescription(this, core::mem::transmute(&pdescription)).into()
        }
        unsafe extern "system" fn IsEncrypted<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pfencrypted: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT
        where
            Identity: ITDirectoryObjectConference_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITDirectoryObjectConference_Impl::IsEncrypted(this) {
                Ok(ok__) => {
                    pfencrypted.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetIsEncrypted<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, fencrypted: super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT
        where
            Identity: ITDirectoryObjectConference_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITDirectoryObjectConference_Impl::SetIsEncrypted(this, core::mem::transmute_copy(&fencrypted)).into()
        }
        unsafe extern "system" fn StartTime<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdate: *mut f64) -> windows_core::HRESULT
        where
            Identity: ITDirectoryObjectConference_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITDirectoryObjectConference_Impl::StartTime(this) {
                Ok(ok__) => {
                    pdate.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetStartTime<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, date: f64) -> windows_core::HRESULT
        where
            Identity: ITDirectoryObjectConference_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITDirectoryObjectConference_Impl::SetStartTime(this, core::mem::transmute_copy(&date)).into()
        }
        unsafe extern "system" fn StopTime<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdate: *mut f64) -> windows_core::HRESULT
        where
            Identity: ITDirectoryObjectConference_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITDirectoryObjectConference_Impl::StopTime(this) {
                Ok(ok__) => {
                    pdate.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetStopTime<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, date: f64) -> windows_core::HRESULT
        where
            Identity: ITDirectoryObjectConference_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITDirectoryObjectConference_Impl::SetStopTime(this, core::mem::transmute_copy(&date)).into()
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            Protocol: Protocol::<Identity, OFFSET>,
            Originator: Originator::<Identity, OFFSET>,
            SetOriginator: SetOriginator::<Identity, OFFSET>,
            AdvertisingScope: AdvertisingScope::<Identity, OFFSET>,
            SetAdvertisingScope: SetAdvertisingScope::<Identity, OFFSET>,
            Url: Url::<Identity, OFFSET>,
            SetUrl: SetUrl::<Identity, OFFSET>,
            Description: Description::<Identity, OFFSET>,
            SetDescription: SetDescription::<Identity, OFFSET>,
            IsEncrypted: IsEncrypted::<Identity, OFFSET>,
            SetIsEncrypted: SetIsEncrypted::<Identity, OFFSET>,
            StartTime: StartTime::<Identity, OFFSET>,
            SetStartTime: SetStartTime::<Identity, OFFSET>,
            StopTime: StopTime::<Identity, OFFSET>,
            SetStopTime: SetStopTime::<Identity, OFFSET>,
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
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ITDirectoryObjectUser_Vtbl
    where
        Identity: ITDirectoryObjectUser_Impl,
    {
        unsafe extern "system" fn IPPhonePrimary<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppname: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: ITDirectoryObjectUser_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITDirectoryObjectUser_Impl::IPPhonePrimary(this) {
                Ok(ok__) => {
                    ppname.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetIPPhonePrimary<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pname: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: ITDirectoryObjectUser_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITDirectoryObjectUser_Impl::SetIPPhonePrimary(this, core::mem::transmute(&pname)).into()
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            IPPhonePrimary: IPPhonePrimary::<Identity, OFFSET>,
            SetIPPhonePrimary: SetIPPhonePrimary::<Identity, OFFSET>,
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
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ITDispatchMapper_Vtbl
    where
        Identity: ITDispatchMapper_Impl,
    {
        unsafe extern "system" fn QueryDispatchInterface<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, piid: core::mem::MaybeUninit<windows_core::BSTR>, pinterfacetomap: *mut core::ffi::c_void, ppreturnedinterface: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ITDispatchMapper_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITDispatchMapper_Impl::QueryDispatchInterface(this, core::mem::transmute(&piid), windows_core::from_raw_borrowed(&pinterfacetomap)) {
                Ok(ok__) => {
                    ppreturnedinterface.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self { base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(), QueryDispatchInterface: QueryDispatchInterface::<Identity, OFFSET> }
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
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ITFileTerminalEvent_Vtbl
    where
        Identity: ITFileTerminalEvent_Impl,
    {
        unsafe extern "system" fn Terminal<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppterminal: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ITFileTerminalEvent_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITFileTerminalEvent_Impl::Terminal(this) {
                Ok(ok__) => {
                    ppterminal.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Track<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pptrackterminal: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ITFileTerminalEvent_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITFileTerminalEvent_Impl::Track(this) {
                Ok(ok__) => {
                    pptrackterminal.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Call<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppcall: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ITFileTerminalEvent_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITFileTerminalEvent_Impl::Call(this) {
                Ok(ok__) => {
                    ppcall.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn State<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pstate: *mut TERMINAL_MEDIA_STATE) -> windows_core::HRESULT
        where
            Identity: ITFileTerminalEvent_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITFileTerminalEvent_Impl::State(this) {
                Ok(ok__) => {
                    pstate.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Cause<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcause: *mut FT_STATE_EVENT_CAUSE) -> windows_core::HRESULT
        where
            Identity: ITFileTerminalEvent_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITFileTerminalEvent_Impl::Cause(this) {
                Ok(ok__) => {
                    pcause.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Error<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, phrerrorcode: *mut windows_core::HRESULT) -> windows_core::HRESULT
        where
            Identity: ITFileTerminalEvent_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITFileTerminalEvent_Impl::Error(this) {
                Ok(ok__) => {
                    phrerrorcode.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            Terminal: Terminal::<Identity, OFFSET>,
            Track: Track::<Identity, OFFSET>,
            Call: Call::<Identity, OFFSET>,
            State: State::<Identity, OFFSET>,
            Cause: Cause::<Identity, OFFSET>,
            Error: Error::<Identity, OFFSET>,
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
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ITFileTrack_Vtbl
    where
        Identity: ITFileTrack_Impl,
    {
        unsafe extern "system" fn Format<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppmt: *mut *mut super::super::Media::MediaFoundation::AM_MEDIA_TYPE) -> windows_core::HRESULT
        where
            Identity: ITFileTrack_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITFileTrack_Impl::Format(this) {
                Ok(ok__) => {
                    ppmt.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetFormat<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pmt: *const super::super::Media::MediaFoundation::AM_MEDIA_TYPE) -> windows_core::HRESULT
        where
            Identity: ITFileTrack_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITFileTrack_Impl::SetFormat(this, core::mem::transmute_copy(&pmt)).into()
        }
        unsafe extern "system" fn ControllingTerminal<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppcontrollingterminal: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ITFileTrack_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITFileTrack_Impl::ControllingTerminal(this) {
                Ok(ok__) => {
                    ppcontrollingterminal.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AudioFormatForScripting<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppaudioformat: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ITFileTrack_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITFileTrack_Impl::AudioFormatForScripting(this) {
                Ok(ok__) => {
                    ppaudioformat.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAudioFormatForScripting<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, paudioformat: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ITFileTrack_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITFileTrack_Impl::SetAudioFormatForScripting(this, windows_core::from_raw_borrowed(&paudioformat)).into()
        }
        unsafe extern "system" fn EmptyAudioFormatForScripting<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppaudioformat: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ITFileTrack_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITFileTrack_Impl::EmptyAudioFormatForScripting(this) {
                Ok(ok__) => {
                    ppaudioformat.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            Format: Format::<Identity, OFFSET>,
            SetFormat: SetFormat::<Identity, OFFSET>,
            ControllingTerminal: ControllingTerminal::<Identity, OFFSET>,
            AudioFormatForScripting: AudioFormatForScripting::<Identity, OFFSET>,
            SetAudioFormatForScripting: SetAudioFormatForScripting::<Identity, OFFSET>,
            EmptyAudioFormatForScripting: EmptyAudioFormatForScripting::<Identity, OFFSET>,
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
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ITForwardInformation_Vtbl
    where
        Identity: ITForwardInformation_Impl,
    {
        unsafe extern "system" fn SetNumRingsNoAnswer<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, lnumrings: i32) -> windows_core::HRESULT
        where
            Identity: ITForwardInformation_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITForwardInformation_Impl::SetNumRingsNoAnswer(this, core::mem::transmute_copy(&lnumrings)).into()
        }
        unsafe extern "system" fn NumRingsNoAnswer<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, plnumrings: *mut i32) -> windows_core::HRESULT
        where
            Identity: ITForwardInformation_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITForwardInformation_Impl::NumRingsNoAnswer(this) {
                Ok(ok__) => {
                    plnumrings.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetForwardType<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, forwardtype: i32, pdestaddress: core::mem::MaybeUninit<windows_core::BSTR>, pcalleraddress: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: ITForwardInformation_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITForwardInformation_Impl::SetForwardType(this, core::mem::transmute_copy(&forwardtype), core::mem::transmute(&pdestaddress), core::mem::transmute(&pcalleraddress)).into()
        }
        unsafe extern "system" fn get_ForwardTypeDestination<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, forwardtype: i32, ppdestaddress: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: ITForwardInformation_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITForwardInformation_Impl::get_ForwardTypeDestination(this, core::mem::transmute_copy(&forwardtype)) {
                Ok(ok__) => {
                    ppdestaddress.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn get_ForwardTypeCaller<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, forwardtype: i32, ppcalleraddress: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: ITForwardInformation_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITForwardInformation_Impl::get_ForwardTypeCaller(this, core::mem::transmute_copy(&forwardtype)) {
                Ok(ok__) => {
                    ppcalleraddress.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetForwardType<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, forwardtype: i32, ppdestinationaddress: *mut core::mem::MaybeUninit<windows_core::BSTR>, ppcalleraddress: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: ITForwardInformation_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITForwardInformation_Impl::GetForwardType(this, core::mem::transmute_copy(&forwardtype), core::mem::transmute_copy(&ppdestinationaddress), core::mem::transmute_copy(&ppcalleraddress)).into()
        }
        unsafe extern "system" fn Clear<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ITForwardInformation_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITForwardInformation_Impl::Clear(this).into()
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            SetNumRingsNoAnswer: SetNumRingsNoAnswer::<Identity, OFFSET>,
            NumRingsNoAnswer: NumRingsNoAnswer::<Identity, OFFSET>,
            SetForwardType: SetForwardType::<Identity, OFFSET>,
            get_ForwardTypeDestination: get_ForwardTypeDestination::<Identity, OFFSET>,
            get_ForwardTypeCaller: get_ForwardTypeCaller::<Identity, OFFSET>,
            GetForwardType: GetForwardType::<Identity, OFFSET>,
            Clear: Clear::<Identity, OFFSET>,
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
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ITForwardInformation2_Vtbl
    where
        Identity: ITForwardInformation2_Impl,
    {
        unsafe extern "system" fn SetForwardType2<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, forwardtype: i32, pdestaddress: core::mem::MaybeUninit<windows_core::BSTR>, destaddresstype: i32, pcalleraddress: core::mem::MaybeUninit<windows_core::BSTR>, calleraddresstype: i32) -> windows_core::HRESULT
        where
            Identity: ITForwardInformation2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITForwardInformation2_Impl::SetForwardType2(this, core::mem::transmute_copy(&forwardtype), core::mem::transmute(&pdestaddress), core::mem::transmute_copy(&destaddresstype), core::mem::transmute(&pcalleraddress), core::mem::transmute_copy(&calleraddresstype)).into()
        }
        unsafe extern "system" fn GetForwardType2<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, forwardtype: i32, ppdestinationaddress: *mut core::mem::MaybeUninit<windows_core::BSTR>, pdestaddresstype: *mut i32, ppcalleraddress: *mut core::mem::MaybeUninit<windows_core::BSTR>, pcalleraddresstype: *mut i32) -> windows_core::HRESULT
        where
            Identity: ITForwardInformation2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITForwardInformation2_Impl::GetForwardType2(this, core::mem::transmute_copy(&forwardtype), core::mem::transmute_copy(&ppdestinationaddress), core::mem::transmute_copy(&pdestaddresstype), core::mem::transmute_copy(&ppcalleraddress), core::mem::transmute_copy(&pcalleraddresstype)).into()
        }
        unsafe extern "system" fn get_ForwardTypeDestinationAddressType<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, forwardtype: i32, pdestaddresstype: *mut i32) -> windows_core::HRESULT
        where
            Identity: ITForwardInformation2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITForwardInformation2_Impl::get_ForwardTypeDestinationAddressType(this, core::mem::transmute_copy(&forwardtype)) {
                Ok(ok__) => {
                    pdestaddresstype.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn get_ForwardTypeCallerAddressType<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, forwardtype: i32, pcalleraddresstype: *mut i32) -> windows_core::HRESULT
        where
            Identity: ITForwardInformation2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITForwardInformation2_Impl::get_ForwardTypeCallerAddressType(this, core::mem::transmute_copy(&forwardtype)) {
                Ok(ok__) => {
                    pcalleraddresstype.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: ITForwardInformation_Vtbl::new::<Identity, OFFSET>(),
            SetForwardType2: SetForwardType2::<Identity, OFFSET>,
            GetForwardType2: GetForwardType2::<Identity, OFFSET>,
            get_ForwardTypeDestinationAddressType: get_ForwardTypeDestinationAddressType::<Identity, OFFSET>,
            get_ForwardTypeCallerAddressType: get_ForwardTypeCallerAddressType::<Identity, OFFSET>,
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
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ITILSConfig_Vtbl
    where
        Identity: ITILSConfig_Impl,
    {
        unsafe extern "system" fn Port<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pport: *mut i32) -> windows_core::HRESULT
        where
            Identity: ITILSConfig_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITILSConfig_Impl::Port(this) {
                Ok(ok__) => {
                    pport.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPort<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, port: i32) -> windows_core::HRESULT
        where
            Identity: ITILSConfig_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITILSConfig_Impl::SetPort(this, core::mem::transmute_copy(&port)).into()
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            Port: Port::<Identity, OFFSET>,
            SetPort: SetPort::<Identity, OFFSET>,
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
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ITLegacyAddressMediaControl_Vtbl
    where
        Identity: ITLegacyAddressMediaControl_Impl,
    {
        unsafe extern "system" fn GetID<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdeviceclass: core::mem::MaybeUninit<windows_core::BSTR>, pdwsize: *mut u32, ppdeviceid: *mut *mut u8) -> windows_core::HRESULT
        where
            Identity: ITLegacyAddressMediaControl_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITLegacyAddressMediaControl_Impl::GetID(this, core::mem::transmute(&pdeviceclass), core::mem::transmute_copy(&pdwsize), core::mem::transmute_copy(&ppdeviceid)).into()
        }
        unsafe extern "system" fn GetDevConfig<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdeviceclass: core::mem::MaybeUninit<windows_core::BSTR>, pdwsize: *mut u32, ppdeviceconfig: *mut *mut u8) -> windows_core::HRESULT
        where
            Identity: ITLegacyAddressMediaControl_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITLegacyAddressMediaControl_Impl::GetDevConfig(this, core::mem::transmute(&pdeviceclass), core::mem::transmute_copy(&pdwsize), core::mem::transmute_copy(&ppdeviceconfig)).into()
        }
        unsafe extern "system" fn SetDevConfig<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdeviceclass: core::mem::MaybeUninit<windows_core::BSTR>, dwsize: u32, pdeviceconfig: *const u8) -> windows_core::HRESULT
        where
            Identity: ITLegacyAddressMediaControl_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITLegacyAddressMediaControl_Impl::SetDevConfig(this, core::mem::transmute(&pdeviceclass), core::mem::transmute_copy(&dwsize), core::mem::transmute_copy(&pdeviceconfig)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetID: GetID::<Identity, OFFSET>,
            GetDevConfig: GetDevConfig::<Identity, OFFSET>,
            SetDevConfig: SetDevConfig::<Identity, OFFSET>,
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
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ITLegacyAddressMediaControl2_Vtbl
    where
        Identity: ITLegacyAddressMediaControl2_Impl,
    {
        unsafe extern "system" fn ConfigDialog<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, hwndowner: super::super::Foundation::HWND, pdeviceclass: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: ITLegacyAddressMediaControl2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITLegacyAddressMediaControl2_Impl::ConfigDialog(this, core::mem::transmute_copy(&hwndowner), core::mem::transmute(&pdeviceclass)).into()
        }
        unsafe extern "system" fn ConfigDialogEdit<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, hwndowner: super::super::Foundation::HWND, pdeviceclass: core::mem::MaybeUninit<windows_core::BSTR>, dwsizein: u32, pdeviceconfigin: *const u8, pdwsizeout: *mut u32, ppdeviceconfigout: *mut *mut u8) -> windows_core::HRESULT
        where
            Identity: ITLegacyAddressMediaControl2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITLegacyAddressMediaControl2_Impl::ConfigDialogEdit(this, core::mem::transmute_copy(&hwndowner), core::mem::transmute(&pdeviceclass), core::mem::transmute_copy(&dwsizein), core::mem::transmute_copy(&pdeviceconfigin), core::mem::transmute_copy(&pdwsizeout), core::mem::transmute_copy(&ppdeviceconfigout)).into()
        }
        Self {
            base__: ITLegacyAddressMediaControl_Vtbl::new::<Identity, OFFSET>(),
            ConfigDialog: ConfigDialog::<Identity, OFFSET>,
            ConfigDialogEdit: ConfigDialogEdit::<Identity, OFFSET>,
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
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ITLegacyCallMediaControl_Vtbl
    where
        Identity: ITLegacyCallMediaControl_Impl,
    {
        unsafe extern "system" fn DetectDigits<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, digitmode: i32) -> windows_core::HRESULT
        where
            Identity: ITLegacyCallMediaControl_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITLegacyCallMediaControl_Impl::DetectDigits(this, core::mem::transmute_copy(&digitmode)).into()
        }
        unsafe extern "system" fn GenerateDigits<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdigits: core::mem::MaybeUninit<windows_core::BSTR>, digitmode: i32) -> windows_core::HRESULT
        where
            Identity: ITLegacyCallMediaControl_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITLegacyCallMediaControl_Impl::GenerateDigits(this, core::mem::transmute(&pdigits), core::mem::transmute_copy(&digitmode)).into()
        }
        unsafe extern "system" fn GetID<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdeviceclass: core::mem::MaybeUninit<windows_core::BSTR>, pdwsize: *mut u32, ppdeviceid: *mut *mut u8) -> windows_core::HRESULT
        where
            Identity: ITLegacyCallMediaControl_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITLegacyCallMediaControl_Impl::GetID(this, core::mem::transmute(&pdeviceclass), core::mem::transmute_copy(&pdwsize), core::mem::transmute_copy(&ppdeviceid)).into()
        }
        unsafe extern "system" fn SetMediaType<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, lmediatype: i32) -> windows_core::HRESULT
        where
            Identity: ITLegacyCallMediaControl_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITLegacyCallMediaControl_Impl::SetMediaType(this, core::mem::transmute_copy(&lmediatype)).into()
        }
        unsafe extern "system" fn MonitorMedia<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, lmediatype: i32) -> windows_core::HRESULT
        where
            Identity: ITLegacyCallMediaControl_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITLegacyCallMediaControl_Impl::MonitorMedia(this, core::mem::transmute_copy(&lmediatype)).into()
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            DetectDigits: DetectDigits::<Identity, OFFSET>,
            GenerateDigits: GenerateDigits::<Identity, OFFSET>,
            GetID: GetID::<Identity, OFFSET>,
            SetMediaType: SetMediaType::<Identity, OFFSET>,
            MonitorMedia: MonitorMedia::<Identity, OFFSET>,
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
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ITLegacyCallMediaControl2_Vtbl
    where
        Identity: ITLegacyCallMediaControl2_Impl,
    {
        unsafe extern "system" fn GenerateDigits2<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdigits: core::mem::MaybeUninit<windows_core::BSTR>, digitmode: i32, lduration: i32) -> windows_core::HRESULT
        where
            Identity: ITLegacyCallMediaControl2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITLegacyCallMediaControl2_Impl::GenerateDigits2(this, core::mem::transmute(&pdigits), core::mem::transmute_copy(&digitmode), core::mem::transmute_copy(&lduration)).into()
        }
        unsafe extern "system" fn GatherDigits<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, digitmode: i32, lnumdigits: i32, pterminationdigits: core::mem::MaybeUninit<windows_core::BSTR>, lfirstdigittimeout: i32, linterdigittimeout: i32) -> windows_core::HRESULT
        where
            Identity: ITLegacyCallMediaControl2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITLegacyCallMediaControl2_Impl::GatherDigits(this, core::mem::transmute_copy(&digitmode), core::mem::transmute_copy(&lnumdigits), core::mem::transmute(&pterminationdigits), core::mem::transmute_copy(&lfirstdigittimeout), core::mem::transmute_copy(&linterdigittimeout)).into()
        }
        unsafe extern "system" fn DetectTones<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ptonelist: *const TAPI_DETECTTONE, lnumtones: i32) -> windows_core::HRESULT
        where
            Identity: ITLegacyCallMediaControl2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITLegacyCallMediaControl2_Impl::DetectTones(this, core::mem::transmute_copy(&ptonelist), core::mem::transmute_copy(&lnumtones)).into()
        }
        unsafe extern "system" fn DetectTonesByCollection<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdetecttonecollection: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ITLegacyCallMediaControl2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITLegacyCallMediaControl2_Impl::DetectTonesByCollection(this, windows_core::from_raw_borrowed(&pdetecttonecollection)).into()
        }
        unsafe extern "system" fn GenerateTone<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, tonemode: TAPI_TONEMODE, lduration: i32) -> windows_core::HRESULT
        where
            Identity: ITLegacyCallMediaControl2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITLegacyCallMediaControl2_Impl::GenerateTone(this, core::mem::transmute_copy(&tonemode), core::mem::transmute_copy(&lduration)).into()
        }
        unsafe extern "system" fn GenerateCustomTones<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ptonelist: *const TAPI_CUSTOMTONE, lnumtones: i32, lduration: i32) -> windows_core::HRESULT
        where
            Identity: ITLegacyCallMediaControl2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITLegacyCallMediaControl2_Impl::GenerateCustomTones(this, core::mem::transmute_copy(&ptonelist), core::mem::transmute_copy(&lnumtones), core::mem::transmute_copy(&lduration)).into()
        }
        unsafe extern "system" fn GenerateCustomTonesByCollection<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcustomtonecollection: *mut core::ffi::c_void, lduration: i32) -> windows_core::HRESULT
        where
            Identity: ITLegacyCallMediaControl2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITLegacyCallMediaControl2_Impl::GenerateCustomTonesByCollection(this, windows_core::from_raw_borrowed(&pcustomtonecollection), core::mem::transmute_copy(&lduration)).into()
        }
        unsafe extern "system" fn CreateDetectToneObject<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppdetecttone: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ITLegacyCallMediaControl2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITLegacyCallMediaControl2_Impl::CreateDetectToneObject(this) {
                Ok(ok__) => {
                    ppdetecttone.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateCustomToneObject<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppcustomtone: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ITLegacyCallMediaControl2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITLegacyCallMediaControl2_Impl::CreateCustomToneObject(this) {
                Ok(ok__) => {
                    ppcustomtone.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetIDAsVariant<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrdeviceclass: core::mem::MaybeUninit<windows_core::BSTR>, pvardeviceid: *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT
        where
            Identity: ITLegacyCallMediaControl2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITLegacyCallMediaControl2_Impl::GetIDAsVariant(this, core::mem::transmute(&bstrdeviceclass)) {
                Ok(ok__) => {
                    pvardeviceid.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: ITLegacyCallMediaControl_Vtbl::new::<Identity, OFFSET>(),
            GenerateDigits2: GenerateDigits2::<Identity, OFFSET>,
            GatherDigits: GatherDigits::<Identity, OFFSET>,
            DetectTones: DetectTones::<Identity, OFFSET>,
            DetectTonesByCollection: DetectTonesByCollection::<Identity, OFFSET>,
            GenerateTone: GenerateTone::<Identity, OFFSET>,
            GenerateCustomTones: GenerateCustomTones::<Identity, OFFSET>,
            GenerateCustomTonesByCollection: GenerateCustomTonesByCollection::<Identity, OFFSET>,
            CreateDetectToneObject: CreateDetectToneObject::<Identity, OFFSET>,
            CreateCustomToneObject: CreateCustomToneObject::<Identity, OFFSET>,
            GetIDAsVariant: GetIDAsVariant::<Identity, OFFSET>,
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
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ITLegacyWaveSupport_Vtbl
    where
        Identity: ITLegacyWaveSupport_Impl,
    {
        unsafe extern "system" fn IsFullDuplex<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, psupport: *mut FULLDUPLEX_SUPPORT) -> windows_core::HRESULT
        where
            Identity: ITLegacyWaveSupport_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITLegacyWaveSupport_Impl::IsFullDuplex(this) {
                Ok(ok__) => {
                    psupport.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self { base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(), IsFullDuplex: IsFullDuplex::<Identity, OFFSET> }
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
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ITLocationInfo_Vtbl
    where
        Identity: ITLocationInfo_Impl,
    {
        unsafe extern "system" fn PermanentLocationID<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pllocationid: *mut i32) -> windows_core::HRESULT
        where
            Identity: ITLocationInfo_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITLocationInfo_Impl::PermanentLocationID(this) {
                Ok(ok__) => {
                    pllocationid.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CountryCode<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, plcountrycode: *mut i32) -> windows_core::HRESULT
        where
            Identity: ITLocationInfo_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITLocationInfo_Impl::CountryCode(this) {
                Ok(ok__) => {
                    plcountrycode.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CountryID<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, plcountryid: *mut i32) -> windows_core::HRESULT
        where
            Identity: ITLocationInfo_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITLocationInfo_Impl::CountryID(this) {
                Ok(ok__) => {
                    plcountryid.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Options<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ploptions: *mut i32) -> windows_core::HRESULT
        where
            Identity: ITLocationInfo_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITLocationInfo_Impl::Options(this) {
                Ok(ok__) => {
                    ploptions.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PreferredCardID<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, plcardid: *mut i32) -> windows_core::HRESULT
        where
            Identity: ITLocationInfo_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITLocationInfo_Impl::PreferredCardID(this) {
                Ok(ok__) => {
                    plcardid.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LocationName<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pplocationname: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: ITLocationInfo_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITLocationInfo_Impl::LocationName(this) {
                Ok(ok__) => {
                    pplocationname.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CityCode<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppcode: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: ITLocationInfo_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITLocationInfo_Impl::CityCode(this) {
                Ok(ok__) => {
                    ppcode.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LocalAccessCode<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppcode: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: ITLocationInfo_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITLocationInfo_Impl::LocalAccessCode(this) {
                Ok(ok__) => {
                    ppcode.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LongDistanceAccessCode<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppcode: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: ITLocationInfo_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITLocationInfo_Impl::LongDistanceAccessCode(this) {
                Ok(ok__) => {
                    ppcode.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TollPrefixList<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pptolllist: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: ITLocationInfo_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITLocationInfo_Impl::TollPrefixList(this) {
                Ok(ok__) => {
                    pptolllist.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CancelCallWaitingCode<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppcode: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: ITLocationInfo_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITLocationInfo_Impl::CancelCallWaitingCode(this) {
                Ok(ok__) => {
                    ppcode.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            PermanentLocationID: PermanentLocationID::<Identity, OFFSET>,
            CountryCode: CountryCode::<Identity, OFFSET>,
            CountryID: CountryID::<Identity, OFFSET>,
            Options: Options::<Identity, OFFSET>,
            PreferredCardID: PreferredCardID::<Identity, OFFSET>,
            LocationName: LocationName::<Identity, OFFSET>,
            CityCode: CityCode::<Identity, OFFSET>,
            LocalAccessCode: LocalAccessCode::<Identity, OFFSET>,
            LongDistanceAccessCode: LongDistanceAccessCode::<Identity, OFFSET>,
            TollPrefixList: TollPrefixList::<Identity, OFFSET>,
            CancelCallWaitingCode: CancelCallWaitingCode::<Identity, OFFSET>,
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
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ITMSPAddress_Vtbl
    where
        Identity: ITMSPAddress_Impl,
    {
        unsafe extern "system" fn Initialize<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, hevent: *const i32) -> windows_core::HRESULT
        where
            Identity: ITMSPAddress_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITMSPAddress_Impl::Initialize(this, core::mem::transmute_copy(&hevent)).into()
        }
        unsafe extern "system" fn Shutdown<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ITMSPAddress_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITMSPAddress_Impl::Shutdown(this).into()
        }
        unsafe extern "system" fn CreateMSPCall<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, hcall: *const i32, dwreserved: u32, dwmediatype: u32, pouterunknown: *mut core::ffi::c_void, ppstreamcontrol: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ITMSPAddress_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITMSPAddress_Impl::CreateMSPCall(this, core::mem::transmute_copy(&hcall), core::mem::transmute_copy(&dwreserved), core::mem::transmute_copy(&dwmediatype), windows_core::from_raw_borrowed(&pouterunknown)) {
                Ok(ok__) => {
                    ppstreamcontrol.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ShutdownMSPCall<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pstreamcontrol: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ITMSPAddress_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITMSPAddress_Impl::ShutdownMSPCall(this, windows_core::from_raw_borrowed(&pstreamcontrol)).into()
        }
        unsafe extern "system" fn ReceiveTSPData<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pmspcall: *mut core::ffi::c_void, pbuffer: *const u8, dwsize: u32) -> windows_core::HRESULT
        where
            Identity: ITMSPAddress_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITMSPAddress_Impl::ReceiveTSPData(this, windows_core::from_raw_borrowed(&pmspcall), core::mem::transmute_copy(&pbuffer), core::mem::transmute_copy(&dwsize)).into()
        }
        unsafe extern "system" fn GetEvent<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwsize: *mut u32, peventbuffer: *mut u8) -> windows_core::HRESULT
        where
            Identity: ITMSPAddress_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITMSPAddress_Impl::GetEvent(this, core::mem::transmute_copy(&pdwsize), core::mem::transmute_copy(&peventbuffer)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Initialize: Initialize::<Identity, OFFSET>,
            Shutdown: Shutdown::<Identity, OFFSET>,
            CreateMSPCall: CreateMSPCall::<Identity, OFFSET>,
            ShutdownMSPCall: ShutdownMSPCall::<Identity, OFFSET>,
            ReceiveTSPData: ReceiveTSPData::<Identity, OFFSET>,
            GetEvent: GetEvent::<Identity, OFFSET>,
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
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ITMediaControl_Vtbl
    where
        Identity: ITMediaControl_Impl,
    {
        unsafe extern "system" fn Start<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ITMediaControl_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITMediaControl_Impl::Start(this).into()
        }
        unsafe extern "system" fn Stop<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ITMediaControl_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITMediaControl_Impl::Stop(this).into()
        }
        unsafe extern "system" fn Pause<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ITMediaControl_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITMediaControl_Impl::Pause(this).into()
        }
        unsafe extern "system" fn MediaState<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pterminalmediastate: *mut TERMINAL_MEDIA_STATE) -> windows_core::HRESULT
        where
            Identity: ITMediaControl_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITMediaControl_Impl::MediaState(this) {
                Ok(ok__) => {
                    pterminalmediastate.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            Start: Start::<Identity, OFFSET>,
            Stop: Stop::<Identity, OFFSET>,
            Pause: Pause::<Identity, OFFSET>,
            MediaState: MediaState::<Identity, OFFSET>,
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
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ITMediaPlayback_Vtbl
    where
        Identity: ITMediaPlayback_Impl,
    {
        unsafe extern "system" fn SetPlayList<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, playlistvariant: core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT
        where
            Identity: ITMediaPlayback_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITMediaPlayback_Impl::SetPlayList(this, core::mem::transmute(&playlistvariant)).into()
        }
        unsafe extern "system" fn PlayList<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pplaylistvariant: *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT
        where
            Identity: ITMediaPlayback_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITMediaPlayback_Impl::PlayList(this) {
                Ok(ok__) => {
                    pplaylistvariant.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            SetPlayList: SetPlayList::<Identity, OFFSET>,
            PlayList: PlayList::<Identity, OFFSET>,
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
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ITMediaRecord_Vtbl
    where
        Identity: ITMediaRecord_Impl,
    {
        unsafe extern "system" fn SetFileName<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrfilename: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: ITMediaRecord_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITMediaRecord_Impl::SetFileName(this, core::mem::transmute(&bstrfilename)).into()
        }
        unsafe extern "system" fn FileName<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrfilename: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: ITMediaRecord_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITMediaRecord_Impl::FileName(this) {
                Ok(ok__) => {
                    pbstrfilename.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            SetFileName: SetFileName::<Identity, OFFSET>,
            FileName: FileName::<Identity, OFFSET>,
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
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ITMediaSupport_Vtbl
    where
        Identity: ITMediaSupport_Impl,
    {
        unsafe extern "system" fn MediaTypes<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, plmediatypes: *mut i32) -> windows_core::HRESULT
        where
            Identity: ITMediaSupport_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITMediaSupport_Impl::MediaTypes(this) {
                Ok(ok__) => {
                    plmediatypes.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn QueryMediaType<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, lmediatype: i32, pfsupport: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT
        where
            Identity: ITMediaSupport_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITMediaSupport_Impl::QueryMediaType(this, core::mem::transmute_copy(&lmediatype)) {
                Ok(ok__) => {
                    pfsupport.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            MediaTypes: MediaTypes::<Identity, OFFSET>,
            QueryMediaType: QueryMediaType::<Identity, OFFSET>,
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
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ITMultiTrackTerminal_Vtbl
    where
        Identity: ITMultiTrackTerminal_Impl,
    {
        unsafe extern "system" fn TrackTerminals<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvariant: *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT
        where
            Identity: ITMultiTrackTerminal_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITMultiTrackTerminal_Impl::TrackTerminals(this) {
                Ok(ok__) => {
                    pvariant.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnumerateTrackTerminals<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppenumterminal: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ITMultiTrackTerminal_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITMultiTrackTerminal_Impl::EnumerateTrackTerminals(this) {
                Ok(ok__) => {
                    ppenumterminal.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateTrackTerminal<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, mediatype: i32, terminaldirection: TERMINAL_DIRECTION, ppterminal: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ITMultiTrackTerminal_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITMultiTrackTerminal_Impl::CreateTrackTerminal(this, core::mem::transmute_copy(&mediatype), core::mem::transmute_copy(&terminaldirection)) {
                Ok(ok__) => {
                    ppterminal.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MediaTypesInUse<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, plmediatypesinuse: *mut i32) -> windows_core::HRESULT
        where
            Identity: ITMultiTrackTerminal_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITMultiTrackTerminal_Impl::MediaTypesInUse(this) {
                Ok(ok__) => {
                    plmediatypesinuse.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DirectionsInUse<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pldirectionsinused: *mut TERMINAL_DIRECTION) -> windows_core::HRESULT
        where
            Identity: ITMultiTrackTerminal_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITMultiTrackTerminal_Impl::DirectionsInUse(this) {
                Ok(ok__) => {
                    pldirectionsinused.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveTrackTerminal<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ptrackterminaltoremove: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ITMultiTrackTerminal_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITMultiTrackTerminal_Impl::RemoveTrackTerminal(this, windows_core::from_raw_borrowed(&ptrackterminaltoremove)).into()
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            TrackTerminals: TrackTerminals::<Identity, OFFSET>,
            EnumerateTrackTerminals: EnumerateTrackTerminals::<Identity, OFFSET>,
            CreateTrackTerminal: CreateTrackTerminal::<Identity, OFFSET>,
            MediaTypesInUse: MediaTypesInUse::<Identity, OFFSET>,
            DirectionsInUse: DirectionsInUse::<Identity, OFFSET>,
            RemoveTrackTerminal: RemoveTrackTerminal::<Identity, OFFSET>,
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
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ITPhone_Vtbl
    where
        Identity: ITPhone_Impl,
    {
        unsafe extern "system" fn Open<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, privilege: PHONE_PRIVILEGE) -> windows_core::HRESULT
        where
            Identity: ITPhone_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITPhone_Impl::Open(this, core::mem::transmute_copy(&privilege)).into()
        }
        unsafe extern "system" fn Close<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ITPhone_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITPhone_Impl::Close(this).into()
        }
        unsafe extern "system" fn Addresses<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, paddresses: *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT
        where
            Identity: ITPhone_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITPhone_Impl::Addresses(this) {
                Ok(ok__) => {
                    paddresses.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnumerateAddresses<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppenumaddress: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ITPhone_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITPhone_Impl::EnumerateAddresses(this) {
                Ok(ok__) => {
                    ppenumaddress.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn get_PhoneCapsLong<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pclcap: PHONECAPS_LONG, plcapability: *mut i32) -> windows_core::HRESULT
        where
            Identity: ITPhone_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITPhone_Impl::get_PhoneCapsLong(this, core::mem::transmute_copy(&pclcap)) {
                Ok(ok__) => {
                    plcapability.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn get_PhoneCapsString<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcscap: PHONECAPS_STRING, ppcapability: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: ITPhone_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITPhone_Impl::get_PhoneCapsString(this, core::mem::transmute_copy(&pcscap)) {
                Ok(ok__) => {
                    ppcapability.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn get_Terminals<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, paddress: *mut core::ffi::c_void, pterminals: *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT
        where
            Identity: ITPhone_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITPhone_Impl::get_Terminals(this, windows_core::from_raw_borrowed(&paddress)) {
                Ok(ok__) => {
                    pterminals.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnumerateTerminals<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, paddress: *mut core::ffi::c_void, ppenumterminal: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ITPhone_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITPhone_Impl::EnumerateTerminals(this, windows_core::from_raw_borrowed(&paddress)) {
                Ok(ok__) => {
                    ppenumterminal.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn get_ButtonMode<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, lbuttonid: i32, pbuttonmode: *mut PHONE_BUTTON_MODE) -> windows_core::HRESULT
        where
            Identity: ITPhone_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITPhone_Impl::get_ButtonMode(this, core::mem::transmute_copy(&lbuttonid)) {
                Ok(ok__) => {
                    pbuttonmode.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn put_ButtonMode<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, lbuttonid: i32, buttonmode: PHONE_BUTTON_MODE) -> windows_core::HRESULT
        where
            Identity: ITPhone_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITPhone_Impl::put_ButtonMode(this, core::mem::transmute_copy(&lbuttonid), core::mem::transmute_copy(&buttonmode)).into()
        }
        unsafe extern "system" fn get_ButtonFunction<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, lbuttonid: i32, pbuttonfunction: *mut PHONE_BUTTON_FUNCTION) -> windows_core::HRESULT
        where
            Identity: ITPhone_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITPhone_Impl::get_ButtonFunction(this, core::mem::transmute_copy(&lbuttonid)) {
                Ok(ok__) => {
                    pbuttonfunction.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn put_ButtonFunction<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, lbuttonid: i32, buttonfunction: PHONE_BUTTON_FUNCTION) -> windows_core::HRESULT
        where
            Identity: ITPhone_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITPhone_Impl::put_ButtonFunction(this, core::mem::transmute_copy(&lbuttonid), core::mem::transmute_copy(&buttonfunction)).into()
        }
        unsafe extern "system" fn get_ButtonText<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, lbuttonid: i32, ppbuttontext: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: ITPhone_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITPhone_Impl::get_ButtonText(this, core::mem::transmute_copy(&lbuttonid)) {
                Ok(ok__) => {
                    ppbuttontext.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn put_ButtonText<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, lbuttonid: i32, bstrbuttontext: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: ITPhone_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITPhone_Impl::put_ButtonText(this, core::mem::transmute_copy(&lbuttonid), core::mem::transmute(&bstrbuttontext)).into()
        }
        unsafe extern "system" fn get_ButtonState<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, lbuttonid: i32, pbuttonstate: *mut PHONE_BUTTON_STATE) -> windows_core::HRESULT
        where
            Identity: ITPhone_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITPhone_Impl::get_ButtonState(this, core::mem::transmute_copy(&lbuttonid)) {
                Ok(ok__) => {
                    pbuttonstate.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn get_HookSwitchState<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, hookswitchdevice: PHONE_HOOK_SWITCH_DEVICE, phookswitchstate: *mut PHONE_HOOK_SWITCH_STATE) -> windows_core::HRESULT
        where
            Identity: ITPhone_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITPhone_Impl::get_HookSwitchState(this, core::mem::transmute_copy(&hookswitchdevice)) {
                Ok(ok__) => {
                    phookswitchstate.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn put_HookSwitchState<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, hookswitchdevice: PHONE_HOOK_SWITCH_DEVICE, hookswitchstate: PHONE_HOOK_SWITCH_STATE) -> windows_core::HRESULT
        where
            Identity: ITPhone_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITPhone_Impl::put_HookSwitchState(this, core::mem::transmute_copy(&hookswitchdevice), core::mem::transmute_copy(&hookswitchstate)).into()
        }
        unsafe extern "system" fn SetRingMode<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, lringmode: i32) -> windows_core::HRESULT
        where
            Identity: ITPhone_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITPhone_Impl::SetRingMode(this, core::mem::transmute_copy(&lringmode)).into()
        }
        unsafe extern "system" fn RingMode<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, plringmode: *mut i32) -> windows_core::HRESULT
        where
            Identity: ITPhone_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITPhone_Impl::RingMode(this) {
                Ok(ok__) => {
                    plringmode.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetRingVolume<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, lringvolume: i32) -> windows_core::HRESULT
        where
            Identity: ITPhone_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITPhone_Impl::SetRingVolume(this, core::mem::transmute_copy(&lringvolume)).into()
        }
        unsafe extern "system" fn RingVolume<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, plringvolume: *mut i32) -> windows_core::HRESULT
        where
            Identity: ITPhone_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITPhone_Impl::RingVolume(this) {
                Ok(ok__) => {
                    plringvolume.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Privilege<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pprivilege: *mut PHONE_PRIVILEGE) -> windows_core::HRESULT
        where
            Identity: ITPhone_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITPhone_Impl::Privilege(this) {
                Ok(ok__) => {
                    pprivilege.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPhoneCapsBuffer<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcbcaps: PHONECAPS_BUFFER, pdwsize: *mut u32, ppphonecapsbuffer: *mut *mut u8) -> windows_core::HRESULT
        where
            Identity: ITPhone_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITPhone_Impl::GetPhoneCapsBuffer(this, core::mem::transmute_copy(&pcbcaps), core::mem::transmute_copy(&pdwsize), core::mem::transmute_copy(&ppphonecapsbuffer)).into()
        }
        unsafe extern "system" fn get_PhoneCapsBuffer<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcbcaps: PHONECAPS_BUFFER, pvarbuffer: *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT
        where
            Identity: ITPhone_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITPhone_Impl::get_PhoneCapsBuffer(this, core::mem::transmute_copy(&pcbcaps)) {
                Ok(ok__) => {
                    pvarbuffer.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn get_LampMode<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, llampid: i32, plampmode: *mut PHONE_LAMP_MODE) -> windows_core::HRESULT
        where
            Identity: ITPhone_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITPhone_Impl::get_LampMode(this, core::mem::transmute_copy(&llampid)) {
                Ok(ok__) => {
                    plampmode.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn put_LampMode<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, llampid: i32, lampmode: PHONE_LAMP_MODE) -> windows_core::HRESULT
        where
            Identity: ITPhone_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITPhone_Impl::put_LampMode(this, core::mem::transmute_copy(&llampid), core::mem::transmute_copy(&lampmode)).into()
        }
        unsafe extern "system" fn Display<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrdisplay: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: ITPhone_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITPhone_Impl::Display(this) {
                Ok(ok__) => {
                    pbstrdisplay.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDisplay<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, lrow: i32, lcolumn: i32, bstrdisplay: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: ITPhone_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITPhone_Impl::SetDisplay(this, core::mem::transmute_copy(&lrow), core::mem::transmute_copy(&lcolumn), core::mem::transmute(&bstrdisplay)).into()
        }
        unsafe extern "system" fn PreferredAddresses<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, paddresses: *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT
        where
            Identity: ITPhone_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITPhone_Impl::PreferredAddresses(this) {
                Ok(ok__) => {
                    paddresses.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnumeratePreferredAddresses<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppenumaddress: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ITPhone_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITPhone_Impl::EnumeratePreferredAddresses(this) {
                Ok(ok__) => {
                    ppenumaddress.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DeviceSpecific<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pparams: *const u8, dwsize: u32) -> windows_core::HRESULT
        where
            Identity: ITPhone_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITPhone_Impl::DeviceSpecific(this, core::mem::transmute_copy(&pparams), core::mem::transmute_copy(&dwsize)).into()
        }
        unsafe extern "system" fn DeviceSpecificVariant<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, vardevspecificbytearray: core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT
        where
            Identity: ITPhone_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITPhone_Impl::DeviceSpecificVariant(this, core::mem::transmute(&vardevspecificbytearray)).into()
        }
        unsafe extern "system" fn NegotiateExtVersion<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, llowversion: i32, lhighversion: i32, plextversion: *mut i32) -> windows_core::HRESULT
        where
            Identity: ITPhone_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITPhone_Impl::NegotiateExtVersion(this, core::mem::transmute_copy(&llowversion), core::mem::transmute_copy(&lhighversion)) {
                Ok(ok__) => {
                    plextversion.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            Open: Open::<Identity, OFFSET>,
            Close: Close::<Identity, OFFSET>,
            Addresses: Addresses::<Identity, OFFSET>,
            EnumerateAddresses: EnumerateAddresses::<Identity, OFFSET>,
            get_PhoneCapsLong: get_PhoneCapsLong::<Identity, OFFSET>,
            get_PhoneCapsString: get_PhoneCapsString::<Identity, OFFSET>,
            get_Terminals: get_Terminals::<Identity, OFFSET>,
            EnumerateTerminals: EnumerateTerminals::<Identity, OFFSET>,
            get_ButtonMode: get_ButtonMode::<Identity, OFFSET>,
            put_ButtonMode: put_ButtonMode::<Identity, OFFSET>,
            get_ButtonFunction: get_ButtonFunction::<Identity, OFFSET>,
            put_ButtonFunction: put_ButtonFunction::<Identity, OFFSET>,
            get_ButtonText: get_ButtonText::<Identity, OFFSET>,
            put_ButtonText: put_ButtonText::<Identity, OFFSET>,
            get_ButtonState: get_ButtonState::<Identity, OFFSET>,
            get_HookSwitchState: get_HookSwitchState::<Identity, OFFSET>,
            put_HookSwitchState: put_HookSwitchState::<Identity, OFFSET>,
            SetRingMode: SetRingMode::<Identity, OFFSET>,
            RingMode: RingMode::<Identity, OFFSET>,
            SetRingVolume: SetRingVolume::<Identity, OFFSET>,
            RingVolume: RingVolume::<Identity, OFFSET>,
            Privilege: Privilege::<Identity, OFFSET>,
            GetPhoneCapsBuffer: GetPhoneCapsBuffer::<Identity, OFFSET>,
            get_PhoneCapsBuffer: get_PhoneCapsBuffer::<Identity, OFFSET>,
            get_LampMode: get_LampMode::<Identity, OFFSET>,
            put_LampMode: put_LampMode::<Identity, OFFSET>,
            Display: Display::<Identity, OFFSET>,
            SetDisplay: SetDisplay::<Identity, OFFSET>,
            PreferredAddresses: PreferredAddresses::<Identity, OFFSET>,
            EnumeratePreferredAddresses: EnumeratePreferredAddresses::<Identity, OFFSET>,
            DeviceSpecific: DeviceSpecific::<Identity, OFFSET>,
            DeviceSpecificVariant: DeviceSpecificVariant::<Identity, OFFSET>,
            NegotiateExtVersion: NegotiateExtVersion::<Identity, OFFSET>,
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
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ITPhoneDeviceSpecificEvent_Vtbl
    where
        Identity: ITPhoneDeviceSpecificEvent_Impl,
    {
        unsafe extern "system" fn Phone<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppphone: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ITPhoneDeviceSpecificEvent_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITPhoneDeviceSpecificEvent_Impl::Phone(this) {
                Ok(ok__) => {
                    ppphone.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn lParam1<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pparam1: *mut i32) -> windows_core::HRESULT
        where
            Identity: ITPhoneDeviceSpecificEvent_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITPhoneDeviceSpecificEvent_Impl::lParam1(this) {
                Ok(ok__) => {
                    pparam1.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn lParam2<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pparam2: *mut i32) -> windows_core::HRESULT
        where
            Identity: ITPhoneDeviceSpecificEvent_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITPhoneDeviceSpecificEvent_Impl::lParam2(this) {
                Ok(ok__) => {
                    pparam2.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn lParam3<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pparam3: *mut i32) -> windows_core::HRESULT
        where
            Identity: ITPhoneDeviceSpecificEvent_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITPhoneDeviceSpecificEvent_Impl::lParam3(this) {
                Ok(ok__) => {
                    pparam3.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            Phone: Phone::<Identity, OFFSET>,
            lParam1: lParam1::<Identity, OFFSET>,
            lParam2: lParam2::<Identity, OFFSET>,
            lParam3: lParam3::<Identity, OFFSET>,
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
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ITPhoneEvent_Vtbl
    where
        Identity: ITPhoneEvent_Impl,
    {
        unsafe extern "system" fn Phone<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppphone: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ITPhoneEvent_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITPhoneEvent_Impl::Phone(this) {
                Ok(ok__) => {
                    ppphone.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Event<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pevent: *mut PHONE_EVENT) -> windows_core::HRESULT
        where
            Identity: ITPhoneEvent_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITPhoneEvent_Impl::Event(this) {
                Ok(ok__) => {
                    pevent.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ButtonState<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pstate: *mut PHONE_BUTTON_STATE) -> windows_core::HRESULT
        where
            Identity: ITPhoneEvent_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITPhoneEvent_Impl::ButtonState(this) {
                Ok(ok__) => {
                    pstate.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn HookSwitchState<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pstate: *mut PHONE_HOOK_SWITCH_STATE) -> windows_core::HRESULT
        where
            Identity: ITPhoneEvent_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITPhoneEvent_Impl::HookSwitchState(this) {
                Ok(ok__) => {
                    pstate.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn HookSwitchDevice<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdevice: *mut PHONE_HOOK_SWITCH_DEVICE) -> windows_core::HRESULT
        where
            Identity: ITPhoneEvent_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITPhoneEvent_Impl::HookSwitchDevice(this) {
                Ok(ok__) => {
                    pdevice.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RingMode<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, plringmode: *mut i32) -> windows_core::HRESULT
        where
            Identity: ITPhoneEvent_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITPhoneEvent_Impl::RingMode(this) {
                Ok(ok__) => {
                    plringmode.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ButtonLampId<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, plbuttonlampid: *mut i32) -> windows_core::HRESULT
        where
            Identity: ITPhoneEvent_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITPhoneEvent_Impl::ButtonLampId(this) {
                Ok(ok__) => {
                    plbuttonlampid.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn NumberGathered<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppnumber: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: ITPhoneEvent_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITPhoneEvent_Impl::NumberGathered(this) {
                Ok(ok__) => {
                    ppnumber.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Call<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppcallinfo: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ITPhoneEvent_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITPhoneEvent_Impl::Call(this) {
                Ok(ok__) => {
                    ppcallinfo.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            Phone: Phone::<Identity, OFFSET>,
            Event: Event::<Identity, OFFSET>,
            ButtonState: ButtonState::<Identity, OFFSET>,
            HookSwitchState: HookSwitchState::<Identity, OFFSET>,
            HookSwitchDevice: HookSwitchDevice::<Identity, OFFSET>,
            RingMode: RingMode::<Identity, OFFSET>,
            ButtonLampId: ButtonLampId::<Identity, OFFSET>,
            NumberGathered: NumberGathered::<Identity, OFFSET>,
            Call: Call::<Identity, OFFSET>,
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
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ITPluggableTerminalClassInfo_Vtbl
    where
        Identity: ITPluggableTerminalClassInfo_Impl,
    {
        unsafe extern "system" fn Name<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pname: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: ITPluggableTerminalClassInfo_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITPluggableTerminalClassInfo_Impl::Name(this) {
                Ok(ok__) => {
                    pname.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Company<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcompany: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: ITPluggableTerminalClassInfo_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITPluggableTerminalClassInfo_Impl::Company(this) {
                Ok(ok__) => {
                    pcompany.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Version<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pversion: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: ITPluggableTerminalClassInfo_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITPluggableTerminalClassInfo_Impl::Version(this) {
                Ok(ok__) => {
                    pversion.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TerminalClass<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pterminalclass: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: ITPluggableTerminalClassInfo_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITPluggableTerminalClassInfo_Impl::TerminalClass(this) {
                Ok(ok__) => {
                    pterminalclass.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CLSID<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pclsid: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: ITPluggableTerminalClassInfo_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITPluggableTerminalClassInfo_Impl::CLSID(this) {
                Ok(ok__) => {
                    pclsid.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Direction<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdirection: *mut TERMINAL_DIRECTION) -> windows_core::HRESULT
        where
            Identity: ITPluggableTerminalClassInfo_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITPluggableTerminalClassInfo_Impl::Direction(this) {
                Ok(ok__) => {
                    pdirection.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MediaTypes<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pmediatypes: *mut i32) -> windows_core::HRESULT
        where
            Identity: ITPluggableTerminalClassInfo_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITPluggableTerminalClassInfo_Impl::MediaTypes(this) {
                Ok(ok__) => {
                    pmediatypes.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            Name: Name::<Identity, OFFSET>,
            Company: Company::<Identity, OFFSET>,
            Version: Version::<Identity, OFFSET>,
            TerminalClass: TerminalClass::<Identity, OFFSET>,
            CLSID: CLSID::<Identity, OFFSET>,
            Direction: Direction::<Identity, OFFSET>,
            MediaTypes: MediaTypes::<Identity, OFFSET>,
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
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ITPluggableTerminalEventSink_Vtbl
    where
        Identity: ITPluggableTerminalEventSink_Impl,
    {
        unsafe extern "system" fn FireEvent<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pmspeventinfo: *const MSP_EVENT_INFO) -> windows_core::HRESULT
        where
            Identity: ITPluggableTerminalEventSink_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITPluggableTerminalEventSink_Impl::FireEvent(this, core::mem::transmute_copy(&pmspeventinfo)).into()
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), FireEvent: FireEvent::<Identity, OFFSET> }
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
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ITPluggableTerminalEventSinkRegistration_Vtbl
    where
        Identity: ITPluggableTerminalEventSinkRegistration_Impl,
    {
        unsafe extern "system" fn RegisterSink<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, peventsink: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ITPluggableTerminalEventSinkRegistration_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITPluggableTerminalEventSinkRegistration_Impl::RegisterSink(this, windows_core::from_raw_borrowed(&peventsink)).into()
        }
        unsafe extern "system" fn UnregisterSink<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ITPluggableTerminalEventSinkRegistration_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITPluggableTerminalEventSinkRegistration_Impl::UnregisterSink(this).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            RegisterSink: RegisterSink::<Identity, OFFSET>,
            UnregisterSink: UnregisterSink::<Identity, OFFSET>,
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
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ITPluggableTerminalSuperclassInfo_Vtbl
    where
        Identity: ITPluggableTerminalSuperclassInfo_Impl,
    {
        unsafe extern "system" fn Name<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pname: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: ITPluggableTerminalSuperclassInfo_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITPluggableTerminalSuperclassInfo_Impl::Name(this) {
                Ok(ok__) => {
                    pname.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CLSID<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pclsid: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: ITPluggableTerminalSuperclassInfo_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITPluggableTerminalSuperclassInfo_Impl::CLSID(this) {
                Ok(ok__) => {
                    pclsid.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self { base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(), Name: Name::<Identity, OFFSET>, CLSID: CLSID::<Identity, OFFSET> }
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
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ITPrivateEvent_Vtbl
    where
        Identity: ITPrivateEvent_Impl,
    {
        unsafe extern "system" fn Address<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppaddress: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ITPrivateEvent_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITPrivateEvent_Impl::Address(this) {
                Ok(ok__) => {
                    ppaddress.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Call<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppcallinfo: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ITPrivateEvent_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITPrivateEvent_Impl::Call(this) {
                Ok(ok__) => {
                    ppcallinfo.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CallHub<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppcallhub: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ITPrivateEvent_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITPrivateEvent_Impl::CallHub(this) {
                Ok(ok__) => {
                    ppcallhub.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EventCode<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pleventcode: *mut i32) -> windows_core::HRESULT
        where
            Identity: ITPrivateEvent_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITPrivateEvent_Impl::EventCode(this) {
                Ok(ok__) => {
                    pleventcode.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EventInterface<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, peventinterface: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ITPrivateEvent_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITPrivateEvent_Impl::EventInterface(this) {
                Ok(ok__) => {
                    peventinterface.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            Address: Address::<Identity, OFFSET>,
            Call: Call::<Identity, OFFSET>,
            CallHub: CallHub::<Identity, OFFSET>,
            EventCode: EventCode::<Identity, OFFSET>,
            EventInterface: EventInterface::<Identity, OFFSET>,
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
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ITQOSEvent_Vtbl
    where
        Identity: ITQOSEvent_Impl,
    {
        unsafe extern "system" fn Call<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppcall: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ITQOSEvent_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITQOSEvent_Impl::Call(this) {
                Ok(ok__) => {
                    ppcall.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Event<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pqosevent: *mut QOS_EVENT) -> windows_core::HRESULT
        where
            Identity: ITQOSEvent_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITQOSEvent_Impl::Event(this) {
                Ok(ok__) => {
                    pqosevent.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MediaType<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, plmediatype: *mut i32) -> windows_core::HRESULT
        where
            Identity: ITQOSEvent_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITQOSEvent_Impl::MediaType(this) {
                Ok(ok__) => {
                    plmediatype.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            Call: Call::<Identity, OFFSET>,
            Event: Event::<Identity, OFFSET>,
            MediaType: MediaType::<Identity, OFFSET>,
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
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ITQueue_Vtbl
    where
        Identity: ITQueue_Impl,
    {
        unsafe extern "system" fn SetMeasurementPeriod<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, lperiod: i32) -> windows_core::HRESULT
        where
            Identity: ITQueue_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITQueue_Impl::SetMeasurementPeriod(this, core::mem::transmute_copy(&lperiod)).into()
        }
        unsafe extern "system" fn MeasurementPeriod<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, plperiod: *mut i32) -> windows_core::HRESULT
        where
            Identity: ITQueue_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITQueue_Impl::MeasurementPeriod(this) {
                Ok(ok__) => {
                    plperiod.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TotalCallsQueued<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, plcalls: *mut i32) -> windows_core::HRESULT
        where
            Identity: ITQueue_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITQueue_Impl::TotalCallsQueued(this) {
                Ok(ok__) => {
                    plcalls.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CurrentCallsQueued<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, plcalls: *mut i32) -> windows_core::HRESULT
        where
            Identity: ITQueue_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITQueue_Impl::CurrentCallsQueued(this) {
                Ok(ok__) => {
                    plcalls.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TotalCallsAbandoned<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, plcalls: *mut i32) -> windows_core::HRESULT
        where
            Identity: ITQueue_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITQueue_Impl::TotalCallsAbandoned(this) {
                Ok(ok__) => {
                    plcalls.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TotalCallsFlowedIn<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, plcalls: *mut i32) -> windows_core::HRESULT
        where
            Identity: ITQueue_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITQueue_Impl::TotalCallsFlowedIn(this) {
                Ok(ok__) => {
                    plcalls.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TotalCallsFlowedOut<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, plcalls: *mut i32) -> windows_core::HRESULT
        where
            Identity: ITQueue_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITQueue_Impl::TotalCallsFlowedOut(this) {
                Ok(ok__) => {
                    plcalls.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LongestEverWaitTime<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, plwaittime: *mut i32) -> windows_core::HRESULT
        where
            Identity: ITQueue_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITQueue_Impl::LongestEverWaitTime(this) {
                Ok(ok__) => {
                    plwaittime.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CurrentLongestWaitTime<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, plwaittime: *mut i32) -> windows_core::HRESULT
        where
            Identity: ITQueue_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITQueue_Impl::CurrentLongestWaitTime(this) {
                Ok(ok__) => {
                    plwaittime.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AverageWaitTime<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, plwaittime: *mut i32) -> windows_core::HRESULT
        where
            Identity: ITQueue_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITQueue_Impl::AverageWaitTime(this) {
                Ok(ok__) => {
                    plwaittime.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FinalDisposition<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, plcalls: *mut i32) -> windows_core::HRESULT
        where
            Identity: ITQueue_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITQueue_Impl::FinalDisposition(this) {
                Ok(ok__) => {
                    plcalls.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Name<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppname: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: ITQueue_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITQueue_Impl::Name(this) {
                Ok(ok__) => {
                    ppname.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            SetMeasurementPeriod: SetMeasurementPeriod::<Identity, OFFSET>,
            MeasurementPeriod: MeasurementPeriod::<Identity, OFFSET>,
            TotalCallsQueued: TotalCallsQueued::<Identity, OFFSET>,
            CurrentCallsQueued: CurrentCallsQueued::<Identity, OFFSET>,
            TotalCallsAbandoned: TotalCallsAbandoned::<Identity, OFFSET>,
            TotalCallsFlowedIn: TotalCallsFlowedIn::<Identity, OFFSET>,
            TotalCallsFlowedOut: TotalCallsFlowedOut::<Identity, OFFSET>,
            LongestEverWaitTime: LongestEverWaitTime::<Identity, OFFSET>,
            CurrentLongestWaitTime: CurrentLongestWaitTime::<Identity, OFFSET>,
            AverageWaitTime: AverageWaitTime::<Identity, OFFSET>,
            FinalDisposition: FinalDisposition::<Identity, OFFSET>,
            Name: Name::<Identity, OFFSET>,
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
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ITQueueEvent_Vtbl
    where
        Identity: ITQueueEvent_Impl,
    {
        unsafe extern "system" fn Queue<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppqueue: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ITQueueEvent_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITQueueEvent_Impl::Queue(this) {
                Ok(ok__) => {
                    ppqueue.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Event<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pevent: *mut ACDQUEUE_EVENT) -> windows_core::HRESULT
        where
            Identity: ITQueueEvent_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITQueueEvent_Impl::Event(this) {
                Ok(ok__) => {
                    pevent.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self { base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(), Queue: Queue::<Identity, OFFSET>, Event: Event::<Identity, OFFSET> }
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
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ITRendezvous_Vtbl
    where
        Identity: ITRendezvous_Impl,
    {
        unsafe extern "system" fn DefaultDirectories<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvariant: *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT
        where
            Identity: ITRendezvous_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITRendezvous_Impl::DefaultDirectories(this) {
                Ok(ok__) => {
                    pvariant.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnumerateDefaultDirectories<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppenumdirectory: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ITRendezvous_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITRendezvous_Impl::EnumerateDefaultDirectories(this) {
                Ok(ok__) => {
                    ppenumdirectory.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateDirectory<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, directorytype: DIRECTORY_TYPE, pname: core::mem::MaybeUninit<windows_core::BSTR>, ppdir: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ITRendezvous_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITRendezvous_Impl::CreateDirectory(this, core::mem::transmute_copy(&directorytype), core::mem::transmute(&pname)) {
                Ok(ok__) => {
                    ppdir.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateDirectoryObject<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, directoryobjecttype: DIRECTORY_OBJECT_TYPE, pname: core::mem::MaybeUninit<windows_core::BSTR>, ppdirectoryobject: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ITRendezvous_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITRendezvous_Impl::CreateDirectoryObject(this, core::mem::transmute_copy(&directoryobjecttype), core::mem::transmute(&pname)) {
                Ok(ok__) => {
                    ppdirectoryobject.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            DefaultDirectories: DefaultDirectories::<Identity, OFFSET>,
            EnumerateDefaultDirectories: EnumerateDefaultDirectories::<Identity, OFFSET>,
            CreateDirectory: CreateDirectory::<Identity, OFFSET>,
            CreateDirectoryObject: CreateDirectoryObject::<Identity, OFFSET>,
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
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ITRequest_Vtbl
    where
        Identity: ITRequest_Impl,
    {
        unsafe extern "system" fn MakeCall<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdestaddress: core::mem::MaybeUninit<windows_core::BSTR>, pappname: core::mem::MaybeUninit<windows_core::BSTR>, pcalledparty: core::mem::MaybeUninit<windows_core::BSTR>, pcomment: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: ITRequest_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITRequest_Impl::MakeCall(this, core::mem::transmute(&pdestaddress), core::mem::transmute(&pappname), core::mem::transmute(&pcalledparty), core::mem::transmute(&pcomment)).into()
        }
        Self { base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(), MakeCall: MakeCall::<Identity, OFFSET> }
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
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ITRequestEvent_Vtbl
    where
        Identity: ITRequestEvent_Impl,
    {
        unsafe extern "system" fn RegistrationInstance<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, plregistrationinstance: *mut i32) -> windows_core::HRESULT
        where
            Identity: ITRequestEvent_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITRequestEvent_Impl::RegistrationInstance(this) {
                Ok(ok__) => {
                    plregistrationinstance.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RequestMode<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, plrequestmode: *mut i32) -> windows_core::HRESULT
        where
            Identity: ITRequestEvent_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITRequestEvent_Impl::RequestMode(this) {
                Ok(ok__) => {
                    plrequestmode.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DestAddress<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppdestaddress: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: ITRequestEvent_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITRequestEvent_Impl::DestAddress(this) {
                Ok(ok__) => {
                    ppdestaddress.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AppName<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppappname: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: ITRequestEvent_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITRequestEvent_Impl::AppName(this) {
                Ok(ok__) => {
                    ppappname.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CalledParty<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppcalledparty: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: ITRequestEvent_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITRequestEvent_Impl::CalledParty(this) {
                Ok(ok__) => {
                    ppcalledparty.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Comment<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppcomment: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: ITRequestEvent_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITRequestEvent_Impl::Comment(this) {
                Ok(ok__) => {
                    ppcomment.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            RegistrationInstance: RegistrationInstance::<Identity, OFFSET>,
            RequestMode: RequestMode::<Identity, OFFSET>,
            DestAddress: DestAddress::<Identity, OFFSET>,
            AppName: AppName::<Identity, OFFSET>,
            CalledParty: CalledParty::<Identity, OFFSET>,
            Comment: Comment::<Identity, OFFSET>,
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
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ITScriptableAudioFormat_Vtbl
    where
        Identity: ITScriptableAudioFormat_Impl,
    {
        unsafe extern "system" fn Channels<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut i32) -> windows_core::HRESULT
        where
            Identity: ITScriptableAudioFormat_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITScriptableAudioFormat_Impl::Channels(this) {
                Ok(ok__) => {
                    pval.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetChannels<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, nnewval: i32) -> windows_core::HRESULT
        where
            Identity: ITScriptableAudioFormat_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITScriptableAudioFormat_Impl::SetChannels(this, core::mem::transmute_copy(&nnewval)).into()
        }
        unsafe extern "system" fn SamplesPerSec<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut i32) -> windows_core::HRESULT
        where
            Identity: ITScriptableAudioFormat_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITScriptableAudioFormat_Impl::SamplesPerSec(this) {
                Ok(ok__) => {
                    pval.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSamplesPerSec<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, nnewval: i32) -> windows_core::HRESULT
        where
            Identity: ITScriptableAudioFormat_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITScriptableAudioFormat_Impl::SetSamplesPerSec(this, core::mem::transmute_copy(&nnewval)).into()
        }
        unsafe extern "system" fn AvgBytesPerSec<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut i32) -> windows_core::HRESULT
        where
            Identity: ITScriptableAudioFormat_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITScriptableAudioFormat_Impl::AvgBytesPerSec(this) {
                Ok(ok__) => {
                    pval.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAvgBytesPerSec<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, nnewval: i32) -> windows_core::HRESULT
        where
            Identity: ITScriptableAudioFormat_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITScriptableAudioFormat_Impl::SetAvgBytesPerSec(this, core::mem::transmute_copy(&nnewval)).into()
        }
        unsafe extern "system" fn BlockAlign<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut i32) -> windows_core::HRESULT
        where
            Identity: ITScriptableAudioFormat_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITScriptableAudioFormat_Impl::BlockAlign(this) {
                Ok(ok__) => {
                    pval.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetBlockAlign<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, nnewval: i32) -> windows_core::HRESULT
        where
            Identity: ITScriptableAudioFormat_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITScriptableAudioFormat_Impl::SetBlockAlign(this, core::mem::transmute_copy(&nnewval)).into()
        }
        unsafe extern "system" fn BitsPerSample<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut i32) -> windows_core::HRESULT
        where
            Identity: ITScriptableAudioFormat_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITScriptableAudioFormat_Impl::BitsPerSample(this) {
                Ok(ok__) => {
                    pval.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetBitsPerSample<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, nnewval: i32) -> windows_core::HRESULT
        where
            Identity: ITScriptableAudioFormat_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITScriptableAudioFormat_Impl::SetBitsPerSample(this, core::mem::transmute_copy(&nnewval)).into()
        }
        unsafe extern "system" fn FormatTag<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut i32) -> windows_core::HRESULT
        where
            Identity: ITScriptableAudioFormat_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITScriptableAudioFormat_Impl::FormatTag(this) {
                Ok(ok__) => {
                    pval.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetFormatTag<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, nnewval: i32) -> windows_core::HRESULT
        where
            Identity: ITScriptableAudioFormat_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITScriptableAudioFormat_Impl::SetFormatTag(this, core::mem::transmute_copy(&nnewval)).into()
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            Channels: Channels::<Identity, OFFSET>,
            SetChannels: SetChannels::<Identity, OFFSET>,
            SamplesPerSec: SamplesPerSec::<Identity, OFFSET>,
            SetSamplesPerSec: SetSamplesPerSec::<Identity, OFFSET>,
            AvgBytesPerSec: AvgBytesPerSec::<Identity, OFFSET>,
            SetAvgBytesPerSec: SetAvgBytesPerSec::<Identity, OFFSET>,
            BlockAlign: BlockAlign::<Identity, OFFSET>,
            SetBlockAlign: SetBlockAlign::<Identity, OFFSET>,
            BitsPerSample: BitsPerSample::<Identity, OFFSET>,
            SetBitsPerSample: SetBitsPerSample::<Identity, OFFSET>,
            FormatTag: FormatTag::<Identity, OFFSET>,
            SetFormatTag: SetFormatTag::<Identity, OFFSET>,
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
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ITStaticAudioTerminal_Vtbl
    where
        Identity: ITStaticAudioTerminal_Impl,
    {
        unsafe extern "system" fn WaveId<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, plwaveid: *mut i32) -> windows_core::HRESULT
        where
            Identity: ITStaticAudioTerminal_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITStaticAudioTerminal_Impl::WaveId(this) {
                Ok(ok__) => {
                    plwaveid.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self { base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(), WaveId: WaveId::<Identity, OFFSET> }
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
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ITStream_Vtbl
    where
        Identity: ITStream_Impl,
    {
        unsafe extern "system" fn MediaType<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, plmediatype: *mut i32) -> windows_core::HRESULT
        where
            Identity: ITStream_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITStream_Impl::MediaType(this) {
                Ok(ok__) => {
                    plmediatype.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Direction<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ptd: *mut TERMINAL_DIRECTION) -> windows_core::HRESULT
        where
            Identity: ITStream_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITStream_Impl::Direction(this) {
                Ok(ok__) => {
                    ptd.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Name<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppname: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: ITStream_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITStream_Impl::Name(this) {
                Ok(ok__) => {
                    ppname.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn StartStream<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ITStream_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITStream_Impl::StartStream(this).into()
        }
        unsafe extern "system" fn PauseStream<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ITStream_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITStream_Impl::PauseStream(this).into()
        }
        unsafe extern "system" fn StopStream<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ITStream_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITStream_Impl::StopStream(this).into()
        }
        unsafe extern "system" fn SelectTerminal<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pterminal: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ITStream_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITStream_Impl::SelectTerminal(this, windows_core::from_raw_borrowed(&pterminal)).into()
        }
        unsafe extern "system" fn UnselectTerminal<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pterminal: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ITStream_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITStream_Impl::UnselectTerminal(this, windows_core::from_raw_borrowed(&pterminal)).into()
        }
        unsafe extern "system" fn EnumerateTerminals<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppenumterminal: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ITStream_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITStream_Impl::EnumerateTerminals(this) {
                Ok(ok__) => {
                    ppenumterminal.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Terminals<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pterminals: *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT
        where
            Identity: ITStream_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITStream_Impl::Terminals(this) {
                Ok(ok__) => {
                    pterminals.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            MediaType: MediaType::<Identity, OFFSET>,
            Direction: Direction::<Identity, OFFSET>,
            Name: Name::<Identity, OFFSET>,
            StartStream: StartStream::<Identity, OFFSET>,
            PauseStream: PauseStream::<Identity, OFFSET>,
            StopStream: StopStream::<Identity, OFFSET>,
            SelectTerminal: SelectTerminal::<Identity, OFFSET>,
            UnselectTerminal: UnselectTerminal::<Identity, OFFSET>,
            EnumerateTerminals: EnumerateTerminals::<Identity, OFFSET>,
            Terminals: Terminals::<Identity, OFFSET>,
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
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ITStreamControl_Vtbl
    where
        Identity: ITStreamControl_Impl,
    {
        unsafe extern "system" fn CreateStream<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, lmediatype: i32, td: TERMINAL_DIRECTION, ppstream: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ITStreamControl_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITStreamControl_Impl::CreateStream(this, core::mem::transmute_copy(&lmediatype), core::mem::transmute_copy(&td)) {
                Ok(ok__) => {
                    ppstream.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveStream<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pstream: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ITStreamControl_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITStreamControl_Impl::RemoveStream(this, windows_core::from_raw_borrowed(&pstream)).into()
        }
        unsafe extern "system" fn EnumerateStreams<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppenumstream: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ITStreamControl_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITStreamControl_Impl::EnumerateStreams(this) {
                Ok(ok__) => {
                    ppenumstream.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Streams<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvariant: *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT
        where
            Identity: ITStreamControl_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITStreamControl_Impl::Streams(this) {
                Ok(ok__) => {
                    pvariant.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            CreateStream: CreateStream::<Identity, OFFSET>,
            RemoveStream: RemoveStream::<Identity, OFFSET>,
            EnumerateStreams: EnumerateStreams::<Identity, OFFSET>,
            Streams: Streams::<Identity, OFFSET>,
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
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ITSubStream_Vtbl
    where
        Identity: ITSubStream_Impl,
    {
        unsafe extern "system" fn StartSubStream<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ITSubStream_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITSubStream_Impl::StartSubStream(this).into()
        }
        unsafe extern "system" fn PauseSubStream<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ITSubStream_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITSubStream_Impl::PauseSubStream(this).into()
        }
        unsafe extern "system" fn StopSubStream<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ITSubStream_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITSubStream_Impl::StopSubStream(this).into()
        }
        unsafe extern "system" fn SelectTerminal<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pterminal: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ITSubStream_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITSubStream_Impl::SelectTerminal(this, windows_core::from_raw_borrowed(&pterminal)).into()
        }
        unsafe extern "system" fn UnselectTerminal<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pterminal: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ITSubStream_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITSubStream_Impl::UnselectTerminal(this, windows_core::from_raw_borrowed(&pterminal)).into()
        }
        unsafe extern "system" fn EnumerateTerminals<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppenumterminal: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ITSubStream_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITSubStream_Impl::EnumerateTerminals(this) {
                Ok(ok__) => {
                    ppenumterminal.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Terminals<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pterminals: *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT
        where
            Identity: ITSubStream_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITSubStream_Impl::Terminals(this) {
                Ok(ok__) => {
                    pterminals.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Stream<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppitstream: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ITSubStream_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITSubStream_Impl::Stream(this) {
                Ok(ok__) => {
                    ppitstream.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            StartSubStream: StartSubStream::<Identity, OFFSET>,
            PauseSubStream: PauseSubStream::<Identity, OFFSET>,
            StopSubStream: StopSubStream::<Identity, OFFSET>,
            SelectTerminal: SelectTerminal::<Identity, OFFSET>,
            UnselectTerminal: UnselectTerminal::<Identity, OFFSET>,
            EnumerateTerminals: EnumerateTerminals::<Identity, OFFSET>,
            Terminals: Terminals::<Identity, OFFSET>,
            Stream: Stream::<Identity, OFFSET>,
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
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ITSubStreamControl_Vtbl
    where
        Identity: ITSubStreamControl_Impl,
    {
        unsafe extern "system" fn CreateSubStream<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppsubstream: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ITSubStreamControl_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITSubStreamControl_Impl::CreateSubStream(this) {
                Ok(ok__) => {
                    ppsubstream.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveSubStream<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, psubstream: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ITSubStreamControl_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITSubStreamControl_Impl::RemoveSubStream(this, windows_core::from_raw_borrowed(&psubstream)).into()
        }
        unsafe extern "system" fn EnumerateSubStreams<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppenumsubstream: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ITSubStreamControl_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITSubStreamControl_Impl::EnumerateSubStreams(this) {
                Ok(ok__) => {
                    ppenumsubstream.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SubStreams<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvariant: *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT
        where
            Identity: ITSubStreamControl_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITSubStreamControl_Impl::SubStreams(this) {
                Ok(ok__) => {
                    pvariant.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            CreateSubStream: CreateSubStream::<Identity, OFFSET>,
            RemoveSubStream: RemoveSubStream::<Identity, OFFSET>,
            EnumerateSubStreams: EnumerateSubStreams::<Identity, OFFSET>,
            SubStreams: SubStreams::<Identity, OFFSET>,
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
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ITTAPI_Vtbl
    where
        Identity: ITTAPI_Impl,
    {
        unsafe extern "system" fn Initialize<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ITTAPI_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITTAPI_Impl::Initialize(this).into()
        }
        unsafe extern "system" fn Shutdown<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ITTAPI_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITTAPI_Impl::Shutdown(this).into()
        }
        unsafe extern "system" fn Addresses<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvariant: *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT
        where
            Identity: ITTAPI_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITTAPI_Impl::Addresses(this) {
                Ok(ok__) => {
                    pvariant.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnumerateAddresses<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppenumaddress: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ITTAPI_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITTAPI_Impl::EnumerateAddresses(this) {
                Ok(ok__) => {
                    ppenumaddress.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RegisterCallNotifications<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, paddress: *mut core::ffi::c_void, fmonitor: super::super::Foundation::VARIANT_BOOL, fowner: super::super::Foundation::VARIANT_BOOL, lmediatypes: i32, lcallbackinstance: i32, plregister: *mut i32) -> windows_core::HRESULT
        where
            Identity: ITTAPI_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITTAPI_Impl::RegisterCallNotifications(this, windows_core::from_raw_borrowed(&paddress), core::mem::transmute_copy(&fmonitor), core::mem::transmute_copy(&fowner), core::mem::transmute_copy(&lmediatypes), core::mem::transmute_copy(&lcallbackinstance)) {
                Ok(ok__) => {
                    plregister.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UnregisterNotifications<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, lregister: i32) -> windows_core::HRESULT
        where
            Identity: ITTAPI_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITTAPI_Impl::UnregisterNotifications(this, core::mem::transmute_copy(&lregister)).into()
        }
        unsafe extern "system" fn CallHubs<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvariant: *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT
        where
            Identity: ITTAPI_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITTAPI_Impl::CallHubs(this) {
                Ok(ok__) => {
                    pvariant.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnumerateCallHubs<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppenumcallhub: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ITTAPI_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITTAPI_Impl::EnumerateCallHubs(this) {
                Ok(ok__) => {
                    ppenumcallhub.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCallHubTracking<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, paddresses: core::mem::MaybeUninit<windows_core::VARIANT>, btracking: super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT
        where
            Identity: ITTAPI_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITTAPI_Impl::SetCallHubTracking(this, core::mem::transmute(&paddresses), core::mem::transmute_copy(&btracking)).into()
        }
        unsafe extern "system" fn EnumeratePrivateTAPIObjects<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppenumunknown: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ITTAPI_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITTAPI_Impl::EnumeratePrivateTAPIObjects(this) {
                Ok(ok__) => {
                    ppenumunknown.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PrivateTAPIObjects<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvariant: *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT
        where
            Identity: ITTAPI_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITTAPI_Impl::PrivateTAPIObjects(this) {
                Ok(ok__) => {
                    pvariant.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RegisterRequestRecipient<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, lregistrationinstance: i32, lrequestmode: i32, fenable: super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT
        where
            Identity: ITTAPI_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITTAPI_Impl::RegisterRequestRecipient(this, core::mem::transmute_copy(&lregistrationinstance), core::mem::transmute_copy(&lrequestmode), core::mem::transmute_copy(&fenable)).into()
        }
        unsafe extern "system" fn SetAssistedTelephonyPriority<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pappfilename: core::mem::MaybeUninit<windows_core::BSTR>, fpriority: super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT
        where
            Identity: ITTAPI_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITTAPI_Impl::SetAssistedTelephonyPriority(this, core::mem::transmute(&pappfilename), core::mem::transmute_copy(&fpriority)).into()
        }
        unsafe extern "system" fn SetApplicationPriority<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pappfilename: core::mem::MaybeUninit<windows_core::BSTR>, lmediatype: i32, fpriority: super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT
        where
            Identity: ITTAPI_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITTAPI_Impl::SetApplicationPriority(this, core::mem::transmute(&pappfilename), core::mem::transmute_copy(&lmediatype), core::mem::transmute_copy(&fpriority)).into()
        }
        unsafe extern "system" fn SetEventFilter<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, lfiltermask: i32) -> windows_core::HRESULT
        where
            Identity: ITTAPI_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITTAPI_Impl::SetEventFilter(this, core::mem::transmute_copy(&lfiltermask)).into()
        }
        unsafe extern "system" fn EventFilter<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, plfiltermask: *mut i32) -> windows_core::HRESULT
        where
            Identity: ITTAPI_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITTAPI_Impl::EventFilter(this) {
                Ok(ok__) => {
                    plfiltermask.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            Initialize: Initialize::<Identity, OFFSET>,
            Shutdown: Shutdown::<Identity, OFFSET>,
            Addresses: Addresses::<Identity, OFFSET>,
            EnumerateAddresses: EnumerateAddresses::<Identity, OFFSET>,
            RegisterCallNotifications: RegisterCallNotifications::<Identity, OFFSET>,
            UnregisterNotifications: UnregisterNotifications::<Identity, OFFSET>,
            CallHubs: CallHubs::<Identity, OFFSET>,
            EnumerateCallHubs: EnumerateCallHubs::<Identity, OFFSET>,
            SetCallHubTracking: SetCallHubTracking::<Identity, OFFSET>,
            EnumeratePrivateTAPIObjects: EnumeratePrivateTAPIObjects::<Identity, OFFSET>,
            PrivateTAPIObjects: PrivateTAPIObjects::<Identity, OFFSET>,
            RegisterRequestRecipient: RegisterRequestRecipient::<Identity, OFFSET>,
            SetAssistedTelephonyPriority: SetAssistedTelephonyPriority::<Identity, OFFSET>,
            SetApplicationPriority: SetApplicationPriority::<Identity, OFFSET>,
            SetEventFilter: SetEventFilter::<Identity, OFFSET>,
            EventFilter: EventFilter::<Identity, OFFSET>,
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
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ITTAPI2_Vtbl
    where
        Identity: ITTAPI2_Impl,
    {
        unsafe extern "system" fn Phones<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pphones: *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT
        where
            Identity: ITTAPI2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITTAPI2_Impl::Phones(this) {
                Ok(ok__) => {
                    pphones.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnumeratePhones<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppenumphone: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ITTAPI2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITTAPI2_Impl::EnumeratePhones(this) {
                Ok(ok__) => {
                    ppenumphone.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateEmptyCollectionObject<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppcollection: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ITTAPI2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITTAPI2_Impl::CreateEmptyCollectionObject(this) {
                Ok(ok__) => {
                    ppcollection.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: ITTAPI_Vtbl::new::<Identity, OFFSET>(),
            Phones: Phones::<Identity, OFFSET>,
            EnumeratePhones: EnumeratePhones::<Identity, OFFSET>,
            CreateEmptyCollectionObject: CreateEmptyCollectionObject::<Identity, OFFSET>,
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
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ITTAPICallCenter_Vtbl
    where
        Identity: ITTAPICallCenter_Impl,
    {
        unsafe extern "system" fn EnumerateAgentHandlers<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppenumhandler: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ITTAPICallCenter_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITTAPICallCenter_Impl::EnumerateAgentHandlers(this) {
                Ok(ok__) => {
                    ppenumhandler.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AgentHandlers<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvariant: *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT
        where
            Identity: ITTAPICallCenter_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITTAPICallCenter_Impl::AgentHandlers(this) {
                Ok(ok__) => {
                    pvariant.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            EnumerateAgentHandlers: EnumerateAgentHandlers::<Identity, OFFSET>,
            AgentHandlers: AgentHandlers::<Identity, OFFSET>,
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
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ITTAPIDispatchEventNotification_Vtbl
    where
        Identity: ITTAPIDispatchEventNotification_Impl,
    {
        Self { base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, OFFSET>() }
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
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ITTAPIEventNotification_Vtbl
    where
        Identity: ITTAPIEventNotification_Impl,
    {
        unsafe extern "system" fn Event<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, tapievent: TAPI_EVENT, pevent: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ITTAPIEventNotification_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITTAPIEventNotification_Impl::Event(this, core::mem::transmute_copy(&tapievent), windows_core::from_raw_borrowed(&pevent)).into()
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), Event: Event::<Identity, OFFSET> }
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
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ITTAPIObjectEvent_Vtbl
    where
        Identity: ITTAPIObjectEvent_Impl,
    {
        unsafe extern "system" fn TAPIObject<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pptapiobject: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ITTAPIObjectEvent_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITTAPIObjectEvent_Impl::TAPIObject(this) {
                Ok(ok__) => {
                    pptapiobject.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Event<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pevent: *mut TAPIOBJECT_EVENT) -> windows_core::HRESULT
        where
            Identity: ITTAPIObjectEvent_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITTAPIObjectEvent_Impl::Event(this) {
                Ok(ok__) => {
                    pevent.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Address<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppaddress: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ITTAPIObjectEvent_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITTAPIObjectEvent_Impl::Address(this) {
                Ok(ok__) => {
                    ppaddress.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CallbackInstance<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, plcallbackinstance: *mut i32) -> windows_core::HRESULT
        where
            Identity: ITTAPIObjectEvent_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITTAPIObjectEvent_Impl::CallbackInstance(this) {
                Ok(ok__) => {
                    plcallbackinstance.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            TAPIObject: TAPIObject::<Identity, OFFSET>,
            Event: Event::<Identity, OFFSET>,
            Address: Address::<Identity, OFFSET>,
            CallbackInstance: CallbackInstance::<Identity, OFFSET>,
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
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ITTAPIObjectEvent2_Vtbl
    where
        Identity: ITTAPIObjectEvent2_Impl,
    {
        unsafe extern "system" fn Phone<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppphone: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ITTAPIObjectEvent2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITTAPIObjectEvent2_Impl::Phone(this) {
                Ok(ok__) => {
                    ppphone.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self { base__: ITTAPIObjectEvent_Vtbl::new::<Identity, OFFSET>(), Phone: Phone::<Identity, OFFSET> }
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
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ITTTSTerminalEvent_Vtbl
    where
        Identity: ITTTSTerminalEvent_Impl,
    {
        unsafe extern "system" fn Terminal<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppterminal: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ITTTSTerminalEvent_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITTTSTerminalEvent_Impl::Terminal(this) {
                Ok(ok__) => {
                    ppterminal.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Call<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppcall: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ITTTSTerminalEvent_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITTTSTerminalEvent_Impl::Call(this) {
                Ok(ok__) => {
                    ppcall.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Error<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, phrerrorcode: *mut windows_core::HRESULT) -> windows_core::HRESULT
        where
            Identity: ITTTSTerminalEvent_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITTTSTerminalEvent_Impl::Error(this) {
                Ok(ok__) => {
                    phrerrorcode.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            Terminal: Terminal::<Identity, OFFSET>,
            Call: Call::<Identity, OFFSET>,
            Error: Error::<Identity, OFFSET>,
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
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ITTerminal_Vtbl
    where
        Identity: ITTerminal_Impl,
    {
        unsafe extern "system" fn Name<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppname: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: ITTerminal_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITTerminal_Impl::Name(this) {
                Ok(ok__) => {
                    ppname.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn State<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pterminalstate: *mut TERMINAL_STATE) -> windows_core::HRESULT
        where
            Identity: ITTerminal_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITTerminal_Impl::State(this) {
                Ok(ok__) => {
                    pterminalstate.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TerminalType<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ptype: *mut TERMINAL_TYPE) -> windows_core::HRESULT
        where
            Identity: ITTerminal_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITTerminal_Impl::TerminalType(this) {
                Ok(ok__) => {
                    ptype.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TerminalClass<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppterminalclass: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: ITTerminal_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITTerminal_Impl::TerminalClass(this) {
                Ok(ok__) => {
                    ppterminalclass.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MediaType<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, plmediatype: *mut i32) -> windows_core::HRESULT
        where
            Identity: ITTerminal_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITTerminal_Impl::MediaType(this) {
                Ok(ok__) => {
                    plmediatype.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Direction<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdirection: *mut TERMINAL_DIRECTION) -> windows_core::HRESULT
        where
            Identity: ITTerminal_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITTerminal_Impl::Direction(this) {
                Ok(ok__) => {
                    pdirection.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            Name: Name::<Identity, OFFSET>,
            State: State::<Identity, OFFSET>,
            TerminalType: TerminalType::<Identity, OFFSET>,
            TerminalClass: TerminalClass::<Identity, OFFSET>,
            MediaType: MediaType::<Identity, OFFSET>,
            Direction: Direction::<Identity, OFFSET>,
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
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ITTerminalSupport_Vtbl
    where
        Identity: ITTerminalSupport_Impl,
    {
        unsafe extern "system" fn StaticTerminals<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvariant: *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT
        where
            Identity: ITTerminalSupport_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITTerminalSupport_Impl::StaticTerminals(this) {
                Ok(ok__) => {
                    pvariant.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnumerateStaticTerminals<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppterminalenumerator: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ITTerminalSupport_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITTerminalSupport_Impl::EnumerateStaticTerminals(this) {
                Ok(ok__) => {
                    ppterminalenumerator.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DynamicTerminalClasses<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvariant: *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT
        where
            Identity: ITTerminalSupport_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITTerminalSupport_Impl::DynamicTerminalClasses(this) {
                Ok(ok__) => {
                    pvariant.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnumerateDynamicTerminalClasses<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppterminalclassenumerator: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ITTerminalSupport_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITTerminalSupport_Impl::EnumerateDynamicTerminalClasses(this) {
                Ok(ok__) => {
                    ppterminalclassenumerator.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateTerminal<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pterminalclass: core::mem::MaybeUninit<windows_core::BSTR>, lmediatype: i32, direction: TERMINAL_DIRECTION, ppterminal: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ITTerminalSupport_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITTerminalSupport_Impl::CreateTerminal(this, core::mem::transmute(&pterminalclass), core::mem::transmute_copy(&lmediatype), core::mem::transmute_copy(&direction)) {
                Ok(ok__) => {
                    ppterminal.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDefaultStaticTerminal<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, lmediatype: i32, direction: TERMINAL_DIRECTION, ppterminal: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ITTerminalSupport_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITTerminalSupport_Impl::GetDefaultStaticTerminal(this, core::mem::transmute_copy(&lmediatype), core::mem::transmute_copy(&direction)) {
                Ok(ok__) => {
                    ppterminal.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            StaticTerminals: StaticTerminals::<Identity, OFFSET>,
            EnumerateStaticTerminals: EnumerateStaticTerminals::<Identity, OFFSET>,
            DynamicTerminalClasses: DynamicTerminalClasses::<Identity, OFFSET>,
            EnumerateDynamicTerminalClasses: EnumerateDynamicTerminalClasses::<Identity, OFFSET>,
            CreateTerminal: CreateTerminal::<Identity, OFFSET>,
            GetDefaultStaticTerminal: GetDefaultStaticTerminal::<Identity, OFFSET>,
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
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ITTerminalSupport2_Vtbl
    where
        Identity: ITTerminalSupport2_Impl,
    {
        unsafe extern "system" fn PluggableSuperclasses<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvariant: *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT
        where
            Identity: ITTerminalSupport2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITTerminalSupport2_Impl::PluggableSuperclasses(this) {
                Ok(ok__) => {
                    pvariant.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnumeratePluggableSuperclasses<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppsuperclassenumerator: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ITTerminalSupport2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITTerminalSupport2_Impl::EnumeratePluggableSuperclasses(this) {
                Ok(ok__) => {
                    ppsuperclassenumerator.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn get_PluggableTerminalClasses<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrterminalsuperclass: core::mem::MaybeUninit<windows_core::BSTR>, lmediatype: i32, pvariant: *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT
        where
            Identity: ITTerminalSupport2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITTerminalSupport2_Impl::get_PluggableTerminalClasses(this, core::mem::transmute(&bstrterminalsuperclass), core::mem::transmute_copy(&lmediatype)) {
                Ok(ok__) => {
                    pvariant.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnumeratePluggableTerminalClasses<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, iidterminalsuperclass: windows_core::GUID, lmediatype: i32, ppclassenumerator: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ITTerminalSupport2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITTerminalSupport2_Impl::EnumeratePluggableTerminalClasses(this, core::mem::transmute(&iidterminalsuperclass), core::mem::transmute_copy(&lmediatype)) {
                Ok(ok__) => {
                    ppclassenumerator.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: ITTerminalSupport_Vtbl::new::<Identity, OFFSET>(),
            PluggableSuperclasses: PluggableSuperclasses::<Identity, OFFSET>,
            EnumeratePluggableSuperclasses: EnumeratePluggableSuperclasses::<Identity, OFFSET>,
            get_PluggableTerminalClasses: get_PluggableTerminalClasses::<Identity, OFFSET>,
            EnumeratePluggableTerminalClasses: EnumeratePluggableTerminalClasses::<Identity, OFFSET>,
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
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ITToneDetectionEvent_Vtbl
    where
        Identity: ITToneDetectionEvent_Impl,
    {
        unsafe extern "system" fn Call<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppcallinfo: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ITToneDetectionEvent_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITToneDetectionEvent_Impl::Call(this) {
                Ok(ok__) => {
                    ppcallinfo.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AppSpecific<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, plappspecific: *mut i32) -> windows_core::HRESULT
        where
            Identity: ITToneDetectionEvent_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITToneDetectionEvent_Impl::AppSpecific(this) {
                Ok(ok__) => {
                    plappspecific.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TickCount<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pltickcount: *mut i32) -> windows_core::HRESULT
        where
            Identity: ITToneDetectionEvent_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITToneDetectionEvent_Impl::TickCount(this) {
                Ok(ok__) => {
                    pltickcount.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CallbackInstance<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, plcallbackinstance: *mut i32) -> windows_core::HRESULT
        where
            Identity: ITToneDetectionEvent_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITToneDetectionEvent_Impl::CallbackInstance(this) {
                Ok(ok__) => {
                    plcallbackinstance.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            Call: Call::<Identity, OFFSET>,
            AppSpecific: AppSpecific::<Identity, OFFSET>,
            TickCount: TickCount::<Identity, OFFSET>,
            CallbackInstance: CallbackInstance::<Identity, OFFSET>,
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
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ITToneTerminalEvent_Vtbl
    where
        Identity: ITToneTerminalEvent_Impl,
    {
        unsafe extern "system" fn Terminal<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppterminal: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ITToneTerminalEvent_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITToneTerminalEvent_Impl::Terminal(this) {
                Ok(ok__) => {
                    ppterminal.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Call<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppcall: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ITToneTerminalEvent_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITToneTerminalEvent_Impl::Call(this) {
                Ok(ok__) => {
                    ppcall.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Error<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, phrerrorcode: *mut windows_core::HRESULT) -> windows_core::HRESULT
        where
            Identity: ITToneTerminalEvent_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITToneTerminalEvent_Impl::Error(this) {
                Ok(ok__) => {
                    phrerrorcode.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            Terminal: Terminal::<Identity, OFFSET>,
            Call: Call::<Identity, OFFSET>,
            Error: Error::<Identity, OFFSET>,
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
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ITnef_Vtbl
    where
        Identity: ITnef_Impl,
    {
        unsafe extern "system" fn AddProps<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ulflags: u32, ulelemid: u32, lpvdata: *mut core::ffi::c_void, lpproplist: *mut super::super::System::AddressBook::SPropTagArray) -> windows_core::HRESULT
        where
            Identity: ITnef_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITnef_Impl::AddProps(this, core::mem::transmute_copy(&ulflags), core::mem::transmute_copy(&ulelemid), core::mem::transmute_copy(&lpvdata), core::mem::transmute_copy(&lpproplist)).into()
        }
        unsafe extern "system" fn ExtractProps<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ulflags: u32, lpproplist: *mut super::super::System::AddressBook::SPropTagArray, lpproblems: *mut *mut STnefProblemArray) -> windows_core::HRESULT
        where
            Identity: ITnef_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITnef_Impl::ExtractProps(this, core::mem::transmute_copy(&ulflags), core::mem::transmute_copy(&lpproplist), core::mem::transmute_copy(&lpproblems)).into()
        }
        unsafe extern "system" fn Finish<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ulflags: u32, lpkey: *mut u16, lpproblems: *mut *mut STnefProblemArray) -> windows_core::HRESULT
        where
            Identity: ITnef_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITnef_Impl::Finish(this, core::mem::transmute_copy(&ulflags), core::mem::transmute_copy(&lpkey), core::mem::transmute_copy(&lpproblems)).into()
        }
        unsafe extern "system" fn OpenTaggedBody<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, lpmessage: *mut core::ffi::c_void, ulflags: u32, lppstream: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ITnef_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITnef_Impl::OpenTaggedBody(this, windows_core::from_raw_borrowed(&lpmessage), core::mem::transmute_copy(&ulflags)) {
                Ok(ok__) => {
                    lppstream.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetProps<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ulflags: u32, ulelemid: u32, cvalues: u32, lpprops: *mut super::super::System::AddressBook::SPropValue) -> windows_core::HRESULT
        where
            Identity: ITnef_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITnef_Impl::SetProps(this, core::mem::transmute_copy(&ulflags), core::mem::transmute_copy(&ulelemid), core::mem::transmute_copy(&cvalues), core::mem::transmute_copy(&lpprops)).into()
        }
        unsafe extern "system" fn EncodeRecips<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ulflags: u32, lprecipienttable: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ITnef_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITnef_Impl::EncodeRecips(this, core::mem::transmute_copy(&ulflags), windows_core::from_raw_borrowed(&lprecipienttable)).into()
        }
        unsafe extern "system" fn FinishComponent<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ulflags: u32, ulcomponentid: u32, lpcustomproplist: *mut super::super::System::AddressBook::SPropTagArray, lpcustomprops: *mut super::super::System::AddressBook::SPropValue, lpproplist: *mut super::super::System::AddressBook::SPropTagArray, lpproblems: *mut *mut STnefProblemArray) -> windows_core::HRESULT
        where
            Identity: ITnef_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITnef_Impl::FinishComponent(this, core::mem::transmute_copy(&ulflags), core::mem::transmute_copy(&ulcomponentid), core::mem::transmute_copy(&lpcustomproplist), core::mem::transmute_copy(&lpcustomprops), core::mem::transmute_copy(&lpproplist), core::mem::transmute_copy(&lpproblems)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            AddProps: AddProps::<Identity, OFFSET>,
            ExtractProps: ExtractProps::<Identity, OFFSET>,
            Finish: Finish::<Identity, OFFSET>,
            OpenTaggedBody: OpenTaggedBody::<Identity, OFFSET>,
            SetProps: SetProps::<Identity, OFFSET>,
            EncodeRecips: EncodeRecips::<Identity, OFFSET>,
            FinishComponent: FinishComponent::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITnef as windows_core::Interface>::IID
    }
}
