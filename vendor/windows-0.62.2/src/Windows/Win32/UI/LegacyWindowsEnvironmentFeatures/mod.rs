pub const ALL_RECONCILE_FLAGS: RECONCILEF = RECONCILEF(127i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct EMPTY_VOLUME_CACHE_FLAGS(pub u32);
pub const EVCCBF_LASTNOTIFICATION: u32 = 1u32;
pub const EVCF_DONTSHOWIFZERO: EMPTY_VOLUME_CACHE_FLAGS = EMPTY_VOLUME_CACHE_FLAGS(16u32);
pub const EVCF_ENABLEBYDEFAULT: EMPTY_VOLUME_CACHE_FLAGS = EMPTY_VOLUME_CACHE_FLAGS(2u32);
pub const EVCF_ENABLEBYDEFAULT_AUTO: EMPTY_VOLUME_CACHE_FLAGS = EMPTY_VOLUME_CACHE_FLAGS(8u32);
pub const EVCF_HASSETTINGS: EMPTY_VOLUME_CACHE_FLAGS = EMPTY_VOLUME_CACHE_FLAGS(1u32);
pub const EVCF_OUTOFDISKSPACE: EMPTY_VOLUME_CACHE_FLAGS = EMPTY_VOLUME_CACHE_FLAGS(64u32);
pub const EVCF_REMOVEFROMLIST: EMPTY_VOLUME_CACHE_FLAGS = EMPTY_VOLUME_CACHE_FLAGS(4u32);
pub const EVCF_SETTINGSMODE: EMPTY_VOLUME_CACHE_FLAGS = EMPTY_VOLUME_CACHE_FLAGS(32u32);
pub const EVCF_SYSTEMAUTORUN: EMPTY_VOLUME_CACHE_FLAGS = EMPTY_VOLUME_CACHE_FLAGS(256u32);
pub const EVCF_USERCONSENTOBTAINED: EMPTY_VOLUME_CACHE_FLAGS = EMPTY_VOLUME_CACHE_FLAGS(128u32);
windows_core::imp::define_interface!(IADesktopP2, IADesktopP2_Vtbl, 0xb22754e2_4574_11d1_9888_006097deacf9);
windows_core::imp::interface_hierarchy!(IADesktopP2, windows_core::IUnknown);
impl IADesktopP2 {
    pub unsafe fn ReReadWallpaper(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).ReReadWallpaper)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn GetADObjectFlags(&self, pdwflags: *mut u32, dwmask: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetADObjectFlags)(windows_core::Interface::as_raw(self), pdwflags as _, dwmask).ok() }
    }
    pub unsafe fn UpdateAllDesktopSubscriptions(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).UpdateAllDesktopSubscriptions)(windows_core::Interface::as_raw(self)).ok() }
    }
    #[cfg(feature = "Win32_System_Ole")]
    pub unsafe fn MakeDynamicChanges<P0>(&self, poleobj: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::System::Ole::IOleObject>,
    {
        unsafe { (windows_core::Interface::vtable(self).MakeDynamicChanges)(windows_core::Interface::as_raw(self), poleobj.param().abi()).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IADesktopP2_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub ReReadWallpaper: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetADObjectFlags: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32, u32) -> windows_core::HRESULT,
    pub UpdateAllDesktopSubscriptions: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Ole")]
    pub MakeDynamicChanges: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Ole"))]
    MakeDynamicChanges: usize,
}
#[cfg(feature = "Win32_System_Ole")]
pub trait IADesktopP2_Impl: windows_core::IUnknownImpl {
    fn ReReadWallpaper(&self) -> windows_core::Result<()>;
    fn GetADObjectFlags(&self, pdwflags: *mut u32, dwmask: u32) -> windows_core::Result<()>;
    fn UpdateAllDesktopSubscriptions(&self) -> windows_core::Result<()>;
    fn MakeDynamicChanges(&self, poleobj: windows_core::Ref<super::super::System::Ole::IOleObject>) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Ole")]
impl IADesktopP2_Vtbl {
    pub const fn new<Identity: IADesktopP2_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn ReReadWallpaper<Identity: IADesktopP2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IADesktopP2_Impl::ReReadWallpaper(this).into()
            }
        }
        unsafe extern "system" fn GetADObjectFlags<Identity: IADesktopP2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwflags: *mut u32, dwmask: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IADesktopP2_Impl::GetADObjectFlags(this, core::mem::transmute_copy(&pdwflags), core::mem::transmute_copy(&dwmask)).into()
            }
        }
        unsafe extern "system" fn UpdateAllDesktopSubscriptions<Identity: IADesktopP2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IADesktopP2_Impl::UpdateAllDesktopSubscriptions(this).into()
            }
        }
        unsafe extern "system" fn MakeDynamicChanges<Identity: IADesktopP2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, poleobj: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IADesktopP2_Impl::MakeDynamicChanges(this, core::mem::transmute_copy(&poleobj)).into()
            }
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
#[cfg(feature = "Win32_System_Ole")]
impl windows_core::RuntimeName for IADesktopP2 {}
windows_core::imp::define_interface!(IActiveDesktopP, IActiveDesktopP_Vtbl, 0x52502ee0_ec80_11d0_89ab_00c04fc2972d);
windows_core::imp::interface_hierarchy!(IActiveDesktopP, windows_core::IUnknown);
impl IActiveDesktopP {
    pub unsafe fn SetSafeMode(&self, dwflags: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetSafeMode)(windows_core::Interface::as_raw(self), dwflags).ok() }
    }
    pub unsafe fn EnsureUpdateHTML(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).EnsureUpdateHTML)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn SetScheme<P0>(&self, pwszschemename: P0, dwflags: u32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetScheme)(windows_core::Interface::as_raw(self), pwszschemename.param().abi(), dwflags).ok() }
    }
    pub unsafe fn GetScheme(&self, pwszschemename: windows_core::PWSTR, pdwcchbuffer: *mut u32, dwflags: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetScheme)(windows_core::Interface::as_raw(self), core::mem::transmute(pwszschemename), pdwcchbuffer as _, dwflags).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IActiveDesktopP_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub SetSafeMode: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub EnsureUpdateHTML: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetScheme: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, u32) -> windows_core::HRESULT,
    pub GetScheme: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PWSTR, *mut u32, u32) -> windows_core::HRESULT,
}
pub trait IActiveDesktopP_Impl: windows_core::IUnknownImpl {
    fn SetSafeMode(&self, dwflags: u32) -> windows_core::Result<()>;
    fn EnsureUpdateHTML(&self) -> windows_core::Result<()>;
    fn SetScheme(&self, pwszschemename: &windows_core::PCWSTR, dwflags: u32) -> windows_core::Result<()>;
    fn GetScheme(&self, pwszschemename: windows_core::PWSTR, pdwcchbuffer: *mut u32, dwflags: u32) -> windows_core::Result<()>;
}
impl IActiveDesktopP_Vtbl {
    pub const fn new<Identity: IActiveDesktopP_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn SetSafeMode<Identity: IActiveDesktopP_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwflags: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IActiveDesktopP_Impl::SetSafeMode(this, core::mem::transmute_copy(&dwflags)).into()
            }
        }
        unsafe extern "system" fn EnsureUpdateHTML<Identity: IActiveDesktopP_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IActiveDesktopP_Impl::EnsureUpdateHTML(this).into()
            }
        }
        unsafe extern "system" fn SetScheme<Identity: IActiveDesktopP_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwszschemename: windows_core::PCWSTR, dwflags: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IActiveDesktopP_Impl::SetScheme(this, core::mem::transmute(&pwszschemename), core::mem::transmute_copy(&dwflags)).into()
            }
        }
        unsafe extern "system" fn GetScheme<Identity: IActiveDesktopP_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwszschemename: windows_core::PWSTR, pdwcchbuffer: *mut u32, dwflags: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IActiveDesktopP_Impl::GetScheme(this, core::mem::transmute_copy(&pwszschemename), core::mem::transmute_copy(&pdwcchbuffer), core::mem::transmute_copy(&dwflags)).into()
            }
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
impl windows_core::RuntimeName for IActiveDesktopP {}
windows_core::imp::define_interface!(IBriefcaseInitiator, IBriefcaseInitiator_Vtbl, 0x99180164_da16_101a_935c_444553540000);
windows_core::imp::interface_hierarchy!(IBriefcaseInitiator, windows_core::IUnknown);
impl IBriefcaseInitiator {
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn IsMonikerInBriefcase<P0>(&self, pmk: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::System::Com::IMoniker>,
    {
        unsafe { (windows_core::Interface::vtable(self).IsMonikerInBriefcase)(windows_core::Interface::as_raw(self), pmk.param().abi()).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IBriefcaseInitiator_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub IsMonikerInBriefcase: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    IsMonikerInBriefcase: usize,
}
#[cfg(feature = "Win32_System_Com")]
pub trait IBriefcaseInitiator_Impl: windows_core::IUnknownImpl {
    fn IsMonikerInBriefcase(&self, pmk: windows_core::Ref<super::super::System::Com::IMoniker>) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl IBriefcaseInitiator_Vtbl {
    pub const fn new<Identity: IBriefcaseInitiator_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn IsMonikerInBriefcase<Identity: IBriefcaseInitiator_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pmk: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IBriefcaseInitiator_Impl::IsMonikerInBriefcase(this, core::mem::transmute_copy(&pmk)).into()
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), IsMonikerInBriefcase: IsMonikerInBriefcase::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IBriefcaseInitiator as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IBriefcaseInitiator {}
windows_core::imp::define_interface!(IEmptyVolumeCache, IEmptyVolumeCache_Vtbl, 0x8fce5227_04da_11d1_a004_00805f8abe06);
windows_core::imp::interface_hierarchy!(IEmptyVolumeCache, windows_core::IUnknown);
impl IEmptyVolumeCache {
    #[cfg(feature = "Win32_System_Registry")]
    pub unsafe fn Initialize<P1>(&self, hkregkey: super::super::System::Registry::HKEY, pcwszvolume: P1, ppwszdisplayname: *mut windows_core::PWSTR, ppwszdescription: *mut windows_core::PWSTR, pdwflags: *mut EMPTY_VOLUME_CACHE_FLAGS) -> windows_core::Result<()>
    where
        P1: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).Initialize)(windows_core::Interface::as_raw(self), hkregkey, pcwszvolume.param().abi(), ppwszdisplayname as _, ppwszdescription as _, pdwflags as _).ok() }
    }
    pub unsafe fn GetSpaceUsed<P1>(&self, pdwlspaceused: *mut u64, picb: P1) -> windows_core::Result<()>
    where
        P1: windows_core::Param<IEmptyVolumeCacheCallBack>,
    {
        unsafe { (windows_core::Interface::vtable(self).GetSpaceUsed)(windows_core::Interface::as_raw(self), pdwlspaceused as _, picb.param().abi()).ok() }
    }
    pub unsafe fn Purge<P1>(&self, dwlspacetofree: u64, picb: P1) -> windows_core::Result<()>
    where
        P1: windows_core::Param<IEmptyVolumeCacheCallBack>,
    {
        unsafe { (windows_core::Interface::vtable(self).Purge)(windows_core::Interface::as_raw(self), dwlspacetofree, picb.param().abi()).ok() }
    }
    pub unsafe fn ShowProperties(&self, hwnd: super::super::Foundation::HWND) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).ShowProperties)(windows_core::Interface::as_raw(self), hwnd).ok() }
    }
    pub unsafe fn Deactivate(&self) -> windows_core::Result<EMPTY_VOLUME_CACHE_FLAGS> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Deactivate)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IEmptyVolumeCache_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_System_Registry")]
    pub Initialize: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::System::Registry::HKEY, windows_core::PCWSTR, *mut windows_core::PWSTR, *mut windows_core::PWSTR, *mut EMPTY_VOLUME_CACHE_FLAGS) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Registry"))]
    Initialize: usize,
    pub GetSpaceUsed: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u64, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Purge: unsafe extern "system" fn(*mut core::ffi::c_void, u64, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ShowProperties: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::HWND) -> windows_core::HRESULT,
    pub Deactivate: unsafe extern "system" fn(*mut core::ffi::c_void, *mut EMPTY_VOLUME_CACHE_FLAGS) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_Registry")]
