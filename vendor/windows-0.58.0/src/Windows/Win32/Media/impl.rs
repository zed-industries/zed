pub trait IReferenceClock_Impl: Sized {
    fn GetTime(&self) -> windows_core::Result<i64>;
    fn AdviseTime(&self, basetime: i64, streamtime: i64, hevent: super::Foundation::HANDLE) -> windows_core::Result<usize>;
    fn AdvisePeriodic(&self, starttime: i64, periodtime: i64, hsemaphore: super::Foundation::HANDLE) -> windows_core::Result<usize>;
    fn Unadvise(&self, dwadvisecookie: usize) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IReferenceClock {}
impl IReferenceClock_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IReferenceClock_Vtbl
    where
        Identity: IReferenceClock_Impl,
    {
        unsafe extern "system" fn GetTime<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ptime: *mut i64) -> windows_core::HRESULT
        where
            Identity: IReferenceClock_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IReferenceClock_Impl::GetTime(this) {
                Ok(ok__) => {
                    ptime.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AdviseTime<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, basetime: i64, streamtime: i64, hevent: super::Foundation::HANDLE, pdwadvisecookie: *mut usize) -> windows_core::HRESULT
        where
            Identity: IReferenceClock_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IReferenceClock_Impl::AdviseTime(this, core::mem::transmute_copy(&basetime), core::mem::transmute_copy(&streamtime), core::mem::transmute_copy(&hevent)) {
                Ok(ok__) => {
                    pdwadvisecookie.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AdvisePeriodic<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, starttime: i64, periodtime: i64, hsemaphore: super::Foundation::HANDLE, pdwadvisecookie: *mut usize) -> windows_core::HRESULT
        where
            Identity: IReferenceClock_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IReferenceClock_Impl::AdvisePeriodic(this, core::mem::transmute_copy(&starttime), core::mem::transmute_copy(&periodtime), core::mem::transmute_copy(&hsemaphore)) {
                Ok(ok__) => {
                    pdwadvisecookie.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Unadvise<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwadvisecookie: usize) -> windows_core::HRESULT
        where
            Identity: IReferenceClock_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IReferenceClock_Impl::Unadvise(this, core::mem::transmute_copy(&dwadvisecookie)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetTime: GetTime::<Identity, OFFSET>,
            AdviseTime: AdviseTime::<Identity, OFFSET>,
            AdvisePeriodic: AdvisePeriodic::<Identity, OFFSET>,
            Unadvise: Unadvise::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IReferenceClock as windows_core::Interface>::IID
    }
}
pub trait IReferenceClock2_Impl: Sized + IReferenceClock_Impl {}
impl windows_core::RuntimeName for IReferenceClock2 {}
impl IReferenceClock2_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IReferenceClock2_Vtbl
    where
        Identity: IReferenceClock2_Impl,
    {
        Self { base__: IReferenceClock_Vtbl::new::<Identity, OFFSET>() }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IReferenceClock2 as windows_core::Interface>::IID || iid == &<IReferenceClock as windows_core::Interface>::IID
    }
}
pub trait IReferenceClockTimerControl_Impl: Sized {
    fn SetDefaultTimerResolution(&self, timerresolution: i64) -> windows_core::Result<()>;
    fn GetDefaultTimerResolution(&self) -> windows_core::Result<i64>;
}
impl windows_core::RuntimeName for IReferenceClockTimerControl {}
impl IReferenceClockTimerControl_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IReferenceClockTimerControl_Vtbl
    where
        Identity: IReferenceClockTimerControl_Impl,
    {
        unsafe extern "system" fn SetDefaultTimerResolution<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, timerresolution: i64) -> windows_core::HRESULT
        where
            Identity: IReferenceClockTimerControl_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IReferenceClockTimerControl_Impl::SetDefaultTimerResolution(this, core::mem::transmute_copy(&timerresolution)).into()
        }
        unsafe extern "system" fn GetDefaultTimerResolution<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ptimerresolution: *mut i64) -> windows_core::HRESULT
        where
            Identity: IReferenceClockTimerControl_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IReferenceClockTimerControl_Impl::GetDefaultTimerResolution(this) {
                Ok(ok__) => {
                    ptimerresolution.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            SetDefaultTimerResolution: SetDefaultTimerResolution::<Identity, OFFSET>,
            GetDefaultTimerResolution: GetDefaultTimerResolution::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IReferenceClockTimerControl as windows_core::Interface>::IID
    }
}
