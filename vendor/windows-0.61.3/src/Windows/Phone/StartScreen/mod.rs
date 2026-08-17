#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DualSimTile(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(DualSimTile, windows_core::IUnknown, windows_core::IInspectable);
impl DualSimTile {
    pub fn new() -> windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&windows_core::imp::IGenericFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<DualSimTile, windows_core::imp::IGenericFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn SetDisplayName(&self, value: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetDisplayName)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(value)).ok() }
    }
    pub fn DisplayName(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DisplayName)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn IsPinnedToStart(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsPinnedToStart)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn CreateAsync(&self) -> windows_core::Result<windows_future::IAsyncOperation<bool>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateAsync)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn UpdateAsync(&self) -> windows_core::Result<windows_future::IAsyncOperation<bool>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).UpdateAsync)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn DeleteAsync(&self) -> windows_core::Result<windows_future::IAsyncOperation<bool>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DeleteAsync)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn GetTileForSim2() -> windows_core::Result<DualSimTile> {
        Self::IDualSimTileStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetTileForSim2)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn UpdateDisplayNameForSim1Async(name: &windows_core::HSTRING) -> windows_core::Result<windows_future::IAsyncOperation<bool>> {
        Self::IDualSimTileStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).UpdateDisplayNameForSim1Async)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(name), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[cfg(feature = "UI_Notifications")]
    pub fn CreateTileUpdaterForSim1() -> windows_core::Result<super::super::UI::Notifications::TileUpdater> {
        Self::IDualSimTileStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateTileUpdaterForSim1)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[cfg(feature = "UI_Notifications")]
    pub fn CreateTileUpdaterForSim2() -> windows_core::Result<super::super::UI::Notifications::TileUpdater> {
        Self::IDualSimTileStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateTileUpdaterForSim2)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[cfg(feature = "UI_Notifications")]
    pub fn CreateBadgeUpdaterForSim1() -> windows_core::Result<super::super::UI::Notifications::BadgeUpdater> {
        Self::IDualSimTileStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateBadgeUpdaterForSim1)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[cfg(feature = "UI_Notifications")]
    pub fn CreateBadgeUpdaterForSim2() -> windows_core::Result<super::super::UI::Notifications::BadgeUpdater> {
        Self::IDualSimTileStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateBadgeUpdaterForSim2)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[cfg(feature = "UI_Notifications")]
    pub fn CreateToastNotifierForSim1() -> windows_core::Result<super::super::UI::Notifications::ToastNotifier> {
        Self::IDualSimTileStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateToastNotifierForSim1)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[cfg(feature = "UI_Notifications")]
    pub fn CreateToastNotifierForSim2() -> windows_core::Result<super::super::UI::Notifications::ToastNotifier> {
        Self::IDualSimTileStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateToastNotifierForSim2)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    fn IDualSimTileStatics<R, F: FnOnce(&IDualSimTileStatics) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<DualSimTile, IDualSimTileStatics> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for DualSimTile {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IDualSimTile>();
}
unsafe impl windows_core::Interface for DualSimTile {
    type Vtable = <IDualSimTile as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IDualSimTile as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for DualSimTile {
    const NAME: &'static str = "Windows.Phone.StartScreen.DualSimTile";
}
windows_core::imp::define_interface!(IDualSimTile, IDualSimTile_Vtbl, 0x143ab213_d05f_4041_a18c_3e3fcb75b41e);
impl windows_core::RuntimeType for IDualSimTile {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IDualSimTile_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub SetDisplayName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub DisplayName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub IsPinnedToStart: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub CreateAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub UpdateAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub DeleteAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IDualSimTileStatics, IDualSimTileStatics_Vtbl, 0x50567c9e_c58f_4dc9_b6e8_fa6777eeeb37);
impl windows_core::RuntimeType for IDualSimTileStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IDualSimTileStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub GetTileForSim2: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub UpdateDisplayNameForSim1Async: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "UI_Notifications")]
    pub CreateTileUpdaterForSim1: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "UI_Notifications"))]
    CreateTileUpdaterForSim1: usize,
    #[cfg(feature = "UI_Notifications")]
    pub CreateTileUpdaterForSim2: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "UI_Notifications"))]
    CreateTileUpdaterForSim2: usize,
    #[cfg(feature = "UI_Notifications")]
    pub CreateBadgeUpdaterForSim1: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "UI_Notifications"))]
    CreateBadgeUpdaterForSim1: usize,
    #[cfg(feature = "UI_Notifications")]
    pub CreateBadgeUpdaterForSim2: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "UI_Notifications"))]
    CreateBadgeUpdaterForSim2: usize,
    #[cfg(feature = "UI_Notifications")]
    pub CreateToastNotifierForSim1: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "UI_Notifications"))]
    CreateToastNotifierForSim1: usize,
    #[cfg(feature = "UI_Notifications")]
    pub CreateToastNotifierForSim2: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "UI_Notifications"))]
    CreateToastNotifierForSim2: usize,
}
windows_core::imp::define_interface!(IToastNotificationManagerStatics3, IToastNotificationManagerStatics3_Vtbl, 0x2717f54b_50df_4455_8e6e_41e0fc8e13ce);
impl windows_core::RuntimeType for IToastNotificationManagerStatics3 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
windows_core::imp::interface_hierarchy!(IToastNotificationManagerStatics3, windows_core::IUnknown, windows_core::IInspectable);
impl IToastNotificationManagerStatics3 {
    #[cfg(feature = "UI_Notifications")]
    pub fn CreateToastNotifierForSecondaryTile(&self, tileid: &windows_core::HSTRING) -> windows_core::Result<super::super::UI::Notifications::ToastNotifier> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateToastNotifierForSecondaryTile)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(tileid), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[cfg(feature = "UI_Notifications")]
impl windows_core::RuntimeName for IToastNotificationManagerStatics3 {
    const NAME: &'static str = "Windows.Phone.StartScreen.IToastNotificationManagerStatics3";
}
#[cfg(feature = "UI_Notifications")]
pub trait IToastNotificationManagerStatics3_Impl: windows_core::IUnknownImpl {
    fn CreateToastNotifierForSecondaryTile(&self, tileId: &windows_core::HSTRING) -> windows_core::Result<super::super::UI::Notifications::ToastNotifier>;
}
#[cfg(feature = "UI_Notifications")]
impl IToastNotificationManagerStatics3_Vtbl {
    pub const fn new<Identity: IToastNotificationManagerStatics3_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn CreateToastNotifierForSecondaryTile<Identity: IToastNotificationManagerStatics3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, tileid: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IToastNotificationManagerStatics3_Impl::CreateToastNotifierForSecondaryTile(this, core::mem::transmute(&tileid)) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        core::mem::forget(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, IToastNotificationManagerStatics3, OFFSET>(),
            CreateToastNotifierForSecondaryTile: CreateToastNotifierForSecondaryTile::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IToastNotificationManagerStatics3 as windows_core::Interface>::IID
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IToastNotificationManagerStatics3_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "UI_Notifications")]
    pub CreateToastNotifierForSecondaryTile: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "UI_Notifications"))]
    CreateToastNotifierForSecondaryTile: usize,
}