pub trait IEmptyVolumeCache_Impl: windows_core::IUnknownImpl {
    fn Initialize(&self, hkregkey: super::super::System::Registry::HKEY, pcwszvolume: &windows_core::PCWSTR, ppwszdisplayname: *mut windows_core::PWSTR, ppwszdescription: *mut windows_core::PWSTR, pdwflags: *mut EMPTY_VOLUME_CACHE_FLAGS) -> windows_core::Result<()>;
    fn GetSpaceUsed(&self, pdwlspaceused: *mut u64, picb: windows_core::Ref<IEmptyVolumeCacheCallBack>) -> windows_core::Result<()>;
    fn Purge(&self, dwlspacetofree: u64, picb: windows_core::Ref<IEmptyVolumeCacheCallBack>) -> windows_core::Result<()>;
    fn ShowProperties(&self, hwnd: super::super::Foundation::HWND) -> windows_core::Result<()>;
    fn Deactivate(&self) -> windows_core::Result<EMPTY_VOLUME_CACHE_FLAGS>;
}
#[cfg(feature = "Win32_System_Registry")]
impl IEmptyVolumeCache_Vtbl {
    pub const fn new<Identity: IEmptyVolumeCache_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Initialize<Identity: IEmptyVolumeCache_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hkregkey: super::super::System::Registry::HKEY, pcwszvolume: windows_core::PCWSTR, ppwszdisplayname: *mut windows_core::PWSTR, ppwszdescription: *mut windows_core::PWSTR, pdwflags: *mut EMPTY_VOLUME_CACHE_FLAGS) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IEmptyVolumeCache_Impl::Initialize(this, core::mem::transmute_copy(&hkregkey), core::mem::transmute(&pcwszvolume), core::mem::transmute_copy(&ppwszdisplayname), core::mem::transmute_copy(&ppwszdescription), core::mem::transmute_copy(&pdwflags)).into()
            }
        }
        unsafe extern "system" fn GetSpaceUsed<Identity: IEmptyVolumeCache_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwlspaceused: *mut u64, picb: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IEmptyVolumeCache_Impl::GetSpaceUsed(this, core::mem::transmute_copy(&pdwlspaceused), core::mem::transmute_copy(&picb)).into()
            }
        }
        unsafe extern "system" fn Purge<Identity: IEmptyVolumeCache_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwlspacetofree: u64, picb: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IEmptyVolumeCache_Impl::Purge(this, core::mem::transmute_copy(&dwlspacetofree), core::mem::transmute_copy(&picb)).into()
            }
        }
        unsafe extern "system" fn ShowProperties<Identity: IEmptyVolumeCache_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hwnd: super::super::Foundation::HWND) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IEmptyVolumeCache_Impl::ShowProperties(this, core::mem::transmute_copy(&hwnd)).into()
            }
        }
        unsafe extern "system" fn Deactivate<Identity: IEmptyVolumeCache_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwflags: *mut EMPTY_VOLUME_CACHE_FLAGS) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IEmptyVolumeCache_Impl::Deactivate(this) {
                    Ok(ok__) => {
                        pdwflags.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
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
impl windows_core::RuntimeName for IEmptyVolumeCache {}
windows_core::imp::define_interface!(IEmptyVolumeCache2, IEmptyVolumeCache2_Vtbl, 0x02b7e3ba_4db3_11d2_b2d9_00c04f8eec8c);
impl core::ops::Deref for IEmptyVolumeCache2 {
    type Target = IEmptyVolumeCache;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IEmptyVolumeCache2, windows_core::IUnknown, IEmptyVolumeCache);
impl IEmptyVolumeCache2 {
    #[cfg(feature = "Win32_System_Registry")]
    pub unsafe fn InitializeEx<P1, P2>(&self, hkregkey: super::super::System::Registry::HKEY, pcwszvolume: P1, pcwszkeyname: P2, ppwszdisplayname: *mut windows_core::PWSTR, ppwszdescription: *mut windows_core::PWSTR, ppwszbtntext: *mut windows_core::PWSTR, pdwflags: *mut EMPTY_VOLUME_CACHE_FLAGS) -> windows_core::Result<()>
    where
        P1: windows_core::Param<windows_core::PCWSTR>,
        P2: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).InitializeEx)(windows_core::Interface::as_raw(self), hkregkey, pcwszvolume.param().abi(), pcwszkeyname.param().abi(), ppwszdisplayname as _, ppwszdescription as _, ppwszbtntext as _, pdwflags as _).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IEmptyVolumeCache2_Vtbl {
    pub base__: IEmptyVolumeCache_Vtbl,
    #[cfg(feature = "Win32_System_Registry")]
    pub InitializeEx: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::System::Registry::HKEY, windows_core::PCWSTR, windows_core::PCWSTR, *mut windows_core::PWSTR, *mut windows_core::PWSTR, *mut windows_core::PWSTR, *mut EMPTY_VOLUME_CACHE_FLAGS) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Registry"))]
    InitializeEx: usize,
}
#[cfg(feature = "Win32_System_Registry")]
pub trait IEmptyVolumeCache2_Impl: IEmptyVolumeCache_Impl {
    fn InitializeEx(&self, hkregkey: super::super::System::Registry::HKEY, pcwszvolume: &windows_core::PCWSTR, pcwszkeyname: &windows_core::PCWSTR, ppwszdisplayname: *mut windows_core::PWSTR, ppwszdescription: *mut windows_core::PWSTR, ppwszbtntext: *mut windows_core::PWSTR, pdwflags: *mut EMPTY_VOLUME_CACHE_FLAGS) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Registry")]
impl IEmptyVolumeCache2_Vtbl {
    pub const fn new<Identity: IEmptyVolumeCache2_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn InitializeEx<Identity: IEmptyVolumeCache2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hkregkey: super::super::System::Registry::HKEY, pcwszvolume: windows_core::PCWSTR, pcwszkeyname: windows_core::PCWSTR, ppwszdisplayname: *mut windows_core::PWSTR, ppwszdescription: *mut windows_core::PWSTR, ppwszbtntext: *mut windows_core::PWSTR, pdwflags: *mut EMPTY_VOLUME_CACHE_FLAGS) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IEmptyVolumeCache2_Impl::InitializeEx(this, core::mem::transmute_copy(&hkregkey), core::mem::transmute(&pcwszvolume), core::mem::transmute(&pcwszkeyname), core::mem::transmute_copy(&ppwszdisplayname), core::mem::transmute_copy(&ppwszdescription), core::mem::transmute_copy(&ppwszbtntext), core::mem::transmute_copy(&pdwflags)).into()
            }
        }
        Self { base__: IEmptyVolumeCache_Vtbl::new::<Identity, OFFSET>(), InitializeEx: InitializeEx::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IEmptyVolumeCache2 as windows_core::Interface>::IID || iid == &<IEmptyVolumeCache as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Registry")]
impl windows_core::RuntimeName for IEmptyVolumeCache2 {}
windows_core::imp::define_interface!(IEmptyVolumeCacheCallBack, IEmptyVolumeCacheCallBack_Vtbl, 0x6e793361_73c6_11d0_8469_00aa00442901);
windows_core::imp::interface_hierarchy!(IEmptyVolumeCacheCallBack, windows_core::IUnknown);
impl IEmptyVolumeCacheCallBack {
    pub unsafe fn ScanProgress<P2>(&self, dwlspaceused: u64, dwflags: u32, pcwszstatus: P2) -> windows_core::Result<()>
    where
        P2: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).ScanProgress)(windows_core::Interface::as_raw(self), dwlspaceused, dwflags, pcwszstatus.param().abi()).ok() }
    }
    pub unsafe fn PurgeProgress<P3>(&self, dwlspacefreed: u64, dwlspacetofree: u64, dwflags: u32, pcwszstatus: P3) -> windows_core::Result<()>
    where
        P3: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).PurgeProgress)(windows_core::Interface::as_raw(self), dwlspacefreed, dwlspacetofree, dwflags, pcwszstatus.param().abi()).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IEmptyVolumeCacheCallBack_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub ScanProgress: unsafe extern "system" fn(*mut core::ffi::c_void, u64, u32, windows_core::PCWSTR) -> windows_core::HRESULT,
    pub PurgeProgress: unsafe extern "system" fn(*mut core::ffi::c_void, u64, u64, u32, windows_core::PCWSTR) -> windows_core::HRESULT,
}
pub trait IEmptyVolumeCacheCallBack_Impl: windows_core::IUnknownImpl {
    fn ScanProgress(&self, dwlspaceused: u64, dwflags: u32, pcwszstatus: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn PurgeProgress(&self, dwlspacefreed: u64, dwlspacetofree: u64, dwflags: u32, pcwszstatus: &windows_core::PCWSTR) -> windows_core::Result<()>;
}
impl IEmptyVolumeCacheCallBack_Vtbl {
    pub const fn new<Identity: IEmptyVolumeCacheCallBack_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn ScanProgress<Identity: IEmptyVolumeCacheCallBack_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwlspaceused: u64, dwflags: u32, pcwszstatus: windows_core::PCWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IEmptyVolumeCacheCallBack_Impl::ScanProgress(this, core::mem::transmute_copy(&dwlspaceused), core::mem::transmute_copy(&dwflags), core::mem::transmute(&pcwszstatus)).into()
            }
        }
        unsafe extern "system" fn PurgeProgress<Identity: IEmptyVolumeCacheCallBack_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwlspacefreed: u64, dwlspacetofree: u64, dwflags: u32, pcwszstatus: windows_core::PCWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IEmptyVolumeCacheCallBack_Impl::PurgeProgress(this, core::mem::transmute_copy(&dwlspacefreed), core::mem::transmute_copy(&dwlspacetofree), core::mem::transmute_copy(&dwflags), core::mem::transmute(&pcwszstatus)).into()
            }
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
impl windows_core::RuntimeName for IEmptyVolumeCacheCallBack {}
windows_core::imp::define_interface!(IReconcilableObject, IReconcilableObject_Vtbl, 0x99180162_da16_101a_935c_444553540000);
windows_core::imp::interface_hierarchy!(IReconcilableObject, windows_core::IUnknown);
impl IReconcilableObject {
    #[cfg(feature = "Win32_System_Com_StructuredStorage")]
    pub unsafe fn Reconcile<P0, P7>(&self, pinitiator: P0, dwflags: u32, hwndowner: super::super::Foundation::HWND, hwndprogressfeedback: super::super::Foundation::HWND, rgpmkotherinput: &mut [Option<super::super::System::Com::IMoniker>], ploutindex: *mut i32, pstgnewresidues: P7, pvreserved: Option<*const core::ffi::c_void>) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IReconcileInitiator>,
        P7: windows_core::Param<super::super::System::Com::StructuredStorage::IStorage>,
    {
        unsafe { (windows_core::Interface::vtable(self).Reconcile)(windows_core::Interface::as_raw(self), pinitiator.param().abi(), dwflags, hwndowner, hwndprogressfeedback, rgpmkotherinput.len().try_into().unwrap(), core::mem::transmute(rgpmkotherinput.as_ptr()), ploutindex as _, pstgnewresidues.param().abi(), pvreserved.unwrap_or(core::mem::zeroed()) as _).ok() }
    }
    pub unsafe fn GetProgressFeedbackMaxEstimate(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetProgressFeedbackMaxEstimate)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IReconcilableObject_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_System_Com_StructuredStorage")]
    pub Reconcile: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, u32, super::super::Foundation::HWND, super::super::Foundation::HWND, u32, *mut *mut core::ffi::c_void, *mut i32, *mut core::ffi::c_void, *const core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com_StructuredStorage"))]
    Reconcile: usize,
    pub GetProgressFeedbackMaxEstimate: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_Com_StructuredStorage")]
pub trait IReconcilableObject_Impl: windows_core::IUnknownImpl {
    fn Reconcile(&self, pinitiator: windows_core::Ref<IReconcileInitiator>, dwflags: u32, hwndowner: super::super::Foundation::HWND, hwndprogressfeedback: super::super::Foundation::HWND, ulcinput: u32, rgpmkotherinput: *mut Option<super::super::System::Com::IMoniker>, ploutindex: *mut i32, pstgnewresidues: windows_core::Ref<super::super::System::Com::StructuredStorage::IStorage>, pvreserved: *const core::ffi::c_void) -> windows_core::Result<()>;
    fn GetProgressFeedbackMaxEstimate(&self) -> windows_core::Result<u32>;
}
#[cfg(feature = "Win32_System_Com_StructuredStorage")]
impl IReconcilableObject_Vtbl {
    pub const fn new<Identity: IReconcilableObject_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Reconcile<Identity: IReconcilableObject_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pinitiator: *mut core::ffi::c_void, dwflags: u32, hwndowner: super::super::Foundation::HWND, hwndprogressfeedback: super::super::Foundation::HWND, ulcinput: u32, rgpmkotherinput: *mut *mut core::ffi::c_void, ploutindex: *mut i32, pstgnewresidues: *mut core::ffi::c_void, pvreserved: *const core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IReconcilableObject_Impl::Reconcile(this, core::mem::transmute_copy(&pinitiator), core::mem::transmute_copy(&dwflags), core::mem::transmute_copy(&hwndowner), core::mem::transmute_copy(&hwndprogressfeedback), core::mem::transmute_copy(&ulcinput), core::mem::transmute_copy(&rgpmkotherinput), core::mem::transmute_copy(&ploutindex), core::mem::transmute_copy(&pstgnewresidues), core::mem::transmute_copy(&pvreserved)).into()
            }
        }
        unsafe extern "system" fn GetProgressFeedbackMaxEstimate<Identity: IReconcilableObject_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pulprogressmax: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IReconcilableObject_Impl::GetProgressFeedbackMaxEstimate(this) {
                    Ok(ok__) => {
                        pulprogressmax.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
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
#[cfg(feature = "Win32_System_Com_StructuredStorage")]
impl windows_core::RuntimeName for IReconcilableObject {}
windows_core::imp::define_interface!(IReconcileInitiator, IReconcileInitiator_Vtbl, 0x99180161_da16_101a_935c_444553540000);
windows_core::imp::interface_hierarchy!(IReconcileInitiator, windows_core::IUnknown);
impl IReconcileInitiator {
    pub unsafe fn SetAbortCallback<P0>(&self, punkforabort: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::IUnknown>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetAbortCallback)(windows_core::Interface::as_raw(self), punkforabort.param().abi()).ok() }
    }
    pub unsafe fn SetProgressFeedback(&self, ulprogress: u32, ulprogressmax: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetProgressFeedback)(windows_core::Interface::as_raw(self), ulprogress, ulprogressmax).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IReconcileInitiator_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub SetAbortCallback: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetProgressFeedback: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32) -> windows_core::HRESULT,
}
pub trait IReconcileInitiator_Impl: windows_core::IUnknownImpl {
    fn SetAbortCallback(&self, punkforabort: windows_core::Ref<windows_core::IUnknown>) -> windows_core::Result<()>;
    fn SetProgressFeedback(&self, ulprogress: u32, ulprogressmax: u32) -> windows_core::Result<()>;
}
impl IReconcileInitiator_Vtbl {
    pub const fn new<Identity: IReconcileInitiator_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn SetAbortCallback<Identity: IReconcileInitiator_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, punkforabort: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IReconcileInitiator_Impl::SetAbortCallback(this, core::mem::transmute_copy(&punkforabort)).into()
            }
        }
        unsafe extern "system" fn SetProgressFeedback<Identity: IReconcileInitiator_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ulprogress: u32, ulprogressmax: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IReconcileInitiator_Impl::SetProgressFeedback(this, core::mem::transmute_copy(&ulprogress), core::mem::transmute_copy(&ulprogressmax)).into()
            }
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
impl windows_core::RuntimeName for IReconcileInitiator {}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct RECONCILEF(pub i32);
pub const RECONCILEF_FEEDBACKWINDOWVALID: RECONCILEF = RECONCILEF(2i32);
pub const RECONCILEF_MAYBOTHERUSER: RECONCILEF = RECONCILEF(1i32);
pub const RECONCILEF_NORESIDUESOK: RECONCILEF = RECONCILEF(4i32);
pub const RECONCILEF_OMITSELFRESIDUE: RECONCILEF = RECONCILEF(8i32);
pub const RECONCILEF_ONLYYOUWERECHANGED: RECONCILEF = RECONCILEF(64i32);
pub const RECONCILEF_RESUMERECONCILIATION: RECONCILEF = RECONCILEF(16i32);
pub const RECONCILEF_YOUMAYDOTHEUPDATES: RECONCILEF = RECONCILEF(32i32);
pub const REC_E_ABORTED: windows_core::HRESULT = windows_core::HRESULT(0x80041000_u32 as _);
pub const REC_E_INEEDTODOTHEUPDATES: windows_core::HRESULT = windows_core::HRESULT(0x80041004_u32 as _);
pub const REC_E_NOCALLBACK: windows_core::HRESULT = windows_core::HRESULT(0x80041001_u32 as _);
pub const REC_E_NORESIDUES: windows_core::HRESULT = windows_core::HRESULT(0x80041002_u32 as _);
pub const REC_E_TOODIFFERENT: windows_core::HRESULT = windows_core::HRESULT(0x80041003_u32 as _);
pub const REC_S_IDIDTHEUPDATES: windows_core::HRESULT = windows_core::HRESULT(0x41000_u32 as _);
pub const REC_S_NOTCOMPLETE: windows_core::HRESULT = windows_core::HRESULT(0x41001_u32 as _);
pub const REC_S_NOTCOMPLETEBUTPROPAGATE: windows_core::HRESULT = windows_core::HRESULT(0x41002_u32 as _);
pub const STATEBITS_FLAT: u32 = 1u32;
