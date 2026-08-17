windows_core::imp::define_interface!(IADesktopP2, IADesktopP2_Vtbl, 0xb22754e2_4574_11d1_9888_006097deacf9);
impl core::ops::Deref for IADesktopP2 {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IADesktopP2, windows_core::IUnknown);
impl IADesktopP2 {
    pub unsafe fn ReReadWallpaper(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).ReReadWallpaper)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn GetADObjectFlags(&self, pdwflags: *mut u32, dwmask: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetADObjectFlags)(windows_core::Interface::as_raw(self), pdwflags, dwmask).ok()
    }
    pub unsafe fn UpdateAllDesktopSubscriptions(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).UpdateAllDesktopSubscriptions)(windows_core::Interface::as_raw(self)).ok()
    }
    #[cfg(feature = "Win32_System_Ole")]
    pub unsafe fn MakeDynamicChanges<P0>(&self, poleobj: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::System::Ole::IOleObject>,
    {
        (windows_core::Interface::vtable(self).MakeDynamicChanges)(windows_core::Interface::as_raw(self), poleobj.param().abi()).ok()
    }
}
#[repr(C)]
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
windows_core::imp::define_interface!(IActiveDesktopP, IActiveDesktopP_Vtbl, 0x52502ee0_ec80_11d0_89ab_00c04fc2972d);
impl core::ops::Deref for IActiveDesktopP {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IActiveDesktopP, windows_core::IUnknown);
impl IActiveDesktopP {
    pub unsafe fn SetSafeMode(&self, dwflags: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetSafeMode)(windows_core::Interface::as_raw(self), dwflags).ok()
    }
    pub unsafe fn EnsureUpdateHTML(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).EnsureUpdateHTML)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn SetScheme<P0>(&self, pwszschemename: P0, dwflags: u32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).SetScheme)(windows_core::Interface::as_raw(self), pwszschemename.param().abi(), dwflags).ok()
    }
    pub unsafe fn GetScheme(&self, pwszschemename: windows_core::PWSTR, pdwcchbuffer: *mut u32, dwflags: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetScheme)(windows_core::Interface::as_raw(self), core::mem::transmute(pwszschemename), pdwcchbuffer, dwflags).ok()
    }
}
#[repr(C)]
pub struct IActiveDesktopP_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub SetSafeMode: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub EnsureUpdateHTML: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetScheme: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, u32) -> windows_core::HRESULT,
    pub GetScheme: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PWSTR, *mut u32, u32) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IBriefcaseInitiator, IBriefcaseInitiator_Vtbl, 0x99180164_da16_101a_935c_444553540000);
impl core::ops::Deref for IBriefcaseInitiator {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IBriefcaseInitiator, windows_core::IUnknown);
impl IBriefcaseInitiator {
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn IsMonikerInBriefcase<P0>(&self, pmk: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::System::Com::IMoniker>,
    {
        (windows_core::Interface::vtable(self).IsMonikerInBriefcase)(windows_core::Interface::as_raw(self), pmk.param().abi()).ok()
    }
}
#[repr(C)]
pub struct IBriefcaseInitiator_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub IsMonikerInBriefcase: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    IsMonikerInBriefcase: usize,
}
windows_core::imp::define_interface!(IEmptyVolumeCache, IEmptyVolumeCache_Vtbl, 0x8fce5227_04da_11d1_a004_00805f8abe06);
impl core::ops::Deref for IEmptyVolumeCache {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IEmptyVolumeCache, windows_core::IUnknown);
impl IEmptyVolumeCache {
    #[cfg(feature = "Win32_System_Registry")]
    pub unsafe fn Initialize<P0, P1>(&self, hkregkey: P0, pcwszvolume: P1, ppwszdisplayname: *mut windows_core::PWSTR, ppwszdescription: *mut windows_core::PWSTR, pdwflags: *mut EMPTY_VOLUME_CACHE_FLAGS) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::System::Registry::HKEY>,
        P1: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).Initialize)(windows_core::Interface::as_raw(self), hkregkey.param().abi(), pcwszvolume.param().abi(), ppwszdisplayname, ppwszdescription, pdwflags).ok()
    }
    pub unsafe fn GetSpaceUsed<P0>(&self, pdwlspaceused: *mut u64, picb: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IEmptyVolumeCacheCallBack>,
    {
        (windows_core::Interface::vtable(self).GetSpaceUsed)(windows_core::Interface::as_raw(self), pdwlspaceused, picb.param().abi()).ok()
    }
    pub unsafe fn Purge<P0>(&self, dwlspacetofree: u64, picb: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IEmptyVolumeCacheCallBack>,
    {
        (windows_core::Interface::vtable(self).Purge)(windows_core::Interface::as_raw(self), dwlspacetofree, picb.param().abi()).ok()
    }
    pub unsafe fn ShowProperties<P0>(&self, hwnd: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::HWND>,
    {
        (windows_core::Interface::vtable(self).ShowProperties)(windows_core::Interface::as_raw(self), hwnd.param().abi()).ok()
    }
    pub unsafe fn Deactivate(&self) -> windows_core::Result<EMPTY_VOLUME_CACHE_FLAGS> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Deactivate)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
}
#[repr(C)]
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
    pub unsafe fn InitializeEx<P0, P1, P2>(&self, hkregkey: P0, pcwszvolume: P1, pcwszkeyname: P2, ppwszdisplayname: *mut windows_core::PWSTR, ppwszdescription: *mut windows_core::PWSTR, ppwszbtntext: *mut windows_core::PWSTR, pdwflags: *mut EMPTY_VOLUME_CACHE_FLAGS) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::System::Registry::HKEY>,
        P1: windows_core::Param<windows_core::PCWSTR>,
        P2: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).InitializeEx)(windows_core::Interface::as_raw(self), hkregkey.param().abi(), pcwszvolume.param().abi(), pcwszkeyname.param().abi(), ppwszdisplayname, ppwszdescription, ppwszbtntext, pdwflags).ok()
    }
}
#[repr(C)]
pub struct IEmptyVolumeCache2_Vtbl {
    pub base__: IEmptyVolumeCache_Vtbl,
    #[cfg(feature = "Win32_System_Registry")]
    pub InitializeEx: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::System::Registry::HKEY, windows_core::PCWSTR, windows_core::PCWSTR, *mut windows_core::PWSTR, *mut windows_core::PWSTR, *mut windows_core::PWSTR, *mut EMPTY_VOLUME_CACHE_FLAGS) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Registry"))]
    InitializeEx: usize,
}
windows_core::imp::define_interface!(IEmptyVolumeCacheCallBack, IEmptyVolumeCacheCallBack_Vtbl, 0x6e793361_73c6_11d0_8469_00aa00442901);
impl core::ops::Deref for IEmptyVolumeCacheCallBack {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IEmptyVolumeCacheCallBack, windows_core::IUnknown);
impl IEmptyVolumeCacheCallBack {
    pub unsafe fn ScanProgress<P0>(&self, dwlspaceused: u64, dwflags: u32, pcwszstatus: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).ScanProgress)(windows_core::Interface::as_raw(self), dwlspaceused, dwflags, pcwszstatus.param().abi()).ok()
    }
    pub unsafe fn PurgeProgress<P0>(&self, dwlspacefreed: u64, dwlspacetofree: u64, dwflags: u32, pcwszstatus: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).PurgeProgress)(windows_core::Interface::as_raw(self), dwlspacefreed, dwlspacetofree, dwflags, pcwszstatus.param().abi()).ok()
    }
}
#[repr(C)]
pub struct IEmptyVolumeCacheCallBack_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub ScanProgress: unsafe extern "system" fn(*mut core::ffi::c_void, u64, u32, windows_core::PCWSTR) -> windows_core::HRESULT,
    pub PurgeProgress: unsafe extern "system" fn(*mut core::ffi::c_void, u64, u64, u32, windows_core::PCWSTR) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IReconcilableObject, IReconcilableObject_Vtbl, 0x99180162_da16_101a_935c_444553540000);
