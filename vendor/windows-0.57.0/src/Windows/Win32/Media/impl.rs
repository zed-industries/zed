pub trait IReferenceClock_Impl: Sized {
    fn GetTime(&self) -> windows_core::Result<i64>;
    fn AdviseTime(&self, basetime: i64, streamtime: i64, hevent: super::Foundation::HANDLE) -> windows_core::Result<usize>;
    fn AdvisePeriodic(&self, starttime: i64, periodtime: i64, hsemaphore: super::Foundation::HANDLE) -> windows_core::Result<usize>;
    fn Unadvise(&self, dwadvisecookie: usize) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IReferenceClock {}
impl IReferenceClock_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IReferenceClock_Impl, const OFFSET: isize>() -> IReferenceClock_Vtbl {
        unsafe extern "system" fn GetTime<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IReferenceClock_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ptime: *mut i64) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IReferenceClock_Impl::GetTime(this) {
                Ok(ok__) => {
                    core::ptr::write(ptime, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AdviseTime<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IReferenceClock_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, basetime: i64, streamtime: i64, hevent: super::Foundation::HANDLE, pdwadvisecookie: *mut usize) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IReferenceClock_Impl::AdviseTime(this, core::mem::transmute_copy(&basetime), core::mem::transmute_copy(&streamtime), core::mem::transmute_copy(&hevent)) {
                Ok(ok__) => {
                    core::ptr::write(pdwadvisecookie, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AdvisePeriodic<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IReferenceClock_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, starttime: i64, periodtime: i64, hsemaphore: super::Foundation::HANDLE, pdwadvisecookie: *mut usize) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IReferenceClock_Impl::AdvisePeriodic(this, core::mem::transmute_copy(&starttime), core::mem::transmute_copy(&periodtime), core::mem::transmute_copy(&hsemaphore)) {
                Ok(ok__) => {
                    core::ptr::write(pdwadvisecookie, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Unadvise<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IReferenceClock_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwadvisecookie: usize) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IReferenceClock_Impl::Unadvise(this, core::mem::transmute_copy(&dwadvisecookie)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetTime: GetTime::<Identity, Impl, OFFSET>,
            AdviseTime: AdviseTime::<Identity, Impl, OFFSET>,
            AdvisePeriodic: AdvisePeriodic::<Identity, Impl, OFFSET>,
            Unadvise: Unadvise::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IReferenceClock as windows_core::Interface>::IID
    }
}
pub trait IReferenceClock2_Impl: Sized + IReferenceClock_Impl {}
impl windows_core::RuntimeName for IReferenceClock2 {}
impl IReferenceClock2_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IReferenceClock2_Impl, const OFFSET: isize>() -> IReferenceClock2_Vtbl {
        Self { base__: IReferenceClock_Vtbl::new::<Identity, Impl, OFFSET>() }
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
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IReferenceClockTimerControl_Impl, const OFFSET: isize>() -> IReferenceClockTimerControl_Vtbl {
        unsafe extern "system" fn SetDefaultTimerResolution<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IReferenceClockTimerControl_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, timerresolution: i64) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IReferenceClockTimerControl_Impl::SetDefaultTimerResolution(this, core::mem::transmute_copy(&timerresolution)).into()
        }
        unsafe extern "system" fn GetDefaultTimerResolution<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IReferenceClockTimerControl_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ptimerresolution: *mut i64) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IReferenceClockTimerControl_Impl::GetDefaultTimerResolution(this) {
                Ok(ok__) => {
                    core::ptr::write(ptimerresolution, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            SetDefaultTimerResolution: SetDefaultTimerResolution::<Identity, Impl, OFFSET>,
            GetDefaultTimerResolution: GetDefaultTimerResolution::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IReferenceClockTimerControl as windows_core::Interface>::IID
    }
}
