#[cfg(feature = "Win32_System_Ole")]
pub trait IADesktopP2_Impl: Sized {
    fn ReReadWallpaper(&self) -> windows_core::Result<()>;
    fn GetADObjectFlags(&self, pdwflags: *mut u32, dwmask: u32) -> windows_core::Result<()>;
    fn UpdateAllDesktopSubscriptions(&self) -> windows_core::Result<()>;
    fn MakeDynamicChanges(&self, poleobj: Option<&super::super::System::Ole::IOleObject>) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Ole")]
impl windows_core::RuntimeName for IADesktopP2 {}
#[cfg(feature = "Win32_System_Ole")]
impl IADesktopP2_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IADesktopP2_Vtbl
    where
        Identity: IADesktopP2_Impl,
    {
        unsafe extern "system" fn ReReadWallpaper<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IADesktopP2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IADesktopP2_Impl::ReReadWallpaper(this).into()
        }
        unsafe extern "system" fn GetADObjectFlags<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwflags: *mut u32, dwmask: u32) -> windows_core::HRESULT
        where
            Identity: IADesktopP2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IADesktopP2_Impl::GetADObjectFlags(this, core::mem::transmute_copy(&pdwflags), core::mem::transmute_copy(&dwmask)).into()
        }
        unsafe extern "system" fn UpdateAllDesktopSubscriptions<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IADesktopP2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IADesktopP2_Impl::UpdateAllDesktopSubscriptions(this).into()
        }
        unsafe extern "system" fn MakeDynamicChanges<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, poleobj: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IADesktopP2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IADesktopP2_Impl::MakeDynamicChanges(this, windows_core::from_raw_borrowed(&poleobj)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            ReReadWallpaper: ReReadWallpaper::<Identity, OFFSET>,
            GetADObjectFlags: GetADObjectFlags::<Identity, OFFSET>,
            UpdateAllDesktopSubscriptions: UpdateAllDesktopSubscriptions::<Identity, OFFSET>,
            MakeDynamicChanges: MakeDynamicChanges::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IADesktopP2 as windows_core::Interface>::IID
    }
}
pub trait IActiveDesktopP_Impl: Sized {
    fn SetSafeMode(&self, dwflags: u32) -> windows_core::Result<()>;
    fn EnsureUpdateHTML(&self) -> windows_core::Result<()>;
    fn SetScheme(&self, pwszschemename: &windows_core::PCWSTR, dwflags: u32) -> windows_core::Result<()>;
    fn GetScheme(&self, pwszschemename: windows_core::PWSTR, pdwcchbuffer: *mut u32, dwflags: u32) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IActiveDesktopP {}
impl IActiveDesktopP_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IActiveDesktopP_Vtbl
    where
        Identity: IActiveDesktopP_Impl,
    {
        unsafe extern "system" fn SetSafeMode<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwflags: u32) -> windows_core::HRESULT
        where
            Identity: IActiveDesktopP_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IActiveDesktopP_Impl::SetSafeMode(this, core::mem::transmute_copy(&dwflags)).into()
        }
        unsafe extern "system" fn EnsureUpdateHTML<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IActiveDesktopP_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IActiveDesktopP_Impl::EnsureUpdateHTML(this).into()
        }
        unsafe extern "system" fn SetScheme<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwszschemename: windows_core::PCWSTR, dwflags: u32) -> windows_core::HRESULT
        where
            Identity: IActiveDesktopP_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IActiveDesktopP_Impl::SetScheme(this, core::mem::transmute(&pwszschemename), core::mem::transmute_copy(&dwflags)).into()
        }
        unsafe extern "system" fn GetScheme<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwszschemename: windows_core::PWSTR, pdwcchbuffer: *mut u32, dwflags: u32) -> windows_core::HRESULT
        where
            Identity: IActiveDesktopP_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IActiveDesktopP_Impl::GetScheme(this, core::mem::transmute_copy(&pwszschemename), core::mem::transmute_copy(&pdwcchbuffer), core::mem::transmute_copy(&dwflags)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            SetSafeMode: SetSafeMode::<Identity, OFFSET>,
            EnsureUpdateHTML: EnsureUpdateHTML::<Identity, OFFSET>,
            SetScheme: SetScheme::<Identity, OFFSET>,
            GetScheme: GetScheme::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IActiveDesktopP as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IBriefcaseInitiator_Impl: Sized {
    fn IsMonikerInBriefcase(&self, pmk: Option<&super::super::System::Com::IMoniker>) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IBriefcaseInitiator {}
#[cfg(feature = "Win32_System_Com")]
impl IBriefcaseInitiator_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IBriefcaseInitiator_Vtbl
    where
        Identity: IBriefcaseInitiator_Impl,
    {
        unsafe extern "system" fn IsMonikerInBriefcase<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pmk: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IBriefcaseInitiator_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IBriefcaseInitiator_Impl::IsMonikerInBriefcase(this, windows_core::from_raw_borrowed(&pmk)).into()
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), IsMonikerInBriefcase: IsMonikerInBriefcase::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IBriefcaseInitiator as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Registry")]
pub trait IEmptyVolumeCache_Impl: Sized {
    fn Initialize(&self, hkregkey: super::super::System::Registry::HKEY, pcwszvolume: &windows_core::PCWSTR, ppwszdisplayname: *mut windows_core::PWSTR, ppwszdescription: *mut windows_core::PWSTR, pdwflags: *mut EMPTY_VOLUME_CACHE_FLAGS) -> windows_core::Result<()>;
    fn GetSpaceUsed(&self, pdwlspaceused: *mut u64, picb: Option<&IEmptyVolumeCacheCallBack>) -> windows_core::Result<()>;
    fn Purge(&self, dwlspacetofree: u64, picb: Option<&IEmptyVolumeCacheCallBack>) -> windows_core::Result<()>;
    fn ShowProperties(&self, hwnd: super::super::Foundation::HWND) -> windows_core::Result<()>;
    fn Deactivate(&self) -> windows_core::Result<EMPTY_VOLUME_CACHE_FLAGS>;
}
#[cfg(feature = "Win32_System_Registry")]
impl windows_core::RuntimeName for IEmptyVolumeCache {}
#[cfg(feature = "Win32_System_Registry")]
impl IEmptyVolumeCache_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IEmptyVolumeCache_Vtbl
    where
        Identity: IEmptyVolumeCache_Impl,
    {
        unsafe extern "system" fn Initialize<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, hkregkey: super::super::System::Registry::HKEY, pcwszvolume: windows_core::PCWSTR, ppwszdisplayname: *mut windows_core::PWSTR, ppwszdescription: *mut windows_core::PWSTR, pdwflags: *mut EMPTY_VOLUME_CACHE_FLAGS) -> windows_core::HRESULT
        where
            Identity: IEmptyVolumeCache_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEmptyVolumeCache_Impl::Initialize(this, core::mem::transmute_copy(&hkregkey), core::mem::transmute(&pcwszvolume), core::mem::transmute_copy(&ppwszdisplayname), core::mem::transmute_copy(&ppwszdescription), core::mem::transmute_copy(&pdwflags)).into()
        }
        unsafe extern "system" fn GetSpaceUsed<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwlspaceused: *mut u64, picb: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IEmptyVolumeCache_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEmptyVolumeCache_Impl::GetSpaceUsed(this, core::mem::transmute_copy(&pdwlspaceused), windows_core::from_raw_borrowed(&picb)).into()
        }
        unsafe extern "system" fn Purge<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwlspacetofree: u64, picb: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IEmptyVolumeCache_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEmptyVolumeCache_Impl::Purge(this, core::mem::transmute_copy(&dwlspacetofree), windows_core::from_raw_borrowed(&picb)).into()
        }
        unsafe extern "system" fn ShowProperties<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, hwnd: super::super::Foundation::HWND) -> windows_core::HRESULT
        where
            Identity: IEmptyVolumeCache_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEmptyVolumeCache_Impl::ShowProperties(this, core::mem::transmute_copy(&hwnd)).into()
        }
        unsafe extern "system" fn Deactivate<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwflags: *mut EMPTY_VOLUME_CACHE_FLAGS) -> windows_core::HRESULT
        where
            Identity: IEmptyVolumeCache_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IEmptyVolumeCache_Impl::Deactivate(this) {
                Ok(ok__) => {
                    pdwflags.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Initialize: Initialize::<Identity, OFFSET>,
            GetSpaceUsed: GetSpaceUsed::<Identity, OFFSET>,
            Purge: Purge::<Identity, OFFSET>,
            ShowProperties: ShowProperties::<Identity, OFFSET>,
            Deactivate: Deactivate::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IEmptyVolumeCache as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Registry")]
pub trait IEmptyVolumeCache2_Impl: Sized + IEmptyVolumeCache_Impl {
    fn InitializeEx(&self, hkregkey: super::super::System::Registry::HKEY, pcwszvolume: &windows_core::PCWSTR, pcwszkeyname: &windows_core::PCWSTR, ppwszdisplayname: *mut windows_core::PWSTR, ppwszdescription: *mut windows_core::PWSTR, ppwszbtntext: *mut windows_core::PWSTR, pdwflags: *mut EMPTY_VOLUME_CACHE_FLAGS) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Registry")]
impl windows_core::RuntimeName for IEmptyVolumeCache2 {}
#[cfg(feature = "Win32_System_Registry")]
impl IEmptyVolumeCache2_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IEmptyVolumeCache2_Vtbl
    where
        Identity: IEmptyVolumeCache2_Impl,
    {
        unsafe extern "system" fn InitializeEx<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, hkregkey: super::super::System::Registry::HKEY, pcwszvolume: windows_core::PCWSTR, pcwszkeyname: windows_core::PCWSTR, ppwszdisplayname: *mut windows_core::PWSTR, ppwszdescription: *mut windows_core::PWSTR, ppwszbtntext: *mut windows_core::PWSTR, pdwflags: *mut EMPTY_VOLUME_CACHE_FLAGS) -> windows_core::HRESULT
        where
            Identity: IEmptyVolumeCache2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEmptyVolumeCache2_Impl::InitializeEx(this, core::mem::transmute_copy(&hkregkey), core::mem::transmute(&pcwszvolume), core::mem::transmute(&pcwszkeyname), core::mem::transmute_copy(&ppwszdisplayname), core::mem::transmute_copy(&ppwszdescription), core::mem::transmute_copy(&ppwszbtntext), core::mem::transmute_copy(&pdwflags)).into()
        }
        Self { base__: IEmptyVolumeCache_Vtbl::new::<Identity, OFFSET>(), InitializeEx: InitializeEx::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IEmptyVolumeCache2 as windows_core::Interface>::IID || iid == &<IEmptyVolumeCache as windows_core::Interface>::IID
    }
}
pub trait IEmptyVolumeCacheCallBack_Impl: Sized {
    fn ScanProgress(&self, dwlspaceused: u64, dwflags: u32, pcwszstatus: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn PurgeProgress(&self, dwlspacefreed: u64, dwlspacetofree: u64, dwflags: u32, pcwszstatus: &windows_core::PCWSTR) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IEmptyVolumeCacheCallBack {}
impl IEmptyVolumeCacheCallBack_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IEmptyVolumeCacheCallBack_Vtbl
    where
        Identity: IEmptyVolumeCacheCallBack_Impl,
    {
        unsafe extern "system" fn ScanProgress<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwlspaceused: u64, dwflags: u32, pcwszstatus: windows_core::PCWSTR) -> windows_core::HRESULT
        where
            Identity: IEmptyVolumeCacheCallBack_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEmptyVolumeCacheCallBack_Impl::ScanProgress(this, core::mem::transmute_copy(&dwlspaceused), core::mem::transmute_copy(&dwflags), core::mem::transmute(&pcwszstatus)).into()
        }
        unsafe extern "system" fn PurgeProgress<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwlspacefreed: u64, dwlspacetofree: u64, dwflags: u32, pcwszstatus: windows_core::PCWSTR) -> windows_core::HRESULT
        where
            Identity: IEmptyVolumeCacheCallBack_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEmptyVolumeCacheCallBack_Impl::PurgeProgress(this, core::mem::transmute_copy(&dwlspacefreed), core::mem::transmute_copy(&dwlspacetofree), core::mem::transmute_copy(&dwflags), core::mem::transmute(&pcwszstatus)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            ScanProgress: ScanProgress::<Identity, OFFSET>,
            PurgeProgress: PurgeProgress::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IEmptyVolumeCacheCallBack as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com_StructuredStorage")]
pub trait IReconcilableObject_Impl: Sized {
    fn Reconcile(&self, pinitiator: Option<&IReconcileInitiator>, dwflags: u32, hwndowner: super::super::Foundation::HWND, hwndprogressfeedback: super::super::Foundation::HWND, ulcinput: u32, rgpmkotherinput: *mut Option<super::super::System::Com::IMoniker>, ploutindex: *mut i32, pstgnewresidues: Option<&super::super::System::Com::StructuredStorage::IStorage>, pvreserved: *const core::ffi::c_void) -> windows_core::Result<()>;
    fn GetProgressFeedbackMaxEstimate(&self) -> windows_core::Result<u32>;
}
#[cfg(feature = "Win32_System_Com_StructuredStorage")]
impl windows_core::RuntimeName for IReconcilableObject {}
#[cfg(feature = "Win32_System_Com_StructuredStorage")]
impl IReconcilableObject_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IReconcilableObject_Vtbl
    where
        Identity: IReconcilableObject_Impl,
    {
        unsafe extern "system" fn Reconcile<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pinitiator: *mut core::ffi::c_void, dwflags: u32, hwndowner: super::super::Foundation::HWND, hwndprogressfeedback: super::super::Foundation::HWND, ulcinput: u32, rgpmkotherinput: *mut *mut core::ffi::c_void, ploutindex: *mut i32, pstgnewresidues: *mut core::ffi::c_void, pvreserved: *const core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IReconcilableObject_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IReconcilableObject_Impl::Reconcile(this, windows_core::from_raw_borrowed(&pinitiator), core::mem::transmute_copy(&dwflags), core::mem::transmute_copy(&hwndowner), core::mem::transmute_copy(&hwndprogressfeedback), core::mem::transmute_copy(&ulcinput), core::mem::transmute_copy(&rgpmkotherinput), core::mem::transmute_copy(&ploutindex), windows_core::from_raw_borrowed(&pstgnewresidues), core::mem::transmute_copy(&pvreserved)).into()
        }
        unsafe extern "system" fn GetProgressFeedbackMaxEstimate<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pulprogressmax: *mut u32) -> windows_core::HRESULT
        where
            Identity: IReconcilableObject_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IReconcilableObject_Impl::GetProgressFeedbackMaxEstimate(this) {
                Ok(ok__) => {
                    pulprogressmax.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Reconcile: Reconcile::<Identity, OFFSET>,
            GetProgressFeedbackMaxEstimate: GetProgressFeedbackMaxEstimate::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IReconcilableObject as windows_core::Interface>::IID
    }
}
pub trait IReconcileInitiator_Impl: Sized {
    fn SetAbortCallback(&self, punkforabort: Option<&windows_core::IUnknown>) -> windows_core::Result<()>;
    fn SetProgressFeedback(&self, ulprogress: u32, ulprogressmax: u32) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IReconcileInitiator {}
impl IReconcileInitiator_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IReconcileInitiator_Vtbl
    where
        Identity: IReconcileInitiator_Impl,
    {
        unsafe extern "system" fn SetAbortCallback<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, punkforabort: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IReconcileInitiator_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IReconcileInitiator_Impl::SetAbortCallback(this, windows_core::from_raw_borrowed(&punkforabort)).into()
        }
        unsafe extern "system" fn SetProgressFeedback<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ulprogress: u32, ulprogressmax: u32) -> windows_core::HRESULT
        where
            Identity: IReconcileInitiator_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IReconcileInitiator_Impl::SetProgressFeedback(this, core::mem::transmute_copy(&ulprogress), core::mem::transmute_copy(&ulprogressmax)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            SetAbortCallback: SetAbortCallback::<Identity, OFFSET>,
            SetProgressFeedback: SetProgressFeedback::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IReconcileInitiator as windows_core::Interface>::IID
    }
}