impl core::ops::Deref for IReconcilableObject {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IReconcilableObject, windows_core::IUnknown);
impl IReconcilableObject {
    #[cfg(feature = "Win32_System_Com_StructuredStorage")]
    pub unsafe fn Reconcile<P0, P1, P2, P3>(&self, pinitiator: P0, dwflags: u32, hwndowner: P1, hwndprogressfeedback: P2, rgpmkotherinput: &mut [Option<super::super::System::Com::IMoniker>], ploutindex: *mut i32, pstgnewresidues: P3, pvreserved: Option<*const core::ffi::c_void>) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IReconcileInitiator>,
        P1: windows_core::Param<super::super::Foundation::HWND>,
        P2: windows_core::Param<super::super::Foundation::HWND>,
        P3: windows_core::Param<super::super::System::Com::StructuredStorage::IStorage>,
    {
        (windows_core::Interface::vtable(self).Reconcile)(windows_core::Interface::as_raw(self), pinitiator.param().abi(), dwflags, hwndowner.param().abi(), hwndprogressfeedback.param().abi(), rgpmkotherinput.len().try_into().unwrap(), core::mem::transmute(rgpmkotherinput.as_ptr()), ploutindex, pstgnewresidues.param().abi(), core::mem::transmute(pvreserved.unwrap_or(std::ptr::null()))).ok()
    }
    pub unsafe fn GetProgressFeedbackMaxEstimate(&self) -> windows_core::Result<u32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetProgressFeedbackMaxEstimate)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
}
#[repr(C)]
pub struct IReconcilableObject_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_System_Com_StructuredStorage")]
    pub Reconcile: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, u32, super::super::Foundation::HWND, super::super::Foundation::HWND, u32, *mut *mut core::ffi::c_void, *mut i32, *mut core::ffi::c_void, *const core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com_StructuredStorage"))]
    Reconcile: usize,
    pub GetProgressFeedbackMaxEstimate: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IReconcileInitiator, IReconcileInitiator_Vtbl, 0x99180161_da16_101a_935c_444553540000);
impl core::ops::Deref for IReconcileInitiator {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IReconcileInitiator, windows_core::IUnknown);
impl IReconcileInitiator {
    pub unsafe fn SetAbortCallback<P0>(&self, punkforabort: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::IUnknown>,
    {
        (windows_core::Interface::vtable(self).SetAbortCallback)(windows_core::Interface::as_raw(self), punkforabort.param().abi()).ok()
    }
    pub unsafe fn SetProgressFeedback(&self, ulprogress: u32, ulprogressmax: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetProgressFeedback)(windows_core::Interface::as_raw(self), ulprogress, ulprogressmax).ok()
    }
}
#[repr(C)]
pub struct IReconcileInitiator_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub SetAbortCallback: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetProgressFeedback: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32) -> windows_core::HRESULT,
}
pub const ALL_RECONCILE_FLAGS: RECONCILEF = RECONCILEF(127i32);
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
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct EMPTY_VOLUME_CACHE_FLAGS(pub u32);
impl windows_core::TypeKind for EMPTY_VOLUME_CACHE_FLAGS {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for EMPTY_VOLUME_CACHE_FLAGS {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("EMPTY_VOLUME_CACHE_FLAGS").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct RECONCILEF(pub i32);
impl windows_core::TypeKind for RECONCILEF {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for RECONCILEF {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("RECONCILEF").field(&self.0).finish()
    }
}
#[cfg(feature = "implement")]
core::include!("impl.rs");
